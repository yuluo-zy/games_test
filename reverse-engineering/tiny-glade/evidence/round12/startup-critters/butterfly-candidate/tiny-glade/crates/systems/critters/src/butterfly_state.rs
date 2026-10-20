//! Original complete state policy141d85655..141d85317.
use country_core::resources::critters::Butterfly;
use glam::{Vec2, Vec3};

pub trait ButterflyEnvironment {
    fn rock(&self, xz: Vec2) -> u32;
    fn water(&self, xz: Vec2) -> f32;
}

#[allow(clippy::too_many_arguments)]
pub fn update_state(
    st: &mut Butterfly,
    position: Vec3,
    velocity: Vec3,
    goal: bool,
    delta: f32,
    camera_mode: u8,
    eye: Vec3,
    cursor: Vec3,
    sheep: &[Vec2],
    environment: &impl ButterflyEnvironment,
    uniform: &mut impl FnMut() -> f32,
) {
    let p = Vec2::new(position.x, position.z);
    let near = if camera_mode == 3 {
        let d = Vec2::new(eye.x, eye.z) - p;
        (d.y * d.y + d.x * d.x).sqrt() < 3.0
    } else {
        let d = p - Vec2::new(cursor.x, cursor.z);
        d.y * d.y + d.x * d.x < 4.0
    };
    let sheep_near = sheep.iter().any(|s| {
        let d = p - *s;
        // JBE continues on unordered as well as >=4.
        d.y * d.y + d.x * d.x < 4.0
    });
    let phase = st.phase - 0.5 * delta;
    st.phase = if phase > 0.0 { phase } else { 0.0 };
    if near || sheep_near {
        st.phase = 1.0;
    }

    let flag;
    // UCOMISS/JBE sends NaN down the same first path as <=0.
    if !(st.dwell > 0.0) {
        st.dwell += delta;
        if st.dwell > 0.0 {
            st.dwell = uniform() * 0.3 + 0.2;
        }
        flag = if st.phase > 0.0 { 4.0 } else { 3.0 };
    } else {
        let offset = (0.5 - st.phase_ref).abs();
        if !(velocity.y <= 0.0) || !(offset >= 0.2) || !(offset < 0.45) {
            flag = if st.phase > 0.0 { 4.0 } else { 3.0 };
        } else {
            st.dwell -= delta;
            if st.dwell <= 0.0 {
                st.dwell = -(uniform() * 1.5 + 0.5);
            }
            flag = if st.phase > 0.0 { 4.0 } else { 1.0 };
        }
    }

    let mut flag = flag;
    match st.state {
        0 => {
            if st.phase_ref <= 0.5 {
                let candidate = (0.5 - st.phase_ref) * 0.1 / delta;
                // The original Rust min policy selects the non-NaN operand.
                flag = candidate.min(flag);
            }
            st.state_t -= delta;
            if !(st.state_t > 0.0) || near || sheep_near {
                st.state = 3;
            }
        }
        1 => {
            st.state = 2;
            st.state_t = 2.5; // original1429cd064, not the dwell-dispatch register
            if st.phase == 0.0
                && goal
                && environment.rock(p) == 0
                && environment.water(p) < f32::from_bits(0x3c23d70a)
            {
                st.state = 1;
                st.state_t = f32::from_bits(0x3f19999a);
                // UCOMISS(0.6,y); JB keeps1 for unordered too.
                if position.y <= 0.6 {
                    st.state_t = uniform() * 10.0 + 5.0;
                    st.state = 0;
                }
            }
        }
        2 => {
            st.state_t -= delta;
            if goal {
                st.state = if st.state_t <= 0.0 && st.phase == 0.0 {
                    1
                } else {
                    2
                };
            } else {
                // MAXSS(t,5): source5 wins NaN and equal.
                st.state_t = if st.state_t > 5.0 { st.state_t } else { 5.0 };
            }
        }
        3 => {
            st.state_t = position.y;
            if position.y >= 1.5 {
                st.state_t = uniform() * 5.0 + 5.0;
                st.state = 2;
            }
        }
        _ => panic!("invalid Butterfly state discriminant"),
    }
    st.phase_ref = (flag * delta + st.phase_ref) % 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct StateCase {
        label: &'static str,
        initial: [u32; 6],
        position: [u32; 3],
        velocity: [u32; 3],
        delta: u32,
        camera_mode: u8,
        eye: [u32; 3],
        cursor: [u32; 3],
        sheep: &'static [[u32; 2]],
        goal: u32,
        water: u32,
        rock: u32,
        random: &'static [u32],
        expected: [u32; 6],
        draws: usize,
        calls: &'static [(&'static str, [u32; 2])],
    }
    struct Environment<'a> {
        case: &'a StateCase,
        calls: RefCell<Vec<(&'static str, [u32; 2])>>,
    }
    impl ButterflyEnvironment for Environment<'_> {
        fn rock(&self, p: Vec2) -> u32 {
            self.calls
                .borrow_mut()
                .push(("rock", [p.x.to_bits(), p.y.to_bits()]));
            self.case.rock
        }
        fn water(&self, p: Vec2) -> f32 {
            self.calls
                .borrow_mut()
                .push(("water", [p.x.to_bits(), p.y.to_bits()]));
            f32::from_bits(self.case.water)
        }
    }
    fn vec3(bits: [u32; 3]) -> Vec3 {
        Vec3::from_array(bits.map(f32::from_bits))
    }
    include!("tests/fixtures/butterfly_state_original_b.rs");
    #[test]
    fn complete_original_state_cfg_cases() {
        for case in STATE_CASES {
            let b = case.initial;
            let mut st = Butterfly {
                state: b[0],
                state_t: f32::from_bits(b[1]),
                goal_timer: f32::from_bits(b[2]),
                dwell: f32::from_bits(b[3]),
                phase: f32::from_bits(b[4]),
                phase_ref: f32::from_bits(b[5]),
            };
            let env = Environment {
                case,
                calls: RefCell::new(Vec::new()),
            };
            let mut draws = 0;
            let sheep: Vec<_> = case
                .sheep
                .iter()
                .map(|p| Vec2::from_array(p.map(f32::from_bits)))
                .collect();
            update_state(
                &mut st,
                vec3(case.position),
                vec3(case.velocity),
                case.goal != 0,
                f32::from_bits(case.delta),
                case.camera_mode,
                vec3(case.eye),
                vec3(case.cursor),
                &sheep,
                &env,
                &mut || {
                    let v = f32::from_bits(case.random[draws % case.random.len()]);
                    draws += 1;
                    v
                },
            );
            let actual = [
                st.state,
                st.state_t.to_bits(),
                st.goal_timer.to_bits(),
                st.dwell.to_bits(),
                st.phase.to_bits(),
                st.phase_ref.to_bits(),
            ];
            for (i, (a, e)) in actual.into_iter().zip(case.expected).enumerate() {
                if i != 0 && f32::from_bits(e).is_nan() {
                    assert!(f32::from_bits(a).is_nan(), "{} field{}", case.label, i);
                } else {
                    assert_eq!(a, e, "{} field{}", case.label, i);
                }
            }
            assert_eq!(draws, case.draws, "{} RNG", case.label);
            assert_eq!(
                &*env.calls.borrow(),
                case.calls,
                "{} environment reads",
                case.label
            );
        }
    }
}
