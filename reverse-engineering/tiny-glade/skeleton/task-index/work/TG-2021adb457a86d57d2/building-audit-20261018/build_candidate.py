from pathlib import Path
import json,re,difflib,hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
OUT=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-2021adb457a86d57d2/building-audit-20261018'
CAND=OUT/'candidate';CAND.mkdir(exist_ok=True)
sources={};candidates={}
def load(rel):
 text=(ROOT/rel).read_text();sources[rel]=text;return text
def save(rel,text):
 path=CAND/rel;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text);candidates[rel]=text
core='games_test/tiny-glade/crates/country-core/src/resources/critters/sheep.rs'
s=load(core)
s=s.replace('//! 羊群的移动状态和快照所需字段。B 原版将移动状态保存在 Sheep+0x30，\n//! 世界位置仍由 Transform.translation 持有；完整原生 Sheep 为 104 字节，\n//! 此类型仅表示已恢复的语义字段，不声明原生布局。','//! Shared live sheep component, used by the current critter systems and core\n//! snapshot consumers. B movement occupies +0x30; position stays in Transform.\n//! Existing animation fields are retained semantically, without a native ABI.')
s=s.replace('/// B capture/apply读取+0x30移动状态；being_petted读取+0x58。\n/// 未恢复的动画/附件字段不填充猜测默认值。','/// The single live sheep owner. Animation/behavior fields come from the\n/// existing full critter component; the former flag_30/shared_timers are the\n/// typed movement state, including the previously dropped force byte+0x31.\n/// No birth/default constructor is inferred from this consolidation.')
s=s.replace('    pub move_state: SheepMoveState,\n    pub being_petted_timer: f32,','''    /// Original +8 / +0xc: follow behavior and its timer.
    pub behavior_state: u32,
    pub goal_timer: f32,
    /// Existing pet animation state, progress and target identifier.
    pub pet_phase: u32,
    pub pet_progress: f32,
    pub pet_target: u64,
    pub move_state: SheepMoveState,
    /// Original +0x58, shared with the pet animation consumer.
    pub amplitude: f32,
    pub control_flag: u8,''')
s=s.replace('self.being_petted_timer','self.amplitude')
extras=['behavior_state: 0,','goal_timer: 0.0,','pet_phase: 0,','pet_progress: 0.0,','pet_target: 0,','control_flag: 0,']
def adapt_literals(s):
 s=s.replace('being_petted_timer','amplitude')
 def insert(m):
  indent=m.group(2)
  return m.group(1)+''.join(indent+x+'\n'for x in extras)+indent+m.group(3)
 return re.sub(r'(Sheep \{\n)([ \t]+)(move_state:)',insert,s)
save(core,adapt_literals(s))
for rel in ['games_test/tiny-glade/crates/country-core/src/resources/critters/sheep/control.rs','games_test/tiny-glade/crates/country-core/src/systems/debug/sheep_controls.rs','games_test/tiny-glade/crates/country-core/tests/sheep_snapshot_native.rs']:
 save(rel,adapt_literals(load(rel)))
