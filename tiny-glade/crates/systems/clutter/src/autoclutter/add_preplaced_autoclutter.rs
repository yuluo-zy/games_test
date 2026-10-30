//! add_preplaced_autoclutter（B 主基线；0x141ce5900，size 1566）。
//!
//! 2026-10-20 按机器码重新还原（`ClutterPlugin` 闭合项 4）：读取的是
//! `country_core::resources::preplaced_autoclutter::PreplacedAutoClutter`（原版条件
//! `resource_exists<PreplacedAutoClutter>` 与本系统的 `Res` 参数是**同一个** core 资源；此前本文件
//! 自带的 crate 局部同名类型是错的，会让预放置永远不生效）。
//!
//! 参数面（FunctionSystem `validate_param_unsafe` 0x140edf4d0，错误串顺序）：
//! `Res<UiSignifierStream>`（函数体未使用）、`Res<PreplacedAutoClutter>`、
//! `EventReader<NewSessionStartedCmd>`、`EventReader<SessionLoadedCmd>`、`ResMut<AutoClutterState>`，
//! 另有一个 `Local<bool>`（0x141ce5900 的 `param_6`，字节）。
//!
//! 控制流（O，fresh 反编译 + 反汇编 0x141ce5c05…）：
//! 1. 先看 `NewSessionStartedCmd` 读者：非空 → 读者清空（游标置队尾）、`Local = false`、返回；
//! 2. 否则看 `SessionLoadedCmd` 读者：非空 → 同样清空、`Local = false`、返回（第一个非空即止，
//!    另一个读者本帧不检查）；
//! 3. 两者皆空：`Local == false` → 置 `true`、返回（静默一帧）；`Local == true` → **每帧**对
//!    `PreplacedAutoClutter` 每个条目（步长 0x1c = `Rectangle2d`(axis@0, center@8, size@0x10) +
//!    `kind: u8`@0x18）`AutoClutterState::add_region`（`reset_autoclutter_for_new_frame` 每帧清区域，
//!    故需逐帧重加；`Local` 保持 `true`）。
//!
//! 每条区域（O 栈布局，`AutoClutterRegion` 对应字段）：
//! - kind = 5；基向量 4×f32 @+0x18 = `(axis.x, axis.y, size.x, size.y)`；
//! - 中心 @+0x28 = `center.xvy(1.0) + x0y(rot) * (-0.5, -0.5, -0.5)`，其中
//!   `rot = (size.x*axis.x - axis.y*size.y, size.x*axis.y + axis.x*size.y)`
//!   （`ADDSUBPS` 复数乘；`xvy` 的 `v` = 常量 0x142925904 = 1.0；`-0.5` = 0x142a171f0 / 0x14295a0e8）；
//! - i32 @+0x34 = 条目下标；u8 @+0x38 = `entry.kind + 2`；u8 @+0x39 = `(!DEV_OVERRIDE_IS_DEMO_MODE) * 5`
//!   （经 0x14362c098 指向 `_DEV_OVERRIDE_IS_DEMO_MODE` 0x143634d90；`tiny_glade::main` 恒写 0 → 5）；
//! - 候选索引 @+0x3a = 0xffff；epoch 由 `add_region` 写；
//! - 键 = `calculate_hash(&("preplaced_clutter", i))`（串 0x142f35fe3，len 0x11）。
//!
//! 未闭合：`utils::calculate_hash` 的确切算法（仓库内全体以 std `DefaultHasher` 近似，见
//! `country_core::render::utils::calculate_hash`），故区域键数值与原版不保证逐位一致；
//! `Res<UiSignifierStream>` 参数未纳入（函数体不使用）。

use bevy_ecs::prelude::*;
use country_core::resources::preplaced_autoclutter::PreplacedAutoClutter;
use country_core::resources::session::{NewSessionStartedCmd, SessionLoadedCmd};
use glam::{Vec2, Vec3};

use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

/// 区域键输入名（O：0x142f35fe3，len 0x11）。
pub const PREPLACED_CLUTTER_NAME: &str = "preplaced_clutter";
/// `_DEV_OVERRIDE_IS_DEMO_MODE`（静态字节 0x143634d90；`tiny_glade::main` 恒写 0）。
const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;

