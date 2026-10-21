import json, struct, hashlib, difflib
from pathlib import Path
w = Path(__file__).parent
source = Path('tiny-glade/crates/systems/tree-tool/src/ui_place_tree.rs')
before = (w / 'before.rs').read_text()
s = before[before.index('use bevy_ecs'):]
s = '''//! 树木放置 UI 与编辑命令；保留当前系统和权威 TreeStorage。
//!
//! B 原字节 142269970：以存储索引初始化本地 fastrand1.8.0，依次采样
//! 缩放、变体、旋转，提交 Insert 后录入历史。14226a170 的三个 ICF
//! 同址函数由 ActiveSession 的 PDB inline 确认为 olden/winter/autumn。
//! UI seed_cursor_icon 的原闭包仍待恢复，不能把 raw 工厂视图视为已渲染。

''' + s
start=s.index('/// 树缩放 lerp'); end=s.index('/// 树 prefab 路径表',start)
s=s[:start]+'''/// B 142269c00..c8c：缩放在 .5 与 .9 之间插值。
pub const TREE_SCALE_MIN: f32 = 0.5;
pub const TREE_SCALE_MAX: f32 = 0.9;
/// B 142269aa0 用 MOVSS 载入 .5，并把高位清零；旋转 lane 的下限为零。
pub const TREE_ROTATION_MAX: f32 = 360.0;
pub const TREE_ROTATION_MIN: f32 = 0.0;

'''+s[end:]
start=s.index('/// `random_tree_prefab` @');end=s.index('/// 水下 y 重映射',start)
s=s[:start]+'''/// B 14226a170：仅按存储索引奇偶及会话主题选择七种原 prefab。
/// 参数次序保留现有调用面；ICF 别名不能解释为加载、损坏或拖动状态。
pub fn random_tree_prefab(
    selector: u8,
    is_olden: bool,
    is_halloween: bool,
    is_winter: bool,
    is_autumn: bool,
) -> &'static str {
    let bit = (selector & 1) as usize;
    if is_winter && bit == 0 {
        return TREE_PREFAB_NO_MUSHROOMS;
    }
    if (is_olden || is_halloween) && bit == 0 {
        return TREE_PREFAB_NAKED[0];
    }
    let table = if is_autumn {
        &TREE_PREFAB_AUTUMN
    } else {
        &TREE_PREFAB_BRANCHY
    };
    table[bit]
}

/// 同一存储索引的放置属性使用同一随机流，不读取线程全局 RNG。
/// 原版整数采样为 i32(0..10)，由 pinned fastrand 保留拒绝重采样。
fn placement_randoms(index: u64) -> (f32, i32, f32) {
    let rng = fastrand::Rng::with_seed(index);
    let scale = rng.f32();
    let variant = rng.i32(0..10);
    let rotation = rng.f32();
    (scale, variant, rotation)
}

'''+s[end:]
start=s.index('/// `place_tree` @'); end=s.index('/// `seed_cursor_icon`',start)
s=s[:start]+'''/// B 142269970：从真实位置事件生成完整插入命令，并录入同一份历史数据。
/// Insert 先入队；缺少历史录制时保留错误，不能静默丢失撤销/保存记录。
pub fn place_tree<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut commands: EventReader<PlaceTreeCmd>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut history: ResMut<History<S>>,
    session: Res<ActiveSession>,
) {
    use country_core::resources::history::tree_edit::{TreeEdit, TreeEditData};
    use std::sync::Arc;

    for cmd in commands.read() {
        let index = storage.next_index();
        let (r1, variant, r2) = placement_randoms(index.0);
        let prefab = random_tree_prefab(
            index.0 as u8,
            session.is_olden(),
            session.meta.is_halloween_any(),
            session.is_winter(),
            session.is_autumn(),
        );
        let scale = r1 * TREE_SCALE_MAX + (1.0 - r1) * TREE_SCALE_MIN;
        let rotation = r2 * TREE_ROTATION_MAX + (1.0 - r2) * TREE_ROTATION_MIN;
        // B 142269cc7 写入 1；该字节原样复制到 Insert 与 TreeEdit 的布尔字段。
        let data = TreeEditData {
            storage_index: Some(index),
            id: variant,
            position: cmd.position,
            previous_position: glam::Vec3::ZERO,
            scale,
            rotation_degrees: rotation,
            prefab_name: prefab.to_string(),
            is_preplaced: true,
        };
        inserts.write(InsertTreeCmd {
            id: index.0,
            rotation_degrees: rotation,
            variant,
            is_preplaced: true,
            position: cmd.position,
            scale,
            prefab_name: prefab.to_string(),
        });
        history
            .add(EditType::Tree(Arc::new(TreeEdit::Place(data))))
            .expect("tree placement requires an active history recording");
    }
}

'''+s[end:]
mask=(1<<64)-1
f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
def randoms(seed):
    state=seed
    def draw():
        nonlocal state
        state=(state+0xa0761d6478bd642f)&mask
        t=state*(state^0xe7037ed1a0b428db)
        return ((t&mask)^(t>>64))&0xffffffff
    def draw_float():
        return f32(struct.unpack('<f',struct.pack('<I',0x3f800000+(draw()>>9)))[0]-1)
    r1=draw_float()
    while True:
        n=draw()*10
        if (n&0xffffffff)>=6:break
    variant=n>>32
    r2=draw_float()
    scale=f32(f32(r1*f32(.9))+f32(f32(1-r1)*f32(.5)))
    rotation=f32(f32(r2*f32(360))+f32(f32(1-r2)*f32(0)))
    return variant,bits(scale),bits(rotation)
