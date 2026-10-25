"""Prepare only unconnected candidates while the rendering build owns live sources."""
from pathlib import Path

root = Path('D:/game')
stage = root / 'tiny-glade/reports/business/2026-10-21-business-module-recovery/proposed'

def save(relative, transform):
    text = (root / relative).read_text(encoding='utf-8')
    output = stage / relative
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(transform(text), encoding='utf-8')

def mei(text):
    old = '            let max = (node.pos + node.size) * scale;'
    assert text.count(old) == 1
    text = text.replace(old, '            // Original two MULPS followed by ADDPS, preserving rounding.\n            let max = node.size * scale + min;')
    old = '            .no_history_recording()\n            .occlude_by_world(true)'
    assert text.count(old) == 1
    return text.replace(old, '''            // Original screen rectangle stores complete bounds and negative z.
            .screen_aabb(rect_min, rect_max,
                f32::from_bits(mei.nodes.get(id).unwrap().z.to_bits() ^ 0x8000_0000))
            .no_history_recording()
            .occlude_by_world(false)''')

save('tiny-glade/crates/systems/toolbar/src/mei/systems.rs', mei)

def aabb(text):
    old = '''pub fn is_inside_bbx(point: Vec2, min: Vec2, max: Vec2) -> bool {
    Aabb2 { min, max }.contains(point)
}'''
    assert text.count(old) == 1
    return text.replace(old, '''/// B 0x14123ffc0: all four comparisons are strict; this private picker
/// predicate intentionally differs from the public closed Aabb2::contains.
pub fn is_inside_bbx(point: Vec2, min: Vec2, max: Vec2) -> bool {
    min.x < point.x && point.x < max.x && min.y < point.y && point.y < max.y
}''')

save('tiny-glade/crates/utils/src/geometry/aabb.rs', aabb)

def plane(text):
    start = text.index('        let denominator =')
    text = text[:start] + '''        // B 0x141280443..485: retain the two original SIMD reduction orders.
        let direction = ray.direction;
        let normal = self.normal;
        let offset = self.point - ray.origin;
        let denominator = normal.z * direction.z
            + (normal.y * direction.y + normal.x * direction.x);
        let numerator = normal.z * offset.z
            + (normal.x * offset.x + normal.y * offset.y);
        let t = numerator / denominator;
        // Native UCOMISS min_t,t; JA rejects only ordered min_t > t.
        // Unordered and positive-infinite results retain the original payload;
        // callers such as PlaneDisc apply their own geometric acceptance.
        if min_t > t {
            None
        } else {
            Some((t, ray.origin + ray.direction * t))
        }
    }
}
'''
    return text

save('tiny-glade/crates/utils/src/geometry/plane.rs', plane)

def plane_review(text):
    start = text.index('#[test]\nfn planes_reject_parallel_and_nonfinite_hits()')
    end = text.index('\n#[test]', start + 8)
    return text[:start] + '''#[test]
fn planes_preserve_original_ordered_minimum_test() {
    // Original 225B function uses UCOMISS/JA, not finite/parallel guards.
    let plane = Plane { point: Vec3::ZERO, normal: Vec3::Y };
    let parallel = |origin| Ray { origin, direction: Vec3::X };
    let coplanar = plane.intersect_ray(parallel(Vec3::ZERO), 0.0).unwrap();
    assert!(coplanar.0.is_nan());
    assert!(coplanar.1.is_nan());
    assert!(plane.intersect_ray(parallel(Vec3::Y), 0.0).is_none());
    let forward_infinite = plane.intersect_ray(parallel(-Vec3::Y), 0.0).unwrap();
    assert_eq!(forward_infinite.0, f32::INFINITY);
    let ray = Ray { origin: Vec3::Y * 3.0, direction: -Vec3::Y * 2.0 };
    assert_eq!(plane.intersect_ray(ray, 0.0), Some((1.5, Vec3::ZERO)));
    assert_eq!(plane.intersect_ray(ray, 2.0), None);
    assert_eq!(plane.intersect_ray(ray, f32::NAN), Some((1.5, Vec3::ZERO)));
}
''' + text[end:]

save('tiny-glade/crates/utils/tests/geometry_review.rs', plane_review)

def wall_geometry_assertion(text):
    old = '        assert_eq!(params.mesh_volume.as_ref().unwrap().len(), 2 * (points - 1));'
    assert text.count(old) == 1
    return text.replace(old, '''        let geometry = params.resolved_geometry();
        let crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(triangles)
            = geometry.as_ref() else { panic!("splitter must register complete triangle mesh") };
        assert_eq!(triangles.len(), 2 * (points - 1));''')

# The existing wall registration builder remains unchanged. Adapt only its test
# to the single owned geometry, avoiding a second copy of every mesh volume.
save('tiny-glade/crates/country-core/src/systems/wall/wall_splitter.rs', wall_geometry_assertion)
print('Staged MEI geometry plus two original-evidenced helper corrections; live sources untouched.')
