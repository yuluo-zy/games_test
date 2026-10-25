from pathlib import Path
import json

work = Path(__file__).resolve().parent
s = (work / 'base/snapshot.rs').read_text()
s = s.replace('        decorator::DecoratorInfo,\n', '')
s = s.replace('    stairs::stairs_state::{StairsSnapshot, StairsState},', '    stairs::{stairs_removed_supports::StairsRemovedSupports, stairs_state::{StairsSnapshot, StairsState}},')
s = s.replace('use std::sync::Arc;', 'use std::{io::{Cursor, Write}, ops::Range, sync::Arc};')
s = s.replace('#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct SnapshotTreesEntry', '#[derive(Clone, Debug, PartialEq)]\npub struct SnapshotTreesEntry', 1)
marker = '\n/// 快照 Trees 节收集器'
at = s.index(marker)
s = s[:at] + '''
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
''' + s[at:]
a = s.index('pub const SNAPSHOT_SECTION_ORDER:')
b = s.index('\n];', a) + len('\n];')
s = s[:a] + '''pub const SNAPSHOT_SECTION_ORDER: &[&str] = &[
    "Version", "Walls", "Roofs", "Decorators", "Raster", "WallColors", "GateColors",
    "HalftimberColors", "Terrain", "TerrainHeightRange", "Trees", "AutoClutter",
    "Stairs", "StairsRemovedSupports",
];
/// B snapshot format version, independent of the enclosing game-save version.
pub const SNAPSHOT_FORMAT_VERSION: i32 = 2;''' + s[b:]
marker = '    pub section_values: Vec<(&\'static str, Value)>,\n'
s = s.replace(marker, marker + '''    /// Raw terrain bytes captured from the actual height-buffer owner. These
    /// are compressed into the final Terrain chunk, not the Terrain JSON field.
    pub terrain_data: Option<Vec<u8>>,
    /// The actual height resource's range; no bounds are inferred from strokes.
    pub terrain_height_range: Option<Range<f32>>,
    /// Actual removed-support set, including the legitimate empty set.
    pub removed_supports: Option<StairsRemovedSupports>,
''', 1)
s = s.replace('            section_values: Vec::new(),\n', '            section_values: Vec::new(),\n            terrain_data: None,\n            terrain_height_range: None,\n            removed_supports: None,\n', 1)
a = s.index('    /// 按 `SNAPSHOT_SECTION_ORDER` 节序产出 JSON 节载荷。')
b = s.index('\n}\n\n/// 快照存档数据', a)
s = s[:a] + '''    /// Supplies the three captures absent from the older collector signature.
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
        let range = self.terrain_height_range.as_ref()
            .ok_or_else(|| anyhow::anyhow!("snapshot capture has no TerrainHeightRange"))?;
        let removed = self.removed_supports.as_ref()
            .ok_or_else(|| anyhow::anyhow!("snapshot capture has no StairsRemovedSupports"))?;
        let mut walls: Vec<_> = self.walls.iter().collect();
        walls.sort_unstable_by_key(|(id, _)| id.0);
        let mut roofs: Vec<_> = self.roofs.iter().collect();
        roofs.sort_unstable_by_key(|roof| roof.wall_id().0);
        let mut decorators: Vec<_> = self.decorators.iter().collect();
        decorators.sort_unstable_by_key(|(addr, _)| addr_sort_key(*addr));
        let states = decorators.into_iter().map(|(addr, bucket)| {
            let state = bucket.states.iter().map(|(id, state)| {
                Ok(json!([id.0, decorator_history_state_json(state)?]))
            }).collect::<anyhow::Result<Vec<Value>>>()?;
            Ok(json!([decorator_storage_addr_json(*addr), {"state": state}]))
        }).collect::<anyhow::Result<Vec<Value>>>()?;
        let mut trees: Vec<_> = self.trees.iter().collect();
        trees.sort_unstable_by_key(|entry| entry.index.0);
        let mut ids = self.ids.clone();
        ids.sort_unstable();
        let mut sections = vec![
            ("Version", json!(SNAPSHOT_FORMAT_VERSION)),
            ("Walls", serde_json::to_value(walls)?),
            ("Roofs", serde_json::to_value(roofs)?),
            ("Decorators", json!({"states": states})),
            ("Raster", json!(self.raster.iter().map(|(name, _)| *name).collect::<Vec<_>>())),
            ("WallColors", serde_json::to_value(&self.wall_colors)?),
            ("GateColors", serde_json::to_value(&self.gate_colors)?),
            ("HalftimberColors", serde_json::to_value(&self.halftimber_colors)?),
            ("Terrain", json!({"strokes": self.terrain.0.strokes,
                "next_stroke_id": self.terrain.0.next_index as usize})),
            ("TerrainHeightRange", serde_json::to_value(range)?),
            ("Trees", serde_json::to_value(trees)?),
            ("AutoClutter", json!(ids)),
            ("Stairs", json!({"edges": self.stairs.edges(),
                "checksum": self.stairs.cached_checksum()})),
            ("StairsRemovedSupports", json!({"edges": removed.edges.keys()
                .map(|edge| edge.nodes()).collect::<Vec<_>>()})),
        ];
        // Old extension callers may add independent keys, but cannot replace a
        // captured native domain or hide missing data behind injected JSON.
        for (key, value) in &self.section_values {
            anyhow::ensure!(!sections.iter().any(|(name, _)| name == key),
                "snapshot section extension duplicates {key}");
            sections.push((*key, value.clone()));
        }
        Ok(sections)
    }

    /// Compatibility entry for callers requiring an infallible projection.
    /// File writers should call encode_game_snapshot and propagate its Result.
    pub fn to_section_values(&self) -> Vec<(&'static str, Value)> {
        self.try_to_section_values().expect("snapshot domain serialization failed")
    }

    pub fn try_to_json(&self) -> anyhow::Result<Value> {
        let object = self.try_to_section_values()?.into_iter()
            .map(|(key, value)| (key.to_owned(), value)).collect();
        Ok(Value::Object(object))
    }
''' + s[b:]
a = s.index('    /// 构建完整快照 JSON 文档')
b = s.index('\n}\n\n/// 解析后的快照数据', a)
s = s[:a] + '''    /// JSON metadata only; the saved file also needs the compressed chunks.
    pub fn try_to_json(&self) -> anyhow::Result<Value> {
        self.domain.try_to_json()
    }
    pub fn to_json(&self) -> Value {
        self.try_to_json().expect("snapshot domain serialization failed")
    }
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        encode_game_snapshot(&self.domain)
    }
''' + s[b:]
a = s.index('pub fn capture_game_snapshot(domain: SnapshotDomainData) -> Vec<u8> {')
b = s.index('\n/// Stairs 快照采集', a)
s = s[:a] + '''pub fn capture_game_snapshot(domain: SnapshotDomainData) -> Vec<u8> {
    encode_game_snapshot(&domain).expect("snapshot serialization failed (native unwrap_failed)")
}

/// The single file encoder for both explicit and automatic snapshot saves.
/// B@1417a31e2: Snapshot JSON chunk; @1417a35e7: each named raster chunk;
/// @1417a3a38: Terrain chunk. Existing motomoto owns the chunk framing.
pub fn encode_game_snapshot(domain: &SnapshotDomainData) -> anyhow::Result<Vec<u8>> {
    let terrain = domain.terrain_data.as_deref()
        .ok_or_else(|| anyhow::anyhow!("snapshot capture has no raw Terrain data"))?;
    let document = domain.try_to_json()?;
    let mut writer = motomoto::Writer::new(Cursor::new(
        Vec::with_capacity(SNAPSHOT_JSON_BUFFER_CAPACITY)));
    {
        let mut chunk = writer.add_chunk("Snapshot")?;
        serde_json::to_writer(&mut chunk, &document)?;
        chunk.finish()?;
    }
    for (name, bytes) in &domain.raster {
        let mut chunk = writer.add_chunk(motomoto::tag_from_str_trunc(name))?;
        // Raster chunks alone prepend the uncompressed byte count.
        chunk.write_all(&(bytes.len() as u64).to_le_bytes())?;
        let mut compressed = flate2::write::DeflateEncoder::new(
            &mut chunk, flate2::Compression::new(6));
        compressed.write_all(bytes)?;
        compressed.finish()?;
        chunk.finish()?;
    }
    {
        let mut chunk = writer.add_chunk("Terrain")?;
        let mut compressed = flate2::write::DeflateEncoder::new(
            &mut chunk, flate2::Compression::new(6));
        compressed.write_all(terrain)?;
        compressed.finish()?;
        chunk.finish()?;
    }
    Ok(writer.finish()?.into_inner())
}
''' + s[b:]
# The older JSON-only loader is retained as a compatibility surface. It cannot
# turn the old partial DTO into a real HistoryState: reject rather than fabricate.
a = s.index('    pub fn into_storage_snapshot(&self) -> DecoratorStorageSnapshot {')
b = s.index('\n}\n\n/// 每地址的装饰状态列表', a)
s = s[:a] + '''    pub fn into_storage_snapshot(&self) -> anyhow::Result<DecoratorStorageSnapshot> {
        anyhow::bail!("legacy DecoratorInfo JSON is not the native DecoratorHistoryState schema")
    }
''' + s[b:]
s = s.replace('                    let snapshot = section.into_storage_snapshot();\n                    let count = snapshot.states.len();\n                    decorators.restore_snapshot(&snapshot);\n                    log::trace!("apply Decorators: restore {count} snapshot entries");', '''                    match section.into_storage_snapshot() {
                        Ok(snapshot) => decorators.restore_snapshot(&snapshot),
                        Err(error) => log::error!("apply Decorators: {error}"),
                    }''')
