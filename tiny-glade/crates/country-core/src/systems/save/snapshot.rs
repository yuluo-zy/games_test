//! B `systems/save/snapshot.rs` — 快照捕获/加载。
//!
//! 快照 schema 为 **Motomoto chunks + JSON + deflate**：元数据字段
//! 共十四项，格式 Version=2。Snapshot chunk 保存 JSON；之后为命名 Raster
//! chunks（u64 原长 + deflate6）和 Terrain chunk（仅 deflate6）。
//!
//! - `capture_game_snapshot_save_data` 0x1417a0430：从世界数据构建快照结构并返回 **Arc**。
//!   各节载荷由所属域序列化器提供（StairsSnapshot{edges,checksum}、
//!   TerrainEditingData{strokes,next_stroke_id,next_stroke_index}、
//!   DecoratorStorageSnapshot{states} 等）；本层不伪造域载荷。
//! - The production `SaveSnapshotWorld` gathers actual Query/resource owners;
//!   the async worker encodes that owned capture using the single chunk codec.
//! - `validate_and_load_closest_snapshot_data` 0x1417a6e50：解析器，节序 + Version 校验
//!   （"Version mismatch; expected , got "）+ Terrain 解压 + Raster 键白名单外忽略
//!   （"Ignoring unknown raster data in snapshot: "）。
//! - `load_snapshot_file`：文件入口，装配 motomoto chunks（Snapshot JSON + 命名栅格 +
//!   Terrain）为 [`LoadedSnapshotData`]。
//! - `init_game_state_from_snapshot` 0x1417a9c50：World 级域写回（Walls/Roofs/
//!   Decorators/Terrain/高度/AutoClutter/Raster/Stairs），并发送原Roof/三色/Tree命令；未闭合项记入
//!   [`SnapshotApplyReport::gaps`]。
use crate::resources::{
    color_id::ColorId,
    roofs::associations::{ResolvedRoofAssociation, RoofAssociations},
    roofs::roof_shape::Roof,
    session::SessionId,
    stairs::{
        stairs_removed_supports::StairsRemovedSupports,
        stairs_state::{StairsSnapshot, StairsState},
    },
    terrain_editing::TerrainEditingData,
    walls::{
        WallId,
        decorator::DecoratorIdGen,
        decorator_storage::{
            DecoratorAddrSnapshot, DecoratorStorageAddr, DecoratorStorageSnapshot,
        },
        inner_wall_state::WallState,
        public_walls::PublicWalls,
        publication_fields::PublicWallBuildingState,
        publication_fields::WallId as PublicWallId,
    },
    world_raster_defs::{AllWorldRasterWrite, NamedRasterData},
};
use serde_json::Value;

/// `RoofAssociations` 在原版是墙-屋顶关联表 owner；本 crate 的
/// `resources/roofs/associations.rs` 未派生 `Resource`（该文件不在本轮文件权内）。
/// 同 crate 内为其登记 `Resource`，使快照 apply 能把重建结果交给生产世界。
impl bevy_ecs::resource::Resource for RoofAssociations {}
use std::{
    io::{Cursor, Write},
    ops::Range,
    sync::Arc,
};

/// 装饰存储地址排序键（O：decorator_storage::impl_18::sort；枚举判别序
/// 0=Wall/1=Terrain/2=Stair，Wall 附 payload）。用于快照采集排序。
#[inline]
fn addr_sort_key(addr: DecoratorStorageAddr) -> (u8, u64) {
    match addr {
        DecoratorStorageAddr::Wall(id) => (0, id.0),
        DecoratorStorageAddr::Terrain => (1, 0),
        DecoratorStorageAddr::Stair => (2, 0),
    }
}

/// 快照 Trees 节条目（X5，RE-X5X7 §1：Trees = `Query<(&PrefabInstanceData,&PrefabInstance,
/// &TreeComponent)>` → map 出 `(TreeStorageIndex, TreeData)`）。`TreeData` 原类型待域恢复；
/// 以 `tree_edit::TreeEditData`（id/position/scale/rotation/prefab_name/is_preplaced）登记。
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotTreesEntry {
    pub index: crate::resources::tree_storage::TreeStorageIndex,
    pub tree: crate::resources::history::tree_edit::TreeEditData,
}

// B collect_seq<Vec<(TreeStorageIndex, TreeData)>>: the public struct remains
// convenient for collectors, while the file contains a two-element tuple.
impl serde::Serialize for SnapshotTreesEntry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&(&self.index, &self.tree), serializer)
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotTreesEntry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (index, tree) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self { index, tree })
    }
}

/// 快照 Trees 节收集器（O：capture_game_snapshot_save_data closure_env$0 逐字）。
/// 把树 Query 数据映射为快照 Trees 条目。
pub fn collect_trees(
    entries: impl IntoIterator<
        Item = (
            crate::resources::tree_storage::TreeStorageIndex,
            crate::resources::history::tree_edit::TreeEditData,
        ),
    >,
) -> Vec<SnapshotTreesEntry> {
    entries
        .into_iter()
        .map(|(index, tree)| SnapshotTreesEntry { index, tree })
        .collect()
}

/// Trees 节反序列化（G1① 契约 §3.2，36 样本实证）。
///
/// 590 条真实树记录中 **50 条缺 `prefab_name` / `is_preplaced`**，原版 Deserialize
/// 对这两字段取缺省（空串 / false）。`TreeEditData` 的 canonical 定义在
/// `resources/history/tree_edit.rs`（不在本组文件权内），故此处解析前补齐这两个缺省键，
/// 其余字段一律要求原样存在（不编造）。
pub fn parse_trees_section(value: &Value) -> anyhow::Result<Vec<SnapshotTreesEntry>> {
    use crate::resources::history::tree_edit::TreeEditData;
    use crate::resources::tree_storage::TreeStorageIndex;
    let Value::Array(entries) = value else {
        anyhow::bail!("Trees: expected an array");
    };
    let mut out = Vec::with_capacity(entries.len());
    for entry in entries {
        let Value::Array(pair) = entry else {
            anyhow::bail!("Trees: expected [TreeStorageIndex, TreeData] pairs");
        };
        anyhow::ensure!(
            pair.len() == 2,
            "Trees: expected 2-element pairs, got {}",
            pair.len()
        );
        let index: TreeStorageIndex = serde_json::from_value(pair[0].clone())
            .map_err(|error| anyhow::anyhow!("Trees index: {error}"))?;
        let mut tree = pair[1].clone();
        let Some(map) = tree.as_object_mut() else {
            anyhow::bail!("Trees: tree record is not an object");
        };
        map.entry("prefab_name")
            .or_insert_with(|| Value::String(String::new()));
        map.entry("is_preplaced").or_insert(Value::Bool(false));
        let tree: TreeEditData =
            serde_json::from_value(tree).map_err(|error| anyhow::anyhow!("Trees data: {error}"))?;
        out.push(SnapshotTreesEntry { index, tree });
    }
    Ok(out)
}

/// Wall 采集器（0x141308b40，① T3 t3-save-domain-collectors §1）：`inner_walls::all()`
/// HashMap<WallId, WallState> → Vec<(WallId, WallState)>（capture_game_snapshot_save_data
/// closure_env$1）。inner_walls 容器访问面待取证（宿主按容器接线）；采集输出 = 快照 Wall
/// 节载荷，序列化节布局随 save-DELIVERY 待取证，不编造字段名。
pub fn collect_walls(
    walls: impl IntoIterator<Item = (WallId, WallState)>,
) -> Vec<(WallId, WallState)> {
    walls.into_iter().collect()
}

/// u64 集合采集器（0x141309840，① T3 §1）：`HashSet<u64>` → Vec<u64>（decorator/stroke id
/// 集候选；来源集身份待取证，宿主接线）。
pub fn collect_ids(ids: &std::collections::HashSet<u64>) -> Vec<u64> {
    ids.iter().copied().collect()
}

/// 快照节序（O：0x1417a6e50；RNE 键串 0x142cdc550 区）。
pub const SNAPSHOT_SECTION_ORDER: &[&str] = &[
    "Version",
    "Walls",
    "Roofs",
    "Decorators",
    "Raster",
    "WallColors",
    "GateColors",
    "HalftimberColors",
    "Terrain",
    "TerrainHeightRange",
    "Trees",
    "AutoClutter",
    "Stairs",
    "StairsRemovedSupports",
];
/// B snapshot format version, independent of the enclosing game-save version.
pub const SNAPSHOT_FORMAT_VERSION: i32 = 2;

/// 快照域收集数据（O：capture_game_snapshot_save_data 0x1417a0430 的 ~9 组世界数据）。
///
/// 各域收集器输出即本结构字段；域 Query/资源由调用方（宿主/系统）喂入，本结构承载
/// 快照收集主体。字段对应 fresh 反编译 0x1417a0430 的 from_iter 序列：
/// - `walls`：`inner_walls::all()` → `(WallId, WallState)`（closure_env_1）
/// - `roofs`：`Query<&Roof, Without<RoofMarkedForDeletion>>` → 逐实体
/// - `decorators`：`decorator_storage::take_snapshot` → `(DecoratorStorageAddr, DecoratorAddrSnapshot)`
/// - `wall_colors/gate_colors/halftimber_colors`：三色表 `iter_nondefault` → `(index, ColorId)`
/// - `terrain`：TerrainEditingData（strokes + next_index；flate2 deflate 只用于栅格尾）
/// - `trees`：`Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>` →
///   `(TreeStorageIndex, TreeData)` 投影为 [`SnapshotTreesEntry`]
/// - `stairs`：`StairsState::take_snapshot`（`backup_accurate()` 断言前置）
/// - `raster`：Raster 节 6 组 `(键, 载荷)`（fold 6 entries）
/// - `ids`：`HashSet<u64>` → `Vec<u64>`（decorator/stroke id 集候选）
///
/// 快照 JSON 节载荷（`to_section_values`）由域序列化器产出；已就绪节（Trees）直接
/// 序列化，其余节缺失不伪造（`section_values` 供域所有者注入未就绪节载荷）。
#[derive(Clone, Debug)]
pub struct SnapshotDomainData {
    /// Walls 节：`(WallId, WallState)` 对。
    pub walls: Vec<(WallId, WallState)>,
    /// Roofs 节：`Query<&Roof, Without<RoofMarkedForDeletion>>` 实体。
    pub roofs: Vec<Roof>,
    /// Decorators 节：`(DecoratorStorageAddr, DecoratorAddrSnapshot)` 快照条目。
    pub decorators: Vec<(DecoratorStorageAddr, DecoratorAddrSnapshot)>,
    /// WallColors 节：`WallColorIds::iter_nondefault` → `(index, ColorId)`。
    pub wall_colors: Vec<(usize, ColorId)>,
    /// GateColors 节：`GateColorIds::iter_nondefault`。
    pub gate_colors: Vec<(usize, ColorId)>,
    /// HalftimberColors 节：`HalftimberColorIds::iter_nondefault`。
    pub halftimber_colors: Vec<(usize, ColorId)>,
    /// Terrain 节：TerrainEditingData。
    pub terrain: TerrainEditingData,
    /// Trees 节：树 Query 投影条目。
    pub trees: Vec<SnapshotTreesEntry>,
    /// Stairs metadata: original edges plus the optional cached checksum.
    pub stairs: StairsSnapshot,
    /// Raster 节 6 组 `(键, 载荷)`（O：capture_game_snapshot_save_data fold 6 entries）。
    pub raster: Vec<(&'static str, Vec<u8>)>,
    /// Actual DeletedAutoClutter IDs, serialized as AutoClutter.
    pub ids: Vec<u64>,
    /// 调用方注入的 JSON 节载荷（域序列化器未就绪的节；与 typed 序列化输出合并，
    /// 键名即快照 JSON 键）。
    pub section_values: Vec<(&'static str, Value)>,
    /// Raw terrain bytes captured from the actual height-buffer owner. These
    /// are compressed into the final Terrain chunk, not the Terrain JSON field.
    pub terrain_data: Option<Vec<u8>>,
    /// The actual height resource's range; no bounds are inferred from strokes.
    pub terrain_height_range: Option<Range<f32>>,
    /// Actual removed-support set, including the legitimate empty set.
    pub removed_supports: Option<StairsRemovedSupports>,
}

impl SnapshotDomainData {
    /// **9 域收集器在函数内构建**（O：capture_game_snapshot_save_data 0x1417a0430
    /// 的 SystemParam bundle，fresh callees）：
    /// ① Trees `Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>`
    ///   → 0x141305170；② `inner_walls::all()` → 0x141308b40；③ `HashSet<u64>`
    ///   → 0x141309840；④ `StairsState::take_snapshot`（backup_accurate 断言）
    ///   → 0x14157d200/0x140901120；⑤ `DecoratorStorage::take_snapshot` → 0x1412fbcd0；
    /// ⑥ 三色表 `iter_nondefault` → 0x141306570；⑦ Terrain strokes + heights；
    /// ⑧ Raster 6 组 fold → 0x141477c30；⑨ Roofs Query。
    ///
    /// 调用方提供各域迭代器/资源引用（域 Query 由宿主 SystemParam 取得）；
    /// 收集结果在此函数内一次性组装，保持原收集器顺序。
    #[allow(clippy::too_many_arguments)]
    pub fn collect(
        trees: impl IntoIterator<Item = SnapshotTreesEntry>,
        walls: impl IntoIterator<Item = (WallId, WallState)>,
        ids: impl IntoIterator<Item = u64>,
        stairs: Option<StairsSnapshot>,
        decorators: impl IntoIterator<Item = (DecoratorStorageAddr, DecoratorAddrSnapshot)>,
        colors: (
            impl IntoIterator<Item = (usize, ColorId)>,
            impl IntoIterator<Item = (usize, ColorId)>,
            impl IntoIterator<Item = (usize, ColorId)>,
        ),
        terrain: TerrainEditingData,
        raster: impl IntoIterator<Item = (&'static str, Vec<u8>)>,
        roofs: impl IntoIterator<Item = Roof>,
    ) -> Self {
        Self {
            walls: walls.into_iter().collect(),
            roofs: roofs.into_iter().collect(),
            decorators: decorators.into_iter().collect(),
            wall_colors: colors.0.into_iter().collect(),
            gate_colors: colors.1.into_iter().collect(),
            halftimber_colors: colors.2.into_iter().collect(),
            terrain,
            trees: trees.into_iter().collect(),
            stairs: stairs.expect("snapshot capture requires an accurate StairsSnapshot"),
            raster: raster.into_iter().collect(),
            ids: ids.into_iter().collect(),
            section_values: Vec::new(),
            terrain_data: None,
            terrain_height_range: None,
            removed_supports: None,
        }
    }

    /// 快照采集排序（O：capture_game_snapshot 0x1417a1d00 行 ~387..497 对收集结果
    /// 做 `sort_unstable_by_key`）：
    /// - walls 按 `WallId`（closure_env_0）；roofs 按所属 `WallId`（closure_env_1，
    ///   `Roof::wall_id` = 首 8B）；
    /// - decorators 按 `DecoratorStorageAddr`（decorator_storage::impl_18::sort，枚举
    ///   判别序 0=Wall/1=Terrain/2=Stair 后 payload）；trees 按 `TreeStorageIndex`
    ///   （closure_env_2）；ids（`Vec<u64>`）按数值序（ipnsort<u64>）。
    /// 排序只对已采集节生效；域未就绪的空节不受影响。
    pub fn sorted(mut self) -> Self {
        self.walls.sort_unstable_by_key(|(id, _)| id.0);
        self.roofs.sort_unstable_by_key(|roof| roof.wall_id().0);
        self.decorators
            .sort_unstable_by_key(|(addr, _)| addr_sort_key(*addr));
        for (_, bucket) in &mut self.decorators {
            bucket.states.sort_unstable_by_key(|(id, _)| *id);
        }
        self.trees.sort_unstable_by_key(|entry| entry.index.0);
        self.ids.sort_unstable();
        self
    }

    /// Supplies the three captures absent from the older collector signature.
    /// Keeping them optional distinguishes an actual empty capture from a
    /// caller that has not supplied the original resource at all.
    pub fn with_terrain_and_supports(
        mut self,
        terrain_data: Vec<u8>,
        terrain_height_range: Range<f32>,
        removed_supports: StairsRemovedSupports,
    ) -> Self {
        self.terrain_data = Some(terrain_data);
        self.terrain_height_range = Some(terrain_height_range);
        self.removed_supports = Some(removed_supports);
        self
    }

    /// B capture_game_snapshot@1417a1d00 writes all fourteen fields. Sort
    /// borrowed domain rows, retaining the caller's buffers without copying.
    pub fn try_to_section_values(&self) -> anyhow::Result<Vec<(&'static str, Value)>> {
        use serde_json::json;
        let range = self
            .terrain_height_range
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("snapshot capture has no TerrainHeightRange"))?;
        let removed = self
            .removed_supports
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("snapshot capture has no StairsRemovedSupports"))?;
        let mut walls: Vec<_> = self.walls.iter().collect();
        walls.sort_unstable_by_key(|(id, _)| id.0);
        let mut roofs: Vec<_> = self.roofs.iter().collect();
        roofs.sort_unstable_by_key(|roof| roof.wall_id().0);
        let mut decorators: Vec<_> = self.decorators.iter().collect();
        decorators.sort_unstable_by_key(|(addr, _)| addr_sort_key(*addr));
        let states = decorators
            .into_iter()
            .map(|(addr, bucket)| {
                let mut entries: Vec<_> = bucket.states.iter().collect();
                entries.sort_unstable_by_key(|(id, _)| *id);
                let state = entries
                    .into_iter()
                    .map(|(id, state)| Ok(json!([id.0, decorator_history_state_json(state)?])))
                    .collect::<anyhow::Result<Vec<Value>>>()?;
                Ok(json!([decorator_storage_addr_json(*addr), {"state": state}]))
            })
            .collect::<anyhow::Result<Vec<Value>>>()?;
        let mut trees: Vec<_> = self.trees.iter().collect();
        trees.sort_unstable_by_key(|entry| entry.index.0);
        let mut ids = self.ids.clone();
        ids.sort_unstable();
        let mut sections = vec![
            ("Version", json!(SNAPSHOT_FORMAT_VERSION)),
            (
                "Walls",
                serde_json::to_value(
                    walls
                        .into_iter()
                        .map(|(id, state)| (id.0, state))
                        .collect::<Vec<_>>(),
                )?,
            ),
            ("Roofs", serde_json::to_value(roofs)?),
            ("Decorators", json!({"states": states})),
            (
                "Raster",
                json!(
                    self.raster
                        .iter()
                        .map(|(name, _)| *name)
                        .collect::<Vec<_>>()
                ),
            ),
            ("WallColors", serde_json::to_value(&self.wall_colors)?),
            ("GateColors", serde_json::to_value(&self.gate_colors)?),
            (
                "HalftimberColors",
                serde_json::to_value(&self.halftimber_colors)?,
            ),
            (
                "Terrain",
                json!({"strokes": self.terrain.0.strokes,
                "next_stroke_id": self.terrain.0.next_index}),
            ),
            ("TerrainHeightRange", serde_json::to_value(range)?),
            ("Trees", serde_json::to_value(trees)?),
            ("AutoClutter", json!(ids)),
            (
                "Stairs",
                json!({"edges": self.stairs.edges(),
                "checksum": self.stairs.cached_checksum()}),
            ),
            (
                "StairsRemovedSupports",
                json!({"edges": removed.edges.keys()
                .map(|edge| edge.nodes()).collect::<Vec<_>>()}),
            ),
        ];
        // Old extension callers may add independent keys, but cannot replace a
        // captured native domain or hide missing data behind injected JSON.
        for (key, value) in &self.section_values {
            anyhow::ensure!(
                !sections.iter().any(|(name, _)| name == key),
                "snapshot section extension duplicates {key}"
            );
            sections.push((*key, value.clone()));
        }
        Ok(sections)
    }

    pub fn try_to_json(&self) -> anyhow::Result<Value> {
        let object = self
            .try_to_section_values()?
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect();
        Ok(Value::Object(object))
    }
}

// Native serde adapters over the canonical owned HistoryState projection.
// They do not decode process pointers or manufacture DecoratorInfo values.
pub(crate) fn decorator_storage_addr_json(addr: DecoratorStorageAddr) -> Value {
    match addr {
        DecoratorStorageAddr::Wall(id) => serde_json::json!({"Wall": id.0}),
        DecoratorStorageAddr::Terrain => Value::String("Terrain".into()),
        DecoratorStorageAddr::Stair => Value::String("Stairs".into()),
    }
}

const CLUTTER_MESH_NAMES: [&str; 56] = [
    "plant_pot_v1",
    "ladder",
    "wheel",
    "pitchfork",
    "shovel",
    "box_w_stuff",
    "watering_can",
    "firewood",
    "bucket",
    "birdnest",
    "bench",
    "bench_w_basket",
    "bench_short",
    "barrel_w_plantpot",
    "haystack",
    "plant_pot_v2",
    "birdhouse",
    "basket",
    "basket_w_food",
    "barrel_w_food_basket",
    "potato_sack",
    "road_sign",
    "sunflower",
    "cart",
    "cart2",
    "crates_w_tools",
    "crates_w_flowers",
    "window_flowerpot",
    "window_shutter",
    "window_flowerbeds",
    "reed",
    "lilypad",
    "fallen_leaves",
    "fallen_petals",
    "olden_rock",
    "olden_tree_log",
    "olden_tree_stump",
    "pumpkin",
    "pumpkin2",
    "pumpkin_w_face",
    "cart_w_pumpkins",
    "crates_w_pumpkins",
    "olden_tree_stump_autumn",
    "barrel_w_candles",
    "anvil",
    "fabric_rolls",
    "stall_bread",
    "stall_veggies",
    "crate_fish",
    "rack_brooms",
    "flowerbox",
    "door_bell",
    "door_krans",
    "door_krans_autumn",
    "cart_w_pumpkins_regular",
    "crates_w_pumpkins_regular",
];

pub(crate) fn decorator_history_state_json(
    state: &crate::resources::history::decorator_edit::DecoratorHistoryState,
) -> anyhow::Result<Value> {
    use serde_json::json;
    let b = state.to_bytes();
    let u32_at = |at| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
    let u64_at = |at| u64::from_le_bytes(b[at..at + 8].try_into().unwrap());
    let f = |at| f32::from_bits(u32_at(at));
    let mut dst = serde_json::Map::new();
    match b[0x22] {
        0 | 1 => {
            let mut anchor = serde_json::Map::new();
            if f(0x10) != 0.0 {
                anchor.insert("wall_height_relative_y".into(), json!(f(0x10)));
            }
            let xyz = [f(0x14), f(0x18), f(0x1c)];
            if xyz.iter().any(|v| *v != 0.0) {
                anchor.insert("anchor_ws".into(), json!(xyz));
            }
            anchor.insert("corner".into(), json!(b[0x20]));
            if u64_at(8) != 0 {
                anchor.insert("platform".into(), json!(u64_at(8)));
            }
            if b[0x21] != 0 {
                anchor.insert("is_bottom_door".into(), json!(true));
            }
            if b[0x22] == 0 {
                anchor.insert("side".into(), json!("Inner"));
            }
            dst.insert(
                "Wall".into(),
                json!({"coord": {"x": f(0), "y": f(4)}, "anchor": anchor}),
            );
        }
        2 | 3 => {
            let mut attachment = serde_json::Map::new();
            let (variant, position, yaw) = if b[0x22] == 2 {
                ("Roof", "mesh_uv", "relative_yaw_radians")
            } else {
                ("Terrain", "pos_xz", "yaw_radians")
            };
            attachment.insert(position.into(), json!([f(0), f(4)]));
            if f(8) != 0.0 {
                attachment.insert(yaw.into(), json!(f(8)));
            }
            dst.insert(variant.into(), Value::Object(attachment));
        }
        4 => {
            let mut attachment = serde_json::Map::new();
            attachment.insert("graph_edge".into(), json!([u32_at(0), u32_at(4)]));
            attachment.insert("uv".into(), json!([f(8), f(0xc)]));
            if f(0x10) != 0.0 {
                attachment.insert("yaw_radians".into(), json!(f(0x10)));
            }
            dst.insert("Stair".into(), Value::Object(attachment));
        }
        other => anyhow::bail!("invalid DecoratorDst niche {other}"),
    }
    let ty = match b[0x4a] {
        index @ 0..=55 => json!({"Clutter": {"ty": CLUTTER_MESH_NAMES[index as usize]}}),
        56 => json!("CottageWindow"),
        57 => json!("GothicWindow"),
        58 => json!("ArrowSlit"),
        59 => json!("Chimney"),
        60 => json!("Lantern"),
        61 => json!("Flag"),
        63 => json!("Stairs"),
        other => anyhow::bail!("invalid DecoratorType {other}"),
    };
    let mut value = serde_json::Map::new();
    value.insert("dst".into(), Value::Object(dst));
    value.insert("ty".into(), ty);
    value.insert("rank".into(), json!({"x": b[0x48]}));
    value.insert("seed".into(), json!(u32_at(0x2c)));
    for (name, offset) in [
        ("color_id", 0x28),
        ("window_light_color", 0x34),
        ("ladder_color", 0x40),
        ("magic_color", 0x44),
    ] {
        if u32_at(offset) != 0 {
            value.insert(name.into(), json!(u32_at(offset)));
        }
    }
    if b[0x49] != 0 {
        value.insert("clv".into(), json!(b[0x49]));
    }
    for (name, offset, default) in [
        ("h_offset", 0x30, 0.0),
        ("lantern_brightness", 0x38, 1.0),
        ("stair_width", 0x3c, 1.0),
    ] {
        if f(offset) != default {
            value.insert(name.into(), json!(f(offset)));
        }
    }
    if b[0x4c] == 0 {
        value.insert("allow_connection".into(), json!(false));
    }
    if b[0x4d] == 0 {
        value.insert("chimney_smoke".into(), json!(false));
    }
    if b[0x4b] != 0 {
        let railing = match b[0x4b] {
            1 => "Low",
            2 => "High",
            3 => "Wooden",
            other => anyhow::bail!("invalid RailingType {other}"),
        };
        value.insert("railing".into(), json!(railing));
    }
    if b[0x4e] != 0 {
        value.insert("stairs_supports".into(), json!("None"));
    }
    Ok(Value::Object(value))
}

/// Owned snapshot capture used by the current Rust save-thread interface.
/// The native SnapshotSaveData has a Box-owned 0x198-byte payload; retaining Arc
/// here is an existing architecture adapter, not an assertion of native ABI.
#[derive(Clone, Debug)]
pub struct SnapshotSaveData {
    /// 所属会话。
    pub session: SessionId,
    /// 快照身份（保存文件 snapshots 节引用的 `{snapshot_id, chain_index}`）。
    pub snapshot_id: u64,
    pub chain_index: i32,
    /// 域收集数据（快照主体）。
    pub domain: SnapshotDomainData,
}

impl SnapshotSaveData {
    /// JSON metadata only; the saved file also needs the compressed chunks.
    pub fn try_to_json(&self) -> anyhow::Result<Value> {
        self.domain.try_to_json()
    }
    pub fn to_json(&self) -> Value {
        self.try_to_json()
            .expect("snapshot domain serialization failed")
    }
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        encode_game_snapshot(&self.domain)
    }
}

