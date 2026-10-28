//! 渲染侧 GPU 回读发布器（PARALLEL_REFACTOR.md 节点 4：回读改为消息）。
//!
//! 游戏系统不再读 GPU 每帧改写的持久映射，而是读游戏 World 中的
//! [`country_core::render::gpu_readback::GpuReadbacks`]。本模块在渲染常驻线程上：
//!
//! 1. 每个成功提交的帧之后，对游戏侧真正消费的回读 SSBO 发出一次 GPU→暂存区
//!    复制（`TransferManager::read_buffer`，同队列、依赖由 rhapsody 的访问跟踪
//!    建立，不等待）。
//! 2. 之后每次帧收尾时非阻塞轮询这些复制（`ReadbackTicket::try_read`），完成的
//!    字节移入池化的 owned 容器，经 `FrameFeedback` 移交游戏 World。
//!
//! 每份数据带来源 World 和渲染已提交帧序号：旧 App 的结果不会写进新 World，
//! 较旧帧的结果不会覆盖较新的。游戏线程与渲染线程之间不共享映射、不加锁、
//! 不等 GPU。暂存槽满时跳过本帧该类型（游戏继续使用上一份），并计数。
use std::{any::TypeId, collections::VecDeque, sync::Arc};

use country_core::{
    asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType},
    render::{gpu_readback::GpuReadbacks, ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer}},
    resources::{
        compute_mouse_depth_value::MouseDepthValueSsbo,
        terrain_height::{TerrainHeightSsbo, VisualTerrainHeightSsbo},
        terrain_rock_mask::TerrainRockMaskSsbo,
    },
    systems::render_loop::compute_passes::{
        dof_autofocus::DofFocusReadbackSsbo,
        mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo,
        read_bush_positions_for_cpu::BushCpuSsbo,
        read_terrain_data::ScheduledTerrainReadback,
    },
};
use parking_lot::Mutex;
use rhapsody::transfer::{ReadbackTicket, TransferConfig, TransferManager};

use crate::render_host::BufferHost;

/// 暂存槽：最大回读（两个地形高度各 512 KiB）放得进一个槽；按帧在途数留余量。
const SLOT_BYTES: u64 = 512 << 10;
const SLOTS: u32 = 48;
/// 超过这么多未完成复制时丢弃最旧的（只会在 GPU 长时间停滞时出现）。
const MAX_PENDING: usize = 40;
/// 游戏侧返还、等待复用的容器上限。
const MAX_POOLED: usize = 32;

/// Actual main-graph submission, independent of export/present bookkeeping.
#[derive(bevy_ecs::resource::Resource)]
pub(crate) struct SubmittedFrameCompletion(pub rhapsody::sync::SyncSet);

type HandleFn = fn(&AssetSsboLibrary) -> Option<Buffer>;

fn handle<T>(library: &AssetSsboLibrary) -> Option<Buffer>
where T: SsboType<Storage = MappedStorageBuffer<'static, T>> {
    library.get::<T>().map(|storage| storage.buffer)
}

/// 游戏侧实际消费的回读类型。`on_request` 为真的只在游戏请求过（或从未发布）时复制。
struct Source { ty: TypeId, name: &'static str, handle: HandleFn, on_request: bool }

fn sources() -> [Source; 7] {
    fn s<T>(on_request: bool) -> Source
    where T: SsboType<Storage = MappedStorageBuffer<'static, T>> {
        Source { ty: TypeId::of::<T>(), name: core::any::type_name::<T>(), handle: handle::<T>, on_request }
    }
    [
        s::<TerrainRockMaskSsbo>(false),
        s::<MouseTerrainRaycastSsbo>(false),
        s::<MouseDepthValueSsbo>(false),
        s::<DofFocusReadbackSsbo>(false),
        s::<BushCpuSsbo>(false),
        s::<TerrainHeightSsbo>(true),
        s::<VisualTerrainHeightSsbo>(true),
    ]
}

