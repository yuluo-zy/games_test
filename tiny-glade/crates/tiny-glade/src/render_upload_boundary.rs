//! CPU declaration/submission boundary for shared physical buffer versions.
//! This gate is not a GPU completion fence: submitted range dependencies still
//! belong to rhapsody. It prevents a producer from submitting N+1 writes before
//! N has registered its reads. Unrelated CPU simulation can continue.
use parking_lot::{Condvar, Mutex, MutexGuard};

/// Temporary shared-pool admission rule until all physical writes are moved
/// behind owned upload messages. Both sides use try_lock: Collect falls back
/// inline during Update, and a new Update yields to the OS during Collect.
pub(crate) static ECS_COLLECT_GATE: Mutex<()> = Mutex::new(());
use std::{sync::Arc, thread::ThreadId};

#[derive(Default)]
pub(crate) struct UploadBoundary {
    owner: Mutex<Option<ThreadId>>,
    released: Condvar,
}

pub(crate) struct ReadLease(Arc<UploadBoundary>);

impl UploadBoundary {
    pub(crate) fn begin_read(self: &Arc<Self>, thread: ThreadId) -> ReadLease {
        let mut owner = self.owner.lock();
        assert!(
            owner.is_none(),
            "previous CPU read boundary is still active"
        );
        *owner = Some(thread);
        ReadLease(self.clone())
    }

    /// Keep the lock through publication so begin_read cannot overtake a write
    /// which already passed the boundary. Render-local writes are allowed.
    pub(crate) fn write(&self) -> MutexGuard<'_, Option<ThreadId>> {
        let current = std::thread::current().id();
        let mut owner = self.owner.lock();
        while owner.is_some_and(|reader| reader != current) {
            self.released.wait(&mut owner);
        }
        owner
    }
}

impl Drop for ReadLease {
    fn drop(&mut self) {
        *self.0.owner.lock() = None;
        self.0.released.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn cancelled_packet_releases_boundaries_before_world_resource_destructors() {
        #[derive(bevy_ecs::resource::Resource)]
        struct LastGpuOwner(Arc<UploadBoundary>);
        impl Drop for LastGpuOwner {
            fn drop(&mut self) {
                assert!(self.0.owner.lock().is_none(), "World dropped while its CPU boundary was still sealed");
                drop(self.0.write());
            }
        }
        let buffers = Arc::new(UploadBoundary::default());
        let textures = Arc::new(UploadBoundary::default());
        let worker = std::thread::spawn(|| std::thread::current().id()).join().unwrap();
        let mut side = crate::render_thread::RenderSide::default();
        side.read_boundary = Some([buffers.begin_read(worker), textures.begin_read(worker)]);
        side.world.insert_resource(LastGpuOwner(buffers));
        drop(side);
    }

    #[test]
    fn next_frame_publication_waits_for_reads_but_render_local_writes_do_not() {
        let boundary = Arc::new(UploadBoundary::default());
        let read = boundary.begin_read(std::thread::current().id());
        drop(boundary.write());
        let (entered, waiting) = channel();
        let (wrote, completed) = channel();
        let writer = boundary.clone();
        let worker = std::thread::spawn(move || {
            entered.send(()).unwrap();
            let _publication = writer.write();
            wrote.send(()).unwrap();
        });
        waiting.recv().unwrap();
        assert!(completed.try_recv().is_err());
        drop(read);
        completed
            .recv_timeout(std::time::Duration::from_secs(5))
            .unwrap();
        worker.join().unwrap();
    }

    #[test]
    fn cancelled_declaration_releases_waiting_producers() {
        let boundary = Arc::new(UploadBoundary::default());
        let reader = boundary.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _read = reader.begin_read(std::thread::current().id());
            panic!("cancel declaration");
        }));
        assert!(result.is_err());
        let writer = std::thread::spawn(move || {
            drop(boundary.write());
        });
        writer.join().unwrap();
    }
}