/// 解析后的快照数据（PDB：`snapshot::LoadedSnapshotData`）。
#[derive(Clone, Debug, Default)]
pub struct LoadedSnapshotData {
    /// 存档版本（Version 节）。
    pub version: i64,
    /// 已解析节载荷。
    pub sections: Vec<(&'static str, Value)>,
    /// Raster 节 flate2 解压尾（O：DeflateDecoder @0x1417a6e50 行 1386；Terrain 为
    /// JSON 明文，不在此）。
    pub terrain: Vec<u8>,
    /// Raster 节逐白名单键解压后的载荷（O：`ordered_names()` 6 槽 + `DeflateDecoder`
    /// + `read_exact` 到目标 composite 缓冲）。
    pub rasters: Vec<(String, Vec<u8>)>,
}

/// 快照加载所需资源面（PDB：`snapshot::SnapshotLoadResources`）。域 Query 未齐，
/// 仅登记合同；`init_game_state_from_snapshot` 的应用逻辑留待对应域恢复。
#[derive(Clone, Debug, Default)]
pub struct SnapshotLoadResources {
    pub loaded: Option<LoadedSnapshotData>,
}

/// Retains the current shared capture API. The caller supplies actual domain
/// captures; metadata serialization and file framing happen only in encode.
/// The native capture is 6,343 bytes and independently asserts accurate stairs.
pub fn capture_game_snapshot_save_data(
    session: SessionId,
    snapshot_id: u64,
    chain_index: i32,
    domain: SnapshotDomainData,
) -> Arc<SnapshotSaveData> {
    Arc::new(SnapshotSaveData {
        session,
        snapshot_id,
        chain_index,
        domain,
    })
}

/// 快照 JSON 序列化缓冲初容量（O：capture_game_snapshot 0x1417a1d00 首部
/// `MOVZX EAX,[0x143634c09]` + `MOV ECX,0x80000; MOV EDX,1; CALL __rust_alloc`；
/// 0x80000 = 512 KiB，分配失败走 handle_alloc_error）。
pub const SNAPSHOT_JSON_BUFFER_CAPACITY: usize = 0x80000;

/// The single file encoder for both explicit and automatic snapshot saves.
/// B@1417a31e2: Snapshot JSON chunk; @1417a35e7: each named raster chunk;
/// @1417a3a38: Terrain chunk. Existing motomoto owns the chunk framing.
pub fn encode_game_snapshot(domain: &SnapshotDomainData) -> anyhow::Result<Vec<u8>> {
    let terrain = domain
        .terrain_data
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("snapshot capture has no raw Terrain data"))?;
    let document = domain.try_to_json()?;
    let mut writer = motomoto::Writer::new(Cursor::new(Vec::with_capacity(
        SNAPSHOT_JSON_BUFFER_CAPACITY,
    )));
    {
        let mut chunk = writer.add_chunk("Snapshot")?;
        serde_json::to_writer(&mut chunk, &document)?;
        chunk.finish()?;
    }
    for (name, bytes) in &domain.raster {
        let mut chunk = writer.add_chunk(motomoto::tag_from_str_trunc(name))?;
        // Raster chunks alone prepend the uncompressed byte count.
        chunk.write_all(&(bytes.len() as u64).to_le_bytes())?;
        let mut compressed =
            flate2::write::DeflateEncoder::new(&mut chunk, flate2::Compression::new(6));
        compressed.write_all(bytes)?;
        compressed.finish()?;
        chunk.finish()?;
    }
    {
        let mut chunk = writer.add_chunk("Terrain")?;
        let mut compressed =
            flate2::write::DeflateEncoder::new(&mut chunk, flate2::Compression::new(6));
        compressed.write_all(terrain)?;
        compressed.finish()?;
        chunk.finish()?;
    }
    Ok(writer.finish()?.into_inner())
}

#[cfg(test)]
mod snapshot_encode_tests {
    use super::*;
    use crate::resources::{
        history::{decorator_edit::DecoratorHistoryState, tree_edit::TreeEditData},
        stairs::stairs_state::StairGraphEdge,
        tree_storage::TreeStorageIndex,
        walls::decorator::DecoratorId,
    };
    use serde_json::json;

    fn domain() -> SnapshotDomainData {
        SnapshotDomainData::collect(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Some(StairsSnapshot::new(Vec::new())),
            Vec::new(),
            (Vec::new(), Vec::new(), Vec::new()),
            TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()),
            Vec::new(),
            Vec::new(),
        )
        .with_terrain_and_supports(Vec::new(), -2.0..7.0, StairsRemovedSupports::default())
    }

    fn state() -> DecoratorHistoryState {
        DecoratorHistoryState {
            // The destination niche byte +0x22 is Terrain; +0/+4 are pos_xz.
            dst_wall: (2.0f32.to_bits() as u64) << 32 | 1.0f32.to_bits() as u64,
            u20: 3 << 16,
            u28: 0x1234_5678u64 << 32,
            u38: 1.0f32.to_bits(),
            u3c: 1.0f32.to_bits(),
            rank: 2,
            ty: 56,
            u4c: 1,
            u4d: 1,
            ..Default::default()
        }
    }

    #[test]
    fn fourteen_native_domains_keep_nonempty_owned_payloads() {
        let mut d = domain();
        let wall: WallState = serde_json::from_value(json!({
            "curve": {"points": [[0.0,0.0,0.0],[1.0,0.0,0.0]],
                "points_u": [0.0,1.0], "length": 1.0},
            "height": 3.9, "is_enclosed": "No", "style": "HalfTimbered"
        }))
        .unwrap();
        d.walls.push((WallId(0x100000007), wall));
        let mut roof = [0u8; 88];
        roof[..8].copy_from_slice(&0x100000007u64.to_le_bytes());
        d.roofs.push(Roof::from_observed(roof));
        d.decorators.push((
            DecoratorStorageAddr::Terrain,
            DecoratorAddrSnapshot {
                states: vec![(DecoratorId(23), state())],
            },
        ));
        d.decorators.push((
            DecoratorStorageAddr::Stair,
            DecoratorAddrSnapshot { states: Vec::new() },
        ));
        d.wall_colors.push((0x100000007usize, ColorId(9)));
        d.gate_colors.push((10, ColorId(11)));
        d.halftimber_colors.push((12, ColorId(13)));
        let stroke = tg_terrain::stroke::TerrainStroke {
            points: vec![[1.0, 2.0, 3.0]],
            falloff: 0.4,
            index: tg_terrain::schema::StrokeId(4),
            height_ws: 6.0,
        };
        d.terrain.0.strokes.insert(stroke.index, stroke);
        d.terrain.0.next_index = 5;
        d.trees.push(SnapshotTreesEntry {
            index: TreeStorageIndex(31),
            tree: TreeEditData {
                id: -8,
                scale: 1.5,
                prefab_name: "tree_a".into(),
                ..Default::default()
            },
        });
        d.stairs = StairsSnapshot::new(vec![(7, 8)]);
        d.removed_supports
            .as_mut()
            .unwrap()
            .record(StairGraphEdge::new(11, 12));
        d.ids = vec![0x100000000, 7];
        d.raster.push(("PathRaster", vec![1, 2, 3]));
        let v = d.try_to_json().unwrap();
        assert_eq!(v.as_object().unwrap().len(), 14);
        assert_eq!(v["Version"], 2);
        assert_eq!(v["Walls"][0][0], 0x100000007u64);
        assert_eq!(v["Walls"][0][1]["style"], "HalfTimbered");
        assert_eq!(v["Roofs"][0]["id"], 0x100000007u64);
        assert_eq!(v["Roofs"][0].as_object().unwrap().len(), 7);
        assert_eq!(v["Decorators"]["states"][0][1]["state"][0][0], 23);
        assert_eq!(
            v["Decorators"]["states"][1],
            json!(["Stairs", {"state": []}])
        );
        assert_eq!(v["WallColors"], json!([[0x100000007u64, 9]]));
        assert_eq!(v["GateColors"], json!([[10, 11]]));
        assert_eq!(v["HalftimberColors"], json!([[12, 13]]));
        assert_eq!(v["Terrain"]["next_stroke_id"], 5);
        assert!(v["Terrain"].get("rasterized").is_none());
        assert_eq!(v["Terrain"]["strokes"]["4"]["height_ws"], 6.0);
        assert_eq!(v["TerrainHeightRange"], json!({"start":-2.0,"end":7.0}));
        assert_eq!(v["Trees"][0][0], 31);
        assert_eq!(v["Trees"][0][1]["id"], -8);
        assert_eq!(v["AutoClutter"], json!([7, 0x100000000u64]));
        assert_eq!(v["Raster"], json!(["PathRaster"]));
        assert_eq!(v["Stairs"]["edges"], json!([[7, 8]]));
        assert_eq!(v["Stairs"]["checksum"], json!(d.stairs.cached_checksum()));
        assert_eq!(v["StairsRemovedSupports"], json!({"edges":[[11,12]]}));
    }

    #[test]
    fn archive_uses_existing_motomoto_and_distinct_compression_payloads() {
        let mut d = domain();
        d.raster = vec![("PathRaster", vec![0, 1, 2, 3, 254, 255])];
        d.terrain_data = Some(vec![7, 6, 5, 4, 3, 2, 1]);
        let encoded = encode_game_snapshot(&d).unwrap();
        let chunks = motomoto::Reader::new(&encoded)
            .iter()
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].tag, motomoto::tag_from_str("Snapshot"));
        let metadata: Value = serde_json::from_slice(chunks[0].data).unwrap();
        assert_eq!(metadata["Raster"], json!(["PathRaster"]));
        assert_eq!(chunks[1].tag, motomoto::tag_from_str_trunc("PathRaster"));
        assert_eq!(
            u64::from_le_bytes(chunks[1].data[..8].try_into().unwrap()),
            6
        );
        assert_eq!(
            inflate_terrain(&chunks[1].data[8..]).unwrap(),
            d.raster[0].1
        );
        assert_eq!(chunks[2].tag, motomoto::tag_from_str("Terrain"));
        // No raster-length prefix is inserted before the Terrain deflate data.
        assert_eq!(
            inflate_terrain(chunks[2].data).unwrap(),
            d.terrain_data.unwrap()
        );
    }

    #[test]
    fn missing_captures_and_native_key_overrides_are_errors() {
        let mut d = domain();
        d.terrain_data = None;
        assert!(
            encode_game_snapshot(&d)
                .unwrap_err()
                .to_string()
                .contains("raw Terrain")
        );
        d.terrain_data = Some(Vec::new());
        d.terrain_height_range = None;
        assert!(
            encode_game_snapshot(&d)
                .unwrap_err()
                .to_string()
                .contains("TerrainHeightRange")
        );
        d.terrain_height_range = Some(0.0..1.0);
        d.removed_supports = None;
        assert!(
            encode_game_snapshot(&d)
                .unwrap_err()
                .to_string()
                .contains("StairsRemovedSupports")
        );
        d.removed_supports = Some(StairsRemovedSupports::default());
        d.section_values.push(("Version", json!(27)));
        assert!(
            encode_game_snapshot(&d)
                .unwrap_err()
                .to_string()
                .contains("duplicates Version")
        );
    }

    #[test]
    fn state_defaults_nan_and_all_destination_variants_follow_native_fields() {
        let mut s = state();
        let v = decorator_history_state_json(&s).unwrap();
        assert_eq!(
            v,
            json!({"dst":{"Terrain":{"pos_xz":[1.0,2.0]}},
            "ty":"CottageWindow","rank":{"x":2},"seed":0x12345678u32})
        );
        s.u30 = (-0.0f32).to_bits();
        assert!(
            decorator_history_state_json(&s)
                .unwrap()
                .get("h_offset")
                .is_none()
        );
        s.u30 = f32::NAN.to_bits();
        s.u38 = f32::NAN.to_bits();
        s.u4c = 0;
        s.u4d = 0;
        s.u4b = 3;
        s.style = 1;
        let v = decorator_history_state_json(&s).unwrap();
        assert!(v["h_offset"].is_null());
        assert!(v["lantern_brightness"].is_null());
        assert_eq!(v["allow_connection"], false);
        assert_eq!(v["chimney_smoke"], false);
        assert_eq!(v["railing"], "Wooden");
        assert_eq!(v["stairs_supports"], "None");
        s = state();
        s.u20 = 2 << 16;
        assert_eq!(
            decorator_history_state_json(&s).unwrap()["dst"],
            json!({"Roof":{"mesh_uv":[1.0,2.0]}})
        );
        s.u20 = 4 << 16;
        s.dst_wall = (8u64 << 32) | 7;
        s.dst_secondary = (0.75f32.to_bits() as u64) << 32 | 0.25f32.to_bits() as u64;
        assert_eq!(
            decorator_history_state_json(&s).unwrap()["dst"],
            json!({"Stair":{"graph_edge":[7,8],"uv":[0.25,0.75]}})
        );
        s = state();
        s.u20 = 0;
        assert_eq!(
            decorator_history_state_json(&s).unwrap()["dst"],
            json!({"Wall":{"coord":{"x":1.0,"y":2.0},"anchor":{"corner":0,"side":"Inner"}}})
        );
        s.u20 = 1 << 16;
        assert!(
            decorator_history_state_json(&s).unwrap()["dst"]["Wall"]["anchor"]
                .get("side")
                .is_none()
        );
        s.ty = 62;
        assert!(decorator_history_state_json(&s).is_err());
    }

    #[test]
    fn trees_keep_tuple_wire_shape_and_existing_six_field_data() {
        let entry = SnapshotTreesEntry {
            index: TreeStorageIndex(44),
            tree: TreeEditData {
                id: -3,
                prefab_name: "tree".into(),
                ..Default::default()
            },
        };
        let v = serde_json::to_value(&entry).unwrap();
        assert_eq!(v[0], 44);
        assert_eq!(v[1].as_object().unwrap().len(), 6);
        let decoded: SnapshotTreesEntry = serde_json::from_value(v).unwrap();
        assert_eq!(decoded.index, entry.index);
        assert_eq!(decoded.tree.id, -3);
        assert!(decoded.tree.storage_index.is_none());
        assert!(
            serde_json::from_value::<SnapshotTreesEntry>(json!({
            "index":44, "tree":serde_json::to_value(&entry.tree).unwrap()}))
            .is_err()
        );
    }
}

