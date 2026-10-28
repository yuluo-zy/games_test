//! 指示符的逐帧请求、去重和实体创建。
//!
//! 请求追加到活动 WidgetParams 的工厂列表；删除系统读取同一活动所有者。
//! 创建去重只看本次执行前已有实体，保持原延迟命令语义。
//! 原生取证记录保留在历史报告中，本模块描述当前 ECS 实现的生命周期合同。
use std::collections::HashSet;

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::asset_libraries::shader_library::AssetShaderLibrary;
use crate::systems::ui::resources::ui_state::UiState;

use super::signifier_factory::SignifierFactory;
use super::signifier_mark_for_delete::SignifierMarkedForDeletion;

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

    /// 原始 5×qword 载荷入 typed 槽（保持字节位型；机器码 write 拷贝 40B = 5 qword）。
    /// `[q2] = position.x | position.y<<32`、`[q3] = position.z | scale<<32`。
    pub fn from_raw_qwords(q: [u64; 5]) -> Self {
        Self {
            tag: q[0],
            kind: q[1],
            position: Vec3::new(
                f32::from_bits(q[2] as u32),
                f32::from_bits((q[2] >> 32) as u32),
                f32::from_bits(q[3] as u32),
            ),
            scale: f32::from_bits((q[3] >> 32) as u32),
            name: q[4],
        }
    }
}

/// 原生 `AppendOnlyVec` 的桶上限（O：`indices()` 对桶号 > 0x2b 走
/// `panic_bounds_check`；0x2b = 43 个桶，8·2^b 元素/桶）。
pub const STREAM_MAX_BUCKETS: usize = 0x2c;

/// 指示符实体标识（O：`country_core::systems::ui::signifiers::SignifierId` 组件；
/// `signifier_create` 以其去重）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SignifierId(pub u64, pub crate::systems::ui::resources::ui_state::WidgetId);

/// 无锁 append-only 流的槽位（crate 内部表示）。
///
/// - [`Raw`](Self::Raw)：crate 旧的 5-qword 视图（`tag/kind/position/scale/name`），
///   由 `write`/`write_raw` 与没有闭包的 `SignifierFactory` 产生（既有 wall/terrain
///   调用点）。
/// - [`Factory`](Self::Factory)：原生 40 B 记录 `{name: String, inner: Arc<dyn Fn(..)
///   -> Vec<Entity> + Send + Sync>}`（`DebugLine::show` 与 glade-border/water 的
///   perimeter 写路径），消费时由 `signifier_create` 唤醒 vtable。
#[derive(Clone, Debug)]
pub enum UiSignifierEntry {
    /// 旧 crate 5-qword 记录。
    Raw(UiAppendSignifier),
    /// 原生闭包记录（40 B）。
    Factory(SignifierFactory),
}

/// 待消费的指示符请求；活动列表归 UiState 的 captured params 所有。
///
/// 独占可变访问由 Bevy 资源借用保证；inactive 时保留请求。
#[derive(Resource, Clone, Debug, Default)]
pub struct UiSignifierStream {
    slots: Vec<UiSignifierEntry>,
}

impl UiSignifierStream {
    /// 资源已经被独占借用，直接追加，不模拟原生并发队列的预留和自旋。
    fn push_entry(&mut self, entry: UiSignifierEntry) {
        self.slots.push(entry);
    }

    /// `write`/`push`（0x14146c880，O）：crate 旧 5-qword 记录入流。语义上永不丢记录。
    pub fn write(&mut self, record: UiAppendSignifier) {
        self.push_entry(UiSignifierEntry::Raw(record));
    }

    /// 原始 5×qword 写面（承接 40B 记录，如 `SignifierFactory` 的
    /// `{len,ptr,len,&inner,marker}`——glade-border perimeter 入流路径，O 机器码
    /// write 拷贝 40B）。与 `write` 同槽机制；consumer 按槽字节读。
    pub fn write_raw(&mut self, qwords: [u64; 5]) {
        self.write(UiAppendSignifier::from_raw_qwords(qwords));
    }