rel='games_test/tiny-glade/crates/systems/critters/src/components.rs';s=load(rel)
start=s.index('/// `Sheep`（stride 0x68');end=s.index('/// 羊抚摸控制',start)
s=s[:start]+'''/// Shared ECS identity with core snapshot/save/freeze consumers.
pub use country_core::resources::critters::sheep::{Sheep, SheepMoveState};

'''+s[end:]
s=s.replace('标志字节 @+0x30、8B 双计时 @+0x34、幅度 @+0x58','移动状态 @+0x30（含force @+0x31和计时 @+0x34/+0x38）、幅度 @+0x58')
save(rel,s)
rel='games_test/tiny-glade/crates/systems/critters/src/sheep/sheep_behavior.rs';s=load(rel)
s=s.replace('use glam::{Vec2, Vec3};','use glam::{Quat, Vec2, Vec3};').replace('use crate::components::{CritterVelocityTarget, Sheep, Transform};','use crate::components::{CritterVelocityTarget, Sheep, SheepMoveState, Transform};')
start=s.index('/// 羊目标组件');end=s.index('/// `utils::uniform_sample_disk`',start)
s=s[:start]+'''/// Existing public path retained as an alias to the shared goal Component.
/// Rust field order does not reproduce native Option<Vec3> layout.
pub use country_core::resources::critters::CritterGoal as SheepGoal;

'''+s[end:]
start=s.index('/// update_sheep_goals（');end=s.index('/// 羊跟随目标模式',start)
new='''/// Original movement phase: timer updates happen before deciding whether to
/// retain the existing goal, start a state transition, or choose a new goal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GoalAction {
    Keep,
    Refresh,
    StartIdle,
    StartWalking,
}

fn advance_move_state(state: &mut SheepMoveState, dt: f32, goal: &SheepGoal, position: Vec3) -> GoalAction {
    match state {
        SheepMoveState::Idle { timeout_s } => {
            *timeout_s -= dt;
            if *timeout_s <= 0.0 { GoalAction::StartWalking } else { GoalAction::Keep }
        }
        SheepMoveState::Walking { timeout_s, goal_change_timeout, force_reach_goal } => {
            *timeout_s -= dt;
            *goal_change_timeout -= dt;
            if *timeout_s <= 0.0 && !*force_reach_goal {
                return GoalAction::StartIdle;
            }
            let dx = goal.position.x - position.x;
            let dz = goal.position.z - position.z;
            let distance_squared = dx * dx + dz * dz;
            // Ordered comparisons preserve the original UCOMISS NaN branches.
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            let retain = (!(*goal_change_timeout <= 0.0) || *force_reach_goal)
                && goal.goal_tag & 1 != 0
                && !(distance_squared < f32::from_bits(0x3e23_d70b));
            if retain { GoalAction::Keep } else { GoalAction::Refresh }
        }
    }
}

/// B 14231596a..142315acd: rotate fixed -Z by the existing quaternion,
/// halve/project it to XZ, add the resolved disk sample, then normalize.
/// This reuses current glam instead of reproducing its dependency internals.
fn sheep_goal_direction(rotation: Quat, disk: Vec2, active_goal: bool) -> Vec3 {
    let forward = (rotation * Vec3::NEG_Z) * 0.5;
    let scale = if active_goal { 0.8 } else { 0.5 };
    Vec3::new(disk.x + scale * forward.x, 0.0, disk.y + scale * forward.z)
        .normalize_or_zero()
}

/// B update_sheep_goals 0x142315050. Retain the existing current-system
/// architecture/RNG; typed movement is shared with snapshot capture/restore.
#[allow(clippy::too_many_arguments)]
pub fn update_sheep_goals(
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    cursor: Res<CursorTerrainRaycast>,
    water: Res<WorldRaster<512>>,
    rng: Res<CritterRng>,
    mut query: Query<(&Transform, &mut Sheep, &SheepTerrainState, &mut SheepGoal)>,
) {
    let dt = time.delta;
    let mut rng_ctx = rng.0.lock().unwrap();
    let w = (*water).as_ref();
    for (transform, mut sheep, terrain_state, mut goal) in &mut query {
        let pos = transform.translation;
        // The original keeps a pending state transition while processing the
        // previous movement state. Idle's inactive timer/force bytes are not data.
        let follow_transition = if sheep.behavior_state == 1 {
            let target = if camera.mode_byte() == 3 {
                camera.eye_position() + camera.eye_forward() * 2.0
            } else {
                cursor.position()
            };
            goal.goal_tag = 1;
            goal.position = target;
            sheep.goal_timer -= dt;
            if (cursor.position() - pos).length() < 0.2 {
                sheep.goal_timer -= 3.0 * dt;
            }
            if sheep.goal_timer >= 0.0 {
                let timeout_s = rng_ctx.uniform_f32() * 12.0 + 4.0;
                let goal_change_timeout = rng_ctx.uniform_f32() * 3.0 + 1.0;
                Some(SheepMoveState::Walking {
                    timeout_s, goal_change_timeout, force_reach_goal: false,
                })
            } else {
                sheep.behavior_state = 0;
                Some(SheepMoveState::Idle { timeout_s: rng_ctx.uniform_f32() * 7.0 + 1.0 })
            }
        } else { None };

        match advance_move_state(&mut sheep.move_state, dt, &goal, pos) {
            GoalAction::StartWalking => {
                let timeout_s = rng_ctx.uniform_f32() * 12.0 + 4.0;
                let goal_change_timeout = rng_ctx.uniform_f32() * 3.0 + 1.0;
                sheep.move_state = SheepMoveState::Walking {
                    timeout_s, goal_change_timeout, force_reach_goal: false,
                };
                continue;
            }
            GoalAction::StartIdle => {
                sheep.move_state = SheepMoveState::Idle { timeout_s: rng_ctx.uniform_f32() * 7.0 + 1.0 };
                continue;
            }
            GoalAction::Keep => {}
            GoalAction::Refresh => {
                let step = rng_ctx.uniform_f32() * 3.0 + 1.0;
                if let SheepMoveState::Walking { goal_change_timeout, .. } = &mut sheep.move_state {
                    *goal_change_timeout = step;
                }
                let disk = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                let mut direction = sheep_goal_direction(transform.rotation, disk, goal.goal_tag != 0);
                if demo_mode() && pos.length() > 35.0 {
                    let disk2 = disk_sample(rng_ctx.uniform_f32(), rng_ctx.uniform_f32());
                    direction = (x0y(disk2) * 25.0 - pos).normalize();
                }
                let new_pos = pos + direction * step;
                let r_new = (new_pos.x * new_pos.x + new_pos.z * new_pos.z).sqrt();
                let r_old = (pos.x * pos.x + pos.z * pos.z).sqrt();
                let allowed = (r_new < 65.0 || r_new < r_old)
                    && (terrain_state.state != STATE_GROUNDED || {
                        let s1 = w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0);
                        s1 <= 0.0 && {
                            let s2 = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
                            s2 > 0.0 || w.bilinear_sample_at_world_xz(new_pos.x, new_pos.z, 130.0, 130.0) <= 0.0
                        }
                    });
                goal.goal_tag = u32::from(allowed);
                if allowed { goal.position = new_pos; }
                // Both accepted and rejected replacement clear the original +31.
                if let SheepMoveState::Walking { force_reach_goal, .. } = &mut sheep.move_state {
                    *force_reach_goal = false;
                }
            }
        }
        if let Some(state) = follow_transition { sheep.move_state = state; }
    }
}

'''
s=s[:start]+new+s[end:];s=s.replace('goal.active != 0 && sheep.flag_30 == 1','goal.goal_tag != 0 && matches!(sheep.move_state, SheepMoveState::Walking { .. })').replace('            goal.pos - pos','            goal.position - pos')
# Preserve all other consumers and their current types.
save(rel,s)
rel='games_test/tiny-glade/crates/systems/critters/src/sheep/sheep_unstuck.rs';s=load(rel)
s=s.replace('use crate::components::{Sheep, Transform, Velocity};','use crate::components::{Sheep, SheepMoveState, Transform, Velocity};')
s=s.replace('''        if loc.0 != 2 && sheep.flag_30 == 0 {
            sheep.shared_timers = [0.0, 0.0];
        }''','''        if loc.0 != 2 {
            // B 142324d21..142324d2c clears only the active Idle timeout.
            if let SheepMoveState::Idle { timeout_s } = &mut sheep.move_state {
                *timeout_s = 0.0;
            }
        }''')
