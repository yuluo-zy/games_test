//! 宿主侧接缝：用 rhapsody 的设备实现 country-core 的 `rhapsody_boundary::GraphicsDevice`。
//!
//! country-core 里的 `Buffer` 是可拷贝的 40 字节值（`id`/`generation` + `byte_size`），对应原版
//! 的托管句柄；槽位表映射到 rhapsody 的 [`ManagedBuffer`]，每次分配发布全局唯一代数，
//! 过期句柄按“不属于本设备/已失效”处理。句柄释放见 [`BufferHost::release`]。
use ash::vk;
use country_core::render::ssbo::rhapsody_boundary::{
    BackendError, Buffer, BufferDesc, GraphicsDevice as CoreGraphicsDevice,
};
use parking_lot::Mutex;
use rhapsody::{
    device::Device, resource::BufferDesc as RhBufferDesc, resource::ManagedBuffer,
    resource::MemoryLocation,
};
use std::{collections::HashMap, ops::Range, sync::{Arc, Weak, atomic::{AtomicUsize, Ordering}}};

// Keep both graph token tag bits free. The unique generation covers all host
// tables and slot reuse; ID-only core bindings retain it as well.
fn next_resource_generation() -> Result<u64, BackendError> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_update(Ordering::Relaxed, Ordering::Relaxed,
        |generation| generation.checked_add(1).filter(|next| *next <= (1 << 30)))
        .map_err(|_| BackendError::invalid_usage("physical resource generation exhausted"))
}

struct Slot {
    generation: u64,
    buffer: Option<Arc<BufferVersion>>,
    cpu_authoritative: bool,
    stable_address: bool,
}

struct BufferVersion {
    buffer: ManagedBuffer,
    pins: AtomicUsize,
    incomplete_upload: Mutex<Option<IncompleteUpload>>,
}
impl BufferVersion {
    fn new(buffer: ManagedBuffer) -> Arc<Self> {
        Arc::new(Self { buffer, pins: AtomicUsize::new(0), incomplete_upload: Mutex::new(None) })
    }
}

const UPLOAD_COVERAGE_BYTES: usize = 1 << 20;
#[derive(Default)]
struct UploadCoverageBudget {
    bytes: AtomicUsize,
    failed: AtomicUsize,
}
struct UploadCoverage {
    ranges: Vec<Range<u64>>,
    budget: Arc<UploadCoverageBudget>,
    charged: usize,
}
impl Drop for UploadCoverage {
    fn drop(&mut self) { self.budget.bytes.fetch_sub(self.charged, Ordering::AcqRel); }
}
struct IncompleteUpload {
    coverage: UploadCoverage,
    error: Arc<BackendError>,
}
impl Drop for IncompleteUpload {
    fn drop(&mut self) { self.coverage.budget.failed.fetch_sub(1, Ordering::AcqRel); }
}
impl UploadCoverage {
    fn covers(&self, required: &Self) -> bool {
        let mut available = self.ranges.iter();
        let mut current = available.next();
        for needed in &required.ranges {
            while current.is_some_and(|range| range.end <= needed.start) { current = available.next(); }
            if !current.is_some_and(|range| range.start <= needed.start && range.end >= needed.end) { return false; }
        }
        true
    }
}

/// The physical owners observed at one extraction boundary. Clones share one
/// explicit pin set; neither persistent cache Arcs nor GPU receipts are pins.
#[derive(Clone)]
pub struct FrameBufferSnapshot(Arc<FrameBufferSnapshotData>);
struct FrameBufferSnapshotData {
    source: Weak<Mutex<Vec<Slot>>>,
    buffers: HashMap<u64, Arc<BufferVersion>>,
    /// Render-local buffers created after extraction are private to this frame.
    /// A missing producer ID must never fall through to the next frame's table.
    local: Mutex<HashMap<u64, ManagedBuffer>>,
}
impl Drop for FrameBufferSnapshotData {
    fn drop(&mut self) {
        for version in self.buffers.values() {
            let previous = version.pins.fetch_sub(1, Ordering::AcqRel);
            debug_assert!(previous > 0, "balanced frame buffer pin");
        }
    }
}
impl FrameBufferSnapshot {
    pub fn buffer_count(&self) -> usize { self.0.buffers.len() }
}

const COW_MAX_RETIRED_BYTES: u64 = 256 << 20;
const COW_MAX_RETIRED_BUFFERS: usize = 64;
struct RetiredBuffer {
    owner: Weak<rhapsody::resource::Buffer>,
    bytes: u64,
}
struct CowBudget {
    retired: Vec<RetiredBuffer>,
    max_bytes: u64,
    max_buffers: usize,
    forks: u64,
    fallbacks: u64,
}
impl Default for CowBudget {
    fn default() -> Self {
        Self { retired: Vec::new(), max_bytes: COW_MAX_RETIRED_BYTES,
            max_buffers: COW_MAX_RETIRED_BUFFERS, forks: 0, fallbacks: 0 }
    }
}
impl CowBudget {
    fn admits(&mut self, bytes: u64) -> bool {
        // Native graph/command owners, not the CPU snapshot alone, determine
        // when an old allocation has actually retired.
        self.retired.retain(|entry| entry.owner.strong_count() != 0);
        self.retired.len() < self.max_buffers
            && self.retired.iter().map(|entry| entry.bytes).sum::<u64>()
                .checked_add(bytes).is_some_and(|total| total <= self.max_bytes)
    }
}

pub struct BufferHost {
    device: Arc<Device>,
    slots: Arc<Mutex<Vec<Slot>>>,
    uploads: Arc<Mutex<Option<BufferUploads>>>,
    read_boundary: Arc<crate::render_upload_boundary::UploadBoundary>,
    /// Serialize rename/publication with stable-address escapes and release.
    publication: Arc<Mutex<()>>,
    cow_budget: Arc<Mutex<CowBudget>>,
    upload_coverage: Arc<UploadCoverageBudget>,
    frame: Option<FrameBufferSnapshot>,
    #[cfg(test)]
    fail_upload_chunk: Mutex<Option<(usize, bool)>>,
}
struct BufferUploads {
    manager: rhapsody::transfer::TransferManager,
    epoch: u64,
    /// Bytes accepted since the last flush.
    queued_bytes: u64,
}

struct MappedHostWriter {
    buffer: ManagedBuffer,
    boundary: Arc<crate::render_upload_boundary::UploadBoundary>,
}

#[cfg(test)]
mod mapped_upload_tests {
    use super::*;
    use country_core::{
        render::ssbo::MappedStorageBuffer,
        systems::render_loop::resources::critters::FireflySsbo,
    };

    #[test]
    fn typed_mapped_upload_owns_memory_and_obeys_the_queued_frame_boundary() {
        let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        let host = BufferHost::new(device);
        let mut storage = MappedStorageBuffer::<FireflySsbo>::new_storage_buffer(&host, None);
        let native = host.managed_buffer(&storage.buffer).unwrap();
        let read = host.begin_frame_read(std::thread::current().id());
        let (started, waiting) = std::sync::mpsc::channel();
        let (send, result) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            started.send(()).unwrap();
            storage.cpu_write(&[FireflySsbo::from_translation(glam::Vec3::new(7.0, 8.0, 9.0))]);
            send.send(storage).unwrap();
        });
        waiting.recv().unwrap();
        assert!(result.try_recv().is_err());
        drop(read);
        let mut storage = result.recv_timeout(Duration::from_secs(10)).unwrap();
        thread.join().unwrap();
        assert_eq!(storage.mapped[0], FireflySsbo::from_translation(glam::Vec3::new(7.0, 8.0, 9.0)));
        // The upload capability pins the allocation even after its host table
        // and other native owners are gone; the retained mapping stays valid.
        drop(native);
        drop(host);
        storage.cpu_write(&[FireflySsbo::from_translation(glam::Vec3::ONE)]);
        assert_eq!(storage.mapped[0], FireflySsbo::from_translation(glam::Vec3::ONE));
    }
}
impl country_core::render::ssbo::rhapsody_boundary::MappedUploadWriter for MappedHostWriter {
    fn write(&self, offset: usize, data: &[u8]) -> Result<(), BackendError> {
        let _publication = self.boundary.write();
        self.buffer.write_bytes(offset as u64, data)
    }
    fn with_write(&self, offset: usize, bytes: usize, edit: &mut dyn FnMut())
        -> Result<(), BackendError> {
        let _publication = self.boundary.write();
        self.buffer.with_host_write(offset as u64, bytes as u64, edit)
    }
    fn try_read(&self, offset: usize, bytes: usize, read: &mut dyn FnMut())
        -> Result<bool, BackendError> {
        self.buffer.try_with_host_read(offset as u64, bytes as u64, read)
    }
}

#[cfg(test)]
mod frame_buffer_tests {
    use super::*;
    use std::{sync::mpsc, time::Duration};

    fn device() -> Option<Arc<Device>> {
        match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => Some(device),
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                None
            }
        }
    }

    fn cpu_buffer(host: &BufferHost, value: u8) -> Buffer {
        let usage = vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
        let buffer = host.create_buffer(&BufferDesc::new_gpu_only(16, usage.as_raw()),
            "frame version fixture", Some(&[value; 16])).unwrap();
        // SAFETY: this fixture has one complete CPU source and no GPU writer,
        // BDA or mapped pointer; native owners are only used by readback.
        unsafe { host.register_cpu_authoritative_buffer(&buffer); }
        buffer
    }

    fn gpu_bytes(device: &Arc<Device>, buffer: ManagedBuffer) -> Vec<u8> {
        let mut transfer = rhapsody::transfer::TransferManager::new(
            device.clone(), rhapsody::transfer::TransferConfig::default()).unwrap();
        let ticket = transfer.read_buffer(buffer.shared(), 0, 16).unwrap();
        ticket.wait(10_000_000_000).unwrap();
        let mut bytes = vec![0; 16];
        assert_eq!(ticket.try_read(&mut bytes).unwrap(), Some(16));
        bytes
    }

    #[test]
    fn pinned_cpu_source_forks_before_admission_and_old_and_new_gpu_contents_differ() {
        let Some(device) = device() else { return; };
        let host = Arc::new(BufferHost::new(device.clone()));
        let buffer = cpu_buffer(&host, 1);
        let old = host.managed_buffer(&buffer).unwrap();
        let boundary = host.begin_frame_read(std::thread::current().id());
        let snapshot = host.freeze_frame_resources();
        let pins = snapshot.0.buffers[&buffer.id].clone();
        let frozen = host.with_frame_snapshot(snapshot.clone()).unwrap();
        assert_eq!(pins.pins.load(Ordering::Acquire), 1);
        let producer = host.clone();
        let (send, receive) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            // Only one dirty interval is supplied. A fork must reconstruct all
            // bytes from the full CPU source, not copy just this interval.
            send.send(producer.update_cpu_buffer_bytes(&buffer, &[2; 16], &[4..8])).unwrap();
        });
        let result = receive.recv_timeout(Duration::from_secs(5));
        drop(boundary); // Always release before asserting/joining on a failure.
        worker.join().unwrap();
        result.expect("a COW producer must not wait for the old frame's CPU boundary").unwrap();
        let current = host.managed_buffer(&buffer).unwrap();
        assert!(!Arc::ptr_eq(&old.shared(), &current.shared()));
        assert!(Arc::ptr_eq(&old.shared(), &frozen.managed_buffer(&buffer).unwrap().shared()));
        host.flush_pending_uploads().unwrap();
        assert_eq!(gpu_bytes(&device, old.clone()), vec![1; 16]);
        assert_eq!(gpu_bytes(&device, current), vec![2; 16]);
        assert_eq!(host.cow_budget.lock().forks, 1);

        // Cancellation only releases frame pins; it must not discard uploads.
        drop(snapshot);
        assert_eq!(pins.pins.load(Ordering::Acquire), 1);
        drop(frozen);
        assert_eq!(pins.pins.load(Ordering::Acquire), 0);
        assert!(host.release(&buffer));
    }

    #[test]
    fn frozen_ids_survive_release_and_reuse_and_only_frame_local_new_ids_are_visible() {
        let Some(device) = device() else { return; };
        let host = BufferHost::new(device.clone());
        let buffer = cpu_buffer(&host, 3);
        let old = host.managed_buffer(&buffer).unwrap();
        let snapshot = host.freeze_frame_resources();
        let frozen = host.with_frame_snapshot(snapshot).unwrap();
        assert!(host.release(&buffer));
        let replacement = cpu_buffer(&host, 4);
        assert_eq!(buffer.id as u32, replacement.id as u32);
        assert_ne!(buffer.id, replacement.id);
        assert!(frozen.get_by_id(replacement.id).is_none(), "never read the next producer frame");
        assert!(Arc::ptr_eq(&old.shared(), &frozen.get_by_id(buffer.id).unwrap().shared()));
        let local = frozen.create_buffer(&BufferDesc::new_gpu_only(16,
            (vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST).as_raw()),
            "render-local fixture", None).unwrap();
        assert!(frozen.managed_buffer(&local).is_some());
        assert!(frozen.release(&local));
        assert!(host.get_by_id(local.id).is_none());
        assert!(frozen.get_by_id(local.id).is_some(), "local graph owner remains until cancellation");
        assert!(BufferHost::new(device).with_frame_snapshot(host.freeze_frame_resources()).is_err());
        assert!(host.release(&replacement));
    }

    #[test]
    fn exhausted_cow_budget_waits_for_registered_reads_without_publishing_a_partial_owner() {
        let Some(device) = device() else { return; };
        let host = Arc::new(BufferHost::new(device.clone()));
        let buffer = cpu_buffer(&host, 5);
        let old = host.managed_buffer(&buffer).unwrap();
        host.cow_budget.lock().max_bytes = 0;
        let boundary = host.begin_frame_read(std::thread::current().id());
        let snapshot = host.freeze_frame_resources();
        let (started, starts) = mpsc::channel();
        let (send, receive) = mpsc::channel();
        let producer = host.clone();
        let worker = std::thread::spawn(move || {
            started.send(()).unwrap();
            send.send(producer.update_cpu_buffer_bytes(&buffer, &[6; 16], &[0..16])).unwrap();
        });
        starts.recv().unwrap();
        let blocked = receive.recv_timeout(Duration::from_millis(30)).is_err();
        let same_before_release = Arc::ptr_eq(&old.shared(), &host.managed_buffer(&buffer).unwrap().shared());
        // Cancellation ends this frame's registration boundary. A live graph
        // would register its physical reads before opening the same boundary.
        drop(snapshot);
        drop(boundary);
        let result = receive.recv_timeout(Duration::from_secs(5));
        worker.join().unwrap();
        assert!(blocked && same_before_release);
        result.unwrap().unwrap();
        let current = host.managed_buffer(&buffer).unwrap();
        assert!(Arc::ptr_eq(&old.shared(), &current.shared()));
        host.flush_pending_uploads().unwrap();
        assert_eq!(gpu_bytes(&device, current), vec![6; 16]);
        let budget = host.cow_budget.lock();
        assert_eq!(budget.forks, 0);
        assert_eq!(budget.fallbacks, 1);
        drop(budget);
        assert!(host.release(&buffer));
    }

    #[test]
    fn accepted_sparse_upload_survives_submit_oom_after_dirty_clear_and_preserves_frame_versions() {
        use country_core::render::{sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo}, ssbo::StorageElement};
        #[derive(Clone, Copy, Default)]
        #[repr(transparent)]
        struct Value(u32);
        impl StorageElement for Value {
            const NAME: &'static str = "submit OOM sparse fixture";
            const BUFFER_LEN: usize = 32;
        }
        // SAFETY: transparent u32 has no padding; one 32-element chunk. This
        // fixture has a complete CPU source, no GPU writer or escaped address.
        unsafe impl SparseSsbo for Value {
            const CHUNK_COUNT: usize = 1;
            const CPU_AUTHORITATIVE: bool = true;
        }
        let Some(device) = device() else { return; };
        let host = BufferHost::new(device.clone());
        let mut storage = SparseInstanceBuffer::<Value>::try_new(&host).unwrap();
        let read_all = |buffer: ManagedBuffer| {
            let mut transfers = rhapsody::transfer::TransferManager::new(device.clone(),
                rhapsody::transfer::TransferConfig::default()).unwrap();
            let read = transfers.read_buffer(buffer.shared(), 0, buffer.size as usize).unwrap();
            read.wait(10_000_000_000).unwrap();
            let mut bytes = vec![0; buffer.size as usize];
            assert_eq!(read.try_read(&mut bytes).unwrap(), Some(bytes.len()));
            bytes
        };
        let allocation = storage.new_allocation();
        storage.writer(&allocation).replace_all(&[Value(3); 32]).unwrap();
        storage.try_gpu_flush(&host).unwrap();
        host.flush_pending_uploads().unwrap();
        let handle = storage.buffer.buffer;
        let old = host.managed_buffer(&handle).unwrap();
        let expected_old: Vec<_> = (0..32).flat_map(|_| 3u32.to_ne_bytes()).collect();
        assert_eq!(read_all(old.clone()), expected_old);
        let snapshot = host.freeze_frame_resources();
        storage.writer(&allocation).replace_all(&[Value(9); 32]).unwrap();
        storage.try_gpu_flush(&host).unwrap();
        assert!(storage.dirty_bitset.iter().all(|word| word.load(Ordering::Relaxed) == 0),
            "the producer has already transferred dirty ownership to the queue");
        let next = host.managed_buffer(&handle).unwrap();
        assert!(!Arc::ptr_eq(&old.shared(), &next.shared()));
        let before = host.uploads.lock().as_ref().unwrap().manager.upload_queue_stats();
        assert_eq!(before.queued_jobs, 1);

        // Real allocator failure after acceptance: command metadata cannot
        // reserve any more host bytes. Restore the limit before assertions so
        // test cleanup and recovery remain able to allocate.
        let budget = device.allocator_stats();
        let previous_limit = budget.host_budget_limit();
        budget.set_host_budget(budget.cpu_memory_usage().total()).unwrap();
        let failed = host.flush_pending_uploads();
        budget.set_host_budget(previous_limit).unwrap();
        assert_eq!(failed.unwrap_err().kind(), rhapsody::ErrorKind::OutOfMemory);
        let after = host.uploads.lock().as_ref().unwrap().manager.upload_queue_stats();
        assert_eq!((after.queued_jobs, after.queued_bytes), (before.queued_jobs, before.queued_bytes));
        assert!(after.known_cpu_bytes <= before.known_cpu_bytes,
            "a retry must not accumulate a second copy or failure envelope");

        // Add a later write to the same newly published owner. FIFO recovery
        // must apply the retained full source first, then this newer interval.
        host.update_buffer_bytes(&handle, &11u32.to_ne_bytes(), &[0..4], 0).unwrap();
        host.flush_pending_uploads().unwrap();
        let expected_new: Vec<_> = (0..32).map(|index| if index == 0 { 11u32 } else { 9 })
            .flat_map(u32::to_ne_bytes).collect();
        assert_eq!(read_all(next), expected_new);
        assert_eq!(read_all(old), expected_old);
        assert_eq!(host.uploads.lock().as_ref().unwrap().manager.upload_queue_stats().queued_jobs, 0);
        drop(snapshot);
        assert!(host.release(&handle));
    }
}

#[cfg(test)]
mod stable_frame_buffer_tests {
    use super::*;

    #[test]
    fn mapped_and_device_address_allocations_keep_their_physical_owner() {
        let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        let host = BufferHost::new(device.clone());
        let usage = vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;
        let mapped = host.create_buffer(&BufferDesc::new(16, usage.as_raw(), 2),
            "stable mapped fixture", Some(&[7; 16])).unwrap();
        // SAFETY: complete CPU data exists and no GPU pass writes it. The host
        // must still reject versioning for mapping/address capabilities.
        unsafe { host.register_cpu_authoritative_buffer(&mapped); }
        let old = host.managed_buffer(&mapped).unwrap();
        let snapshot = host.freeze_frame_resources();
        let _writer = host.mapped_upload_writer(&mapped).unwrap();
        host.update_cpu_buffer_bytes(&mapped, &[8; 16], &[0..16]).unwrap();
        assert!(Arc::ptr_eq(&old.shared(), &host.managed_buffer(&mapped).unwrap().shared()));
        let mut observed = [0; 16];
        old.read_bytes(0, &mut observed).unwrap();
        assert_eq!(observed, [8; 16]);
        drop(snapshot);
        assert!(host.release(&mapped));

        if device.caps().is_enabled(rhapsody::device::Feature::BufferDeviceAddress) {
            let addressable = host.create_buffer(&BufferDesc::new_gpu_only(16,
                (usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS).as_raw()),
                "stable BDA fixture", Some(&[9; 16])).unwrap();
            // SAFETY: full CPU data exists; BDA usage must override eligibility
            // before any persistent native address can escape.
            unsafe { host.register_cpu_authoritative_buffer(&addressable); }
            let old = host.managed_buffer(&addressable).unwrap();
            let address = host.device_address(&addressable);
            assert_ne!(address, 0);
            let snapshot = host.freeze_frame_resources();
            host.update_cpu_buffer_bytes(&addressable, &[10; 16], &[0..16]).unwrap();
            host.flush_pending_uploads().unwrap();
            assert_eq!(address, host.device_address(&addressable));
            assert!(Arc::ptr_eq(&old.shared(), &host.managed_buffer(&addressable).unwrap().shared()));
            let mut transfer = rhapsody::transfer::TransferManager::new(
                device.clone(), rhapsody::transfer::TransferConfig::default()).unwrap();
            let ticket = transfer.read_buffer(old.shared(), 0, 16).unwrap();
            ticket.wait(10_000_000_000).unwrap();
            assert_eq!(ticket.try_read(&mut observed).unwrap(), Some(16));
            assert_eq!(observed, [10; 16]);
            drop(snapshot);
            assert!(host.release(&addressable));
        }
        assert_eq!(host.cow_budget.lock().forks, 0);
    }
}

#[cfg(test)]
mod mapped_raster_gpu_tests {
    use super::*;
    use country_core::{render::ssbo::{MappedStorageBuffer, StorageElement, MAPPED_MEMORY_READBACK},
        resources::{world_raster::{DirtyTilePublisher, WorldRaster}, world_raster_defs::PathRasterStagingSsbo}};
    use rhapsody::command::VulkanCommandPool;
    use std::{sync::mpsc, time::Duration};

