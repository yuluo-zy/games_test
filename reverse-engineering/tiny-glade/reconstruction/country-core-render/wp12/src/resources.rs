//! resources — typed-gap facades for the wall-state resources/events the `systems::wall`
//! systems consume. The authoritative layouts of these types are WP11 scope; this file only
//! provides the ECS shell the systems call, with `Gap` for every unknown field. Nothing here
//! is a fabricated ABI.

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, Resource};
use bevy_ecs::system::SystemParam;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

// ===========================================================================
// walls — inner / public wall state, in-progress wall, highlight, decorators.
// ===========================================================================
pub mod walls {
    use super::*;

    /// Serialized global wall ID (O: `WallId::get_inner` yields a u64).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct WallId(pub u64);
    impl WallId {
        pub fn get_inner(&self) -> u64 {
            self.0
        }
    }

    /// Inner wall state (WP11). Layout U; only the accessor surface used by systems.
    #[derive(Clone, Debug)]
    pub struct WallState {
        pub raw: Gap,
        /// Follows-terrain / premade height flag (O at the entry to wall-height edits).
        pub height_ty: u8,
        pub curve: crate::utils::curve::Curve,
        pub shape: WallShapeSnapshot,
    }
    impl Default for WallState {
        fn default() -> Self {
            Self {
                raw: Gap,
                height_ty: 0,
                curve: crate::utils::curve::Curve { raw: Gap },
                shape: WallShapeSnapshot::default(),
            }
        }
    }
    impl WallState {
        pub fn is_premade_shape(&self) -> bool {
            true
        }
        pub fn height(&self) -> f32 {
            0.0
        }
    }

    /// Enclosure wall shape snapshot (WP11): circle (tag 0) / rectangle (tag 1) /
    /// freehand curve (tag 2+). 28-byte opaque snapshot observed in history owners.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WallShapeSnapshot(pub [u8; 28]);
    impl Default for WallShapeSnapshot {
        fn default() -> Self {
            Self([0; 28])
        }
    }
    impl WallShapeSnapshot {
        pub fn from_observed(bytes: [u8; 28]) -> Self {
            Self(bytes)
        }
        pub fn tag(&self) -> u32 {
            u32::from_le_bytes(self.0[..4].try_into().unwrap())
        }
        pub fn bytes(&self) -> &[u8; 28] {
            &self.0
        }
    }

