// Type-check owner candidates against cached canonical workspace dependencies.
// Does not replace central Cargo/integration validation.
extern crate country_core as baseline;
pub use baseline::{render, utils};
pub mod resources {
    pub use baseline::resources::*;
    pub mod walls {
        pub use baseline::resources::walls::*;
        #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs"]
        pub mod decorator_archivist;
        #[path = "D:/game/tiny-glade/crates/country-core/src/resources/walls/decorator_collision.rs"]
        pub mod decorator_collision;
    }
}
pub mod systems {
    pub use baseline::systems::*;
    pub mod collision {
        pub use baseline::systems::collision::*;
        #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/candidate/tiny-glade/crates/country-core/src/systems/collision/world.rs"]
        pub mod world;
    }
}