    // The fixture adds TRANSFER_SRC to the actual original staging allocation
    // so a real GPU copy can observe its contents without a substitute shader.
    struct CopyableHost(BufferHost);
    impl CoreGraphicsDevice for CopyableHost {
        fn create_buffer(&self, desc: &BufferDesc, name: &str, data: Option<&[u8]>) -> Result<Buffer, BackendError> {
            let mut desc = *desc;
            desc.usage |= vk::BufferUsageFlags::TRANSFER_SRC.as_raw();
            self.0.create_buffer(&desc, name, data)
        }
        fn update_buffer_bytes(&self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>], offset: usize)
            -> Result<(), BackendError> { self.0.update_buffer_bytes(buffer, data, ranges, offset) }
        fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> { self.0.mapped_slice_mut(buffer) }
        fn mapped_upload_writer(&self, buffer: &Buffer)
            -> Option<Arc<dyn country_core::render::ssbo::rhapsody_boundary::MappedUploadWriter>> {
            self.0.mapped_upload_writer(buffer)
        }
    }
    #[derive(Clone, Copy, Default)]
    struct ReadbackByte(u8);
    impl StorageElement for ReadbackByte {
        const NAME: &'static str = "protected raster readback fixture";
        const BUFFER_LEN: usize = 512 * 512;
        const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
    }
    struct SignalOnDrop(Arc<Device>, vk::Event, bool);
    impl SignalOnDrop {
        fn release(&mut self) {
            if self.2 {
                let result = unsafe { self.0.raw_device().set_event(self.1) };
                self.2 = false;
                result.unwrap();
            }
        }
    }
    impl Drop for SignalOnDrop {
        fn drop(&mut self) {
            self.release();
            // Native interop object: retain it through actual GPU completion,
            // including assertion unwinding before the explicit pending wait.
            let _ = self.0.wait_idle();
            unsafe { self.0.raw_device().destroy_event(self.1, None); }
        }
    }

    #[test]
    fn raster_edit_waits_for_real_gpu_read_and_readback_skips_unfinished_gpu_write() {
        let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        let host = CopyableHost(BufferHost::new(device.clone()));
        let mut staging = MappedStorageBuffer::<PathRasterStagingSsbo>::new_storage_buffer(&host, None);
        let readback = MappedStorageBuffer::<ReadbackByte>::new_storage_buffer(&host, None);
        let source = host.0.managed_buffer(&staging.buffer).unwrap();
        let output = host.0.managed_buffer(&readback.buffer).unwrap();
        let mut raster = WorldRaster::<512>::new(DirtyTilePublisher::unique());
        raster.composite_mut()[0] = 11;
        raster.dirty_mut().set_cell(0, 0);
        staging.with_cpu_write(|mapped| raster.copy_to_staging(0, bytemuck::cast_slice_mut(mapped))).unwrap();

        let pool = VulkanCommandPool::new(device.clone()).unwrap();
        let gate = unsafe { device.raw_device().create_event(&vk::EventCreateInfo::default(), None) }.unwrap();
        let mut release = SignalOnDrop(device.clone(), gate, true);
        let mut command = pool.allocate().unwrap();
        unsafe {
            command.retain_opaque_buffer_range(source.shared(), 0, source.size).unwrap();
            command.retain_opaque_buffer_range(output.shared(), 0, output.size).unwrap();
        }
        command.begin().unwrap();
        unsafe {
            device.raw_device().cmd_wait_events(command.raw_handle(), &[gate],
                vk::PipelineStageFlags::HOST, vk::PipelineStageFlags::TRANSFER, &[], &[], &[]);
            device.raw_device().cmd_pipeline_barrier(command.raw_handle(), vk::PipelineStageFlags::HOST,
                vk::PipelineStageFlags::TRANSFER, vk::DependencyFlags::empty(), &[],
                &[vk::BufferMemoryBarrier::default().src_access_mask(vk::AccessFlags::HOST_WRITE)
                    .dst_access_mask(vk::AccessFlags::TRANSFER_READ).buffer(source.raw_handle())
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED).dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .size(source.size)], &[]);
            device.raw_device().cmd_copy_buffer(command.raw_handle(), source.raw_handle(), output.raw_handle(),
                &[vk::BufferCopy::default().size(source.size)]);
            device.raw_device().cmd_pipeline_barrier(command.raw_handle(), vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST, vk::DependencyFlags::empty(), &[],
                &[vk::BufferMemoryBarrier::default().src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::HOST_READ).buffer(output.raw_handle())
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED).dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .size(output.size)], &[]);
        }
        command.end().unwrap();
        let pending = command.submit().unwrap();
        assert!(!pending.is_complete().unwrap());
        let mut inspected = false;
        assert_eq!(readback.try_cpu_read(|_| { inspected = true; }).unwrap(), None);
        assert!(!inspected, "unfinished GPU writes must not reach a CPU reader");
        let (started, starts) = mpsc::channel();
        let (finished, finishes) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            raster.composite_mut()[0] = 22;
            raster.dirty_mut().set_cell(0, 0);
            started.send(()).unwrap();
            staging.with_cpu_write(|mapped| raster.copy_to_staging(0, bytemuck::cast_slice_mut(mapped))).unwrap();
            finished.send(staging).unwrap();
        });
        starts.recv().unwrap();
        let early = finishes.recv_timeout(Duration::from_millis(30));
        let blocked = matches!(&early, Err(mpsc::RecvTimeoutError::Timeout));
        release.release(); // Unblock before any assertion can unwind.
        pending.wait(10_000_000_000).unwrap();
        let staging = early.unwrap_or_else(|_| finishes.recv_timeout(Duration::from_secs(5)).unwrap());
        worker.join().unwrap();
        assert!(blocked, "the producer overwrote an input still being read by GPU work");
        assert_eq!(readback.try_cpu_read(|mapped| mapped[0].0).unwrap(), Some(11));
        assert_eq!(staging.try_cpu_read(|mapped| mapped[0].0).unwrap(), Some(22));
    }
}

const UPLOAD_CHUNK_BYTES: u64 = 1 << 20;
const UPLOAD_STAGING_SLOTS: u32 = 3;
const UPLOAD_STAGING_BYTES: u64 = 16 << 20;
const UPLOAD_FLUSH_TIMEOUT: Duration = Duration::from_secs(10);

/// `TG_SYNC_UPLOAD=1` restores the old behaviour: every device-local write waits
/// for its own completion. Rollback switch for the queued path.
fn sync_upload_forced() -> bool {
    static FORCED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *FORCED.get_or_init(|| std::env::var_os("TG_SYNC_UPLOAD").is_some_and(|value| value == "1"))
}

impl BufferHost {
    pub(crate) fn begin_frame_read(&self, owner: std::thread::ThreadId) -> crate::render_upload_boundary::ReadLease {
        self.read_boundary.begin_read(owner)
    }

    pub fn graphics_device(&self) -> &Arc<Device> {
        &self.device
    }
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            slots: Arc::new(Mutex::new(Vec::new())),
            uploads: Arc::new(Mutex::new(None)),
            read_boundary: Default::default(),
            publication: Default::default(),
            cow_budget: Default::default(),
            upload_coverage: Default::default(),
            frame: None,
            #[cfg(test)]
            fail_upload_chunk: Mutex::new(None),
        }
    }

    /// Call after sealing begin_frame_read, before publishing the render packet.
    pub fn freeze_frame_resources(&self) -> FrameBufferSnapshot {
        let slots = self.slots.lock();
        let mut buffers = HashMap::with_capacity(slots.len());
        for (index, slot) in slots.iter().enumerate() {
            if let Some(version) = &slot.buffer {
                let id = (slot.generation << 32) | (index as u64 + 1);
                version.pins.fetch_add(1, Ordering::AcqRel);
                buffers.insert(id, version.clone());
            }
        }
        FrameBufferSnapshot(Arc::new(FrameBufferSnapshotData {
            source: Arc::downgrade(&self.slots), buffers, local: Mutex::new(HashMap::new()),
        }))
    }

    pub fn with_frame_snapshot(&self, snapshot: FrameBufferSnapshot) -> Result<Arc<Self>, BackendError> {
        if !Weak::ptr_eq(&snapshot.0.source, &Arc::downgrade(&self.slots)) {
            return Err(BackendError::invalid_usage("frame buffer snapshot belongs to another App"));
        }
        Ok(Arc::new(Self {
            device: self.device.clone(), slots: self.slots.clone(), uploads: self.uploads.clone(),
            read_boundary: self.read_boundary.clone(), publication: self.publication.clone(),
            cow_budget: self.cow_budget.clone(), upload_coverage: self.upload_coverage.clone(), frame: Some(snapshot),
            #[cfg(test)]
            fail_upload_chunk: Mutex::new(None),
        }))
    }

    /// Reserve exact retry coverage before the first accepted byte. A failed
    /// transaction may only be replaced by one complete covering operation.
    fn prepare_upload_coverage(&self, version: &BufferVersion, ranges: &[Range<usize>], offset: usize)
        -> Result<UploadCoverage, BackendError> {
        let count = ranges.iter().filter(|range| !range.is_empty()).count();
        let estimated = count.checked_mul(std::mem::size_of::<Range<u64>>())
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<IncompleteUpload>()))
            .filter(|&bytes| bytes <= UPLOAD_COVERAGE_BYTES)
            .ok_or(BackendError::OutOfHostMemory)?;
        let mut normalized = Vec::new();
        normalized.try_reserve_exact(count).map_err(|_| BackendError::OutOfHostMemory)?;
        normalized.extend(ranges.iter().filter(|range| !range.is_empty())
            .map(|range| offset as u64 + range.start as u64..offset as u64 + range.end as u64));
        normalized.sort_unstable_by_key(|range| range.start);
        let mut merged = 0usize;
        for index in 0..normalized.len() {
            let range = normalized[index].clone();
            if merged != 0 && normalized[merged - 1].end >= range.start {
                normalized[merged - 1].end = normalized[merged - 1].end.max(range.end);
            } else { normalized[merged] = range; merged += 1; }
        }
        normalized.truncate(merged);
        let charged = normalized.capacity().checked_mul(std::mem::size_of::<Range<u64>>())
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<IncompleteUpload>()))
            .ok_or(BackendError::OutOfHostMemory)?;
        debug_assert!(charged >= estimated);
        self.upload_coverage.bytes.fetch_update(Ordering::AcqRel, Ordering::Acquire, |bytes|
            bytes.checked_add(charged).filter(|&total| total <= UPLOAD_COVERAGE_BYTES))
            .map_err(|_| BackendError::OutOfHostMemory)?;
        let coverage = UploadCoverage { ranges: normalized, budget: self.upload_coverage.clone(), charged };
        if let Some(failed) = version.incomplete_upload.lock().as_ref()
            && !coverage.covers(&failed.coverage) {
            return Err(failed.error.clone().into());
        }
        Ok(coverage)
    }

    fn finish_upload_transaction(&self, version: &BufferVersion, coverage: UploadCoverage,
        result: Result<(), BackendError>) -> Result<(), BackendError> {
        let mut incomplete = version.incomplete_upload.lock();
        match result {
            Ok(()) => { incomplete.take(); Ok(()) }
            Err(error) => {
                let error = Arc::new(error);
                let original = incomplete.as_ref().map(|failed| failed.error.clone()).unwrap_or_else(|| error.clone());
                self.upload_coverage.failed.fetch_add(1, Ordering::AcqRel);
                *incomplete = Some(IncompleteUpload { coverage, error: original });
                Err(error.into())
            }
        }
    }

    fn check_upload_transactions(&self) -> Result<(), BackendError> {
        if self.upload_coverage.failed.load(Ordering::Acquire) == 0 { return Ok(()); }
        let check = |version: &BufferVersion| -> Result<(), BackendError> {
            match version.incomplete_upload.lock().as_ref() {
                Some(failed) => Err(failed.error.clone().into()),
                None => Ok(()),
            }
        };
        if let Some(frame) = &self.frame {
            for version in frame.0.buffers.values() { check(version)?; }
            // Render-local owners are created after the immutable extraction.
            let local = frame.0.local.lock();
            let slots = self.slots.lock();
            for slot in slots.iter() {
                if let Some(version) = &slot.buffer
                    && local.values().any(|buffer| Arc::ptr_eq(&buffer.shared(), &version.buffer.shared())) {
                    check(version)?;
                }
            }
        } else {
            for slot in self.slots.lock().iter() {
                if let Some(version) = &slot.buffer { check(version)?; }
            }
        }
        Ok(())
    }

    /// Lazily creates the bounded upload service shared by the queued and the
    /// waited paths. Three 1 MiB staging slots cover two frames in flight plus the
    /// frame being declared.
    fn upload_service<'a>(
        &self,
        uploads: &'a mut Option<BufferUploads>,
    ) -> Result<&'a mut BufferUploads, BackendError> {
        if uploads.is_none() {
            *uploads = Some(BufferUploads {
                manager: rhapsody::transfer::TransferManager::new_with_queue(
                    self.device.clone(),
                    rhapsody::transfer::TransferConfig {
                        slots: UPLOAD_STAGING_SLOTS,
                        chunk_bytes: UPLOAD_CHUNK_BYTES,
                        max_bytes: UPLOAD_STAGING_BYTES,
                    },
                    rhapsody::transfer::UploadQueueConfig {
                        max_keys: 1024,
                        ..Default::default()
                    },
                )?,
                epoch: 0,
                queued_bytes: 0,
            });
        }
        Ok(uploads.as_mut().expect("initialized bounded upload service"))
    }

    /// Accepts ordered dirty intervals without GPU work. Each interval owns
    /// its bytes and remains pending across epochs until submitted. A newer
    /// frame must not cancel an earlier sparse update to another interval.
    /// `flush_pending_uploads` submits them before the graph that reads them.
    fn queue_upload(
        &self,
        managed: &ManagedBuffer,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let mut uploads = self.uploads.lock();
        let service = self.upload_service(&mut uploads)?;
        // A pressure flush advances the epoch. Retry under the current epoch
        // while preserving every accepted delta and its original FIFO order.
        let options_at = |epoch: u64| rhapsody::transfer::UploadOptions {
            key: rhapsody::transfer::UploadKey(buffer.id),
            epoch,
            priority: rhapsody::transfer::UploadPriority::High,
        };
        let mut result = Ok(());
        'ranges: for range in ranges.iter().filter(|range| !range.is_empty()) {
            let mut cursor = range.start;
            while cursor < range.end {
                let bytes = (range.end - cursor).min(UPLOAD_CHUNK_BYTES as usize);
                let at = offset as u64 + cursor as u64;
                let slice = &data[cursor..cursor + bytes];
                let mut payload = Vec::new();
                payload.try_reserve_exact(bytes).map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                payload.extend_from_slice(slice);
                let mut accepted = service.manager.enqueue_upload_owned(
                    managed.shared(),
                    at,
                    payload,
                    options_at(service.epoch),
                    rhapsody::transfer::UploadMode::OrderedDelta,
                );
                if let Err(rejected) = accepted {
                    // Queue or memory pressure: drain what is pending, then retry once.
                    Self::flush_locked(service)?;
                    accepted = service.manager.enqueue_upload_owned(
                        managed.shared(),
                        at,
                        rejected.payload,
                        options_at(service.epoch),
                        rhapsody::transfer::UploadMode::OrderedDelta,
                    );
                }
                if let Err(error) = accepted {
                    result = Err(error.error);
                    break 'ranges;
                }
                service.queued_bytes += bytes as u64;
                cursor += bytes;
            }
        }
        if let Err(error) = result {
            // Never lose the write: complete everything queued so far, then wait for this one.
            log::warn!(target: "tiny_glade::upload", "queued upload rejected ({error}); falling back to a waited write");
            drop(uploads);
            return self.upload_waited(managed, buffer, data, ranges, offset);
        }
        Ok(())
    }

    /// Submits every queued write to the graphics queue, in order, without
    /// waiting for their completion. Call after all host writes of a frame and
    /// before the graph that reads them is submitted: the submitted jobs publish
    /// their completion into the buffers' range state, so the graph's bind step
    /// orders itself after them (queue order plus the recorded dependency).
    /// Waits only while the staging ring is full, bounded by `UPLOAD_FLUSH_TIMEOUT`.
    pub fn flush_pending_uploads(&self) -> Result<(), BackendError> {
        let mut uploads = self.uploads.lock();
        let Some(service) = uploads.as_mut() else {
            return Ok(());
        };
        Self::flush_locked(service)
    }

    fn flush_locked(service: &mut BufferUploads) -> Result<(), BackendError> {
        if service.queued_bytes == 0 {
            return Ok(());
        }
        let deadline = Instant::now() + UPLOAD_FLUSH_TIMEOUT;
        loop {
            // Producers have already handed off their dirty bytes. A submit
            // allocation failure must keep that accepted payload for the next
            // explicit recovery attempt, while aborting the current graph.
            let flush = service.manager.flush_uploads_preserving_oom(usize::MAX)?;
            if flush.remaining_jobs == 0 {
                break;
            }
            if !flush.capacity_deferred {
                return Err(rhapsody::Error::invalid_usage(
                    "upload queue stalled without staging pressure",
                ));
            }
            // Staging is full: earlier uploads must finish first. `flush_uploads`
            // polls the device each call, so back off briefly and retry within the
            // bound. Counted, because this is a real wait on the GPU.
            if Instant::now() >= deadline {
                return Err(rhapsody::Error::Timeout);
            }
            crate::frame_counters::record_upload_capacity_wait();
            std::thread::sleep(Duration::from_micros(100));
        }
        // Next frame's writes get a fresh epoch; this frame's jobs are submitted.
        service.epoch = service
            .epoch
            .checked_add(1)
            .ok_or_else(|| BackendError::invalid_usage("buffer upload epoch exhausted"))?;
        service.queued_bytes = 0;
        Ok(())
    }

    /// Explicit synchronous write: queued writes are submitted first (the waited
    /// path requires an empty queue), then this write returns once complete.
    pub fn update_buffer_bytes_wait(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let _publication = self.read_boundary.write();
        let _serial = self.publication.lock();
        let managed = self.lookup_current(buffer).ok_or_else(|| {
            BackendError::invalid_usage("buffer host handle is stale or belongs to another host")
        })?;
        if self.frame.is_some() && self.lookup(buffer).is_none_or(|frozen|
            !Arc::ptr_eq(&frozen.shared(), &managed.shared())) {
            return Err(BackendError::invalid_usage("waited write differs from its frozen owner"));
        }
        rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
        self.upload_waited(&managed, buffer, data, ranges, offset)
    }

    fn upload_waited(
        &self,
        managed: &ManagedBuffer,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let mut uploads = self.uploads.lock();
        let service = self.upload_service(&mut uploads)?;
        Self::flush_locked(service)?;
        service.epoch = service
            .epoch
            .checked_add(1)
            .ok_or_else(|| BackendError::invalid_usage("buffer upload epoch exhausted"))?;
        let started = crate::frame_counters::start();
        let result = service.manager.upload_buffer_ranges_wait(
            managed.shared(),
            data,
            ranges,
            offset as u64,
            rhapsody::transfer::UploadOptions {
                key: rhapsody::transfer::UploadKey(buffer.id),
                epoch: service.epoch,
                priority: rhapsody::transfer::UploadPriority::High,
            },
            u64::MAX,
        );
        crate::frame_counters::record_upload_wait(started, data.len());
        result
    }

    fn lookup(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        if handle.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != 0 { return None; }
        if handle.generation != handle.id >> 32 { return None; }
        self.get_by_id(handle.id)
    }

    fn current_version(&self, handle: &Buffer) -> Option<Arc<BufferVersion>> {
        let slots = self.slots.lock();
        if handle.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != 0 { return None; }
        let slot = slots.get(((handle.id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        (slot.generation == handle.generation && slot.generation == handle.id >> 32)
            .then(|| slot.buffer.clone())
            .flatten()
    }

    fn lookup_current(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        Some(self.current_version(handle)?.buffer.clone())
    }

    /// Stable-address capabilities permanently exclude this logical allocation
    /// from COW. Call under publication, before returning the mapping/address.
    fn require_stable_address(&self, handle: &Buffer) {
        let mut slots = self.slots.lock();
        if let Some(slot) = ((handle.id & 0xffff_ffff) as usize).checked_sub(1)
            .and_then(|index| slots.get_mut(index))
            .filter(|slot| slot.generation == handle.generation && slot.generation == handle.id >> 32)
        {
            slot.stable_address = true;
            slot.cpu_authoritative = false;
        }
    }

    /// Resolve a retained core buffer only while its slot generation is live.
    pub fn managed_buffer(&self, handle: &Buffer) -> Option<ManagedBuffer> {
        self.lookup(handle)
    }

    /// ID-only graph/material bindings also validate the encoded allocation
    /// generation. A released slot can never resolve to its next occupant.
    pub fn get_by_id(&self, id: u64) -> Option<ManagedBuffer> {
        if id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != 0 { return None; }
        if let Some(frame) = &self.frame {
            return frame.0.buffers.get(&id).map(|version| version.buffer.clone())
                .or_else(|| frame.0.local.lock().get(&id).cloned());
        }
        let slots = self.slots.lock();
        let slot = slots.get(((id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        slot.buffer.as_ref().filter(|_| slot.generation == id >> 32)
            .map(|version| version.buffer.clone())
    }

    /// 释放句柄：槽位清空并递增代数，缓冲随最后一个克隆释放进入延迟销毁。
    pub fn release(&self, handle: &Buffer) -> bool {
        let _publication = self.read_boundary.write();
        let _serial = self.publication.lock();
        if handle.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != 0 { return false; }
        let mut slots = self.slots.lock();
        let Some(slot) = ((handle.id & 0xffff_ffff) as usize)
            .checked_sub(1)
            .and_then(|i| slots.get_mut(i))
        else {
            return false;
        };
        if slot.generation != handle.generation || slot.generation != handle.id >> 32 || slot.buffer.is_none() {
            return false;
        }
        slot.buffer = None;
        slot.generation = 0;
        true
    }

    /// Return the real description only while the host handle is live.
    pub fn description(&self, handle: &Buffer) -> Option<RhBufferDesc> {
        Some(self.lookup(handle)?.desc)
    }

    /// Diagnostics for the actual bounded device-local sparse update service.
    pub fn queued_upload_stats(&self) -> Option<rhapsody::transfer::UploadQueueStats> {
        self.uploads
            .lock()
            .as_ref()
            .map(|service| service.manager.upload_queue_stats())
    }

    pub fn convert(desc: &BufferDesc) -> rhapsody::Result<RhBufferDesc> {
        let location = MemoryLocation::from_raw(desc.memory)
            .ok_or_else(|| rhapsody::Error::invalid_usage("unknown buffer memory location"))?;
        let alignment = u16::try_from(desc.alignment)
            .map_err(|_| rhapsody::Error::invalid_usage("buffer alignment exceeds u16"))?;
        if alignment != 0 && !alignment.is_power_of_two() {
            return Err(rhapsody::Error::invalid_usage(
                "buffer alignment must be a power of two",
            ));
        }
        // The current shader heap lowers original uniform bindings to storage
        // reads. Keep the core/native descriptor intact and admit that physical
        // use on this host's allocation, including persistent library buffers.
        let mut usage = vk::BufferUsageFlags::from_raw(desc.usage);
        if usage.contains(vk::BufferUsageFlags::UNIFORM_BUFFER) {
            usage |= vk::BufferUsageFlags::STORAGE_BUFFER;
        }
        Ok(RhBufferDesc::new(
            u64::try_from(desc.size)
                .map_err(|_| rhapsody::Error::invalid_usage("buffer size exceeds u64"))?,
            usage,
            location,
        )
        .with_alignment(alignment))
    }
}

impl CoreGraphicsDevice for BufferHost {
    unsafe fn register_cpu_authoritative_buffer(&self, handle: &Buffer) {
        let _serial = self.publication.lock();
        let mut slots = self.slots.lock();
        if let Some(slot) = ((handle.id & 0xffff_ffff) as usize).checked_sub(1)
            .and_then(|index| slots.get_mut(index))
            .filter(|slot| handle.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) == 0
                && slot.generation == handle.generation && slot.generation == handle.id >> 32)
            && !slot.stable_address
            && slot.buffer.as_ref().is_some_and(|version|
                version.buffer.desc.memory_location == MemoryLocation::GpuOnly
                && !version.buffer.desc.usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS))
        {
            slot.cpu_authoritative = true;
        }
    }

    fn update_cpu_buffer_bytes(
        &self, handle: &Buffer, data: &[u8], ranges: &[Range<usize>],
    ) -> Result<(), BackendError> {
        // Render-local producers have the old stable boundary semantics; only
        // a canonical producer can publish a subsequent physical version.
        if self.frame.is_none() {
            let serial = self.publication.lock();
            let previous = self.current_version(handle).ok_or_else(||
                BackendError::invalid_usage("CPU source buffer handle is stale"))?;
            rhapsody::resource::plan_buffer_update(&previous.buffer.desc, data.len(), ranges, 0)?;
            if data.len() as u64 != previous.buffer.size {
                return Err(BackendError::invalid_usage("CPU authority requires the complete allocation"));
            }
            let eligible = {
                let slots = self.slots.lock();
                slots[(handle.id as u32 - 1) as usize].cpu_authoritative
            };
            if eligible && !ranges.iter().all(Range::is_empty)
                && previous.pins.load(Ordering::Acquire) != 0
            {
                let mut budget = self.cow_budget.lock();
                if budget.admits(previous.buffer.size) {
                    let next = self.device.create_buffer(&previous.buffer.desc,
                        "CPU authoritative frame version", None)?;
                    // Every byte comes from the authoritative CPU source. The
                    // existing bounded service owns each accepted upload chunk;
                    // no old queued job is cancelled or retargeted.
                    let uploaded = if sync_upload_forced() {
                        self.upload_waited(&next, handle, data, &[0..data.len()], 0)
                    } else {
                        self.queue_upload(&next, handle, data, &[0..data.len()], 0)
                    };
                    if let Err(error) = uploaded {
                        // A partially accepted upload may still own the new
                        // buffer. Charge it until those native owners retire.
                        budget.retired.push(RetiredBuffer {
                            owner: Arc::downgrade(&next.shared()), bytes: next.size,
                        });
                        return Err(error);
                    }
                    let old_owner = previous.buffer.shared();
                    budget.retired.push(RetiredBuffer {
                        owner: Arc::downgrade(&old_owner), bytes: previous.buffer.size,
                    });
                    budget.forks += 1;
                    let mut slots = self.slots.lock();
                    let slot = &mut slots[(handle.id as u32 - 1) as usize];
                    slot.buffer = Some(BufferVersion::new(next));
                    return Ok(());
                }
                budget.fallbacks += 1;
            }
            // Do not hold publication while waiting for the old frame: its
            // render-local uploads and resource release also need that lock.
            drop(serial);
        }
        self.update_buffer_bytes(handle, data, ranges, 0)
    }

    fn release_buffer(&self, buffer: &Buffer) {
        self.release(buffer);
    }

    fn create_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
        data: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let managed = self
            .device
            .create_buffer(&Self::convert(desc)?, name, data)?;
        // 宿主可移植性策略：原 None 不承诺字节值，原异步加载却会在首次
        // GPU 写入前读取回读头。只在新建且没有初始数据的读回缓冲上定义零值；
        // 原构造仍传 None，已有映射、导入和真实 GPU 结果不会经过这里重置。
        if data.is_none() && managed.desc.memory_location == MemoryLocation::GpuToCpu {
            managed.fill_mapped_bytes_unsynchronized(0, managed.size, 0)?;
        }
        let mut slots = self.slots.lock();
        let index = slots
            .iter()
            .position(|s| s.buffer.is_none())
            .unwrap_or_else(|| {
                slots.push(Slot {
                    generation: 0,
                    buffer: None,
                    cpu_authoritative: false,
                    stable_address: false,
                });
                slots.len() - 1
            });
        let slot_id = u32::try_from(index + 1)
            .map_err(|_| BackendError::invalid_usage("buffer slot namespace exhausted"))?;
        let generation = next_resource_generation()?;
        let id = (generation << 32) | u64::from(slot_id);
        let byte_size = managed.size;
        slots[index].generation = generation;
        if let Some(frame) = &self.frame {
            frame.0.local.lock().insert(id, managed.clone());
        }
        slots[index].buffer = Some(BufferVersion::new(managed));
        slots[index].cpu_authoritative = false;
        slots[index].stable_address = false;
        Ok(Buffer {
            id,
            generation,
            byte_size,
            _gap10: 0,
            _gap20: 0,
        })
    }

    fn update_buffer_bytes(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), BackendError> {
        let _publication = self.read_boundary.write();
        let _serial = self.publication.lock();
        let managed = self.lookup_current(buffer).ok_or_else(|| {
            BackendError::invalid_usage("buffer host handle is stale or belongs to another host")
        })?;
        if self.frame.is_some() && self.lookup(buffer).is_none_or(|frozen|
            !Arc::ptr_eq(&frozen.shared(), &managed.shared())) {
            return Err(BackendError::invalid_usage("render-local write differs from its frozen owner"));
        }
        if managed.desc.memory_location != MemoryLocation::GpuOnly
            && managed.persistent_host_ptr().is_some()
        {
            // Device::update_buffer_bytes is an asynchronous staging copy,
            // even for mapped targets. A host-visible update must instead wait
            // for overlapping GPU uses and write/flush the actual mapping.
            // Validate the entire request before the first mutation.
            rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
            for range in ranges.iter().filter(|range| !range.is_empty()) {
                managed.write_bytes(offset as u64 + range.start as u64, &data[range.clone()])?;
            }
            return Ok(());
        }
        // Reject invalid ranges before allocating the lazy transfer service.
        rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
        if sync_upload_forced() {
            return self.upload_waited(&managed, buffer, data, ranges, offset);
        }
        self.queue_upload(&managed, buffer, data, ranges, offset)
    }

    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        let _serial = self.publication.lock();
        self.require_stable_address(buffer);
        let managed = self.lookup(buffer)?;
        let ptr = managed.persistent_host_ptr()?;
        // SAFETY: 指针来自持久映射且已含偏移；槽位在 `release` 之前一直持有缓冲，
        // 调用方（原版语义）负责与 GPU 的同步及不并发别名。
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, managed.size as usize) })
    }

    fn mapped_upload_writer(&self, buffer: &Buffer)
        -> Option<Arc<dyn country_core::render::ssbo::rhapsody_boundary::MappedUploadWriter>> {
        let _serial = self.publication.lock();
        self.require_stable_address(buffer);
        let managed = self.lookup(buffer)?;
        if !matches!(managed.desc.memory_location, MemoryLocation::CpuToGpu | MemoryLocation::CpuToGpuLowPrio | MemoryLocation::GpuToCpu)
            || managed.persistent_host_ptr().is_none() { return None; }
        Some(Arc::new(MappedHostWriter { buffer: managed, boundary: self.read_boundary.clone() }))
    }

    fn device_address(&self, buffer: &Buffer) -> u64 {
        let _serial = self.publication.lock();
        self.require_stable_address(buffer);
        self.lookup(buffer)
            .and_then(|b| b.device_address().ok())
            .expect("Buffer device-address backend: invalid handle or buffer without BDA usage")
    }
}

