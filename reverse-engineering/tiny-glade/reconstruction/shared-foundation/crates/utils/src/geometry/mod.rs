//! B 私有几何规则；glam运算与容器直接复用原依赖。
//! 逐原入口证据：evidence/shared-foundation-2026-10-13/geometry/。
//! 恢复API不宣称原Rust ABI；未恢复的网格UV/三角交叠helper不补造占位输出。
pub mod aabb;
pub mod barycentric;
pub mod circle;
pub mod intersect;
pub mod line_relation;
pub mod plane;
pub mod position_from_mesh_uvs;
pub mod ray;
pub mod rectangle;
pub mod split_heightmap;
pub mod triangle_triangle;
pub use aabb::{Aabb2, Aabb3, RayBoxIntersection};
pub use circle::Circle2d;
pub use plane::Plane;
pub use ray::{Ray, Ray2};
pub use rectangle::{Rectangle2d, Rectangle3d};
/// B 0x14127c970 projects a world-size unit at a given distance into normalized
/// screen-height units. It is a scalar scale factor, not a world-position transform.
pub fn ws_to_cs(distance: f32, projection: glam::Mat4) -> f32 {
    (0.5 / distance) * projection.y_axis.y
}
/// B 0x14127c950 additionally converts normalized screen height to pixels.
pub fn ws_to_ss(distance: f32, projection: glam::Mat4, screen_height: f32) -> f32 {
    ws_to_cs(distance, projection) * screen_height
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod completion {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../evidence/foundation-completion-2026-10-13/geometry/native_tests.rs"));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod native_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/geometry_tests.rs"
    ));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod boundary_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/boundary_tests.rs"
    ));
}
#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod edge_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/edge_tests.rs"
    ));
}
#[cfg(test)]
mod linear_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/linear_tests.rs"
    ));
}
#[cfg(test)]
mod triangle_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/triangle_tests.rs"
    ));
}
