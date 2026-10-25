#[path = "support/performance_input.rs"]
mod input;
use country_core::render::mesh::{Mesh, ATTR_POSITION, ATTR_NORMAL};
#[test]
fn mesh_input_identity_ignores_hash_iteration_and_source_stamp_but_preserves_actual_geometry() {
    let mut a = Mesh::new("semantic input");
    a.indices = vec![0, 1];
    a.set_attribute(ATTR_POSITION, vec![glam::Vec3::ZERO, glam::Vec3::X]);
    a.set_attribute(ATTR_NORMAL, vec![glam::Vec3::Y; 2]);
    let mut b = Mesh::new("semantic input");
    b.indices = a.indices.clone();
    b.set_attribute(ATTR_NORMAL, vec![glam::Vec3::Y; 2]);
    b.set_attribute(ATTR_POSITION, vec![glam::Vec3::ZERO, glam::Vec3::X]);
    assert_eq!(input::mesh_digest(&a), input::mesh_digest(&b));
    b.indices.reverse();
    assert_ne!(input::mesh_digest(&a), input::mesh_digest(&b));
    b.indices = a.indices.clone();
    b.set_attribute(ATTR_POSITION, vec![glam::Vec3::new(-0.0, 0.0, 0.0), glam::Vec3::X]);
    assert_ne!(input::mesh_digest(&a), input::mesh_digest(&b));
}