// ---------------------------------------------------------------- texture

use country_core::render::load_texture::boundary::{
    BackendError as TextureBackendError, Texture, TextureDesc, TextureDevice,
};
use rhapsody::{
    resource::ManagedTexture, resource::TextureDesc as RhTextureDesc, resource::TextureKind,
};

/// `Texture.id` 的标记位：图导入入口 `import_unowned(id, access)` 对缓冲和纹理共用，靠它区分。
pub const TEXTURE_ID_TAG: u64 = 1 << 63;

/// 缓冲导入 token 的标记位。rhapsody 后端的 token 空间由 `allocate_token()`（从小整数递增，图内创建的资源也从这里取）
/// 与显式 `bind_import*` 共用；宿主槽位序号是小整数，直接当 token 会与图内资源撞车
/// （“token已经属于其他物理资源”），所以导入图时一律经 [`graph_token`] 打上高位标记。
pub const BUFFER_TOKEN_TAG: u64 = 1 << 62;

/// core 句柄 `id`（含唯一分配代数与槽位，纹理另带 [`TEXTURE_ID_TAG`]）→ rhapsody 后端的跨帧 token。
/// 帧驱动方用同一个值调 `VulkanGraphBackend::bind_import_named(.., graph_token(id), ..)`。
pub fn graph_token(id: u64) -> u64 {
    if id & TEXTURE_ID_TAG != 0 {
        id
    } else {
        id | BUFFER_TOKEN_TAG
    }
}

struct TextureSlot {
    generation: u64,
    texture: Option<ManagedTexture>,
}

/// core 的 `TextureDevice` 接缝：`Texture.id` 编码全局分配代数与槽位序号。
#[derive(Clone)]
pub struct TextureHost {
    device: Arc<Device>,
    slots: Arc<Mutex<Vec<TextureSlot>>>,
    /// 默认视图缓存（原 `ManagedTexture::view` 按描述缓存，M2.6）：视图随纹理一起存活，`release` 时丢弃。
    views: Arc<Mutex<std::collections::HashMap<u64, rhapsody::resource::ManagedTextureView>>>,
    /// Native explicit subresource views, retained until the texture owner retires.
    view_variants: Arc<
        Mutex<
            std::collections::HashMap<
                (u64, country_core::rhapsody_types::TextureViewDesc),
                rhapsody::resource::ManagedTextureView,
            >,
        >,
    >,
    /// 宿主附加的用途位（core 的 `TextureDesc` 只有 sampled/storage/附件五个开关，没有 TRANSFER_SRC；
    /// 读回像素的宿主/测试在这里统一加上。原版 `TextureDesc.usage` 是完整位集，此项是宿主策略，不是原版行为）。
    extra_usage: vk::ImageUsageFlags,
    read_boundary: Arc<crate::render_upload_boundary::UploadBoundary>,
}

