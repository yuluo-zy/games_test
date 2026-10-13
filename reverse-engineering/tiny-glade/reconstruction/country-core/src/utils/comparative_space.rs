//! utils/comparative_space.rs — `utils::comparative_space::ComparativeSpace`
//! (wp12 wall-domain facade).

use crate::typed_gap::Gap;

pub struct ComparativeSpace {
    pub raw: Gap,
}
impl ComparativeSpace {
    pub fn project_unbounded(&self, _p: glam::Vec3) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}
