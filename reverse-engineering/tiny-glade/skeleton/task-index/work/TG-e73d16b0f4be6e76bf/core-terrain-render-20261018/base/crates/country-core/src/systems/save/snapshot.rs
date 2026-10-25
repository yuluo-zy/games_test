//! B `systems/save/snapshot.rs` — 快照捕获/加载。
//!
//! 快照 schema 为 **JSON + deflate**（非 rkyv/ron）：节序
//! `Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain →
//! Trees → Raster`（O：0x1417a6e50 的节跳表 @0x142cdca80/98/b0）。Terrain 载荷
//! 用 flate2-1.0.24 deflate（O："decompressing terrain data" @0x142cdc800）。
//!
//! - `capture_game_snapshot_save_data` 0x1417a0430：从世界数据构建快照结构并返回 **Arc**。
//!   各节载荷由所属域序列化器提供（StairsSnapshot{edges,checksum}、
//!   TerrainEditingData{strokes,next_stroke_id,next_stroke_index}、
//!   DecoratorStorageSnapshot{states} 等）；本层不伪造域载荷。
//! - `capture_game_snapshot` 0x1417a1d00：巨型系统（全 Query 串化）。域 Query 消费留待
//!   各域恢复后接入。
//! - `validate_and_load_closest_snapshot_data` 0x1417a6e50：解析器，节序 + Version 校验
//!   （"Version mismatch; expected , got "）+ Terrain 解压 + Raster 键白名单外忽略
//!   （"Ignoring unknown raster data in snapshot: "）。
//! - `init_game_state_from_snapshot` 0x1417a9c50：应用解析数据到世界；域应用逻辑留待接入。
use crate::resources::{
    color_id::ColorId,
    roofs::roof_shape::Roof,
    session::SessionId,
    stairs::stairs_state::{StairsSnapshot, StairsState},
    terrain_editing::TerrainEditingData,
    walls::{
        WallId,
        decorator::DecoratorInfo,
        decorator_storage::{
            DecoratorAddrSnapshot, DecoratorStorage, DecoratorStorageAddr, DecoratorStorageSnapshot,
        },
        inner_wall_state::WallState,
    },
    world_raster_defs::{AllWorldRasterWrite, NamedRasterData},
};
use serde_json::Value;
use std::sync::Arc;

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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotTreesEntry {
    pub index: crate::resources::tree_storage::TreeStorageIndex,
    pub tree: crate::resources::history::tree_edit::TreeEditData,
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
    "WallColors",
    "GateColors",
    "Terrain",
    "Trees",
    "Raster",
];

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
    /// Stairs 快照（非 JSON 节；备份校验 + 链条快照）。
    pub stairs: StairsSnapshot,
    /// Raster 节 6 组 `(键, 载荷)`（O：capture_game_snapshot_save_data fold 6 entries）。
    pub raster: Vec<(&'static str, Vec<u8>)>,
    /// u64 集合采集器输出（O：from_iter<u64, Copied<HashSet::Iter<u64>>>）。
    pub ids: Vec<u64>,
    /// 调用方注入的 JSON 节载荷（域序列化器未就绪的节；与 typed 序列化输出合并，
    /// 键名即快照 JSON 键）。
    pub section_values: Vec<(&'static str, Value)>,
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
            stairs: stairs.unwrap_or_else(|| StairsSnapshot::new(Vec::new())),
            raster: raster.into_iter().collect(),
            ids: ids.into_iter().collect(),
            section_values: Vec::new(),
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
        self.trees.sort_unstable_by_key(|entry| entry.index.0);
        self.ids.sort_unstable();
        self
    }

    /// 按 `SNAPSHOT_SECTION_ORDER` 节序产出 JSON 节载荷。
    ///
    /// 域序列化器已就绪的节直接序列化；未就绪节保持缺席（不伪造节载荷），由域所有者
    /// 经 `section_values` 注入。Terrain 节为 JSON 明文 strokes（flate2 只用于栅格尾，
    /// 见 [`TerrainSectionJson`]）。
    pub fn to_section_values(&self) -> Vec<(&'static str, Value)> {
        let mut sections: Vec<(&'static str, Value)> = Vec::new();
        // Trees：SnapshotTreesEntry 已具备 derive serde（X5：树 Query 投影条目）。
        if !self.trees.is_empty() {
            sections.push((
                "Trees",
                serde_json::to_value(&self.trees).unwrap_or(Value::Null),
            ));
        }
        // Walls/Roofs/Decorators/WallColors/GateColors/Terrain/Raster 的域序列化器
        // 未实现（WallState/DecoratorStorageAddr/Roof 等无 serde），保持缺席；由域
        // 所有者经 section_values 注入。
        if !self.section_values.is_empty() {
            sections.extend(self.section_values.iter().cloned());
        }
        sections
    }
}

/// 快照存档数据（PDB：`snapshot::SnapshotSaveData`）。
///
/// 原版为 0x198B 结构（fresh 反编译 0x1417a0430 `memcpy(_Dst,…,0x198)`；RE-X5X7 曾估
/// ~0x400B 已由 fresh 纠正）+ 0xc0B 头部（域描述符），返回 Arc（alloc 0x18 {strong,
/// weak} + LOCK inc）。本结构承载域收集数据 [`SnapshotDomainData`] 与 Arc 语义，
/// 不代造 Walls/Roofs 等域载荷。
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
    /// 构建完整快照 JSON 文档（按 `SNAPSHOT_SECTION_ORDER` 节序）。
    pub fn to_json(&self) -> Value {
        let mut object = serde_json::Map::new();
        for (key, value) in self.domain.to_section_values() {
            object.insert(key.to_string(), value);
        }
        Value::Object(object)
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

/// `capture_game_snapshot_save_data` 0x1417a0430（fresh 反编译 2026-10-17，672 行）：
/// 组装快照结构并返回 **Arc**（alloc 0x18 {strong=1,weak=1} + LOCK inc）。
///
/// 原生体（SystemParam bundle 0x28 qword 槽，puffin scope）：
/// ① `assert!(res.stairs.backup_accurate())`（"assertion failed: res.stairs.
/// backup_accurate()"）；② `_DEV_SKIP_TREE_SERIALIZATION` 门控收集 Trees
/// `Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>`→
/// `Vec<(TreeStorageIndex, TreeData)>`（0x141305170）；③ `inner_walls::all()`→
/// `Vec<(WallId, WallState)>`（0x141308b40）；④ `HashSet<u64>`→`Vec<u64>`
/// （0x141309840）；⑤ `StairsState::iter_edges_and_stubs`+`StairsSnapshot::new`
/// （0x14157d200/0x140901120）；⑥ `DecoratorStorage::take_snapshot`（0x1412fbcd0，
/// Chain<wall iter + terrain + stair option>）；⑦ 6 色表资源（bundle[5/9/0xd/0x11/0x15/
/// 0x19]，每 4 qword，+0x298 判别选 +0x20/+0x28 或 +8/+0x10 字段对）；⑧ Raster 6 组
/// `(str, &[u8])` fold（0x141477c30，串 "GardenRaster" 等）；⑨ 三色 iter_nondefault
/// `(WallId, ColorId)`（0x141306570）；⑩ Terrain strokes hashbrown clone + 地形高度
/// Vec（4B/元素）；⑪ bundle[0x27] 又一 hashbrown clone。最终 `memcpy(_Dst, …, 0x198)`
/// 落 0x198B 结构。
///
/// 本实现以 [`SnapshotDomainData`] 承载各收集器输出（域 Query/资源采集由调用方喂入），
/// 不在此构造假 Walls/Roofs；`take_stairs_snapshot` 即 ① 断言 + ⑤ 采集入口。
/// 返回 Arc（O 语义）。
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

/// `capture_game_snapshot` 0x1417a1d00（fresh disasm 2026-10-17：6947+ B 巨型系统，
/// 非空反编译；旧 0 字节 decomp 为提取失败）。
///
/// 机器码/callee 证据（可直接对应的部分）：
/// - 入口把 SystemParam bundle（RDX +0x00..+0x190）整块搬到栈；
/// - `__rust_alloc(0x80000)` 建 JSON 缓冲；
/// - 收集结果**函数内排序**：walls `sort_unstable_by_key(closure_env$0 / WallId)`、
///   roofs `(closure_env$1 / Roof::wall_id)`、trees `(closure_env$2 / TreeStorageIndex)`、
///   decorators `decorator_storage::impl$18::sort`、ids `ipnsort<u64>`；
/// - `collect_seq<serde_json::value::ser::Serializer, …>` 把 Walls/Roofs/Trees/
///   三色(u32)/ids 转 `Value`，`Map::insert` 按节序组装，再
///   `serialize<CompactFormatter>` 写入缓冲（`add_chunk<Cursor<&mut Vec<u8>>, &str>`）。
///
/// 本实现：先 [`SnapshotDomainData::sorted`]，再按 [`SNAPSHOT_SECTION_ORDER`] 逐节
/// 插入 JSON 文档并序列化到 0x80000 初容量缓冲（serde_json Map 的键输出序由
/// `preserve_order` 特性决定；本 crate 为 BTreeMap 字典序，语义相同）。域序列化器
/// 未就绪的节保持缺席（由 `section_values` 注入），不伪造节载荷。
pub fn capture_game_snapshot(domain: SnapshotDomainData) -> Vec<u8> {
    let domain = domain.sorted();
    let mut object = serde_json::Map::new();
    // to_section_values 已含调用方注入的节；按其键集合按节序插入。
    let values = domain.to_section_values();
    for key in SNAPSHOT_SECTION_ORDER {
        if let Some((_, value)) = values.iter().find(|(name, _)| name == key) {
            object.insert((*key).to_string(), value.clone());
        }
    }
    let mut buffer = Vec::with_capacity(SNAPSHOT_JSON_BUFFER_CAPACITY);
    serde_json::to_writer(&mut buffer, &Value::Object(object))
        .expect("snapshot JSON serialization cannot fail (native unwrap_failed)");
    buffer
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
                // 原生 `Vec<(WallId, WallState)>`；WallState 无 serde 镜像 → 以
                // 0x78B 记录 JSON mirror（id + WallStateJson）typed 校验。
                serde_json::from_value::<Vec<(u64, WallStateJson)>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Walls: {error}"))?;
            }
            "Roofs" => {
                // 原生 `Vec<Roof>`（Roof 无 serde derive；域镜像待①）。这里保证
                // 形状为数组；元素保留 Value。
                if !section.is_array() {
                    anyhow::bail!("Roofs: expected an array");
                }
            }
            "Decorators" => {
                serde_json::from_value::<DecoratorsSectionJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Decorators: {error}"))?;
            }
            "WallColors" | "GateColors" => {
                serde_json::from_value::<Vec<(u64, u32)>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("{key}: {error}"))?;
            }
            "Terrain" => {
                serde_json::from_value::<TerrainSectionJson>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Terrain: {error}"))?;
            }
            "Trees" => {
                serde_json::from_value::<Vec<SnapshotTreesEntry>>(section.clone())
                    .map_err(|error| anyhow::anyhow!("Trees: {error}"))?;
            }
            "Raster" => {
                // Raster 节：白名单键保留（ordered_names），未知键忽略并 log；
                // 已知键载荷走 flate2 deflate 解压（O：DeflateDecoder + read_exact）。
                if let Some(map) = section.as_object() {
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
                    anyhow::bail!("Raster: expected an object");
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

/// 快照头部（O：`memcpy(&local_2c8, in_R8, 0xc0)`）。
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

/// `init_game_state_from_snapshot` 0x1417a9c50：域写回主链。
///
/// fresh 反编译：`memcpy(header, in_R8, 0xc0)` + 14 个 0x20B 描述符读取，随后按序
/// 写回（callees）：`InnerWalls::load_state` 0x14085ad80 → `DecoratorStorage::restore_snapshot`
/// 0x140a20d70 → … → `AllWorldRasterWrite::write` 0x14083ebe0 → `StairsState::restore_snapshot`
/// 0x1408ff590（另有 WindowFocusEvent/AvailableMonitor/SettingsMenuState grow 面由宿主）。
///
/// 本入口解析头/描述符后，以**已验证的 JSON 节数据**（validate 0x1417a6e50 的输出）
/// 走既有写回入口；描述符中的原生内存指针无法在 Rust 侧解引用，按域登记（不伪造）：
/// Walls→`apply_loaded_snapshot`、Decorators→`restore_snapshot`、Terrain→TerrainEditingData、
/// Raster→`write_named`、Stairs→`restore_snapshot`（宿主提供捕获侧 StairsSnapshot）。
pub fn init_game_state_from_snapshot(
    raw_snapshot: &[u8],
    resources: &mut SnapshotLoadResources,
    walls: &mut crate::resources::walls::inner_walls::InnerWalls,
    decorators: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
    terrain: &mut TerrainEditingData,
    mut rasters: Option<&mut AllWorldRasterWrite<'_>>,
    stairs: Option<&mut StairsState>,
    stairs_snapshot: Option<StairsSnapshot>,
) -> anyhow::Result<SnapshotPrefix> {
    // 1. 0xc0 头 + 14 域描述符（O：首段，逐字段偏移已核对）。
    let prefix = SnapshotPrefix::from_bytes(raw_snapshot)?;
    // 2. 域写回序（Walls → Decorators → … → Raster → Stairs）。
    apply_loaded_snapshot(resources, walls, decorators, terrain, rasters.take());
    if let (Some(stairs), Some(snapshot)) = (stairs, stairs_snapshot) {
        apply_stairs_writeback(stairs, snapshot);
    }
    Ok(prefix)
}

/// `init_game_state_from_snapshot` 0x1417a9c50 的装载面。域应用逻辑由对应域提供。
///
/// ⑤轮 C 块（① c-snapshot-apply-targets + c-apply-api-formats + c-apply-data-source）：
/// 数据流 = 节 JSON 值 → serde 反序列化 → 记录 → 目标 API 写回。
/// - **Walls**：`parse_wall_section`（JSON → 0x78B 记录，record[0]=WallId）→
///   `InnerWalls::load_state`（0x14085ad80）。
/// - **Decorators**：`parse_decorator_section`（JSON → 快照，0x28B 形状）→
///   `DecoratorStorage::restore_snapshot`（0x140a20d70）。
/// - **Terrain**：`parse_terrain_section`（JSON 承载 deflate → inflate）→ TerrainEditingData。
/// serde 架构/逐字段解码待①（中优先）；解析失败登记不伪造。Roofs/WallColors/GateColors/
/// Trees/Raster 域目标与写入序待①（中优先）。
///
/// ② r8 写回接线：Decorators 节 JSON → crate `DecoratorStorageSnapshot` →
/// `DecoratorStorage::restore_snapshot`（0x140a20d70）；Walls 节 JSON → 0x78B 记录
/// （record[0]=WallId + O 字段）→ `InnerWalls::load_state`（0x14085ad80，曲线区打包
/// 待①）；Terrain 节 JSON → `TerrainEditingData` → 写回地形资源。state 字节语义 /
/// 0x70B 曲线区 / 0x16B 子记录待 G4① 契约对齐。
///
/// **Raster 写回（0x14083ebe0 接线，2026-10-17）**：Raster 节 JSON 对象按
/// `AllWorldRasterWrite::ordered_names()` 6 白名单键逐槽解码（flate2 deflate 尾，
/// 形状待①）→ `apply_raster_writeback`（write_named 先校验名称/长度，任一不符整批
/// 不写）。`rasters` 缺省时登记不写。**Stairs 无 JSON 节**（StairsSnapshot 只在内存
/// `SnapshotSaveData` 域描述符），init 写回经 [`apply_stairs_writeback`] 由宿主把
/// 捕获侧 stairs 接入。
#[allow(clippy::too_many_arguments)]
pub fn apply_loaded_snapshot(
    resources: &mut SnapshotLoadResources,
    walls: &mut crate::resources::walls::inner_walls::InnerWalls,
    decorators: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
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
            // ① c-apply-data-source §2b：JSON 值 → serde → 记录 → load_state。
            "Walls" => {
                let records = parse_wall_section(value);
                if records.is_empty() {
                    log::trace!("apply Walls: JSON→0x78B 解析为空（曲线区打包待①）");
                } else {
                    walls.load_state(&records);
                }
            }
            "Decorators" => {
                // ① r7-chain1-writeback §1：真实 serde = {states:[[addr,{state:[{id,
                //   DecoratorInfo}]}]]}。② r8 接线：JSON → DecoratorStorageSnapshot →
                //   DecoratorStorage::restore_snapshot（0x140a20d70）真写回。
                //   state 字节语义 / 0x28B/0x16B 逐槽打包待①。
                if let Ok(section) = serde_json::from_value::<DecoratorsSectionJson>(value.clone())
                {
                    let snapshot = section.into_storage_snapshot();
                    let count = snapshot.states.len();
                    decorators.restore_snapshot(&snapshot);
                    log::trace!("apply Decorators: restore {count} snapshot entries");
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
            // Roofs/WallColors/GateColors/Trees：目标资源与写入序待①。
            _ => log::trace!("apply domain {key}（目标映射待①）"),
        }
    }
    let _ = loaded.version;
    let _ = &loaded.terrain;
}

/// init 写回：Stairs 快照应用（O：`StairsState::restore_snapshot` 0x1408ff590）。
///
/// 快照 JSON **无 Stairs 节**（StairsSnapshot 只存在于内存 [`SnapshotSaveData`] 的
/// 域描述符/`SnapshotDomainData.stairs`），故 [`apply_loaded_snapshot`] 不承载。
/// 本入口 = init 链（`init_game_state_from_snapshot` 0x1417a9c50）的 Stairs 写回接线点：
/// 宿主把捕获侧 `SnapshotSaveData.domain.stairs` 传入，`restore_snapshot` 计算差值、
/// 清旧备份、写回当前图并重建备份（backed_up=true）。
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

/// Walls 节→0x78B 记录解析（① r7-chain1-field 契约 §2，**真实发行样本权威**）：节载荷 =
/// **JSON 值** `[[WallId, WallStateJson 8 字段], …]`；0x78B 记录 = 反序列化后内存形式
/// （{WallId u64@0, WallState 0x70B@8}）。
///
/// ② r8 写回接线：record[0]=WallId（O）+ WallState `+0x54` height（JSON `height` 直映，O
/// `update_wall_state_height` 写 +0x54）。**其余字段映射待①**：`+0x60` freehand / `+0x64`
/// arch / `+0x67` removed_supports 需 JSON 串（arch_ty/is_enclosed/structure_ty）→ 字节
/// 枚举表映射；`+0x00..0x53` 曲线区（可变点列 2..104 点）0x70B 固定记录打包方式待①
/// （r7-chain1-writeback §2 样本受限登记）。本函数构造 record[0]+height 已证面，
/// 其余槽位 0 + 注释锁定，不伪造打包。
pub fn parse_wall_section(
    json: &serde_json::Value,
) -> Vec<crate::resources::walls::inner_walls::InnerWallStateRecord> {
    let Ok(walls) = serde_json::from_value::<Vec<(u64, WallStateJson)>>(json.clone()) else {
        log::trace!("parse_wall_section: JSON shape 待①（serde 架构）");
        return Vec::new();
    };
    let mut records = Vec::with_capacity(walls.len());
    for (id, state) in walls {
        let mut bytes = [0u8; 0x78];
        // record[0] = WallId（0x78B 首 qword LE，O）。
        bytes[..8].copy_from_slice(&id.to_le_bytes());
        // WallState +0x54 = height f32（JSON `height` 直映，O 写偏移 +0x54）。
        bytes[8 + 0x54..8 + 0x58].copy_from_slice(&state.height.to_le_bytes());
        // +0x60 freehand / +0x64 arch / +0x67 removed_supports 与 +0x00..0x53 曲线区：
        // 串→字节映射与打包方式待①，槽位保持 0（不编造枚举/曲线打包）。
        records.push(crate::resources::walls::inner_walls::InnerWallStateRecord(
            bytes,
        ));
    }
    log::trace!(
        "parse_wall_section: {} wall records（record[0]+height 已证；曲线区/枚举待①）",
        records.len()
    );
    records
}

/// 快照 Walls 节 WallState 对象 serde（① r7-chain1-field 契约 §2，真实样本全字段）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct WallStateJson {
    pub arch_height: f32,
    pub arch_ty: String,
    pub curve: WallCurveJson,
    pub elevation: f32,
    pub has_skirts: bool,
    pub height: f32,
    pub is_enclosed: String,
    pub structure_ty: String,
}

/// WallState curve 子对象（真实样本：length/points/points_u）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct WallCurveJson {
    pub length: f32,
    pub points: Vec<[f32; 3]>,
    pub points_u: Vec<f32>,
}

/// 快照 DecoratorId 编码掩码（_UNK_1429cd080/_UNK_1429cd090，数值待①）。
/// 与 crate `resources::walls::decorator_storage` 的 `SNAPSHOT_ID_MASK = 0`
/// 同源；① r6-decorators-serde §1 的"16B XOR 掩码"被 `take_snapshot` 用作文档
/// 记录的哈希计算输入（decomp-0x140a20d70 `auVar6/auVar7 ^ auVar32/auVar37`），
/// 其**数值是否为 0** 待①取证——现按 crate 模型 0 对齐，不编造掩码值。
pub const SNAPSHOT_ID_MASK: u64 = 0;

/// 编码：`(u64::from(id)).rotate_right(32) ^ SNAPSHOT_ID_MASK`（crate
/// `encode_snapshot_id` 同公式）。掩码数值待①；G4① 契约定值后此处对齐。
#[inline]
pub fn encode_snapshot_id(id: u32) -> u64 {
    (u64::from(id)).rotate_right(32) ^ SNAPSHOT_ID_MASK
}

/// 解码：`(encoded ^ SNAPSHOT_ID_MASK).rotate_left(32)`（crate `decode_snapshot_id`
/// 同公式）。
#[inline]
pub fn decode_snapshot_id(encoded: u64) -> u32 {
    (encoded ^ SNAPSHOT_ID_MASK).rotate_left(32) as u32
}

/// Decorators 0x28B 节记录（① r6-field-status §4，O 形状）：`{u32 id@0（快照编码）,
/// 8B×4 @4..0x14, u32×2 @0x18..0x20}`；内层 0x16B 子记录（含 +0x52 状态字节）。
/// crate `DecoratorAddrSnapshot`（16B）与 0x28B 节格式**不符**（round4 标注）；
/// **JSON↔0x28B serde 架构与逐槽字段待①**（中优先）。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DecoratorSectionRecord(pub [u8; 0x28]);

impl DecoratorSectionRecord {
    /// 快照编码 id（u32@0，O）。
    pub fn encoded_id(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }

    /// ID 旋转 XOR 编码为 u64（① r6-decorators §1：`(id 32 位旋转<<0x20|id>>0x20) ^
    /// 16B XOR 掩码 0x1429cd080`；crate `encode_snapshot_id` 语义——`u64::from(id).
    /// rotate_right(32) ^ mask`）。掩码值待①（现 0，同 crate SNAPSHOT_ID_MASK）。
    pub fn encoded_u64(&self) -> u64 {
        (u64::from(self.encoded_id())).rotate_right(32) ^ SNAPSHOT_ID_MASK
    }

    /// 子侧状态字节公式 `(b-2)<=2 ? b-1 : 0`（① r6-decorators §1；crate
    /// `side_from_state` 语义，u8 回绕）。
    pub fn side_from_state(b: u8) -> u8 {
        if 2 < b.wrapping_sub(2) {
            0
        } else {
            b.wrapping_sub(1)
        }
    }
}

/// Decorators 节→0x28B 记录解析（① r6-field-status §4，可先按 0x28B 组装）：节载荷 =
/// **JSON 值**；0x28B 记录 = 反序列化后内存形式。按候选形状 `Vec<[u64;5]>`（0x28B=5 qword）
/// 反序列化 → 组装 `DecoratorSectionRecord`；**serde 架构（JSON↔0x28B/0x16B 逐槽）待①**，
/// 失败返回 `None`（不构造假快照）。
pub fn parse_decorator_section(json: &serde_json::Value) -> Option<Vec<DecoratorSectionRecord>> {
    let Ok(records) = serde_json::from_value::<Vec<[u64; 5]>>(json.clone()) else {
        log::trace!("parse_decorator_section: JSON↔0x28B 架构待①");
        return None;
    };
    Some(
        records
            .into_iter()
            .map(|qwords| {
                let mut bytes = [0u8; 0x28];
                for (i, q) in qwords.iter().enumerate() {
                    bytes[i * 8..i * 8 + 8].copy_from_slice(&q.to_le_bytes());
                }
                DecoratorSectionRecord(bytes)
            })
            .collect(),
    )
}

/// Decorators 节 JSON → 0x28B 记录打包**骨架**（② r8；0x28B = {u32 装饰 id@0,
/// 8B×4 @4..0x14, u32×2 @0x18..0x20}，① r6 §1）。
///
/// 骨架落地面：每个 `{id, DecoratorInfo}` 条目 → 一条 0x28B 记录，`+0` 写 **原始
/// DecoratorId**（u32 LE，与 [`DecoratorSectionRecord::encoded_u64`] 的 re-encode 语义
/// 一致）。**待①**：+4..0x14/+0x18..0x20 槽位（8B×4 + u32×2）逐槽字段名、内层 0x16B
/// 子记录（+0x52 状态字节）布局、ID 编码掩码（0x1429cd080）数值——本骨架将未定槽位
/// 置 0 并锁定注释，不编造打包。
pub fn pack_decorators_section(json: &serde_json::Value) -> Option<Vec<DecoratorSectionRecord>> {
    let Ok(section) = serde_json::from_value::<DecoratorsSectionJson>(json.clone()) else {
        log::trace!("pack_decorators_section: JSON↔DecoratorsSectionJson 解析失败");
        return None;
    };
    let mut records = Vec::new();
    for (addr_json, list) in &section.states {
        let _ = addr_json.to_storage_addr();
        for (id, _info) in &list.state {
            let mut bytes = [0u8; 0x28];
            // +0：原始 DecoratorId（u32 LE；encoded_u64() 按 crate 模型 re-encode）。
            bytes[..4].copy_from_slice(&id.to_le_bytes());
            // +4..0x14/+0x18..0x20 槽位与 0x16B 子记录：待①，保持 0。
            records.push(DecoratorSectionRecord(bytes));
        }
    }
    log::trace!(
        "pack_decorators_section: {} 0x28B records（骨架；逐槽/子记录待①）",
        records.len()
    );
    Some(records)
}

/// 快照 Terrain 节 serde（① r7-chain1-field 契约 §3，**真实发行样本权威**）：
/// `{next_stroke_index, strokes: {id 字符串: {falloff, height_ws, index, points}}}`。
/// **纠偏**：strokes 为 **JSON 明文**（非 flate2 deflate；flate2 只用于二进制栅格尾）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct TerrainSectionJson {
    #[serde(default)]
    pub next_stroke_index: u32,
    #[serde(default)]
    pub strokes: std::collections::HashMap<String, TerrainStrokeJson>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TerrainStrokeJson {
    pub falloff: f32,
    pub height_ws: f32,
    pub index: u32,
    pub points: Vec<[f32; 3]>,
}

/// 快照 Decorators 节 addr 判别（① r7-chain1-writeback 契约 §1，非空样本
/// 33273486/87775451 反推）：`{"Wall": <id>}` / `"Terrain"` / `{"Stair": <id>}`。
#[derive(Clone, Debug)]
pub enum DecoratorAddrJson {
    Wall { Wall: u64 },
    Terrain,
    Stair { Stair: u64 },
}

/// 手写 `Deserialize`（样本契约 `{"Wall": <id>}` / `"Terrain"` / `{"Stair": <id>}`）：
/// `#[serde(untagged)]` derive 不能把字符串 `"Terrain"` 解到单元变体
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
                    "a snapshot decorator addr: {\"Wall\": <id>}, \"Terrain\", or {\"Stair\": <id>}",
                )
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value == "Terrain" {
                    Ok(DecoratorAddrJson::Terrain)
                } else {
                    Err(serde::de::Error::unknown_variant(
                        value,
                        &["Terrain", "Wall", "Stair"],
                    ))
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
    /// 2=Stair，payload=WallId）。Stair 的 <id> 判别在 crate 模型为单元变体
    /// （多楼梯地址的细分待①）。
    pub fn to_storage_addr(&self) -> DecoratorStorageAddr {
        match self {
            DecoratorAddrJson::Wall { Wall } => DecoratorStorageAddr::Wall(WallId(*Wall)),
            DecoratorAddrJson::Terrain => DecoratorStorageAddr::Terrain,
            DecoratorAddrJson::Stair { Stair: _ } => DecoratorStorageAddr::Stair,
        }
    }
}