/// Stairs 快照采集（O：capture_game_snapshot_save_data 0x1417a0430 的
/// `assert(res.stairs.backup_accurate())` + `iter_edges_and_stubs` →
/// `StairsSnapshot::new`）。Rust `StairsState::is_backed_up` 为 `backup_accurate()`
/// 的最近语义（原生字段名 `backup_accurate` U，登记）；`take_snapshot` 即
/// `iter_edges_and_stubs` + `StairsSnapshot::new` 的等价入口。
pub fn take_stairs_snapshot(
    state: &crate::resources::stairs::stairs_state::StairsState,
) -> StairsSnapshot {
    assert!(
        state.is_backed_up(),
        "assertion failed: res.stairs.backup_accurate()"
    );
    state.take_snapshot()
}

/// Terrain 载荷 deflate 压缩（O：flate2 deflate，0x1417a6e50 解压方互证）。
pub fn deflate_terrain(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    use std::io::Write;
    encoder.write_all(data)?;
    encoder.finish()
}

/// Terrain 载荷 deflate 解压（O："decompressing terrain data" @0x142cdc800）。
pub fn inflate_terrain(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut decoder = flate2::read::DeflateDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

/// 快照节载荷值是否符合 Raster 键白名单（O：0x1417a6e50 "Ignoring unknown raster
/// data in snapshot: "；原生 `AllWorldRasterWrite::ordered_names()` 6 名 + 按名
/// `would_write_succeed`）。白名单 = 六个命名栅格槽（world_raster_defs ORDERED_NAMES）。
pub fn is_known_raster_key(key: &str) -> bool {
    crate::resources::world_raster_defs::AllWorldRasterWrite::ordered_names()
        .iter()
        .any(|name| *name == key)
}

/// `validate_and_load_closest_snapshot_data` 0x1417a6e50：解析快照 JSON。
///
/// 节序 `Version → Walls → … → Raster`（节跳表 @0x142cdca80/98/b0）；**Version 为
/// typed `deserialize_i32`**（0x1417a6e50 行 427，非宽松 `as_i64`）；
/// Walls/Roofs/Decorators/WallColors/Terrain/Trees 逐节 **typed deserialize**
/// （0x1417a6e50 行 609/764/899/1015/1138/1164；Roof 的 serde 镜像仍待域，
/// 以 `Value` 保留并登记）。Terrain 节 = **JSON 明文 strokes**
/// （`TerrainSectionJson`，flate2 只用于栅格尾——"Decompressing raster data"
/// @0x142cdc620 区）；Raster 键按 `AllWorldRasterWrite::ordered_names()` 白名单
/// 校验（"Ignoring unknown raster data in snapshot: "）+ `would_write_succeed`
/// 目标面 + flate2 `DeflateDecoder` 尾。
///
/// 日志串（fresh `read_memory` 2026-10-17）：模板块
/// `country_core::systems::save::snapshot`、`Deserializing history snapshot`、
/// `parsing Version`、`parsing {section}`、`Ignoring unknown raster data in snapshot: `。
pub fn validate_and_load_closest_snapshot_data(json: &[u8]) -> anyhow::Result<LoadedSnapshotData> {
    log::trace!("Deserializing history snapshot");
    let value: Value = serde_json::from_slice(json)?;
    let Some(object) = value.as_object() else {
        anyhow::bail!("Snapshot document is not an object");
    };
    log::trace!("parsing Version");
    // Version：typed i32（O：deserialize_i32）。
    let version_value = object
        .get("Version")
        .ok_or_else(|| anyhow::anyhow!("Snapshot has no Version"))?;
    let version_i32: i32 = serde_json::from_value(version_value.clone())
        .map_err(|error| anyhow::anyhow!("Version: {error}"))?;
    // 原生 0x1417a6e50：版本不符 → "Version mismatch; expected , got "（panic 面；
    // 此处以 Err 保留失败语义）。
    anyhow::ensure!(
        version_i32 == SNAPSHOT_FORMAT_VERSION,
        "Version mismatch; expected {SNAPSHOT_FORMAT_VERSION}, got {version_i32}"
    );
    let version = i64::from(version_i32);
    let mut sections = Vec::new();
    let mut rasters = Vec::new();
    for key in SNAPSHOT_SECTION_ORDER {
        let Some(section) = object.get(*key) else {
            continue;
        };
        log::trace!("parsing {key}");
        match *key {
            // 逐节 typed deserialize（失败 = 解析错误，原版直接传播）。
            "Walls" => {
                // 原生 `Vec<(WallId, WallState)>`；`WallState` 的 Deserialize 即原版
                // 0x1415bb1c0 object visitor（saved_wall_state.rs）。
                serde_json::from_value::<
                    Vec<(u64, crate::resources::walls::inner_wall_state::WallState)>,
                >(section.clone())
                .map_err(|error| anyhow::anyhow!("Walls: {error}"))?;
            }
            "Roofs" => {
                // 原生 `Vec<Roof>`；Roof 的 Deserialize 即 B 0x1415c5460/0x1415a9230
                // wire view（roof_shape.rs）。
                serde_json::from_value::<Vec<Roof>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Roofs: {error}"))?;
            }
            "Decorators" => {
                serde_json::from_value::<DecoratorsSectionJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Decorators: {error}"))?;
            }
            "WallColors" | "GateColors" | "HalftimberColors" => {
                serde_json::from_value::<Vec<(u64, u32)>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("{key}: {error}"))?;
            }
            "Terrain" => {
                serde_json::from_value::<TerrainSectionJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Terrain: {error}"))?;
            }
            "TerrainHeightRange" => {
                serde_json::from_value::<Range<f32>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("TerrainHeightRange: {error}"))?;
            }
            "Trees" => {
                // 缺省 prefab_name/is_preplaced 由 parse_trees_section 补齐（G1① §3.2）。
                parse_trees_section(section)?;
            }
            "AutoClutter" => {
                // B loads the actual deleted-ID HashSet, rather than ignoring
                // this captured section or treating an invalid record as empty.
                serde_json::from_value::<std::collections::HashSet<u64>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("AutoClutter: {error}"))?;
            }
            "Stairs" => {
                serde_json::from_value::<StairsSectionJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Stairs: {error}"))?;
            }
            "StairsRemovedSupports" => {
                serde_json::from_value::<StairsRemovedSupportsJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("StairsRemovedSupports: {error}"))?;
            }
            "Raster" => {
                // 真实发行样本（r7-chain1-field §1/§5）：`"Raster": ["PathRaster", …]`
                // 只是在场栅格名表，载荷在 motomoto 二进制 chunks（`load_snapshot_file`
                // 装配进 `rasters`）。旧兼容对象形状仍按白名单逐键解压。
                if let Some(names) = section.as_array() {
                    for name in names {
                        let Some(name) = name.as_str() else {
                            anyhow::bail!("Raster: expected raster names");
                        };
                        if !is_known_raster_key(name) {
                            log::debug!("Ignoring unknown raster data in snapshot: {name}");
                        }
                    }
                } else if let Some(map) = section.as_object() {
                    for (name, payload) in map {
                        if !is_known_raster_key(name) {
                            log::debug!("Ignoring unknown raster data in snapshot: {name}");
                            continue;
                        }
                        let bytes = decode_raster_payload(payload).ok_or_else(|| {
                            anyhow::anyhow!("Decompressing raster data failed for {name}")
                        })?;
                        rasters.push((name.clone(), bytes));
                    }
                } else {
                    anyhow::bail!("Raster: expected an array of raster names");
                }
            }
            _ => {}
        }
        sections.push((*key, section.clone()));
    }
    Ok(LoadedSnapshotData {
        version,
        sections,
        terrain: Vec::new(),
        rasters,
    })
}

/// Stairs 节 serde（14 节捕获面；真实 17 版样本无此节）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct StairsSectionJson {
    #[serde(default)]
    pub edges: Vec<(u32, u32)>,
    #[serde(default)]
    pub checksum: Option<u64>,
}

/// StairsRemovedSupports 节 serde：`{"edges": [[a, b], …]}`。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct StairsRemovedSupportsJson {
    #[serde(default)]
    pub edges: Vec<(u32, u32)>,
}

/// 从快照文件载入（`load_game` 的 `{snapshot_id}.snapshot` 路径）。
///
/// 文件 = motomoto chunks（r7-chain1-field §1，真实样本实证）：
/// `"Snapshot"` JSON chunk → 逐命名栅格 chunk（u64 原长 + deflate6）→ `"Terrain"`
/// chunk（仅 deflate6，0x80000B 高度缓冲）。本函数按 `try_chunk` 0x1417a6d70 的
/// Result 语义消费每个 chunk，把栅格/地形载荷装配进 [`LoadedSnapshotData`]。
///
/// 未知 chunk 标签按 "Ignoring unknown raster data in snapshot: " 忽略。
pub fn load_snapshot_file(path: &std::path::Path) -> anyhow::Result<LoadedSnapshotData> {
    let bytes = std::fs::read(path).map_err(|error| {
        anyhow::anyhow!(
            "Failed to load history snapshot {}: {error}",
            path.display()
        )
    })?;
    let chunks = motomoto::Reader::new(&bytes)
        .iter()
        .collect::<std::io::Result<Vec<_>>>()?;
    let mut chunks = chunks.into_iter();
    let Some(first) = chunks.next() else {
        return Err(anyhow::anyhow!("No chunks left in iterator"));
    };
    let mut loaded = validate_and_load_closest_snapshot_data(&first.data)?;
    // Raster 名表给出 chunk 顺序（写出侧按 ordered_names 顺序逐 chunk）；截断到 8 字节
    // 的 tag 会让 Cobblestone1/2/3 同名，故用名表游标按顺序消歧。
    let names: Vec<String> = match loaded
        .sections
        .iter()
        .find(|(name, _)| *name == "Raster")
        .map(|(_, value)| value)
    {
        Some(Value::Array(values)) => values
            .iter()
            .filter_map(|value| value.as_str())
            .filter(|name| is_known_raster_key(name))
            .map(str::to_owned)
            .collect(),
        _ => AllWorldRasterWrite::ordered_names()
            .iter()
            .map(|name| (*name).to_string())
            .collect(),
    };
    let mut cursor = 0usize;
    let terrain_tag = motomoto::tag_from_str("Terrain");
    for chunk in chunks {
        if chunk.tag == terrain_tag {
            loaded.terrain = inflate_terrain(&chunk.data)
                .map_err(|error| anyhow::anyhow!("Terrain: {error}"))?;
            continue;
        }
        let Some(offset) = names[cursor..]
            .iter()
            .position(|name| motomoto::tag_from_str_trunc(name) == chunk.tag)
        else {
            log::debug!("Ignoring unknown raster data in snapshot: {:?}", chunk.tag);
            continue;
        };
        let name = names[cursor + offset].clone();
        cursor += offset + 1;
        // 栅格 chunk 前置 u64 原长，其余为 deflate 载荷。
        anyhow::ensure!(chunk.data.len() >= 8, "Raster {name}: truncated chunk");
        let uncompressed = u64::from_le_bytes(chunk.data[..8].try_into().unwrap());
        let payload = inflate_terrain(&chunk.data[8..]).map_err(|error| {
            anyhow::anyhow!("Decompressing raster data failed for {name}: {error}")
        })?;
        anyhow::ensure!(
            payload.len() as u64 == uncompressed,
            "Raster {name}: expected {uncompressed} bytes, got {}",
            payload.len()
        );
        loaded.rasters.push((name, payload));
    }
    Ok(loaded)
}

/// `try_chunk` 0x1417a6d70（fresh disasm 2026-10-17，148 B）：**Result 包装闭包**，
/// 消费 chunk 迭代器的一次产出，不碰文件系统。
///
/// 机器码：入参 = `Option<std::io::Result<Vec<u8>>>`（判别 qword@+0 = 1 表示 Some；
/// Vec 3 qword 位于 +8..+0x20）。`Some(Ok(chunk))` → 返回该 Vec；`Some(Err(io))`
/// → `anyhow::Error::from<std::io::Error>`（CALL 0x1428a9590 0x1417a6dea）；
/// `None` → `anyhow::__private::format_err`（CALL 0x1428a8020，串
/// 0x142cdc635 len 0x1a = **"No chunks left in iterator"**）。
pub fn try_chunk(chunk: Option<std::io::Result<Vec<u8>>>) -> anyhow::Result<Vec<u8>> {
    match chunk {
        Some(Ok(data)) => Ok(data),
        Some(Err(error)) => Err(anyhow::Error::from(error)),
        None => Err(anyhow::anyhow!("No chunks left in iterator")),
    }
}

/// 快照域描述符（O：init_game_state_from_snapshot 0x1417a9c50 fresh 反编译）。
///
/// 0xc0 头之后是 **14 个 0x20B 块**（偏移 +0xc0、+0xe0、+0x100、+0x120、+0x140、
/// +0x160、+0x180、+0x1a0、+0x1c0、+0x1e0、+0x200、+0x220、+0x240、+0x260，
/// 末块止于 +0x27c；旧 `11` 计数按 fresh 纠正为 14）。每块读取：
/// `qword@+0x00`（数据/长度字）、`qword@+0x10`（目标指针）、`u32@+0x1c`（计数）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SnapshotDomainDescriptor {
    /// +0x00：数据字（原生逐域 qword；语义随域，U）。
    pub data: u64,
    /// +0x10：目标指针（写回目标）。
    pub target: u64,
    /// +0x1c：计数（u32）。
    pub count: u32,
}

/// Legacy native-memory projection (not a Motomoto file header).
pub const SNAPSHOT_HEADER_SIZE: usize = 0xc0;
/// 域描述符个数（O：14 个 0x20B 块，+0xc0..+0x27c）。
pub const SNAPSHOT_DOMAIN_COUNT: usize = 14;
/// 域描述符 stride（O：0x20）。
pub const SNAPSHOT_DOMAIN_STRIDE: usize = 0x20;
/// 头部 + 14 描述符总长（0x280）。
pub const SNAPSHOT_PREFIX_SIZE: usize =
    SNAPSHOT_HEADER_SIZE + SNAPSHOT_DOMAIN_COUNT * SNAPSHOT_DOMAIN_STRIDE;

/// 快照 0xc0 头 + 14 域描述符（O：init_game_state_from_snapshot 0x1417a9c50 首段）。
#[derive(Clone, Debug)]
pub struct SnapshotPrefix {
    pub header: [u8; SNAPSHOT_HEADER_SIZE],
    pub domains: [SnapshotDomainDescriptor; SNAPSHOT_DOMAIN_COUNT],
}

impl Default for SnapshotPrefix {
    fn default() -> Self {
        Self {
            header: [0u8; SNAPSHOT_HEADER_SIZE],
            domains: [SnapshotDomainDescriptor::default(); SNAPSHOT_DOMAIN_COUNT],
        }
    }
}