/// add_preplaced_autoclutter（0x141ce5900）。
pub fn add_preplaced_autoclutter(
    preplaced: Res<PreplacedAutoClutter>,
    mut new_session: EventReader<NewSessionStartedCmd>,
    mut session_loaded: EventReader<SessionLoadedCmd>,
    mut state: ResMut<AutoClutterState>,
    mut settled: Local<bool>,
) {
    // O：第一个非空读者即清空并复位 Local；另一个读者不再检查。
    if !new_session.is_empty() {
        new_session.clear();
        *settled = false;
        return;
    }
    if !session_loaded.is_empty() {
        session_loaded.clear();
        *settled = false;
        return;
    }
    // O：静默一帧后才逐帧放置。
    if !*settled {
        *settled = true;
        return;
    }
    for (i, entry) in preplaced.0.iter().enumerate() {
        let rect = entry.rectangle;
        // O：`ADDSUBPS` 复数乘 (size × axis)。
        let rot = Vec2::new(
            rect.size.x * rect.axis.x - rect.axis.y * rect.size.y,
            rect.size.x * rect.axis.y + rect.axis.x * rect.size.y,
        );
        // O：`center.xvy(1.0)` + `x0y(rot) * -0.5`（y 分量 1.0 + (0.0 * -0.5)）。
        let center = Vec3::new(
            rect.center.x + rot.x * -0.5,
            1.0 + 0.0 * -0.5,
            rot.y * -0.5 + rect.center.y,
        );
        let key = hash_preplaced_key(PREPLACED_CLUTTER_NAME, i);
        let region = AutoClutterRegion {
            source: country_core::components::drawable::AutoClutterSource::None,
            kind: 5,
            key: ClutterRegionId(key),
            center,
            basis: [rect.axis, rect.size],
            f34: i as i32,
            b38: entry.kind.wrapping_add(2),
            b39: if DEV_OVERRIDE_IS_DEMO_MODE { 0 } else { 5 },
            // O：候选索引 0xffff。
            candidate_index: ClutterCandidateIndex::NONE,
            epoch: state.epoch,
        };
        state.add_region(ClutterRegionId(key), &region);
    }
}

/// 预放置键（`calculate_hash<tuple<&str,usize>>` 0x141cf70d0 语义）。
pub fn hash_preplaced_key(name: &str, i: usize) -> u64 {
    utils::calculate_hash(&(name, i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::event::Events;
    use country_core::resources::preplaced_autoclutter::PreplacedAutoClutterEntry;
    use utils::geometry::Rectangle2d;

    fn world() -> (World, Schedule) {
        let mut world = World::new();
        world.init_resource::<Events<NewSessionStartedCmd>>();
        world.init_resource::<Events<SessionLoadedCmd>>();
        world.init_resource::<AutoClutterState>();
        world.insert_resource(PreplacedAutoClutter(vec![PreplacedAutoClutterEntry {
            rectangle: Rectangle2d::new(Vec2::new(10.0, 20.0), Vec2::new(4.0, 2.0), Vec2::new(0.0, 1.0)),
            kind: 1,
        }]));
        let mut schedule = Schedule::default();
        schedule.add_systems(add_preplaced_autoclutter);
        (world, schedule)
    }

    fn region_count(world: &World) -> usize {
        world.resource::<AutoClutterState>().regions.len()
    }

    #[test]
    fn reads_core_preplaced_resource_after_one_quiet_frame_and_places_every_frame() {
        let (mut world, mut schedule) = world();
        // 第 1 帧：Local 为 false → 只置位，不放置。
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 0);
        // 第 2 帧：放置。
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 1);
        let key = ClutterRegionId(hash_preplaced_key(PREPLACED_CLUTTER_NAME, 0));
        let region = world.resource::<AutoClutterState>().regions.get(&key).cloned().expect("preplaced region");
        let rect = Rectangle2d::new(Vec2::new(10.0, 20.0), Vec2::new(4.0, 2.0), Vec2::new(0.0, 1.0));
        // rot = (sx*ax - ay*sy, sx*ay + ax*sy)（`ADDSUBPS` 复数乘）。
        let rot = Vec2::new(
            rect.size.x * rect.axis.x - rect.axis.y * rect.size.y,
            rect.size.x * rect.axis.y + rect.axis.x * rect.size.y,
        );
        assert_eq!(region.kind, 5);
        assert_eq!(region.center, Vec3::new(rect.center.x - 0.5 * rot.x, 1.0, rect.center.y - 0.5 * rot.y));
        assert_eq!(region.basis, [rect.axis, rect.size]);
        assert_eq!(region.f34, 0);
        assert_eq!(region.b38, 3, "entry kind + 2");
        assert_eq!(region.b39, 5, "!DEV_OVERRIDE_IS_DEMO_MODE * 5");
        // 区域每帧由 reset 清掉后，本系统必须再次放置（Local 保持 true）。
        world.resource_mut::<AutoClutterState>().regions.clear();
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 1);
    }

    #[test]
    fn session_events_reset_the_gate_and_only_the_first_reader_is_cleared() {
        let (mut world, mut schedule) = world();
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 1);
        world.resource_mut::<AutoClutterState>().regions.clear();

        world.send_event(NewSessionStartedCmd);
        world.send_event(SessionLoadedCmd);
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 0, "事件帧不放置");
        // 下一帧：NewSessionStartedCmd 已清；SessionLoadedCmd 仍未读（原版第一个非空读者即止）→ 再复位一次。
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 0);
        // 之后无事件：静默一帧，再放置。
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 0);
        schedule.run(&mut world);
        assert_eq!(region_count(&world), 1);
    }

    #[test]
    fn plugin_gate_uses_the_core_resource_not_a_local_copy() {
        let (mut world, _) = world();
        assert!(world.contains_resource::<PreplacedAutoClutter>());
        world.remove_resource::<PreplacedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            add_preplaced_autoclutter.run_if(bevy_ecs::schedule::common_conditions::resource_exists::<PreplacedAutoClutter>),
        );
        schedule.run(&mut world); // 缺资源：条件为假，不 panic、不放置
        assert_eq!(region_count(&world), 0);
    }
}