s=s.replace('清零 shared_timers','清零 Idle timeout_s').replace('清零共享计时','清零 Idle timeout_s')
save(rel,s)
rel='games_test/tiny-glade/crates/systems/critters/tests/numeric_assertions.rs';s=load(rel)
s=s.replace('use system_critters::components::{Sheep, Transform};','use system_critters::components::{Sheep, SheepMoveState, Transform};').replace('''        flag_30: 0,
        shared_timers: [0.0; 2],''','''        move_state: SheepMoveState::Idle { timeout_s: 0.0 },''')
s+='''

/// This fixture supplies unrelated fields explicitly; it is not a recovered
/// birth/default initializer. Core save must see the actual critter component.
#[test]
fn critter_sheep_and_goal_are_the_core_save_component_identity() {
    use country_core::resources::critters::{CritterGoal, sheep::Sheep as CoreSheep};
    use country_core::systems::save::save_game::{SheepSaveSectionValue, sheep_save_section_system};
    use system_critters::sheep::sheep_behavior::SheepGoal;
    assert_eq!(std::any::TypeId::of::<Sheep>(), std::any::TypeId::of::<CoreSheep>());
    assert_eq!(std::any::TypeId::of::<SheepGoal>(), std::any::TypeId::of::<CritterGoal>());
    let mut live = sheep(17, 0.75);
    live.behavior_state = 1;
    live.pet_phase = 2;
    live.pet_progress = 0.5;
    live.move_state = SheepMoveState::Walking {
        timeout_s: 4.0, goal_change_timeout: 5.0, force_reach_goal: true,
    };
    let mut world = World::new();
    world.init_resource::<SheepSaveSectionValue>();
    let entity = world.spawn((
        Transform::from_translation(Vec3::new(1.0, 2.0, 3.0)), live,
        SheepGoal { position: Vec3::new(6.0, 7.0, 8.0), goal_tag: 1 },
    )).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(sheep_save_section_system);
    schedule.run(&mut world);
    assert_eq!(world.resource::<SheepSaveSectionValue>().0.as_str(),
        r#"{"sheep":[{"position":[1.0,2.0,3.0],"goal":[6.0,7.0,8.0],"move_state":{"Walking":{"timeout_s":4.0,"goal_change_timeout":5.0,"force_reach_goal":true}}}]}"#);
    assert_eq!(*world.get::<CoreSheep>(entity).unwrap(), live);
}
'''
save(rel,s)
# Native movement/direction expectations are generated from the bounded original probe.
p=OUT/'evidence/force-phase-native.json';native=json.loads(p.read_text());rel='games_test/tiny-glade/crates/systems/critters/src/sheep/sheep_behavior.rs';s=candidates[rel]
s+='''

#[cfg(test)]
mod original_movement_phase_tests {
    use super::*;
    fn f(bits: u32) -> f32 { f32::from_bits(bits) }
    #[test]
    fn movement_phase_matches_original_b_instruction_fixtures() {
'''
for i,c in enumerate(native['cases']):
 st=(f"SheepMoveState::Walking {{ timeout_s: f({c['timeout']}), goal_change_timeout: f({c['goal_change_timeout']}), force_reach_goal: {str(c['force']).lower()} }}" if c['walking'] else f"SheepMoveState::Idle {{ timeout_s: f({c['timeout']}) }}")
 action={'keep':'Keep','start_walking':'StartWalking','start_idle':'StartIdle','refresh':'Refresh'}[c['decision']]
 pp=lambda v:'Vec3::new('+', '.join(f'f({x})'for x in v)+')'
 s+=f'''        // Original resolved phase case{i}.
        let mut state = {st};
        let goal = SheepGoal {{ position: {pp(c['goal'])}, goal_tag: {int(c['active'])} }};
        assert_eq!(advance_move_state(&mut state, f({c['dt']}), &goal, {pp(c['position'])}), GoalAction::{action});
'''
 post=c['movement_after_bits']
 if c['walking']:
  s+=f'''        match state {{
            SheepMoveState::Walking {{ timeout_s, goal_change_timeout, force_reach_goal }} => {{
                assert_eq!(timeout_s.to_bits(), {post[2]});
                assert_eq!(goal_change_timeout.to_bits(), {post[3]});
                assert_eq!(force_reach_goal, {str(bool(post[1])).lower()});
            }}
            _ => panic!("phase changed the movement discriminant before its transition"),
        }}
'''
 else:s+=f'        assert!(matches!(state, SheepMoveState::Idle {{ timeout_s }} if timeout_s.to_bits() == {post[2]}));\n'
