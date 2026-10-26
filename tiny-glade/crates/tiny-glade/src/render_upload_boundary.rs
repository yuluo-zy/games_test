//! Bounded ownership hand-off before a frame registers its physical reads.
//! Accepted jobs retain their bytes across transfer-queue pressure. Producers
//! never wait on the read lease or pump GPU work; the render/event loop polls.
use parking_lot::Mutex;
use std::{collections::VecDeque, sync::Arc, thread::ThreadId};

pub(crate) const DEFERRED_BYTES: usize = 64 << 20;
pub(crate) const CAPACITY: &str = "owned upload admission capacity exhausted";
pub(crate) fn backend_error(error: String) -> rhapsody::Error {
    if error == CAPACITY { rhapsody::Error::exhausted(CAPACITY) }
    else { rhapsody::Error::invalid_usage(error) }
}
pub(crate) fn job_error(error: rhapsody::Error) -> String {
    if matches!(error, rhapsody::Error::Exhausted(_) | rhapsody::Error::Timeout) { CAPACITY.to_owned() }
    else { error.to_string() }
}
type DeferredJob = Box<dyn FnMut() -> Result<(), String> + Send>;
#[derive(Default)]
struct Deferred {
    jobs: VecDeque<(usize, bool, DeferredJob)>,
    bytes: usize,
    failed: Option<String>,
}
#[derive(Default, Clone, Copy, Debug)]
pub struct BoundaryStats {
    pub deferred_jobs: u64,
    pub deferred_bytes: u64,
    /// Rejected admissions; no accepted job exceeds the budget.
    pub budget_overflows: u64,
}
#[derive(Default)]
pub(crate) struct UploadBoundary {
    owner: Mutex<Option<ThreadId>>,
    deferred: Mutex<Deferred>,
    stats: Mutex<BoundaryStats>,
}
pub(crate) struct ReadLease(Arc<UploadBoundary>);
impl UploadBoundary {
    pub(crate) fn begin_read(self: &Arc<Self>, thread: ThreadId) -> ReadLease {
        let mut owner = self.owner.lock();
        assert!(owner.is_none(), "previous CPU read boundary is still active");
        *owner = Some(thread);
        ReadLease(self.clone())
    }
    fn drain(deferred: &mut Deferred, render_only: bool) {
        while !deferred.jobs.is_empty() {
            if render_only && !deferred.jobs.front().unwrap().1 { break; }
            let (bytes, local, mut job) = deferred.jobs.pop_front().unwrap();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut job))
                .unwrap_or_else(|_| Err("deferred upload job panicked".to_owned()));
            if matches!(&result, Err(error) if error == CAPACITY) {
                deferred.jobs.push_front((bytes, local, job));
                break;
            }
            deferred.bytes -= bytes;
            if let Err(error) = result { deferred.failed.get_or_insert(error); }
        }
    }
    pub(crate) fn write_or_defer<F>(&self, bytes: usize, job: F) -> Result<(), String>
    where F: FnMut() -> Result<(), String> + Send + 'static {
        let current = std::thread::current().id();
        let owner = self.owner.lock();
        let mut deferred = self.deferred.lock();
        if let Some(error) = deferred.failed.take() { return Err(error); }
        let charge = bytes.saturating_add(std::mem::size_of::<F>())
            .saturating_add(std::mem::size_of::<(usize, bool, DeferredJob)>());
        if deferred.bytes.saturating_add(charge) > DEFERRED_BYTES || deferred.jobs.len() >= 4096 {
            self.stats.lock().budget_overflows += 1;
            return Err(CAPACITY.to_owned());
        }
        deferred.bytes += charge;
        let local = *owner == Some(current);
        if local {
            let at = deferred.jobs.iter().position(|entry| !entry.1).unwrap_or(deferred.jobs.len());
            deferred.jobs.insert(at, (charge, true, Box::new(job)));
        } else { deferred.jobs.push_back((charge, false, Box::new(job))); }
        let mut stats = self.stats.lock();
        stats.deferred_jobs += 1;
        stats.deferred_bytes += charge as u64;
        drop(stats);
        if owner.is_none() || *owner == Some(current) { Self::drain(&mut deferred, owner.is_some()); }
        deferred.failed.take().map_or(Ok(()), Err)
    }
    pub(crate) fn poll_deferred(&self) -> Result<bool, String> {
        let owner = self.owner.lock();
        if owner.is_some_and(|reader| reader != std::thread::current().id()) { return Ok(false); }
        let mut deferred = self.deferred.lock();
        Self::drain(&mut deferred, owner.is_some());
        if let Some(error) = deferred.failed.take() { return Err(error); }
        Ok(if owner.is_some() { deferred.jobs.iter().all(|entry| !entry.1) } else { deferred.jobs.is_empty() })
    }
    pub(crate) fn stats(&self) -> BoundaryStats { *self.stats.lock() }
    pub(crate) fn report_failure(&self, error: String) {
        self.deferred.lock().failed.get_or_insert(error);
    }
    #[cfg(test)]
    pub(crate) fn pending_jobs(&self) -> usize { self.deferred.lock().jobs.len() }
}
impl Drop for ReadLease {
    fn drop(&mut self) {
        let mut owner = self.0.owner.lock();
        UploadBoundary::drain(&mut self.0.deferred.lock(), false);
        *owner = None;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_capacity_retry_preserves_fifo_and_render_local_order() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let boundary = Arc::new(UploadBoundary::default());
        let ready = Arc::new(AtomicBool::new(false));
        let log = Arc::new(Mutex::new(Vec::new()));
        let read = boundary.begin_read(std::thread::current().id());
        let (writer, gate, output) = (boundary.clone(), ready.clone(), log.clone());
        std::thread::spawn(move || {
            writer.write_or_defer(8, move || {
                if !gate.load(Ordering::Acquire) { return Err(CAPACITY.to_owned()); }
                output.lock().push(1); Ok(())
            }).unwrap();
        }).join().unwrap();
        let output = log.clone();
        boundary.write_or_defer(8, move || { output.lock().push(0); Ok(()) }).unwrap();
        assert_eq!(*log.lock(), [0], "next-frame writes stay sealed during render-local work");
        drop(read);
        assert!(!boundary.poll_deferred().unwrap());
        let output = log.clone();
        boundary.write_or_defer(8, move || { output.lock().push(2); Ok(()) }).unwrap();
        assert_eq!(*log.lock(), [0]);
        ready.store(true, Ordering::Release);
        assert!(boundary.poll_deferred().unwrap());
        assert_eq!(*log.lock(), [0, 1, 2]);
    }

