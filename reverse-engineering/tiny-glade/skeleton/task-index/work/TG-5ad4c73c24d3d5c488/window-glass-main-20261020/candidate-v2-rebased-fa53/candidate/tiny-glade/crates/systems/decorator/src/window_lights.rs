//! B1407b5980 的真实窗口查询、昼夜相位与最后一条点灯命令。
//! 共享值归 core 所有，装饰物生产者与玻璃更新器使用相同 TypeId。
use bevy_ecs::prelude::*;
use country_core::resources::{events::LightWindowCmd, session::ActiveSession,
    time::GameTime, time_of_day::TimeOfDay,
    window_lights::{LightInfo, WindowLight}};
pub use country_core::resources::window_lights::GlobalWindowLightsControl;
pub use country_core::resources::window_lights::LightInfo as WindowLightState;
pub use utils::time_continuous_lerp;

pub fn lights_on(t: f32, olden: bool) -> bool {
    let lower = if olden { 0.44 } else { 0.51 };
    lower <= t && t < 0.72
}

pub fn window_lights(
    mut control: ResMut<GlobalWindowLightsControl>,
    windows: Query<&WindowLight>,
    time_of_day: Res<TimeOfDay>,
    time: Res<GameTime>,
    session: Res<ActiveSession>,
    mut light_commands: EventReader<LightWindowCmd>,
) {
    // 原版读尽两个未读事件缓冲，仅保留最后一个载荷。
    let latest = light_commands.read().last().map(|command| command.0);
    let on = lights_on(time_of_day.time_of_day, session.is_olden());
    control.lights_on = on;
    // 先枚举既有表项再插入或删除；目标未变的表项也占用原序号。
    // 复用锁定的 fastrand1.8，本地 Rng 只保存此表项的种子。
    for (ordinal, state) in control.lights.values_mut().enumerate() {
        if state.target_on != on as u8 {
            let seed = u64::from(time.frame_count).wrapping_add(ordinal as u64);
            state.timer = 2.0 * fastrand::Rng::with_seed(seed).f32();
            state.target_on = on as u8;
        }
    }
    let mut live = Vec::new();
    for window in &windows {
        live.push(window.key);
        let state = control.lights.entry(window.key).or_insert_with(|| LightInfo::new(on));
        if latest == Some(window.decorator_id) {
            // 原命令只闪亮当前亮度并设置目标，不改实际开关与计时器。
            state.brightness = 1.0;
            state.target_on = 1;
        }
    }
    control.lights.retain(|key, _| live.contains(key));
    for state in control.lights.values_mut() {
        if state.timer > 0.0 {
            let remaining = state.timer - time.delta;
            // 原 MAXSS(value,0) 在 NaN 或相等时同样返回第二操作数。
            state.timer = if remaining > 0.0 { remaining } else { 0.0 };
            if state.timer == 0.0 {
                state.actual_on = state.target_on;
            }
        }
        state.brightness = time_continuous_lerp(state.brightness,
            if state.actual_on != 0 { 1.0 } else { 0.0 }, time.delta, 3.0);
    }
}

