pub use country_core::resources;
mod systems {
    pub use country_core::systems::camera_rig;
    pub mod ui {
        pub mod resources {
            #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/widget_params.rs"]
            pub mod widget_params;
            #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/ui_state.rs"]
            pub mod ui_state;
        }
        #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/ui-picker/proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs"]
        pub mod hover_intersection;
    }
}
