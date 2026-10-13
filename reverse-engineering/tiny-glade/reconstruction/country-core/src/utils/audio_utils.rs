//! utils/audio_utils.rs — 池化循环播放器（AudioLoopDesc / AudioLoop）。
//!
//! PDB 骨架：`AudioLoopDesc::new`（行 12）、`AudioLoop::{current, is_playing,
//! start_if_not_playing, stop}`、`AudioCore::instantiate_loop`（0x140a59220）、
//! `AudioCore::instantiate_loop_snappy`。
//!
//! 反汇编证据（O）：
//! - `AudioLoopDesc`：`{name: &'static str, count: u64 (=3), min_interval_ms: u32 (=0x32=50)}`
//!   （wall/stairs/building init 的 desc{name_ptr,len,u64=3,u32=0x32}；0x140a59220 读 desc+0x18）。
//! - `instantiate_loop`：`from_iter(0..count, |_| instantiate_audio(desc.name))` 建实例池；
//!   对首个实例 `EventInstance::get_parameter_by_name` 得 `has_parameter`。
//! - `AudioLoop`：`{result: Result<Vec<AudioInstance>,Error> @0/@8, count @0x10,
//!   last_start Instant @0x18/@0x20 (哨兵 1000000000), current @0x28, min_interval_ms @0x30,
//!   should_play @0x34, has_parameter @0x35}`（I）。
//! - `start_if_not_playing`（0x140a596b0）：轮转实例并 `start`；`stop`（0x140a59430）：
//!   50ms 冷却后轮转并停。

#![allow(dead_code, unused_imports)]

use std::time::Instant;

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// 循环描述（O 布局：name + count(=3) + min_interval_ms(=50)）。
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioLoopDesc {
    /// FMOD 事件名（`event:/…`）。
    pub name: &'static str,
    /// 实例池大小（=3，O）。
    pub count: u64,
    /// 两次 stop 的最小间隔 ms（=0x32=50，O）。
    pub min_interval_ms: u32,
}

impl AudioLoopDesc {
    /// `AudioLoopDesc::new`（行 12，O）。各 `*_init_system` 在行 12 调用。
    pub fn new(name: &'static str, _param: Option<&'static str>, count: u64, max: u32) -> Self {
        Self {
            name,
            count,
            min_interval_ms: max,
        }
    }
}

/// 池化循环播放器（O 布局部分恢复；字段名 I）。
#[derive(Debug)]
pub struct AudioLoop {
    /// 实例池（Result<Vec<AudioInstance>, anyhow::Error> @0/@8，O）。
    pub instances: Result<Vec<AudioInstance>, anyhow::Error>,
    /// 池大小 @0x10（O）。
    pub count: usize,
    /// 上次 start 时间（O：`Instant @0x18/@0x20`，初值哨兵 1000000000 = 从未；此处以
    /// `Option` 表示，精确布局 U）。
    pub last_start: Option<Instant>,
    /// 当前轮转索引 @0x28（O）。
    pub current: usize,
    /// stop 冷却 ms @0x30（O）。
    pub min_interval_ms: u32,
    /// 应播标志 @0x34（O）。
    pub should_play: bool,
    /// 首个实例是否含 desc 参数 @0x35（O）。
    pub has_parameter: bool,
    /// 其余 @0x36…（U）。
    pub _gap: Gap,
}

impl AudioLoop {
    /// 取当前实例（`current` 0x140a593f0，O；越界 panic）。
    pub fn current(&self) -> Option<&AudioInstance> {
        let ok = self.instances.as_ref().ok()?;
        ok.get(self.current)
    }

    /// `start_if_not_playing`（0x140a596b0，O）。
    pub fn start_if_not_playing(&mut self) {
        self.last_start = Some(Instant::now());
        if self.should_play {
            return;
        }
        if self.has_parameter {
            // 先 set_parameter（desc 参数），再 EventInstance::start（libfmod 边界）。
        }
        if let Some(inst) = self.current() {
            inst.start_if_not_playing();
        }
        self.should_play = true;
    }

    /// `stop`（0x140a59430，O）：冷却期内只记时间；冷却后轮转并停。
    pub fn stop(&mut self) {
        let Some(last) = self.last_start else {
            self.last_start = Some(Instant::now());
            return;
        };
        let elapsed_ms = last.elapsed().as_millis() as u32;
        if self.min_interval_ms < elapsed_ms {
            // has_parameter 时先 set_parameter，再 EventInstance::stop（libfmod 边界）。
            if let Some(inst) = self.current() {
                inst.stop();
            }
            let count = self.count.max(1);
            self.current = (self.current + 1) % count;
            self.should_play = false;
            self.last_start = None;
        }
    }

    /// `is_playing`（O）。
    pub fn is_playing(&self) -> bool {
        self.current().map_or(false, AudioInstance::is_playing)
    }
}

impl AudioCore {
    /// `AudioCore::instantiate_loop`（0x140a59220，O）。
    pub fn instantiate_loop(&self, desc: &AudioLoopDesc) -> Result<AudioLoop, anyhow::Error> {
        let inner = match self.inner() {
            Some(inner) => inner,
            None => {
                // 无内核：返回空池（调用方容错，O 判别 0 路径）。
                return Ok(AudioLoop {
                    instances: Ok(Vec::new()),
                    count: desc.count as usize,
                    last_start: None,
                    current: 0,
                    min_interval_ms: desc.min_interval_ms,
                    should_play: false,
                    has_parameter: false,
                    _gap: Gap,
                });
            }
        };
        let mut instances = Vec::new();
        for _ in 0..desc.count {
            instances.push(inner.instantiate_audio(desc.name)?);
        }
        let has_parameter = instances.first().map_or(false, |i| i.has_parameter(""));
        Ok(AudioLoop {
            instances: Ok(instances),
            count: desc.count as usize,
            last_start: None,
            current: 0,
            min_interval_ms: desc.min_interval_ms,
            should_play: false,
            has_parameter,
            _gap: Gap,
        })
    }

    /// `AudioCore::instantiate_loop_snappy`（行 65，O 名；path_audio_init 用）。
    pub fn instantiate_loop_snappy(&self, desc: &AudioLoopDesc) -> Result<AudioLoop, anyhow::Error> {
        self.instantiate_loop(desc)
    }

    /// `AudioCore::instantiate_audio`（0x140a55f40，O）：无内核时返回 `Disabled` 变体。
    pub fn instantiate_audio(&self, name: &str) -> Result<AudioInstance, anyhow::Error> {
        match self.inner() {
            Some(inner) => inner.instantiate_audio(name),
            None => Ok(AudioInstance::Disabled {
                name: name.to_string(),
                state: 2,
            }),
        }
    }
}