/// 仅对应原 decorators_on_glade_unload 中的窗灯资源片段。
/// 原版释放旧表并重建 RandomState，然后清除全局开灯标志。
pub fn window_lights_on_glade_unload(mut control: ResMut<GlobalWindowLightsControl>) {
    *control = GlobalWindowLightsControl::default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::session::{SessionId, SessionMeta};

    fn light_world(phase: f32, delta: f32) -> (World, Schedule) {
        let mut world = World::new();
        world.init_resource::<GlobalWindowLightsControl>();
        world.init_resource::<Events<LightWindowCmd>>();
        world.insert_resource(TimeOfDay { time_of_day: phase, ..Default::default() });
        world.insert_resource(GameTime { delta, frame_count: 17, ..Default::default() });
        world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        let mut schedule = Schedule::default();
        schedule.add_systems(window_lights);
        (world, schedule)
    }

    #[test]
    fn lights_on_uses_normal_and_olden_lower_bounds() {
        assert!(!lights_on(0.5099, false));
        assert!(lights_on(0.51, false));
        assert!(lights_on(0.7199, true));
        assert!(!lights_on(0.72, true));
        assert!(lights_on(0.44, true));
        assert!(!lights_on(0.4399, true));
        assert!(!lights_on(0.44, false), "0.44 olden-only lower bound");
        assert!(!lights_on(f32::NAN, false));
    }

    #[test]
    fn window_light_default_is_off() {
        let s = WindowLightState::default();
        assert_eq!(s.actual_on, 0);
        assert_eq!(s.target_on, 0);
        assert_eq!(s.brightness, 0.0);
        assert_eq!(s.timer.to_bits(), 0);
    }

    #[test]
    fn actual_time_resource_query_creation_and_live_removal_drive_the_owner() {
        let (mut world, mut schedule) = light_world(0.6, 0.25);
        world.clear_trackers();
        schedule.run(&mut world);
        assert!(world.get_resource_ref::<GlobalWindowLightsControl>().unwrap().is_changed());
        assert!(world.resource::<GlobalWindowLightsControl>().lights_on);
        let entity = world.spawn(WindowLight { key: 0x1_0000_0061, decorator_id: 61 }).id();
        schedule.run(&mut world);
        let state = world.resource::<GlobalWindowLightsControl>().lights[&0x1_0000_0061];
        assert_eq!((state.actual_on, state.target_on), (1, 1));
        assert_eq!(state.timer, 0.0);
        assert!((state.brightness - 0.527_633_4).abs() < 2.0e-7);
        world.resource_mut::<TimeOfDay>().time_of_day = 0.0;
        schedule.run(&mut world);
        assert!(!world.resource::<GlobalWindowLightsControl>().lights_on);
        world.despawn(entity);
        schedule.run(&mut world);
        assert!(world.resource::<GlobalWindowLightsControl>().lights.is_empty());
    }

    #[test]
    fn frame_plus_actual_map_ordinal_rng_does_not_consume_global_rng() {
        let (mut world, mut schedule) = light_world(0.6, 0.125);
        for key in [9, 21, 53, 87] {
            world.spawn(WindowLight { key, decorator_id: key as u32 });
            world.resource_mut::<GlobalWindowLightsControl>().lights.insert(key, LightInfo::default());
        }
        // 已处于目标状态的表项仍参与序号计数。
        world.resource_mut::<GlobalWindowLightsControl>().lights.get_mut(&53).unwrap().target_on = 1;
        let expected: Vec<_> = world.resource::<GlobalWindowLightsControl>().lights.iter()
            .enumerate().map(|(ordinal, (key, value))| {
                let timer = if value.target_on == 0 {
                    let next = 2.0 * fastrand::Rng::with_seed(17 + ordinal as u64).f32() - 0.125;
                    if next > 0.0 { next } else { 0.0 }
                } else { 0.0 };
                (*key, timer.to_bits())
            }).collect();
        fastrand::seed(0x1234);
        let expected_global = fastrand::Rng::with_seed(0x1234).u64(..);
        schedule.run(&mut world);
        assert_eq!(fastrand::u64(..), expected_global);
        for (key, bits) in expected {
            let state = world.resource::<GlobalWindowLightsControl>().lights[&key];
            assert_eq!(state.target_on, 1);
            assert_eq!(state.timer.to_bits(), bits);
        }
    }

    #[test]
    fn latest_unread_command_flashes_only_its_decorator_then_fades_from_actual() {
        let (mut world, mut schedule) = light_world(0.0, 0.1);
        world.spawn(WindowLight { key: 11, decorator_id: 1 });
        world.spawn(WindowLight { key: 22, decorator_id: 2 });
        world.resource_mut::<Events<LightWindowCmd>>().send(LightWindowCmd(1));
        world.resource_mut::<Events<LightWindowCmd>>().update();
        world.resource_mut::<Events<LightWindowCmd>>().send(LightWindowCmd(2));
        schedule.run(&mut world);
        let a = world.resource::<GlobalWindowLightsControl>().lights[&11];
        let b = world.resource::<GlobalWindowLightsControl>().lights[&22];
        assert_eq!(a.brightness, 0.0);
        assert_eq!((b.actual_on, b.target_on, b.timer), (0, 1, 0.0));
        assert!((b.brightness - 0.740_818_2).abs() < 2.0e-7);
        schedule.run(&mut world);
        let second = world.resource::<GlobalWindowLightsControl>().lights[&22];
        assert_eq!(second.target_on, 0, "read command must not replay");
        assert!(second.brightness < b.brightness);
    }

    #[test]
    fn timer_initial_gate_zero_nan_and_second_operand_nan_behavior_are_preserved() {
        for (timer, delta, expected_timer, expected_actual) in [
            (0.0, 0.1, 0.0, 1), (-0.25, 0.1, -0.25, 1),
            (f32::NAN, 0.1, f32::NAN, 1),
            (0.25, 0.1, 0.15, 1), (0.1, 0.25, 0.0, 0),
            (0.1, f32::NAN, 0.0, 0),
        ] {
            let (mut world, mut schedule) = light_world(0.0, delta);
            world.spawn(WindowLight { key: 9, decorator_id: 9 });
            world.resource_mut::<GlobalWindowLightsControl>().lights.insert(9,
                LightInfo { brightness: 0.5, timer, actual_on: 1, target_on: 0 });
            schedule.run(&mut world);
            let state = world.resource::<GlobalWindowLightsControl>().lights[&9];
            assert_eq!(state.actual_on, expected_actual);
            if expected_timer.is_nan() { assert!(state.timer.is_nan()); }
            else { assert!((state.timer - expected_timer).abs() < 2.0e-8); }
            if expected_timer == 0.0 { assert_eq!(state.timer.to_bits(), 0); }
        }
    }

    #[test]
    fn duplicate_entity_keys_share_one_state_and_remove_only_after_last_window() {
        let (mut world, mut schedule) = light_world(0.0, 0.0);
        let a = world.spawn(WindowLight { key: 81, decorator_id: 1 }).id();
        let b = world.spawn(WindowLight { key: 81, decorator_id: 2 }).id();
        schedule.run(&mut world);
        assert_eq!(world.resource::<GlobalWindowLightsControl>().lights.len(), 1);
        assert_eq!(world.resource::<GlobalWindowLightsControl>().lights[&81].brightness, 0.0);
        world.despawn(a); schedule.run(&mut world);
        assert_eq!(world.resource::<GlobalWindowLightsControl>().lights.len(), 1);
        world.despawn(b); schedule.run(&mut world);
        assert!(world.resource::<GlobalWindowLightsControl>().lights.is_empty());
    }

    #[test]
    fn actual_owner_unload_fragment_resets_before_a_reused_key() {
        let (mut world, mut schedule) = light_world(0.6, 0.25);
        world.spawn(WindowLight { key: 71, decorator_id: 71 });
        schedule.run(&mut world);
        assert!(world.resource::<GlobalWindowLightsControl>().lights[&71].brightness > 0.0);
        let mut unload = Schedule::default(); unload.add_systems(window_lights_on_glade_unload);
        unload.run(&mut world);
        let control = world.resource::<GlobalWindowLightsControl>();
        assert!(control.lights.is_empty() && !control.lights_on);
        world.resource_mut::<TimeOfDay>().time_of_day = 0.0;
        schedule.run(&mut world);
        assert_eq!(world.resource::<GlobalWindowLightsControl>().lights[&71], LightInfo::new(false));
    }
}