fixtures=[(i,*randoms(i)) for i in [0,1,2,7,42,100000,0x123456789abcdef0,mask]]
(w/'native-placement-fixtures.json').write_text(json.dumps({'source':'B 142269b50..c8c; integer WY multiply/rejection and f32 SSE rounding; no target execution','fixtures':[{'index':i,'variant':v,'scale_bits':hex(sc),'rotation_bits':hex(ro)} for i,v,sc,ro in fixtures]},indent=2))
rows='\n'.join(f'            ({i}u64, {v}, 0x{sc:08x}, 0x{ro:08x}),' for i,v,sc,ro in fixtures)
s+='''
#[cfg(test)]
mod placement_producer_tests {
    use super::*;
    use country_core::resources::{
        history::tree_edit::TreeEdit,
        session::{GladeType, SessionId, SessionMeta, SessionMutator},
    };

    fn run(glade: GladeType, mutators: Vec<SessionMutator>, global_seed: u64) -> (InsertTreeCmd, World) {
        fastrand::seed(global_seed);
        let mut world = World::new();
        world.init_resource::<TreeStorage>();
        world.init_resource::<Events<PlaceTreeCmd>>();
        world.init_resource::<Events<InsertTreeCmd>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(16).unwrap();
        world.insert_resource(history);
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta { glade, mutators, image: None },
        ));
        world.send_event(PlaceTreeCmd { position: glam::Vec3::new(1.0, -1.2, 3.0) });
        let mut schedule = Schedule::default();
        schedule.add_systems(place_tree::<()>);
        schedule.run(&mut world);
        let events = world.resource::<Events<InsertTreeCmd>>();
        let mut reader = events.get_cursor();
        let insert = reader.read(events).next().unwrap().clone();
        (insert, world)
    }

    #[test]
    fn frozen_native_placement_fixtures_keep_rng_draw_order_and_zero_rotation_lane() {
        for (index, variant, scale_bits, rotation_bits) in [
'''+rows+'''
        ] {
            let (r1, actual_variant, r2) = placement_randoms(index);
            let scale = r1 * TREE_SCALE_MAX + (1.0 - r1) * TREE_SCALE_MIN;
            let rotation = r2 * TREE_ROTATION_MAX + (1.0 - r2) * TREE_ROTATION_MIN;
            assert_eq!(actual_variant, variant, "index={index}");
            assert_eq!(scale.to_bits(), scale_bits, "index={index}");
            assert_eq!(rotation.to_bits(), rotation_bits, "index={index}");
        }
    }

    #[test]
    fn placement_uses_session_theme_and_same_full_data_for_insert_and_history() {
        for (glade, mutators, expected) in [
            (GladeType::Summer, vec![], TREE_PREFAB_BRANCHY[0]),
            (GladeType::Olden, vec![], TREE_PREFAB_NAKED[0]),
            (GladeType::Flowery, vec![], TREE_PREFAB_BRANCHY[0]),
            (GladeType::Autumn, vec![], TREE_PREFAB_AUTUMN[0]),
            (GladeType::Winter, vec![], TREE_PREFAB_NO_MUSHROOMS),
            (GladeType::Summer, vec![SessionMutator::Halloween2024], TREE_PREFAB_NAKED[0]),
        ] {
            let (insert, world) = run(glade, mutators, 1);
            assert_eq!(insert.id, 0);
            assert_eq!(insert.prefab_name, expected);
            assert!(insert.is_preplaced);
            let pending = world.resource::<History<()>>().pending().unwrap();
            let EditType::Tree(edit) = &pending.edits[0] else { panic!("expected tree edit") };
            let TreeEdit::Place(data) = edit.as_ref() else { panic!("expected place edit") };
            assert_eq!(data.storage_index.unwrap().0, insert.id);
            assert_eq!(data.id, insert.variant);
            assert_eq!(data.position, insert.position);
            assert_eq!(data.scale.to_bits(), insert.scale.to_bits());
            assert_eq!(data.rotation_degrees.to_bits(), insert.rotation_degrees.to_bits());
            assert_eq!(data.prefab_name, insert.prefab_name);
            assert_eq!(data.is_preplaced, insert.is_preplaced);
        }
    }

    #[test]
    fn unrelated_global_rng_seed_does_not_change_the_same_tree() {
        let (a, _) = run(GladeType::Autumn, vec![], 1);
        let (b, _) = run(GladeType::Autumn, vec![], u64::MAX);
        assert_eq!(a.variant, b.variant);
        assert_eq!(a.scale.to_bits(), b.scale.to_bits());
        assert_eq!(a.rotation_degrees.to_bits(), b.rotation_degrees.to_bits());
        assert_eq!(a.prefab_name, b.prefab_name);
    }
}
'''
(w/'candidate.rs').write_text(s)
(w/'candidate.patch').write_text(''.join(difflib.unified_diff(before.splitlines(True),s.splitlines(True),fromfile='a/'+str(source),tofile='b/'+str(source))))
(w/'hashes.json').write_text(json.dumps({'before_sha256':hashlib.sha256(before.encode()).hexdigest(),'candidate_sha256':hashlib.sha256(s.encode()).hexdigest(),'source':str(source),'tests_executed':False},indent=2))
print((w/'hashes.json').read_text())