impl SnapshotPrefix {
    /// 从快照原始 0x198+B 结构解析头部与 14 描述符（逐字段 LE）。
    /// 长度不足 → Err（不越界读）。
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < SNAPSHOT_PREFIX_SIZE {
            anyhow::bail!(
                "snapshot prefix too short: {} < {}",
                bytes.len(),
                SNAPSHOT_PREFIX_SIZE
            );
        }
        let mut header = [0u8; SNAPSHOT_HEADER_SIZE];
        header.copy_from_slice(&bytes[..SNAPSHOT_HEADER_SIZE]);
        let mut domains = [SnapshotDomainDescriptor::default(); SNAPSHOT_DOMAIN_COUNT];
        for (index, slot) in domains.iter_mut().enumerate() {
            let base = SNAPSHOT_HEADER_SIZE + index * SNAPSHOT_DOMAIN_STRIDE;
            *slot = SnapshotDomainDescriptor {
                data: u64::from_le_bytes(bytes[base..base + 8].try_into().unwrap()),
                target: u64::from_le_bytes(bytes[base + 0x10..base + 0x18].try_into().unwrap()),
                count: u32::from_le_bytes(bytes[base + 0x1c..base + 0x20].try_into().unwrap()),
            };
        }
        Ok(Self { header, domains })
    }
}

/// B init_game_state_from_snapshot's World adapter. The original R8 contains
/// actual SystemParam resource/event borrows; its first0xc0 bytes are six raster
/// borrows, not a file header or fourteen serialized domain descriptors.
/// Motomoto parsing yields the owned data consumed by this function; domain
/// requests go to the same maintained events as editing/history replay.
pub fn init_game_state_from_snapshot(
    world: &mut bevy_ecs::world::World,
    resources: &SnapshotLoadResources,
) -> anyhow::Result<SnapshotApplyReport> {
    apply_loaded_snapshot_to_world(world, resources)
}

/// `init_game_state_from_snapshot` 的**纯资源视图**（不接管 World）。
///
/// 生产读档路径走 [`init_game_state_from_snapshot`]（World 级，覆盖全部 11 个已解析域）；
/// 本函数保留给只需 Walls/Decorators/Terrain/Raster 且由调用方显式持有资源的场景
/// （现有测试）。数据流 = 节 JSON 值 → serde 反序列化 → 目标 API 写回：
/// - **Walls**：`parse_wall_section`（canonical `WallState` serde → 0x78B 记录）→
///   `InnerWalls::load_state`（0x14085ad80）。
/// - **Decorators**：`DecoratorsSectionJson` → `DecoratorStorageSnapshot` →
///   `restore_snapshot_with_id_gen`（0x140a20d70）。
/// - **Terrain**：`TerrainSectionJson`（JSON 明文 strokes）→ `TerrainEditingData`。
/// - **Raster**：6 白名单键 → `apply_raster_writeback`（0x14083ebe0）。
///
/// 该视图不处理 Roofs/WallColors/GateColors/HalftimberColors/Trees/地形高度/
/// Stairs —— 这些域由 [`apply_loaded_snapshot_to_world`] 写回。
#[allow(clippy::too_many_arguments)]
pub fn apply_loaded_snapshot(
    resources: &mut SnapshotLoadResources,
    walls: &mut crate::resources::walls::inner_walls::InnerWalls,
    decorators: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
    decorator_ids: &mut DecoratorIdGen,
    terrain: &mut TerrainEditingData,
    mut rasters: Option<&mut AllWorldRasterWrite<'_>>,
) {
    let Some(loaded) = resources.loaded.as_ref() else {
        return;
    };
    // ① r7-chain1-field 契约 §6：5 域 apply 序（validate 0x1417a6e50 按名分派）——
    // Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster。
    const APPLY_ORDER: [&str; 9] = [
        "Version",
        "Walls",
        "Roofs",
        "Decorators",
        "WallColors",
        "GateColors",
        "Terrain",
        "Trees",
        "Raster",
    ];
    for key in APPLY_ORDER {
        let Some((_, value)) = loaded.sections.iter().find(|(k, _)| *k == key) else {
            continue;
        };
        match key {
            // G1① §4：语义 WallState（含点列）直接插入，不做 0x70 字节打包。
            "Walls" => {
                match serde_json::from_value::<
                    Vec<(u64, crate::resources::walls::inner_wall_state::WallState)>,
                >(value.clone())
                {
                    Ok(rows) => {
                        if let Err(error) = load_snapshot_walls(
                            walls,
                            rows.into_iter().map(|(id, state)| (WallId(id), state)),
                        ) {
                            log::error!("apply Walls: {error}");
                        }
                    }
                    Err(error) => log::error!("apply Walls: {error}"),
                }
            }
            "Decorators" => {
                // G1① §5：JSON → DecoratorStorageSnapshot → restore_snapshot（0x140a20d70）。
                // 内层记录 0x58B = 明文 DecoratorId + DecoratorHistoryState 0x50。
                if let Ok(section) = serde_json::from_value::<DecoratorsSectionJson>(value.clone())
                {
                    match section.into_storage_snapshot() {
                        Ok(snapshot) => {
                            decorators.restore_snapshot_with_id_gen(&snapshot, decorator_ids)
                        }
                        Err(error) => log::error!("apply Decorators: {error}"),
                    }
                } else {
                    log::trace!("apply Decorators: JSON→DecoratorsSectionJson 解析失败");
                }
            }
            "Terrain" => {
                // ① r7-chain1-field 纠偏：strokes 为 JSON 明文（flate2 只用于栅格尾）。
                // ② r8 接线：JSON → TerrainEditingData → 写回地形资源。
                if let Ok(section) = serde_json::from_value::<TerrainSectionJson>(value.clone()) {
                    let editing = section.into_editing_data();
                    let strokes = editing.0.strokes.len();
                    *terrain = editing;
                    log::trace!("apply Terrain: {strokes} JSON strokes 写回地形资源");
                } else {
                    log::trace!("apply Terrain: JSON→TerrainEditingData 解析失败");
                }
            }
            // ② r8 接线 + ③ init Raster 写回（0x14083ebe0）：按 6 白名单键解码 +
            // write_named（先校验名称/长度，整批写）。
            "Raster" => {
                if value.as_object().is_some() {
                    let mut data: Vec<NamedRasterData> = Vec::with_capacity(6);
                    // validate 已逐键 flate2 解压（`loaded.rasters`）；优先消费 typed 输出，
                    // 缺失时回退对 JSON 载荷解压。
                    for name in AllWorldRasterWrite::ordered_names() {
                        let typed = loaded
                            .rasters
                            .iter()
                            .find(|(key, _)| key == name)
                            .map(|(_, bytes)| bytes.clone());
                        let bytes =
                            typed.or_else(|| value.get(name).and_then(decode_raster_payload));
                        if bytes.is_none() && value.get(name).is_some() {
                            log::trace!("apply Raster: {name} 载荷解码失败（形状待①）");
                            continue;
                        }
                        data.push((name.to_string(), bytes));
                    }
                    if let Some(rasters) = rasters.as_mut() {
                        match apply_raster_writeback(rasters, &data) {
                            Ok(()) => log::trace!("apply Raster: write_named 6 槽写回"),
                            Err(e) => log::trace!("apply Raster: 写回校验失败（整批不写）: {e}"),
                        }
                    } else {
                        log::trace!(
                            "apply Raster: 写回目标（AllWorldRasterWrite）未提供，宿主接线"
                        );
                    }
                }
            }
            // Roofs/WallColors/GateColors/HalftimberColors/Trees 由 World 级
            // `apply_loaded_snapshot_to_world` 写回。
            _ => log::trace!("apply domain {key}: 由 init_game_state_from_snapshot (World) 处理"),
        }
    }
    let _ = loaded.version;
    let _ = &loaded.terrain;
}

/// 快照应用到 World 的结果面。`gaps` 非空表示该域只完成部分写回（宿主面缺失），
/// 读取方不得把带 gap 的结果当作完整恢复。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnapshotApplyReport {
    pub walls: usize,
    /// Actual CreateRoofCmd requests queued for the canonical roof consumer.
    pub roofs: usize,
    /// Legacy direct-application report field; command-based loading leaves0.
    pub roof_associations: usize,
    pub decorators: usize,
    /// Actual color requests queued, with full IDs and custom=false.
    pub wall_colors: usize,
    pub gate_colors: usize,
    pub halftimber_colors: usize,
    /// Actual InsertTreeCmd requests queued for the canonical tree consumer.
    pub trees: usize,
    pub terrain_strokes: usize,
    pub terrain_heights: bool,
    pub raster_slots: usize,
    pub stairs: bool,
    pub removed_supports: usize,
    /// Number of unique deleted auto-clutter IDs restored to the canonical set.
    pub auto_clutter: usize,
    /// 已解析但未完整应用的项（缺资源/缺消费者/未接线域等）。
    pub gaps: Vec<String>,
}

/// 读档时 `PublicWalls` 尚未发布（`Walls` 段只写 `InnerWalls`），
/// Roofs 段的关联重建因此延后到墙发布后：apply 记录待重建行，由
/// [`rebuild_pending_roof_associations`] 在 `WallCpuStage::Publish` 之后
/// 一次成功重建。避免“读档期恒 gap、之后永不重建”。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct PendingRoofAssociationRebuild {
    pub rows: Vec<PendingRoofAssociationRow>,
}

/// 待重建行：`entity.to_bits()` + `Roof::wall_id`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingRoofAssociationRow {
    pub entity: u64,
    pub wall: u64,
}

/// 读取待重建行，等 `PublicWalls` 拥有全部墙行后调用
/// `RoofAssociations::rebuild`，并消费 `AssociationChanges`
/// （`DeleteRoofCmd` / `WallRoofPresenceChanged`）。未集齐前保持 pending，
/// 不清空、不伪造。
pub fn rebuild_pending_roof_associations(
    mut pending: Option<bevy_ecs::prelude::ResMut<PendingRoofAssociationRebuild>>,
    public: Option<bevy_ecs::prelude::Res<PublicWalls>>,
    mut associations: Option<bevy_ecs::prelude::ResMut<RoofAssociations>>,
    mut delete_roof: Option<
        bevy_ecs::prelude::ResMut<
            bevy_ecs::event::Events<crate::resources::roofs::commands::DeleteRoofCmd>,
        >,
    >,
    mut presence: Option<
        bevy_ecs::prelude::ResMut<
            bevy_ecs::event::Events<crate::resources::roofs::commands::WallRoofPresenceChanged>,
        >,
    >,
) {
    let (Some(pending), Some(public), Some(associations)) =
        (pending.as_mut(), public.as_ref(), associations.as_mut())
    else {
        return;
    };
    if pending.rows.is_empty() {
        return;
    }
    let mut projections: std::collections::HashMap<u64, PublicWallBuildingState> =
        std::collections::HashMap::new();
    for row in &pending.rows {
        if let Some(state) = public.get(WallId(row.wall)) {
            projections.insert(row.wall, state.building_projection());
        }
    }
    if pending
        .rows
        .iter()
        .any(|row| !projections.contains_key(&row.wall))
    {
        // 墙发布尚未覆盖全部行：下帧再试。
        return;
    }
    let order: Vec<PublicWallId> = public.wall_ids().map(|wall| PublicWallId(wall.0)).collect();
    let rows: Vec<ResolvedRoofAssociation<'_>> = pending
        .rows
        .iter()
        .map(|row| ResolvedRoofAssociation {
            wall_id: PublicWallId(row.wall),
            entity: row.entity,
            marked: false,
            public_wall: projections.get(&row.wall),
        })
        .collect();
    match associations.rebuild(rows, &order, 0) {
        Ok(changes) => {
            if let Some(delete_roof) = delete_roof.as_mut() {
                for wall in &changes.invalid_roof_requests {
                    delete_roof.send(crate::resources::roofs::commands::DeleteRoofCmd(WallId(
                        wall.0,
                    )));
                }
            }
            if let Some(presence) = presence.as_mut() {
                for wall in &changes.wall_presence_changes {
                    presence.send(crate::resources::roofs::commands::WallRoofPresenceChanged(
                        WallId(wall.0),
                    ));
                }
            }
            pending.rows.clear();
        }
        Err(error) => {
            log::warn!(
                target: "country_core::systems::save::snapshot",
                "deferred RoofAssociations rebuild failed: {error:?}"
            );
            pending.rows.clear();
        }
    }
}

/// Project the original snapshot Roof fields into its actual create request.
fn snapshot_roof_request(roof: &Roof) -> crate::resources::roofs::commands::CreateRoofCmd {
    use crate::resources::roofs::{commands::CreateRoofCmd, creation::RoofCreateRequestObserved};
    let source = roof.observed();
    // EventInstance metadata is owned by Bevy. Only the actual business fields
    // below are projected from Roof, matching1417aa090..1417aa0de.
    let mut record = [0; 64];
    record[8..16].copy_from_slice(&source[..8]);
    record[16..20].copy_from_slice(&source[0x50..0x54]);
    record[20..32].copy_from_slice(&source[0x40..0x4c]);
    record[32..60].copy_from_slice(&source[0x24..0x40]);
    record[60] = 1;
    CreateRoofCmd(RoofCreateRequestObserved::from_observed(record))
}