struct Pending {
    ty: TypeId,
    ticket: ReadbackTicket,
    source: bevy_ecs::world::WorldId,
    frame: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadbackPublisherStats {
    pub issued: u64,
    pub published: u64,
    /// 暂存槽不足而跳过的复制。
    pub skipped: u64,
    /// GPU 停滞导致丢弃的未完成复制。
    pub dropped: u64,
    pub errors: u64,
}

pub struct ReadbackPublisher {
    device: Arc<rhapsody::device::Device>,
    transfer: Option<TransferManager>,
    pending: VecDeque<Pending>,
    pool: Arc<Mutex<Vec<Vec<u128>>>>,
    published_once: Vec<TypeId>,
    /// 请求过但因暂存槽不足没能发出的按需类型；下个提交帧重试，避免消费者永久等待。
    owed: Vec<TypeId>,
    frame: u64,
    stats: ReadbackPublisherStats,
    completions: VecDeque<(bevy_ecs::world::WorldId, u64, rhapsody::sync::SyncSet)>,
    source: Option<bevy_ecs::world::WorldId>,
    submitted_points: rhapsody::sync::SyncSet,
}

impl ReadbackPublisher {
    pub fn new(device: Arc<rhapsody::device::Device>) -> Self {
        Self {
            device, transfer: None, pending: VecDeque::new(),
            pool: Default::default(), published_once: Vec::new(), owed: Vec::new(), frame: 0,
            stats: Default::default(),
            completions: VecDeque::new(), source: None,
            submitted_points: rhapsody::sync::SyncSet::EMPTY,
        }
    }

    pub fn stats(&self) -> ReadbackPublisherStats { self.stats }

    fn transfer(&mut self) -> rhapsody::Result<&mut TransferManager> {
        if self.transfer.is_none() {
            self.transfer = Some(TransferManager::new(self.device.clone(), TransferConfig {
                slots: SLOTS, chunk_bytes: SLOT_BYTES, max_bytes: SLOT_BYTES * u64::from(SLOTS) * 2,
            })?);
        }
        Ok(self.transfer.as_mut().expect("transfer manager was created"))
    }

    /// 帧收尾（渲染线程）：收割已完成的复制；若本帧已提交，再为本帧发出复制。
    /// 结果以一个闭包推入 `feedback`，必须在提交写回之后调用，使游戏在同一次
    /// 交接里先看到 `copy_issued` 再看到对应的已提交帧号。
    ///
    /// Also carries the staged asset acknowledgments (asset_ack): revisions
    /// applied this hand-over, confirmed as submitted with the frame, and as
    /// GPU-complete only after every queue in the main graph's real receipt
    /// completes. Apps without readback buffers use this same proof.
    pub fn finish_frame(
        &mut self,
        render: &mut bevy_ecs::world::World,
        buffers: &BufferHost,
        submitted: Option<rhapsody::sync::SyncSet>,
        feedback: &mut crate::render_thread::FrameFeedback,
    ) {
        let mut ready = Vec::new();
        self.collect(&mut ready);
        let source = Some(render.get_resource::<crate::render_thread::ExtractedAt>()
            .map_or(render.id(), |at| at.source));
        if source != self.source {
            self.published_once.clear();
            self.owed.clear();
            self.source = source;
            // Old Apps receive no feedback through this World. On reactivation
            // their next checkpoint covers all preceding queue submissions.
            self.completions.clear();
        }
        let applied = render.get_resource::<country_core::render::asset_ack::AppliedAssetRevisions>()
            .map(|applied| applied.0.clone()).unwrap_or_default();
        let operations = render.get_resource_mut::<country_core::render::asset_ack::AppliedAssetOperations>()
            .map(|mut operations| operations.feedback(submitted.is_some())).unwrap_or_default();
        let mut completed = None;
        match self.device.poll() {
            Ok(values) => {
                self.completions.retain(|(world, frame, points)| {
                    if Some(*world) != source || !points.is_reached(&values) { return true; }
                    if Some(*world) == source { completed = Some(completed.unwrap_or(0u64).max(*frame)); }
                    false
                });
            }
            Err(error) => { feedback.error.get_or_insert_with(|| error.into()); }
        }
        let mut note = None;
        if let Some(points) = submitted && let Some(source) = source {
            self.submitted_points.merge(&points);
            let points = self.submitted_points;
            self.frame += 1;
            note = Some((source, self.frame));
            if self.completions.len() >= 64
                && let Some((_, frame, existing)) = self.completions.iter_mut().rev().find(|entry| entry.0 == source) {
                *frame = self.frame;
                existing.merge(&points);
            } else {
                self.completions.push_back((source, self.frame, points));
            }
            self.issue(render, buffers, source);
        }
        if ready.is_empty() && note.is_none() && applied.is_empty() && operations.is_empty() && completed.is_none() {
            return;
        }
        let pool = self.pool.clone();
        feedback.defer(move |game: &mut bevy_ecs::world::World| {
            let id = game.id();
            let applies = |world: bevy_ecs::world::WorldId| world == id;
            if source.is_some_and(applies) {
                let mut acks = game.get_resource_or_init::<country_core::render::asset_ack::RenderAssetAcks>();
                acks.acknowledge(&applied, note.map(|(_, frame)| frame));
                acks.acknowledge_entries(operations, note.map(|(_, frame)| frame));
                if let Some(frame) = completed { acks.gpu_completed(frame); }
            }
            if !ready.iter().any(|(_, _, _, world, _)| applies(*world)) && !note.is_some_and(|(world, _)| applies(world)) {
                return;
            }
            let mut readbacks = game.get_resource_or_init::<GpuReadbacks>();
            let mut returned = Vec::new();
            for (ty, words, bytes, world, frame) in ready {
                if !applies(world) { returned.push(words); continue; }
                if let Some(old) = readbacks.publish_words(ty, words, bytes, frame) { returned.push(old); }
            }
            if let Some((world, frame)) = note && applies(world) {
                readbacks.note_submitted(frame);
            }
            let mut pool = pool.lock();
            for words in returned {
                if pool.len() < MAX_POOLED { pool.push(words); }
            }
        });
    }

