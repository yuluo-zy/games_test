// Type-check owner candidates against cached canonical workspace dependencies.
// Does not replace central Cargo/integration validation.
extern crate country_core as baseline;
extern crate self as country_core;
pub use baseline::{render, utils};
pub mod resources {
    pub use baseline::resources::*;
    pub mod walls {
        pub use baseline::resources::walls::*;
        #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/v2_archivist_check.rs"]
        pub mod decorator_archivist;
        #[path = "D:/game/tiny-glade/crates/country-core/src/resources/walls/decorator_collision.rs"]
        pub mod decorator_collision;
    }
}
pub mod systems {
    pub use baseline::systems::*;
    pub mod collision {
        pub use baseline::systems::collision::*;
#[path="D:/game/tiny-glade/crates/country-core/src/systems/collision/intersect_meshes.rs"] pub mod intersect_meshes;
        #[path = "D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/frozen-v2/candidate/tiny-glade/crates/country-core/src/systems/collision/world.rs"]
        pub mod world;
    }
}

pub mod apply_decorator_op {
use crate::resources::walls::decorator::{DecoratorInfo,DecoratorExtraInfo,StairExtra};
pub(crate) fn info_to_bytes(info: DecoratorInfo) -> [u8; 72] {
    let mut out = [0u8; 72];
    out[0x00..0x08].copy_from_slice(&info.dst_wall.to_le_bytes());
    out[0x08..0x10].copy_from_slice(&info.dst_secondary.to_le_bytes());
    out[0x10..0x18].copy_from_slice(&info.dst_payload.to_le_bytes());
    out[0x18..0x20].copy_from_slice(&info.u18.to_le_bytes());
    out[0x20..0x24].copy_from_slice(&info.u20.to_le_bytes());
    out[0x24..0x28].copy_from_slice(&info.u24.to_le_bytes());
    out[0x28..0x2c].copy_from_slice(&info.u28.to_le_bytes());
    out[0x2c..0x30].copy_from_slice(&info.decorator_id.to_le_bytes());
    out[0x30..0x38].copy_from_slice(&info.u30.to_le_bytes());
    out[0x38..0x3c].copy_from_slice(&info.u38.to_le_bytes());
    out[0x3c..0x40].copy_from_slice(&info.u3c.to_le_bytes());
    out[0x40..0x44].copy_from_slice(&info.u40.to_le_bytes());
    out[0x44] = info.rank;
    out[0x45] = info.u45;
    out[0x46] = info.ty;
    out[0x47] = info.state;
    out
}
pub(crate) fn bytes_to_info(bytes: &[u8; 72]) -> DecoratorInfo {
    let r = |off: usize| u64::from_le_bytes(bytes[off..off + 8].try_into().unwrap());
    let r4 = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
    DecoratorInfo {
        dst_wall: r(0x00),
        dst_secondary: r(0x08),
        dst_payload: r(0x10),
        u18: r(0x18),
        u20: r4(0x20),
        u24: r4(0x24),
        u28: r4(0x28),
        decorator_id: r4(0x2c),
        u30: r(0x30),
        u38: r4(0x38),
        u3c: r4(0x3c),
        u40: r4(0x40),
        rank: bytes[0x44],
        u45: bytes[0x45],
        ty: bytes[0x46],
        state: bytes[0x47],
    }
}
}
#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/frozen-v2/candidate/tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs"] pub mod restore_archived_decorators;
