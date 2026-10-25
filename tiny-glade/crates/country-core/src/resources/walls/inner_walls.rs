//! B InnerWalls ownership and notifications, using the real map/set dependency.
//! New (+0x30), edited (+0x60) and removed (+0x90) IDs stay distinct.

use bevy_ecs::resource::Resource;
use hashbrown::{HashMap, HashSet};
use crate::resources::roofs::geometry::surface::Curve2;

use super::{WallId, inner_wall_state::WallState};
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Default, Debug)]
pub struct WallChanges {
    pub created: HashSet<WallId>,
    pub updated: HashSet<WallId>,
    pub removed: HashSet<WallId>,
}

#[derive(Resource, Default, Debug)]
pub struct InnerWalls {
    walls: HashMap<WallId, WallState>,
    changes: WallChanges,
    max_id: u64,
}

/// 0x78B（120B）墙状态记录 = `{WallId u64@0, WallState 0x70B@8}`（① r6-walls-0x78b
/// 契约 §1，O：InnerWalls HashMap 条目 stride 0x78 权威）。
/// **record[0] = WallId**（FUN_1402ca2f0 = `HashMap::insert<WallId,()>`）；qword[1..15] =
/// WallState 0x70B。O 字段：WallState +0x54 height f32 / +0x60 freehand u8 / +0x64 arch u8 /
/// +0x67 removed_supports u8；曲线区（+0x00..0x53 等）待①。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InnerWallStateRecord(pub [u8; 0x78]);

impl InnerWallStateRecord {
    /// `record[0]` = WallId（0x78B 首 qword，LE；O 解码）。
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[..8].try_into().unwrap()))
    }

    /// WallState 0x70B（record qword[1..15]，O）。
    pub fn wall_state_bytes(&self) -> &[u8; 0x70] {
        self.0[8..].try_into().unwrap()
    }

    /// 记录内 WallState 0x70B → 权威 WallState（`load_state` 0x14085ad80 消费）。
    pub fn wall_state(&self) -> WallState {
        WallState::from_record_bytes(self.wall_state_bytes())
    }

    /// WallState `+0x54` height f32（O：update_wall_state_height 0x14085ad60 写 +0x54）。
    pub fn height(&self) -> f32 {
        f32::from_le_bytes(self.wall_state_bytes()[0x54..0x58].try_into().unwrap())
    }

    /// WallState `+0x60` freehand_type u8（O：update_freehand_ty 0x140aa99e0）。
    pub fn freehand_type(&self) -> u8 {
        self.wall_state_bytes()[0x60]
    }

    /// WallState `+0x64` arch_type u8（O：update_arch_type 0x140aa9eb0）。
    pub fn arch_type(&self) -> u8 {
        self.wall_state_bytes()[0x64]
    }

    /// WallState `+0x67` removed_supports u8（O：update_supports 0x140aa9800）。
    pub fn removed_supports(&self) -> u8 {
        self.wall_state_bytes()[0x67]
    }
}