s+='''    }
    #[test]
    fn projected_goal_direction_matches_original_b_instruction_fixtures() {
'''
for i,c in enumerate(native['direction_cases']):
 q=', '.join(f'f({x})'for x in c['quaternion']);d=', '.join(f'f({x})'for x in c['disk']);expected=', '.join(f'f({x})'for x in c['direction']);scale=c['scale'];active=scale==1061997773
 s+=f'''        // Existing glam implementation is reused; native fixture case{i}.
        let actual = sheep_goal_direction(Quat::from_xyzw({q}), Vec2::new({d}), {str(active).lower()});
        let expected = Vec3::new({expected});
        assert!((actual - expected).abs().max_element() < 2.0e-6, "case{i}: {{actual:?}} vs {{expected:?}}");
'''
s+='''    }
}
'''
save(rel,s)
# Build per-owner patches; core and components/tests are explicitly root integration adapters.
groups={'owned-behavior.patch':['games_test/tiny-glade/crates/systems/critters/src/sheep/sheep_behavior.rs'],'owned-unstuck.patch':['games_test/tiny-glade/crates/systems/critters/src/sheep/sheep_unstuck.rs'],'integration-owner.patch':[x for x in candidates if '/sheep/sheep_behavior.rs'not in x and '/sheep/sheep_unstuck.rs'not in x]}
for name,files in groups.items():
 patch=''.join(''.join(difflib.unified_diff(sources[rel].splitlines(True),candidates[rel].splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))for rel in files);(OUT/name).write_text(patch)
(OUT/'manifest.json').write_text(json.dumps({'target':'B','files':[{'source':rel,'base_sha256':hashlib.sha256(sources[rel].encode()).hexdigest(),'candidate_sha256':hashlib.sha256(s.encode()).hexdigest()}for rel,s in candidates.items()],'cargo_run':False,'candidate_executed':False},indent=2)+'\n')
print({'files':len(candidates),'patches':list(groups)})