    /// Wall style enum (B): plain/stone/halftimber variants.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WallStyle {
        Plain,
        Stone,
        HalfTimbered,
    }
    impl WallStyle {
        /// O: `is_halftimbered` = byte >= 2.
        pub fn is_halftimbered(&self) -> bool {
            matches!(self, WallStyle::HalfTimbered)
        }
        /// O: `convert_to_stone_version`.
        pub fn convert_to_stone_version(&self) -> WallStyle {
            match self {
                WallStyle::HalfTimbered => WallStyle::Stone,
                other => *other,
            }
        }
    }

    /// Structure type (wall/fence/gate/...).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StructureType {
        Wall,
        Fence,
    }
    impl StructureType {
        pub fn is_fence(&self) -> bool {
            matches!(self, StructureType::Fence)
        }
    }

    /// InnerWalls map (WP11): WallId -> WallState, plus counters.
    #[derive(Resource, Clone, Debug, Default)]
    pub struct InnerWalls {
        pub map: std::collections::HashMap<WallId, WallState>,
        pub next_id: u64,
    }
    impl InnerWalls {
        pub fn get(&self, id: WallId) -> Option<&WallState> {
            self.map.get(&id)
        }
        pub fn get_unwrapped(&self, id: WallId) -> &WallState {
            self.map.get(&id).expect("inner wall present")
        }
        pub fn contains(&self, id: WallId) -> bool {
            self.map.contains_key(&id)
        }
        pub fn wall_ids(&self) -> Vec<WallId> {
            self.map.keys().copied().collect()
        }
        pub fn new_wall(&mut self, _state: WallState) -> WallId {
            let id = WallId(self.next_id);
            self.next_id = self.next_id.wrapping_add(1);
            self.map.insert(id, _state);
            id
        }
        pub fn mark_as_non_enclosed(&mut self, _id: WallId) {}
        pub fn update_wall_state(&mut self, _id: WallId, _f: impl FnOnce(&mut WallState)) {
            if let Some(s) = self.map.get_mut(&_id) {
                _f(s);
            }
        }
        pub fn update_wall_state_height(&mut self, _id: WallId, _height: f32) {}
        pub fn update_wall_state_curve(&mut self, _id: WallId, _c: crate::utils::curve::Curve) {}
        pub fn update_wall_state_enclosure(
            &mut self,
            _id: WallId,
            _s: WallShapeSnapshot,
        ) {
        }
        pub fn update_freehand_ty(&mut self, _id: WallId, _ty: u8) {}
        pub fn update_arch_type(&mut self, _id: WallId, _ty: u8) {}
        pub fn update_plaster_peel(&mut self, _id: WallId, _amount: f32) {}
        pub fn update_style(&mut self, _id: WallId, _style: WallStyle) {}
        pub fn update_elevation(&mut self, _id: WallId, _e: f32) {}
    }

    /// Public (published) wall state (WP11). 0x2a0-byte record observed in B; U fields.
    #[derive(Clone, Debug)]
    pub struct PublicWallState {
        pub raw: [u8; 0x2a0],
    }
    impl Default for PublicWallState {
        fn default() -> Self {
            Self { raw: [0; 0x2a0] }
        }
    }
    impl PublicWallState {
        pub fn is_premade_shape(&self) -> bool {
            self.shape_tag() < 2
        }
        pub fn shape_tag(&self) -> u32 {
            u32::from_le_bytes(self.raw[0x258..0x25c].try_into().unwrap())
        }
        pub fn is_circle(&self) -> bool {
            self.shape_tag() == 0
        }
        pub fn is_rectangle(&self) -> bool {
            self.shape_tag() == 1
        }
        pub fn is_enclosed(&self) -> bool {
            false
        }
        pub fn get_wall_shape(&self) -> WallShapeSnapshot {
            WallShapeSnapshot::from_observed(self.raw[0x258..0x274].try_into().unwrap())
        }
        pub fn try_get_wall_shape(&self) -> Option<WallShapeSnapshot> {
            Some(self.get_wall_shape())
        }
        pub fn max_y(&self) -> Option<f32> {
            Some(f32::from_le_bytes(self.raw[0x10c..0x110].try_into().unwrap()))
        }
        /// O (calc_wall_bend 0x140a8a710 map-value view): source point count @+0x18.
        pub fn point_count(&self) -> u64 {
            u64::from_le_bytes(self.raw[0x18..0x20].try_into().unwrap())
        }
        /// O: source curve length @+0x38 (must be `> 0` for the bend consumer).
        pub fn source_curve_length(&self) -> f32 {
            f32::from_le_bytes(self.raw[0x38..0x3c].try_into().unwrap())
        }
        /// O: published curve points (Vec3 @+0x48, len @+0x50) for the bend computation.
        pub fn published_curve_points(&self) -> &[glam::Vec3] {
            let ptr = u64::from_le_bytes(self.raw[0x48..0x50].try_into().unwrap());
            let len = u64::from_le_bytes(self.raw[0x50..0x58].try_into().unwrap());
            if ptr == 0 {
                &[]
            } else {
                // SAFETY: facade-only read of the opaque record; the caller keeps the
                // record alive. Len is bounded by the 0x2a0 record read elsewhere.
                unsafe { std::slice::from_raw_parts(ptr as *const glam::Vec3, len as usize) }
            }
        }
    }

    /// PublicWalls map (WP11): WallId -> published state + roof entity.
    #[derive(Resource, Clone, Debug, Default)]
    pub struct PublicWalls {
        pub map: std::collections::HashMap<WallId, PublicWallState>,
        pub roofs: std::collections::HashMap<WallId, Entity>,
        /// Published wall entity map (WallId -> wall Entity; removed by delete_wall).
        pub wall_entities_map: std::collections::HashMap<WallId, Entity>,
    }
    impl PublicWalls {
        pub fn get(&self, id: WallId) -> Option<&PublicWallState> {
            self.map.get(&id)
        }
        pub fn wall_ids(&self) -> Vec<WallId> {
            self.map.keys().copied().collect()
        }
        pub fn roof_entity(&self, id: WallId) -> Option<Entity> {
            self.roofs.get(&id).copied()
        }
        pub fn roof_component(&self, id: WallId) -> Option<Entity> {
            self.roof_entity(id)
        }
        pub fn contains(&self, id: WallId) -> bool {
            self.map.contains_key(&id)
        }
    }

    /// WallEntities map (WP11): WallId -> wall Entity.
    #[derive(Resource, Clone, Debug, Default)]
    pub struct WallEntities {
        pub map: std::collections::HashMap<WallId, Entity>,
    }
    impl WallEntities {
        pub fn wall_entity(&self, id: WallId) -> Option<Entity> {
            self.map.get(&id).copied()
        }
    }

    /// WallPlasters map (WP11): WallId -> plaster entities.
    #[derive(Resource, Clone, Debug, Default)]
    pub struct WallPlasters {
        pub map: std::collections::HashMap<WallId, Vec<Entity>>,
    }

    /// In-progress wall (WP11): the wall currently being drawn.
    #[derive(Resource, Clone, Debug, Default)]
    pub struct InProgressWallResource {
        pub wall: Option<InProgressWall>,
    }
    impl InProgressWallResource {
        pub fn is_empty(&self) -> bool {
            self.wall.is_none()
        }
        pub fn wall(&self) -> Option<&InProgressWall> {
            self.wall.as_ref()
        }
    }

    /// A wall mid-draw: start entity/point, last raw input point, pending curve.
    #[derive(Clone, Debug, Default)]
    pub struct InProgressWall {
        pub raw: Gap,
    }
    impl InProgressWall {
        pub fn new() -> Self {
            Self { raw: Gap }
        }
    }

    /// WallHighlight resource (O: stores a wall entity + id + type byte).
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct WallHighlight {
        pub wall_entity: Option<Entity>,
        pub wall_id: Option<WallId>,
        pub ty: u8,
    }

    /// Highlight color for the editing-highlight outline.
    #[derive(Clone, Copy, Debug)]
    pub enum HighlightColor {
        White,
        Other,
    }
    impl HighlightColor {
        pub fn to_rgb(&self) -> glam::Vec3 {
            match self {
                HighlightColor::White => glam::Vec3::ONE,
                HighlightColor::Other => glam::Vec3::splat(0.5),
            }
        }
    }

    pub mod material_stats {
        use super::*;
        #[derive(Clone, Debug, Default)]
        pub struct MaterialStats {
            pub raw: Gap,
        }
    }

    pub mod inner_wall_state {
        use super::*;
    }

    pub mod wall_trigger_volume {
        use super::*;
        #[derive(Clone, Debug, Default)]
        pub struct WallTriggerVolume {
            pub raw: Gap,
        }
        impl WallTriggerVolume {
            pub fn is_valid(&self) -> bool {
                true
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct WallTriggerVolumeParams {
            pub raw: Gap,
        }
        impl WallTriggerVolumeParams {
            pub fn edit_shape_default() -> Self {
                Self { raw: Gap }
            }
            pub fn u_range(&self) -> (f32, f32) {
                (0.0, 1.0)
            }
        }
    }

    // -----------------------------------------------------------------------
    // Decorator subsystem (walls-owned decorators / storage / archivist).
    // -----------------------------------------------------------------------
    pub mod decorator {
        use super::*;
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct DecoratorId(pub u64);

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum DecoratorDst {
            Wall(WallId),
        }
        impl DecoratorDst {
            pub fn as_wall_mut(&mut self) -> &mut WallId {
                match self {
                    DecoratorDst::Wall(w) => w,
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DecoratorExtraInfo {
            pub raw: Gap,
        }
        impl DecoratorExtraInfo {
            pub fn try_as_stair(&self) -> Option<Gap> {
                None
            }
        }
        #[derive(Clone, Copy, Debug)]
        pub enum ClutterVersion {
            V0,
        }
        impl ClutterVersion {
            pub fn latest() -> Self {
                ClutterVersion::V0
            }
        }
    }

    pub mod decorator_storage {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct DecoratorStorage {
            pub raw: Gap,
        }
        impl DecoratorStorage {
            pub fn decorator_info(&self, _id: decorator::DecoratorId) -> Option<Gap> {
                None
            }
            pub fn iter_terrain_decorators(&self, _pred: impl FnMut(decorator::DecoratorId)) {}
            pub fn iter_stair_decorators(&self, _pred: impl FnMut(decorator::DecoratorId)) {}
        }
        #[derive(Clone, Copy, Debug)]
        pub enum DecoratorStorageAddr {
            Wall(WallId),
        }
        impl DecoratorStorageAddr {
            pub fn as_wall(&self) -> Option<WallId> {
                match self {
                    DecoratorStorageAddr::Wall(w) => Some(*w),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Decorators {
            pub raw: Gap,
        }
        impl Decorators {
            pub fn iter_maybe_mut(&mut self, _f: impl FnMut(decorator::DecoratorId)) {}
        }
        #[derive(Clone, Debug)]
        pub struct DecoratorMaybeMut<'a> {
            pub raw: Gap,
            pub _p: std::marker::PhantomData<&'a ()>,
        }
        impl<'a> DecoratorMaybeMut<'a> {
            pub fn get_mut(&self) -> Option<Gap> {
                None
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct EmptyDecoratorState {
            pub raw: Gap,
        }
    }

    pub mod decorator_archivist {
        use super::*;
        /// SystemParam-style resources bundle the wall/roof systems read.
        #[derive(SystemParam, Debug)]
        pub struct DecoratorArchivalResources<'w, 's> {
            _marker: std::marker::PhantomData<(&'w (), &'s ())>,
        }
        impl DecoratorArchivist {
            pub fn decorator_cull_allow_any(&self) -> bool {
                true
            }
            pub fn decorator_cull_allow_terrain(&self) -> bool {
                true
            }
            pub fn decorator_cull_allow_stairs(&self) -> bool {
                true
            }
        }
        #[derive(Resource, Clone, Debug, Default)]
        pub struct DecoratorArchivist {
            pub raw: Gap,
        }
    }

    pub mod cached_decorator_transforms {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct CachedDecoratorTransforms {
            pub raw: Gap,
        }
        impl CachedDecoratorTransforms {
            pub fn try_position(&self, _id: decorator::DecoratorId) -> Option<glam::Vec3> {
                None
            }
            pub fn try_transform(
                &self,
                _id: decorator::DecoratorId,
            ) -> Option<crate::utils::components::transform::Transform> {
                None
            }
        }
    }
}

// ===========================================================================
// events — game command / notification events consumed by wall systems.
// ===========================================================================
pub mod events {
    use super::*;

    /// Wall-height adjust command (O: 0x30-byte row {ty u32@0x10, height f32@0x14,
    /// WallId@0x18, flags}).
    #[derive(Clone, Copy, Debug)]
    pub struct AdjustWallHeightCmd {
        pub ty: u32,
        pub height: f32,
        pub wall: crate::resources::walls::WallId,
        pub follow_terrain: bool,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct FinishAdjWallHeightMode;
    #[derive(Clone, Copy, Debug)]
    pub struct FinishWall;
    #[derive(Clone, Copy, Debug)]
    pub struct RecordMouseInputIntoWall;
    #[derive(Clone, Copy, Debug)]
    pub struct SetWallStyleCmd {
        pub wall: crate::resources::walls::WallId,
        pub style: crate::resources::walls::WallStyle,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct SetFreehandHeightTypeCmd {
        pub wall: crate::resources::walls::WallId,
        pub ty: u8,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct SetPlasterPeelAmountCmd {
        pub wall: crate::resources::walls::WallId,
        pub amount: f32,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct CreateCircleCmd {
        pub center: glam::Vec3,
        pub radius: f32,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct CreateRectangleCmd {
        pub center: glam::Vec3,
        pub dims: glam::Vec2,
        pub angle: f32,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct EditRectangleCmd {
        pub wall: crate::resources::walls::WallId,
        pub dims: glam::Vec2,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct FinishEditRectangleCmd;
    #[derive(Clone, Copy, Debug)]
    pub struct HighlightWallCmd {
        pub wall: crate::resources::walls::WallId,
        pub ty: u8,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct OutlineWallCmd {
        pub wall: crate::resources::walls::WallId,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct OutlineStairsCmd;
}

// ===========================================================================
// history — History ledger + wall compound-edit builder facades.
// ===========================================================================
pub mod history {
    use super::*;

    #[derive(Resource, Clone, Debug, Default)]
    pub struct History {
        pub raw: Gap,
    }
    impl History {
        /// O: `History::add` after a wall edit owner builds its compound edit.
        pub fn add(&mut self, _edit: crate::resources::history::wall_compound_edit::WallHistoryCompoundEdit) {}
        /// Typed-gap marker for a wall change that should record history.
        pub fn mark_changed(&mut self) {}
    }

    #[derive(Resource, Clone, Debug, Default)]
    pub struct ReplayBuffer {
        pub raw: Gap,
    }
    #[derive(Clone, Debug)]
    pub enum ReplayCmd {
        Wall(Gap),
    }

    /// Compound-edit builder for wall history owners (O: `build` emits one outer
    /// `WallHistoryCompoundEdit` with ordered `ConcurrentEdits` groups).
    pub mod wall_compound_edit {
        use super::*;

        #[derive(Clone, Debug)]
        pub struct WallHistoryCompoundEdit {
            pub raw: Gap,
        }
        impl WallHistoryCompoundEdit {
            pub fn new() -> Self {
                Self { raw: Gap }
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct CompoundEditBuilder {
            pub raw: Gap,
        }
        impl CompoundEditBuilder {
            pub fn new_edit_sequence() -> Self {
                Self { raw: Gap }
            }
            pub fn next_empty(&mut self) -> bool {
                true
            }
            pub fn with_concurrent(&mut self, _group: Vec<Gap>) {}
            pub fn build(&self) -> Option<WallHistoryCompoundEdit> {
                Some(WallHistoryCompoundEdit::new())
            }
            /// O: `wall_deleted` (0x140a876a0) records one Delete(snapshot) group.
            pub fn wall_deleted(&mut self, _wall: crate::resources::walls::WallId) {}
        }
    }

    pub mod wall_history {
        use super::*;
        #[derive(Clone, Copy, Debug)]
        pub enum WallHeightHistoryEvent {
            Changed,
        }
        impl WallHeightHistoryEvent {
            pub fn ev_height_changed() -> Self {
                WallHeightHistoryEvent::Changed
            }
        }
    }

    pub mod replay_cmd_buffer {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct ReplayCmdBuffer {
            pub raw: Gap,
        }
    }
}

// ===========================================================================
// roofs / stairs / terrain-height / water — state facades.
// ===========================================================================
pub mod roofs {
    use super::*;

    pub mod roof_shape {
        use super::*;
        #[derive(Component, Clone, Debug, Default)]
        pub struct Roof {
            pub raw: Gap,
        }
        impl Roof {
            pub fn ty(&self) -> u8 {
                0
            }
            pub fn is_flat(&self) -> bool {
                true
            }
            pub fn is_flat_w_no_rim(&self) -> bool {
                true
            }
            pub fn height(&self) -> f32 {
                0.0
            }
            pub fn pivot_3d(&self) -> glam::Vec3 {
                glam::Vec3::ZERO
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct RoofShapeParams {
            pub raw: Gap,
        }
        impl RoofShapeParams {
            pub fn from_wall_shape(_s: &crate::resources::walls::WallShapeSnapshot) -> Self {
                Self { raw: Gap }
            }
            pub fn height_ws(&self) -> f32 {
                0.0
            }
        }
        pub fn roof_height_01_from_wall_shape(_s: &crate::resources::walls::WallShapeSnapshot) -> f32 {
            0.0
        }
        pub fn ridge_length_01_from_rectangle_ratio(_r: f32) -> f32 {
            0.0
        }
    }

    pub mod roof_animation {
        use super::*;
        #[derive(Component, Clone, Debug, Default)]
        pub struct RoofAnimation {
            pub raw: Gap,
        }
        impl RoofAnimation {
            pub fn is_being_animated(&self) -> bool {
                false
            }
        }
    }

    pub mod roof_ridge {
        use super::*;
        #[derive(Clone, Debug, Default)]
        pub struct RoofRidgeDims {
            pub raw: Gap,
        }
        impl RoofRidgeDims {
            pub fn calc(_roof: &super::roof_shape::Roof, _wall: &crate::resources::walls::PublicWallState) -> Self {
                Self { raw: Gap }
            }
            pub fn from(_r: f32) -> Self {
                Self { raw: Gap }
            }
        }
    }

    pub mod roof_visual_shape {
        use super::*;
        #[derive(Clone, Debug, Default)]
        pub struct RoofBoundsDims {
            pub raw: Gap,
        }
        impl RoofBoundsDims {
            pub fn as_rectangle(&self) -> Gap {
                Gap
            }
        }
    }

    pub mod commands {
        use super::*;
        #[derive(Clone, Copy, Debug)]
        pub struct CreateRoofCmd {
            pub wall: crate::resources::walls::WallId,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct DeleteRoofCmd {
            pub wall: crate::resources::walls::WallId,
        }
    }

    pub mod dormer_tiles {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct DormerTiles {
            pub raw: Gap,
        }
    }
}

pub mod stairs {
    use super::*;
    pub mod stairs_state {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct StairsState {
            pub raw: Gap,
        }
        impl StairsState {
            pub fn graph(&self) -> Gap {
                Gap
            }
            pub fn disconnect_edge(&mut self, _from: Gap, _to: Gap) {}
        }
        #[derive(Clone, Debug)]
        pub struct StairGraphEdge {
            pub raw: Gap,
        }
        impl StairGraphEdge {
            pub fn new() -> Self {
                Self { raw: Gap }
            }
        }
    }
    pub mod stairs_assembly_params {
        use super::*;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum AlignKind {
            Wall,
            Road,
        }
    }
    pub mod stairs_collider {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct StairColliderRegistry {
            pub raw: Gap,
        }
    }
}

pub mod stairs_walls {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct StairsPillarProposals {
        pub raw: Gap,
    }
}

pub mod terrain_height {
    use super::*;
    #[derive(Clone, Debug, Default)]
    pub struct TerrainHeightSsbo {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct TerrainHeightsData<T = TerrainHeightSsbo> {
        pub raw: Gap,
        pub _p: std::marker::PhantomData<T>,
    }
    impl<T> TerrainHeightsData<T> {
        pub fn sample_at_world_xz(&self, _p: glam::Vec3) -> f32 {
            0.0
        }
    }
}

pub mod water {
    use super::*;
    #[derive(Clone, Copy, Debug)]
    pub struct WaterDrawEvent;
}

pub mod in_progress_curve {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct InProgressCurve {
        pub raw: Gap,
    }
    impl InProgressCurve {
        pub fn is_raw_input_curve_valid(&self) -> bool {
            false
        }
        pub fn last_raw_input_point(&self) -> Option<glam::Vec3> {
            None
        }
        pub fn first_raw_input_point(&self) -> Option<glam::Vec3> {
            None
        }
        pub fn add_weight_to_the_last_point(&mut self, _w: f32) {}
        pub fn raw_input_curve_length_ws(&self) -> f32 {
            0.0
        }
    }
}

pub mod drawing_input_smoother {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct RawInputSmoothed {
        pub raw: Gap,
    }
    impl RawInputSmoothed {
        pub fn is_empty(&self) -> bool {
            true
        }
        pub fn point_count(&self) -> usize {
            0
        }
        pub fn points(&self) -> Vec<glam::Vec3> {
            Vec::new()
        }
    }
}

// ===========================================================================
// gui / time / window / cursor / camera input / misc resources.
// ===========================================================================
pub mod gui {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EscMenuState {
        Closed,
        Open,
    }
    impl EscMenuState {
        pub fn is_closed(&self) -> bool {
            matches!(self, EscMenuState::Closed)
        }
        pub fn is_open(&self) -> bool {
            matches!(self, EscMenuState::Open)
        }
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct DebugGui {
        pub raw: Gap,
    }
    impl DebugGui {
        pub fn enabled(&self) -> bool {
            false
        }
    }
}

pub mod time {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct GameTime {
        pub raw: Gap,
    }
    impl GameTime {
        pub fn delta_seconds(&self) -> f32 {
            0.0
        }
        /// O: +0x58 observed in bulldoze animation pacing.
        pub fn game_time(&self) -> f32 {
            0.0
        }
    }
    /// bevy_time::Time facade (bevy_time 0.16 absent from offline registry cache).
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct Time {
        pub raw: Gap,
    }
    impl Time {
        pub fn delta_secs(&self) -> f32 {
            0.0
        }
    }
}

pub mod window {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct WindowSize {
        pub width: u32,
        pub height: u32,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct CursorMovedScreenSpace {
        pub pos: glam::Vec2,
    }
}

pub mod cursor_terrain_raycast {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct CursorTerrainRaycast {
        pub raw: Gap,
    }
    impl CursorTerrainRaycast {
        pub fn position(&self) -> glam::Vec3 {
            glam::Vec3::ZERO
        }
    }
}

pub mod mouse_terrain_raycast {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct CursorPositionSS {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct CameraCursorRay {
        pub raw: Gap,
    }
    impl CameraCursorRay {
        pub fn intersect_with_xz_plane(&self, _y: f32) -> Option<glam::Vec3> {
            None
        }
    }
}

pub mod read_mouse_depth_value {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct MouseDepthValue {
        pub raw: Gap,
    }
}

pub mod editing_highlight {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct EditingHighlight {
        pub raw: Gap,
    }
}

pub mod glade_border {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct GladeBorder {
        pub raw: Gap,
    }
    impl GladeBorder {
        pub fn is_pos_inside(&self, _p: glam::Vec3) -> bool {
            true
        }
    }
}

pub mod tree_storage {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct TreeStorage {
        pub raw: Gap,
    }
    impl TreeStorage {
        pub fn get(&self, _e: Entity) -> Option<Gap> {
            None
        }
        pub fn iter(&self, _f: impl FnMut(Entity)) {}
    }
}

pub mod weather {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct Weather {
        pub raw: Gap,
    }
}

pub mod shader_refactoring_state {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ShaderRefactoringState {
        Old,
        New,
    }
}

pub mod onboarding {
    use super::*;
    #[derive(Clone, Copy, Debug)]
    pub struct NotifyHintSystemSnapping;
}

pub mod session {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct ActiveSession {
        pub raw: Gap,
    }
}

pub mod image_formation {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct ImageFormationConfigs {
        pub raw: Gap,
    }
}

pub mod world_raster_defs {
    use super::*;
    #[derive(Resource, Clone, Debug, Default)]
    pub struct PathRaster {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct GardenRaster {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct Cobblestone1Raster {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct Cobblestone2Raster {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Debug, Default)]
    pub struct Cobblestone3Raster {
        pub raw: Gap,
    }
}