impl TextureHost {
    pub(crate) fn begin_frame_read(&self, owner: std::thread::ThreadId) -> crate::render_upload_boundary::ReadLease {
        self.read_boundary.begin_read(owner)
    }
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
            slots: Arc::new(Mutex::new(Vec::new())),
            views: Arc::new(Mutex::new(Default::default())),
            view_variants: Arc::new(Mutex::new(Default::default())),
            extra_usage: vk::ImageUsageFlags::empty(),
            read_boundary: Default::default(),
        }
    }

    /// 之后创建的所有纹理（含图内创建的）额外带上这些用途位。
    pub fn with_extra_usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.extra_usage = usage;
        self
    }

    /// `convert` + 宿主附加用途位（图内创建纹理与 `create_texture` 共用）。
    pub fn convert_for_graph(
        &self,
        desc: &TextureDesc,
    ) -> Result<RhTextureDesc, TextureBackendError> {
        let mut out = Self::convert(desc)?;
        out.usage |= self.extra_usage;
        Ok(out)
    }

    /// 默认全范围视图的原生句柄（首次创建后缓存）。
    pub fn default_view(&self, id: u64) -> rhapsody::Result<u64> {
        if self.device.is_device_lost() {
            return Err(rhapsody::Error::DeviceLost);
        }
        let texture = self
            .get_by_id(id)
            .ok_or_else(|| rhapsody::Error::invalid_usage("texture host handle is not live"))?;
        let mut views = self.views.lock();
        if let Some(view) = views.get(&id) {
            return Ok(ash::vk::Handle::as_raw(view.raw()));
        }
        let desc = rhapsody::resource::TextureViewDesc::new(&texture.desc)?;
        let view = self.device.get_vk_image_view(&texture, &desc)?;
        let raw = ash::vk::Handle::as_raw(view.raw());
        views.insert(id, view);
        Ok(raw)
    }

    /// Select the native disk texture subresource with the recovered descriptor.
    pub fn explicit_view(
        &self,
        id: u64,
        desc: &country_core::rhapsody_types::TextureViewDesc,
    ) -> rhapsody::Result<u64> {
        let texture = self
            .get_by_id(id)
            .ok_or_else(|| rhapsody::Error::invalid_usage("texture host handle is not live"))?;
        let mut variants = self.view_variants.lock();
        if let Some(view) = variants.get(&(id, *desc)) {
            return Ok(ash::vk::Handle::as_raw(view.raw()));
        }
        let native = rh_view_desc(&texture, desc)?;
        let view = self.device.get_vk_image_view(&texture, &native)?;
        let raw = ash::vk::Handle::as_raw(view.raw());
        variants.insert((id, *desc), view);
        Ok(raw)
    }

    pub(crate) fn owned_view(
        &self,
        id: u64,
        raw: u64,
    ) -> rhapsody::Result<Arc<rhapsody::resource::TextureView>> {
        if let Some(view) = self.views.lock().get(&id) {
            if ash::vk::Handle::as_raw(view.raw()) == raw {
                return Ok(view.view.clone());
            }
        }
        self.view_variants
            .lock()
            .iter()
            .find(|((owner, _), view)| *owner == id && ash::vk::Handle::as_raw(view.raw()) == raw)
            .map(|(_, view)| view.view.clone())
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("external texture view is not owned by this host")
            })
    }

    /// rhapsody 描述 → core 描述（`managed_texture_desc`）。
    pub fn core_desc(texture: &ManagedTexture) -> TextureDesc {
        let d = &texture.desc;
        TextureDesc {
            flags: d.flags.as_raw(),
            extent: [d.width, d.height, d.depth],
            kind: d.kind as u8,
            sampled: d.usage.contains(vk::ImageUsageFlags::SAMPLED),
            mip_levels: d.mip_levels,
            array_elements: d.array_elements as u16,
            storage: d.usage.contains(vk::ImageUsageFlags::STORAGE),
            color_attachment: d.usage.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT),
            depth_stencil_attachment: d
                .usage
                .contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
            format: d.format.as_raw() as u32,
        }
    }

    pub fn get(&self, texture: &Texture) -> Option<ManagedTexture> {
        if texture.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != TEXTURE_ID_TAG { return None; }
        let slots = self.slots.lock();
        let slot = slots.get(((texture.id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        slot.texture
            .clone()
            .filter(|_| slot.generation == texture.generation())
    }

    /// 按 `Texture.id`（含标记与代数）取，代数不符返回 `None`。
    pub fn get_by_id(&self, id: u64) -> Option<ManagedTexture> {
        if id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != TEXTURE_ID_TAG { return None; }
        let slots = self.slots.lock();
        let slot = slots.get(((id & 0xffff_ffff) as usize).checked_sub(1)?)?;
        let generation = (id >> 32) & 0x7fff_ffff;
        slot.texture
            .clone()
            .filter(|_| slot.generation == generation)
    }

    pub fn release(&self, texture: &Texture) -> bool {
        let _publication = self.read_boundary.write();
        if texture.id & (TEXTURE_ID_TAG | BUFFER_TOKEN_TAG) != TEXTURE_ID_TAG { return false; }
        let mut slots = self.slots.lock();
        let Some(slot) = ((texture.id & 0xffff_ffff) as usize)
            .checked_sub(1)
            .and_then(|i| slots.get_mut(i))
        else {
            return false;
        };
        if slot.generation != texture.generation() || slot.texture.is_none() {
            return false;
        }
        slot.texture = None;
        slot.generation = 0;
        self.views.lock().remove(&texture.id);
        self.view_variants
            .lock()
            .retain(|(id, _), _| *id != texture.id);
        true
    }

    /// core 描述 → rhapsody 原布局描述；`kind` 非法时报错而不是猜。
    pub fn convert(desc: &TextureDesc) -> Result<RhTextureDesc, TextureBackendError> {
        let kind = TextureKind::from_raw(desc.kind).ok_or_else(|| {
            TextureBackendError::invalid_usage(format!("unknown texture kind {}", desc.kind))
        })?;
        let extent = vk::Extent3D {
            width: desc.extent[0],
            height: desc.extent[1],
            depth: desc.extent[2],
        };
        let mut out = RhTextureDesc::new(extent, kind, vk::Format::from_raw(desc.format as i32))
            .mip_levels(desc.mip_levels.max(1))
            .array_elements(u32::from(desc.array_elements).max(1))?;
        out.flags = vk::ImageCreateFlags::from_raw(desc.flags);
        if desc.sampled {
            out = out.sampled();
        }
        if desc.storage {
            out = out.storage();
        }
        if desc.color_attachment {
            out = out.color_attachment();
        }
        if desc.depth_stencil_attachment {
            out = out.depth_stencil_attachment();
        }
        Ok(out)
    }
}

impl TextureDevice for TextureHost {
    fn release_texture(&self, texture: &Texture) {
        self.release(texture);
    }
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        data: &[u8],
    ) -> Result<Texture, TextureBackendError> {
        let rh = self.convert_for_graph(desc)?;
        let managed = if data.is_empty() && rh.usage.contains(vk::ImageUsageFlags::SAMPLED) {
            self.device.create_texture_zeroed(&rh, name)?
        } else {
            self.device.create_texture(&rh, name, data)?
        };
        let mut slots = self.slots.lock();
        let index = slots
            .iter()
            .position(|s| s.texture.is_none())
            .unwrap_or_else(|| {
                slots.push(TextureSlot {
                    generation: 0,
                    texture: None,
                });
                slots.len() - 1
            });
        let slot_id = u32::try_from(index + 1)
            .map_err(|_| BackendError::invalid_usage("texture slot namespace exhausted"))?;
        let generation = next_resource_generation()?;
        slots[index].texture = Some(managed);
        slots[index].generation = generation;
        Ok(Texture {
            id: TEXTURE_ID_TAG | (generation << 32) | u64::from(slot_id),
            width: desc.extent[0],
            height: desc.extent[1],
            format: desc.format,
        })
    }
}

impl country_core::asset_libraries::texture_library::TextureLibraryDevice for TextureHost {
    fn create_persistent_descriptor_set(
        &self,
        _name: &str,
    ) -> rhapsody::Result<country_core::rhapsody_contract::DescriptorSet> {
        use country_core::asset_libraries::texture_library::{
            REQUIRED_BINDLESS_DESCRIPTOR_COUNT, RESERVED_DESCRIPTOR_COUNT,
        };
        let mut sampled_capacity = self
            .device
            .max_bindless_descriptor_count()
            .min(REQUIRED_BINDLESS_DESCRIPTOR_COUNT - RESERVED_DESCRIPTOR_COUNT);
        loop {
            match rhapsody::bindless::BindlessHeap::new(
                self.device.clone(),
                rhapsody::bindless::BindlessConfig {
                    sampled_images: sampled_capacity,
                    storage_images: 1,
                    storage_buffers: 1,
                    samplers: 1,
                },
            ) {
                Ok(heap) => {
                    let host = self.clone();
                    return Ok(country_core::rhapsody_contract::DescriptorSet::from_heap(
                        heap,
                        sampled_capacity,
                        move |texture| {
                            let raw = country_core::asset_libraries::texture_library::TextureLibraryDevice::get_vk_image_view(&host, texture, 0)?;
                            host.owned_view(texture.id, raw.0)
                        },
                    ));
                }
                Err(error)
                    if error.kind() == rhapsody::ErrorKind::Unsupported && sampled_capacity > 1 =>
                {
                    // 复用后端的真实布局支持检查，逐步收缩到设备可用容量。
                    sampled_capacity /= 2;
                }
                Err(error) => return Err(error),
            }
        }
    }

    fn update_persistent_descriptor_set(
        &self,
        descriptors: &country_core::rhapsody_contract::DescriptorSet,
        updates: &[country_core::asset_libraries::texture_library::DescriptorImageUpdate],
    ) -> rhapsody::Result<()> {
        let _publication = self.read_boundary.write();
        for update in updates {
            if update.binding != 0
                || update.layout != vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw() as u64
            {
                return Err(rhapsody::Error::invalid_usage("纹理库只接受只读采样描述符"));
            }
            // 使用资产 ID 查表并核对视图，避免把任意整数作为 Vulkan 句柄。
            self.get_by_id(update.texture_id)
                .ok_or_else(|| rhapsody::Error::invalid_usage("纹理已释放"))?;
            let view = self.owned_view(update.texture_id, update.image_view.0)?;
            descriptors.validate_sampled_image(update.index.0, &view)?;
        }
        Ok(())
    }
    fn get_vk_image_view(
        &self,
        texture: &Texture,
        mip: u32,
    ) -> rhapsody::Result<country_core::asset_libraries::texture_library::ImageView> {
        if mip != 0 || self.get(texture).is_none() {
            return Err(rhapsody::Error::invalid_usage("纹理或 mip 无效"));
        }
        self.default_view(texture.id)
            .map(country_core::asset_libraries::texture_library::ImageView)
    }
    fn update_texture(
        &self,
        texture: &Texture,
        update: &country_core::asset_libraries::texture_library::TextureUpdate,
    ) -> rhapsody::Result<()> {
        let _publication = self.read_boundary.write();
        let managed = self
            .get(texture)
            .ok_or_else(|| rhapsody::Error::invalid_usage("纹理已释放"))?;
        if update.buffer_offset != 0 || update.array_layers == 0 {
            return Err(rhapsody::Error::unsupported(
                "纹理更新要求零偏移和非零数组层数",
            ));
        }
        let layer_count = u32::try_from(update.array_layers)
            .map_err(|_| rhapsody::Error::invalid_usage("纹理层数溢出"))?;
        if update.data.len() % layer_count as usize != 0 {
            return Err(rhapsody::Error::invalid_usage("纹理数据层长度不一致"));
        }
        let bytes_per_layer = update.data.len() / layer_count as usize;
        let regions: Vec<_> = (0..layer_count)
            .map(|layer| {
                let start = layer as usize * bytes_per_layer;
                rhapsody::resource::TextureUpdateRegion {
                    data: &update.data[start..start + bytes_per_layer],
                    extent: update.extent,
                    mip_level: update.mip_level,
                    array_layer: layer,
                }
            })
            .collect();
        rhapsody::resource::update_texture_regions(&self.device, &managed, &regions)
    }
}

trait TextureGeneration {
    fn generation(&self) -> u64;
}
impl TextureGeneration for Texture {
    /// core 的 `Texture` 只有 `id: u64`：bit 63 是纹理标记，bits 32..63 是代数，低 32 位是槽位序号。
    fn generation(&self) -> u64 {
        (self.id >> 32) & 0x7fff_ffff
    }
}

// ---------------------------------------------------------------- compute pipeline

use country_core::render::pass_builder::{
    compute_pass_builder::{ComputePassDevice, ResourceRegistry},
    uniform_binder::{UniformBlockInfo, UniformReflection},
};
use country_core::render::shader::boundary::{
    CompiledShaderPipeline as CoreCompiled, ComputePipeline, ComputePipelineDesc,
    Error as ShaderError, RasterPipeline, RasterPipelineDesc, RasterPipelineKey, ShaderBackend,
};
use rhapsody::bindless::BindlessShaderAbi as BindlessAbi;
use rhapsody::bindless::{
    BindlessConfig, BindlessHeap, BindlessLease, BindlessParameters, BindlessResource,
    BindlessShaderPackage,
};
use rhapsody::pipeline::{
    AsyncPipelineCompiler, CompilerConfig, PersistentPipelineCache, PipelineTicket,
};
use rhapsody::rg::RenderPassKey;
use rhapsody::{
    pipeline::DynamicRenderingFormats, pipeline::PipelineCache,
    pipeline::RasterPipelineDesc as RhRasterDesc, pipeline::RasterPipelineOpts,
    pipeline::ShaderModule, pipeline::VulkanPipeline, pipeline::vertex_input_from_reflection,
    resource::Sampler,
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline as RhCompiled, compiled::ShaderStage,
    pipeline_desc::ComputePipelineDesc as FileComputeDesc, shader_defines::ShaderDefines,
};
use shader_pipeline::{
    reflection::BindingKind, reflection::BindingLocation, reflection::ShaderReflection,
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{Duration, Instant},
};

/// 发行 .bin 里存的反射 → core 的 `ResourceRegistry`：名字哈希 → (set, binding)、uniform 块大小与所在 set、
/// uniform 字段名哈希 → 偏移。`UniformBlockInfo.set_index` 取块绑定位置的 set（原偏移 `+0x130` 的 u32，字段名未证实）。
pub fn registry_from_reflection(reflection: &ShaderReflection) -> ResourceRegistry {
    ResourceRegistry {
        bindings: reflection
            .binding_name_hashes
            .iter()
            .map(|(hash, location)| (*hash, (location.set, location.binding)))
            .collect(),
        uniform_block: reflection
            .uniform_block
            .as_ref()
            .map(|block| UniformBlockInfo {
                size: block.byte_size,
                set_index: block.binding.set,
            }),
        uniform: UniformReflection {
            valid: reflection.uniform_block.is_some(),
            fields: reflection
                .uniform_block
                .iter()
                .flat_map(|block| block.name_hash_offsets.iter().copied())
                .collect(),
        },
    }
}

/// core 的 `ShaderBackend` 接缝，创建计算/光栅管线及其共享 bindless ABI。
///
/// 发行 .bin 的路径由 shader-pipeline 的 `compiled_file_path` 给出（相对 `root`），解码、反射布局、
/// 描述符/管线布局和管线创建全部走 rhapsody。Sampler 先取宿主覆盖，随后按反射名字查
/// rhapsody 已恢复的内置采样器表；未知名字保留设备返回的错误。
pub struct ShaderHost {
    device: Arc<Device>,
    pipeline_namespaces: [u64; 2],
    root: PathBuf,
    compiler: AsyncPipelineCompiler,
    disk_cache: Option<PersistentPipelineCache>,
    samplers: BTreeMap<String, Arc<Sampler>>,
    heap: Arc<BindlessHeap>,
    pages: Arc<rhapsody::bindless::BindlessPagePool>,
    sampler_leases: Arc<SamplerPages>,
    compute: Mutex<Vec<Arc<ComputeEntry>>>,
    raster: Mutex<Vec<Arc<RasterEntry>>>,
    passes: Arc<RenderPassHost>,
    frame_globals: Arc<Mutex<Option<Arc<crate::frame_setup::FrameGlobals>>>>,
    indexed_count_adapter: Mutex<Option<Arc<rhapsody::rg::IndexedCountAdapter>>>,
    /// Requested pipelines not yet observed ready by `prewarm`.
    pending_tickets: Mutex<Vec<PendingTicket>>,
}

struct PendingTicket {
    ticket: PipelineTicket,
    identity: PrewarmIdentity,
}

/// What a pending ticket was requested for; only used to word failures.
enum PrewarmIdentity {
    Compute(Arc<ComputeEntry>),
    Raster(Arc<RasterEntry>),
    IndexedCount,
}

/// Bounded attachment signatures used to request dynamic-rendering pipelines.
#[derive(Default)]
pub struct RenderPassHost {
    table: Mutex<Vec<Arc<RenderPassEntry>>>,
    by_desc: Mutex<std::collections::HashMap<rhapsody::rg::RenderPassDesc, u64>>,
}

pub struct RenderPassEntry {
    pub formats: DynamicRenderingFormats,
    pub color_attachments: usize,
    pub samples: vk::SampleCountFlags,
}

impl RenderPassHost {
    /// Resolve a complete attachment signature without creating native render objects.
    pub fn create_from_desc(
        &self,
        device: &Arc<Device>,
        desc: &rhapsody::rg::RenderPassDesc,
    ) -> Result<u64, rhapsody::Error> {
        let mut by_desc = self.by_desc.lock();
        if let Some(id) = by_desc.get(desc) {
            return Ok(*id);
        }
        let mut attachments = desc
            .colors
            .as_slice()
            .iter()
            .map(|color| color.to_attachment_key())
            .collect::<Vec<_>>();
        if let Some((depth, access)) = desc.depth {
            let mut attachment = depth.to_attachment_key();
            attachment.initial_layout = access.layout().as_raw();
            attachment.final_layout = access.layout().as_raw();
            attachment.subpass_layout = access.layout().as_raw();
            attachments.push(attachment);
        }
        let entry = Self::entry(device, &RenderPassKey { attachments })?;
        let mut table = self.table.lock();
        if table.len() >= 256 {
            return Err(rhapsody::Error::Timeout);
        }
        table.push(Arc::new(entry));
        let id = table.len() as u64;
        by_desc.insert(*desc, id);
        Ok(id)
    }

    fn entry(device: &Arc<Device>, key: &RenderPassKey) -> rhapsody::Result<RenderPassEntry> {
        let first = key
            .attachments
            .first()
            .ok_or_else(|| rhapsody::Error::invalid_usage("raster target has no attachments"))?;
        let samples = vk::SampleCountFlags::from_raw(first.samples);
        if !first.samples.is_power_of_two()
            || key.attachments.iter().any(|a| a.samples != first.samples)
            || key.attachments.iter().filter(|a| a.depth_stencil).count() > 1
        {
            return Err(rhapsody::Error::invalid_usage(
                "raster target sample/depth contract is inconsistent",
            ));
        }
        let mut formats = DynamicRenderingFormats {
            colors: Vec::new(),
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        };
        for attachment in &key.attachments {
            let format = vk::Format::from_raw(attachment.format);
            if attachment.depth_stencil {
                let aspects = rhapsody::resource::vk_format_valid_aspect_flags(format);
                if aspects.contains(vk::ImageAspectFlags::DEPTH) {
                    formats.depth = format;
                }
                if aspects.contains(vk::ImageAspectFlags::STENCIL) {
                    formats.stencil = format;
                }
            } else {
                formats.colors.push(format);
            }
        }
        device.caps().require_all_enabled(
            rhapsody::device::FeatureSet::EMPTY.with(rhapsody::device::Feature::DynamicRendering),
        )?;
        Ok(RenderPassEntry {
            color_attachments: formats.colors.len(),
            formats,
            samples,
        })
    }

    /// 已建通道数（测试/诊断）。
    pub fn len(&self) -> usize {
        self.table.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 按 `RenderPassKey` 建一个渲染通道并登记，返回 id（从 1 起）。
    pub fn create(
        &self,
        device: &Arc<Device>,
        key: &RenderPassKey,
    ) -> Result<u64, rhapsody::Error> {
        let entry = Self::entry(device, key)?;
        let mut table = self.table.lock();
        if table.len() >= 256 {
            return Err(rhapsody::Error::Timeout);
        }
        table.push(Arc::new(entry));
        Ok(table.len() as u64)
    }

    pub fn get(&self, id: u64) -> Option<Arc<RenderPassEntry>> {
        self.table
            .lock()
            .get((id as usize).checked_sub(1)?)
            .cloned()
    }
}

/// 一条已创建的光栅管线及绘制时需要的反射信息。
pub struct RasterEntry {
    pub pipeline: PipelineTicket,
    pub layout: Arc<rhapsody::pipeline::PipelineLayout>,
    pub registry: Arc<ResourceRegistry>,
    pub pass: Arc<RenderPassEntry>,
    pub bindings: ShaderBindings,
    pub uniform_binding: Option<u32>,
    /// Exact names and locations from the same original compiled reflection
    /// used by vertex_input_from_reflection to build this Vulkan pipeline.
    /// `(属性名, location, 顶点步长字节)`；步长取反射类型 `nbyte`，与管线 binding stride 一致。
    pub vertex_inputs: Vec<(String, u32, u32)>,
}

/// 一条已创建的计算管线及执行它所需的反射信息。
pub struct ComputeEntry {
    pub pipeline: PipelineTicket,
    pub registry: Arc<ResourceRegistry>,
    /// 着色器 `local_size`（反射 `local_size`，缺省 1×1×1）。
    pub local_size: [u32; 3],
    pub bindings: ShaderBindings,
    /// uniform 块所在的 binding 号（反射 `uniform_block.binding.binding`）。
    pub uniform_binding: Option<u32>,
}

struct SamplerBinding {
    location: BindingLocation,
    element: u32,
    name: Arc<str>,
    sampler: Arc<Sampler>,
}
#[derive(Clone)]
struct CatalogSampler {
    name: Arc<str>,
    sampler: Arc<Sampler>,
}
struct CachedSampler {
    name: Arc<str>,
    sampler: Arc<Sampler>,
    lease: Arc<BindlessLease>,
}
#[derive(Default)]
struct SamplerPageCache {
    leases: Vec<CachedSampler>,
    catalog: Option<Arc<Vec<CatalogSampler>>>,
    epoch: u64,
}
struct SamplerPage {
    heap: Arc<BindlessHeap>,
    leases: Mutex<SamplerPageCache>,
}
#[derive(Default)]
struct SamplerPagesState {
    pages: Vec<Arc<SamplerPage>>,
    // 仅来自本 App 已安装管线的实际反射，保留同名不同 owner 的冻结版本。
    catalog: Option<Arc<Vec<CatalogSampler>>>,
    catalog_disabled: bool,
    #[cfg(test)]
    fail_next_registration: bool,
}
#[derive(Default)]
struct SamplerPages {
    state: Mutex<SamplerPagesState>,
}
struct PendingSampler {
    name: Arc<str>,
    sampler: Arc<Sampler>,
    lease: Option<Arc<BindlessLease>>,
    changed: bool,
}
struct SamplerPreparation {
    page: Arc<SamplerPage>,
    values: Vec<PendingSampler>,
    catalog: Option<Arc<Vec<CatalogSampler>>>,
    publish_catalog: bool,
    epoch: u64,
}
fn same_sampler(
    left_name: &str,
    left: &Arc<Sampler>,
    right_name: &str,
    right: &Arc<Sampler>,
) -> bool {
    left_name == right_name && Arc::ptr_eq(left, right)
}
impl SamplerPages {
    /// 管线表已准入；目录只是可选缓存，元数据不足不能拒绝原本可安装的 entry。
    fn register(&self, bindings: &[SamplerBinding]) {
        let previous = {
            let mut state = self.state.lock();
            if state.catalog_disabled {
                return;
            }
            let next = (|| -> rhapsody::Result<Option<Vec<CatalogSampler>>> {
                #[cfg(test)]
                if std::mem::take(&mut state.fail_next_registration) {
                    return Err(rhapsody::Error::OutOfHostMemory);
                }
                let old = state.catalog.as_deref().map(Vec::as_slice).unwrap_or(&[]);
                let missing = bindings
                    .iter()
                    .enumerate()
                    .filter(|(index, binding)| {
                        !old.iter().any(|value| {
                            same_sampler(
                                &value.name,
                                &value.sampler,
                                &binding.name,
                                &binding.sampler,
                            )
                        }) && !bindings[..*index].iter().any(|prior| {
                            same_sampler(
                                &prior.name,
                                &prior.sampler,
                                &binding.name,
                                &binding.sampler,
                            )
                        })
                    })
                    .count();
                if missing == 0 {
                    return Ok(None);
                }
                let mut next = Vec::new();
                next.try_reserve(
                    old.len()
                        .checked_add(missing)
                        .ok_or(rhapsody::Error::OutOfHostMemory)?,
                )
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                next.extend(old.iter().cloned());
                for binding in bindings {
                    if !next.iter().any(|value| {
                        same_sampler(&value.name, &value.sampler, &binding.name, &binding.sampler)
                    }) {
                        next.push(CatalogSampler {
                            name: binding.name.clone(),
                            sampler: binding.sampler.clone(),
                        });
                    }
                }
                Ok(Some(next))
            })();
            match next {
                Ok(Some(next)) => state.catalog.replace(Arc::new(next)),
                Ok(None) => return,
                Err(_) => {
                    // 遗漏登记后不能再假称目录完整；本 App 退回原逐组缓存路径。
                    state.catalog_disabled = true;
                    state.catalog.take()
                }
            }
        };
        drop(previous);
    }
    fn prepare(
        &self,
        heap: &Arc<BindlessHeap>,
        bindings: &[SamplerBinding],
    ) -> rhapsody::Result<SamplerPreparation> {
        let (page, mut catalog) = {
            let mut state = self.state.lock();
            let page = if let Some(page) = state
                .pages
                .iter()
                .find(|page| Arc::ptr_eq(&page.heap, heap))
            {
                page.clone()
            } else {
                state
                    .pages
                    .try_reserve(1)
                    .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                let page = Arc::new(SamplerPage {
                    heap: heap.clone(),
                    leases: Mutex::new(SamplerPageCache::default()),
                });
                state.pages.push(page.clone());
                page
            };
            (page, state.catalog.clone())
        };
        let mut values: Vec<PendingSampler> = Vec::new();
        values
            .try_reserve(bindings.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        let epoch;
        {
            let mut cached = page.leases.lock();
            epoch = cached.epoch;
            if catalog.as_ref().is_some_and(|current| {
                cached
                    .catalog
                    .as_ref()
                    .is_some_and(|old| Arc::ptr_eq(old, current))
            }) {
                catalog = None;
            }
            for binding in bindings {
                if values.iter().any(|value| {
                    same_sampler(&value.name, &value.sampler, &binding.name, &binding.sampler)
                }) {
                    continue;
                }
                let lease = cached
                    .leases
                    .iter()
                    .find(|value| {
                        same_sampler(&value.name, &value.sampler, &binding.name, &binding.sampler)
                    })
                    .map(|value| value.lease.clone());
                values.push(PendingSampler {
                    name: binding.name.clone(),
                    sampler: binding.sampler.clone(),
                    changed: lease.is_none(),
                    lease,
                });
            }
            let missing = values.iter().filter(|value| value.changed).count();
            // 实际组仍只按原需求预留；目录额外资源不得先占其预算。
            cached
                .leases
                .try_reserve(missing)
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        }
        Ok(SamplerPreparation {
            page,
            values,
            catalog,
            publish_catalog: false,
            epoch,
        })
    }
    fn invalidate(&self, name: &str) {
        let pages: Vec<_> = self.state.lock().pages.iter().cloned().collect();
        for page in pages {
            page.begin_invalidation();
            page.finish_invalidation(name);
        }
    }
}
impl SamplerPage {
    fn begin_invalidation(&self) {
        let previous = {
            let mut cached = self.leases.lock();
            // 达到极限后不再发布 stamp，禁止环绕使旧准备误命中。
            cached.epoch = cached.epoch.saturating_add(1);
            cached.catalog.take()
        };
        drop(previous);
    }
    fn finish_invalidation(&self, name: &str) {
        loop {
            let (old, previous) = {
                let mut cached = self.leases.lock();
                let old = cached
                    .leases
                    .iter()
                    .position(|value| value.name.as_ref() == name)
                    .map(|index| cached.leases.swap_remove(index));
                let previous = if old.is_none() {
                    // 删除间隙可能有新准备借用旧项并发布 stamp；最终空检查同锁再撤销。
                    cached.epoch = cached.epoch.saturating_add(1);
                    cached.catalog.take()
                } else {
                    None
                };
                (old, previous)
            };
            drop(previous);
            match old {
                Some(old) => drop(old),
                None => break,
            }
        }
    }
}
impl SamplerPreparation {
    fn missing(&self) -> usize {
        self.values.iter().filter(|value| value.changed).count()
    }
    fn materialize(&mut self) -> rhapsody::Result<()> {
        for value in &mut self.values {
            if value.lease.is_none() {
                value.lease = Some(self.page.heap.sampler(value.sampler.clone())?);
            }
        }
        Ok(())
    }
    fn write(
        &mut self,
        bindings: &[SamplerBinding],
        parameters: &mut BindlessParameters,
    ) -> rhapsody::Result<()> {
        self.materialize()?;
        for binding in bindings {
            let value = self
                .values
                .iter()
                .find(|value| {
                    same_sampler(&value.name, &value.sampler, &binding.name, &binding.sampler)
                })
                .expect("prepared sampler metadata");
            parameters.write_lease(
                binding.location,
                binding.element,
                value.lease.as_ref().expect("materialized sampler").clone(),
            )?;
        }
        Ok(())
    }
    /// 仅在实际参数 finish 成功后、同页 guard 内调用；不再写实际参数或分配动态块。
    fn try_catalog_extras(
        &mut self,
        admit: impl FnOnce([usize; 4]) -> rhapsody::bindless::GroupPreparationResult<()>,
    ) -> rhapsody::Result<()> {
        let Some(catalog) = self.catalog.as_ref() else {
            return Ok(());
        };
        let capacity = self
            .page
            .heap
            .layout
            .plan()
            .bindings
            .iter()
            .find(|binding| {
                binding.binding == 3
                    && binding.descriptor_type == vk::DescriptorType::SAMPLER.as_raw()
            })
            .map_or(0, |binding| binding.descriptor_count as usize);
        let actual_extra = self
            .values
            .iter()
            .filter(|value| {
                !catalog.iter().any(|entry| {
                    same_sampler(&entry.name, &entry.sampler, &value.name, &value.sampler)
                })
            })
            .count();
        if !catalog
            .len()
            .checked_add(actual_extra)
            .is_some_and(|total| total <= capacity)
            || catalog
                .iter()
                .any(|value| !Arc::ptr_eq(value.sampler.device(), self.page.heap.layout.device()))
        {
            return Ok(());
        }
        let mut extras = Vec::new();
        if extras.try_reserve(catalog.len()).is_err() {
            return Ok(());
        }
        {
            let mut cached = self.page.leases.lock();
            for value in catalog.iter() {
                if self.values.iter().any(|actual| {
                    same_sampler(&actual.name, &actual.sampler, &value.name, &value.sampler)
                }) || cached
                    .leases
                    .iter()
                    .any(|old| same_sampler(&old.name, &old.sampler, &value.name, &value.sampler))
                {
                    continue;
                }
                extras.push(PendingSampler {
                    name: value.name.clone(),
                    sampler: value.sampler.clone(),
                    lease: None,
                    changed: true,
                });
            }
            let Some(wanted) = self.missing().checked_add(extras.len()) else {
                return Ok(());
            };
            if self.values.try_reserve(extras.len()).is_err()
                || cached.leases.try_reserve(wanted).is_err()
            {
                return Ok(());
            }
        }
        if !extras.is_empty() {
            match admit([0, 0, 0, extras.len()]) {
                Ok(()) => {}
                Err(rhapsody::bindless::GroupPreparationError::InsufficientSlots { .. }) => {
                    return Ok(());
                }
                Err(rhapsody::bindless::GroupPreparationError::Error(error)) => return Err(error),
            }
            for value in &mut extras {
                match self.page.heap.sampler(value.sampler.clone()) {
                    Ok(lease) => value.lease = Some(lease),
                    Err(error) if optional_sampler_budget_error(&error) => return Ok(()),
                    Err(error) => return Err(error),
                }
            }
        }
        // 全部额外租约成功才合并；取消时本地 extras 归还未提交的新槽。
        self.values.extend(extras);
        self.publish_catalog = true;
        Ok(())
    }
    fn commit(self) {
        for value in self.values.into_iter().filter(|value| value.changed) {
            let new = CachedSampler {
                name: value.name,
                sampler: value.sampler,
                lease: value.lease.expect("complete sampler group"),
            };
            let old =
                {
                    let mut cached = self.page.leases.lock();
                    if let Some(index) = cached.leases.iter().position(|old| {
                        same_sampler(&old.name, &old.sampler, &new.name, &new.sampler)
                    }) {
                        Some(std::mem::replace(&mut cached.leases[index], new))
                    } else {
                        cached.leases.push(new);
                        None
                    }
                };
            drop(old);
        }
        if let Some(catalog) = self.catalog.filter(|_| self.publish_catalog) {
            let previous = {
                let mut cached = self.page.leases.lock();
                if cached.epoch == self.epoch && self.epoch != u64::MAX {
                    cached.catalog.replace(catalog)
                } else {
                    None
                }
            };
            drop(previous);
        }
    }
}
fn optional_sampler_budget_error(error: &rhapsody::Error) -> bool {
    matches!(
        error,
        rhapsody::Error::OutOfHostMemory
            | rhapsody::Error::OutOfDeviceMemory
            | rhapsody::Error::Exhausted(_)
    )
}
/// 覆盖规则已在实际资源收集端执行；declared 保留 UniformBytes 的正式图访问角色。
pub struct ShaderResourceBinding {
    pub location: BindingLocation,
    pub element: u32,
    pub resource: BindlessResource,
    pub declared: bool,
}
pub struct ShaderBindingSnapshot {
    name: Arc<str>,
    abi: Arc<BindlessAbi>,
    samplers: Arc<Vec<SamplerBinding>>,
    sampler_pages: Arc<SamplerPages>,
    globals: Option<Arc<crate::frame_setup::FrameGlobals>>,
}
impl ShaderBindingSnapshot {
    pub fn prepare<'a>(
        &self,
        cursor: &mut rhapsody::bindless::BindlessPageCursor,
        push: &[u8],
        resources: impl Iterator<Item = (BindingLocation, u32, &'a BindlessResource, bool)> + Clone,
        previous: Option<&rhapsody::bindless::PreparedBindlessParameters>,
        constants: &Mutex<rhapsody::resource::DynamicConstants>,
    ) -> rhapsody::Result<rhapsody::bindless::PreparedBindlessParameters> {
        if push.len() != self.abi.original_push_bytes as usize {
            return Err(rhapsody::Error::invalid_usage(
                "shader push parameter size does not match its ABI",
            ));
        }
        let t = country_core::render::declare_probe::start();
        let mut local = BindlessParameters::resource_requirements(
            &self.abi,
            resources
                .clone()
                .filter(|(_, _, _, declared)| !*declared)
                .map(|(location, element, resource, _)| (location, element, resource)),
        )?;
        local[2] += 1 + resources
            .clone()
            .filter(|(_, _, _, declared)| *declared)
            .count();
        country_core::render::declare_probe::sub(0, t);
        cursor
            .prepare(|group| {
                if let Some(previous) =
                    previous.filter(|previous| previous.belongs_to(group.heap()))
                {
                    group.require_slots([0, 0, 1, 0])?;
                    return previous
                        .with_push(push, &mut constants.lock())
                        .map_err(Into::into);
                }
                let t = country_core::render::declare_probe::start();
                let globals = self
                    .globals
                    .as_ref()
                    .map(|globals| globals.prepare_page(&self.abi, group.heap()))
                    .transpose()?;
                country_core::render::declare_probe::sub(1, t);
                let t = country_core::render::declare_probe::start();
                let mut samplers = self.sampler_pages.prepare(group.heap(), &self.samplers)?;
                country_core::render::declare_probe::sub(2, t);
                let t = country_core::render::declare_probe::start();
                let mut required = local;
                required[3] += samplers.missing();
                if let Some(globals) = &globals {
                    for (need, extra) in required.iter_mut().zip(globals.requirements()) {
                        *need += extra;
                    }
                }
                group.require_slots(required)?;
                let mut parameters =
                    BindlessParameters::new(group.heap().clone(), self.abi.clone(), push)?;
                country_core::render::declare_probe::sub(3, t);
                let t = country_core::render::declare_probe::start();
                samplers.write(&self.samplers, &mut parameters)?;
                country_core::render::declare_probe::sub(4, t);
                let t = country_core::render::declare_probe::start();
                let publication = globals
                    .as_ref()
                    .zip(self.globals.as_ref())
                    .map(|(pending, globals)| pending.write_to(globals, &mut parameters))
                    .transpose()?;
                country_core::render::declare_probe::sub(5, t);
                let t = country_core::render::declare_probe::start();
                for (location, element, resource, declared) in resources.clone() {
                    if declared {
                        let BindlessResource::Buffer {
                            buffer,
                            offset,
                            size,
                        } = resource
                        else {
                            return Err(rhapsody::Error::invalid_usage(
                                "declared uniform descriptor is not its actual buffer",
                            )
                            .into());
                        };
                        let lease = group
                            .heap()
                            .storage_buffer(buffer.clone(), *offset, *size)?;
                        parameters.write_lease(location, element, lease)?;
                    } else {
                        parameters.write(location, element, resource.clone())?;
                    }
                }
                country_core::render::declare_probe::sub(6, t);
                let t = country_core::render::declare_probe::start();
                let prepared = parameters.finish(&mut constants.lock())?;
                samplers.try_catalog_extras(|required| group.require_slots(required))?;
                // 实际组和可选目录准备均已完成；不发布半个绑定组或半个目录。
                if let Some(publication) = publication {
                    publication.commit();
                }
                samplers.commit();
                country_core::render::declare_probe::sub(7, t);
                Ok(prepared)
            })
            .map_err(|error| {
                if error.kind() == rhapsody::ErrorKind::Unsupported {
                    rhapsody::Error::unsupported(format!(
                        "shader {} ABI {} bytes/{} bindings: {error}",
                        self.name,
                        self.abi.parameter_bytes,
                        self.abi.bindings.len()
                    ))
                } else {
                    error
                }
            })
    }
}

pub struct ShaderBindings {
    name: Arc<str>,
    pub heap: Arc<BindlessHeap>,
    pub abi: Arc<BindlessAbi>,
    pages: Arc<rhapsody::bindless::BindlessPagePool>,
    samplers: Arc<Vec<SamplerBinding>>,
    sampler_pages: Arc<SamplerPages>,
    frame_globals: Arc<Mutex<Option<Arc<crate::frame_setup::FrameGlobals>>>>,
}
impl ShaderBindings {
    /// Owner and range of the FrameConstants block behind set 1/b1.
    pub fn frame_storage_view(
        &self,
    ) -> rhapsody::Result<(Arc<rhapsody::resource::Buffer>, u64, u64)> {
        self.frame_globals
            .lock()
            .as_ref()
            .map(|globals| globals.storage_view())
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage(
                    "frame constants storage binding requires the frame globals",
                )
            })
    }
    /// 全局纹理表（set 2/b0）本帧快照的视图；无帧全局时为空。
    pub fn frame_sampled_table(&self) -> Vec<Arc<rhapsody::resource::TextureView>> {
        self.frame_globals
            .lock()
            .as_ref()
            .map(|globals| {
                globals
                    .sampled_table()
                    .iter()
                    .map(|(_, view)| view.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
    pub fn snapshot(&self) -> ShaderBindingSnapshot {
        self.snapshot_with_globals(self.frame_globals.lock().clone())
    }
    pub fn snapshot_with_globals(
        &self,
        globals: Option<Arc<crate::frame_setup::FrameGlobals>>,
    ) -> ShaderBindingSnapshot {
        ShaderBindingSnapshot {
            name: self.name.clone(),
            abi: self.abi.clone(),
            samplers: self.samplers.clone(),
            sampler_pages: self.sampler_pages.clone(),
            globals,
        }
    }
    pub fn image(
        &self,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
    ) -> rhapsody::Result<BindlessResource> {
        let layout = match self
            .abi
            .slot(BindingLocation { set: 0, binding })
            .map(|slot| slot.kind)
        {
            Some(BindingKind::StorageImage) => vk::ImageLayout::GENERAL,
            Some(BindingKind::SampledImage) => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            _ => {
                return Err(rhapsody::Error::invalid_usage(
                    "image binding differs from shader ABI",
                ));
            }
        };
        self.image_with_layout(binding, view, layout)
    }

    fn image_with_layout(
        &self,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
        layout: vk::ImageLayout,
    ) -> rhapsody::Result<BindlessResource> {
        let kind = self
            .abi
            .slot(BindingLocation { set: 0, binding })
            .map(|slot| slot.kind)
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("image binding is absent from shader ABI")
            })?;
        validate_image_binding_layout(kind, layout)?;
        match kind {
            BindingKind::StorageImage => Ok(BindlessResource::StorageImage(view)),
            BindingKind::SampledImage => Ok(BindlessResource::SampledImage { view, layout }),
            _ => unreachable!("validated image ABI kind"),
        }
    }

    pub(crate) fn image_for_pass(
        &self,
        ctx: &rhapsody::rg::PassContext<'_>,
        resource: u32,
        binding: u32,
        view: Arc<rhapsody::resource::TextureView>,
    ) -> rhapsody::Result<BindlessResource> {
        let selected = view.desc;
        let layout = ctx.image_layout_for_range(
            resource,
            rhapsody::rg::AccessRange::Texture {
                aspect_mask: selected.aspect.as_raw(),
                base_mip_level: selected.base_mip_level,
                level_count: selected.level_count,
                base_array_layer: selected.base_layer,
                layer_count: selected.layer_count,
            },
        )?;
        self.image_with_layout(binding, view, layout)
    }
}

fn validate_image_binding_layout(
    kind: BindingKind,
    layout: vk::ImageLayout,
) -> rhapsody::Result<()> {
    let valid = match kind {
        BindingKind::SampledImage => matches!(
            layout,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                | vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
                | vk::ImageLayout::GENERAL
        ),
        BindingKind::StorageImage => layout == vk::ImageLayout::GENERAL,
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(rhapsody::Error::invalid_usage(
            "planned image layout differs from shader ABI",
        ))
    }
}

impl ShaderHost {
    pub fn new(device: Arc<Device>, root: impl Into<PathBuf>) -> rhapsody::Result<Self> {
        Self::new_with_pipeline_cache(device, root, None)
    }
    /// Pipeline compilation runs on bounded workers. No frame callback waits.
    pub fn new_with_pipeline_cache(
        device: Arc<Device>,
        root: impl Into<PathBuf>,
        cache_path: Option<PathBuf>,
    ) -> rhapsody::Result<Self> {
        let disk_cache = cache_path
            .map(|path| PersistentPipelineCache::open(device.clone(), path, 256))
            .transpose()?;
        let cache = match &disk_cache {
            Some(disk) => disk.cache.clone(),
            None => Arc::new(PipelineCache::new(device.clone(), &[])?),
        };
        let compiler = AsyncPipelineCompiler::new(cache, CompilerConfig::default())?;
        // A complete original Nani round retains immutable parameter blocks
        // for hundreds of sublist dispatches until its submission completes.
        // The library's small default (256 storage descriptors) exhausted in
        // the actual fill-unsorted-indirection pass. This is an explicit host
        // budget, not an original game constant or a change to shader ABI;
        // it adapts to the enabled device's descriptor-indexing limits.
        let heap = negotiated_shader_heap(&device)?;
        let pages = rhapsody::bindless::BindlessPagePool::new(heap.clone())?;
        Ok(Self {
            device,
            pipeline_namespaces: [next_resource_generation()?, next_resource_generation()?],
            root: root.into(),
            compiler,
            disk_cache,
            samplers: BTreeMap::new(),
            heap,
            pages,
            sampler_leases: Arc::new(SamplerPages::default()),
            compute: Mutex::new(Vec::new()),
            raster: Mutex::new(Vec::new()),
            passes: Arc::new(RenderPassHost::default()),
            frame_globals: Arc::new(Mutex::new(None)),
            indexed_count_adapter: Mutex::new(None),
            pending_tickets: Mutex::new(Vec::new()),
        })
    }
    /// Uses this host's existing heap/compiler. Device support policy is
    /// selected by the raster host; no second descriptor backend is created.
    pub fn indexed_count_adapter(
        &self,
    ) -> rhapsody::Result<Arc<rhapsody::rg::IndexedCountAdapter>> {
        let mut cached = self.indexed_count_adapter.lock();
        if let Some(adapter) = cached.as_ref() {
            return Ok(adapter.clone());
        }
        let adapter = rhapsody::rg::IndexedCountAdapter::new_paged(
            self.device.clone(),
            self.pages.clone(),
            &self.compiler,
        )?;
        self.note_pending(adapter.ticket(), PrewarmIdentity::IndexedCount)?;
        *cached = Some(adapter.clone());
        Ok(adapter)
    }
    pub fn set_frame_globals(&self, globals: Arc<crate::frame_setup::FrameGlobals>) {
        *self.frame_globals.lock() = Some(globals);
    }
    /// Tickets that were requested and not yet observed ready by `prewarm`.
    /// Registered at request time so a warm frame scans nothing.
    fn note_pending(
        &self,
        ticket: PipelineTicket,
        identity: PrewarmIdentity,
    ) -> rhapsody::Result<()> {
        let mut pending = self.pending_tickets.lock();
        pending
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        pending.push(PendingTicket { ticket, identity });
        Ok(())
    }
    /// Puts tickets back in front of newer registrations, keeping request order.
    /// Poll only the owned tickets registered by declaration. Pending and a
    /// terminal Timeout failure remain distinct; no render-thread sleep occurs.
    pub fn poll_preparation(&self) -> rhapsody::Result<bool> {
        let mut pending = self.pending_tickets.lock();
        let mut failed = None;
        pending.retain(|request| match request.ticket.poll() {
            rhapsody::pipeline::PipelinePoll::Ready(_) => false,
            rhapsody::pipeline::PipelinePoll::Pending => true,
            rhapsody::pipeline::PipelinePoll::Failed(error) => {
                if failed.is_none() { failed = Some(error); }
                true
            }
        });
        if let Some(error) = failed { return Err(error.into()); }
        Ok(pending.is_empty())
    }
    fn requeue(&self, items: impl Iterator<Item = PendingTicket>) {
        let mut rest: Vec<_> = items.collect();
        let mut pending = self.pending_tickets.lock();
        rest.append(&mut pending);
        *pending = rest;
    }
    /// Explicit startup/prewarm operation. Rendering uses ticket.try_get().
    ///
    /// Waits only for tickets registered since the last successful call. A
    /// failed or timed-out ticket stays registered, so every later call reports
    /// it again, exactly like the former whole-table scan did.
    pub fn prewarm(&self, timeout: Duration) -> rhapsody::Result<()> {
        let deadline = Instant::now()
            .checked_add(timeout)
            .ok_or(rhapsody::Error::Timeout)?;
        // 不持锁等待;保持原等待次序和错误类别;只在失败时补充实际请求身份。
        let batch = std::mem::take(&mut *self.pending_tickets.lock());
        let total = batch.len();
        // Counts every scan, including ones that return early with an error.
        struct PrewarmSample(Option<Instant>, usize);
        impl Drop for PrewarmSample {
            fn drop(&mut self) {
                crate::frame_counters::record_prewarm(self.0, self.1);
            }
        }
        let _sample = PrewarmSample(crate::frame_counters::start(), total);
        let mut iter = batch.into_iter().enumerate();
        while let Some((index, pending)) = iter.next() {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                self.requeue(std::iter::once(pending).chain(iter.map(|(_, rest)| rest)));
                return Err(rhapsody::Error::Timeout);
            };
            let Err(error) = pending.ticket.wait(remaining) else {
                continue;
            };
            let identity = match &pending.identity {
                PrewarmIdentity::Compute(entry) => format!(
                    "compute shader={} local_size={:?} ABI={}bytes/{}bindings",
                    entry.bindings.name,
                    entry.local_size,
                    entry.bindings.abi.parameter_bytes,
                    entry.bindings.abi.bindings.len(),
                ),
                PrewarmIdentity::Raster(entry) => format!(
                    "raster shader={} samples={:?} colors={:?} depth={:?} stencil={:?} view_mask={} ABI={}bytes/{}bindings",
                    entry.bindings.name,
                    entry.pass.samples,
                    entry.pass.formats.colors,
                    entry.pass.formats.depth,
                    entry.pass.formats.stencil,
                    entry.pass.formats.view_mask,
                    entry.bindings.abi.parameter_bytes,
                    entry.bindings.abi.bindings.len(),
                ),
                PrewarmIdentity::IndexedCount => "indexed-count platform adapter".to_owned(),
            };
            let message = format!(
                "pipeline prewarm failed [{}/{}] {}: {} ({:?}); compiler={:?}",
                index + 1,
                total,
                identity,
                error,
                error.kind(),
                self.compiler.stats(),
            );
            log::error!(target: "tiny_glade::pipeline", "{message}");
            // 集成测试可能未安装 logger；失败诊断仍需在实际运行日志可见。
            if !log::log_enabled!(target: "tiny_glade::pipeline", log::Level::Error) {
                eprintln!("{message}");
            }
            // Timeout and Failed both stay registered: the next call reports them again.
            self.requeue(std::iter::once(pending).chain(iter.map(|(_, rest)| rest)));
            return Err(error.into());
        }
        Ok(())
    }
    /// Registered tickets `prewarm` has not yet observed ready (diagnostics/tests).
    #[doc(hidden)]
    pub fn pending_ticket_count(&self) -> usize {
        self.pending_tickets.lock().len()
    }
    pub fn save_pipeline_cache(&self) -> rhapsody::Result<()> {
        self.disk_cache
            .as_ref()
            .map_or(Ok(()), PersistentPipelineCache::save)
    }
    pub fn compiler_stats(&self) -> rhapsody::pipeline::CompilerStats {
        self.compiler.stats()
    }

    pub fn passes(&self) -> &Arc<RenderPassHost> {
        &self.passes
    }

    pub fn raster_entry(&self, handle: &RasterPipeline) -> Option<Arc<RasterEntry>> {
        if handle.id >> 32 != self.pipeline_namespaces[1] { return None; }
        self.raster
            .lock()
            .get(((handle.id & 0xffff_ffff) as usize).checked_sub(1)?)
            .cloned()
    }

    pub fn with_sampler(mut self, name: &str, sampler: Arc<Sampler>) -> Self {
        self.samplers.insert(name.to_owned(), sampler);
        self.sampler_leases.invalidate(name);
        self
    }

    pub fn compute_pipeline(&self, handle: &ComputePipeline) -> Option<Arc<VulkanPipeline>> {
        self.compute_entry(handle)
            .and_then(|e| e.pipeline.try_get().ok())
    }

    pub fn compute_entry(&self, handle: &ComputePipeline) -> Option<Arc<ComputeEntry>> {
        if handle.id >> 32 != self.pipeline_namespaces[0] { return None; }
        self.compute
            .lock()
            .get(((handle.id & 0xffff_ffff) as usize).checked_sub(1)?)
            .cloned()
    }

    /// 该管线的反射登记表（`ComputePass::new` 里 `[arc+0x10]` 一族偏移读到的那份）。
    pub fn compute_registry(&self, handle: &ComputePipeline) -> Option<Arc<ResourceRegistry>> {
        self.compute_entry(handle).map(|e| e.registry.clone())
    }

    pub fn bindless_stats(&self) -> rhapsody::bindless::BindlessStats {
        self.pages.stats()
    }
    /// Intended retained descriptors in table order (sampled, storage image,
    /// storage buffer, sampler). Compare with heap live counts only after all
    /// graph work and retired command owners have completed. This does not
    /// count GPU in-flight leases or the weak residency lookup index.
    pub fn retained_bindless_counts(&self) -> [usize; 4] {
        let globals = self.frame_globals.lock().clone();
        let mut counts = globals.map_or([0; 4], |globals| globals.retained_bindless_counts());
        for page in &self.sampler_leases.state.lock().pages {
            counts[3] += page.leases.lock().leases.len();
        }
        counts
    }
    pub fn page_cursor(&self) -> rhapsody::bindless::BindlessPageCursor {
        self.pages.cursor()
    }
    pub fn frame_globals_snapshot(&self) -> Option<Arc<crate::frame_setup::FrameGlobals>> {
        self.frame_globals.lock().clone()
    }
    fn bindings(
        &self,
        original: &RhCompiled,
        abi: Arc<BindlessAbi>,
    ) -> rhapsody::Result<ShaderBindings> {
        let mut samplers = Vec::new();
        for slot in &abi.bindings {
            if slot.kind != BindingKind::Sampler {
                continue;
            }
            let name = original
                .reflection
                .descriptor_sets
                .get(slot.location.set as usize)
                .and_then(|set| {
                    set.iter()
                        .rev()
                        .find(|(binding, _)| *binding == slot.location.binding)
                })
                .map(|(_, binding)| binding.name.as_str())
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "sampler location absent from original reflection",
                    )
                })?;
            let sampler = match self.samplers.get(name) {
                Some(sampler) => sampler.clone(),
                None => self.device.get_built_in_sampler_by_name(name)?,
            };
            samplers
                .try_reserve(slot.parameter_count as usize)
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
            let name: Arc<str> = Arc::from(name);
            for element in 0..slot.parameter_count {
                samplers.push(SamplerBinding {
                    location: slot.location,
                    element,
                    name: name.clone(),
                    sampler: sampler.clone(),
                });
            }
        }
        Ok(ShaderBindings {
            name: Arc::from(
                original
                    .source_dependencies
                    .iter()
                    .find(|path| path.ends_with(".raster.hlsl") || path.ends_with(".cs.hlsl"))
                    .or_else(|| original.source_dependencies.first())
                    .map_or("<generated>", String::as_str),
            ),
            heap: self.heap.clone(),
            pages: self.pages.clone(),
            abi,
            samplers: Arc::new(samplers),
            sampler_pages: self.sampler_leases.clone(),
            frame_globals: self.frame_globals.clone(),
        })
    }

    fn file_desc(desc: &ComputePipelineDesc) -> FileComputeDesc {
        FileComputeDesc {
            source_path: desc.path.clone(),
            entry_point: desc.entry.clone(),
            defines: ShaderDefines(desc.defines.clone()),
        }
    }

    fn read(&self, relative: PathBuf) -> Result<CoreCompiled, ShaderError> {
        let path = self.root.join(relative);
        let bytes = std::fs::read(&path)
            .map_err(|e| anyhow::anyhow!("Error opening {}: {e}", path.display()))?;
        Ok(CoreCompiled(bytes))
    }
}

