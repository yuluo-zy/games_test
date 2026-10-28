//! Review diagnostics for the current CPU geometry API.
//! These assertions record observed limitations, not desired fixes or GPU acceptance.
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::geometry::instanced_wall::{BrickTransformSsbo, InstancedWall};
use country_core::geometry::sphere::Sphere;
use country_core::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo};
use country_core::render::ssbo::StorageElement;
use country_core::render::ssbo::rhapsody_boundary::{
    BackendError, Buffer, BufferDesc, GraphicsDevice,
};
use country_core::systems::render_loop::draw_passes::draw_callbacks_b::LibView;
use country_core::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    BrickTransformTag, instance_buffer,
};
use std::ops::Range;
use std::panic::{AssertUnwindSafe, catch_unwind};

struct CpuDevice;
impl GraphicsDevice for CpuDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _: &str,
        _: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        Ok(Buffer {
            byte_size: desc.size as u64,
            ..Buffer::default()
        })
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        _: &[u8],
        _: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct SmallBrick(u32);
impl StorageElement for SmallBrick {
    const NAME: &'static str = "GeometryReviewSmallBrick";
    const BUFFER_LEN: usize = 64 * 32;
}
// SAFETY: Copy, no padding, and BUFFER_LEN == CHUNK_COUNT * 32.
unsafe impl SparseSsbo for SmallBrick {
    const CHUNK_COUNT: usize = 64;
}

#[test]
fn sphere_triangles_face_outward_and_attributes_are_finite() {
    for subdivisions in [0, 1, 5, 79] {
        let mesh = Sphere::new(subdivisions, 2.0).to_mesh().unwrap();
        let p = mesh.positions_without_indices();
        let n = mesh.normals_without_indices();
        assert!(p.iter().all(|v| v.is_finite()));
        assert!(n.iter().all(|v| v.is_finite()));
        assert!(mesh.uvs_without_indices().iter().all(|v| {
            v.is_finite() && (0.0..=1.0).contains(&v.x) && (0.0..=1.0).contains(&v.y)
        }));
        for tri in mesh.indices.chunks_exact(3) {
            let [a, b, c] = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
            assert!((p[b] - p[a]).cross(p[c] - p[a]).dot(n[a]) > 0.0);
        }
    }
}

#[test]
fn upload_replaces_chunks_and_explicit_empty_update_clears_them() {
    let mut buffer = SparseInstanceBuffer::<SmallBrick>::new_storage_buffer(&CpuDevice, None);
    let allocation = buffer.new_allocation();
    let mut wall = InstancedWall::new(7, (0..33).map(SmallBrick).collect());
    wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    assert!(wall.brick_transforms.is_empty());
    assert_eq!(
        buffer.allocation_to_draw_multi_instanced(&allocation),
        [(0, 33)]
    );
    assert_eq!(buffer.data[32], SmallBrick(32));
    wall.update(vec![SmallBrick(99)]);
    wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    assert_eq!(
        buffer.allocation_to_draw_multi_instanced(&allocation),
        [(0, 1)]
    );
    assert_eq!(buffer.data[0], SmallBrick(99));
    wall.update(vec![]);
    wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    assert!(
        buffer
            .allocation_to_draw_multi_instanced(&allocation)
            .is_empty()
    );
    assert_eq!(buffer.free_chunks.read().len(), 64);
}

#[test]
fn repeated_upload_without_update_clears_the_previous_geometry() {
    let mut buffer = SparseInstanceBuffer::<SmallBrick>::new_storage_buffer(&CpuDevice, None);
    let allocation = buffer.new_allocation();
    let mut wall = InstancedWall::new(7, vec![SmallBrick(1)]);
    wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    assert_eq!(
        buffer.allocation_to_draw_multi_instanced(&allocation),
        [(0, 1)]
    );
    wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    assert!(
        buffer
            .allocation_to_draw_multi_instanced(&allocation)
            .is_empty()
    );
}

#[test]
fn missing_writer_panics_after_consuming_the_cpu_payload() {
    let mut wall = InstancedWall::new(7, vec![SmallBrick(1)]);
    assert!(catch_unwind(AssertUnwindSafe(|| wall.upload_to_storage_buffer(None))).is_err());
    assert!(wall.brick_transforms.is_empty());
}

#[test]
fn exhaustion_leaves_a_partial_write_and_no_cpu_retry_payload() {
    let mut buffer = SparseInstanceBuffer::<SmallBrick>::new_storage_buffer(&CpuDevice, None);
    let allocation = buffer.new_allocation();
    let mut wall = InstancedWall::new(7, vec![SmallBrick(1); SmallBrick::BUFFER_LEN + 1]);
    let failure = catch_unwind(AssertUnwindSafe(|| {
        wall.upload_to_storage_buffer(Some(&mut buffer.writer(&allocation)));
    }));
    assert!(failure.is_err());
    assert!(wall.brick_transforms.is_empty());
    assert_eq!(
        buffer.allocation_to_draw_multi_instanced(&allocation),
        [(0, 2048)]
    );
}

#[test]
fn registered_sparse_bricks_are_invisible_to_the_current_draw_instance_view() {
    let library = AssetSsboLibrary::new();
    library.add_new::<BrickTransformSsbo>(&CpuDevice);
    assert!(library.get::<BrickTransformSsbo>().is_some());
    assert!(instance_buffer::<BrickTransformTag>(&LibView(&library)).is_none());
}