    fn collect(&mut self, ready: &mut Vec<(TypeId, Vec<u128>, usize, bevy_ecs::world::WorldId, u64)>) {
        while self.pending.len() > MAX_PENDING {
            self.pending.pop_front();
            self.stats.dropped += 1;
        }
        let mut index = 0;
        while index < self.pending.len() {
            let pending = &self.pending[index];
            let bytes = pending.ticket.len();
            let mut words = self.pool.lock().pop().unwrap_or_default();
            words.clear();
            words.resize(bytes.div_ceil(16), 0);
            match pending.ticket.try_read(&mut bytemuck::cast_slice_mut::<u128, u8>(&mut words)[..]) {
                Ok(None) => {
                    let mut pool = self.pool.lock();
                    if pool.len() < MAX_POOLED { pool.push(words); }
                    index += 1;
                }
                Ok(Some(len)) => {
                    let pending = self.pending.remove(index).expect("pending readback index");
                    ready.push((pending.ty, words, len, pending.source, pending.frame));
                    self.stats.published += 1;
                }
                Err(error) => {
                    log::warn!(target: "tiny_glade::gpu_readback", "readback copy failed: {error}");
                    self.pending.remove(index);
                    self.stats.errors += 1;
                }
            }
        }
    }

    fn issue(&mut self, render: &bevy_ecs::world::World, buffers: &BufferHost, source: bevy_ecs::world::WorldId) {
        let Some(library) = render.get_resource::<AssetSsboLibrary>() else { return; };
        // 渲染 World 的 copy_issued 可能沿用上一帧的 true：这只多复制一次，
        // 不影响正确性（消费者按帧号判断新旧）。
        let requested = render.get_resource::<ScheduledTerrainReadback>().is_some_and(|readback| readback.copy_issued);
        for source_desc in sources() {
            let owed = self.owed.contains(&source_desc.ty);
            if source_desc.on_request && !requested && !owed && self.published_once.contains(&source_desc.ty) {
                continue;
            }
            let Some(handle) = (source_desc.handle)(library) else { continue; };
            let Some(managed) = buffers.get_by_id(handle.id) else { continue; };
            let bytes = handle.byte_size as usize;
            if bytes == 0 || bytes as u64 > SLOT_BYTES || bytes as u64 > managed.size {
                continue;
            }
            let frame = self.frame;
            let result = self.transfer().and_then(|transfer| transfer.read_buffer(&managed.shared(), 0, bytes));
            match result {
                Ok(ticket) => {
                    self.pending.push_back(Pending { ty: source_desc.ty, ticket, source, frame });
                    if !self.published_once.contains(&source_desc.ty) { self.published_once.push(source_desc.ty); }
                    self.owed.retain(|ty| *ty != source_desc.ty);
                    self.stats.issued += 1;
                }
                Err(rhapsody::Error::Timeout) => {
                    self.stats.skipped += 1;
                    if source_desc.on_request && !owed { self.owed.push(source_desc.ty); }
                }
                Err(error) => {
                    self.stats.errors += 1;
                    log::warn!(target: "tiny_glade::gpu_readback", "readback of {} not issued: {error}", source_desc.name);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::{component::Tick, world::World};

    fn device() -> Option<Arc<rhapsody::device::Device>> {
        match rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
            Ok(device) => Some(device),
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                None
            }
        }
    }

    fn apply(feedback: &mut crate::render_thread::FrameFeedback, game: &mut World) {
        for step in feedback.apply.drain(..) { step(game); }
    }

    #[test]
    fn unchanged_revision_retries_and_completes_without_any_readback_sources() {
        let Some(device) = device() else { return; };
        let host = BufferHost::new(device.clone());
        let buffer = device.create_buffer(&rhapsody::resource::BufferDesc::new(
            4, ash::vk::BufferUsageFlags::TRANSFER_SRC, rhapsody::resource::MemoryLocation::CpuToGpu),
            "asset acknowledgment proof", Some(&[0u8; 4][..])).unwrap();
        let mut transfer = TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
        let proof = transfer.read_buffer(&buffer.shared(), 0, 4).unwrap();
        let points = rhapsody::sync::SyncSet::from_point(proof.completion());
        let (mut game, mut render) = (World::new(), World::new());
        render.insert_resource(crate::render_thread::ExtractedAt {
            source: game.id(), game: Tick::new(0), render: Tick::new(0) });
        struct Library;
        use country_core::render::asset_ack::{AppliedAssetRevisions, RenderAssetAcks,
            AppliedAssetOperations, AssetEntryKey, AssetOperation, AssetOperationKind};
        render.insert_resource(AppliedAssetRevisions(vec![(TypeId::of::<Library>(), 9)]));
        let key = AssetEntryKey::Id(17);
        render.init_resource::<AppliedAssetOperations>();
        render.resource_mut::<AppliedAssetOperations>().record(AssetOperation {
            library: TypeId::of::<Library>(), entry: key.clone(), revision: 9, kind: AssetOperationKind::Upsert,
        });
        let mut publisher = ReadbackPublisher::new(device);
        let mut feedback = crate::render_thread::FrameFeedback::default();
        publisher.finish_frame(&mut render, &host, None, &mut feedback);
        apply(&mut feedback, &mut game);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().applied, 9);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().submitted, 0);
        assert_eq!(game.resource::<RenderAssetAcks>().entry::<Library>(&key).unwrap().stages.submitted, 0);
        // No extraction and no library mutation between failure and retry.
        publisher.finish_frame(&mut render, &host, Some(points), &mut feedback);
        apply(&mut feedback, &mut game);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().submitted, 9);
        assert_eq!(game.resource::<RenderAssetAcks>().entry::<Library>(&key).unwrap().stages.submitted, 9);
        let start = std::time::Instant::now();
        while !game.resource::<RenderAssetAcks>().gpu_complete::<Library>(9) {
            publisher.finish_frame(&mut render, &host, None, &mut feedback);
            apply(&mut feedback, &mut game);
            assert!(start.elapsed() < std::time::Duration::from_secs(10));
        }
        assert_eq!(publisher.stats().issued, 0);
        assert!(game.resource::<RenderAssetAcks>().entry_gpu_complete::<Library>(&key, 9));
        assert!(render.resource_mut::<AppliedAssetOperations>().feedback(false).is_empty());
        let mut other = World::new();
        publisher.finish_frame(&mut render, &host, Some(points), &mut feedback);
        apply(&mut feedback, &mut other);
        assert!(!other.contains_resource::<RenderAssetAcks>());
    }