impl ShaderBackend for ShaderHost {
    fn load_compiled_compute(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<CoreCompiled, ShaderError> {
        self.read(Self::file_desc(desc).compiled_file_path())
    }

    fn load_compiled_raster(&self, desc: &RasterPipelineDesc) -> Result<CoreCompiled, ShaderError> {
        let file = shader_pipeline::pipeline_desc::RasterPipelineDesc {
            source_path: desc.path.clone(),
            vertex_entry: String::new(),
            fragment_entry: desc.entry.clone(),
            defines: ShaderDefines(desc.defines.clone()),
        };
        self.read(file.compiled_file_path())
    }

    fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        // O 0x1417388a0 `get_raster_shader_variants`：把 desc 克隆两份，各在 defines 末尾追加
        // `("SHADER_VARIANT_DEPTH_ONLY", "0")` / `("SHADER_VARIANT_DEPTH_ONLY", "1")`
        // （25 字节名字在 0x142ccd739，值字节 0x30 / 0x31 是立即数）。
        let with = |value: &str| {
            let mut variant = desc.clone();
            variant
                .defines
                .push(("SHADER_VARIANT_DEPTH_ONLY".to_owned(), value.to_owned()));
            variant
        };
        [with("0"), with("1")]
    }

    fn create_compute_pipeline(
        &self,
        name: &str,
        compiled: Arc<CoreCompiled>,
    ) -> rhapsody::Result<ComputePipeline> {
        if self.compute.lock().len() >= 4096 {
            return Err(rhapsody::Error::Timeout);
        }
        let decoded = RhCompiled::decode(&compiled.0).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("{name}: compiled shader decode: {error}"))
        })?;
        let package = BindlessShaderPackage::new(&self.heap, &decoded, 64)?;
        let stage = package
            .stages
            .iter()
            .find(|stage| stage.stage == ShaderStage::Compute)
            .ok_or_else(|| rhapsody::Error::invalid_usage("no compute stage in shader package"))?;
        let bytes = stage
            .spirv
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect::<Vec<_>>();
        let entry = std::ffi::CString::new(stage.entry_point.as_str())
            .map_err(|error| rhapsody::Error::invalid_usage_with("compute entry point", error))?;
        let bindings = self.bindings(&decoded, package.abi)?;
        let pipeline = self.compiler.defer_compute_spirv(bytes, entry, package.layout)?;
        let entry = Arc::new(ComputeEntry {
            pipeline,
            registry: Arc::new(registry_from_reflection(&decoded.reflection)),
            local_size: decoded.reflection.local_size.unwrap_or([1, 1, 1]),
            bindings,
            uniform_binding: decoded
                .reflection
                .uniform_block
                .as_ref()
                .map(|block| block.binding.binding),
        });
        let mut table = self.compute.lock();
        if table.len() >= 4096 {
            return Err(rhapsody::Error::Timeout);
        }
        table
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        self.sampler_leases.register(entry.bindings.samplers.as_slice());
        self.note_pending(entry.pipeline.clone(), PrewarmIdentity::Compute(entry.clone()))?;
        table.push(entry);
        Ok(ComputePipeline {
            id: (self.pipeline_namespaces[0] << 32) | table.len() as u64,
        })
    }

    fn create_raster_pipeline(
        &self,
        compiled: Arc<CoreCompiled>,
        key: &RasterPipelineKey,
    ) -> Result<RasterPipeline, ShaderError> {
        if self.raster.lock().len() >= 4096 {
            return Err(rhapsody::Error::Timeout.into());
        }
        // O 0x1416a3720：key.handle 是渲染通道句柄，key.selector 是 4 字节 `RasterPipelineOpts`。
        let pass = self.passes.get(key.handle).ok_or_else(|| {
            anyhow::anyhow!(
                "raster pipeline: render pass handle {} is not live",
                key.handle
            )
        })?;
        let decoded = RhCompiled::decode(&compiled.0)?;
        let package = BindlessShaderPackage::new(&self.heap, &decoded, 64)?;
        // The native Egui producer stores its actual 8-byte screen extent in
        // DrawPacket.unknown_bytes (B 0x1400def7e..0x1400defd0). Preserve the
        // package's reflected push ABI here; RasterHost forwards the packet's
        // bytes to ShaderBindings::parameters, and BindlessParameters::new
        // rejects any byte count differing from abi.original_push_bytes before
        // recording a draw. A pipeline with a push block is not itself missing
        // a producer; a draw with missing/wrong bytes still fails precisely.
        let stage_of = |wanted: ShaderStage| {
            package
                .stages
                .iter()
                .find(|s| s.stage == wanted)
                .ok_or_else(|| anyhow::anyhow!("raster pipeline: missing {wanted:?} stage"))
        };
        let vertex = stage_of(ShaderStage::Vertex)?;
        let fragment = stage_of(ShaderStage::Fragment)?;

        let layout = package.layout.clone();
        let bindings = self.bindings(&decoded, package.abi.clone())?;
        let module = |stage: &shader_pipeline::compiled::CompiledShaderStage| {
            let bytes = stage
                .spirv
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>();
            ShaderModule::from_spirv(self.device.clone(), &bytes).map_err(ShaderError::from)
        };
        let mut desc = RhRasterDesc::new(module(vertex)?, layout);
        desc.fragment = Some(module(fragment)?);
        desc.entry = std::ffi::CString::new(vertex.entry_point.as_str())?;
        desc.fragment_entry = Some(std::ffi::CString::new(fragment.entry_point.as_str())?);
        desc.dynamic_rendering = Some(pass.formats.clone());
        desc.samples = pass.samples;
        let (vertex_bindings, attributes) = vertex_input_from_reflection(&decoded.reflection)
            .map_err(|e| anyhow::anyhow!("raster vertex input: {e:?}"))?;
        desc.vertex_bindings = vertex_bindings;
        desc.vertex_attributes = attributes;
        RasterPipelineOpts(key.selector).apply_to(&mut desc, pass.color_attachments)?;
        // Vulkan ignores depth/stencil state for a rendering that has no such attachment (the
        // original default state keeps depth test/write on in every pass); rhapsody rejects the
        // combination, so drop exactly the bits whose attachment is absent.
        if pass.formats.depth == vk::Format::UNDEFINED {
            desc.depth_test = false;
            desc.depth_write = false;
        }
        if pass.formats.stencil == vk::Format::UNDEFINED {
            desc.stencil_test = false;
        }

        let layout = desc.layout.clone();
        let pipeline = self.compiler.defer_raster(desc)?;
        let entry = Arc::new(RasterEntry {
            pipeline,
            layout,
            registry: Arc::new(registry_from_reflection(&decoded.reflection)),
            pass,
            bindings,
            vertex_inputs: decoded
                .reflection
                .effective_vertex_inputs()
                .into_iter()
                .map(|(name, input)| {
                    let stride = input
                        .ty
                        .nbyte()
                        .and_then(|bytes| u32::try_from(bytes).ok())
                        .ok_or_else(|| {
                            anyhow::anyhow!("vertex input {name} has no byte size")
                        })?;
                    Ok((name.to_owned(), input.location, stride))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
            uniform_binding: decoded
                .reflection
                .uniform_block
                .as_ref()
                .map(|b| b.binding.binding),
        });
        let mut table = self.raster.lock();
        if table.len() >= 4096 {
            return Err(rhapsody::Error::Timeout.into());
        }
        table
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        self.sampler_leases.register(entry.bindings.samplers.as_slice());
        self.note_pending(entry.pipeline.clone(), PrewarmIdentity::Raster(entry.clone()))?;
        table.push(entry);
        Ok(RasterPipeline {
            id: (self.pipeline_namespaces[1] << 32) | table.len() as u64,
            compiled,
        })
    }
}

// ---------------------------------------------------------------- render graph

use country_core::rhapsody_types::{BufferGraphBackend, ImportedBuffer};
use rhapsody::{rg::RenderGraph as RhGraph, rg::ResourceSpec};
use std::{cell::RefCell, rc::Rc};

/// core `RenderGraph` 的后端：把 `import_unowned(id, access)` / `export` 落到 rhapsody 图声明上。
///
/// `id` 是 core 句柄里的 `id`（缓冲槽位序号，或带 [`TEXTURE_ID_TAG`] 的纹理 id），描述符取自宿主表里
/// 的真实资源；图里记录的 token 就是这个 id，供之后的物理资源映射反查。
pub struct GraphHost {
    graph: Rc<RefCell<RhGraph>>,
    buffers: Arc<BufferHost>,
    textures: Arc<TextureHost>,
}

impl GraphHost {
    pub fn new(
        graph: Rc<RefCell<RhGraph>>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
    ) -> Self {
        Self {
            graph,
            buffers,
            textures,
        }
    }
}

impl BufferGraphBackend for GraphHost {
    fn import_unowned(&mut self, id: u64, access: u8) -> ImportedBuffer {
        let descriptor = if id & TEXTURE_ID_TAG != 0 {
            let texture = self
                .textures
                .get_by_id(id)
                .expect("import_unowned: texture handle is not live");
            ResourceSpec::Image(texture.desc).descriptor()
        } else {
            let buffer = self
                .buffers
                .get_by_id(id)
                .expect("import_unowned: buffer handle is not live");
            ResourceSpec::Buffer(buffer.desc).descriptor()
        };
        let handle = self
            .graph
            .borrow_mut()
            // 名字按 id 区分：同一张图里导入多个资源时，同名会让后端的名字解析器串位。
            .import_unowned(
                format!("imported-{id:x}"),
                descriptor,
                graph_token(id),
                access,
            )
            .expect("import_unowned: graph rejected the resource");
        ImportedBuffer {
            resource: u64::from(handle.raw()),
            generation: 0,
            access: u32::from(access),
        }
    }

    fn export(&mut self, imported: ImportedBuffer, access: u8) {
        let mut graph = self.graph.borrow_mut();
        let handle = graph
            .resource_handle(imported.resource as u32)
            .expect("export: handle does not belong to this graph");
        graph
            .export(handle, access)
            .expect("export: handle does not belong to this graph");
    }
}

// ---------------------------------------------------------------- compute pass device

use country_core::{render::shader::ComputeShaderProgram, rhapsody_contract::ManagedHandle};

/// core `ComputePassDevice` 的宿主实现：把缓冲/纹理/着色器三张表合在一起。
pub struct DeviceHost {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl TextureDevice for DeviceHost {
    fn release_texture(&self, texture: &Texture) {
        self.textures.release(texture);
    }
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        data: &[u8],
    ) -> Result<Texture, TextureBackendError> {
        self.textures.create_texture(name, desc, data)
    }
}

impl ComputePassDevice for DeviceHost {
    /// `GraphicsDevice::get_vk_buffer`：核心的 `ManagedHandle` 里装的是缓冲 `id`。
    fn get_vk_buffer(&self, handle: &ManagedHandle) -> Option<u64> {
        let buffer = self.buffers.get_by_id(handle.0)?;
        Some(ash::vk::Handle::as_raw(buffer.raw()))
    }

    fn pipeline_registry(&self, program: &ComputeShaderProgram) -> Arc<ResourceRegistry> {
        self.shaders
            .compute_registry(&program.pipeline)
            .expect("pipeline_registry: program was not created through this host")
    }
}

use country_core::render::pass_builder::pass_resource_access::TextureDevice as PassTextureDevice;
use country_core::rhapsody_types::BackendError as PassBackendError;

impl PassTextureDevice for DeviceHost {
    /// `GraphicsDevice::get_vk_image_view(handle, None)`：默认视图。
    fn get_vk_image_view(&self, handle: &ManagedHandle) -> Result<u64, PassBackendError> {
        self.textures.default_view(handle.0)
    }

    fn managed_texture_desc(&self, handle: &ManagedHandle) -> TextureDesc {
        let texture = self
            .textures
            .get_by_id(handle.0)
            .expect("managed_texture_desc: texture handle is not live");
        TextureHost::core_desc(&texture)
    }
}

// ---------------------------------------------------------------- render graph passes

use country_core::render::rg_helpers::rg_boundary::{
    RenderFn, RenderGraph as CorePassGraph, RenderPassApi as CoreRenderPassApi, ResourceLease,
    StagingAlloc, StagingCopy,
};
use rhapsody::{
    resource::DynamicConstants, resource::DynamicConstantsChunk, rg::AshCommandBackend,
    rg::CommandBackend, rg::ManagedPassResources, rg::RenderPassApi as RhRenderPassApi,
};
/// core `rg_boundary::RenderGraph` 的宿主实现：pass 声明落到 rhapsody 图，动态常量走 rhapsody 的
/// `DynamicConstants`（原版 256 KiB 页、线性分配）。
pub struct GraphPasses<'session> {
    graph: Rc<RefCell<RhGraph>>,
    device: Arc<Device>,
    dynamic: Arc<Mutex<DynamicConstants>>,
    tables: Option<HostTables>,
    cache_session: Option<GraphSession<'session>>,
    // Same Arc table -> same real constants range within this declaration
    // host. Actual pages remain owned by the existing DynamicConstants pool;
    // the graph/session and commands retain each imported allocation.
    uniform_uploads: Vec<(Arc<[u8]>, DynamicConstantsChunk)>,
}
enum GraphSession<'a> {
    Exclusive(&'a mut rhapsody::rg::VulkanGraphSession),
    Shared(Rc<RefCell<rhapsody::rg::VulkanGraphSession>>),
}
enum SessionBorrow<'a> {
    Exclusive(&'a mut rhapsody::rg::VulkanGraphSession),
    Shared(std::cell::RefMut<'a, rhapsody::rg::VulkanGraphSession>),
}
impl GraphSession<'_> {
    fn borrow_mut(&mut self) -> rhapsody::Result<SessionBorrow<'_>> {
        match self {
            Self::Exclusive(session) => Ok(SessionBorrow::Exclusive(session)),
            Self::Shared(session) => {
                session
                    .try_borrow_mut()
                    .map(SessionBorrow::Shared)
                    .map_err(|_| {
                        rhapsody::Error::invalid_usage(
                            "frame session is already borrowed during a graph operation",
                        )
                    })
            }
        }
    }
}
impl std::ops::Deref for SessionBorrow<'_> {
    type Target = rhapsody::rg::VulkanGraphSession;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Exclusive(s) => s,
            Self::Shared(s) => s,
        }
    }
}
impl std::ops::DerefMut for SessionBorrow<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Exclusive(s) => s,
            Self::Shared(s) => s,
        }
    }
}

