//! Original B startup_olden returns a Vec of28-byte entries. The complete
//! game owner is inserted by init_clearing and removed in every other theme.
use bevy_ecs::resource::Resource;
use utils::geometry::Rectangle2d;
/// Native kind byte: rocks0, logs1, stumps2. Rectangle preserves native axis,
/// center and full dimensions; no circles or simplified exclusion masks.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreplacedAutoClutterEntry {
    pub rectangle: Rectangle2d,
    pub kind: u8,
}
#[derive(Resource, Debug)]
pub struct PreplacedAutoClutter(pub Vec<PreplacedAutoClutterEntry>);
