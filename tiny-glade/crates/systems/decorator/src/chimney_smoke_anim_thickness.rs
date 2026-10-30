//! 原B140818df0：从同一World的真实Smoke组件推进厚度并清除离场key。
//! PDB四组参数为GlobalSmokeControl、Query<&mut Smoke>、TimeOfDay、GameTime。
//! 原Query内部的archetype/table迭代布局不是PublicWalls，禁止手填spots替代。

use bevy_ecs::prelude::*;
use country_core::{components::Smoke, resources::{time::GameTime, time_of_day::TimeOfDay}};

const THICKNESS_MAX: f32 = 1.0;

/// 保留现资源身份；原GlobalSmokeControl的唯一权威是key→厚度表。
#[derive(Resource, Clone, Debug, Default)]
pub struct ChimneySmokeThickness {
    pub thickness: hashbrown::HashMap<usize, f32>,
}

/// 昼夜循环镜像资源（I/U）：`TimeOfDayCycle.t` = `TimeOfDay.time_of_day`。
///
/// `decorator_visual/lantern.rs` 直接消费该镜像；系统优先读真实 `TimeOfDay`。
#[derive(Resource, Clone, Debug, Default)]
pub struct TimeOfDayCycle {
    /// 循环值 t ∈ [0,1)（对应 core `TimeOfDay +0x08`）。
    pub t: f32,
}

/// 原14081908b..096先算inactive范围，再取反；NaN无序时inactive=false。
pub fn smoke_is_active(t: f32) -> bool {
    !(t >= 0.72 && t < 0.9)
}

pub fn initial_thickness(t: f32, flag: u8) -> f32 {
    if smoke_is_active(t) && flag == 1 { 1.0 } else { 0.0 }
}

/// 原1408196c4..708；非零flag先把(1-r)加倍，再与r*4相加。
pub fn target_and_sign(flag: u8, active: bool, r: f32) -> (f32, f32) {
    if flag == 0 {
        (r * 3.0 + (1.0 - r), -1.0)
    } else {
        let inverse = 1.0 - r;
        (r * 4.0 + (inverse + inverse), if active { 1.0 } else { -1.0 })
    }
}

/// 原1408197f7..81a：乘、除、加后MAXSS/MINSS保留第二操作数的NaN/负零。
pub fn step_thickness(old: f32, sign: f32, target: f32, delta: f32) -> f32 {
    let mut value = (sign * delta) / target + old;
    if value.is_nan() {
        // 原跨平台契约：只在异常结果时重放SSE的payload优先与新生invalid位型。
        #[cold]
        fn exceptional(old: f32, sign: f32, target: f32, delta: f32) -> f32 {
            fn result(left: f32, right: f32, value: f32) -> f32 {
                if !value.is_nan() { value }
                else if left.is_nan() { f32::from_bits(left.to_bits() | 0x0040_0000) }
                else if right.is_nan() { f32::from_bits(right.to_bits() | 0x0040_0000) }
                else { f32::from_bits(0xffc0_0000) }
            }
            let product = result(sign, delta, sign * delta);
            let quotient = result(product, target, product / target);
            result(quotient, old, quotient + old)
        }
        value = exceptional(old, sign, target, delta);
    }
    value.clamp(0.0, THICKNESS_MAX)
}