impl InnerWalls {
    /// B 0x140859f90 initializes +0xc0 to zero. It is the ID counter,
    /// not an edit generation; new_wall_w_params increments it before insertion.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_id(&self) -> u64 {
        self.max_id
    }

    pub fn set_max_id(&mut self, id: u64) {
        self.max_id = id;
    }

    /// B 0x14085a550 only clears the counter, leaving walls and notices intact.
    pub fn reset_max_id(&mut self) {
        self.max_id = 0;
    }

    pub fn get(&self, id: WallId) -> Option<&WallState> {
        self.walls.get(&id)
    }

    pub fn get_unwrapped(&self, id: WallId) -> &WallState {
        self.get(id).unwrap()
    }

    pub fn contains(&self, id: WallId) -> bool {
        self.walls.contains_key(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = (WallId, &WallState)> {
        self.walls.iter().map(|(&id, state)| (id, state))
    }

    pub fn wall_ids(&self) -> impl Iterator<Item = WallId> + '_ {
        self.walls.keys().copied()
    }

    pub fn changes(&self) -> &WallChanges {
        &self.changes
    }

    pub fn take_changes(&mut self) -> WallChanges {
        std::mem::take(&mut self.changes)
    }

    /// B 0x14085a370 increments even when the subsequent map insertion replaces
    /// an existing ID. No collision retry or counter recomputation is added.
    pub fn new_wall(&mut self, state: WallState) -> WallId {
        self.max_id = self.max_id.wrapping_add(1);
        let id = WallId(self.max_id);
        self.walls.insert(id, state);
        self.changes.created.insert(id);
        id
    }

    /// `load_state` 0x14085ad80（fresh 反编译/反汇编 2026-10-17 逐分支）：
    /// 1. **前置 3 panic**：walls / changes.updated / changes.removed 非空即 panic；
    ///    **不检查 changes.created**（`in_RCX[3]`/`[0xf]`/`[0x15]` 三个 items 探针）。
    /// 2. 记录数 0：仅释放入参 Vec 后 return；max_id 与各集合保持不变。
    /// 3. 逐 0x78B 记录：id 插 `changes.created`。
    /// 4. `HashMap::from_iter<(WallId, WallState)>` 重建 walls（旧表丢弃）。
    /// 5. `max_id = 记录 id 最大值`（解引用记录首 qword，从首元素起 max）。
    ///
    /// 记录内 WallState 曲线点列不可由 0x70B 字节重建（见
    /// `WallState::from_record_bytes` 登记）。
    pub fn load_state(&mut self, records: &[InnerWallStateRecord]) {
        assert!(
            self.walls.is_empty(),
            "assertion failed: self.walls.is_empty()"
        );
        assert!(
            self.changes.updated.is_empty(),
            "assertion failed: self.changes.updated.is_empty()"
        );
        assert!(
            self.changes.removed.is_empty(),
            "assertion failed: self.changes.removed.is_empty()"
        );
        if records.is_empty() {
            return;
        }
        let mut walls = HashMap::with_capacity(records.len());
        let mut max_id = 0u64;
        for record in records {
            let id = record.wall_id();
            self.changes.created.insert(id);
            walls.insert(id, record.wall_state());
            max_id = max_id.max(id.0);
        }
        self.walls = walls;
        self.max_id = max_id;
    }

    pub fn new_wall_w_params(
        &mut self,
        curve: Curve2,
        height: f32,
        style: u8,
        clutter_version: u8,
        demo: bool,
    ) -> WallId {
        self.new_wall(WallState::new(curve, height, style, clutter_version, demo))
    }

    /// 0x14085a200（fresh 反编译 2026-10-17）：与 `new_wall_w_params` 同构的
    /// 三操作序列——`max_id += 1` → walls insert（旧值 drop）→ created insert。
    /// 构造 WallState 的工作由其调用方完成（函数体不含状态装配）。
    pub fn new_wall_from_curve(&mut self, state: WallState) -> WallId {
        self.new_wall(state)
    }

    /// B 0x14085a4a0 inserts, then panics if an old row was displaced.
    /// The insertion survives the panic; the creation notice is only emitted
    /// on success. Restoring an explicit ID does not advance +0xc0.
    pub fn new_wall_with_id(&mut self, id: WallId, state: WallState) {
        assert!(
            self.walls.insert(id, state).is_none(),
            "Duplicate wall id: {}",
            id.0
        );
        self.changes.created.insert(id);
    }

    /// B 0x14085aa60 checks existence, notifies, then removes the authority row.
    pub fn remove(&mut self, id: WallId) -> WallState {
        assert!(self.contains(id), "Remove failed: {} doesnt exist", id.0);
        self.changes.removed.insert(id);
        self.walls.remove(&id).unwrap()
    }

    /// B 0x14085a730 keeps the ID counter while clearing all source rows.
    pub fn remove_all(&mut self) {
        self.changes.removed.extend(self.walls.keys().copied());
        self.walls.clear();
    }

    /// Successful _try_update_state instances notify before an unconditional
    /// store, including equal bits. Missing rows only produce the original warn.
    fn try_update(&mut self, id: WallId, update: impl FnOnce(&mut WallState)) -> bool {
        let Some(state) = self.walls.get_mut(&id) else {
            log::warn!(target: "country_core::resources::walls::inner_walls", "Was requested to update {} that doesn't exist", id.0);
            return false;
        };
        self.changes.updated.insert(id);
        update(state);
        true
    }

    pub fn update_wall_state(&mut self, id: WallId, state: WallState) -> bool {
        self.try_update(id, |old| *old = state)
    }

    pub fn update_wall_state_height(&mut self, id: WallId, height: f32) -> bool {
        self.try_update(id, |state| state.snapshot.source.extra = height)
    }

    pub fn update_elevation(&mut self, id: WallId, elevation: f32) -> bool {
        self.try_update(id, |state| state.snapshot.source.terrain_floor = elevation)
    }

    pub fn update_arch_height(&mut self, id: WallId, height: f32) -> bool {
        self.try_update(id, |state| {
            state.snapshot.source.observed_path_scalar = height
        })
    }

    pub fn update_wall_state_curve(&mut self, id: WallId, curve: Curve2) -> bool {
        self.try_update(id, |state| state.snapshot.source.curve = curve)
    }

    pub fn update_wall_state_enclosure(&mut self, id: WallId, shape: WallShapeSnapshot) -> bool {
        self.try_update(id, |state| {
            state.snapshot.source.curve_kind_tag = shape.tag();
            state.snapshot.source.shape = shape;
        })
    }

    /// The original writes only the tag; inactive enclosure payload is retained.
    pub fn mark_as_non_enclosed(&mut self, id: WallId) -> bool {
        self.try_update(id, |state| {
            let mut bytes = *state.snapshot.source.shape.bytes();
            bytes[..4].copy_from_slice(&2u32.to_le_bytes());
            state.snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
            state.snapshot.source.curve_kind_tag = 2;
        })
    }

    pub fn update_style(&mut self, id: WallId, style: u8) -> bool {
        self.try_update(id, |state| state.snapshot.source.style = style)
    }

    pub fn update_freehand_ty(&mut self, id: WallId, ty: u8) -> bool {
        self.try_update(id, |state| state.snapshot.metadata.source_66 = Some(ty))
    }

    /// B 0x140aaa530 writes all four bytes at +0x60. The enum's full schema
    /// is not established, so the API accepts the observed encoding explicitly.
    pub fn update_plaster_peel_bits(&mut self, id: WallId, bits: u32) -> bool {
        self.try_update(id, |state| state.snapshot.metadata.source_60_bits = bits)
    }

    pub fn update_plaster_peel_amount(&mut self, id: WallId, amount: f32) -> bool {
        self.update_plaster_peel_bits(id, amount.to_bits())
    }

    pub fn update_arch_type(&mut self, id: WallId, ty: u8) -> bool {
        self.try_update(id, |state| {
            state.snapshot.source.observed_source_flag_64 = ty
        })
    }

    pub fn update_supports(&mut self, id: WallId, supports: u8) -> bool {
        self.try_update(id, |state| state.snapshot.metadata.source_67 = supports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: u64, height: f32, style: u8) -> InnerWallStateRecord {
        let mut bytes = [0u8; 0x78];
        bytes[..8].copy_from_slice(&id.to_le_bytes());
        // WallState +0x54 height / +0x68 style（0x78B 记录内偏移 8 + 字段）。
        bytes[8 + 0x54..8 + 0x58].copy_from_slice(&height.to_le_bytes());
        bytes[8 + 0x68] = style;
        InnerWallStateRecord(bytes)
    }

    fn wall_state() -> WallState {
        WallState::new(
            Curve2 {
                points: vec![[0.0, 0.0], [1.0, 0.0]],
                points_u: vec![0.0, 1.0],
                length: 1.0,
            },
            1.0,
            0,
            0,
            false,
        )
    }

    #[test]
    fn load_state_rebuilds_map_created_and_max_id() {
        let mut walls = InnerWalls::new();
        walls.load_state(&[record(4, 1.5, 2), record(9, 2.5, 0), record(2, 0.5, 1)]);
        assert_eq!(walls.max_id(), 9);
        assert_eq!(walls.get(WallId(4)).unwrap().height(), 1.5);
        assert_eq!(walls.get(WallId(4)).unwrap().style(), 2);
        assert_eq!(walls.get(WallId(2)).unwrap().style(), 1);
        assert_eq!(walls.changes().created.len(), 3);
        assert!(walls.changes().created.contains(&WallId(2)));
        assert!(walls.changes().updated.is_empty());
        assert!(walls.changes().removed.is_empty());
        // created 集合不再作为"表"使用：重复 load 前置会 panic（walls 非空）。
    }

    #[test]
    fn load_state_empty_records_keeps_max_id() {
        let mut walls = InnerWalls::new();
        walls.set_max_id(7);
        walls.load_state(&[]);
        assert_eq!(walls.max_id(), 7);
        assert!(walls.all().next().is_none());
    }

    #[test]
    #[should_panic(expected = "self.walls.is_empty()")]
    fn load_state_requires_empty_walls() {
        let mut walls = InnerWalls::new();
        walls.new_wall(wall_state());
        walls.load_state(&[record(1, 1.0, 0)]);
    }

    #[test]
    #[should_panic(expected = "self.changes.updated.is_empty()")]
    fn load_state_requires_empty_updated_set() {
        // updated 非空仅可能来自已存在的墙；故直接构造资源表达前置（同模块可见私有字段）。
        let mut changes = WallChanges::default();
        changes.updated.insert(WallId(1));
        let mut walls = InnerWalls {
            walls: HashMap::new(),
            changes,
            max_id: 0,
        };
        walls.load_state(&[record(1, 1.0, 0)]);
    }

    #[test]
    #[should_panic(expected = "self.changes.removed.is_empty()")]
    fn load_state_requires_empty_removed_set() {
        let mut changes = WallChanges::default();
        changes.removed.insert(WallId(1));
        let mut walls = InnerWalls {
            walls: HashMap::new(),
            changes,
            max_id: 0,
        };
        walls.load_state(&[record(1, 1.0, 0)]);
    }

    #[test]
    fn new_wall_from_curve_uses_new_wall_sequence() {
        let mut walls = InnerWalls::new();
        let first = walls.new_wall_from_curve(wall_state());
        let second = walls.new_wall_from_curve(wall_state());
        assert_eq!((first.0, second.0), (1, 2));
        assert_eq!(walls.max_id(), 2);
        assert!(walls.changes().created.contains(&first));
        assert!(walls.changes().created.contains(&second));
    }
}