    #[test]
    fn deferred_failure_reaches_the_frame_without_another_write() {
        let boundary = Arc::new(UploadBoundary::default());
        let read = boundary.begin_read(std::thread::current().id());
        let writer = boundary.clone();
        std::thread::spawn(move || writer.write_or_defer(8, || Err("lost upload".into())).unwrap())
            .join().unwrap();
        drop(read);
        assert_eq!(boundary.poll_deferred(), Err("lost upload".into()));
    }

    #[test]
    fn cancelled_packet_releases_boundaries_before_world_resource_destructors() {
        #[derive(bevy_ecs::resource::Resource)]
        struct LastGpuOwner(Arc<UploadBoundary>);
        impl Drop for LastGpuOwner {
            fn drop(&mut self) {
                assert!(self.0.owner.lock().is_none(), "World dropped while its CPU boundary was still sealed");
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
    fn pool_workers_reject_capacity_without_waiting_or_exceeding_the_budget() {
        let boundary = Arc::new(UploadBoundary::default());
        let read = boundary.begin_read(std::thread::current().id());
        let writer = boundary.clone();
        std::thread::Builder::new().name("tg-compute (3)".into()).spawn(move || {
            writer.write_or_defer(DEFERRED_BYTES - 1024, || Ok(())).unwrap();
            assert_eq!(writer.write_or_defer(2048, || Ok(())), Err(CAPACITY.to_owned()));
        }).unwrap().join().unwrap();
        assert_eq!(boundary.pending_jobs(), 1);
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
    fn cancelled_declaration_releases_owned_producer_jobs() {
        let boundary = Arc::new(UploadBoundary::default());
        let reader = boundary.clone();
        let log = Arc::new(Mutex::new(Vec::new()));
        let written = log.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _read = reader.begin_read(std::thread::current().id());
            let writer = reader.clone();
            std::thread::spawn(move || writer.write_or_defer(8, move || {
                written.lock().push(1); Ok(())
            }).unwrap()).join().unwrap();
            assert_eq!(reader.pending_jobs(), 1);
            panic!("cancel declaration");
        }));
        assert!(result.is_err());
        assert!(boundary.owner.lock().is_none());
        assert_eq!(boundary.pending_jobs(), 0);
        assert_eq!(*log.lock(), [1]);
    }
}
