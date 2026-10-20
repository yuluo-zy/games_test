// Only WidgetId's scalar boundary is represented here; the hash implementation
// and all 25 native goldens are included directly from production source.
extern crate self as country_core;
pub mod systems { pub mod ui { pub mod resources { pub mod ui_state {
    pub struct WidgetId(pub u64);
} } } }
#[path = "D:/game/tiny-glade/crates/systems/decorator/src/decorator_resources.rs"]
mod actual;