/// 计算 Pass 执行时需要按 id 反查的三张宿主表。
#[derive(Clone)]
pub struct HostTables {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
}

impl<'session> GraphPasses<'session> {
    pub fn new(graph: Rc<RefCell<RhGraph>>, device: Arc<Device>) -> Result<Self, rhapsody::Error> {
        let dynamic = Arc::new(Mutex::new(DynamicConstants::new(device.clone())?));
        Ok(Self::from_parts(graph, device, dynamic))
    }

    /// 生产计算入口沿用本 App 的常量 owner；独立消费者仍可使用 new。
    pub(crate) fn new_with_app_constants(
        graph: Rc<RefCell<RhGraph>>,
        device: Arc<Device>,
        raster: &crate::raster_host::RasterShared,
    ) -> rhapsody::Result<Self> {
        if device.is_device_lost() {
            return Err(rhapsody::Error::DeviceLost);
        }
        let dynamic = raster.dynamic_constants_owner(&device)?;
        Ok(Self::from_parts(graph, device, dynamic))
    }

    fn from_parts(
        graph: Rc<RefCell<RhGraph>>,
        device: Arc<Device>,
        dynamic: Arc<Mutex<DynamicConstants>>,
    ) -> Self {
        Self {
            graph,
            device,
            dynamic,
            tables: None,
            cache_session: None,
            uniform_uploads: Vec::new(),
        }
    }

    pub fn with_tables(mut self, tables: HostTables) -> Self {
        self.tables = Some(tables);
        self
    }

    /// Borrow the frame driver's real session only for declaration building.
    /// Drop this host before executing the frame; neither a second graph nor a
    /// second cache is introduced.
    pub fn with_session(
        mut self,
        session: &'session mut rhapsody::rg::VulkanGraphSession,
    ) -> rhapsody::Result<Self> {
        if !Arc::ptr_eq(&self.device, &session.backend.device()) {
            return Err(rhapsody::Error::invalid_usage(
                "upload cache belongs to another device",
            ));
        }
        self.cache_session = Some(GraphSession::Exclusive(session));
        Ok(self)
    }

    /// The frame facade may lazily import a library SSBO while a compute pass
    /// is being constructed. Hold the actual session identity, taking a short
    /// borrow only inside each declaration/upload operation, never around the
    /// caller's whole ComputePass construction closure.
    pub fn with_shared_session(
        mut self,
        session: Rc<RefCell<rhapsody::rg::VulkanGraphSession>>,
    ) -> rhapsody::Result<Self> {
        let device = session
            .try_borrow()
            .map_err(|_| rhapsody::Error::invalid_usage("frame session is already borrowed"))?
            .backend
            .device()
            .clone();
        if !Arc::ptr_eq(&self.device, &device) {
            return Err(rhapsody::Error::invalid_usage(
                "frame session belongs to another device",
            ));
        }
        self.cache_session = Some(GraphSession::Shared(session));
        Ok(self)
    }
}

use country_core::render::pass_builder::{
    compute_pass_builder::RgBuffer,
    pass_resource_access::RgTexture,
    rg_resources::{RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair},
};

// Compute PassEnv requests must use the same session that owns pass imports,
// uploads, named history and final submission. A second GraphRgResources with a
// separate session would make otherwise valid resource ids refer to another frame.
/// 原版渲染图里资源名只是调试标签，同一帧内同名资源可以多次创建（例如每个 Nani 外部子集各建一个
/// "Dispatch args"）；rhapsody 的 rg 要求名字唯一，所以这里在重名时追加 `#n` 后缀重试。
/// `declare` 在发现重名时尚未修改任何状态，重试是安全的。
fn declare_label_unique(
    backend: &mut rhapsody::rg::VulkanGraphBackend,
    graph: &mut rhapsody::rg::RenderGraph,
    name: &str,
    spec: rhapsody::rg::ResourceSpec,
) -> Result<rhapsody::rg::ResourceHandle, rhapsody::rg::BuildError> {
    use rhapsody::rg::BuildError;
    // The original declares many resources under one name (every Nani subset has
    // a "Dispatch args"). Retrying from `#2` each time made k duplicates cost
    // k(k+1)/2 declarations, so remember, per graph, where each name stopped.
    thread_local! {
        static NEXT_ATTEMPT: std::cell::RefCell<(Option<rhapsody::rg::GraphId>, std::collections::HashMap<String, u32>)> =
            std::cell::RefCell::new((None, std::collections::HashMap::new()));
    }
    let graph_id = graph.graph_id();
    let mut attempt = NEXT_ATTEMPT.with(|state| {
        let mut state = state.borrow_mut();
        if state.0 != Some(graph_id) {
            state.0 = Some(graph_id);
            state.1.clear();
        }
        state.1.get(name).copied().unwrap_or(1)
    });
    loop {
        let label = if attempt == 1 { name.to_owned() } else { format!("{name}#{attempt}") };
        match backend.declare(graph, &label, spec.clone()) {
            Err(BuildError::DuplicateResourceName(_)) => attempt += 1,
            other => {
                if other.is_ok() {
                    NEXT_ATTEMPT.with(|state| {
                        state.borrow_mut().1.insert(name.to_owned(), attempt + 1);
                    });
                }
                return other;
            }
        }
    }
}

impl RgResources for GraphPasses<'_> {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        let tables = self
            .tables
            .as_ref()
            .expect("frame resources require HostTables");
        let native = tables
            .textures
            .convert_for_graph(desc)
            .unwrap_or_else(|error| panic!("create_raw_texture({name}): {error:?}"));
        let mut session = self
            .cache_session
            .as_mut()
            .expect("frame resources require the frame driver's session")
            .borrow_mut()
            .expect("frame session already borrowed during declaration");
        let handle = declare_label_unique(
            &mut session.backend,
            &mut self.graph.borrow_mut(),
            name,
            ResourceSpec::Image(native),
        )
        .unwrap_or_else(|error| panic!("create_raw_texture({name}): {error:?}"));
        RgTexture {
            resource: handle.raw(),
            desc: *desc,
        }
    }

    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        let native = BufferHost::convert(desc)
            .unwrap_or_else(|error| panic!("create_raw_buffer({name}): {error}"));
        let mut session = self
            .cache_session
            .as_mut()
            .expect("frame resources require the frame driver's session")
            .borrow_mut()
            .expect("frame session already borrowed during declaration");
        let handle = declare_label_unique(
            &mut session.backend,
            &mut self.graph.borrow_mut(),
            name,
            ResourceSpec::Buffer(native),
        )
        .unwrap_or_else(|error| panic!("create_raw_buffer({name}): {error:?}"));
        RgBuffer {
            resource: handle.raw(),
        }
    }

    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        let tables = self
            .tables
            .as_ref()
            .ok_or_else(|| rhapsody::Error::invalid_usage("frame resources require HostTables"))?;
        let native = tables.textures.convert_for_graph(desc).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("invalid temporal texture: {error:?}"))
        })?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        let named = session.named_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexture {
            texture: RgTexture {
                resource: named.texture.raw().raw(),
                desc: *desc,
            },
            // B get_or_create_temporal @1404f702e / 1404f72d6:
            // 1 means reusable history, 0 means a new allocation. Preserve
            // the current session's stronger committed-content validity.
            flag: u8::from(named.history_valid),
        })
    }

    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        let tables = self
            .tables
            .as_ref()
            .ok_or_else(|| rhapsody::Error::invalid_usage("frame resources require HostTables"))?;
        let native = tables.textures.convert_for_graph(desc).map_err(|error| {
            rhapsody::Error::invalid_usage(format!("invalid temporal texture: {error:?}"))
        })?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        let pair = session.temporal_texture(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalTexturePair {
            current: RgTexture {
                resource: pair.current().raw().raw(),
                desc: *desc,
            },
            previous: RgTexture {
                resource: pair.previous().raw().raw(),
                desc: *desc,
            },
            // The core field retains its old name, but native consumers send
            // this byte directly to the shader's `history_valid` uniform.
            fresh: pair.history_valid(),
        })
    }

    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        let native = BufferHost::convert(desc)?;
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::invalid_usage("frame resources require the frame driver's session")
            })?;
        // Temporal buffers of the game are zero-filled on creation.
        let named = session.named_buffer_zeroed(&mut self.graph.borrow_mut(), name, native)?;
        Ok(TemporalBuffer {
            buffer: RgBuffer {
                resource: named.buffer.raw().raw(),
            },
            // B buffer instance @1404f643d / 1404f665a has the same polarity.
            flag: u8::from(named.history_valid),
        })
    }
}

impl CorePassGraph for GraphPasses<'_> {
    fn add_pass(&mut self, name: &'static str, resource: u32, access: u8, render: RenderFn) {
        let device = self.device.clone();
        let mut graph = self.graph.borrow_mut();
        let mut pass = graph.add_pass(name);
        // 原 `add_resource_access(id, access, 0)`：同步组 0。
        let resource = pass
            .resource_handle(resource)
            .expect("add_resource_access: resource does not belong to this graph");
        pass.add_resource_access(resource, access, 0)
            .expect("add_resource_access: resource does not belong to this graph");
        pass.render_with_result_context(move |ctx| {
            let managed = ctx
                .managed
                .expect("pass callback without managed resources");
            // SAFETY: 命令缓冲、资源与设备在回调期间由 recorder 持有存活。
            let mut backend = unsafe { AshCommandBackend::new(&device.raw_device()) };
            let command =
                <ash::vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let mut host = PassApiHost {
                api: RhRenderPassApi::new(&mut backend, command, 0, false),
                managed,
                error: None,
            };
            render(&mut host);
            host.error.map_or(Ok(()), Err)
        });
    }

    fn copy_from_staging(
        &mut self,
        name: &'static str,
        copies: Vec<StagingCopy>,
    ) -> rhapsody::Result<()> {
        use rhapsody::{rg::AccessRange, rg::Descriptor};
        let mut session = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
            .ok_or_else(|| {
                rhapsody::Error::unsupported("staging copies require the frame driver's session")
            })?;
        if copies.is_empty() {
            return Err(rhapsody::Error::invalid_usage("empty staging copy batch"));
        }
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("staging graph is already borrowed"))?;
        let mut declared = Vec::new();
        declared
            .try_reserve_exact(copies.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        for copy in copies {
            let chunk = copy
                .source
                .lease
                .as_ref()
                .and_then(ResourceLease::downcast_ref::<DynamicConstantsChunk>)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage("staging copy has no allocation lease")
                })?;
            let size = copy.source.bytes.len() as u64;
            if size == 0
                || ash::vk::Handle::as_raw(chunk.buffer.raw_handle()) != copy.source.buffer
                || chunk.offset != u64::from(copy.source.offset)
                || chunk.size != size
            {
                return Err(rhapsody::Error::invalid_usage(
                    "staging metadata differs from its allocation",
                ));
            }
            let target = graph.resource_handle(copy.target)?;
            let Descriptor::Buffer(desc) =
                graph.declaration().resources[target.raw() as usize].descriptor
            else {
                return Err(rhapsody::Error::invalid_usage(
                    "staging target is not a buffer",
                ));
            };
            if copy
                .dst_offset
                .checked_add(size)
                .is_none_or(|end| end > desc.word_at_00)
                || !chunk.offset.is_multiple_of(4)
                || !copy.dst_offset.is_multiple_of(4)
                || !size.is_multiple_of(4)
            {
                return Err(rhapsody::Error::invalid_usage(
                    "staging copy range is invalid",
                ));
            }
            chunk.write_bytes(&copy.source.bytes)?;
            let source = session.import_frame_buffer(&mut graph, chunk.buffer.clone(), 0x25)?;
            declared.push((source, target, chunk.offset, copy.dst_offset, size));
        }
        let mut pass = graph.add_pass(name);
        for &(source, target, src_offset, dst_offset, size) in &declared {
            pass.add_resource_access_range(
                source,
                0x25,
                0,
                AccessRange::Buffer {
                    offset: src_offset,
                    size,
                },
            )?;
            pass.add_resource_access_range(
                target,
                0x34,
                0,
                AccessRange::Buffer {
                    offset: dst_offset,
                    size,
                },
            )?;
        }
        let device = self.device.clone();
        pass.render_with_result_context(move |ctx| {
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("staging copy has no resources"))?;
            for &(source, target, src_offset, dst_offset, size) in &declared {
                let source = managed.buffer_for(source.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("staging source", error)
                })?;
                let target = managed.buffer_for(target.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("staging target", error)
                })?;
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle()),
                        source.raw_handle(),
                        target.raw_handle(),
                        &[vk::BufferCopy {
                            src_offset,
                            dst_offset,
                            size,
                        }],
                    );
                }
            }
            Ok(())
        });
        Ok(())
    }

    fn allocate_dynamic(&mut self, size: usize) -> rhapsody::Result<StagingAlloc> {
        let chunk = self.dynamic.lock().allocate(size as u64)?;
        let buffer = ash::vk::Handle::as_raw(chunk.buffer.raw_handle());
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        bytes.resize(size, 0);
        Ok(StagingAlloc {
            buffer,
            offset: u32::try_from(chunk.offset).map_err(|_| {
                rhapsody::Error::invalid_usage("staging offset exceeds the core allocation domain")
            })?,
            bytes,
            lease: Some(ResourceLease::new(Arc::new(chunk))),
        })
    }

    fn upload_complete_bytes(
        &mut self,
        buffer: u32,
        bytes: &[u8],
        source: u64,
        version: u64,
    ) -> rhapsody::Result<bool> {
        use rhapsody::{
            rg::AccessRange, rg::ContentDependency, rg::DependencyId, rg::ResultDependencies,
            rg::ResultNodeId, rg::ResultRequest,
        };
        let Some(mut session) = self
            .cache_session
            .as_mut()
            .map(GraphSession::borrow_mut)
            .transpose()?
        else {
            return Ok(false);
        };
        if bytes.is_empty() || !bytes.len().is_multiple_of(4) {
            return Err(rhapsody::Error::invalid_usage(
                "cached u32 upload must be nonempty and word aligned",
            ));
        }
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("upload graph is already borrowed"))?;
        let target = graph.resource_handle(buffer)?;
        let size = bytes.len() as u64;
        let mut parameters = Vec::new();
        parameters
            .try_reserve_exact(bytes.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        parameters.extend_from_slice(bytes);
        session.invalidate_result_dependency(DependencyId::External(source), version);
        let device = self.device.clone();
        let dynamic = self.dynamic.clone();
        let result = session.cached_result(
            &mut graph,
            ResultRequest {
                node: ResultNodeId(source),
                dependencies: ResultDependencies::Complete(vec![ContentDependency::external(
                    source, version,
                )]),
                parameters,
                shader_implementation: 1,
                shader_interface: 1,
            },
            ResourceSpec::Buffer(RhBufferDesc::new_gpu_only(
                size,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            )),
            0x25,
            move |session, graph, output| {
                let chunk = dynamic.lock().allocate(size)?;
                chunk.write_bytes(bytes)?;
                let source = session.import_frame_buffer(graph, chunk.buffer.clone(), 0x25)?;
                graph.subgraph("country.cpu-upload.produce", |scope| {
                    scope.input(
                        source,
                        AccessRange::Buffer {
                            offset: chunk.offset,
                            size,
                        },
                        0x25,
                    )?;
                    scope.interface_mut().implementation_signature = 1;
                    let mut pass = scope.graph().add_pass("Produce immutable CPU upload");
                    pass.add_resource_access_range(
                        source,
                        0x25,
                        0,
                        AccessRange::Buffer {
                            offset: chunk.offset,
                            size,
                        },
                    )?;
                    pass.add_resource_access(output, 0x34, 0)?;
                    let source_offset = chunk.offset;
                    pass.render_with_result_context(move |ctx| {
                        let managed = ctx.managed.ok_or_else(|| {
                            rhapsody::Error::invalid_usage("upload producer has no resources")
                        })?;
                        let output = managed.buffer_for(output.raw()).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("upload output", error)
                        })?;
                        let source = managed.buffer_for(source.raw()).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("upload staging source", error)
                        })?;
                        unsafe {
                            device.raw_device().cmd_copy_buffer(
                                <vk::CommandBuffer as ash::vk::Handle>::from_raw(
                                    ctx.command.handle(),
                                ),
                                source.raw_handle(),
                                output.raw_handle(),
                                &[vk::BufferCopy {
                                    src_offset: source_offset,
                                    dst_offset: 0,
                                    size,
                                }],
                            );
                        }
                        Ok(())
                    });
                    scope.output(output, AccessRange::Whole, 0x34)
                })
            },
        )?;
        let device = self.device.clone();
        graph.subgraph("country.cpu-upload.copy", |scope| {
            scope.interface_mut().implementation_signature = 1;
            scope.input(result.handle, AccessRange::Whole, 0x25)?;
            let mut pass = scope.graph().add_pass("Copy immutable CPU upload");
            pass.add_resource_access(result.handle, 0x25, 0)?;
            pass.add_resource_access_range(
                target,
                0x34,
                0,
                AccessRange::Buffer { offset: 0, size },
            )?;
            pass.render_with_result_context(move |ctx| {
                let managed = ctx.managed.ok_or_else(|| {
                    rhapsody::Error::invalid_usage("upload consumer has no resources")
                })?;
                let source = managed.buffer_for(result.handle.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("cached upload source", error)
                })?;
                let target = managed.buffer_for(target.raw()).map_err(|error| {
                    rhapsody::Error::invalid_usage_with("cached upload target", error)
                })?;
                unsafe {
                    device.raw_device().cmd_copy_buffer(
                        <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle()),
                        source.raw_handle(),
                        target.raw_handle(),
                        &[vk::BufferCopy {
                            src_offset: 0,
                            dst_offset: 0,
                            size,
                        }],
                    );
                }
                Ok(())
            });
            scope.output(target, AccessRange::Buffer { offset: 0, size }, 0x34)
        })?;
        Ok(true)
    }
}

struct PassApiHost<'a, B: CommandBackend> {
    api: RhRenderPassApi<'a, B>,
    managed: &'a ManagedPassResources,
    error: Option<rhapsody::Error>,
}

impl<B: CommandBackend> CoreRenderPassApi for PassApiHost<'_, B> {
    fn fill_buffer(&mut self, buffer: u32, range: Option<std::ops::Range<u64>>, value: u32) {
        let buffer = self
            .managed
            .buffer_for(buffer)
            .expect("fill_buffer: unresolved buffer");
        self.api
            .fill_buffer(
                buffer.raw_handle(),
                range.map(|r| (r.start, r.end - r.start)),
                value,
            )
            .expect("fill_buffer: unaligned range");
    }

    fn clear_color_image(&mut self, image: u32) {
        let texture = self
            .managed
            .texture_for(image)
            .expect("clear_color_image: unresolved image");
        let layers = match texture.desc.array_layers() {
            Ok(layers) => layers,
            Err(error) => {
                self.error.get_or_insert(error);
                return;
            }
        };
        self.api
            .clear_color_image(texture.raw_handle(), texture.desc.mip_levels, layers);
    }
}

// ---------------------------------------------------------------- compute pass execution

use country_core::render::pass_builder::compute_pass_builder::{
    ComputeExec, ComputeExecution, ExternalSlot, RecordedComputePass,
};
use country_core::render::pass_builder::pass_resource_access::PassBinding as CorePassBinding;

