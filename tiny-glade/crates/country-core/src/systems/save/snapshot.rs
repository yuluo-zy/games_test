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
use crate::resources::terrain_editing::TerrainEditingData;
use crate::resources::walls::{
    WallId,
    decorator_storage::{DecoratorAddrSnapshot, DecoratorStorageAddr, DecoratorStorageSnapshot},
    inner_wall_state::WallState,
};
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

/// 快照域描述符（① c 契约 §1，O）：`{ptr, len, count}`，每域 0x20B 对齐
/// （8B 指针 + 8B 长度 + 4B 计数）。`init_game_state_from_snapshot` 0x1417a9c50 =
/// **0xc0B 头部 + 11 个域描述符**（decomp-r5-0x1417a9c50）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SnapshotDomainDescriptor {
    /// 8B 指针（域数据）。
    pub ptr: u64,
    /// 8B 长度。
    pub len: u64,
    /// 4B 计数。
    pub count: u32,
}

/// 快照头部（O：0xc0B 整体拷贝，`func_0x000142923c00(&buf, snapshot, 0xc0)`）。
pub const SNAPSHOT_HEADER_SIZE: usize = 0xc0;
/// 域描述符个数（O：11，偏移 +0xc0..+0x25c）。
pub const SNAPSHOT_DOMAIN_COUNT: usize = 11;

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
#[allow(clippy::too_many_arguments)]
pub fn apply_loaded_snapshot(
    resources: &mut SnapshotLoadResources,
    walls: &mut crate::resources::walls::inner_walls::InnerWalls,
    decorators: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
    terrain: &mut TerrainEditingData,
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
                if let Ok(section) =
                    serde_json::from_value::<DecoratorsSectionJson>(value.clone())
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
            // Roofs/WallColors/GateColors/Trees/Raster：目标资源与写入序待①。
            _ => log::trace!("apply domain {key}（目标映射待①）"),
        }
    }
    let _ = loaded.version;
    let _ = &loaded.terrain;
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
        records.push(crate::resources::walls::inner_walls::InnerWallStateRecord(bytes));
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
pub fn parse_decorator_section(
    json: &serde_json::Value,
) -> Option<Vec<DecoratorSectionRecord>> {
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
pub fn pack_decorators_section(
    json: &serde_json::Value,
) -> Option<Vec<DecoratorSectionRecord>> {
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
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DecoratorAddrJson {
    Wall { Wall: u64 },
    Terrain,
    Stair { Stair: u64 },
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
    /// state}`；state 字节语义 U（待①；crate `restore_snapshot` 现按 removed 处理）。
    ///
    /// 这是 `apply_loaded_snapshot` "Decorators" 分支的真实写回输入面：
    /// `DecoratorStorage::restore_snapshot`（0x140a20d70）消费。
    pub fn into_storage_snapshot(&self) -> DecoratorStorageSnapshot {
        let mut states = Vec::new();
        for (addr_json, list) in &self.states {
            let addr = addr_json.to_storage_addr();
            for (id, _info) in &list.state {
                states.push(DecoratorAddrSnapshot {
                    encoded_id: encode_snapshot_id(*id),
                    addr,
                    // U：原生状态字节判别（0x142ac9bdc/0x142ac9bfc 跳表）待①；
                    // crate restore 现不区分状态字节，统一按 removed 恢复。
                    state: 0,
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
    fn snapshot_id_encode_decode_roundtrip() {
        for id in [0u32, 1, 0x1020, u32::MAX] {
            assert_eq!(decode_snapshot_id(encode_snapshot_id(id)), id);
        }
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
        };
        let mut resources = SnapshotLoadResources {
            loaded: Some(loaded),
        };
        let mut walls = InnerWalls::default();
        let mut decorators = DecoratorStorage::default();
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        apply_loaded_snapshot(&mut resources, &mut walls, &mut decorators, &mut terrain);
        assert_eq!(
            decorators.lookup_storage(DecoratorId(10)),
            Some(DecoratorStorageAddr::Wall(WallId(5)))
        );
        let wall_bucket = decorators
            .get(DecoratorStorageAddr::Wall(WallId(5)))
            .unwrap();
        assert!(wall_bucket.removed_ids.contains(&10));
        assert!(wall_bucket.removed_ids.contains(&11));
        let terrain_bucket = decorators.get(DecoratorStorageAddr::Terrain).unwrap();
        assert!(terrain_bucket.removed_ids.contains(&12));
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
        };
        let mut resources = SnapshotLoadResources {
            loaded: Some(loaded),
        };
        let mut walls = InnerWalls::default();
        let mut decorators = DecoratorStorage::default();
        let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
        apply_loaded_snapshot(&mut resources, &mut walls, &mut decorators, &mut terrain);
        assert_eq!(terrain.0.strokes.len(), 1);
        assert_eq!(terrain.0.next_index, 3);
    }
}