/// Apply to the canonical owners, preserving the original producer/consumer boundary.
/// B order: walls; roof requests; decorator state/ID owner; three color requests;
/// incoming terrain-stroke invalidations and owner replacement; removed supports;
/// raster write/upload; terrain heights; tree requests; deleted-clutter set; stairs.
/// Roof/tree entity construction and color/plaster effects belong to their real
/// event consumers. Report counts for those domains mean queued requests, never
/// proof that the downstream entities, geometry or GPU resources are ready.
/// Missing mandatory owners/channels fail; other unresolved targets remain gaps.
pub fn apply_loaded_snapshot_to_world(
    world: &mut bevy_ecs::world::World,
    resources: &SnapshotLoadResources,
) -> anyhow::Result<SnapshotApplyReport> {
    use crate::resources::walls::inner_wall_state::WallState;
    let Some(loaded) = resources.loaded.as_ref() else {
        return Ok(SnapshotApplyReport::default());
    };
    let mut report = SnapshotApplyReport::default();
    let section = |key: &str| {
        loaded
            .sections
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| value)
    };

    // ---- Walls（O：InnerWalls::load_state 0x14085ad80）----
    if let Some(value) = section("Walls") {
        anyhow::ensure!(
            world.contains_resource::<crate::resources::walls::InnerWalls>(),
            "B InnerWalls constructor missing"
        );
        let rows: Vec<(u64, WallState)> = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("Walls section: {error}"))?;
        report.walls = load_snapshot_walls(
            &mut world.resource_mut::<crate::resources::walls::InnerWalls>(),
            rows.into_iter().map(|(id, state)| (WallId(id), state)),
        )?;
    }

    // ---- Roofs ----
    // B @1417aa042..1417aa17b emits CreateRoofCmd. The real roof consumer
    // supplies animation, tile simulation, trigger, Nani, sounds and geometry;
    // spawning a bare Roof here bypasses those owners and their lifecycle.
    if let Some(value) = section("Roofs") {
        use crate::resources::roofs::commands::CreateRoofCmd;
        let roofs: Vec<Roof> = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("Roofs section: {error}"))?;
        if !roofs.is_empty() {
            anyhow::ensure!(
                world.contains_resource::<bevy_ecs::event::Events<CreateRoofCmd>>(),
                "B Events<CreateRoofCmd> producer missing"
            );
            let mut requests = world.resource_mut::<bevy_ecs::event::Events<CreateRoofCmd>>();
            for roof in roofs {
                let source = roof.observed();
                // Native iterator sentinel; validated file Roofs only have tags0/1.
                if u32::from_le_bytes(source[8..12].try_into().unwrap()) == 2 {
                    break;
                }
                requests.send(snapshot_roof_request(&roof));
                report.roofs += 1;
            }
        }
    }

    // ---- Decorators（O：DecoratorStorage::restore_snapshot 0x140a20d70）----
    if let Some(value) = section("Decorators") {
        anyhow::ensure!(
            world
                .contains_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>(
                ),
            "B DecoratorStorage constructor missing"
        );
        anyhow::ensure!(
            world.contains_resource::<DecoratorIdGen>(),
            "B DecoratorIdGen constructor missing"
        );
        let section: DecoratorsSectionJson = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("Decorators section: {error}"))?;
        let snapshot = section.into_storage_snapshot()?;
        let empty = {
            let storage =
                world.resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
            storage.per_wall.is_empty()
                && storage.terrain.is_none()
                && storage.stairs.is_none()
                && storage.removed.is_empty()
                && storage.decorator_to_storage.is_empty()
        };
        anyhow::ensure!(
            empty,
            "snapshot load requires an empty DecoratorStorage (B restore_snapshot assertion)"
        );
        report.decorators = snapshot
            .states
            .iter()
            .map(|(_, list)| list.states.len())
            .sum();
        world.resource_scope(
            |world,
             mut decorators: bevy_ecs::prelude::Mut<
                crate::resources::walls::decorator_storage::DecoratorStorage,
            >| {
                world.resource_scope(|_world, mut ids: bevy_ecs::prelude::Mut<DecoratorIdGen>| {
                    decorators.restore_snapshot_with_id_gen(&snapshot, &mut ids);
                });
            },
        );
    }

    // ---- Three color tables ----
    // Original loops @1417aa1ee..1417aa498 preserve the full u64 wall ID
    // and custom=false. Real consumers own table writes/plaster side effects.
    for (key, family) in [("WallColors", 0u8), ("GateColors", 1), ("HalftimberColors", 2)] {
        let Some(value) = section(key) else { continue; };
        let rows: Vec<(u64, u32)> = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("{key}: {error}"))?;
        if rows.is_empty() { continue; }
        use crate::resources::color_id::{SetWallColorIdCmd, SetGateColorIdCmd, SetHalftimberColorIdCmd};
        macro_rules! queue_colors {
            ($event:ty, $count:ident) => {{
                anyhow::ensure!(
                    world.contains_resource::<bevy_ecs::event::Events<$event>>(),
                    "B Events<{}> producer missing", stringify!($event)
                );
                let mut events = world.resource_mut::<bevy_ecs::event::Events<$event>>();
                for (id, color) in &rows {
                    events.send(<$event>::new(WallId(*id), *color, 0));
                }
                report.$count = rows.len();
            }};
        }
        match family {
            0 => queue_colors!(SetWallColorIdCmd, wall_colors),
            1 => queue_colors!(SetGateColorIdCmd, gate_colors),
            _ => queue_colors!(SetHalftimberColorIdCmd, halftimber_colors),
        }
    }

    // ---- Terrain strokes（O：TerrainEditingData）----
    if let Some(value) = section("Terrain") {
        anyhow::ensure!(
            world.contains_resource::<TerrainEditingData>(),
            "B TerrainEditingData constructor missing"
        );
        let section: TerrainSectionJson = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("Terrain section: {error}"))?;
        let editing = section.into_editing_data();
        report.terrain_strokes = editing.0.strokes.len();
        // B @1417aa4ba..1417aa60b iterates the incoming snapshot's map
        // (+100), publishing its stroke IDs before moving that map into the
        // existing TerrainEditingData owner. Invalidation schedules the real
        // height recalculation/readback; assigning the resource alone loses it.
        let stroke_ids: Vec<_> = editing.0.strokes.keys().copied().collect();
        if !stroke_ids.is_empty() {
            use crate::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged;
            anyhow::ensure!(
                world.contains_resource::<bevy_ecs::event::Events<EvTerrainStrokeChanged>>(),
                "B Events<EvTerrainStrokeChanged> producer missing"
            );
            world
                .resource_mut::<bevy_ecs::event::Events<EvTerrainStrokeChanged>>()
                .send_batch(stroke_ids.into_iter().map(EvTerrainStrokeChanged));
        }
        *world.resource_mut::<TerrainEditingData>() = editing;
    }

    // ---- StairsRemovedSupports (native order before raster restoration) ----
    if let Some(value) = section("StairsRemovedSupports") {
        let section: StairsRemovedSupportsJson = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("StairsRemovedSupports: {error}"))?;
        if world.contains_resource::<StairsRemovedSupports>() {
            let mut supports = world.resource_mut::<StairsRemovedSupports>();
            supports.edges.clear();
            for (a, b) in section.edges {
                supports.record(crate::resources::stairs::stairs_state::StairGraphEdge::new(
                    a, b,
                ));
            }
            report.removed_supports = supports.edges.len();
        } else {
            report.gaps.push(
                "StairsRemovedSupports: resource missing; removed-support set not restored"
                    .to_string(),
            );
        }
    }

    // ---- Raster（O：AllWorldRasterWrite::write 0x14083ebe0）----
    if section("Raster").is_some() && !loaded.rasters.is_empty() {
        use crate::resources::world_raster_defs::{
            Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, GardenRaster, PathRaster,
            WaterRaster, WorldRasterResources,
        };
        let present = world.contains_resource::<PathRaster>()
            && world.contains_resource::<Cobblestone1Raster>()
            && world.contains_resource::<Cobblestone2Raster>()
            && world.contains_resource::<Cobblestone3Raster>()
            && world.contains_resource::<WaterRaster>()
            && world.contains_resource::<GardenRaster>();
        if !present {
            report.gaps.push(
                "Raster: one or more named raster resources missing; write skipped".to_string(),
            );
        } else {
            let data: Vec<NamedRasterData> = AllWorldRasterWrite::ordered_names()
                .iter()
                .map(|name| {
                    let bytes = loaded
                        .rasters
                        .iter()
                        .find(|(key, _)| key == name)
                        .map(|(_, bytes)| bytes.clone());
                    ((*name).to_string(), bytes)
                })
                .collect();
            report.raster_slots = data.iter().filter(|(_, bytes)| bytes.is_some()).count();
            let mut state =
                bevy_ecs::system::SystemState::<WorldRasterResources<'static>>::new(world);
            {
                let mut rasters = state.get_mut(world);
                apply_raster_writeback(&mut rasters.write_view(), &data)?;
            }
            state.apply(world);
        }
    } else if let Some(value) = section("Raster") {
        // 空名表 = 该快照确实没有栅格；非空名表却无 chunk = 文件不完整。
        let listed = value.as_array().map_or(true, |names| !names.is_empty());
        if listed {
            report.gaps.push(
                "Raster: section lists names but the file carried no raster chunks; write skipped"
                    .to_string(),
            );
        }
    }

    // B @1417aa796 calls AllWorldRasterWrite::write and then sends the unit
    // upload request @1417aa815..1417aa82c, including an empty raster list.
    // The upload consumer owns transfer to the actual mapped staging buffers.
    if section("Raster").is_some() || !loaded.rasters.is_empty() {
        use crate::resources::world_raster::UploadWorldRasterBuffersToStagingCmd;
        anyhow::ensure!(
            world.contains_resource::<bevy_ecs::event::Events<UploadWorldRasterBuffersToStagingCmd>>(),
            "B Events<UploadWorldRasterBuffersToStagingCmd> producer missing"
        );
        world
            .resource_mut::<bevy_ecs::event::Events<UploadWorldRasterBuffersToStagingCmd>>()
            .send(UploadWorldRasterBuffersToStagingCmd);
    }

    // ---- Terrain 高度缓冲 + 范围 ----
    let height_range = section("TerrainHeightRange")
        .map(|value| serde_json::from_value::<Range<f32>>(value.clone()))
        .transpose()
        .map_err(|error| anyhow::anyhow!("TerrainHeightRange: {error}"))?;
    if !loaded.terrain.is_empty() || height_range.is_some() {
        use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
        if !world.contains_resource::<TerrainHeightsData<TerrainHeightSsbo>>() {
            report.gaps.push(
                "Terrain: TerrainHeightsData<TerrainHeightSsbo> resource missing; heights not written"
                    .to_string(),
            );
        } else {
            if !loaded.terrain.is_empty() {
                let heights: Vec<u16> = loaded
                    .terrain
                    .chunks_exact(2)
                    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                    .collect();
                anyhow::ensure!(
                    loaded.terrain.len() % 2 == 0,
                    "Terrain: odd height buffer length {}",
                    loaded.terrain.len()
                );
                let mut heights_data =
                    world.resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>();
                anyhow::ensure!(
                    heights.len() == heights_data.heights().len(),
                    "Terrain: expected {} u16 heights, got {}",
                    heights_data.heights().len(),
                    heights.len()
                );
                heights_data.heights_mut().copy_from_slice(&heights);
                report.terrain_heights = true;
            }
            if let Some(range) = height_range.clone() {
                world
                    .resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>()
                    .set_height_range(range.start, range.end);
            }
        }
    }

    // ---- Trees ----
    // B @1417aa881..1417aa9f5 sends every captured TreeData through the
    // actual InsertTreeCmd consumer. It owns missing-name resolution, complete
    // entity components, ambience, water sounds and replacement/despawn policy.
    if let Some(value) = section("Trees") {
        use crate::resources::history::tree_edit::InsertTreeCmd;
        let trees = parse_trees_section(value)?;
        if !trees.is_empty() {
            anyhow::ensure!(
                world.contains_resource::<bevy_ecs::event::Events<InsertTreeCmd>>(),
                "B Events<InsertTreeCmd> producer missing"
            );
            let mut requests = world.resource_mut::<bevy_ecs::event::Events<InsertTreeCmd>>();
            for entry in trees {
                requests.send(InsertTreeCmd {
                    id: entry.index.0,
                    position: entry.tree.position,
                    scale: entry.tree.scale,
                    rotation_degrees: entry.tree.rotation_degrees,
                    variant: entry.tree.id,
                    is_preplaced: entry.tree.is_preplaced,
                    prefab_name: if entry.tree.prefab_name.is_empty() {
                        crate::resources::history::tree_edit::missing_prefab()
                    } else {
                        entry.tree.prefab_name
                    },
                });
                report.trees += 1;
            }
        }
    }

    // ---- AutoClutter ----
    // B init_game_state_from_snapshot @1417aaa5e..1417aaaea moves the
    // captured deleted-ID owner (+148..178) over the existing resource. This
    // is replacement, including an empty set, not a union with another session.
    if let Some(value) = section("AutoClutter") {
        let deleted: std::collections::HashSet<u64> = serde_json::from_value(value.clone())
            .map_err(|error| anyhow::anyhow!("AutoClutter: {error}"))?;
        anyhow::ensure!(
            world.contains_resource::<crate::systems::ui_clutter::DeletedAutoClutter>(),
            "B DeletedAutoClutter constructor missing"
        );
        report.auto_clutter = deleted.len();
        world
            .resource_mut::<crate::systems::ui_clutter::DeletedAutoClutter>()
            .0 = deleted;
    }

    // ---- Stairs ----
    // B init_game_state_from_snapshot 0x1417a9c50 calls StairsState::restore_snapshot
    // (0x1417aab6b) unconditionally; restore_snapshot ends with make_backup, which
    // capture_game_snapshot_save_data later asserts via `res.stairs.backup_accurate()`.
    // A file without a "Stairs" section (e.g. the v17 shipped samples) therefore restores
    // an empty snapshot instead of skipping the call.
    {
        let snapshot = match section("Stairs") {
            Some(value) => {
                let section: StairsSectionJson = serde_json::from_value(value.clone())
                    .map_err(|error| anyhow::anyhow!("Stairs section: {error}"))?;
                let snapshot = StairsSnapshot::new(section.edges.clone());
                if section.checksum.is_some() && section.checksum != snapshot.cached_checksum() {
                    report.gaps.push(
                        "Stairs: saved checksum differs from the edge-derived checksum; cached value not preserved"
                            .to_string(),
                    );
                }
                snapshot
            }
            None => StairsSnapshot::new(Vec::new()),
        };
        if world.contains_resource::<StairsState>() {
            apply_stairs_writeback(&mut world.resource_mut::<StairsState>(), snapshot);
            report.stairs = true;
        } else {
            report
                .gaps
                .push("Stairs: StairsState resource missing; graph not restored".to_string());
        }
    }
    Ok(report)
}

/// init 写回：Stairs 快照应用（O：`StairsState::restore_snapshot` 0x1408ff590）。
///
/// 现行 14 节捕获侧会写出 `"Stairs": {"edges": …, "checksum": …}` 与
/// `"StairsRemovedSupports"`（`try_to_section_values`）；17 版发行样本没有这两个节，
/// 此时 `StairsSnapshot` 只来自内存 [`SnapshotSaveData`] 的域描述符。
/// [`apply_loaded_snapshot_to_world`] 解析 JSON 节并调用本函数；纯资源版
/// [`apply_loaded_snapshot`] 不承载 Stairs，由宿主显式接入 `SnapshotSaveData.domain.stairs`。
pub fn apply_stairs_writeback(stairs: &mut StairsState, snapshot: StairsSnapshot) {
    stairs.restore_snapshot(snapshot);
}

/// init 写回：Raster 节应用（O：`AllWorldRasterWrite::write` 0x14083ebe0）。
///
/// `data` = 6 槽命名数据（`NamedRasterData`，名称按 `ordered_names` 校验、长度按
/// 目标 composite 校验，任一不符整批不写）。载荷字节 = flate2 deflate 解压后的
/// 目标 composite 缓冲（`decode_raster_payload` 产出）。
pub fn apply_raster_writeback(
    rasters: &mut AllWorldRasterWrite<'_>,
    data: &[NamedRasterData],
) -> anyhow::Result<()> {
    rasters.write_named(data)
}

/// 快照 Raster 节载荷最佳解码（O：栅格尾 flate2 解压，"decompressing terrain data"
/// @0x142cdc800；**JSON 承载形状待①**）。依次尝试 `Vec<u8>`（数字数组）→ 字符串
/// 字节，再 flate2 deflate 解压；全部失败返回 `None`（跳过该栅格，不伪造载荷）。
/// 经 `write_named` 长度校验后才真正写回。
pub fn decode_raster_payload(value: &Value) -> Option<Vec<u8>> {
    let raw = if let Ok(bytes) = serde_json::from_value::<Vec<u8>>(value.clone()) {
        bytes
    } else if let Some(text) = value.as_str() {
        text.as_bytes().to_vec()
    } else {
        return None;
    };
    inflate_terrain(&raw).ok()
}

/// Walls 语义写回（G1① 契约 §0.1/§4/§7-1）：原版 `InnerWalls::load_state`
/// 0x14085ad80 接收的是 `Vec<(WallId, WallState)>`（stride 0x78 = 8 + 0x70），
/// `WallState` 内的 `Curve2` 点列是**堆缓冲区**，不存在 0x70 固定内联打包。
/// 因此读档直接把反序列化出的 `WallState`（含真实点列）插入权威表。
///
/// 逐字复刻 0x14085ad80 的可观察语义：
/// 1. 前置 3 断言（walls / changes.updated / changes.removed 必须为空）；
/// 2. 空记录 → 直接返回，`max_id` 与各集合保持不变；
/// 3. 逐记录插 `changes.created` 并重建 HashMap（同 id 视为非法输入 → Err，不 panic 不静默覆盖）；
/// 4. `max_id = 记录 id 最大值`。
///
/// crate 侧的显式 id 插入入口是 `InnerWalls::new_wall_with_id` 0x14085a4a0（不推进
/// +0xc0），所以最后显式写 `max_id`。
pub fn load_snapshot_walls(
    walls: &mut crate::resources::walls::InnerWalls,
    records: impl IntoIterator<Item = (WallId, crate::resources::walls::inner_wall_state::WallState)>,
) -> anyhow::Result<usize> {
    anyhow::ensure!(
        walls.all().next().is_none(),
        "assertion failed: self.walls.is_empty()"
    );
    anyhow::ensure!(
        walls.changes().updated.is_empty(),
        "assertion failed: self.changes.updated.is_empty()"
    );
    anyhow::ensure!(
        walls.changes().removed.is_empty(),
        "assertion failed: self.changes.removed.is_empty()"
    );
    let mut count = 0usize;
    let mut max_id = 0u64;
    for (id, state) in records {
        anyhow::ensure!(
            !walls.contains(id),
            "Duplicate wall id: {} in snapshot Walls section",
            id.0
        );
        walls.new_wall_with_id(id, state);
        max_id = max_id.max(id.0);
        count += 1;
    }
    if count != 0 {
        walls.set_max_id(max_id);
    }
    Ok(count)
}

/// Walls 节→0x78B 记录解析（① r7-chain1-field 契约 §2，**真实发行样本权威**）：节载荷 =
/// **JSON 值** `[[WallId, WallState 8 字段], …]`；用原版已恢复的 `WallState` Deserialize
/// （`saved_wall_state.rs`，即 B 0x1415bb1c0 object visitor）反序列化，再投影到 0x78B
/// 记录（{WallId u64@0, WallState 0x70B@8}）。
///
/// **读档不使用本函数**（G1① 契约 §4：0x70 固定记录装不下点列，且
/// `from_record_bytes` 只保留 length）。生产路径走 [`load_snapshot_walls`] 的语义插入；
/// 本函数保留给仍接收 0x78B 字节记录的外部调用者。
///
/// 已证偏移（`WallState::from_record_bytes` 解码面，LOGIC3 §0）全部写入：
/// `+0x34` curve.length / `+0x38` shape 28B / `+0x54` height / `+0x58` terrain_floor /
/// `+0x5c` arch scalar / `+0x60` plaster bits / `+0x64` arch_ty / `+0x65` structure_ty /
/// `+0x66` freehand_ty / `+0x67` removed_supports / `+0x68` style / `+0x69` metadata。
///
/// **0x78B 字节投影的固有损失**（G1① §4）：`+0x00..0x33` 是 `Curve2` 点列的
/// `Vec` 头（堆外缓冲），不是内联打包区；`WallState::from_record_bytes` 只保留
/// `+0x34` length，点列留空。因此本函数写出的记录**不含点列**——读档不经过这里，
/// 见 [`load_snapshot_walls`]。
pub fn parse_wall_section(
    json: &serde_json::Value,
) -> Vec<crate::resources::walls::inner_walls::InnerWallStateRecord> {
    use crate::resources::walls::inner_wall_state::WallState;
    let Ok(walls) = serde_json::from_value::<Vec<(u64, WallState)>>(json.clone()) else {
        log::error!("parse_wall_section: canonical WallState serde rejected the Walls section");
        return Vec::new();
    };
    let mut records = Vec::with_capacity(walls.len());
    for (id, state) in walls {
        records.push(encode_wall_record(id, &state));
    }
    log::trace!(
        "parse_wall_section: {} wall records（固定字段全写；0x70 记录不含点列，读档请用 load_snapshot_walls）",
        records.len()
    );
    records
}

/// `WallState` → 0x78B 记录（`InnerWalls::load_state` 0x14085ad80 的输入投影）。
/// 偏移即 `WallState::from_record_bytes` 的读面；record 基址 +8 = WallState +0。
pub fn encode_wall_record(
    id: u64,
    state: &crate::resources::walls::inner_wall_state::WallState,
) -> crate::resources::walls::inner_walls::InnerWallStateRecord {
    let source = state.source();
    let metadata = &state.snapshot().metadata;
    let mut bytes = [0u8; 0x78];
    bytes[..8].copy_from_slice(&id.to_le_bytes());
    let put_f32 = |bytes: &mut [u8; 0x78], offset: usize, value: f32| {
        bytes[8 + offset..8 + offset + 4].copy_from_slice(&value.to_le_bytes());
    };
    // +0x34 Curve2 length。+0x00..0x33 是点列 Vec 头（堆外缓冲），字节记录无法携带。
    put_f32(&mut bytes, 0x34, source.curve.length);
    // +0x38 shape 0x1cB。
    bytes[8 + 0x38..8 + 0x54].copy_from_slice(source.shape.bytes());
    // +0x54 height / +0x58 terrain_floor / +0x5c arch scalar。
    put_f32(&mut bytes, 0x54, source.extra);
    put_f32(&mut bytes, 0x58, source.terrain_floor);
    put_f32(&mut bytes, 0x5c, source.observed_path_scalar);
    // +0x60 plaster bits（u32）。
    bytes[8 + 0x60..8 + 0x64].copy_from_slice(&metadata.source_60_bits.to_le_bytes());
    // +0x64 arch_ty / +0x65 structure_ty / +0x66 freehand / +0x67 supports /
    // +0x68 style / +0x69 metadata source_69。
    bytes[8 + 0x64] = source.observed_source_flag_64;
    bytes[8 + 0x65] = source.clutter_version;
    bytes[8 + 0x66] = metadata.source_66.unwrap_or(0);
    bytes[8 + 0x67] = metadata.source_67;
    bytes[8 + 0x68] = source.style;
    bytes[8 + 0x69] = metadata.source_69;
    crate::resources::walls::inner_walls::InnerWallStateRecord(bytes)
}

/// 原版 `_UNK_1429cd080` / `_UNK_1429cd090` 的 32 字节实值（G1① §5.4，rne-read O）：
/// 8B 词内字节反转 + 词序交换后的 `"somepseudorandomlygeneratedbytes"`，即 aHash
/// 固定种子。用途（`dec-0x140a20d70`）：装入两个 XMM → 与 `map+0x20` 的 16B key
/// 异或 → aHash 风格混合 → `FUN_140b776d0(map, hash, &id)`。
///
/// **不是 ID XOR 掩码**：restore 中 `DecoratorId` 以 u32 明文读出直接使用
/// （`uVar9 = *puVar27`），没有 rotate/XOR 解码步骤。旧 `SNAPSHOT_ID_MASK=0` +
/// `encode/decode_snapshot_id`（rotate_right(32)）为未证实实现，已从读档路径删除。
pub const DECORATOR_HASHER_SEEDS: [u64; 4] = [
    0x736f6d6570736575, // "somepseu"
    0x6c7967656e657261, // "lygenera"
    0x646f72616e646f6d, // "dorandom"
    0x7465646279746573, // "tedbytes"
];

/// Decorators 外层桶条目大小（G1① §5.1，O）：
/// `{storage_disc u64@0, storage_payload u64@8, Vec 头 @0x10/0x18/0x20}`。
pub const DECORATOR_OUTER_RECORD_SIZE: usize = 0x28;
/// Decorators 内层条目大小（G1① §5.1，O）：
/// `{DecoratorId u32@0, pad4, DecoratorHistoryState 0x50@+8}`。**不是 0x16B**。
pub const DECORATOR_INNER_RECORD_SIZE: usize = 0x58;

