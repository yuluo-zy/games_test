//! Candidate append to render::atmosphere. Original B layout, all 31 floats.
use bevy_ecs::resource::Resource;
use glam::Vec3;

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct DerivedAtmosphere {
    pub sun_tint: Vec3,          // +00
    pub fake_light_color: Vec3,  // +0c
    pub fake_light_dir: Vec3,    // +18
    pub fake_light2_color: Vec3, // +24
    pub fake_light2_dir: Vec3,   // +30
    pub sky_low_color: Vec3,     // +3c
    pub sky_high_color: Vec3,    // +48
    pub fog_thickness: f32,      // +54
    pub fog_tint: Vec3,          // +58
    pub dir_to_sun: Vec3,        // +64
    pub dir_to_moon: Vec3,       // +70
}

// Default is original startup only: insert_resources @140940f83..fb9 zeroes
// 0x7c bytes. Runtime consumer must run the original TOD producer first.
const _: () = assert!(core::mem::size_of::<DerivedAtmosphere>() == 0x7c);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, fog_thickness) == 0x54);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, dir_to_sun) == 0x64);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, dir_to_moon) == 0x70);
