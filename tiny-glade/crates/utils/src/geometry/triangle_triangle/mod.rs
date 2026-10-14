pub mod cross_intersect;
pub mod triangles_intersect;
mod utils;
use glam::Vec3;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriangleTriangleIntersection {
    Point(Vec3),
    Segment(Vec3, Vec3),
    Coplanar,
    Disjoint,
}
pub use triangles_intersect::triangles_intersect;

#[cfg(test)]
mod review_tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-review-2026-10-13/geometry-curves/extra_triangle_tests.rs"
    ));
}