/// 子侧状态字节公式 `(b-2)<=2 ? b-1 : 0`（① r6-decorators §1；u8 回绕）。
/// 与记录形状无关的独立标量转换，保留供域调用者使用。
pub fn decorator_side_from_state(b: u8) -> u8 {
    if 2 < b.wrapping_sub(2) {
        0
    } else {
        b.wrapping_sub(1)
    }
}

/// 快照 Terrain 节 serde（① r7-chain1-field 契约 §3，**真实发行样本权威**）：
/// `{next_stroke_index, strokes: {id 字符串: {falloff, height_ws, index, points}}}`。
/// **纠偏**：strokes 为 **JSON 明文**（非 flate2 deflate；flate2 只用于二进制栅格尾）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct TerrainSectionJson {
    // 原1415c64fe要求next_stroke_id；17字节旧名是同一字段的合法别名。
    #[serde(rename = "next_stroke_id", alias = "next_stroke_index")]
    pub next_stroke_index: u64,
    // 直接复用canonical StrokeId的数值serde，错误必须在写回World前传播，不能丢掉非法键。
    pub strokes: std::collections::HashMap<crate::resources::terrain_editing::StrokeId, TerrainStrokeJson>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TerrainStrokeJson {
    pub falloff: f32,
    pub height_ws: f32,
    pub index: u32,
    pub points: Vec<[f32; 3]>,
}

/// 快照 Decorators 节 addr 判别（r7-chain1-writeback §1 + G1① §5.1）：
/// `{"Wall": <id>}` / `"Terrain"` / `"Stairs"`（真实样本 5/36 文件、9 条，写出侧
/// `decorator_storage_addr_json` 同形）/ `{"Stair": <id>}`（旧对象判别）。
#[derive(Clone, Debug)]
pub enum DecoratorAddrJson {
    Wall {
        Wall: u64,
    },
    Terrain,
    /// 单元变体串 `"Stairs"`（写出侧与真实样本的现行形状）。
    Stairs,
    Stair {
        Stair: u64,
    },
}

/// 手写 `Deserialize`（样本契约 `{"Wall": <id>}` / `"Terrain"` / `"Stairs"` /
/// `{"Stair": <id>}`）：`#[serde(untagged)]` derive 不能把字符串解到单元变体
/// （untagged 单元变体只接受 `null`），故按判别形状手写 visitor。
impl<'de> serde::Deserialize<'de> for DecoratorAddrJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AddrVisitor;
        impl<'de> serde::de::Visitor<'de> for AddrVisitor {
            type Value = DecoratorAddrJson;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "a snapshot decorator addr: {\"Wall\": <id>}, \"Terrain\", \"Stairs\", or {\"Stair\": <id>}",
                )
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Terrain" => Ok(DecoratorAddrJson::Terrain),
                    "Stairs" => Ok(DecoratorAddrJson::Stairs),
                    _ => Err(serde::de::Error::unknown_variant(
                        value,
                        &["Terrain", "Stairs", "Wall", "Stair"],
                    )),
                }
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let Some(key) = map.next_key::<String>()? else {
                    return Err(serde::de::Error::custom(
                        "expected one addr key in DecoratorAddrJson",
                    ));
                };
                match key.as_str() {
                    "Wall" => Ok(DecoratorAddrJson::Wall {
                        Wall: map.next_value()?,
                    }),
                    "Stair" => Ok(DecoratorAddrJson::Stair {
                        Stair: map.next_value()?,
                    }),
                    _ => Err(serde::de::Error::unknown_variant(
                        key.as_str(),
                        &["Wall", "Stair"],
                    )),
                }
            }
        }
        deserializer.deserialize_any(AddrVisitor)
    }
}

impl DecoratorAddrJson {
    /// 判别 → crate 权威存储地址（`DecoratorStorageAddr`：disc 0=Wall/1=Terrain/
    /// 2=Stair，payload=WallId）。`"Stairs"` 串与 `{"Stair": id}` 都是 Stair 桶；
    /// 多楼梯地址的细分待①。
    pub fn to_storage_addr(&self) -> DecoratorStorageAddr {
        match self {
            DecoratorAddrJson::Wall { Wall } => DecoratorStorageAddr::Wall(WallId(*Wall)),
            DecoratorAddrJson::Terrain => DecoratorStorageAddr::Terrain,
            DecoratorAddrJson::Stairs => DecoratorStorageAddr::Stair,
            DecoratorAddrJson::Stair { Stair: _ } => DecoratorStorageAddr::Stair,
        }
    }
}

/// 快照 Decorators 节 DecoratorInfo（① r7-chain1-writeback §1，真实样本字段；
/// G1① §5.2 逐偏移复核定案）。
///
/// 字段与 `decorator_history_state_json`（本文件，B 侧面序列化投影）逐项互逆：
/// 仅在非默认值时写出的字段在这里是 `Option`/默认值；`h_offset`/`lantern_brightness`/
/// `stair_width` 以 `Value` 承载，以区分「缺省」与原生 NaN 被 serde_json 写成 `null`
/// 的情形（原生 `f32::NAN` 与默认值比较为不等 → 写出 null）。`seed` 恒写出并落在
/// state +0x2c（不是桶键）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorInfoJson {
    #[serde(default = "default_true")]
    pub allow_connection: bool,
    #[serde(default)]
    pub clv: u32,
    #[serde(default)]
    pub color_id: u32,
    pub dst: DecoratorDstJson,
    #[serde(default)]
    pub rank: DecoratorRankJson,
    #[serde(default)]
    pub seed: u32,
    /// 缺省 1.0；`null` = 原生 NaN。
    #[serde(default, deserialize_with = "deserialize_present_json_value")]
    pub stair_width: Option<Value>,
    pub ty: DecoratorTypeJson,
    #[serde(default)]
    pub window_light_color: u32,
    #[serde(default)]
    pub ladder_color: u32,
    #[serde(default)]
    pub magic_color: u32,
    /// 缺省 0.0；`null` = 原生 NaN。真实样本另写 `chimney_h_offset`（47 条，
    /// G1① §5.5）：原生槽位 U（36 样本恒 0.0），先按 +0x30 别名接收，不丢字段。
    #[serde(
        default,
        alias = "chimney_h_offset",
        deserialize_with = "deserialize_present_json_value"
    )]
    pub h_offset: Option<Value>,
    /// 缺省 1.0；`null` = 原生 NaN。
    #[serde(default, deserialize_with = "deserialize_present_json_value")]
    pub lantern_brightness: Option<Value>,
    #[serde(default = "default_true")]
    pub chimney_smoke: bool,
    #[serde(default)]
    pub railing: Option<String>,
    #[serde(default)]
    pub stairs_supports: Option<String>,
}

fn default_true() -> bool {
    true
}

/// A missing field uses serde's default None. A present JSON null must remain
/// Some(Null): original f32 NaN serializes to null and is not the field default.
/// Option<Value>'s standard deserializer otherwise collapses these two cases.
fn deserialize_present_json_value<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Value>, D::Error> {
    <Value as serde::Deserialize>::deserialize(deserializer).map(Some)
}

/// JSON f32 字段（缺省 → default；`null` → 原生 NaN）。
fn json_f32(value: Option<&Value>, default: f32) -> f32 {
    match value {
        None => default,
        Some(Value::Null) => f32::NAN,
        Some(value) => value
            .as_f64()
            .map(|number| number as f32)
            .unwrap_or(default),
    }
}

