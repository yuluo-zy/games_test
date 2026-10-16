//! B InnerWalls ownership and notifications, using the real map/set dependency.
//! New (+0x30), edited (+0x60) and removed (+0x90) IDs stay distinct.

use bevy_ecs::resource::Resource;
use hashbrown::{HashMap, HashSet};
use crate::resources::roofs::geometry::surface::Curve2;

use super::{WallId, inner_wall_state::WallState};
use crate::recovered_building::wall_state::WallShapeSnapshot;

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

    /// `load_state` 0x14085ad80（① r6-walls-0x78b 契约 §1/§2，O 签名）：从 0x78B
    /// 墙状态记录数组恢复（`*param_2` = 记录数、`param_2[1]` = 记录数组 ptr、元素
    /// 0x78B = `{WallId u64@0, WallState 0x70B@8}`）。
    /// 逐记录：record[0]=WallId + WallState O 字段（+0x54 height / +0x60 freehand /
    /// +0x64 arch / +0x67 removed_supports）解码；**曲线区（WallState +0x00..0x53）待①**，
    /// WallState 完整构造待①——本实现解码 O 字段 + 登记曲线区待①，不伪造。
    pub fn load_state(&mut self, records: &[InnerWallStateRecord]) {
        // 前置（O）：各内层 Vec 空。crate InnerWalls 内层为 walls HashMap——
        // 以 walls 空表达等价前置（原生 assert，本实现登记语义）。
        if !self.walls.is_empty() {
            log::warn!("InnerWalls::load_state precondition: walls not empty");
        }
        // 逐记录：record[0]=WallId + WallState O 字段解码（曲线区→WallState 构造待①）。
        for record in records {
            let id = record.wall_id();
            let height = record.height();
            let freehand = record.freehand_type();
            let arch = record.arch_type();
            let removed = record.removed_supports();
            log::trace!(
                "load_state: wall {:?} height={height} freehand={freehand} arch={arch} removed={removed} (曲线区→WallState 构造待①)",
                id
            );
        }
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
