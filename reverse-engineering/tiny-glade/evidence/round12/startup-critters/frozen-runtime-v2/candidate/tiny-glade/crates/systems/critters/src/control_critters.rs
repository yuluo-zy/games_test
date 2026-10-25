//! Original inline system140f7de90, business span140f7df84..140f7dfef.
use bevy_ecs::prelude::*;
use country_core::resources::{critters::CritterControl, time::GameTime, time_of_day::TimeOfDay};

pub fn control_critters(
    tod: Res<TimeOfDay>,
    time: Res<GameTime>,
    mut control: ResMut<CritterControl>,
) {
    update(&mut control, tod.day_night_factor, time.delta);
}

fn update(control: &mut CritterControl, factor: f32, delta: f32) {
    control.enabled = factor >= 0.05 && factor < 0.8;
    let rate = if control.enabled { 0.2 } else { -0.3 };
    let value = control.intensity + rate * delta;
    // MAXSS(0,value), MINSS(1,lower): the source operand wins unordered/equal.
    let lower = if value < 0.0 { 0.0 } else { value };
    control.intensity = if 1.0 < lower { 1.0 } else { lower };
}

#[cfg(test)]
mod tests {
    use super::*;
    include!("tests/fixtures/control_original_b.rs");
    #[test]
    fn original_inline_control_arithmetic_324_cases() {
        for [factor, delta, before, expected, enabled, birds] in CONTROL_CASES {
            let mut control = CritterControl {
                intensity: f32::from_bits(before),
                enabled: true,
                birds_enabled: birds != 0,
            };
            update(&mut control, f32::from_bits(factor), f32::from_bits(delta));
            let expected = f32::from_bits(expected);
            if expected.is_nan() {
                assert!(control.intensity.is_nan());
            } else {
                assert_eq!(
                    control.intensity.to_bits(),
                    expected.to_bits(),
                    "factor={factor:x},dt={delta:x},before={before:x}"
                );
            }
            assert_eq!(control.enabled, enabled != 0);
            assert_eq!(control.birds_enabled, birds != 0);
        }
        assert!(CritterControl::default().birds_enabled);
    }
}
