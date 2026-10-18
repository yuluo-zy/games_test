//! Original B Weather resource. Native default inline @140940f05..f17,
//! ComponentInfo@1413d4fb0 size16/align4; writer@140ae7cxx.
use bevy_ecs::resource::Resource;

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct Weather {
    /// +00. Added to TOD base_fog_thickness; loaded from GladeSettings+08.
    pub weather_fog: f32,
    /// +04. Loaded from GladeSettings+0c.
    pub wind_strength: f32,
    /// +08. Original default/startup writer sets 0; remaining name unknown.
    pub field_8: f32,
    /// +0c. Initial winter=1/nonwinter=0, weather_system updates in winter.
    pub snow_amount: f32,
}
impl Default for Weather {
    fn default() -> Self {
        Self {
            weather_fog: 0.0,
            wind_strength: 1.0,
            field_8: 0.0,
            snow_amount: 0.0,
        }
    }
}
const _: () = assert!(core::mem::size_of::<Weather>() == 16);
const _: () = assert!(core::mem::align_of::<Weather>() == 4);
