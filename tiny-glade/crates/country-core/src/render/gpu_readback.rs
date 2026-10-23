//! 游戏侧的 GPU 回读接收端（PARALLEL_REFACTOR.md 节点 4：回读改为消息）。
//!
//! 原版游戏系统直接读 GPU 每帧改写的持久映射（`MappedStorageBuffer::mapped`）。
//! 渲染常驻线程与游戏并行后，这会与在途帧的 GPU 写入竞争；若改为
//! "等 GPU 完成才读" 又会让游戏线程与渲染提交互相加锁、甚至长期读不到数据。
//!
//! 现在由渲染线程在每个已提交帧之后发出一次 GPU→暂存区复制，帧完成后把字节
//! 移入拥有所有权的容器，经 `FrameFeedback` 发布到本资源。游戏系统只读这里的
//! 最近一份完整数据：不触碰映射、不拿 GPU 锁、不等待。延迟与原版一样至少晚一帧。
//!
//! 每份数据带渲染侧单调的已提交帧序号 `frame`；[`GpuReadbacks::submitted_frame`]
//! 是游戏已收到确认的最新提交帧。需要"某次 GPU 复制之后的数据"的消费者
//! （例如地形高度）记录请求时的 `submitted_frame`，等到 `frame >= 它` 再使用。
use std::any::TypeId;
use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::render::ssbo::StorageElement;

/// 一份已发布的回读。按 16 字节块存放，保证对 `T`（对齐 ≤ 16）可直接切片视图。
struct Entry {
    words: Vec<u128>,
    bytes: usize,
    frame: u64,
    revision: u64,
}

/// 某个回读类型的最近完整副本。
pub struct ReadbackView<'a, T> {
    pub data: &'a [T],
    /// 产生这份数据的渲染已提交帧序号。
    pub frame: u64,
    /// 本类型的发布次数（每次新数据 +1），用于跳过未变化的重复处理。
    pub revision: u64,
}

#[derive(Resource, Default)]
pub struct GpuReadbacks {
    entries: HashMap<TypeId, Entry>,
    submitted_frame: u64,
}

impl GpuReadbacks {
    /// 最近一份完整回读；渲染侧尚未发布过时为 None（等价于还没有 GPU 结果）。
    pub fn get<T: StorageElement + 'static>(&self) -> Option<ReadbackView<'_, T>> {
        let entry = self.entries.get(&TypeId::of::<T>())?;
        let size = core::mem::size_of::<T>();
        assert!(core::mem::align_of::<T>() <= core::mem::align_of::<u128>(),
            "readback element alignment exceeds the u128 backing store");
        let len = entry.bytes.checked_div(size).unwrap_or(0);
        debug_assert!(len * size <= entry.words.len() * 16);
        // SAFETY: 存储按 16 字节对齐且至少 len*size 字节；字节来自同一类型 GPU 缓冲的
        // 完整副本，与原版把映射区重解释为 `&[T]` 的假设相同（T: Copy、按位有效）。
        let data = unsafe { core::slice::from_raw_parts(entry.words.as_ptr().cast::<T>(), len) };
        Some(ReadbackView { data, frame: entry.frame, revision: entry.revision })
    }

    /// 只取数据切片。
    pub fn latest<T: StorageElement + 'static>(&self) -> Option<&[T]> {
        self.get::<T>().map(|view| view.data)
    }

    /// 游戏已确认的最新渲染提交帧序号。
    pub fn submitted_frame(&self) -> u64 {
        self.submitted_frame
    }

    pub fn note_submitted(&mut self, frame: u64) {
        self.submitted_frame = self.submitted_frame.max(frame);
    }

    /// 发布渲染侧移交的字节（`bytes` 是有效字节数）。较旧帧的结果被拒收并原样返还，
    /// 被替换的旧容器也返还给调用方，供渲染侧复用分配。
    pub fn publish_words(&mut self, ty: TypeId, words: Vec<u128>, bytes: usize, frame: u64) -> Option<Vec<u128>> {
        assert!(bytes <= words.len() * 16, "readback byte count exceeds its storage");
        match self.entries.get_mut(&ty) {
            Some(entry) if entry.frame > frame => Some(words),
            Some(entry) => {
                entry.bytes = bytes;
                entry.frame = frame;
                entry.revision += 1;
                Some(core::mem::replace(&mut entry.words, words))
            }
            None => {
                self.entries.insert(ty, Entry { words, bytes, frame, revision: 1 });
                None
            }
        }
    }

    /// 测试夹具及无渲染器的宿主：直接发布一份类型化数据（不推进已提交帧号）。
    pub fn publish_slice<T: StorageElement + 'static>(&mut self, data: &[T], frame: u64) {
        let bytes = core::mem::size_of_val(data);
        let mut words = vec![0u128; bytes.div_ceil(16)];
        // SAFETY: 目标至少 bytes 字节；T: Copy，按字节复制。
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr().cast::<u8>(), words.as_mut_ptr().cast::<u8>(), bytes);
        }
        self.publish_words(TypeId::of::<T>(), words, bytes, frame);
    }
}

/// 读取 `T` 的最近副本；资源不存在或尚无数据时为 None。
pub fn latest<T: StorageElement + 'static>(readbacks: Option<&GpuReadbacks>) -> Option<&[T]> {
    readbacks?.latest::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Sample(u32, f32);
    impl StorageElement for Sample {
        const NAME: &'static str = "Sample";
    }

    #[test]
    fn newer_frames_replace_older_and_stale_results_are_returned() {
        let mut readbacks = GpuReadbacks::default();
        assert!(readbacks.get::<Sample>().is_none());
        readbacks.publish_slice(&[Sample(1, 0.5), Sample(2, 1.5)], 3);
        let view = readbacks.get::<Sample>().unwrap();
        assert_eq!(view.data, &[Sample(1, 0.5), Sample(2, 1.5)]);
        assert_eq!((view.frame, view.revision), (3, 1));
        // 旧帧被拒收，容器原样返还。
        let stale = readbacks.publish_words(TypeId::of::<Sample>(), vec![0; 1], 8, 2);
        assert_eq!(stale, Some(vec![0]));
        assert_eq!(readbacks.get::<Sample>().unwrap().frame, 3);
        // 新帧替换并返还旧容器。
        let old = readbacks.publish_words(TypeId::of::<Sample>(), vec![0; 1], 8, 4);
        assert_eq!(old.map(|words| words.len()), Some(1));
        let view = readbacks.get::<Sample>().unwrap();
        assert_eq!((view.data, view.frame, view.revision), (&[Sample(0, 0.0)][..], 4, 2));
        assert_eq!(readbacks.submitted_frame(), 0);
    }
}
