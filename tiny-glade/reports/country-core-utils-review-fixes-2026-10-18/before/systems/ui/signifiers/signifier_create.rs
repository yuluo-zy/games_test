//! `signifier_create.rs` — UiSignifierStream + signifier_create（B 0x140b2cf90/0x140b2cfa0，O）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码）：
//! - `UiSignifierStream::write`（0x140b2cf90 是 5 B thunk，实体在
//!   `append_only_vec::AppendOnlyVec::push` 0x14146c880）：**无锁 append-only 流**，
//!   不是环形缓冲——`LOCK xadd` 预留槽位（计数器 @+0x168），按桶分配后写 0x28 B
//!   记录，再自旋等到 `committed(@+0x160) == 预留槽`，最后提交 `committed = 槽+1`。
//!   满时**不丢记录**，也**不回绕**。
//! - `signifier_create`（0x140b2cfa0）系统签名（PDB）：
//!   `ResMut<UiState>`, `Res<AssetMeshLibrary>`, `Res<AssetShaderLibrary>`,
//!   `Commands`, `Query<&SignifierId, Without<SignifierMarkedForDeletion>>`,
//!   `ResMut<UiSignifierStream>`。流程：Query 建 `HashSet<u64>` 去重 →
//!   `UiState+0x18 != 0` 时把流整块 `mem::take`（0x170 B memcpy + memset 0）→
//!   逐条 `calculate_hash<(WidgetId, Option<&str>)>`；已在 seen 中则跳过，否则经
//!   UiState 配置表的工厂（+ AssetMeshLibrary/AssetShaderLibrary）生成实体并
//!   `insert<SignifierId>`。
//!
//! GAP（登记，不编造）：
//! - 原生 `signifier_create` 先把每条记录的 `{String, Arc<closure>}` 克隆进
//!   `UiState` 的配置表（+0x20 的 0x28 B indexmap），再遍历该表唤醒闭包；crate 的
//!   `UiState` 尚未落库该表，因此直接遍历本帧流记录（可观察的 spawn 效果一致）。
//! - `SignifierEntityBuilder::build` 的 mesh/shader 解析面（`AssetMeshLibrary::add`、
//!   `try_get_handle_by_name`、`AssetShaderLibrary::get_raster_handle_by_name`、
//!   `mesh_from_pts`）仍未落库。
//! - 解锁记录：`SignifierFactory` 现带 `Arc<dyn Fn(..) -> Vec<Entity> + Send + Sync>`
//!   闭包槽（vtable 0x142ae70d0/0x142efa150），`write_factory` 以
//!   [`UiSignifierEntry::Factory`] 入流，本系统按 `(WidgetId, Option<&str>)` 哈希去重后
//!   唤醒（`AssetMeshLibrary`/`AssetShaderLibrary` 作为 `Res` 参数）。
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
pub struct SignifierId(pub u64);

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

/// 无锁 append-only 流（O：0x170 B，`committed@+0x160`、`next@+0x168`、0x28 B 槽）。
///
/// 原版用原子预留 + 提交：`next` 先自增拿到槽号，等前序写者提交（自旋），
/// 写记录后把 `committed` 推到槽号+1。**没有回绕/容量上限/丢弃旧记录**。
#[derive(Resource, Clone, Debug)]
pub struct UiSignifierStream {
    slots: Vec<UiSignifierEntry>,
    /// +0x160：已提交长度（consumer 只读到这里）。
    committed: u64,
    /// +0x168：写者预留计数器。
    next: u64,
}

impl Default for UiSignifierStream {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            committed: 0,
            next: 0,
        }
    }
}