s = s.replace('    #[serde(default)]\n    pub next_stroke_index: u32,', '    #[serde(default, alias = "next_stroke_id")]\n    pub next_stroke_index: u32,')
# Update existing test literals to the canonical bucket; tests exercising the
# invented default-info conversion are replaced with an explicit rejection.
import re
s = re.sub(r'DecoratorAddrSnapshot \{\s+encoded_id: 0,\s+addr: DecoratorStorageAddr::(?:Wall\(WallId\(\d+\)\)|Terrain|Stair),\s+state: 0,\s+info: DecoratorInfo::default\(\),\s+\}', 'DecoratorAddrSnapshot { states: Vec::new() }', s)
a = s.index('    #[test]\n    fn decorators_json_into_storage_snapshot_and_apply_restore()')
b = s.index('    #[test]\n    fn parse_wall_section', a)
s = s[:a] + '''    #[test]
    fn legacy_decorator_json_does_not_fabricate_default_info() {
        let section: DecoratorsSectionJson = serde_json::from_value(json!({"states": []})).unwrap();
        assert!(section.into_storage_snapshot().is_err());
    }

''' + s[b:]
# New required fields in all remaining direct domain test literals.
s = s.replace('            section_values: vec![("Version", json!(27))],', '            section_values: Vec::new(),\n            terrain_data: Some(vec![0, 1, 2, 3]),\n            terrain_height_range: Some(-2.0..4.0),\n            removed_supports: Some(StairsRemovedSupports::default()),')
s = s.replace('            section_values: Vec::new(),\n        }\n        .sorted();', '            section_values: Vec::new(),\n            terrain_data: Some(Vec::new()),\n            terrain_height_range: Some(0.0..0.0),\n            removed_supports: Some(StairsRemovedSupports::default()),\n        }\n        .sorted();')
s = s.replace('        let parsed: Value = serde_json::from_slice(&bytes).unwrap();', '        let first = motomoto::Reader::new(&bytes).iter().next().unwrap().unwrap();\n        assert_eq!(first.tag, motomoto::tag_from_str("Snapshot"));\n        let parsed: Value = serde_json::from_slice(first.data).unwrap();', 1)
s = s.replace('        assert_eq!(object["Version"], json!(27));', '        assert_eq!(object["Version"], json!(2));', 1)

