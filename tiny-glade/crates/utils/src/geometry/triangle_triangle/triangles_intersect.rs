use super::{
    TriangleTriangleIntersection,
    cross_intersect::cross_intersect,
    utils::{degenerate, side},
};
use glam::Vec3;
/// B 0x141fdfd80 distinguishes coplanarity from point/segment intersection.
/// Coplanar does not itself prove the two finite triangles overlap.
pub fn triangles_intersect(a: [Vec3; 3], b: [Vec3; 3]) -> TriangleTriangleIntersection {
    if degenerate(a) || degenerate(b) {
        return TriangleTriangleIntersection::Disjoint;
    }
    let signs = a.map(|p| side(b, p));
    if signs[0] == signs[1] && signs[0] == signs[2] {
        return if signs[0] == 0 {
            TriangleTriangleIntersection::Coplanar
        } else {
            TriangleTriangleIntersection::Disjoint
        };
    }
    cross_intersect(a, b)
}