/// 原四组SystemParam，先初始化全部组件key、按存活集合retain，再逐组件推进。
/// 仅Vec<key>是原收集器的临时输出；没有第二份Smoke状态或外部注入列表。
pub fn chimney_smoke_anim_thickness(
    mut control: ResMut<ChimneySmokeThickness>,
    mut smoke: Query<&mut Smoke>,
    time_of_day: Res<TimeOfDay>,
    time: Res<GameTime>,
) {
    let active = smoke_is_active(time_of_day.time_of_day);
    let mut live = Vec::new();
    for component in smoke.iter() {
        live.push(component.key);
        control.thickness.entry(component.key)
            .or_insert_with(|| initial_thickness(time_of_day.time_of_day, component.enabled));
    }
    // 原retain闭包使用live slice.contains；不能按厚度>0丢弃仍存活的烟柱。
    control.thickness.retain(|key, _| live.contains(key));
    for mut component in &mut smoke {
        // 原1408196ad只取key低32位喂init_rng；表的key比较仍是完整usize。
        let rng = fastrand::Rng::with_seed(utils::rng::init_rng(component.key as u32));
        let first = rng.f32();
        let random = if component.enabled == 0 { rng.f32() } else { first };
        let (target, sign) = target_and_sign(component.enabled, active, random);
        let previous = control.thickness.get_mut(&component.key)
            .expect("smoke component thickness missing");
        *previous = step_thickness(*previous, sign, target, time.delta);
        component.thickness = *previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;

    #[test]
    fn real_query_uses_full_keys_low_seed_and_removes_despawned_components() {
        let mut world = World::new();
        world.init_resource::<ChimneySmokeThickness>();
        world.init_resource::<TimeOfDay>();
        world.init_resource::<GameTime>();
        world.resource_mut::<TimeOfDay>().time_of_day = 0.1;
        world.resource_mut::<GameTime>().delta = 0.125;
        let first = world.spawn(Smoke { key: 7, enabled: 1, ..Default::default() }).id();
        let second = world.spawn(Smoke { key: 0x1_0000_0007, enabled: 0,
            thickness: 91.0, kind: 1 }).id();
        world.resource_mut::<ChimneySmokeThickness>().thickness.insert(0x1_0000_0007, 1.0);
        world.resource_mut::<ChimneySmokeThickness>().thickness.insert(999, 0.5);
        world.run_system_once(chimney_smoke_anim_thickness).unwrap();
        assert_eq!(world.get::<Smoke>(first).unwrap().thickness, 1.0);
        let rng = fastrand::Rng::with_seed(utils::rng::init_rng(7));
        let _discarded = rng.f32();
        let (target, sign) = target_and_sign(0, true, rng.f32());
        let expected = step_thickness(1.0, sign, target, 0.125);
        assert_eq!(world.get::<Smoke>(second).unwrap().thickness.to_bits(), expected.to_bits());
        assert_eq!(world.resource::<ChimneySmokeThickness>().thickness.len(), 2);
        world.despawn(first);
        world.run_system_once(chimney_smoke_anim_thickness).unwrap();
        assert!(!world.resource::<ChimneySmokeThickness>().thickness.contains_key(&7));
        assert!(world.resource::<ChimneySmokeThickness>().thickness.contains_key(&0x1_0000_0007));
        world.despawn(second);
        world.run_system_once(chimney_smoke_anim_thickness).unwrap();
        assert!(world.resource::<ChimneySmokeThickness>().thickness.is_empty());
    }

    #[test]
    fn native_windows_targets_and_negative_zero_are_preserved() {
        assert!(smoke_is_active(0.0));
        assert!(!smoke_is_active(0.72));
        assert!(smoke_is_active(0.9));
        assert!(smoke_is_active(f32::NAN));
        assert_eq!(initial_thickness(0.1, 1), 1.0);
        assert_eq!(initial_thickness(0.1, 2), 0.0);
        assert_eq!(target_and_sign(0, false, 0.5), (2.0, -1.0));
        assert_eq!(target_and_sign(1, false, 0.5), (3.0, -1.0));
        assert_eq!(step_thickness(-0.0, -1.0, 1.0, 0.0).to_bits(), 0x8000_0000);
    }

    #[test]
    fn original_rosetta_step_preserves_every_nan_payload_and_invalid_sign() {
        for (case, (input, expected)) in crate::decorator_visual::chimney_roof::original_fixture::SMOKE_STEP_CASES.iter().enumerate() {
            let [old, sign, target, delta] = input.map(f32::from_bits);
            assert_eq!(step_thickness(old, sign, target, delta).to_bits(), *expected,
                "original smoke step case {case}");
        }
    }
}