names = [row['value'] for row in json.loads((work/'clutter-mesh-string-bindings.json').read_text())['bindings']]
helpers = '''
// Native serde adapters over the canonical owned HistoryState projection.
// They do not decode process pointers or manufacture DecoratorInfo values.
fn decorator_storage_addr_json(addr: DecoratorStorageAddr) -> Value {
    match addr {
        DecoratorStorageAddr::Wall(id) => serde_json::json!({"Wall": id.0}),
        DecoratorStorageAddr::Terrain => Value::String("Terrain".into()),
        DecoratorStorageAddr::Stair => Value::String("Stairs".into()),
    }
}

const CLUTTER_MESH_NAMES: [&str; 56] = __NAMES__;

fn decorator_history_state_json(
    state: &crate::resources::history::decorator_edit::DecoratorHistoryState,
) -> anyhow::Result<Value> {
    use serde_json::json;
    let b = state.to_bytes();
    let u32_at = |at| u32::from_le_bytes(b[at..at+4].try_into().unwrap());
    let u64_at = |at| u64::from_le_bytes(b[at..at+8].try_into().unwrap());
    let f = |at| f32::from_bits(u32_at(at));
    let mut dst = serde_json::Map::new();
    match b[0x22] {
        0 | 1 => {
            let mut anchor = serde_json::Map::new();
            if f(0x10) != 0.0 { anchor.insert("wall_height_relative_y".into(), json!(f(0x10))); }
            let xyz = [f(0x14), f(0x18), f(0x1c)];
            if xyz.iter().any(|v| *v != 0.0) { anchor.insert("anchor_ws".into(), json!(xyz)); }
            anchor.insert("corner".into(), json!(b[0x20]));
            if u64_at(8) != 0 { anchor.insert("platform".into(), json!(u64_at(8))); }
            if b[0x21] != 0 { anchor.insert("is_bottom_door".into(), json!(true)); }
            if b[0x22] == 0 { anchor.insert("side".into(), json!("Inner")); }
            dst.insert("Wall".into(), json!({"coord": {"x": f(0), "y": f(4)}, "anchor": anchor}));
        }
        2 | 3 => {
            let mut attachment = serde_json::Map::new();
            let (variant, position, yaw) = if b[0x22] == 2 {
                ("Roof", "mesh_uv", "relative_yaw_radians")
            } else { ("Terrain", "pos_xz", "yaw_radians") };
            attachment.insert(position.into(), json!([f(0), f(4)]));
            if f(8) != 0.0 { attachment.insert(yaw.into(), json!(f(8))); }
            dst.insert(variant.into(), Value::Object(attachment));
        }
        4 => {
            let mut attachment = serde_json::Map::new();
            attachment.insert("graph_edge".into(), json!([u32_at(0), u32_at(4)]));
            attachment.insert("uv".into(), json!([f(8), f(0xc)]));
            if f(0x10) != 0.0 { attachment.insert("yaw_radians".into(), json!(f(0x10))); }
            dst.insert("Stair".into(), Value::Object(attachment));
        }
        other => anyhow::bail!("invalid DecoratorDst niche {other}"),
    }
    let ty = match b[0x4a] {
        index @ 0..=55 => json!({"Clutter": {"ty": CLUTTER_MESH_NAMES[index as usize]}}),
        56 => json!("CottageWindow"), 57 => json!("GothicWindow"),
        58 => json!("ArrowSlit"), 59 => json!("Chimney"),
        60 => json!("Lantern"), 61 => json!("Flag"), 63 => json!("Stairs"),
        other => anyhow::bail!("invalid DecoratorType {other}"),
    };
    let mut value = serde_json::Map::new();
    value.insert("dst".into(), Value::Object(dst));
    value.insert("ty".into(), ty);
    value.insert("rank".into(), json!({"x": b[0x48]}));
    value.insert("seed".into(), json!(u32_at(0x2c)));
    for (name, offset) in [("color_id", 0x28), ("window_light_color", 0x34),
        ("ladder_color", 0x40), ("magic_color", 0x44)] {
        if u32_at(offset) != 0 { value.insert(name.into(), json!(u32_at(offset))); }
    }
    if b[0x49] != 0 { value.insert("clv".into(), json!(b[0x49])); }
    for (name, offset, default) in [("h_offset", 0x30, 0.0),
        ("lantern_brightness", 0x38, 1.0), ("stair_width", 0x3c, 1.0)] {
        if f(offset) != default { value.insert(name.into(), json!(f(offset))); }
    }
    if b[0x4c] == 0 { value.insert("allow_connection".into(), json!(false)); }
    if b[0x4d] == 0 { value.insert("chimney_smoke".into(), json!(false)); }
    if b[0x4b] != 0 {
        let railing = match b[0x4b] {
            1 => "Low", 2 => "High", 3 => "Wooden",
            other => anyhow::bail!("invalid RailingType {other}"),
        };
        value.insert("railing".into(), json!(railing));
    }
    if b[0x4e] != 0 { value.insert("stairs_supports".into(), json!("None")); }
    Ok(Value::Object(value))
}

'''.replace('__NAMES__', json.dumps(names))
at = s.index('/// 快照存档数据')
s = s[:at] + helpers + s[at:]
s = s.replace('//! 快照 schema 为 **JSON + deflate**（非 rkyv/ron）：节序', '//! 快照 schema 为 **Motomoto chunks + JSON + deflate**：元数据字段')
# Make the canonical file-level description truthful without expanding the load work.
s = s.replace('//! `Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain →\n//! Trees → Raster`（O：0x1417a6e50 的节跳表 @0x142cdca80/98/b0）。Terrain 载荷\n//! 用 flate2-1.0.24 deflate（O："decompressing terrain data" @0x142cdc800）。', '//! 共十四项，格式 Version=2。Snapshot chunk 保存 JSON；之后为命名 Raster\n//! chunks（u64 原长 + deflate6）和 Terrain chunk（仅 deflate6）。\n//! 旧 JSON-only 加载入口仍是局部兼容面，不代表完整 chunk 解码/世界恢复。')
(work/'candidate/snapshot.rs').write_text(s)