impl UiSignifierStream {
    /// 预留槽并提交一条槽位记录（O：`AppendOnlyVec::push` 0x14146c880 的预留 →
    /// 写 0x28 B → 自旋等前序 → 提交）。语义上永不丢记录。
    fn push_entry(&mut self, entry: UiSignifierEntry) {
        let slot = self.next;
        self.next = self.next.wrapping_add(1);
        // O：`indices()` 按桶扩展；crate 用连续 Vec 承载同一 append-only 语义。
        while self.slots.len() as u64 <= slot {
            self.slots
                .push(UiSignifierEntry::Raw(UiAppendSignifier::default()));
        }
        self.slots[slot as usize] = entry;
        // O：`while (committed != slot) spin`——等待前序写者把提交游标推进到本槽。
        while self.committed != slot {
            std::hint::spin_loop();
        }
        self.committed = slot.wrapping_add(1);
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

    /// 排空并**重置**整个流（O `signifier_create`：`mem::take` 0x170 B + 源 memset 0），
    /// 返回全部槽位（raw + factory）。流回到空态。
    pub fn take_entries(&mut self) -> Vec<UiSignifierEntry> {
        let n = (self.committed as usize).min(self.slots.len());
        let out = self.slots[..n].to_vec();
        self.slots.clear();
        self.committed = 0;
        self.next = 0;
        out
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
        self.committed
    }
}

/// 记录 → 指示符标识（I：原版 `calculate_hash<WidgetId, Option<&str>>` 把 tag 作 WidgetId、
/// name 作名字句柄哈希；crate 以 `(tag, kind, name)` 折叠）。
fn record_id(record: &UiAppendSignifier) -> u64 {
    // fnv1a 64（crate 内自有，去重键）。
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in [
        record.tag.to_le_bytes(),
        record.kind.to_le_bytes(),
        record.name.to_le_bytes(),
    ]
    .concat()
    {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// `signifier_create` @0x140b2cfa0（O）：drain（mem::take）流 + Query 迭代
/// `SignifierId`（Without<MarkedForDeletion>）建 hashset 去重 + 逐条生成实体。
///
/// 原生系统参数（PDB）：`ResMut<UiState>`, `Res<AssetMeshLibrary>`,
/// `Res<AssetShaderLibrary>`, `Commands`, `Query<&SignifierId, Without<..>>`,
/// `ResMut<UiSignifierStream>`。去重键 = `calculate_hash<(WidgetId, Option<&str>)>`
/// （`UiState+0x38` 的 widget id + 记录名字；0x1405bdda0 零键 SipHash-1-3）。
/// 未命中去重时唤醒记录的 vtable（闭包返回 `Vec<Entity>`），并给每个实体插
/// `SignifierId(hash)`；已在集中则跳过。
pub fn signifier_create(
    ui_state: ResMut<UiState>,
    mesh_library: Res<AssetMeshLibrary>,
    shader_library: Res<AssetShaderLibrary>,
    mut commands: Commands,
    existing: Query<&SignifierId, Without<SignifierMarkedForDeletion>>,
    mut stream: ResMut<UiSignifierStream>,
) {
    // O：Query 迭代建 hashset 去重（Without<MarkedForDeletion>）。
    let mut seen: std::collections::HashSet<u64> = existing.iter().map(|id| id.0).collect();
    // O：`UiState+0x18 != 0` 门（ui_state.interaction().native_tag）；为 0 时流不排空。
    if ui_state.interaction().native_tag == 0 {
        return;
    }
    let entries = stream.take_entries();
    for entry in entries {
        match entry {
            UiSignifierEntry::Raw(record) => {
                let id = record_id(&record);
                if !seen.insert(id) {
                    continue;
                }
                // GAP：旧 crate 5-qword 视图无闭包，唯有 spawn 占位实体。
                commands.spawn(SignifierId(id));
            }
            UiSignifierEntry::Factory(factory) => {
                // O：`calculate_hash<(WidgetId, Option<&str>)>`；名字 = 记录载荷。
                let id = crate::utils::calculate_hash(&(
                    ui_state.interaction().widget,
                    factory.name(),
                ));
                if !seen.insert(id) {
                    continue;
                }
                // O：唤醒 vtable（`Fn(&AssetMeshLibrary, &AssetShaderLibrary,
                // &mut Commands) -> Vec<Entity>`）；给返回实体插 SignifierId。
                let entities = factory.wake(&mesh_library, &shader_library, &mut commands);
                for entity in entities {
                    commands.entity(entity).insert(SignifierId(id));
                }
            }
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
        let factory = SignifierFactory::new(b"perimeter".as_slice(), &42u32 as *const u32 as *const u8, 0x142efa150);
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
