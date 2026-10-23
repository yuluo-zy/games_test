//! CPU declaration/submission boundary for shared physical buffer versions.
//! This gate is not a GPU completion fence: submitted range dependencies still
//! belong to rhapsody. It prevents a producer from submitting N+1 writes before
//! N has registered its reads.
//!
//! Producers never wait for it (PARALLEL_REFACTOR.md 任务 3): while another
//! thread holds the read lease, a write is handed over as an owned job and the
//! lease's release runs every job in FIFO order before reopening the boundary.
//! A later direct write therefore still follows all earlier deferred ones, and
//! render-local writes (the lease owner's own) stay ordered before them.
//! Deferred bytes are bounded. An exhausted budget makes a non-pool thread
//! wait for the lease; a shared task-pool worker (`tg-compute`) never waits
//! here, because the render thread may need that pool to finish the frame
//! whose lease it waits for. It enqueues beyond the budget and is counted.
//! That removes the former ECS_COLLECT_GATE between Update and Collect.
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::{collections::VecDeque, sync::Arc, thread::ThreadId};

/// Upper bound of owned bytes waiting for a lease release.
pub(crate) const DEFERRED_BYTES: usize = 64 << 20;

pub(crate) type DeferredJob = Box<dyn FnOnce() -> Result<(), String> + Send>;

#[derive(Default)]
struct Deferred {
    jobs: VecDeque<DeferredJob>,
    bytes: usize,
    /// First failure of a job that ran after its producer returned. Reported
    /// by the next write through this boundary, so it is never silently lost.
    failed: Option<String>,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct BoundaryStats {
    pub deferred_jobs: u64,
    pub deferred_bytes: u64,
    pub budget_waits: u64,
    /// Pool-worker writes accepted beyond the budget instead of waiting.
    pub budget_overflows: u64,
}

fn on_shared_pool_worker() -> bool {
    std::thread::current().name().is_some_and(|name| name.starts_with("tg-compute"))
}

#[derive(Default)]
pub(crate) struct UploadBoundary {
    owner: Mutex<Option<ThreadId>>,
    released: Condvar,
    deferred: Mutex<Deferred>,
    stats: Mutex<BoundaryStats>,
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
    /// Waiting form; producers use [`Self::write_or_defer`].
    pub(crate) fn write(&self) -> MutexGuard<'_, Option<ThreadId>> {
        let current = std::thread::current().id();
        let mut owner = self.owner.lock();
        while owner.is_some_and(|reader| reader != current) {
            self.released.wait(&mut owner);
        }
        owner
    }

    /// Run `job` now if no other thread holds the read lease, otherwise hand
    /// it over and return immediately. `bytes` is the owned payload it keeps.
    pub(crate) fn write_or_defer(
        &self,
        bytes: usize,
        job: impl FnOnce() -> Result<(), String> + Send + 'static,
    ) -> Result<(), String> {
        let current = std::thread::current().id();
        let mut owner = self.owner.lock();
        if let Some(error) = self.deferred.lock().failed.take() {
            return Err(error);
        }
        if owner.is_some_and(|reader| reader != current) {
            let mut deferred = self.deferred.lock();
            let within = deferred.bytes.saturating_add(bytes) <= DEFERRED_BYTES;
            if within || on_shared_pool_worker() {
                deferred.bytes = deferred.bytes.saturating_add(bytes);
                deferred.jobs.push_back(Box::new(job));
                let mut stats = self.stats.lock();
                stats.deferred_jobs += 1;
                stats.deferred_bytes += bytes as u64;
                if !within { stats.budget_overflows += 1; }
                return Ok(());
            }
            drop(deferred);
            self.stats.lock().budget_waits += 1;
            while owner.is_some_and(|reader| reader != current) {
                self.released.wait(&mut owner);
            }
        }
        let result = job();
        drop(owner);
        result
    }

    pub(crate) fn stats(&self) -> BoundaryStats {
        *self.stats.lock()
    }

    #[cfg(test)]
    pub(crate) fn pending_jobs(&self) -> usize {
        self.deferred.lock().jobs.len()
    }
}

impl Drop for ReadLease {
    fn drop(&mut self) {
        let mut owner = self.0.owner.lock();
        // Producers enqueue under `owner`, so this drains every job accepted
        // during the lease; none can be added until the boundary reopens.
        let jobs = {
            let mut deferred = self.0.deferred.lock();
            deferred.bytes = 0;
            std::mem::take(&mut deferred.jobs)
        };
        for job in jobs {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(job))
                .unwrap_or_else(|_| Err("deferred upload job panicked".to_owned()));
            if let Err(error) = result {
                log::error!(target: "tiny_glade::upload", "deferred upload failed: {error}");
                self.0.deferred.lock().failed.get_or_insert(error);
            }
        }
        *owner = None;
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
    fn pool_workers_never_wait_even_beyond_the_deferred_budget() {
        let boundary = Arc::new(UploadBoundary::default());
        let read = boundary.begin_read(std::thread::current().id());
        let writer = boundary.clone();
        std::thread::Builder::new().name("tg-compute (3)".into()).spawn(move || {
            writer.write_or_defer(DEFERRED_BYTES, || Ok(())).unwrap();
            writer.write_or_defer(1, || Ok(())).unwrap();
        }).unwrap().join().unwrap();
        assert_eq!(boundary.pending_jobs(), 2);
        assert_eq!(boundary.stats().budget_overflows, 1);
        drop(read);
        assert_eq!(boundary.pending_jobs(), 0);
    }

    #[test]
    fn producers_never_wait_and_deferred_writes_run_in_order_before_the_boundary_reopens() {
        let boundary = Arc::new(UploadBoundary::default());
        let read = boundary.begin_read(std::thread::current().id());
        let log = Arc::new(Mutex::new(Vec::new()));
        // Render-local (lease owner) writes run immediately.
        let local = log.clone();
        boundary.write_or_defer(0, move || { local.lock().push(0); Ok(()) }).unwrap();
        let writer = boundary.clone();
        let producer = log.clone();
        std::thread::spawn(move || {
            for value in 1..=3 {
                let log = producer.clone();
                writer.write_or_defer(8, move || { log.lock().push(value); Ok(()) }).unwrap();
            }
        }).join().unwrap();
        assert_eq!(*log.lock(), [0]);
        assert_eq!(boundary.pending_jobs(), 3);
        drop(read);
        assert_eq!(*log.lock(), [0, 1, 2, 3]);
        // A failed deferred job is reported by the next write.
        let read = boundary.begin_read(std::thread::current().id());
        let writer = boundary.clone();
        std::thread::spawn(move || writer.write_or_defer(1, || Err("lost".into())).unwrap()).join().unwrap();
        drop(read);
        assert_eq!(boundary.write_or_defer(0, || Ok(())), Err("lost".to_owned()));
        assert_eq!(boundary.write_or_defer(0, || Ok(())), Ok(()));
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