    /// `SignifierFactory` 记录入流。
    ///
    /// - 携带闭包的记录（`from_closure`，原生 `{name String, inner Arc<dyn Fn..>}`）
    ///   以 [`UiSignifierEntry::Factory`] 入流，`signifier_create` 消费时唤醒闭包。
    /// - 不带闭包的旧 raw 视图仍按 5 qword 字节位型入流（既有 wall 调用点）。
    pub fn write_factory(&mut self, factory: &SignifierFactory) {
        if factory.has_closure() {
            self.push_entry(UiSignifierEntry::Factory(factory.clone()));
            return;
        }
        self.write_raw([
            factory.name_len as u64,
            factory.name_ptr as u64,
            factory.name_len_dup as u64,
            factory.inner as u64,
            factory.marker,
        ]);
    }

    /// 移出全部待消费请求，不克隆名字或闭包，也不改变本帧活动集合。
    pub fn take_entries(&mut self) -> Vec<UiSignifierEntry> {
        std::mem::take(&mut self.slots)
    }

    /// 排空并**重置**整个流，只返回旧 5-qword 记录（crate 旧消费面兼容）。
    ///
    /// **注意**：闭包记录（[`UiSignifierEntry::Factory`]）会被丢弃；需要完整流的
    /// 消费者（`signifier_create`）应使用 [`Self::take_entries`]。
    pub fn take_all(&mut self) -> Vec<UiAppendSignifier> {
        self.take_entries()
            .into_iter()
            .filter_map(|entry| match entry {
                UiSignifierEntry::Raw(record) => Some(record),
                UiSignifierEntry::Factory(_) => None,
            })
            .collect()
    }

    /// 排空游标迭代面（crate 内 audio 消费者沿用旧名；语义同 [`Self::take_all`]：
    /// 一次消费整批，多个消费者中先到者取得记录——原生 consumer 只有
    /// `signifier_create`，此面不新增原生语义）。
    pub fn drain(&mut self) -> std::vec::IntoIter<UiAppendSignifier> {
        self.take_all().into_iter()
    }

    /// 已提交长度（调试/判等用）。
    pub fn committed_len(&self) -> u64 {
        self.slots.len() as u64
    }
}