/// DecoratorType JSON：`{"Clutter":{"ty":"plant_pot_v1"}}` 或单元变体串。
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DecoratorTypeJson {
    Clutter { Clutter: DecoratorClutterJson },
    Name(String),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorClutterJson {
    pub ty: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DecoratorDstJson {
    Wall { Wall: DecoratorWallDstJson },
    Roof { Roof: DecoratorRoofDstJson },
    Terrain { Terrain: DecoratorTerrainDstJson },
    Stair { Stair: DecoratorStairDstJson },
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorWallDstJson {
    pub anchor: DecoratorAnchorJson,
    pub coord: DecoratorCoordJson,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorAnchorJson {
    #[serde(default)]
    pub anchor_ws: [f32; 3],
    #[serde(default)]
    pub corner: u32,
    #[serde(default)]
    pub platform: u64,
    #[serde(default)]
    pub wall_height_relative_y: f32,
    #[serde(default)]
    pub is_bottom_door: bool,
    /// 写出侧仅在 `+0x22 == 0` 时写 `"Inner"`；其余（含缺省）= tag 1。
    #[serde(default)]
    pub side: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorCoordJson {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorRoofDstJson {
    pub mesh_uv: [f32; 2],
    #[serde(default)]
    pub relative_yaw_radians: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorTerrainDstJson {
    pub pos_xz: [f32; 2],
    #[serde(default)]
    pub yaw_radians: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorStairDstJson {
    pub graph_edge: [u32; 2],
    pub uv: [f32; 2],
    #[serde(default)]
    pub yaw_radians: f32,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct DecoratorRankJson {
    #[serde(default)]
    pub x: u32,
}

/// 快照 Decorators 节（① r7-chain1-writeback §1）：`{states: [[addr 判别,
/// {"state": [ {id, DecoratorInfo}, ... ]}], ...]}`。
///
/// G1① §5.1（O）修正的记录模型：外层桶条目 **0x28B**（disc/payload/桶 Vec 头），
/// 内层条目 **0x58B** = `{DecoratorId u32 明文 @+0, pad4, DecoratorHistoryState 0x50 @+8}`。
/// 这里转换出的 `DecoratorAddrSnapshot.states` 即 `(DecoratorId, DecoratorHistoryState)`，
/// 由 `restore_snapshot_with_id_gen` 消费。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorsSectionJson {
    pub states: Vec<(DecoratorAddrJson, DecoratorStateListJson)>,
}

fn pack_f32_pair(a: f32, b: f32) -> u64 {
    u64::from(a.to_bits()) | (u64::from(b.to_bits()) << 32)
}

fn decorator_ty_tag(ty: &DecoratorTypeJson) -> anyhow::Result<u8> {
    Ok(match ty {
        DecoratorTypeJson::Name(name) => match name.as_str() {
            "CottageWindow" => 56,
            "GothicWindow" => 57,
            "ArrowSlit" => 58,
            "Chimney" => 59,
            "Lantern" => 60,
            "Flag" => 61,
            "Stairs" => 63,
            other => anyhow::bail!("unknown DecoratorType {other}"),
        },
        DecoratorTypeJson::Clutter { Clutter } => CLUTTER_MESH_NAMES
            .iter()
            .position(|name| *name == Clutter.ty)
            .ok_or_else(|| anyhow::anyhow!("unknown clutter mesh {}", Clutter.ty))?
            as u8,
    })
}

/// `DecoratorDstJson` → 原生 dst 字段（`decorator_history_state_json` 的逐字节互逆）。
///
/// 返回 `(tag, dst_wall, dst_secondary, dst_payload, u18, u20)`；tag 写在 u20 的
/// byte2（原生 `DecoratorInfo.dst_tag`）。
#[allow(clippy::type_complexity)]
fn decorator_dst_fields(dst: &DecoratorDstJson) -> (u8, u64, u64, u64, u64, u32) {
    match dst {
        DecoratorDstJson::Wall { Wall } => {
            // 写出侧：coord.x=f(+0)、coord.y=f(+4)；anchor wall_height_relative_y=f(+0x10)；
            // anchor_ws=f(+0x14/+0x18/+0x1c)；corner=b(+0x20)、is_bottom_door=b(+0x21)、
            // side 仅 tag 0 时写 "Inner"。
            let tag = if Wall.anchor.side.as_deref() == Some("Inner") {
                0
            } else {
                1
            };
            let dst_wall = pack_f32_pair(Wall.coord.x, Wall.coord.y);
            let dst_payload = u64::from(Wall.anchor.wall_height_relative_y.to_bits())
                | (u64::from(Wall.anchor.anchor_ws[0].to_bits()) << 32);
            let u18 = u64::from(Wall.anchor.anchor_ws[1].to_bits())
                | (u64::from(Wall.anchor.anchor_ws[2].to_bits()) << 32);
            let u20 = Wall.anchor.corner
                | (u32::from(Wall.anchor.is_bottom_door) << 8)
                | (u32::from(tag) << 16);
            (tag, dst_wall, Wall.anchor.platform, dst_payload, u18, u20)
        }
        DecoratorDstJson::Roof { Roof: roof } => (
            2,
            pack_f32_pair(roof.mesh_uv[0], roof.mesh_uv[1]),
            u64::from(roof.relative_yaw_radians.to_bits()),
            0,
            0,
            2 << 16,
        ),
        DecoratorDstJson::Terrain { Terrain } => (
            3,
            pack_f32_pair(Terrain.pos_xz[0], Terrain.pos_xz[1]),
            u64::from(Terrain.yaw_radians.to_bits()),
            0,
            0,
            3 << 16,
        ),
        DecoratorDstJson::Stair { Stair: stair } => (
            4,
            u64::from(stair.graph_edge[0]) | (u64::from(stair.graph_edge[1]) << 32),
            pack_f32_pair(stair.uv[0], stair.uv[1]),
            u64::from(stair.yaw_radians.to_bits()),
            0,
            4 << 16,
        ),
    }
}

impl DecoratorInfoJson {
    /// `DecoratorInfo` JSON 投影 → 原生 `DecoratorHistoryState`（0x50B）。
    ///
    /// 映射是 `decorator_history_state_json`（B 0x140a20d70 take_snapshot 的 serde 投影）
    /// 的逐字段逆。G1① §5.2 定案：`seed` 写 **+0x2c**，与桶键（pair 第一元素）不同
    /// ——36 样本中 5 条 `seed != key`，故这里用 JSON `seed`，桶键只作 map key /
    /// `DecoratorIdGen.reserve`。
    /// **登记 U**：+0x24（u24）、非 Wall dst 的未写出高位、以及 `style` 与
    /// `allow_connection`/`chimney_smoke`/`stairs_supports` 在原字节上复用同一槽
    /// （Flag/Chimney 变体载荷与布尔门同字节）——wire 无法区分，按写出侧默认还原。
    pub fn to_history_state(
        &self,
    ) -> anyhow::Result<crate::resources::history::decorator_edit::DecoratorHistoryState> {
        use crate::resources::history::decorator_edit::DecoratorHistoryState;
        let ty = decorator_ty_tag(&self.ty)?;
        let (_tag, dst_wall, dst_secondary, dst_payload, u18, u20) =
            decorator_dst_fields(&self.dst);
        let railing = match self.railing.as_deref() {
            None => 0,
            Some("Low") => 1,
            Some("High") => 2,
            Some("Wooden") => 3,
            Some(other) => anyhow::bail!("unknown RailingType {other}"),
        };
        Ok(DecoratorHistoryState {
            dst_wall,
            dst_secondary,
            dst_payload,
            u18,
            u20,
            u24: 0,
            // +0x28 color_id / +0x2c seed（G1① §5.2；桶键不参与）。
            u28: u64::from(self.color_id) | (u64::from(self.seed) << 32),
            // +0x30 h_offset（style 4/6/7/8+/<2 变体簇共用）。
            u30: json_f32(self.h_offset.as_ref(), 0.0).to_bits(),
            // +0x34 window_light_color（Window 变体簇）。
            u34: self.window_light_color,
            // +0x38 lantern_brightness（Lantern 变体簇；缺省 1.0）。
            u38: json_f32(self.lantern_brightness.as_ref(), 1.0).to_bits(),
            // +0x3c stair_width（缺省 1.0）。
            u3c: json_f32(self.stair_width.as_ref(), 1.0).to_bits(),
            // +0x40 ladder_color / +0x44 magic_color（默认簇）。
            u40: self.ladder_color,
            u44: self.magic_color,
            rank: self.rank.x as u8,
            u49: self.clv as u8,
            ty,
            u4b: railing,
            u4c: u8::from(self.allow_connection),
            u4d: u8::from(self.chimney_smoke),
            // +0x4e style（写出侧 `stairs_supports != 0` 即写出 "None"）。
            style: u8::from(self.stairs_supports.is_some()),
        })
    }
}

impl DecoratorsSectionJson {
    /// 每地址的装饰状态列表 → crate `DecoratorStorageSnapshot` 快照条目。
    /// 每 `{id, DecoratorInfo}` 条目 → `DecoratorAddrSnapshot`（id + 原生 0x50B 状态）；
    /// restore_snapshot（0x140a20d70）再经 `DecoratorHistoryState::to_info` 重建
    /// `DecoratorInfo` 并写回桶。
    pub fn into_storage_snapshot(&self) -> anyhow::Result<DecoratorStorageSnapshot> {
        use crate::resources::walls::decorator::DecoratorId;
        let mut states = Vec::with_capacity(self.states.len());
        for (addr, list) in &self.states {
            let mut entries = Vec::with_capacity(list.state.len());
            for (id, info) in &list.state {
                entries.push((DecoratorId(*id), info.to_history_state()?));
            }
            states.push((
                addr.to_storage_addr(),
                crate::resources::walls::decorator_storage::DecoratorAddrSnapshot {
                    states: entries,
                },
            ));
        }
        Ok(DecoratorStorageSnapshot { states })
    }
}

/// 每地址的装饰状态列表 `{"state": [...]}`。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorStateListJson {
    #[serde(rename = "state")]
    pub state: Vec<(u32, DecoratorInfoJson)>,
}

impl TerrainSectionJson {
    /// 转 crate `TerrainEditingData`（tg_terrain：id 串 → StrokeId，next_stroke_index →
    /// next_index）。
    pub fn into_editing_data(self) -> crate::resources::terrain_editing::TerrainEditingData {
        use crate::resources::terrain_editing::{StrokeId, TerrainStroke};
        let mut strokes = std::collections::HashMap::new();
        for (id, stroke) in self.strokes {
            strokes.insert(
                id,
                TerrainStroke {
                    points: stroke.points,
                    falloff: stroke.falloff,
                    index: StrokeId(stroke.index),
                    height_ws: stroke.height_ws,
                },
            );
        }
        crate::resources::terrain_editing::TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData {
                // Loaded strokes do not retain a live UI drag's cursor plane.
                cursor_height_override: None,
                strokes,
                next_index: self.next_stroke_index,
                // Loaded strokes have not yet reached this world's GPU raster.
                rasterized: false,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
    use crate::resources::walls::inner_walls::InnerWalls;
    use serde_json::json;

    #[test]
    fn try_chunk_wraps_an_existing_result_three_ways() {
        // Some(Ok(chunk)) → chunk（原生 tag==1 + ptr!=0 路径）。
        assert_eq!(try_chunk(Some(Ok(vec![1u8, 2, 3]))).unwrap(), vec![1, 2, 3]);
        // Some(Err(io)) → anyhow::Error::from<io::Error>（CALL 0x1428a9590）。
        let error = try_chunk(Some(Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "chunk",
        ))))
        .unwrap_err();
        assert!(error.to_string().contains("chunk"));
        // None → "No chunks left in iterator"（串 0x142cdc635 len 0x1a）。
        assert_eq!(
            try_chunk(None).unwrap_err().to_string(),
            "No chunks left in iterator"
        );
    }

    #[test]
    fn capture_game_snapshot_serializes_sections_in_native_order() {
        let domain = SnapshotDomainData {
            walls: Vec::new(),
            roofs: Vec::new(),
            decorators: Vec::new(),
            wall_colors: Vec::new(),
            gate_colors: Vec::new(),
            halftimber_colors: Vec::new(),
            terrain: TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()),
            trees: vec![SnapshotTreesEntry {
                index: crate::resources::tree_storage::TreeStorageIndex(1),
                tree: crate::resources::history::tree_edit::TreeEditData::default(),
            }],
            stairs: StairsSnapshot::new(Vec::new()),
            raster: Vec::new(),
            ids: Vec::new(),
            // 注入一个 Version 节（域序列化器未就绪的节由宿主注入）。
            section_values: Vec::new(),
            terrain_data: Some(vec![0, 1, 2, 3]),
            terrain_height_range: Some(-2.0..4.0),
            removed_supports: Some(StairsRemovedSupports::default()),
        };
        let bytes = encode_game_snapshot(&domain).unwrap();
        assert!(bytes.len() <= SNAPSHOT_JSON_BUFFER_CAPACITY);
        let first = motomoto::Reader::new(&bytes)
            .iter()
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(first.tag, motomoto::tag_from_str("Snapshot"));
        let parsed: Value = serde_json::from_slice(first.data).unwrap();
        let object = parsed.as_object().unwrap();
        // 原生经 serde_json::Map<String, Value>::insert 组装后 serialize；本 crate 的
        // serde_json 未启用 preserve_order，Map = BTreeMap → 键按字典序输出
        // （"Trees" < "Version"）。JSON 对象语义与键序无关；**解析**节序仍是
        // Version → … → Raster（见 validate）。
        assert!(object.contains_key("Version"));
        assert!(object.contains_key("Trees"));
        assert_eq!(object["Version"], json!(2));
        assert_eq!(object["Trees"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn validate_parses_version_typed_and_ignores_unknown_raster_keys() {
        let payload = deflate_terrain(&[1, 2, 3]).unwrap();
        // 真实样本形态：Raster = 在场栅格名数组；兼容对象载荷形状仍解压。
        let json = json!({
            "Version": SNAPSHOT_FORMAT_VERSION,
            "Walls": [],
            "Decorators": {"states": []},
            "Terrain": {"next_stroke_index": 0, "strokes": {}},
            "Trees": [],
            "Raster": ["NotARaster", "PathRaster"]
        });
        let loaded =
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&json).unwrap()).unwrap();
        assert_eq!(loaded.version, 2);
        assert!(loaded.rasters.is_empty(), "name list carries no payload");
        let object_form = json!({
            "Version": SNAPSHOT_FORMAT_VERSION,
            "Raster": {"NotARaster": [], "PathRaster": payload}
        });
        let loaded =
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&object_form).unwrap())
                .unwrap();
        assert_eq!(loaded.rasters.len(), 1);
        assert_eq!(loaded.rasters[0].0, "PathRaster");
        assert_eq!(loaded.rasters[0].1, vec![1, 2, 3]);
        // Version 非 i32 → typed 解析失败（非宽松 as_i64）。
        let bad = json!({"Version": "27"});
        assert!(
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&bad).unwrap()).is_err()
        );
        // 版本不符 → "Version mismatch; expected , got " 语义（原 panic 面）。
        let mismatch = json!({"Version": 27});
        let error =
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&mismatch).unwrap())
                .unwrap_err();
        assert!(error.to_string().contains("Version mismatch"));
    }

    #[test]
    fn snapshot_prefix_reads_header_and_fourteen_descriptors() {
        let mut raw = vec![0u8; SNAPSHOT_PREFIX_SIZE];
        raw[..SNAPSHOT_HEADER_SIZE].fill(0xAB);
        for index in 0..SNAPSHOT_DOMAIN_COUNT {
            let base = SNAPSHOT_HEADER_SIZE + index * SNAPSHOT_DOMAIN_STRIDE;
            raw[base..base + 8].copy_from_slice(&(index as u64).to_le_bytes());
            raw[base + 0x10..base + 0x18].copy_from_slice(&0x1000u64.to_le_bytes());
            raw[base + 0x1c..base + 0x20].copy_from_slice(&(index as u32).to_le_bytes());
        }
        let prefix = SnapshotPrefix::from_bytes(&raw).unwrap();
        assert_eq!(prefix.header[0], 0xAB);
        assert_eq!(SNAPSHOT_DOMAIN_COUNT, 14);
        assert_eq!(prefix.domains[0].data, 0);
        assert_eq!(prefix.domains[13].data, 13);
        assert_eq!(prefix.domains[13].target, 0x1000);
        assert_eq!(prefix.domains[13].count, 13);
        // 过短 → Err（不越界）。
        assert!(SnapshotPrefix::from_bytes(&raw[..SNAPSHOT_PREFIX_SIZE - 1]).is_err());
    }

    #[test]
    fn decorator_hasher_seeds_are_the_shipped_a_hash_words() {
        // G1① §5.4：内存字节按大端读作 ASCII 时拼出 "somepseu|lygenera|
        // dorandom|tedbytes"（即 0x1429cd080/090 的 32B 常量）。
        let words = DECORATOR_HASHER_SEEDS;
        assert_eq!(u64::from_be_bytes(*b"somepseu"), words[0]);
        assert_eq!(u64::from_be_bytes(*b"lygenera"), words[1]);
        assert_eq!(u64::from_be_bytes(*b"dorandom"), words[2]);
        assert_eq!(u64::from_be_bytes(*b"tedbytes"), words[3]);
        assert_eq!(words[0].to_le_bytes(), *b"uespemos");
        assert_eq!(DECORATOR_OUTER_RECORD_SIZE, 0x28);
        assert_eq!(DECORATOR_INNER_RECORD_SIZE, 0x58);
    }

    #[test]
    fn is_known_raster_key_accepts_the_six_ordered_names() {
        assert!(is_known_raster_key("PathRaster"));
        assert!(is_known_raster_key("Cobblestone1Raster"));
        assert!(is_known_raster_key("Cobblestone3Raster"));
        assert!(is_known_raster_key("WaterRaster"));
        assert!(is_known_raster_key("GardenRaster"));
        assert!(!is_known_raster_key("UnkownRaster"));
        assert!(!is_known_raster_key(""));
    }

    #[test]
    fn sorted_applies_the_native_snapshot_sort_orders() {
        // roofs 按所属 WallId（closure_env_1）。
        fn roof(wall: u64) -> Roof {
            let mut bytes = [0u8; 88];
            bytes[..8].copy_from_slice(&wall.to_le_bytes());
            Roof::from_observed(bytes)
        }
        // walls 按 WallId.0（closure_env_0）——WallState 构造面复杂（from_snapshot
        // 需 RawWallPublicationSource），排序键即 id.0，等价性由
        // `sorted()` 的 `sort_unstable_by_key(|(id, _)| id.0)` 直接承载。
        let walls = Vec::new();
        let roofs = vec![roof(9), roof(4), roof(9)];
        let decorators = vec![
            (
                DecoratorStorageAddr::Wall(WallId(2)),
                DecoratorAddrSnapshot { states: Vec::new() },
            ),
            (
                DecoratorStorageAddr::Terrain,
                DecoratorAddrSnapshot { states: Vec::new() },
            ),
            (
                DecoratorStorageAddr::Stair,
                DecoratorAddrSnapshot { states: Vec::new() },
            ),
            (
                DecoratorStorageAddr::Wall(WallId(1)),
                DecoratorAddrSnapshot { states: Vec::new() },
            ),
        ];
        let trees = vec![
            SnapshotTreesEntry {
                index: crate::resources::tree_storage::TreeStorageIndex(20),
                tree: crate::resources::history::tree_edit::TreeEditData::default(),
            },
            SnapshotTreesEntry {
                index: crate::resources::tree_storage::TreeStorageIndex(3),
                tree: crate::resources::history::tree_edit::TreeEditData::default(),
            },
        ];
        let ids = vec![9u64, 2, 7, 1];
        let domain = SnapshotDomainData {
            walls,
            roofs,
            decorators,
            wall_colors: Vec::new(),
            gate_colors: Vec::new(),
            halftimber_colors: Vec::new(),
            terrain: TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()),
            trees,
            stairs: StairsSnapshot::new(Vec::new()),
            raster: Vec::new(),
            ids,
            section_values: Vec::new(),
            terrain_data: Some(Vec::new()),
            terrain_height_range: Some(0.0..0.0),
            removed_supports: Some(StairsRemovedSupports::default()),
        }
        .sorted();
        assert_eq!(
            domain
                .roofs
                .iter()
                .map(|r| r.wall_id().0)
                .collect::<Vec<_>>(),
            [4, 9, 9]
        );
        assert_eq!(
            domain.decorators[0].0,
            DecoratorStorageAddr::Wall(WallId(1))
        );
        assert_eq!(
            domain.decorators[1].0,
            DecoratorStorageAddr::Wall(WallId(2))
        );
        assert_eq!(domain.decorators[2].0, DecoratorStorageAddr::Terrain);
        assert_eq!(domain.decorators[3].0, DecoratorStorageAddr::Stair);
        assert_eq!(
            domain.trees.iter().map(|t| t.index.0).collect::<Vec<_>>(),
            [3, 20]
        );
        assert_eq!(domain.ids, [1, 2, 7, 9]);
    }

    #[test]
    fn decorator_section_states_convert_to_native_history_states() {
        // 真实样本：空 states 合法（inspect states = [["Terrain", {"state": []}]]）。
        let section: DecoratorsSectionJson =
            serde_json::from_value(json!({"states": [["Terrain", {"state": []}]]})).unwrap();
        let snapshot = section.into_storage_snapshot().unwrap();
        assert_eq!(
            snapshot.states,
            vec![(
                DecoratorStorageAddr::Terrain,
                crate::resources::walls::decorator_storage::DecoratorAddrSnapshot {
                    states: Vec::new()
                }
            )]
        );
        // 非空条目：JSON → DecoratorHistoryState → 写出侧 JSON 应保持字段一致。
        let entry = json!({
            "allow_connection": false, "clv": 3, "color_id": 1,
            "dst": {"Terrain": {"pos_xz": [1.0, 2.0], "yaw_radians": 0.5}},
            "rank": {"x": 2}, "seed": 9, "stair_width": 1.5, "ty": "Stairs",
            "stairs_supports": "None"
        });
        let section: DecoratorsSectionJson =
            serde_json::from_value(json!({"states": [["Terrain", {"state": [[9, entry]]}]]}))
                .unwrap();
        let snapshot = section.into_storage_snapshot().unwrap();
        let (id, state) = &snapshot.states[0].1.states[0];
        assert_eq!(id.0, 9);
        assert_eq!(state.ty, 63);
        assert_eq!(state.rank, 2);
        assert_eq!(state.u49, 3);
        assert_eq!(((state.u20 >> 16) & 0xff) as u8, 3, "Terrain dst tag");
        assert_eq!(state.u28 & 0xffff_ffff, 1, "color_id");
        assert_eq!(state.u28 >> 32, 9, "decorator id");
        assert_eq!(f32::from_bits(state.u3c), 1.5, "stair_width");
        assert_eq!(state.u4c, 0, "allow_connection=false");
        assert_eq!(state.style, 1, "stairs_supports=None");
        // 回写侧投影一致（decorator_history_state_json 的逆验证）。
        let projected = decorator_history_state_json(state).unwrap();
        assert_eq!(projected["ty"], json!("Stairs"));
        assert_eq!(projected["rank"], json!({"x": 2}));
        assert_eq!(projected["seed"], json!(9));
        assert_eq!(projected["color_id"], json!(1));
        assert_eq!(projected["allow_connection"], json!(false));
        assert_eq!(projected["stairs_supports"], json!("None"));
        assert_eq!(projected["stair_width"], json!(1.5));
    }

    #[test]
    fn parse_wall_section_builds_all_proven_record_fields() {
        // 真实样本字段名/取值（r7-chain1-field §2）+ 显式非默认枚举/字节字段，
        // 用于锁定 from_record_bytes 已证偏移（+0x60/+0x64/+0x65/+0x66/+0x67/+0x69）。
        let json = json!([[7, {
            "arch_height": 0.5, "arch_ty": "ConstantMaxHeight",
            "curve": {"length": 10.0, "points": [[0.0,0.0,2.0],[3.0,0.0,5.0]],
                      "points_u": [0.0,1.0]},
            "elevation": 0.2, "has_skirts": true, "height": 4.0,
            "is_enclosed": "No", "structure_ty": "Fence",
            "clutter_version": 4, "freehand_ty": "ConstantMaxHeight",
            "plaster_peel": 0.25, "removed_supports": true
        }]]);
        let records = parse_wall_section(&json);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].wall_id(), WallId(7));
        assert_eq!(records[0].height(), 4.0);
        assert_eq!(records[0].removed_supports(), 1);
        let state = records[0].wall_state();
        assert_eq!(state.elevation().to_bits(), 0.2f32.to_bits());
        assert_eq!(state.arch_height().to_bits(), 0.5f32.to_bits());
        assert_eq!(state.style(), 1, "has_skirts=true → StoneWithSkirts");
        assert_eq!(state.plaster_peel_amount().to_bits(), 0.25f32.to_bits());
        assert_eq!(state.freehand_ty(), 1, "+0x66 freehand_ty");
        assert_eq!(state.arch_type(), 1, "+0x64 arch_ty");
        assert_eq!(state.source_flag_65(), 1, "+0x65 structure_ty=Fence");
        assert_eq!(
            state.snapshot().metadata.source_69,
            4,
            "+0x69 clutter_version"
        );
        assert_eq!(state.removed_supports(), 1, "+0x67 removed_supports");
        assert_eq!(state.source().curve.length.to_bits(), 10.0f32.to_bits());
        assert_eq!(
            state.source().shape.tag(),
            2,
            "is_enclosed=No → shape tag 2"
        );
        // 曲线点列区待①：load_state 解码后点列为空（记录 0x70B 固定槽位）。
        assert!(state.source().curve.points.is_empty());
    }

    #[test]
    fn load_snapshot_walls_preserves_heap_curve_points() {
        // G1① §4：读档走语义 WallState，点列（堆 Vec）必须完整保留。
        let json = json!([[7, {
            "arch_height": 0.5, "arch_ty": "FollowTerrain",
            "curve": {"length": 10.0, "points": [[0.0,0.0,2.0],[3.0,0.0,5.0]],
                      "points_u": [0.0,1.0]},
            "elevation": 0.2, "has_skirts": false, "height": 4.0,
            "is_enclosed": "No", "structure_ty": "Wall"
        }]]);
        let rows: Vec<(u64, crate::resources::walls::inner_wall_state::WallState)> =
            serde_json::from_value(json).unwrap();
        let mut walls = InnerWalls::default();
        let count = load_snapshot_walls(
            &mut walls,
            rows.into_iter().map(|(id, state)| (WallId(id), state)),
        )
        .unwrap();
        assert_eq!(count, 1);
        let state = walls.get(WallId(7)).expect("wall restored");
        assert_eq!(state.source().curve.points, vec![[0.0, 2.0], [3.0, 5.0]]);
        assert_eq!(state.source().curve.points_u, vec![0.0, 1.0]);
        assert_eq!(walls.max_id(), 7);
        assert!(walls.changes().created.contains(&WallId(7)));
        // 原版 load_state 的 empty 断言：已有墙时再次载入必须失败。
        assert!(load_snapshot_walls(&mut walls, std::iter::empty()).is_err());
    }

    #[test]
    fn decorator_addr_accepts_terrain_and_stairs_string_forms() {
        // 真实样本 5/36 文件、9 条使用 "Stairs" 单元变体串（写出侧同形）。
        let section: DecoratorsSectionJson = serde_json::from_value(json!({
            "states": [
                ["Terrain", {"state": []}],
                ["Stairs", {"state": []}]
            ]
        }))
        .unwrap();
        let snapshot = section.into_storage_snapshot().unwrap();
        assert_eq!(snapshot.states[0].0, DecoratorStorageAddr::Terrain);
        assert_eq!(snapshot.states[1].0, DecoratorStorageAddr::Stair);
        // 旧对象判别仍兼容。
        let section: DecoratorsSectionJson =
            serde_json::from_value(json!({"states": [[{"Stair": 7}, {"state": []}]]})).unwrap();
        assert_eq!(
            section.into_storage_snapshot().unwrap().states[0].0,
            DecoratorStorageAddr::Stair
        );
    }

    #[test]
    fn decorator_seed_is_written_to_state_plus_0x2c_not_the_bucket_key() {
        // G1① §5.2/§7-7：36 样本中 5 条 seed != 桶键（如 key 9 / seed 18）。
        let json = json!({
            "states": [[{"Wall": 3}, {"state": [[9, {
                "dst": {"Terrain": {"pos_xz": [1.0, 2.0], "yaw_radians": 0.5}},
                "rank": {"x": 0}, "seed": 18, "color_id": 4, "ty": "Stairs"
            }]]}]]
        });
        let section: DecoratorsSectionJson = serde_json::from_value(json).unwrap();
        let snapshot = section.into_storage_snapshot().unwrap();
        let (id, state) = &snapshot.states[0].1.states[0];
        assert_eq!(id.0, 9, "bucket key stays the outer pair id");
        assert_eq!(state.u28 & 0xffff_ffff, 4, "+0x28 color_id");
        assert_eq!(state.u28 >> 32, 18, "+0x2c seed (not the bucket key)");
    }

    #[test]
    fn apply_terrain_section_writes_editing_data() {
        let json = json!({
            "next_stroke_index": 3,
            "strokes": {
                "1": {"falloff": 0.5, "height_ws": 2.0, "index": 1,
                      "points": [[0.0, 1.0, 2.0]]}
            }
        });
        let loaded = LoadedSnapshotData {
            version: 1,
            sections: vec![("Terrain", json)],
            terrain: Vec::new(),
            rasters: Vec::new(),
        };
        let mut resources = SnapshotLoadResources {
            loaded: Some(loaded),
        };
        let mut walls = InnerWalls::default();
        let mut decorators = DecoratorStorage::default();
        let mut decorator_ids = DecoratorIdGen::default();
        decorator_ids.reserve(crate::resources::walls::decorator::DecoratorId(42));
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        apply_loaded_snapshot(
            &mut resources,
            &mut walls,
            &mut decorators,
            &mut decorator_ids,
            &mut terrain,
            None,
        );
        assert_eq!(terrain.0.strokes.len(), 1);
        assert_eq!(terrain.0.next_index, 3);
        assert_eq!(
            decorator_ids.generate().0,
            43,
            "apply retains the caller-owned generator"
        );
    }

    #[test]
    fn world_apply_restores_authority_and_queues_real_domain_consumers() {
        use crate::components::trees::TreeComponent;
        use crate::resources::color_id::{GateColorIds, WallColorIds};
        use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
        use crate::resources::tree_storage::TreeStorage;
        use crate::resources::walls::InnerWalls;
        use crate::resources::world_raster::DirtyTilePublisher;
        use crate::resources::world_raster_defs::{
            AllWorldRasterWrite, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster,
            GardenRaster, PathRaster, WaterRaster,
        };
        use bevy_ecs::prelude::World;

        let json = json!({
            "Version": SNAPSHOT_FORMAT_VERSION,
            "Walls": [[7, {
                "arch_height": 0.5, "arch_ty": "FollowTerrain",
                "curve": {"length": 10.0, "points": [[0.0,0.0,2.0],[3.0,0.0,5.0]],
                          "points_u": [0.0,1.0]},
                "elevation": 0.2, "has_skirts": false, "height": 4.0,
                "is_enclosed": "No", "structure_ty": "Wall"
            }]],
            "Roofs": [{
                "id": 7,
                "params": {"profile_01": -2.0, "height_01": 3.0, "tip_offset_01": [0.25,-0.5]},
                "wall_shape": {"Rectangle": {
                    "orientation": [2.0, 3.0], "width": -4.0,
                    "length": 8.0, "center": [5.0, 6.0]}},
                "pivot_y": 7.0, "color_id": 13,
                "gable_style": "Stepped", "flat_roof_style": "Matching"
            }],
            "Decorators": {"states": []},
            "WallColors": [[5, 9]],
            "GateColors": [[6, 11]],
            "HalftimberColors": [[4, 3]],
            "Terrain": {"next_stroke_index": 2, "strokes": {
                "1": {"falloff": 0.5, "height_ws": 2.0, "index": 1,
                      "points": [[0.0, 1.0, 2.0]]}
            }},
            "Trees": [[3, {
                "position": [1.0, 2.0, 3.0], "scale": 1.5, "rotation_degrees": 0.0,
                "id": 4, "prefab_name": "tree_a", "is_preplaced": false
            }]],
            "Raster": []
        });
        let loaded =
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&json).unwrap()).unwrap();
        let resources = SnapshotLoadResources {
            loaded: Some(loaded),
        };

        let mut world = World::new();
        world.init_resource::<InnerWalls>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DecoratorIdGen>();
        world.insert_resource(TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData::default(),
        ));
        world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        world.init_resource::<WallColorIds>();
        world.init_resource::<GateColorIds>();
        world.init_resource::<crate::resources::color_id::HalftimberColorIds>();
        world.init_resource::<TreeStorage>();
        world.init_resource::<bevy_ecs::event::Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<bevy_ecs::event::Events<crate::resources::history::tree_edit::InsertTreeCmd>>();
        world.init_resource::<bevy_ecs::event::Events<crate::resources::color_id::SetWallColorIdCmd>>();
        world.init_resource::<bevy_ecs::event::Events<crate::resources::color_id::SetGateColorIdCmd>>();
        world.init_resource::<bevy_ecs::event::Events<crate::resources::color_id::SetHalftimberColorIdCmd>>();
        world.init_resource::<bevy_ecs::event::Events<crate::systems::prefab::SpawnPrefabCmd>>();
        world.init_resource::<bevy_ecs::event::Events<
            crate::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged,
        >>();
        world.init_resource::<bevy_ecs::event::Events<
            crate::resources::world_raster::UploadWorldRasterBuffersToStagingCmd,
        >>();
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));

        let report = init_game_state_from_snapshot(&mut world, &resources).unwrap();
        assert_eq!(report.walls, 1);
        assert_eq!(report.roofs, 1);
        assert_eq!(report.wall_colors, 1);
        assert_eq!(report.gate_colors, 1);
        assert_eq!(report.halftimber_colors, 1);
        assert_eq!(report.trees, 1);
        assert_eq!(report.terrain_strokes, 1);
        assert_eq!(report.raster_slots, 0);
        // 真实写回检查。
        assert!(
            world
                .resource::<InnerWalls>()
                .contains(crate::resources::walls::WallId(7))
        );
        // The producer never constructs bare entities or bypasses the color
        // consumers. Those systems own their real components and side effects.
        use bevy_ecs::event::Events;
        use crate::resources::color_id::{SetWallColorIdCmd, SetGateColorIdCmd, SetHalftimberColorIdCmd};
        let mut roofs = world.resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain().collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
        let request = roofs.pop().unwrap();
        assert_eq!(request.0.wall_id().0, 7);
        assert_eq!(u32::from_le_bytes(request.0.record()[16..20].try_into().unwrap()), 13);
        assert_eq!(request.0.record()[60], 1, "snapshot restore flag");
        let walls: Vec<_> = world.resource_mut::<Events<SetWallColorIdCmd>>().drain().collect();
        let gates: Vec<_> = world.resource_mut::<Events<SetGateColorIdCmd>>().drain().collect();
        let half: Vec<_> = world.resource_mut::<Events<SetHalftimberColorIdCmd>>().drain().collect();
        assert_eq!((walls[0].wall_id().0, walls[0].color, walls[0].custom), (5, 9, 0));
        assert_eq!((gates[0].wall_id().0, gates[0].color, gates[0].custom), (6, 11, 0));
        assert_eq!((half[0].wall_id().0, half[0].color, half[0].custom), (4, 3, 0));
        assert_eq!(world.resource::<WallColorIds>().0[5], 0);
        assert_eq!(world.resource::<GateColorIds>().0[6], 0);
        let trees: Vec<_> = world.resource_mut::<Events<crate::resources::history::tree_edit::InsertTreeCmd>>()
            .drain().collect();
        assert_eq!(trees.len(), 1);
        assert_eq!(trees[0].id, 3);
        assert_eq!(trees[0].position, glam::Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(trees[0].variant, 4);
        assert_eq!(trees[0].prefab_name, "tree_a");
        assert_eq!(world.query::<&Roof>().iter(&world).count(), 0);
        assert_eq!(world.query::<&TreeComponent>().iter(&world).count(), 0);
        assert_eq!(
            world.resource::<TerrainEditingData>().0.next_index,
            2,
            "terrain strokes written"
        );
        assert_eq!(AllWorldRasterWrite::ordered_names().len(), 6);
    }

    fn collect_snapshot_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_snapshot_files(&path, out);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("snapshot") {
                out.push(path);
            }
        }
    }

    /// 发行样本回归（G1① §7-12）。基线：36 个 `.snapshot`，计数由契约给出，
    /// 本机用同一批文件复核（162 walls / 73 roofs / 134 decorators / 431 wall
    /// colors / 0 gate / 20 halftimber / 590 trees / 104 点墙 / 5 条 seed≠key /
    /// 50 条缺 prefab_name+is_preplaced）。
    ///
    /// 资产目录可用 `TINY_GLADE_ASSETS_DIR` 覆盖；目录不存在时跳过，不伪造通过。
    #[test]
    fn shipped_snapshot_corpus_parses_expected_domain_counts() {
        let root = std::env::var("TINY_GLADE_ASSETS_DIR").unwrap_or_else(|_| {
            let windows = "D:/game/ljxsj_92385/Tiny Glade/assets/starting-builds";
            if std::path::Path::new(windows).exists() {
                windows.to_string()
            } else {
                concat!(env!("CARGO_MANIFEST_DIR"), "/../../../ljxsj_92385/Tiny Glade/assets/starting-builds").to_string()
            }
        });
        let root = std::path::Path::new(&root);
        if !root.exists() {
            eprintln!(
                "skipping shipped snapshot corpus: {} not found",
                root.display()
            );
            return;
        }
        let mut files = Vec::new();
        collect_snapshot_files(root, &mut files);
        files.sort();
        assert_eq!(files.len(), 36, "shipped snapshot corpus size");

        let (mut walls, mut roofs, mut decorators) = (0usize, 0usize, 0usize);
        let (mut wall_colors, mut gate_colors, mut halftimber) = (0usize, 0usize, 0usize);
        let mut trees = 0usize;
        let mut seed_mismatch = 0usize;
        let mut stair_buckets = 0usize;
        let mut missing_prefab = 0usize;
        let mut max_points = 0usize;
        let mut shape_tags = std::collections::BTreeSet::new();
        for path in &files {
            let loaded = load_snapshot_file(path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let section = |key: &str| {
                loaded
                    .sections
                    .iter()
                    .find(|(name, _)| *name == key)
                    .map(|(_, value)| value)
            };
            if let Some(value) = section("Walls") {
                let rows: Vec<(u64, crate::resources::walls::inner_wall_state::WallState)> =
                    serde_json::from_value(value.clone()).unwrap();
                for (_, state) in &rows {
                    assert!(
                        !state.source().curve.points.is_empty(),
                        "{}: wall without curve points",
                        path.display()
                    );
                    assert_eq!(
                        state.source().curve.points.len(),
                        state.source().curve.points_u.len(),
                        "{}: points/points_u length mismatch",
                        path.display()
                    );
                    max_points = max_points.max(state.source().curve.points.len());
                }
                walls += rows.len();
            }
            if let Some(value) = section("Roofs") {
                let rows: Vec<Roof> = serde_json::from_value(value.clone()).unwrap();
                for roof in &rows {
                    shape_tags.insert(roof.observed()[8]);
                }
                roofs += rows.len();
            }
            if let Some(value) = section("Decorators") {
                let section: DecoratorsSectionJson = serde_json::from_value(value.clone()).unwrap();
                let snapshot = section.into_storage_snapshot().unwrap();
                for (addr, list) in &snapshot.states {
                    if *addr == DecoratorStorageAddr::Stair {
                        stair_buckets += 1;
                    }
                    for (id, state) in &list.states {
                        if state.u28 >> 32 != u64::from(id.0) {
                            seed_mismatch += 1;
                        }
                    }
                    decorators += list.states.len();
                }
            }
            if let Some(value) = section("WallColors") {
                wall_colors += serde_json::from_value::<Vec<(u64, u32)>>(value.clone())
                    .unwrap()
                    .len();
            }
            if let Some(value) = section("GateColors") {
                gate_colors += serde_json::from_value::<Vec<(u64, u32)>>(value.clone())
                    .unwrap()
                    .len();
            }
            if let Some(value) = section("HalftimberColors") {
                halftimber += serde_json::from_value::<Vec<(u64, u32)>>(value.clone())
                    .unwrap()
                    .len();
            }
            if let Some(value) = section("Trees") {
                let entries = parse_trees_section(value).unwrap();
                missing_prefab += entries
                    .iter()
                    .filter(|entry| entry.tree.prefab_name.is_empty())
                    .count();
                trees += entries.len();
            }
        }
        assert_eq!(walls, 162);
        assert_eq!(roofs, 73);
        assert_eq!(decorators, 134);
        assert_eq!(wall_colors, 431);
        assert_eq!(gate_colors, 0);
        assert_eq!(halftimber, 20);
        assert_eq!(trees, 590);
        assert_eq!(max_points, 104, "104-point wall from the shipped corpus");
        assert_eq!(seed_mismatch, 5, "seed != bucket key rows");
        // 5/36 发行文件使用 `"Stairs"` 地址串（M1 回归）。
        assert_eq!(
            stair_buckets, 5,
            "decorator buckets addressed by \"Stairs\""
        );
        assert_eq!(missing_prefab, 50, "tree rows missing prefab_name");
        // 73 roofs: 25 Circle (tag 0) / 48 Rectangle (tag 1) at Roof+8 as ShapeParameters.
        assert_eq!(roofs, 73);
        assert_eq!(shape_tags.len(), 2);
    }

    #[test]
    fn pending_roof_associations_wait_for_public_wall_rows() {
        // 读档时 PublicWalls 尚无行：系统必须保持 pending（下帧重试），
        // 不能清空、也不能把它当作永久完成。
        let mut world = bevy_ecs::world::World::new();
        world.init_resource::<PendingRoofAssociationRebuild>();
        world.init_resource::<RoofAssociations>();
        world.insert_resource(PublicWalls::default());
        world.init_resource::<bevy_ecs::event::Events<
            crate::resources::roofs::commands::DeleteRoofCmd,
        >>();
        world.init_resource::<bevy_ecs::event::Events<
            crate::resources::roofs::commands::WallRoofPresenceChanged,
        >>();
        world
            .resource_mut::<PendingRoofAssociationRebuild>()
            .rows
            .push(PendingRoofAssociationRow {
                entity: 0x1_0000_0009,
                wall: 3,
            });
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(rebuild_pending_roof_associations);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<PendingRoofAssociationRebuild>().rows.len(),
            1
        );
    }
}

