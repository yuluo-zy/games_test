from pathlib import Path
import json,hashlib,difflib,shutil,subprocess
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
fixture=json.loads((out/'native-cursor-event-fixtures.json').read_text())
rel='games_test/tiny-glade/crates/country-core/src/systems/backend/input/update_cursor_position.rs';p=root/rel;base=p.read_text()
old='    events.write(CursorMovedScreenSpace(next));'
new='''    // Position and tool motion use different existing action slots. Native
    // +0x148 updates the window cursor, while +0x140 is the event payload;
    // capture retains tool motion and zero motion produces no event.
    let movement = states.analog[0];
    if movement != Vec2::ZERO {
        events.write(CursorMovedScreenSpace(movement));
    }'''
assert old in base
candidate=base.replace(old,new,1).replace('//! 将窗口空间位移应用到游标；捕获期间保持锚点，原始增量仅供相机等动作使用。','//! 将窗口位移应用到游标；捕获保持锚点，另将原始输入位移交给工具消费者。').replace('/// 在游戏消费者之前生成本帧屏幕坐标和事件。','/// 在游戏消费者之前生成本帧屏幕坐标和工具位移事件。')
rows=[]
for c in fixture['cases']:
 a=lambda xs:'['+', '.join(hex(x)for x in xs)+']'
 payload='Some('+a(c['event_payload_bits'][0])+')'if c['event_payload_bits']else'None'
 rows.append('            ('+a(c['input_bits'])+', '+a(c['hardware_bits'])+', '+str(c['captured']).lower()+', '+a(c['output_position_bits'])+', '+payload+'), // '+c['name'])
candidate+='''
#[cfg(test)]
mod native_event_tests {
    use super::*;
    use crate::{
        resources::terrain_editing::StrokeId,
        systems::terrain::ui_edit_terrain::edit_command_for_drag,
    };
    use glam::Vec3;
    use tg_terrain::edit::{Adjustment, EditCommand};

    fn world(input: [u32; 2], hardware: [u32; 2], captured: bool) -> World {
        let mut world = World::new();
        let mut states = ActionSetStates::default();
        states.analog[0] = Vec2::from_array(input.map(f32::from_bits));
        states.analog[1] = Vec2::from_array(hardware.map(f32::from_bits));
        world.insert_resource(states);
        world.insert_resource(CursorPositionSS(Vec2::new(50.0, 20.0)));
        world.insert_resource(CursorCapturedPosition(
            captured.then_some(Vec2::new(50.0, 20.0)),
        ));
        world.insert_resource(WindowSize::new(100, 100));
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        world
    }

    #[test]
    fn original_event_payload_and_nonzero_gate() {
        // Executed original B inline fixtures. Position assertions are limited
        // to inputs compatible with the retained current clamp/anchor policy.
        for (input, hardware, captured, expected_position, expected_payload) in [
'''+ '\n'.join(rows)+'''
        ] {
            let mut world = world(input, hardware, captured);
            let mut schedule = Schedule::default();
            schedule.add_systems(update_cursor_position);
            schedule.run(&mut world);
            assert_eq!(
                world.resource::<CursorPositionSS>().0.to_array().map(f32::to_bits),
                expected_position
            );
            let payloads: Vec<_> = world
                .resource_mut::<Events<CursorMovedScreenSpace>>()
                .drain()
                .map(|event| event.0.to_array().map(f32::to_bits))
                .collect();
            assert_eq!(payloads, expected_payload.into_iter().collect::<Vec<_>>());
        }
    }

    #[derive(Resource, Default)]
    struct DragCommands(Vec<EditCommand>);

    fn consume_drag(
        mut events: EventReader<CursorMovedScreenSpace>,
        mut commands: ResMut<DragCommands>,
    ) {
        for event in events.read() {
            if let Some(command) = edit_command_for_drag(StrokeId(7), 5.0, Vec3::ZERO, event.0) {
                commands.0.push(command);
            }
        }
    }

    #[test]
    fn captured_tool_motion_reaches_existing_terrain_drag_consumer_once() {
        let mut world = world([1.0_f32.to_bits(), 0], [0, 0], true);
        world.init_resource::<DragCommands>();
        let mut schedule = Schedule::default();
        schedule.add_systems((update_cursor_position, consume_drag).chain());
        schedule.run(&mut world);
        let commands = &world.resource::<DragCommands>().0;
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].id, StrokeId(7));
        assert_eq!(commands[0].falloff, Adjustment::Delta(0.002));
        assert_eq!(commands[0].height, Adjustment::Keep);
        assert_eq!(world.resource::<CursorPositionSS>().0, Vec2::new(50.0, 20.0));

        world.resource_mut::<ActionSetStates>().analog[0] = Vec2::ZERO;
        schedule.run(&mut world);
        assert_eq!(world.resource::<DragCommands>().0.len(), 1);
    }
}
'''
(out/'update_cursor_position.rs').write_text(candidate)
subprocess.run(['rustfmt','--edition','2024',str(out/'update_cursor_position.rs')],check=True)
candidate=(out/'update_cursor_position.rs').read_text()
(out/'owned-producer.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),candidate.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)))
relwindow='games_test/tiny-glade/crates/country-core/src/resources/window.rs';wp=root/relwindow;ws=wp.read_text()
old='''/// `CursorMovedScreenSpace`（B `country_core::resources::window::CursorMovedScreenSpace`，
/// O：PDB `update_cursor_position` into_configs @0x1413ee760 的 EventWriter 类型）。
/// `update_cursor_position` 每帧产出窗口空间游标位置（f32 屏幕空间）。
/// 载荷形状 I（待取证：可能为裸 Vec2；事件按 bevy 每元素 0x10 B 槽承载）。'''
new='''/// 工具消费的输入空间光标位移；由现有 `ActionSetStates::analog[0]` 非零时产生。
/// B `update_cursor_position` inline @0x14173db3e 读取原+0x140 Vec2，
/// @0x14173db95 写入此事件；捕获期间仍发送位移，零向量不发送，NaN 按非零处理。
/// 窗口位置另由 analog[1] 更新到 CursorPositionSS；事件载荷不包含绝对位置。'''
assert old in ws
newws=ws.replace(old,new,1)
(out/'window.rs').write_text(newws)
(out/'integration-owner-contract.patch').write_text(''.join(difflib.unified_diff(ws.splitlines(True),newws.splitlines(True),fromfile='a/'+relwindow,tofile='b/'+relwindow)))
previous=root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/arrow-review'
(out/'evidence').mkdir(exist_ok=True)
for name in['original-input-inline-records.json','14173d9e0-input-wrapper.asm','1414fbb50-input-wrapper.asm']:
 shutil.copy2(previous/name,out/'evidence'/name)
manifest={'target':'B','claim':'TG-0cbb460a1bfa4b83b0','production_edited':False,'cargo_run':False,'original_case_count':len(fixture['cases']),'files':[{'path':str(p),'base_sha256':sha(base.encode()),'candidate':str(out/'update_cursor_position.rs'),'candidate_sha256':sha(candidate.encode())},{'path':str(wp),'base_sha256':sha(ws.encode()),'candidate':str(out/'window.rs'),'candidate_sha256':sha(newws.encode()),'scope':'root shared documentation adaptation only'}],'patches':{name:sha((out/name).read_bytes())for name in['owned-producer.patch','integration-owner-contract.patch']},'existing_input_regression_changes':'none: repository tests do not assert previous absolute CursorMovedScreenSpace payload; new actual producer tests expose it'}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(manifest['patches'])