/// Original B 140b2cfa0: existing hashes are a snapshot taken before commands.
/// Stream factories append to the uniquely owned active params, whose complete
/// factory list remains alive until ui_update replaces that captured owner.
pub fn signifier_create(
    mut ui_state: ResMut<UiState>,
    mesh_library: Res<AssetMeshLibrary>,
    shader_library: Res<AssetShaderLibrary>,
    mut commands: Commands,
    existing: Query<&SignifierId, Without<SignifierMarkedForDeletion>>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let existing_hashes: HashSet<u64> = existing.iter().map(|id| id.0).collect();
    // B140b2d419/41d branches before the stream take/reset at443/456.
    if ui_state.hover_byte() == 0 {
        return;
    }
    let entries = stream.take_entries();
    if !entries.is_empty() {
        // B140b2d4f9..514 checks Arc weak/strong uniqueness before append.
        // Do not clone this Arc or silently copy-on-write the geometry here.
        let params = ui_state.active_params_mut().expect("active UI has captured WidgetParams");
        for entry in entries {
            match entry {
                UiSignifierEntry::Factory(factory) => {
                    assert!(factory.has_closure(), "native signifier factory must own its closure");
                    params.signifiers.push(factory);
                }
                UiSignifierEntry::Raw(_) => panic!(
                    "unrestored raw signifier producer: recover its real factory before integration"
                ),
            }
        }
    }
    let widget = ui_state.active_widget();
    let params = ui_state.active_params_ref().expect("active UI has captured WidgetParams");
    for factory in &params.signifiers {
        assert!(factory.has_closure(), "native signifier factory must own its closure");
        let hash = crate::utils::calculate_hash(&(widget, factory.name()));
        if existing_hashes.contains(&hash) {
            continue;
        }
        // Original only reads the initial hash set. Equal new factories in one
        // batch each wake; inventing seen.insert() changes deferred semantics.
        for entity in factory.wake(&mesh_library, &shader_library, &mut commands) {
            commands.entity(entity).insert(SignifierId(hash, widget));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot_qwords(stream: &UiSignifierStream) -> [u64; 5] {
        let UiSignifierEntry::Raw(s) = &stream.slots[0] else {
            panic!("expected a raw 5-qword slot");
        };
        [
            s.tag,
            s.kind,
            (s.position.x.to_bits() as u64) | ((s.position.y.to_bits() as u64) << 32),
            (s.position.z.to_bits() as u64) | ((s.scale.to_bits() as u64) << 32),
            s.name,
        ]
    }

    #[test]
    fn write_raw_preserves_the_five_qword_payload() {
        let mut stream = UiSignifierStream::default();
        let q = [
            0x1111_1111_1111_1111,
            0x2222_2222_2222_2222,
            0x3f80_0000_4000_0000, // x=1.0 | y=2.0<<32
            0x4040_0000_3fc0_0000, // z=3.0 | scale=1.5<<32
            0x5555_5555_5555_5555,
        ];
        stream.write_raw(q);
        assert_eq!(slot_qwords(&stream), q);
    }

    #[test]
    fn write_factory_carries_the_signifier_factory_record_into_the_stream() {
        let mut stream = UiSignifierStream::default();
        let factory = SignifierFactory::new(
            b"perimeter".as_slice(),
            &42u32 as *const u32 as *const u8,
            0x142efa150,
        );
        stream.write_factory(&factory);
        let q = slot_qwords(&stream);
        assert_eq!(q[0], factory.name_len as u64);
        assert_eq!(q[1], factory.name_ptr as u64);
        assert_eq!(q[2], factory.name_len_dup as u64);
        assert_eq!(q[3], factory.inner as u64);
        assert_eq!(q[4], factory.marker);
    }

    #[test]
    fn writes_never_drop_records_and_never_wrap() {
        let mut stream = UiSignifierStream::default();
        for i in 0..32u64 {
            stream.write(UiAppendSignifier::new(i, 1, Vec3::ZERO, 0.5));
        }
        assert_eq!(stream.committed_len(), 32);
        let all = stream.take_all();
        assert_eq!(all.len(), 32);
        assert_eq!(all[31].tag, 31);
        assert_eq!(stream.committed_len(), 0);
    }

    #[test]
    fn closure_factory_keeps_the_native_slot_and_wakes_once_drained() {
        use std::sync::Arc;

        let mut stream = UiSignifierStream::default();
        let factory = SignifierFactory::from_closure(
            "debug_line",
            Arc::new(|_mesh, _shader, _commands| Vec::new()),
        );
        stream.write_factory(&factory);
        assert_eq!(stream.committed_len(), 1);
        let UiSignifierEntry::Factory(slot) = &stream.slots[0] else {
            panic!("closure factory must occupy a native slot");
        };
        assert!(slot.has_closure());
        assert_eq!(slot.name(), Some("debug_line"));
        let entries = stream.take_entries();
        assert!(matches!(entries.as_slice(), [UiSignifierEntry::Factory(_)]));
        assert_eq!(stream.committed_len(), 0);
    }

    #[test]
    fn legacy_take_all_sees_only_raw_slots() {
        use std::sync::Arc;

        let mut stream = UiSignifierStream::default();
        stream.write(UiAppendSignifier::new(7, 1, Vec3::ZERO, 0.5));
        stream.write_factory(&SignifierFactory::from_closure(
            "native",
            Arc::new(|_mesh, _shader, _commands| Vec::new()),
        ));
        let raw = stream.take_all();
        assert_eq!(raw.len(), 1);
        assert_eq!(raw[0].tag, 7);
    }
}