/// 核心的视图描述（各字段 `Option`）叠加到纹理的默认视图上。
pub(crate) fn rh_view_desc(
    texture: &rhapsody::resource::Texture,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::resource::TextureViewDesc> {
    project_view_desc(&texture.desc, view)
}

fn project_view_desc(
    texture: &rhapsody::resource::TextureDesc,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::resource::TextureViewDesc> {
    let mut out = rhapsody::resource::TextureViewDesc::new(texture)?;
    if let Some(v) = view.view_type {
        out.view_type = vk::ImageViewType::from_raw(v as i32);
    }
    if let Some(v) = view.usage {
        out.usage = vk::ImageUsageFlags::from_raw(v);
    }
    if let Some(v) = view.format {
        out.format = vk::Format::from_raw(v as i32);
    }
    if let Some(v) = view.aspect_mask {
        out.aspect = vk::ImageAspectFlags::from_raw(v);
    } else if out.aspect == (vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL) {
        // O rhapsody::raw::managed::texture::create_vk_image_view 0x140bdcba0 @140bdcc8d..9d:
        // an unspecified aspect is `vk_format_valid_aspect_flags(format)`, and DEPTH|STENCIL (6)
        // becomes DEPTH (2) alone (a sampled view may not carry both: VUID-01976).
        out.aspect = vk::ImageAspectFlags::DEPTH;
    }
    out.level_count = remaining_view_count(out.level_count, view.base_mip_level, view.level_count)?;
    out.layer_count = remaining_view_count(out.layer_count, view.base_layer, view.layer_count)?;
    out.base_mip_level = view.base_mip_level;
    out.base_layer = view.base_layer;
    out.validate(texture)?;
    Ok(out)
}

/// Native None means the remaining subresource count after the selected base,
/// not the whole image count plus that base (rg binding @14205f263..297).
fn remaining_view_count(total: u32, base: u32, explicit: Option<u32>) -> rhapsody::Result<u32> {
    let remaining = total.checked_sub(base).ok_or_else(|| {
        rhapsody::Error::invalid_usage("image view base exceeds its actual image")
    })?;
    let count = explicit.unwrap_or(remaining);
    if count == 0 || count > remaining {
        return Err(rhapsody::Error::invalid_usage(
            "image view count exceeds its actual remaining range",
        ));
    }
    Ok(count)
}

/// Original bind_individual_mips enumerates the exact selected range and
/// creates a level_count=1 view per element (@1418b4740..47f7). The current
/// fully-bound shader ABI additionally fills unused array slots with the last
/// actual view; it creates no owner or mip beyond the declared original range.
fn individual_mip_views(
    view: rhapsody::resource::TextureViewDesc,
    array_count: u32,
) -> rhapsody::Result<Vec<rhapsody::resource::TextureViewDesc>> {
    if view.level_count == 0 || array_count < view.level_count {
        return Err(rhapsody::Error::invalid_usage(
            "individual mip views exceed the actual shader array",
        ));
    }
    let mut views = Vec::new();
    views
        .try_reserve_exact(array_count as usize)
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for element in 0..array_count {
        let mut selected = view;
        selected.base_mip_level = view
            .base_mip_level
            .checked_add(element.min(view.level_count - 1))
            .ok_or_else(|| rhapsody::Error::invalid_usage("individual mip index overflow"))?;
        selected.level_count = 1;
        views.push(selected);
    }
    Ok(views)
}

/// Declaration-time projection of the same view used by rh_view_desc during
/// recording. The logical descriptor is available before GPU allocation; no
/// physical image or substitute owner is created to calculate this range.
pub(crate) fn compute_view_range(
    descriptor: rhapsody::rg::Descriptor,
    view: &country_core::rhapsody_types::TextureViewDesc,
) -> rhapsody::Result<rhapsody::rg::AccessRange> {
    use rhapsody::{rg::AccessRange, rg::Descriptor};
    let Descriptor::Image(image) = descriptor else {
        return Err(rhapsody::Error::invalid_usage(
            "compute image view refers to a buffer",
        ));
    };
    let AccessRange::Texture {
        level_count,
        layer_count,
        ..
    } = AccessRange::Whole.normalized(descriptor).map_err(|error| {
        rhapsody::Error::invalid_usage(format!("compute image shape: {error:?}"))
    })?
    else {
        unreachable!("image descriptor normalizes to a texture range")
    };
    let range = AccessRange::Texture {
        aspect_mask: view.aspect_mask.unwrap_or_else(|| {
            rhapsody::resource::vk_format_valid_aspect_flags(vk::Format::from_raw(
                image.words_at_00_through_18[2] as i32,
            ))
            .as_raw()
        }),
        base_mip_level: view.base_mip_level,
        level_count: remaining_view_count(level_count, view.base_mip_level, view.level_count)?,
        base_array_layer: view.base_layer,
        layer_count: remaining_view_count(layer_count, view.base_layer, view.layer_count)?,
    };
    range.validate(descriptor).map_err(|error| {
        rhapsody::Error::invalid_usage(format!("compute image view range: {error:?}"))
    })?;
    Ok(range)
}

/// 整帧实测需要的 bindless 描述符数(存储缓冲 600 不够 4096 够；采样图像 256 不够)。
/// 宿主预算，非原版常量；实际取值按设备上限收敛，见 [`adaptive_bindless_config`]。
/// 一帧的参数块约占 2700 个存储描述符，且描述符租约要保留到该帧 GPU 完成：多帧稳态下
/// 在途帧(`frames_in_flight` 最多 3) + 正在录制的一帧同时持有租约，4096 只够单帧，第 3~7 帧
/// 起即 `bindless descriptor table is full`。取 16384 覆盖 3 个在途帧 + 录制帧并留余量。
const BINDLESS_STORAGE_BUFFER_DEMAND: u32 = 16384;
/// 采样图像同理：按多帧稳态实测(第 22 帧起 `SampledImage descriptor table is full`)放大到 8192。
const BINDLESS_SAMPLED_IMAGE_DEMAND: u32 = 8192;

/// 各类描述符数取需求与设备 update-after-bind 上限(单 set、单 stage)的较小者，
/// 再把总数收进 stage 资源总数与全池总数：超出时先削存储缓冲，再削采样图像。
/// 上限足够的设备(如 lavapipe)得到完整需求；上限小的设备(如 Mac 的 1212)按自身能力
/// 收敛，不强行超限。
fn adaptive_bindless_config(device: &rhapsody::device::Device) -> BindlessConfig {
    let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
    // SAFETY: the instance and physical device belong to this live device.
    unsafe {
        device
            .instance()
            .raw_instance()
            .get_physical_device_properties2(device.physical_device(), &mut properties);
    }
    let base = BindlessConfig::default();
    let mut sampled_images = BINDLESS_SAMPLED_IMAGE_DEMAND
        .min(indexing.max_descriptor_set_update_after_bind_sampled_images)
        .min(indexing.max_per_stage_descriptor_update_after_bind_sampled_images)
        .max(base.sampled_images);
    let mut storage_buffers = BINDLESS_STORAGE_BUFFER_DEMAND
        .min(indexing.max_descriptor_set_update_after_bind_storage_buffers)
        .min(indexing.max_per_stage_descriptor_update_after_bind_storage_buffers)
        .max(base.storage_buffers);
    let budget = indexing
        .max_per_stage_update_after_bind_resources
        .min(indexing.max_update_after_bind_descriptors_in_all_pools);
    let fixed = base.storage_images + base.samplers;
    let mut excess = (sampled_images + storage_buffers + fixed).saturating_sub(budget);
    let cut = excess.min(storage_buffers.saturating_sub(base.storage_buffers));
    storage_buffers -= cut;
    excess -= cut;
    sampled_images -= excess.min(sampled_images.saturating_sub(base.sampled_images));
    BindlessConfig {
        sampled_images,
        storage_buffers,
        ..base
    }
}

/// 大布局受原生拒绝时只协商物理页大小，完整场景需求仍由多页保全。
fn negotiated_shader_heap(
    device: &Arc<rhapsody::device::Device>,
) -> rhapsody::Result<Arc<BindlessHeap>> {
    let requested = adaptive_bindless_config(device);
    let supported = |config: BindlessConfig| {
        let counts = [
            config.sampled_images,
            config.storage_images,
            config.storage_buffers,
            config.samplers,
        ];
        let types = [
            vk::DescriptorType::SAMPLED_IMAGE,
            vk::DescriptorType::STORAGE_IMAGE,
            vk::DescriptorType::STORAGE_BUFFER,
            vk::DescriptorType::SAMPLER,
        ];
        let bindings: Vec<_> = counts
            .iter()
            .enumerate()
            .map(|(index, count)| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(index as u32)
                    .descriptor_type(types[index])
                    .descriptor_count(*count)
                    .stage_flags(vk::ShaderStageFlags::ALL)
            })
            .collect();
        let flags = [vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING; 4];
        let mut flags_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL)
            .bindings(&bindings)
            .push_next(&mut flags_info);
        let mut support = vk::DescriptorSetLayoutSupport::default();
        unsafe {
            device
                .raw_device()
                .get_descriptor_set_layout_support(&info, &mut support)
        };
        support.supported != vk::FALSE
    };
    if supported(requested) {
        return BindlessHeap::new(device.clone(), requested);
    }
    let mut page = BindlessConfig::default();
    if !supported(page) {
        return Err(rhapsody::Error::unsupported(
            "device rejects the minimum shader page layout; original binding groups are retained",
        ));
    }
    // 始终保存确证 Supported 的下界；驱动打包不单调至多影响页利用率，不会绕过拒绝。
    let mut low = page.storage_buffers;
    let mut high = requested.storage_buffers.max(low);
    while low < high {
        let middle = low + (high - low).div_ceil(2);
        page.storage_buffers = middle;
        if supported(page) {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    page.storage_buffers = low;
    BindlessHeap::new(device.clone(), page)
}

/// 宿主偏差(DEVIATION)：原版整缓冲绑定（range = 缓冲全长），原机 maxStorageBufferRange
/// 足够大(Apple M3 为 4 GiB)。设备上限更小时(lavapipe 固定 128 MiB)整缓冲绑定会被
/// 描述符校验拒绝，此处把绑定范围钳到设备上限；设备上限足够时返回原值，行为与原版一致。
/// 钳制后着色器越界访问由 robustBufferAccess 兜底，仅影响小上限设备上超出部分的数据。
pub(crate) fn clamp_storage_binding_range(device: &rhapsody::device::Device, size: u64) -> u64 {
    size.min(u64::from(device.limits().max_storage_buffer_range))
}

pub(crate) fn upsert_shader_resource(
    resources: &mut Vec<(u32, BindlessResource)>,
    binding: u32,
    resource: BindlessResource,
) -> rhapsody::Result<()> {
    if let Some((_, old)) = resources.iter_mut().find(|(index, _)| *index == binding) {
        *old = resource;
    } else {
        resources
            .try_reserve(1)
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        resources.push((binding, resource));
    }
    Ok(())
}

/// Dispatch admission is shared by direct and indirect commands. In particular,
/// an empty initial payload is not a valid zero-filled replacement for a shader
/// whose first payload is supplied inside `dispatch_with`.
struct ComputeParameterState {
    ready: bool,
    error: Option<rhapsody::Error>,
}

impl ComputeParameterState {
    fn new(
        push_bytes: u32,
        initial: &[u8],
        bind: &mut dyn FnMut(&[u8]) -> rhapsody::Result<()>,
    ) -> rhapsody::Result<Self> {
        let deferred = push_bytes != 0 && initial.is_empty();
        if !deferred {
            bind(initial)?;
        }
        Ok(Self {
            ready: !deferred,
            error: None,
        })
    }

    fn push(&mut self, bytes: &[u8], bind: &mut dyn FnMut(&[u8]) -> rhapsody::Result<()>) {
        if self.error.is_none() {
            match bind(bytes) {
                Ok(()) => self.ready = true,
                Err(error) => self.error = Some(error),
            }
        }
    }

    fn can_dispatch(&mut self) -> bool {
        if self.error.is_some() {
            return false;
        }
        if !self.ready {
            self.error = Some(rhapsody::Error::invalid_usage(
                "compute dispatch requires a valid shader push parameter payload",
            ));
            return false;
        }
        true
    }
}

struct ComputeExecHost<'a> {
    device: &'a ash::Device,
    command: vk::CommandBuffer,
    local_size: [u32; 3],
    managed: &'a ManagedPassResources,
    bind_parameters: &'a mut dyn FnMut(&[u8]) -> rhapsody::Result<()>,
    parameters: ComputeParameterState,
}

impl ComputeExec for ComputeExecHost<'_> {
    /// `[device+0xaba]` 的调试开关（每次 dispatch 前全管线屏障）；本宿主不开启。
    fn full_barrier_enabled(&self) -> bool {
        false
    }
    fn issue_full_pipeline_barrier(&mut self) {}
    fn count_dispatch(&mut self) {}
    fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32) {
        if self.parameters.can_dispatch() {
            unsafe { self.device.cmd_dispatch(self.command, x, y, z) };
        }
    }
    fn cmd_dispatch_indirect(&mut self, buffer: u32, offset: u64) -> Option<()> {
        if !self.parameters.can_dispatch() {
            return Some(());
        }
        let buffer = self.managed.buffer_for(buffer).ok()?;
        unsafe {
            self.device
                .cmd_dispatch_indirect(self.command, buffer.raw_handle(), offset)
        };
        Some(())
    }
    fn local_size(&self) -> [u32; 3] {
        self.local_size
    }
    fn push_constants(&mut self, bytes: &[u8]) {
        self.parameters.push(bytes, self.bind_parameters);
    }
}

impl GraphPasses<'_> {
    /// 登记一个已声明完毕的计算 Pass：访问声明进图；回调里绑定管线与描述符集（set 0）、
    /// 推 push constants、按模式 dispatch。`pipeline` 是该 Pass 着色器程序创建出的管线句柄。
    pub fn add_compute(
        &mut self,
        pass: RecordedComputePass,
        pipeline: &ComputePipeline,
    ) -> rhapsody::Result<()> {
        let tables = self
            .tables
            .clone()
            .ok_or_else(|| rhapsody::Error::invalid_usage("add_compute requires HostTables"))?;
        let entry = tables.shaders.compute_entry(pipeline).ok_or_else(|| {
            rhapsody::Error::invalid_usage("compute pipeline was not created through this host")
        })?;
        let RecordedComputePass {
            name,
            builder,
            bindings,
            access_ranges,
            inactive_texture_bindings,
            undefined_texture_bindings,
            preserving_buffers,
            preserving_texture_bindings,
            external_bindings,
            device_address_reads,
            uniform_binder,
            push_constants,
            dispatch,
            ..
        } = pass;
        // The builder is dropped right after this; move its accesses out
        // instead of cloning the whole vector per compute pass.
        let accesses = {
            let mut builder = builder;
            std::mem::take(&mut builder.accesses)
        };

        let device = self.device.clone();
        let dynamic = self.dynamic.clone();
        let mut graph = self
            .graph
            .try_borrow_mut()
            .map_err(|_| rhapsody::Error::invalid_usage("compute graph is already borrowed"))?;
        let mut external_resources = Vec::new();
        let mut external_accesses = Vec::new();
        external_resources
            .try_reserve_exact(external_bindings.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        external_accesses
            .try_reserve_exact(external_bindings.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        let mut external_writes = false;
        // These immutable uniforms are already native graph resources with an
        // exact imported range. Their descriptor lease must retain that role,
        // rather than also treating the same bytes as a private upload read.
        let mut declared_uniform_bindings = Vec::new();
        for external in external_bindings {
            use rhapsody::rg::AccessRange;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "external compute resources require the frame driver's session",
                    )
                })?;
            let kind = entry
                .bindings
                .abi
                .slot(BindingLocation {
                    set: 0,
                    binding: external.binding,
                })
                .map(|slot| slot.kind)
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "external compute binding is absent from shader ABI",
                    )
                })?;
            let (resource, handle, range, code) = match external.slot {
                ExternalSlot::UniformBytes(bytes) => {
                    declared_uniform_bindings.push(external.binding);
                    if !matches!(
                        kind,
                        BindingKind::StorageBuffer | BindingKind::UniformBuffer
                    ) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform differs from shader ABI",
                        ));
                    }
                    if bytes.is_empty() {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform has an empty range",
                        ));
                    }
                    let (buffer, offset, size) = if let Some((_, chunk)) = self
                        .uniform_uploads
                        .iter()
                        .find(|(source, _)| Arc::ptr_eq(source, &bytes))
                    {
                        (chunk.buffer.clone(), chunk.offset, chunk.size)
                    } else {
                        // Bounded declaration cache, not another allocator or
                        // descriptor heap. Original DoF uses one shared table.
                        if self.uniform_uploads.len() == 64 {
                            return Err(rhapsody::Error::unsupported(
                                "external uniform declaration budget exceeded",
                            ));
                        }
                        self.uniform_uploads
                            .try_reserve(1)
                            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
                        let chunk = self.dynamic.lock().push_slice(&bytes)?;
                        let range = (chunk.buffer.clone(), chunk.offset, chunk.size);
                        self.uniform_uploads.push((bytes, chunk));
                        range
                    };
                    let expected = if kind == BindingKind::UniformBuffer {
                        vk::BufferUsageFlags::UNIFORM_BUFFER
                    } else {
                        vk::BufferUsageFlags::STORAGE_BUFFER
                    };
                    if !Arc::ptr_eq(&buffer.device(), &self.device)
                        || !buffer.usage.contains(expected)
                        || offset.checked_add(size).is_none_or(|end| end > buffer.size)
                    {
                        return Err(rhapsody::Error::invalid_usage(
                            "external uniform physical range/device mismatch",
                        ));
                    }
                    let handle = session.import_frame_buffer(&mut graph, buffer.clone(), 0x1a)?;
                    (
                        BindlessResource::Buffer {
                            buffer,
                            offset,
                            size,
                        },
                        handle,
                        AccessRange::Buffer { offset, size },
                        0x1a,
                    )
                }
                ExternalSlot::Buffer {
                    vk_buffer,
                    offset,
                    range,
                    handle,
                    ..
                } => {
                    if !matches!(
                        kind,
                        BindingKind::StorageBuffer | BindingKind::UniformBuffer
                    ) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer differs from shader ABI",
                        ));
                    }
                    let buffer = tables.buffers.get_by_id(handle.0).ok_or_else(|| {
                        rhapsody::Error::invalid_usage("external compute buffer is not live")
                    })?;
                    if ash::vk::Handle::as_raw(buffer.raw_handle()) != vk_buffer {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer differs from its owner",
                        ));
                    }
                    let size = if range == vk::WHOLE_SIZE {
                        buffer.size.checked_sub(offset).ok_or_else(|| {
                            rhapsody::Error::invalid_usage(
                                "external compute buffer offset exceeds size",
                            )
                        })?
                    } else {
                        range
                    };
                    if size == 0 || offset.checked_add(size).is_none_or(|end| end > buffer.size) {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute buffer range exceeds its owner",
                        ));
                    }
                    let buffer = buffer.shared();
                    let handle = session.import_frame_buffer(&mut graph, buffer.clone(), 0x37)?;
                    external_writes = true;
                    (
                        BindlessResource::Buffer {
                            buffer,
                            offset,
                            size: clamp_storage_binding_range(&self.device, size),
                        },
                        handle,
                        AccessRange::Buffer { offset, size },
                        0x37,
                    )
                }
                ExternalSlot::Image {
                    sampler,
                    image_view,
                    layout,
                    handle,
                    ..
                } => {
                    if sampler != 0 {
                        return Err(rhapsody::Error::unsupported(
                            "combined external image sampler is not a separate shader binding",
                        ));
                    }
                    let view = tables.textures.owned_view(handle.0, image_view)?;
                    let (expected, code) = match kind {
                        BindingKind::SampledImage => {
                            (vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, 0x22)
                        }
                        BindingKind::StorageImage => {
                            external_writes = true;
                            (vk::ImageLayout::GENERAL, 0x37)
                        }
                        _ => {
                            return Err(rhapsody::Error::invalid_usage(
                                "external compute image differs from shader ABI",
                            ));
                        }
                    };
                    if expected.as_raw() != layout as i32 {
                        return Err(rhapsody::Error::invalid_usage(
                            "external compute image layout differs from shader ABI",
                        ));
                    }
                    let range = AccessRange::Texture {
                        aspect_mask: view.desc.aspect.as_raw(),
                        base_mip_level: view.desc.base_mip_level,
                        level_count: view.desc.level_count,
                        base_array_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    };
                    let handle =
                        session.import_frame_texture(&mut graph, view.texture.clone(), code)?;
                    (
                        entry.bindings.image(external.binding, view)?,
                        handle,
                        range,
                        code,
                    )
                }
                ExternalSlot::Other(_) => {
                    return Err(rhapsody::Error::unsupported(
                        "external compute resource kind is not connected",
                    ));
                }
            };
            external_accesses.push((handle, range, code));
            upsert_shader_resource(&mut external_resources, external.binding, resource)?;
        }
        // Original global set 1/b1 is a storage view of the per-frame FrameConstants
        // block (wall/roof chain). Unlike the uniform b0 view, a storage lease must be
        // covered by a declared graph access of its owner, so import the exact frame
        // constants range as a shader read of this pass.
        if entry
            .bindings
            .abi
            .slot(BindingLocation { set: 1, binding: 0 })
            .is_some()
            || entry
                .bindings
                .abi
                .slot(BindingLocation { set: 1, binding: 1 })
                .is_some()
        {
            let (buffer, offset, size) = entry.bindings.frame_storage_view()?;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "frame constants storage view requires the frame driver's session",
                    )
                })?;
            let handle = session.import_frame_buffer(&mut graph, buffer, 0x1a)?;
            external_accesses.push((
                handle,
                rhapsody::rg::AccessRange::Buffer { offset, size },
                0x1a,
            ));
        }
        // BDA pointers are not descriptor bindings, but their physical reads
        // still participate in this graph's synchronization and owner program.
        // The same import/session owns cancellation and completion retirement.
        let mut device_address_accesses = Vec::new();
        device_address_accesses
            .try_reserve_exact(device_address_reads.len())
            .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        for chunk in device_address_reads {
            if chunk.size == 0
                || chunk
                    .offset
                    .checked_add(chunk.size)
                    .is_none_or(|end| end > chunk.buffer.size)
            {
                return Err(rhapsody::Error::invalid_usage("invalid BDA upload range"));
            }
            // Validate actual BDA admission even if a future caller bypasses
            // the terrain uploader's own device_address() call.
            chunk.device_address()?;
            let mut session = self
                .cache_session
                .as_mut()
                .map(GraphSession::borrow_mut)
                .transpose()?
                .ok_or_else(|| {
                    rhapsody::Error::unsupported(
                        "BDA upload ranges require the frame driver's session",
                    )
                })?;
            let handle = session.import_frame_buffer(&mut graph, chunk.buffer.clone(), 0x1a)?;
            device_address_accesses.push((
                handle,
                rhapsody::rg::AccessRange::Buffer {
                    offset: chunk.offset,
                    size: chunk.size,
                },
            ));
        }
        for range in &access_ranges {
            if bindings.get(range.access_index as usize).is_none() {
                return Err(rhapsody::Error::invalid_usage(
                    "compute binding range index is invalid",
                ));
            }
        }
        // Native bind_texture_with_view appends its access and its ImageView
        // together. Match each occurrence once, preserving repeated views of
        // the same image (e.g. read mip N / write mip N+1) in original order.
        // Synchronization code/group remain native values; the legacy API
        // does not invent content initialization from a Vulkan RW mask.
        let mut used_image_views = vec![false; bindings.len()];
        let mut declared_accesses = Vec::with_capacity(accesses.len());
        for access in &accesses {
            let view = bindings.iter().enumerate().find_map(|(index, binding)| {
                match binding {
                    CorePassBinding::ImageView { resource, write, view }
                        if !used_image_views[index] && *resource == access.resource
                            && *write == country_core::render::pass_builder::pass_resource_access::access_is_write(access.access) =>
                        Some((index, view)),
                    _ => None,
                }
            });
            let mut contents_unused = false;
            let mut allow_undefined = false;
            let mut preserves_texture = false;
            let range = if let Some((index, view)) = view {
                used_image_views[index] = true;
                contents_unused = inactive_texture_bindings.contains(&(index as u8));
                allow_undefined = undefined_texture_bindings.contains(&(index as u8));
                if allow_undefined
                    && (contents_unused || rhapsody::command::is_write(access.access))
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "undefined texture read metadata conflicts with its actual access",
                    ));
                }
                preserves_texture = preserving_texture_bindings.contains(&(index as u8));
                let resource = graph.resource_handle(access.resource)?;
                // A storage-image (write) view that is not `bind_individual_mips` and
                // names no level count is a single `RWTexture2D[Array]` operand: the
                // shader can only address the view's base mip, so the pass touches
                // exactly one level even though the native view spans the remaining
                // chain. Declaring the whole chain would demand (preserving RMW) or
                // claim (plain write) contents of levels the pass can never reach.
                // Evidence (original 0x140533b10 / ImageBarrier::new 0x140bd65e0): the original
                // declares access per whole image and tracks no mip contents, so this range is a
                // host-side refinement of rhapsody's content contract (shader semantics), not an
                // original behavior; removing it makes read-storage passes (Water deferred rt,
                // Volumetrics) demand mips 1..5 before "Generate mips" writes them.
                let storage_single_level = if let CorePassBinding::ImageView { write: true, .. } =
                    &bindings[index]
                {
                    view.level_count.is_none() && !view.bind_individual_mips
                } else {
                    false
                };
                let view_for_range;
                let view = if storage_single_level {
                    view_for_range = view.level_count(Some(1));
                    &view_for_range
                } else {
                    view
                };
                compute_view_range(
                    graph.declaration().resources[resource.raw() as usize].descriptor,
                    view,
                )?
            } else {
                // Storage buffers share the same existing per-binding inactive
                // intent. Match each native read occurrence independently so an
                // active alias still tightens the graph's content requirement.
                if let Some(index) =
                    bindings
                        .iter()
                        .enumerate()
                        .find_map(|(index, binding)| match binding {
                            CorePassBinding::BufferRead { resource }
                            | CorePassBinding::Buffer { resource }
                                if !used_image_views[index]
                                    && *resource == access.resource
                                    && !rhapsody::command::is_write(access.access) =>
                            {
                                Some(index)
                            }
                            _ => None,
                        })
                {
                    used_image_views[index] = true;
                    contents_unused = inactive_texture_bindings.contains(&(index as u8));
                }
                rhapsody::rg::AccessRange::Whole
            };
            declared_accesses.push((
                *access,
                range,
                contents_unused,
                allow_undefined,
                preserves_texture,
            ));
        }
        if bindings.iter().enumerate().any(|(index, binding)| {
            matches!(binding, CorePassBinding::ImageView { .. }) && !used_image_views[index]
        }) {
            return Err(rhapsody::Error::invalid_usage(
                "compute image view has no matching native resource access",
            ));
        }
        let mut declared = graph.add_pass(name);
        declared.on_queue(rhapsody::device::QueueClass::Compute);
        for (a, range, contents_unused, allow_undefined, preserves_texture) in &declared_accesses {
            let resource = declared.resource_handle(a.resource)?;
            if *contents_unused && !rhapsody::command::is_write(a.access) {
                declared.add_inactive_resource_read_range(resource, a.access, a.group, *range)?;
            } else if *allow_undefined && !rhapsody::command::is_write(a.access) {
                declared.add_undefined_content_read_range(resource, a.access, a.group, *range)?;
            } else if (*preserves_texture || preserving_buffers.contains(&a.resource))
                && rhapsody::command::is_write(a.access)
            {
                declared
                    .add_preserving_resource_access_range(resource, a.access, a.group, *range)?;
            } else {
                declared.add_resource_access_range(resource, a.access, a.group, *range)?;
            }
        }
        for (resource, range, code) in external_accesses {
            declared.add_resource_access_range(resource, code, 0, range)?;
        }
        for (resource, range) in device_address_accesses {
            declared.add_resource_access_range(resource, 0x1a, 1, range)?;
        }
        if external_writes {
            declared.side_effect();
        }
        declared.render_with_result_context(move |ctx| {
            let managed = ctx.managed.ok_or_else(|| {
                rhapsody::Error::invalid_usage("compute callback without managed resources")
            })?;
            let diagnose = |stage: &str, error: &rhapsody::Error| {
                log::error!(
                    "compute {name}: {stage}: {error:?}; heap {:?}",
                    entry.bindings.heap.stats()
                );
            };
            let pipeline = entry
                .pipeline
                .try_get()
                .inspect_err(|error| diagnose("pipeline.try_get", error))?;
            let command = <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());
            let mut resources = Vec::new();
            let mut image_arrays: Vec<(u32, Vec<BindlessResource>)> = Vec::new();
            for range in &access_ranges {
                let resource = match bindings[range.access_index as usize] {
                    CorePassBinding::Buffer { resource }
                    | CorePassBinding::BufferRead { resource } => {
                        let buffer = managed.buffer_for(resource).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("compute buffer binding", error)
                        })?;
                        BindlessResource::Buffer {
                            buffer: buffer.shared(),
                            offset: 0,
                            size: clamp_storage_binding_range(&device, buffer.size),
                        }
                    }
                    CorePassBinding::ImageView {
                        resource,
                        view,
                        write,
                    } => {
                        let texture = managed.texture_for(resource).map_err(|error| {
                            rhapsody::Error::invalid_usage_with("compute image binding", error)
                        })?;
                        // Same single-operand storage view the graph declaration above
                        // narrowed: the lease must be covered by the declared range.
                        let view =
                            if write && view.level_count.is_none() && !view.bind_individual_mips {
                                view.level_count(Some(1))
                            } else {
                                view
                            };
                        let desc = rh_view_desc(&texture, &view)?;
                        if view.bind_individual_mips {
                            let location = BindingLocation {
                                set: 0,
                                binding: range.resource_index,
                            };
                            let count = entry
                                .bindings
                                .abi
                                .slot(location)
                                .ok_or_else(|| {
                                    rhapsody::Error::invalid_usage(
                                        "individual mip binding is absent from the shader ABI",
                                    )
                                })?
                                .parameter_count;
                            if image_arrays
                                .iter()
                                .any(|(binding, _)| *binding == range.resource_index)
                                || resources
                                    .iter()
                                    .any(|(binding, _)| *binding == range.resource_index)
                            {
                                return Err(rhapsody::Error::invalid_usage(
                                    "individual mip binding has conflicting values",
                                ));
                            }
                            let mut elements = Vec::new();
                            for selected in individual_mip_views(desc, count)? {
                                let selected = texture.image_view(&selected)?;
                                elements.push(entry.bindings.image_for_pass(
                                    &ctx,
                                    resource,
                                    range.resource_index,
                                    selected.view.clone(),
                                )?);
                            }
                            image_arrays.push((range.resource_index, elements));
                            continue;
                        }
                        let view = texture.image_view(&desc)?;
                        entry.bindings.image_for_pass(
                            &ctx,
                            resource,
                            range.resource_index,
                            view.view.clone(),
                        )?
                    }
                };
                upsert_shader_resource(&mut resources, range.resource_index, resource)?;
            }
            for (binding, resource) in external_resources {
                upsert_shader_resource(&mut resources, binding, resource)?;
            }
            if let (Some(uniform), Some(binding)) = (uniform_binder, entry.uniform_binding) {
                let chunk = dynamic.lock().allocate_with_data(uniform.data())?;
                upsert_shader_resource(
                    &mut resources,
                    binding,
                    BindlessResource::Buffer {
                        buffer: chunk.buffer.clone(),
                        offset: chunk.offset,
                        size: chunk.size,
                    },
                )?;
            }
            // SAFETY: pipeline and command belong to this device; graph declares the accessed resources.
            unsafe {
                device.raw_device().cmd_bind_pipeline(
                    command,
                    vk::PipelineBindPoint::COMPUTE,
                    pipeline.raw_handle(),
                );
            }
            let snapshot = entry.bindings.snapshot();
            let mut cursor = entry.bindings.pages.cursor();
            let mut group_resources = Vec::new();
            group_resources
                .try_reserve(
                    resources.len()
                        + image_arrays
                            .iter()
                            .map(|(_, values)| values.len())
                            .sum::<usize>(),
                )
                .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
            for (binding, resource) in &resources {
                group_resources.push(ShaderResourceBinding {
                    location: BindingLocation {
                        set: 0,
                        binding: *binding,
                    },
                    element: 0,
                    resource: resource.clone(),
                    declared: declared_uniform_bindings.contains(binding),
                });
            }
            for (binding, elements) in &image_arrays {
                if resources.iter().any(|(scalar, _)| scalar == binding) {
                    return Err(rhapsody::Error::invalid_usage(
                        "individual mip binding conflicts with a scalar descriptor",
                    ));
                }
                for (element, resource) in elements.iter().enumerate() {
                    group_resources.push(ShaderResourceBinding {
                        location: BindingLocation {
                            set: 0,
                            binding: *binding,
                        },
                        element: element as u32,
                        resource: resource.clone(),
                        declared: false,
                    });
                }
            }
            let mut parameters: Option<rhapsody::bindless::PreparedBindlessParameters> = None;
            let mut bind_parameters = |push: &[u8]| -> rhapsody::Result<()> {
                let next = snapshot
                    .prepare(
                        &mut cursor,
                        push,
                        group_resources.iter().map(|value| {
                            (
                                value.location,
                                value.element,
                                &value.resource,
                                value.declared,
                            )
                        }),
                        parameters.as_ref(),
                        &dynamic,
                    )
                    .inspect_err(|error| diagnose("parameters.complete_group", error))?;
                // 原访问范围与上传角色已在同一组内保留；迁页后不使用旧数值下标。
                unsafe { next.bind_pass(&ctx, pipeline.clone())? };
                parameters = Some(next);
                Ok(())
            };
            let parameter_state = ComputeParameterState::new(
                entry.bindings.abi.original_push_bytes,
                &push_constants,
                &mut bind_parameters,
            )?;
            let mut exec = ComputeExecHost {
                device: &device.raw_device(),
                command,
                local_size: entry.local_size,
                managed,
                bind_parameters: &mut bind_parameters,
                parameters: parameter_state,
            };
            // An explicit initial payload is already bound. Otherwise the first
            // callback push must bind a valid payload before either dispatch kind.
            ComputeExecution {
                push_constants: Vec::new(),
                dispatch,
            }
            .execute(&mut exec);
            exec.parameters.error.map_or(Ok(()), Err)
        });
        Ok(())
    }
}

