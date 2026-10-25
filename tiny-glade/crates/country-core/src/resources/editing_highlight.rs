//! B EditingHighlight: three float color channels and enabled byte at +0xc.
//! clear's 13-byte store sequence is 0x14097689c..0x1409768ab.
use bevy_ecs::resource::Resource;
use glam::Vec3;

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct EditingHighlight {
    pub color: Vec3,
    pub enabled: bool,
}

impl EditingHighlight {
    pub fn clear(&mut self) {
        self.color = Vec3::ZERO;
        self.enabled = false;
    }
}
