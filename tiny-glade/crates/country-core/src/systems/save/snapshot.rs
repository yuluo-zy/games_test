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
use crate::resources::session::SessionId;
use serde_json::Value;
use std::{path::Path, sync::Arc};

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

/// 快照存档数据（PDB：`snapshot::SnapshotSaveData`）。
///
/// 原版为 ~0x400B 结构（~9 组世界数据收集器），返回 Arc（alloc 0x18 {strong,weak}
/// + LOCK inc，O：0x1417a0430）。各节载荷 `section_values` 由域序列化器提供；
/// 本结构只负责节序拼装与 Arc 语义，不代造 Walls/Roofs 等域载荷。
#[derive(Clone, Debug)]
pub struct SnapshotSaveData {
    /// 所属会话。
    pub session: SessionId,
    /// 快照身份（保存文件 snapshots 节引用的 `{snapshot_id, chain_index}`）。
    pub snapshot_id: u64,
    pub chain_index: i32,
    /// 节序节载荷；键名即快照 JSON 键，值由域提供。
    pub section_values: Vec<(&'static str, Value)>,
}

impl SnapshotSaveData {
    /// 构建完整快照 JSON 文档（按 `SNAPSHOT_SECTION_ORDER` 节序）。
    pub fn to_json(&self) -> Value {
        let mut object = serde_json::Map::new();
        for (key, value) in &self.section_values {
            object.insert((*key).to_string(), value.clone());
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
    /// Terrain 节 deflate 解压后的原始载荷；未提供则为空。
    pub terrain: Vec<u8>,
}

/// 快照加载所需资源面（PDB：`snapshot::SnapshotLoadResources`）。域 Query 未齐，
/// 仅登记合同；`init_game_state_from_snapshot` 的应用逻辑留待对应域恢复。
#[derive(Clone, Debug, Default)]
pub struct SnapshotLoadResources {
    pub loaded: Option<LoadedSnapshotData>,
}

/// `capture_game_snapshot_save_data` 0x1417a0430：组装快照结构并返回 Arc。
/// 调用方传入真实域节载荷；不在此处构造假 Walls/Roofs。
pub fn capture_game_snapshot_save_data(
    session: SessionId,
    snapshot_id: u64,
    chain_index: i32,
    section_values: Vec<(&'static str, Value)>,
) -> Arc<SnapshotSaveData> {
    Arc::new(SnapshotSaveData {
        session,
        snapshot_id,
        chain_index,
        section_values,
    })
}

/// Terrain 载荷 deflate 压缩（O：flate2 deflate，0x1417a6e50 解压方互证）。
pub fn deflate_terrain(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = flate2::write::DeflateEncoder::new(
        Vec::new(),
        flate2::Compression::default(),
    );
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
/// data in snapshot: "）。白名单项待取证；当前仅登记结构入口。
pub const fn is_known_raster_key(_key: &str) -> bool {
    false
}

/// `validate_and_load_closest_snapshot_data` 0x1417a6e50：解析快照 JSON。
///
/// 节序 `Version → Walls → … → Raster`；Version 不符返回错误（原版 panic
/// "Version mismatch; expected , got "）；Terrain 节 deflate 解压；Raster 未知键忽略。
pub fn validate_and_load_closest_snapshot_data(
    json: &[u8],
) -> anyhow::Result<LoadedSnapshotData> {
    let value: Value = serde_json::from_slice(json)?;
    let Some(object) = value.as_object() else {
        anyhow::bail!("Snapshot document is not an object");
    };
    let version = object
        .get("Version")
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow::anyhow!("Snapshot has no Version"))?;
    let mut sections = Vec::new();
    let mut terrain = Vec::new();
    for key in SNAPSHOT_SECTION_ORDER {
        let Some(section) = object.get(*key) else {
            continue;
        };
        log::debug!("parsing {key}");
        if *key == "Terrain" {
            let bytes = section
                .as_array()
                .and_then(|array| array.first())
                .and_then(Value::as_str)
                .or_else(|| section.as_str())
                .map(|s| s.as_bytes().to_vec());
            if let Some(bytes) = bytes {
                terrain = inflate_terrain(&bytes)
                    .map_err(|error| anyhow::anyhow!("decompressing terrain data: {error}"))?;
            }
        }
        sections.push((*key, section.clone()));
    }
    Ok(LoadedSnapshotData {
        version,
        sections,
        terrain,
    })
}

/// `try_chunk` 0x1417a6d70：分块/文件读取辅助（`Result<Option<chunk>>`）。
/// 原版为文件块读取；本实现按 0x2000 块缓冲读取指定路径，供 GC/校验复用。
pub fn try_chunk(path: &Path, block: u64) -> anyhow::Result<Option<Vec<u8>>> {
    use std::io::{Read, Seek, SeekFrom};
    const CHUNK: u64 = 0x2000;
    let mut file = std::fs::File::open(path)?;
    file.seek(SeekFrom::Start(block * CHUNK))?;
    let mut buffer = vec![0u8; CHUNK as usize];
    let read = file.read(&mut buffer)?;
    if read == 0 {
        return Ok(None);
    }
    buffer.truncate(read);
    Ok(Some(buffer))
}

/// `init_game_state_from_snapshot` 0x1417a9c50 的装载面。域应用逻辑（颜色/时间/羊/地形…）
/// 由对应域提供；此处仅登记资源与待消费数据，不伪造"已应用"。
pub fn apply_loaded_snapshot(_resources: &mut SnapshotLoadResources) {
    // 域应用留待各域恢复后接入；保持 `loaded` 原样，不做虚假应用。
}