impl rhapsody::resource::GpuUse for RasterEntry {
    fn visit_uses(&self, visit: &mut rhapsody::resource::UseVisitor<'_>) -> rhapsody::Result<()> {
        if let Ok(pipeline) = self.pipeline.try_get() {
            pipeline.visit_uses(visit)?;
        }
        Ok(())
    }
}
impl rhapsody::resource::GpuUse for ComputeEntry {
    fn visit_uses(&self, visit: &mut rhapsody::resource::UseVisitor<'_>) -> rhapsody::Result<()> {
        if let Ok(pipeline) = self.pipeline.try_get() {
            pipeline.visit_uses(visit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod buffer_description_tests {
    use super::*;
    #[test]
    fn uniform_buffer_descriptions_admit_the_current_storage_heap_lowering() {
        for (usage, physical) in [(0x12, 0x32), (0x32, 0x32), (0x20, 0x20), (0, 0)] {
            let desc = BufferDesc::new_gpu_only(432, usage).alignment(4);
            let converted = BufferHost::convert(&desc).unwrap();
            assert_eq!(converted.usage.as_raw(), physical);
            assert_eq!(converted.size, 432);
            assert_eq!(converted.alignment, 4);
            assert_eq!(desc.usage, usage, "native/core descriptor must stay intact");
        }
    }

    #[test]
    fn buffer_description_conversion_preserves_all_memory_locations() {
        for memory in 1..=4 {
            let desc = BufferDesc {
                size: 256,
                usage: 0x20,
                memory,
                alignment: 32,
                gpu_only: false,
            };
            let converted = BufferHost::convert(&desc).unwrap();
            assert_eq!(converted.memory_location.as_raw(), memory);
            assert_eq!(converted.alignment, 32);
            assert_eq!(converted.size, 256);
        }
    }
    #[test]
    fn buffer_description_conversion_rejects_unknown_memory_and_truncating_alignment() {
        for memory in [0, 5, u8::MAX] {
            let error = BufferHost::convert(&BufferDesc {
                size: 16,
                memory,
                ..Default::default()
            })
            .unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        for alignment in [3, 6, 65_536, usize::MAX] {
            let error = BufferHost::convert(&BufferDesc {
                size: 16,
                memory: 1,
                alignment,
                ..Default::default()
            })
            .unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        }
        for alignment in [0, 1, 32_768] {
            let converted = BufferHost::convert(&BufferDesc {
                size: 16,
                memory: 1,
                alignment,
                ..Default::default()
            })
            .unwrap();
            assert_eq!(usize::from(converted.alignment), alignment);
        }
    }
}

#[cfg(test)]
mod compute_parameter_tests {
    use super::ComputeParameterState;

    #[test]
    fn callback_payload_is_bound_before_dispatch_without_a_synthetic_initial_value() {
        let mut payloads = Vec::new();
        let mut bind = |bytes: &[u8]| {
            assert_eq!(bytes.len(), 20);
            payloads.push(bytes.to_vec());
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        assert!(!state.ready);
        state.push(&[7; 20], &mut bind);
        assert!(state.can_dispatch());
        state.push(&[9; 20], &mut bind);
        assert!(state.can_dispatch());
        assert_eq!(payloads, vec![vec![7; 20], vec![9; 20]]);
        assert!(state.error.is_none());
    }

    #[test]
    fn dispatch_before_first_payload_is_rejected_and_a_later_push_cannot_hide_it() {
        let mut bind_calls = 0;
        let mut bind = |_: &[u8]| {
            bind_calls += 1;
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        assert!(!state.can_dispatch());
        state.push(&[1; 20], &mut bind);
        assert!(!state.can_dispatch());
        assert_eq!(bind_calls, 0);
        assert!(matches!(
            state.error,
            Some(rhapsody::Error::InvalidUsage { .. })
        ));
    }

    #[test]
    fn invalid_callback_payload_keeps_the_binding_failure_and_suppresses_dispatch() {
        let mut bind_calls = 0;
        let mut bind = |bytes: &[u8]| {
            bind_calls += 1;
            if bytes.len() != 20 {
                return Err(rhapsody::Error::invalid_usage(
                    "wrong callback payload size",
                ));
            }
            Ok(())
        };
        let mut state = ComputeParameterState::new(20, &[], &mut bind).unwrap();
        state.push(&[1; 4], &mut bind);
        assert!(!state.can_dispatch());
        state.push(&[2; 20], &mut bind);
        assert!(!state.can_dispatch());
        assert_eq!(bind_calls, 1);
        assert!(matches!(
            state.error,
            Some(rhapsody::Error::InvalidUsage { .. })
        ));
    }

    #[test]
    fn zero_byte_abi_and_explicit_initial_payload_bind_immediately() {
        for payload in [Vec::new(), vec![3; 16]] {
            let mut payloads = Vec::new();
            let mut bind = |bytes: &[u8]| {
                payloads.push(bytes.to_vec());
                Ok(())
            };
            let mut state =
                ComputeParameterState::new(payload.len() as u32, &payload, &mut bind).unwrap();
            assert!(state.can_dispatch());
            assert_eq!(payloads, vec![payload]);
        }
        let error = ComputeParameterState::new(20, &[1; 4], &mut |_| {
            Err(rhapsody::Error::invalid_usage("wrong initial payload size"))
        });
        assert!(matches!(error, Err(rhapsody::Error::InvalidUsage { .. })));
    }
}

#[cfg(test)]
mod original_individual_mip_view_tests {
    use super::*;
    #[test]
    fn selected_remaining_mips_are_individual_and_unused_abi_tail_reuses_the_last_actual_view() {
        let desc = rhapsody::resource::TextureDesc::new_2d_array(
            256,
            256,
            2,
            vk::Format::B10G11R11_UFLOAT_PACK32,
        )
        .mip_levels(9)
        .usage(vk::ImageUsageFlags::STORAGE);
        let source = country_core::rhapsody_types::TextureViewDesc::new()
            .base_mip_level(1)
            .base_layer(1)
            .view_type(5)
            .bind_individual_mips(true);
        let selected = project_view_desc(&desc, &source).unwrap();
        assert_eq!(
            (
                selected.base_mip_level,
                selected.level_count,
                selected.base_layer,
                selected.layer_count
            ),
            (1, 8, 1, 1)
        );
        let views = individual_mip_views(selected, 12).unwrap();
        assert_eq!(views.len(), 12);
        assert_eq!(
            views.iter().map(|v| v.base_mip_level).collect::<Vec<_>>(),
            [1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8]
        );
        assert!(
            views
                .iter()
                .all(|v| v.level_count == 1 && v.layer_count == 1 && v.base_layer == 1)
        );
        for v in views {
            v.validate(&desc).unwrap();
        }
        let logical = rhapsody::rg::ResourceSpec::Image(desc).descriptor();
        assert_eq!(
            compute_view_range(logical, &source).unwrap(),
            rhapsody::rg::AccessRange::Texture {
                aspect_mask: 1,
                base_mip_level: 1,
                level_count: 8,
                base_array_layer: 1,
                layer_count: 1
            }
        );
        assert!(individual_mip_views(selected, 7).is_err());
        assert!(project_view_desc(&desc, &source.base_mip_level(9)).is_err());
        assert!(project_view_desc(&desc, &source.level_count(Some(9))).is_err());
    }
}

#[cfg(test)]
mod original_surface_texture_flags_tests {
    use super::*;
    #[test]
    fn ice_radiance_flags_survive_the_actual_canonical_descriptor_projection() {
        let original = TextureDesc::new_2d([64, 32], 0x62).sampled_with_different_format();
        assert_eq!(original.flags, 0x108);
        assert!(original.sampled);
        let native = TextureHost::convert(&original).unwrap();
        assert_eq!(native.flags.as_raw(), 0x108);
        assert_eq!(native.format, vk::Format::R32_UINT);
        assert_eq!(native.width, 64);
        assert_eq!(native.height, 32);
        assert!(native.usage.contains(vk::ImageUsageFlags::SAMPLED));
        assert_eq!(
            TextureHost::convert(&TextureDesc::new_2d([64, 32], 0x62))
                .unwrap()
                .flags
                .as_raw(),
            0
        );
    }
}

#[cfg(test)]
mod planned_image_layout_abi_tests {
    use super::*;
    #[test]
    fn sampled_general_is_explicit_and_storage_never_accepts_sampled_layout() {
        for layout in [
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            vk::ImageLayout::GENERAL,
        ] {
            validate_image_binding_layout(BindingKind::SampledImage, layout).unwrap();
        }
        validate_image_binding_layout(BindingKind::StorageImage, vk::ImageLayout::GENERAL).unwrap();
        assert!(
            validate_image_binding_layout(
                BindingKind::StorageImage,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
            )
            .is_err()
        );
        assert!(
            validate_image_binding_layout(BindingKind::SampledImage, vk::ImageLayout::UNDEFINED)
                .is_err()
        );
        assert!(
            validate_image_binding_layout(
                BindingKind::SampledImage,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL
            )
            .is_err()
        );
        assert!(
            validate_image_binding_layout(BindingKind::Sampler, vk::ImageLayout::GENERAL).is_err()
        );
    }
}

#[cfg(test)]
mod sampler_catalog_tests {
    use super::*;
    use rhapsody::bindless::GroupPreparationError;

    fn device() -> Option<Arc<Device>> {
        match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => Some(device),
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                    "strict sampler catalog GPU initialization failed: {error}"
                );
                eprintln!("SKIP sampler catalog GPU initialization: {error}");
                None
            }
        }
    }
    fn heap(device: &Arc<Device>, samplers: u32) -> Arc<BindlessHeap> {
        BindlessHeap::new(
            device.clone(),
            BindlessConfig {
                sampled_images: 1,
                storage_images: 1,
                storage_buffers: 1,
                samplers,
            },
        )
        .unwrap()
    }
    fn binding(device: &Arc<Device>, name: &str, builtin: &str) -> SamplerBinding {
        SamplerBinding {
            location: BindingLocation { set: 0, binding: 0 },
            element: 0,
            name: Arc::from(name),
            sampler: device.get_built_in_sampler_by_name(builtin).unwrap(),
        }
    }
    fn actual(
        pages: &SamplerPages,
        heap: &Arc<BindlessHeap>,
        bindings: &[SamplerBinding],
    ) -> SamplerPreparation {
        let mut pending = pages.prepare(heap, bindings).unwrap();
        pending.materialize().unwrap();
        pending
    }
    // 测试独立串行 descriptor 路径；生产由 cursor 的真实同页 guard 和 require_slots 准入。
    fn extras(pending: &mut SamplerPreparation) -> rhapsody::Result<()> {
        let heap = pending.page.heap.clone();
        pending.try_catalog_extras(|required| {
            if required[3] <= heap.stats().tables[3].free {
                Ok(())
            } else {
                Err(GroupPreparationError::InsufficientSlots { required })
            }
        })
    }
    fn commit(pages: &SamplerPages, heap: &Arc<BindlessHeap>, bindings: &[SamplerBinding]) {
        let mut pending = actual(pages, heap, bindings);
        extras(&mut pending).unwrap();
        pending.commit();
    }

    #[test]
    fn actual_catalog_bootstraps_each_page_once_and_stays_app_local() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        pages.register(std::slice::from_ref(&b));
        let first = heap(&device, 4);
        let second = first.new_page().unwrap();
        commit(&pages, &first, std::slice::from_ref(&a));
        commit(&pages, &second, std::slice::from_ref(&b));
        assert_eq!(first.stats().tables[3].live, 2);
        assert_eq!(second.stats().tables[3].live, 2);
        for requested in [&a, &b, &a, &b] {
            let pending = pages
                .prepare(&first, std::slice::from_ref(requested))
                .unwrap();
            assert_eq!(pending.missing(), 0, "换 shader 不再给同页补静态目录");
            assert!(pending.catalog.is_none(), "同目录命中不重建 stamp");
            drop(pending);
            commit(&pages, &second, std::slice::from_ref(requested));
        }
        assert_eq!(first.stats().tables[3].live, 2);
        assert_eq!(second.stats().tables[3].live, 2);
        let other_app = SamplerPages::default();
        let c = binding(&device, "other app", "sampler_mipmap");
        other_app.register(std::slice::from_ref(&c));
        let other_heap = heap(&device, 4);
        commit(&other_app, &other_heap, std::slice::from_ref(&c));
        assert_eq!(
            other_heap.stats().tables[3].live,
            1,
            "同设备另 App 不复制此 App 目录"
        );
    }

    #[test]
    fn catalog_larger_than_page_keeps_original_one_sampler_groups_valid() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        pages.register(std::slice::from_ref(&b));
        let first = heap(&device, 1);
        let second = first.new_page().unwrap();
        for (heap, requested) in [(&first, &a), (&second, &b)] {
            let mut pending = actual(&pages, heap, std::slice::from_ref(requested));
            assert_eq!(pending.values.len(), 1, "实际组不先准备全目录");
            extras(&mut pending).unwrap();
            assert!(!pending.publish_catalog, "目录过大只走真实组");
            pending.commit();
            assert_eq!(heap.stats().tables[3].live, 1);
        }
    }

    #[test]
    fn named_versions_keep_owners_and_invalidation_cannot_publish_a_stale_stamp() {
        let Some(device) = device() else { return };
        let old = binding(&device, "custom", "sampler_nnc");
        let new = binding(&device, "custom", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&old));
        let heap = heap(&device, 4);
        commit(&pages, &heap, std::slice::from_ref(&old));
        let page = pages.state.lock().pages[0].clone();
        let original = page.leases.lock().leases[0].lease.clone();
        pages.register(std::slice::from_ref(&new));
        let mut pending = actual(&pages, &heap, std::slice::from_ref(&new));
        assert_eq!(pending.values.len(), 1, "实际请求只准备新 owner");
        extras(&mut pending).unwrap();
        pages.invalidate("custom");
        assert!(page.leases.lock().leases.is_empty());
        assert_eq!(
            heap.stats().tables[3].live,
            2,
            "在途准备/旧命令 owner 仍保活原租约"
        );
        pending.commit();
        {
            let cached = page.leases.lock();
            assert_eq!(cached.leases.len(), 1, "失效后旧缓存未暗中重发");
            assert!(Arc::ptr_eq(&cached.leases[0].sampler, &new.sampler));
            assert!(cached.catalog.is_none(), "旧epoch不能盖掉并发失效");
        }
        drop(original);
        commit(&pages, &heap, std::slice::from_ref(&old));
        let cached = page.leases.lock();
        assert_eq!(cached.leases.len(), 2);
        assert!(cached.catalog.is_some());
        for wanted in [&old, &new] {
            assert!(cached.leases.iter().any(|value| same_sampler(
                &value.name,
                &value.sampler,
                &wanted.name,
                &wanted.sampler
            )));
        }
    }

    #[test]
    fn optional_partial_host_budget_failure_and_cancel_keep_actual_group_and_old_cache() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let c = binding(&device, "c", "sampler_mipmap");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        let heap = heap(&device, 4);
        commit(&pages, &heap, std::slice::from_ref(&a));
        let page = pages.state.lock().pages[0].clone();
        let old_stamp = page.leases.lock().catalog.clone().unwrap();
        let budget = device.allocator_stats();
        let baseline = budget.cpu_memory_usage().total();
        let trial = heap.sampler(b.sampler.clone()).unwrap();
        let one_lease = budget.cpu_memory_usage().total() - baseline;
        assert!(one_lease > 0);
        drop(trial);
        assert_eq!(budget.cpu_memory_usage().total(), baseline);
        pages.register(std::slice::from_ref(&b));
        pages.register(std::slice::from_ref(&c));
        let mut pending = actual(&pages, &heap, std::slice::from_ref(&a));
        let old_limit = budget.host_budget_limit();
        budget
            .set_host_budget(baseline.checked_add(one_lease).unwrap())
            .unwrap();
        let result = extras(&mut pending);
        budget.set_host_budget(old_limit).unwrap();
        result.unwrap();
        assert!(!pending.publish_catalog, "部分额外项 OOM 不发布 stamp");
        assert_eq!(pending.values.len(), 1, "实际组不重放也不混入半目录");
        assert_eq!(heap.stats().tables[3].live, 1, "取消所有未发布的额外租约");
        pending.commit();
        {
            let cached = page.leases.lock();
            assert_eq!(cached.leases.len(), 1);
            assert!(Arc::ptr_eq(cached.catalog.as_ref().unwrap(), &old_stamp));
            assert!(Arc::ptr_eq(&cached.leases[0].sampler, &a.sampler));
        }
        let mut cancelled = actual(&pages, &heap, std::slice::from_ref(&a));
        extras(&mut cancelled).unwrap();
        assert_eq!(heap.stats().tables[3].live, 3);
        drop(cancelled);
        assert_eq!(
            heap.stats().tables[3].live,
            1,
            "完整 extras 未 commit 也要归还"
        );
        assert!(Arc::ptr_eq(
            page.leases.lock().catalog.as_ref().unwrap(),
            &old_stamp
        ));
        commit(&pages, &heap, std::slice::from_ref(&b));
        assert_eq!(heap.stats().tables[3].live, 3);
    }

    #[test]
    fn optional_admission_shortage_falls_back_but_real_errors_propagate() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        pages.register(std::slice::from_ref(&b));
        let heap = heap(&device, 4);
        let mut pending = actual(&pages, &heap, std::slice::from_ref(&a));
        pending
            .try_catalog_extras(|required| {
                Err(GroupPreparationError::InsufficientSlots { required })
            })
            .unwrap();
        assert!(!pending.publish_catalog);
        pending.commit();
        for error in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::Vulkan(vk::Result::ERROR_INITIALIZATION_FAILED),
            rhapsody::Error::unsupported("test admission error"),
            rhapsody::Error::invalid_usage("test admission error"),
        ] {
            let expected = error.kind();
            let mut pending = actual(&pages, &heap, std::slice::from_ref(&a));
            let actual = pending
                .try_catalog_extras(|_| Err(GroupPreparationError::Error(error)))
                .unwrap_err();
            assert_eq!(actual.kind(), expected);
            assert!(!pending.publish_catalog);
            drop(pending);
            assert_eq!(heap.stats().tables[3].live, 1);
        }
        let mut pending = pages.prepare(&heap, std::slice::from_ref(&b)).unwrap();
        let budget = device.allocator_stats();
        let old_limit = budget.host_budget_limit();
        budget
            .set_host_budget(budget.cpu_memory_usage().total())
            .unwrap();
        let actual_error = pending.materialize().unwrap_err();
        budget.set_host_budget(old_limit).unwrap();
        assert_eq!(
            actual_error.kind(),
            rhapsody::ErrorKind::OutOfMemory,
            "实际组错误仍然失败"
        );
    }

    #[test]
    fn registration_metadata_failure_disables_optional_catalog_without_failing_actual_groups() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        let heap = heap(&device, 4);
        commit(&pages, &heap, std::slice::from_ref(&a));
        pages.state.lock().fail_next_registration = true;
        pages.register(std::slice::from_ref(&b));
        {
            let state = pages.state.lock();
            assert!(state.catalog_disabled);
            assert!(state.catalog.is_none());
            assert_eq!(
                state.pages[0].leases.lock().leases.len(),
                1,
                "不清原实际缓存"
            );
        }
        pages.register(std::slice::from_ref(&a));
        assert!(
            pages.state.lock().catalog.is_none(),
            "遗漏登记后不能假标完整"
        );
        let pending = actual(&pages, &heap, std::slice::from_ref(&b));
        assert!(pending.catalog.is_none());
        pending.commit();
        assert_eq!(heap.stats().tables[3].live, 2);
    }

    #[test]
    fn invalidate_clears_stamp_published_between_epoch_change_and_name_removal() {
        let Some(device) = device() else { return };
        let a = binding(&device, "a", "sampler_nnc");
        let b = binding(&device, "b", "sampler_default");
        let pages = SamplerPages::default();
        pages.register(std::slice::from_ref(&a));
        pages.register(std::slice::from_ref(&b));
        let heap = heap(&device, 4);
        commit(&pages, &heap, std::slice::from_ref(&a));
        let page = pages.state.lock().pages[0].clone();
        page.begin_invalidation();
        let mut concurrent = actual(&pages, &heap, std::slice::from_ref(&a));
        let captured_epoch = concurrent.epoch;
        extras(&mut concurrent).unwrap();
        concurrent.commit();
        assert!(
            page.leases.lock().catalog.is_some(),
            "新epoch准备可在删除前发布 stamp"
        );
        page.finish_invalidation("a");
        {
            let cached = page.leases.lock();
            assert!(cached.leases.iter().all(|value| value.name.as_ref() != "a"));
            assert!(
                cached.catalog.is_none(),
                "最终空检查必须撤销删除期间发布的 stamp"
            );
            assert!(cached.epoch > captured_epoch);
        }
        commit(&pages, &heap, std::slice::from_ref(&b));
        assert_eq!(
            page.leases.lock().leases.len(),
            2,
            "下次必须真实补齐被删除项"
        );
        assert!(page.leases.lock().catalog.is_some());
    }

    #[test]
    fn only_explicit_optional_sampler_budget_errors_can_fall_back() {
        assert!(optional_sampler_budget_error(
            &rhapsody::Error::OutOfHostMemory
        ));
        assert!(optional_sampler_budget_error(
            &rhapsody::Error::OutOfDeviceMemory
        ));
        assert!(optional_sampler_budget_error(&rhapsody::Error::exhausted(
            "sampler slots"
        )));
        for error in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY),
            rhapsody::Error::Vulkan(vk::Result::ERROR_INITIALIZATION_FAILED),
            rhapsody::Error::unsupported("not optional"),
            rhapsody::Error::invalid_usage("not optional"),
        ] {
            assert!(!optional_sampler_budget_error(&error));
        }
    }
}