#[cfg(test)]
mod terrain_native_wire_contract_tests {
    use super::*;

    #[test]
    fn terrain_u64_max_roundtrips_through_real_motomoto_file_and_world_restore() {
        use crate::resources::{terrain_editing::TerrainEditingData,
            terrain_height::{TerrainHeightSsbo, TerrainHeightsData}};
        use crate::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged;
        let mut editing = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        editing.new_stroke([1.0, 2.0, 3.0]).unwrap().unwrap();
        editing.next_index = u64::MAX;
        let mut heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
        heights.set_height_range(-2.0, 7.0);
        for (index, value) in heights.heights_mut().iter_mut().enumerate() { *value = index as u16; }
        let raw = bytemuck::cast_slice(heights.heights()).to_vec();
        let domain = SnapshotDomainData::collect(Vec::new(), Vec::new(), Vec::new(),
            Some(StairsSnapshot::new(Vec::new())), Vec::new(),
            (Vec::new(), Vec::new(), Vec::new()), editing, Vec::new(), Vec::new())
            .with_terrain_and_supports(raw.clone(), -2.0..7.0, StairsRemovedSupports::default());
        let encoded = encode_game_snapshot(&domain).unwrap();
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("terrain.snapshot");
        std::fs::write(&path, encoded).unwrap();
        let mut loaded = load_snapshot_file(&path).unwrap();
        assert_eq!(loaded.terrain, raw);
        let section = &loaded.sections.iter().find(|(name, _)| *name == "Terrain").unwrap().1;
        assert_eq!(section["next_stroke_id"].as_u64(), Some(u64::MAX));
        // 本轮只检查地形域；文件另带空stairs快照，由宿主原有的canonical图owner消费。
        loaded.sections.retain(|(name, _)| matches!(*name, "Terrain" | "TerrainHeightRange"));
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()));
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
        world.init_resource::<bevy_ecs::event::Events<EvTerrainStrokeChanged>>();
        let report = apply_loaded_snapshot_to_world(&mut world, &SnapshotLoadResources { loaded: Some(loaded) }).unwrap();
        assert!(report.gaps.is_empty(), "{:?}", report.gaps);
        assert_eq!(report.terrain_strokes, 1);
        assert!(report.terrain_heights);
        assert_eq!(world.resource::<TerrainEditingData>().next_index, u64::MAX);
        assert_eq!(world.resource::<TerrainEditingData>().strokes[&tg_terrain::schema::StrokeId(0)].points, [[1.0, 2.0, 3.0]]);
        assert_eq!(bytemuck::cast_slice::<u16, u8>(world.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()), raw);
        let events = world.resource::<bevy_ecs::event::Events<EvTerrainStrokeChanged>>();
        assert_eq!(events.iter_current_update_events().map(|event| event.0).collect::<Vec<_>>(), [tg_terrain::schema::StrokeId(0)]);
        assert_eq!(world.resource_mut::<TerrainEditingData>().new_stroke([9.0; 3]).unwrap(), None);
        assert_eq!(world.resource::<TerrainEditingData>().next_index, u64::MAX);
    }

    #[test]
    fn terrain_json_rejects_missing_fields_and_invalid_numeric_keys() {
        // 原1415c64f4/650a是missing_field；键采用真实StrokeId数值serde错误传播。
        let mut incorrectly_accepted = Vec::new();
        for (name, text) in [
            ("missing_both", r#"{}"#),
            ("missing_strokes", r#"{"next_stroke_id":1}"#),
            ("missing_counter", r#"{"strokes":{}}"#),
            ("invalid_key", r#"{"next_stroke_id":1,"strokes":{"not-a-number":{"falloff":0.4,"height_ws":2.0,"index":0,"points":[[0.0,0.0,0.0]]}}}"#),
            ("overflow_key", r#"{"next_stroke_id":1,"strokes":{"4294967296":{"falloff":0.4,"height_ws":2.0,"index":0,"points":[[0.0,0.0,0.0]]}}}"#),
        ] {
            match serde_json::from_str::<TerrainSectionJson>(text) {
                Ok(section) => {
                    let before = section.strokes.len();
                    let after = section.into_editing_data().strokes.len();
                    incorrectly_accepted.push((name, before, after));
                }
                Err(error) => eprintln!("{name}: {error}"),
            }
        }
        assert!(incorrectly_accepted.is_empty(), "原应拒绝的地形数据被接受/丢弃: {incorrectly_accepted:?}");
        for text in [r#"{"next_stroke_id":0,"strokes":{}}"#, r#"{"next_stroke_index":0,"strokes":{}}"#] {
            assert!(serde_json::from_str::<TerrainSectionJson>(text).is_ok());
        }
        assert!(serde_json::from_str::<TerrainSectionJson>(r#"{"next_stroke_id":0,"next_stroke_index":0,"strokes":{}}"#).is_err());
    }

    #[test]
    fn terrain_wire_preserves_independent_map_key_and_stroke_index_without_normalization() {
        let text = r#"{"next_stroke_id":18446744073709551615,"strokes":{"4294967295":{"falloff":0.4,"height_ws":2.0,"index":17,"points":[[1.0,2.0,3.0]]}}}"#;
        let section: TerrainSectionJson = serde_json::from_str(text).unwrap();
        let editing = section.into_editing_data();
        let stroke = &editing.strokes[&crate::resources::terrain_editing::StrokeId(u32::MAX)];
        assert_eq!(stroke.index.0, 17);
        assert_eq!(editing.next_index, u64::MAX);
    }

    #[test]
    fn terrain_json_keeps_full_u64_counter_without_allowing_new_ids_after_native_limit() {
        for counter in [0x3ffffeu64, 0x3fffff, u64::from(u32::MAX), u64::from(u32::MAX) + 1, u64::MAX] {
            let text = format!(r#"{{"next_stroke_id":{counter},"strokes":{{}}}}"#);
            let section: TerrainSectionJson = serde_json::from_str(&text).unwrap();
            let mut editing = section.into_editing_data();
            assert_eq!(u64::from(editing.next_index), counter);
            let created = editing.new_stroke([1.0, 2.0, 3.0]).unwrap();
            assert_eq!(created.map(|id| u64::from(id.0)), (counter <= 0x3ffffe).then_some(counter));
            assert_eq!(u64::from(editing.next_index), counter + u64::from(created.is_some()));
        }
    }
}