    /// 真实设备：渲染侧复制映射回读、非阻塞轮询，并只向来源 World 发布；
    /// 游戏侧拿到的是完整副本，之后映射内容变化不影响已发布数据。
    #[test]
    fn submitted_frame_copies_are_published_to_their_source_world_only() {
        let Some(device) = device() else { return; };
        let host = BufferHost::new(device.clone());
        let mut storage = <TerrainRockMaskSsbo as SsboType>::new_storage_buffer(&host, None);
        storage.mapped[7].0 = 0xdead_beef;
        let native = host.managed_buffer(&storage.buffer).unwrap();
        let mut transfer = TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
        let proof = transfer.read_buffer(&native.shared(), 0, 4).unwrap();
        let submitted = rhapsody::sync::SyncSet::from_point(proof.completion());
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(storage);
        let mut game = World::new();
        let other = World::new();
        let mut render = World::new();
        render.insert_resource(library);
        render.insert_resource(crate::render_thread::ExtractedAt {
            source: game.id(), game: Tick::new(0), render: Tick::new(0) });
        let mut publisher = ReadbackPublisher::new(device);
        struct Library;
        use country_core::render::asset_ack::{AppliedAssetRevisions, AssetStageAck, RenderAssetAcks};
        render.insert_resource(AppliedAssetRevisions(vec![(TypeId::of::<Library>(), 7)]));
        let mut feedback = crate::render_thread::FrameFeedback::default();
        publisher.finish_frame(&mut render, &host, Some(submitted), &mut feedback);
        assert_eq!(publisher.stats().issued, 1);
        apply(&mut feedback, &mut game);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>(),
            AssetStageAck { applied: 7, submitted: 7, gpu_complete: 0 });
        // An unsubmitted frame acknowledges CPU application only.
        render.insert_resource(AppliedAssetRevisions(vec![(TypeId::of::<Library>(), 8)]));
        let mut feedback = crate::render_thread::FrameFeedback::default();
        publisher.finish_frame(&mut render, &host, None, &mut feedback);
        apply(&mut feedback, &mut game);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().submitted, 7);
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().applied, 8);
        assert_eq!(game.resource::<GpuReadbacks>().submitted_frame(), 1);
        assert!(game.resource::<GpuReadbacks>().get::<TerrainRockMaskSsbo>().is_none());
        // The mapping changing after the copy was issued is not observed.
        let started = std::time::Instant::now();
        loop {
            let mut feedback = crate::render_thread::FrameFeedback::default();
            publisher.finish_frame(&mut render, &host, None, &mut feedback);
            apply(&mut feedback, &mut game);
            if game.resource::<GpuReadbacks>().get::<TerrainRockMaskSsbo>().is_some() { break; }
            assert!(started.elapsed() < std::time::Duration::from_secs(10), "readback never completed");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let view = game.resource::<GpuReadbacks>().get::<TerrainRockMaskSsbo>().unwrap();
        assert_eq!((view.data.len(), view.data[7].0, view.frame), (0x2000, 0xdead_beef, 1));
        // The completed copy of frame 1 confirms that frame's GPU work.
        assert_eq!(game.resource::<RenderAssetAcks>().stage::<Library>().gpu_complete, 7);
        // A different App's World never receives this App's results.
        let mut other = other;
        let mut feedback = crate::render_thread::FrameFeedback::default();
        publisher.finish_frame(&mut render, &host, Some(submitted), &mut feedback);
        apply(&mut feedback, &mut other);
        assert!(!other.contains_resource::<GpuReadbacks>());
    }
}