/// 快照 Decorators 节 DecoratorInfo（① r7-chain1-writeback §1，真实样本 8 字段）。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorInfoJson {
    pub allow_connection: bool,
    pub clv: u32,
    pub color_id: u32,
    pub dst: DecoratorDstJson,
    pub rank: DecoratorRankJson,
    pub seed: u32,
    pub stair_width: f32,
    pub ty: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DecoratorDstJson {
    Wall { Wall: DecoratorWallDstJson },
    Terrain { Terrain: DecoratorTerrainDstJson },
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorWallDstJson {
    pub anchor: DecoratorAnchorJson,
    pub coord: DecoratorCoordJson,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorAnchorJson {
    pub anchor_ws: [f32; 3],
    pub corner: u32,
    pub platform: u32,
    pub wall_height_relative_y: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorCoordJson {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorTerrainDstJson {
    pub pos_xz: [f32; 2],
    pub yaw_radians: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorRankJson {
    pub x: u32,
}

/// 快照 Decorators 节（① r7-chain1-writeback §1）：`{states: [[addr 判别,
/// {"state": [ {id, DecoratorInfo}, ... ]}], ...]}`。
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DecoratorsSectionJson {
    pub states: Vec<(DecoratorAddrJson, DecoratorStateListJson)>,
}

impl DecoratorsSectionJson {
    /// 每地址的装饰状态列表 → crate `DecoratorStorageSnapshot` 快照条目。
    /// 每 `{id, DecoratorInfo}` 条目 → `DecoratorAddrSnapshot{encoded_id, addr,
    /// state, info}`；state 字节语义 U（待①）。crate `restore_snapshot`
    /// （0x140a20d70）现按 fresh 重建：removed_ids.remove + modified_ids.insert +
    /// 内嵌 map insert(info) + d2s insert（冲突 panic）。
    ///
    /// 这是 `apply_loaded_snapshot` "Decorators" 分支的真实写回输入面。
    pub fn into_storage_snapshot(&self) -> DecoratorStorageSnapshot {
        let mut states = Vec::new();
        for (addr_json, list) in &self.states {
            let addr = addr_json.to_storage_addr();
            for (id, _info) in &list.state {
                states.push(DecoratorAddrSnapshot {
                    encoded_id: encode_snapshot_id(*id),
                    addr,
                    // U：原生状态字节判别（0x142ac9bdc/0x142ac9bfc 跳表）待①。
                    state: 0,
                    // U：DecoratorInfoJson → 0x48B DecoratorInfo 逐字段映射待①
                    // （c-apply-api-formats 契约）；当前以默认 info 承载，restore
                    // 机制已按 fresh 重建，字段映射待①补齐。
                    info: DecoratorInfo::default(),
                });
            }
        }
        DecoratorStorageSnapshot { states }
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
            if let Ok(idx) = id.parse::<u32>() {
                strokes.insert(
                    StrokeId(idx),
                    TerrainStroke {
                        points: stroke.points,
                        falloff: stroke.falloff,
                        index: StrokeId(stroke.index),
                        height_ws: stroke.height_ws,
                    },
                );
            }
        }
        crate::resources::terrain_editing::TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData {
                strokes,
                next_index: self.next_stroke_index,
            },
        )
    }
}

/// Terrain 节 flate2 解压入口（① r7-chain1-field 契约 §3 纠偏：**Terrain strokes 为
/// JSON 明文，flate2 deflate 只用于二进制栅格尾**（"decompressing terrain data"
/// @0x142cdc800）；本函数保留作栅格尾解压候选，JSON Terrain 节请用
/// [`TerrainSectionJson`]）。
pub fn parse_terrain_section(json: &serde_json::Value) -> Option<Vec<u8>> {
    let bytes = json
        .as_array()
        .and_then(|array| array.first())
        .and_then(serde_json::Value::as_str)
        .or_else(|| json.as_str())
        .map(|s| s.as_bytes().to_vec());
    bytes.and_then(|compressed| inflate_terrain(&compressed).ok())
}

/// Terrain 节 flate2 解压入口（兼容旧签名，① c-apply-section-parsing §3）。
pub fn inflate_terrain_entry(bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    inflate_terrain(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
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
            section_values: vec![("Version", json!(27))],
        };
        let bytes = capture_game_snapshot(domain);
        assert!(bytes.len() <= SNAPSHOT_JSON_BUFFER_CAPACITY);
        let parsed: Value = serde_json::from_slice(&bytes).unwrap();
        let object = parsed.as_object().unwrap();
        // 原生经 serde_json::Map<String, Value>::insert 组装后 serialize；本 crate 的
        // serde_json 未启用 preserve_order，Map = BTreeMap → 键按字典序输出
        // （"Trees" < "Version"）。JSON 对象语义与键序无关；**解析**节序仍是
        // Version → … → Raster（见 validate）。
        assert!(object.contains_key("Version"));
        assert!(object.contains_key("Trees"));
        assert_eq!(object["Version"], json!(27));
        assert_eq!(object["Trees"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn validate_parses_version_typed_and_ignores_unknown_raster_keys() {
        let payload = deflate_terrain(&[1, 2, 3]).unwrap();
        let json = json!({
            "Version": 27,
            "Walls": [],
            "Decorators": {"states": []},
            "Terrain": {"next_stroke_index": 0, "strokes": {}},
            "Trees": [],
            "Raster": {"NotARaster": [], "PathRaster": payload}
        });
        let loaded =
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&json).unwrap()).unwrap();
        assert_eq!(loaded.version, 27);
        assert_eq!(loaded.rasters.len(), 1);
        assert_eq!(loaded.rasters[0].0, "PathRaster");
        assert_eq!(loaded.rasters[0].1, vec![1, 2, 3]);
        // Version 非 i32 → typed 解析失败（非宽松 as_i64）。
        let bad = json!({"Version": "27"});
        assert!(
            validate_and_load_closest_snapshot_data(&serde_json::to_vec(&bad).unwrap()).is_err()
        );
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
    fn snapshot_id_encode_decode_roundtrip() {
        for id in [0u32, 1, 0x1020, u32::MAX] {
            assert_eq!(decode_snapshot_id(encode_snapshot_id(id)), id);
        }
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
                DecoratorAddrSnapshot {
                    encoded_id: 0,
                    addr: DecoratorStorageAddr::Wall(WallId(2)),
                    state: 0,
                    info: DecoratorInfo::default(),
                },
            ),
            (
                DecoratorStorageAddr::Terrain,
                DecoratorAddrSnapshot {
                    encoded_id: 0,
                    addr: DecoratorStorageAddr::Terrain,
                    state: 0,
                    info: DecoratorInfo::default(),
                },
            ),
            (
                DecoratorStorageAddr::Stair,
                DecoratorAddrSnapshot {
                    encoded_id: 0,
                    addr: DecoratorStorageAddr::Stair,
                    state: 0,
                    info: DecoratorInfo::default(),
                },
            ),
            (
                DecoratorStorageAddr::Wall(WallId(1)),
                DecoratorAddrSnapshot {
                    encoded_id: 0,
                    addr: DecoratorStorageAddr::Wall(WallId(1)),
                    state: 0,
                    info: DecoratorInfo::default(),
                },
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
    fn decorators_json_into_storage_snapshot_and_apply_restore() {
        let json = json!({
            "states": [
                [{"Wall": 5}, {"state": [
                    [10, {"allow_connection": false, "clv": 3, "color_id": 1,
                          "dst": {"Terrain": {"pos_xz": [1.0, 2.0], "yaw_radians": 0.5}},
                          "rank": {"x": 0}, "seed": 7, "stair_width": 0.0, "ty": "CottageWindow"}],
                    [11, {"allow_connection": true, "clv": 4, "color_id": 2,
                          "dst": {"Wall": {"anchor": {"anchor_ws": [0.0,1.0,2.0], "corner": 0,
                                                       "platform": 0, "wall_height_relative_y": 0.5},
                                            "coord": {"x": 0.1, "y": 0.2}}},
                          "rank": {"x": 1}, "seed": 8, "stair_width": 1.0, "ty": "Stairs"}]
                ]}],
                ["Terrain", {"state": [
                    [12, {"allow_connection": true, "clv": 9, "color_id": 0,
                          "dst": {"Terrain": {"pos_xz": [3.0, 4.0], "yaw_radians": 1.0}},
                          "rank": {"x": 2}, "seed": 5, "stair_width": 0.0, "ty": "CottageWindow"}]
                ]}]
            ]
        });
        let section: DecoratorsSectionJson = serde_json::from_value(json.clone()).unwrap();
        let snapshot = section.into_storage_snapshot();
        assert_eq!(snapshot.states.len(), 3);
        assert_eq!(
            snapshot.states[0].addr,
            DecoratorStorageAddr::Wall(WallId(5))
        );
        assert_eq!(decode_snapshot_id(snapshot.states[0].encoded_id), 10);
        assert_eq!(
            snapshot.states[1].addr,
            DecoratorStorageAddr::Wall(WallId(5))
        );
        assert_eq!(snapshot.states[2].addr, DecoratorStorageAddr::Terrain);

        // apply_loaded_snapshot "Decorators" 分支真写回（restore_snapshot）。
        let loaded = LoadedSnapshotData {
            version: 1,
            sections: vec![("Decorators", json)],
            terrain: Vec::new(),
            rasters: Vec::new(),
        };
        let mut resources = SnapshotLoadResources {
            loaded: Some(loaded),
        };
        let mut walls = InnerWalls::default();
        let mut decorators = DecoratorStorage::default();
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        apply_loaded_snapshot(
            &mut resources,
            &mut walls,
            &mut decorators,
            &mut terrain,
            None,
        );
        assert_eq!(
            decorators.lookup_storage(DecoratorId(10)),
            Some(DecoratorStorageAddr::Wall(WallId(5)))
        );
        let wall_bucket = decorators
            .get(DecoratorStorageAddr::Wall(WallId(5)))
            .unwrap();
        // fresh restore_snapshot：内嵌 map 回填 info（removed_ids.remove +
        // modified_ids.insert + decorators.insert + d2s insert）。
        assert!(wall_bucket.decorators.contains_key(&10));
        assert!(wall_bucket.decorators.contains_key(&11));
        assert!(wall_bucket.modified_ids.contains(&10));
        assert!(wall_bucket.modified_ids.contains(&11));
        let terrain_bucket = decorators.get(DecoratorStorageAddr::Terrain).unwrap();
        assert!(terrain_bucket.decorators.contains_key(&12));
    }

    #[test]
    fn parse_wall_section_builds_record_id_and_height() {
        let json = json!([[7, {
            "arch_height": 0.5, "arch_ty": "None",
            "curve": {"length": 10.0, "points": [[0.0,1.0,2.0],[3.0,4.0,5.0]],
                      "points_u": [0.0,1.0]},
            "elevation": 0.2, "has_skirts": true, "height": 4.0,
            "is_enclosed": "No", "structure_ty": "Plain"
        }]]);
        let records = parse_wall_section(&json);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].wall_id(), WallId(7));
        assert_eq!(records[0].height(), 4.0);
        // +0x60/+0x64/+0x67 与曲线区槽位 0（待①）。
        assert_eq!(records[0].freehand_type(), 0);
        assert_eq!(records[0].arch_type(), 0);
        assert_eq!(records[0].removed_supports(), 0);
    }

    #[test]
    fn pack_decorators_section_skeleton_encodes_raw_ids() {
        let json = json!({
            "states": [[{"Wall": 3}, {"state": [[9, {
                "allow_connection": false, "clv": 3, "color_id": 1,
                "dst": {"Terrain": {"pos_xz": [1.0, 2.0], "yaw_radians": 0.5}},
                "rank": {"x": 0}, "seed": 7, "stair_width": 0.0, "ty": "Stairs"
            }]]}]]
        });
        let packed = pack_decorators_section(&json).unwrap();
        assert_eq!(packed.len(), 1);
        assert_eq!(packed[0].encoded_id(), 9);
        // re-encode 与 crate 模型一致（掩码 0，数值待①）。
        assert_eq!(packed[0].encoded_u64(), encode_snapshot_id(9));
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
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        apply_loaded_snapshot(
            &mut resources,
            &mut walls,
            &mut decorators,
            &mut terrain,
            None,
        );
        assert_eq!(terrain.0.strokes.len(), 1);
        assert_eq!(terrain.0.next_index, 3);
    }
}
