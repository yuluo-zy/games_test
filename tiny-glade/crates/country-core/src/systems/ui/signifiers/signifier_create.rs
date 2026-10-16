//! `signifier_create.rs` — UiSignifierStream + signifier_create（B 0x140b2cf90/0x140b2cfa0，O）。
//!
//! 证据：`chapters/systems/ui/signifiers/signifier_create.rs.md`（wp15 b7）：
//! - `UiSignifierStream` = 0x170 B 无锁环形流（read cursor@+0x160、write counter@+0x168、
//!   0x28 B 槽）；`write` 拷贝 0x28 B `UiAppendSignifier` 记录 + 自旋推进游标（36 callers）；
//! - `signifier_create` 每帧 drain 环形流 → 逐条生成指示符实体。
//! （重建代码写成 `Vec<UiAppendSignifier>` 是错的——真实为无锁环形流。）
use bevy_ecs::prelude::*;
use glam::Vec3;

/// 0x28 B 指示符记录（O：**中央裁决 2026-10-16**——机器码 write 拷贝 5×qword
/// = 40B：tag@0、kind@+8、Vec3@+0x10、f32@+0x1c、qword@+0x20）。
///
/// 字段语义（O 面：water/glade-border 的 0x20B 记录族 = {1, 1, Vec3, f32}；
/// name 槽 = 附加载荷/名字句柄，语义 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiAppendSignifier {
    /// 指示符类型判别（O：water 橡皮记录 = 1）。
    pub tag: u64,
    /// 子类型（O：water/glade-border = 1）。
    pub kind: u64,
    /// 世界位置。
    pub position: Vec3,
    /// 缩放/首附加浮点（O：water = 0.5×半径、glade-border = perimeter）。
    pub scale: f32,
    /// 附加载荷 qword（I：名字/朝向打包，语义 U）。
    pub name: u64,
}

impl UiAppendSignifier {
    /// 便捷构造（O 面：tag/kind/position/scale；name 留 0）。
    pub fn new(tag: u64, kind: u64, position: glam::Vec3, scale: f32) -> Self {
        Self {
            tag,
            kind,
            position,
            scale,
            name: 0,
        }
    }
}

/// 无锁环形流（O：0x170 B，read cursor@+0x160、write counter@+0x168、0x28 B 槽）。
/// 本实现以 VecDeque 环形承载；无锁语义登记（宿主并发接线待取证）。
#[derive(Resource, Clone, Debug, Default)]
pub struct UiSignifierStream {
    records: std::collections::VecDeque<UiAppendSignifier>,
}

/// 槽容量（O：0x170B / 0x28B = 槽数，I）。
pub const STREAM_CAPACITY: usize = 8;

impl UiSignifierStream {
    /// `UiSignifierStream::write`（0x140b2cf90，O）：拷贝 0x28 B 记录 + 自旋推进游标。
    pub fn write(&mut self, record: UiAppendSignifier) {
        self.records.push_back(record);
        while self.records.len() > STREAM_CAPACITY {
            self.records.pop_front();
        }
    }

    /// 排空当前全部记录（0x140b2cfa0 signifier_create 消费）。
    pub fn drain(&mut self) -> impl Iterator<Item = UiAppendSignifier> + '_ {
        self.records.drain(..)
    }
}

/// `signifier_create` @0x140b2cfa0（O）：排空环形流 → 逐条生成指示符事件。
/// 实际 spawn 指示符实体依赖实体机制（宿主），本实现产出 [`SignifierSpawned`] 事件。
pub fn signifier_create(
    mut stream: ResMut<UiSignifierStream>,
    mut spawned: EventWriter<SignifierSpawned>,
) {
    for record in stream.drain() {
        spawned.write(SignifierSpawned(record));
    }
}

/// 指示符生成事件（I：宿主按记录 spawn 实体）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct SignifierSpawned(pub UiAppendSignifier);
