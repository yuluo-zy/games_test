//! Candidate replacement for the final TOD seam write, original B
//! time_of_day @140a741f0..74397. Other TOD inputs remain separate contracts.
use crate::{
    render::{atmosphere::DerivedAtmosphere, directional_light::DirectionalLight},
    resources::time_of_day::TodLightingSnapshot,
};
use glam::Vec3;

/// Both directions are actual suncalc_pos_to_vec outputs of this TOD invocation.
/// This function does not synthesize lighting, a TOD sequence, or a camera.
pub fn write_derived_atmosphere(
    output: &mut DerivedAtmosphere,
    light: &mut DirectionalLight,
    snapshot: &TodLightingSnapshot,
    sun: Vec3,
    moon: Vec3,
    continuous: f32,
    previous: f32,
    weather: f32,
) {
    // Existing SunSkyColors a/b/c preserve twelve contiguous floats; original
    // serde fields are four Vec3. Project without changing resource identity.
    let a = snapshot.sun_sky.a.to_array();
    let b = snapshot.sun_sky.b.to_array();
    let c = snapshot.sun_sky.c.to_array();
    let sun_tint = Vec3::new(a[0], a[1], a[2]);
    let moon_tint = Vec3::new(a[3], b[0], b[1]);
    let sky_low = Vec3::new(b[2], b[3], c[0]);
    let sky_high = Vec3::new(c[1], c[2], c[3]);

    // Original cmp: ucomiss xmm0(0.5),xmm6(continuous); jbe high.
    // Preserve the unordered/NaN destination: NaN also takes this high branch.
    let (selected, fake_sun, fake_moon) = if !(0.5 > continuous) {
        light.set_direction_towards_light(moon);
        let twice = continuous + -0.5;
        let weight = (1.0 - previous) * (twice + twice);
        (moon_tint * weight, sun_tint, moon_tint * (1.0 - weight))
    } else {
        light.set_direction_towards_light(sun);
        let weight = (1.0 - previous) * (1.0 - (continuous + continuous));
        (sun_tint * weight, sun_tint * (1.0 - weight), moon_tint)
    };
    *output = DerivedAtmosphere {
        sun_tint: selected,
        fake_light_color: fake_sun,
        fake_light_dir: sun,
        fake_light2_color: fake_moon,
        fake_light2_dir: moon,
        sky_low_color: sky_low,
        sky_high_color: sky_high,
        fog_thickness: snapshot.base_fog_thickness + weather,
        fog_tint: snapshot.fog_tint,
        dir_to_sun: sun,
        dir_to_moon: moon,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::atmosphere::SunSkyColors;
    use glam::Vec4;

    #[test]
    fn native_low_and_high_branches_select_different_lights_and_split_their_tints() {
        let snapshot = TodLightingSnapshot {
            sun_sky: SunSkyColors {
                a: Vec4::new(1.0, 2.0, 3.0, 4.0),
                b: Vec4::new(5.0, 6.0, 7.0, 8.0),
                c: Vec4::new(9.0, 10.0, 11.0, 12.0),
            },
            base_fog_thickness: 0.125,
            fog_tint: Vec3::new(0.25, 0.5, 1.0),
            speed_bias: 0.0,
        };
        let sun = Vec3::new(1.0, 2.0, 3.0);
        let moon = Vec3::new(-2.0, 3.0, 1.0);
        let mut light = DirectionalLight::look_at(Vec3::new(3.0, 5.0, 2.0), Vec3::ZERO);
        let mut out = DerivedAtmosphere::default();
        write_derived_atmosphere(&mut out, &mut light, &snapshot, sun, moon, 0.25, 0.0, 0.25);
        assert_eq!(out.sun_tint, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(out.fake_light_color, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(out.fake_light2_color, Vec3::new(4.0, 5.0, 6.0));
        assert!((light.forward + sun.normalize()).length() < 1e-6);
        write_derived_atmosphere(&mut out, &mut light, &snapshot, sun, moon, 0.75, 0.0, 0.25);
        assert_eq!(out.sun_tint, Vec3::new(2.0, 2.5, 3.0));
        assert_eq!(out.fake_light_color, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(out.fake_light2_color, Vec3::new(2.0, 2.5, 3.0));
        assert!((light.forward + moon.normalize()).length() < 1e-6);
        assert_eq!(out.sky_low_color, Vec3::new(7.0, 8.0, 9.0));
        assert_eq!(out.sky_high_color, Vec3::new(10.0, 11.0, 12.0));
        assert_eq!(out.fog_thickness, 0.375);
        assert_eq!(out.dir_to_sun, sun);
        assert_eq!(out.dir_to_moon, moon);
    }
}
