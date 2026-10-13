//! time_of_day.rs — 昼夜序列（WP21）。
//!
//! O：`TodSequence::snapshot_at_tod`（0xb067d0）：`fmodf(tod,1)` 定位段 → inverse_lerp
//! 两段 → `TodLightingSnapshot::lerp` + `SunSkyColors::lerp`。`TodLightingSnapshot`
//! 0x44B：`{ 8×f32 @+0x0, 2×u64 @+0x20, 4×f32 @+0x30, f32 @+0x40 }`。`TodSequenceEntry`
//! 0x50B（hour 区间 + snapshot 参数）。`TimeOfDay::hour`。

use bevy_ecs::resource::Resource;
use glam::Vec4;

use crate::typed_gap::Gap;

/// `TodLightingSnapshot`（O：0x44B 光照快照；字段语义 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TodLightingSnapshot {
    /// +0x0 8×f32（太阳方向/天空色）。
    pub a: Vec4,
    pub b: Vec4,
    /// +0x20/+0x28 u64。
    pub c: u64,
    pub d: u64,
    /// +0x30 4×f32。
    pub e: Vec4,
    /// +0x40 f32。
    pub f: f32,
}

impl TodLightingSnapshot {
    /// `lerp`（O 名，2 内联）。
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            a: self.a.lerp(other.a, t),
            b: self.b.lerp(other.b, t),
            c: self.c,
            d: self.d,
            e: self.e.lerp(other.e, t),
            f: self.f + (other.f - self.f) * t,
        }
    }
}

/// `TodSequenceEntry`（O：0x50B；hour 区间 + 快照参数）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TodSequenceEntry {
    /// 起始 hour（O：[0]）。
    pub start_hour: f32,
    /// 终止 hour（O：[0x14]）。
    pub end_hour: f32,
    /// 快照参数（O：+0xc..0x4c）。
    pub snapshot: TodLightingSnapshot,
}

/// `TodSequence`（O：昼夜关键帧序列）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TodSequence {
    pub entries: Vec<TodSequenceEntry>,
}

impl TodSequence {
    /// `snapshot_at_tod`（O 0xb067d0）。
    pub fn snapshot_at_tod(&self, tod: f32) -> TodLightingSnapshot {
        if self.entries.is_empty() {
            return TodLightingSnapshot::default();
        }
        if self.entries.len() == 1 {
            return self.entries[0].snapshot;
        }
        let t = tod.rem_euclid(1.0);
        let mut seg = self.entries.len() - 1;
        for (i, e) in self.entries.iter().enumerate() {
            if t >= e.start_hour && t <= e.end_hour {
                seg = i;
                break;
            }
        }
        let a = &self.entries[seg];
        let b = &self.entries[(seg + 1) % self.entries.len()];
        let span = (b.start_hour - a.start_hour).max(f32::EPSILON);
        let f = ((t - a.start_hour) / span).clamp(0.0, 1.0);
        a.snapshot.lerp(&b.snapshot, f)
    }
}

/// `TodEasing`（O：`impl$5::default` 0xb06790 用 fastrand 选段）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TodEasing(pub Gap);

/// `TimeOfDay`（O：当前小时）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeOfDay(pub f32);

impl TimeOfDay {
    /// `hour`（O 名，1 内联）。
    pub fn hour(&self) -> f32 {
        self.0
    }
}

/// `TempUiId`（O：临时 UI id，U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TempUiId(pub u64);
