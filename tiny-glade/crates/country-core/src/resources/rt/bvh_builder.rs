//! B asynchronous full BVH rebuild. The one pending request and one completed
//! result preserve overwrite/take ownership. Host stop/join only bounds cleanup.
use super::{DirtyCow, InputTriangle, RtBvhBuilderGlueResource, RtMesh, RtMeshHandle, RtWorld};
use glam::Vec3;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
pub struct RtBvhBuildRequest {
    /// Both original Some(false) and Some(true) rebuild the complete snapshot.
    pub topology_changed: bool,
    pub meshes: HashMap<RtMeshHandle, DirtyCow<RtMesh>>,
}

/// Original consume clones the real Arc, carries its old dirty bit into the
/// snapshot, and clears the authoritative dirty bit before the next frame.
pub fn request_rebuild(
    world: &mut RtWorld,
    glue: &RtBvhBuilderGlueResource,
    external_update: bool,
) -> bool {
    world.topology_dirty |= external_update;
    if !world.topology_dirty && !world.meshes.values().any(DirtyCow::is_dirty) {
        return false;
    }
    let topology_changed = std::mem::take(&mut world.topology_dirty);
    let meshes = world
        .meshes
        .iter()
        .map(|(handle, cow)| {
            let old_dirty = cow.dirty.swap(false, Ordering::Relaxed);
            (
                *handle,
                DirtyCow {
                    inner: cow.inner.clone(),
                    dirty: AtomicBool::new(old_dirty),
                },
            )
        })
        .collect();
    glue.inner.write().expect("RT BVH lock poisoned").request = Some(RtBvhBuildRequest {
        topology_changed,
        meshes,
    });
    true
}

fn transform_position(matrix: &[f32; 16], point: Vec3) -> Vec3 {
    let x = Vec3::new(matrix[0], matrix[1], matrix[2]) * point.x;
    let xy = Vec3::new(matrix[4], matrix[5], matrix[6]) * point.y + x;
    let xyz = Vec3::new(matrix[8], matrix[9], matrix[10]) * point.z + xy;
    xyz + Vec3::new(matrix[12], matrix[13], matrix[14])
}

pub fn collect_input_triangles(request: &RtBvhBuildRequest) -> Vec<InputTriangle> {
    request
        .meshes
        .values()
        .flat_map(|cow| {
            let mesh = &cow.inner;
            mesh.positions.chunks_exact(3).filter_map(move |vertices| {
                let a = transform_position(&mesh.matrix, vertices[0]);
                let b = transform_position(&mesh.matrix, vertices[1]);
                let c = transform_position(&mesh.matrix, vertices[2]);
                (a.is_finite() && b.is_finite() && c.is_finite()).then_some(InputTriangle {
                    a,
                    b,
                    c,
                    user_data: mesh.color,
                })
            })
        })
        .collect()
}

#[derive(bevy_ecs::resource::Resource)]
pub struct RtBvhWorker {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl RtBvhWorker {
    pub fn spawn(glue: RtBvhBuilderGlueResource) -> std::io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = stop.clone();
        let thread = thread::Builder::new()
            .name("bvh_builder_worker".into())
            .spawn(move || bvh_builder_worker(glue, thread_stop))?;
        Ok(Self {
            stop,
            thread: Some(thread),
        })
    }
}

impl Drop for RtBvhWorker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn bvh_builder_worker(glue: RtBvhBuilderGlueResource, stop: Arc<AtomicBool>) {
    while !stop.load(Ordering::Acquire) {
        let request = {
            let mut state = glue.inner.write().expect("RT BVH lock poisoned");
            let request = state.request.take();
            if request.is_some() {
                state.busy = true;
            }
            request
        };
        let Some(request) = request else {
            thread::sleep(Duration::from_millis(1));
            continue;
        };
        let output = super::bvh::build_gpu_bvh(&collect_input_triangles(&request));
        let mut state = glue.inner.write().expect("RT BVH lock poisoned");
        state.completed = Some(output);
        state.busy = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consume_clears_authoritative_dirty_and_keeps_shared_snapshot() {
        let glue = RtBvhBuilderGlueResource::default();
        let mut world = RtWorld::default();
        let handle = RtMeshHandle::from_handle(7);
        world.get_mut(handle).color = 0x123456;
        let original = world.meshes[&handle].inner.clone();
        assert!(request_rebuild(&mut world, &glue, false));
        assert!(!world.meshes[&handle].is_dirty());
        assert!(!request_rebuild(&mut world, &glue, false));
        let state = glue.inner.read().unwrap();
        let snapshot = &state.request.as_ref().unwrap().meshes[&handle];
        assert!(Arc::ptr_eq(&original, &snapshot.inner));
        assert!(snapshot.is_dirty());
    }

    #[test]
    fn affine_transform_filters_only_nonfinite_complete_triangles() {
        let mut mesh = RtMesh::default();
        mesh.matrix[12] = 10.;
        mesh.matrix[13] = 20.;
        mesh.matrix[14] = 30.;
        mesh.positions = vec![
            Vec3::ZERO,
            Vec3::X,
            Vec3::Y,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec3::NAN,
            Vec3::ZERO,
            Vec3::ONE,
            Vec3::ONE,
        ];
        let request = RtBvhBuildRequest {
            topology_changed: false,
            meshes: [(RtMeshHandle::from_handle(1), DirtyCow::new(mesh))]
                .into_iter()
                .collect(),
        };
        let triangles = collect_input_triangles(&request);
        assert_eq!(
            triangles.len(),
            2,
            "degenerate finite triangles remain; incomplete trailing chunk is ignored"
        );
        assert_eq!(triangles[0].a, Vec3::new(10., 20., 30.));
        assert_eq!(triangles[0].b, Vec3::new(11., 20., 30.));
        assert_eq!(triangles[0].c, Vec3::new(10., 21., 30.));
    }

    #[test]
    fn real_worker_publishes_empty_result_after_geometry_is_removed() {
        let glue = RtBvhBuilderGlueResource::default();
        let _worker = RtBvhWorker::spawn(glue.clone()).unwrap();
        let mut world = RtWorld::default();
        let handle = RtMeshHandle::from_handle(3);
        world.get_mut(handle).positions = vec![Vec3::ZERO, Vec3::X, Vec3::Y];
        assert!(request_rebuild(&mut world, &glue, false));
        let await_result = || {
            let deadline = std::time::Instant::now() + Duration::from_secs(10);
            loop {
                if let Some(output) = glue.take_completed() {
                    return output;
                }
                assert!(
                    std::time::Instant::now() < deadline,
                    "real BVH worker timed out"
                );
                thread::sleep(Duration::from_millis(1));
            }
        };
        assert!(!await_result().bvh.is_empty());
        world.remove_mesh(&handle);
        assert!(request_rebuild(&mut world, &glue, false));
        let empty = await_result();
        assert!(empty.triangles.is_empty() && empty.metadata.is_empty() && empty.bvh.is_empty());
        assert!(glue.is_idle());
    }
}
