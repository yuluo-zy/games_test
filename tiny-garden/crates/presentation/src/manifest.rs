//! Early asset gates: schema, ownership metadata, safe paths and GLB budgets.
//! Not a replacement for the Khronos validator or visual/engine acceptance.
use crate::catalog::CatalogError;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssetStatus {
    Planned,
    Blockout,
    Approved,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetRecord {
    pub id: String,
    pub group: String,
    pub status: AssetStatus,
    pub file: Option<String>,
    pub attachment: String,
    pub fit_mode: String,
    pub max_triangles: usize,
    pub max_materials: usize,
    pub author: String,
    pub license: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetManifest {
    pub schema_version: u32,
    pub units: String,
    pub up_axis: String,
    pub front_axis: String,
    pub records: Vec<AssetRecord>,
}
#[derive(Debug, Default)]
pub struct AssetSummary {
    pub planned: usize,
    pub blockout: usize,
    pub approved: usize,
    pub triangles: usize,
}

impl AssetManifest {
    pub fn parse(json: &str) -> Result<Self, CatalogError> {
        let manifest: Self = serde_json::from_str(json).map_err(|e| CatalogError(e.to_string()))?;
        if manifest.schema_version != 1
            || manifest.units != "meter"
            || manifest.up_axis != "+Y"
            || manifest.front_axis != "+Z"
        {
            return Err(CatalogError("unsupported asset schema/axes/units".into()));
        }
        let mut ids = BTreeSet::new();
        for r in &manifest.records {
            if r.id.is_empty()
                || !r
                    .id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                || !ids.insert(&r.id)
                || r.max_triangles == 0
                || r.max_materials == 0
                || r.author.is_empty()
                || r.license.is_empty()
                || !["wall-face", "roof", "floor", "ground"].contains(&r.attachment.as_str())
                || !["fixed", "fixed-corners-repeat-center", "repeat-center"]
                    .contains(&r.fit_mode.as_str())
            {
                return Err(CatalogError(format!("invalid asset metadata: {}", r.id)));
            }
            let group = r.group.strip_prefix('A').and_then(|s| s.parse::<u8>().ok());
            if !group.is_some_and(|n| (1..=24).contains(&n)) {
                return Err(CatalogError("invalid art group".into()));
            }
            if let Some(file) = &r.file {
                if !safe_path(file) {
                    return Err(CatalogError(format!("unsafe asset path: {file}")));
                }
            } else if r.status != AssetStatus::Planned {
                return Err(CatalogError(format!("missing exported file: {}", r.id)));
            }
            if r.status == AssetStatus::Approved
                && (r.author == "unassigned" || r.license == "pending")
            {
                return Err(CatalogError(format!(
                    "approved asset has unresolved ownership: {}",
                    r.id
                )));
            }
        }
        Ok(manifest)
    }
    pub fn check(&self, assets_root: &Path, strict: bool) -> Result<AssetSummary, CatalogError> {
        let root = assets_root
            .canonicalize()
            .map_err(|e| CatalogError(e.to_string()))?;
        let mut summary = AssetSummary::default();
        for r in &self.records {
            match r.status {
                AssetStatus::Planned => summary.planned += 1,
                AssetStatus::Blockout => summary.blockout += 1,
                AssetStatus::Approved => summary.approved += 1,
            };
            if strict && r.status != AssetStatus::Approved {
                return Err(CatalogError(format!(
                    "production gate: {} is {:?}",
                    r.id, r.status
                )));
            }
            if r.status == AssetStatus::Planned {
                continue;
            }
            let file = root
                .join(r.file.as_ref().unwrap())
                .canonicalize()
                .map_err(|e| CatalogError(format!("{}: {e}", r.id)))?;
            if !file.starts_with(&root) {
                return Err(CatalogError("asset escapes root through a link".into()));
            }
            let size = fs::metadata(&file)
                .map_err(|e| CatalogError(e.to_string()))?
                .len();
            if size > 16 * 1024 * 1024 {
                return Err(CatalogError("GLB exceeds initial 16MiB file gate".into()));
            }
            let bytes = fs::read(file).map_err(|e| CatalogError(e.to_string()))?;
            let (triangles, materials) = glb_budget(&bytes)?;
            if triangles > r.max_triangles || materials > r.max_materials {
                return Err(CatalogError(format!(
                    "budget exceeded: {} ({triangles} triangles, {materials} materials)",
                    r.id
                )));
            }
            summary.triangles += triangles;
        }
        Ok(summary)
    }
}
fn safe_path(file: &str) -> bool {
    !file.contains('\\')
        && !file.contains(':')
        && file.ends_with(".glb")
        && Path::new(file)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
pub fn glb_budget(bytes: &[u8]) -> Result<(usize, usize), CatalogError> {
    let fail = || CatalogError("malformed/non-self-contained/static-triangle GLB".into());
    let word = |offset: usize| {
        bytes
            .get(offset..offset + 4)
            .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
    };
    if bytes.get(..4) != Some(b"glTF")
        || word(4) != Some(2)
        || word(8).map(|n| n as usize) != Some(bytes.len())
    {
        return Err(fail());
    }
    let mut offset = 12;
    let mut json = None;
    let mut bins = 0;
    while offset < bytes.len() {
        let length = word(offset).ok_or_else(fail)? as usize;
        let kind = word(offset + 4).ok_or_else(fail)?;
        let end = offset
            .checked_add(8)
            .and_then(|n| n.checked_add(length))
            .ok_or_else(fail)?;
        if !length.is_multiple_of(4) || end > bytes.len() {
            return Err(fail());
        }
        if kind == 0x4e4f534a {
            if json.is_some() || offset != 12 {
                return Err(fail());
            }
            json =
                Some(serde_json::from_slice::<Value>(&bytes[offset + 8..end]).map_err(|_| fail())?);
        } else if kind == 0x004e4942 {
            bins += 1;
        } else {
            return Err(fail());
        }
        offset = end;
    }
    let document = json.ok_or_else(fail)?;
    if bins != 1
        || document["asset"]["version"] != "2.0"
        || document["animations"]
            .as_array()
            .is_some_and(|a| !a.is_empty())
        || document["images"]
            .as_array()
            .is_some_and(|a| a.iter().any(|i| i.get("uri").is_some()))
        || document["buffers"]
            .as_array()
            .is_some_and(|a| a.iter().any(|i| i.get("uri").is_some()))
    {
        return Err(fail());
    }
    let accessors = document["accessors"].as_array().ok_or_else(fail)?;
    let mut triangles = 0;
    for mesh in document["meshes"].as_array().ok_or_else(fail)? {
        for primitive in mesh["primitives"].as_array().ok_or_else(fail)? {
            if primitive.get("mode").and_then(Value::as_u64).unwrap_or(4) != 4 {
                return Err(fail());
            }
            let index = primitive
                .get("indices")
                .or_else(|| primitive["attributes"].get("POSITION"))
                .and_then(Value::as_u64)
                .ok_or_else(fail)? as usize;
            let count = accessors
                .get(index)
                .and_then(|a| a["count"].as_u64())
                .ok_or_else(fail)? as usize;
            if count == 0 || !count.is_multiple_of(3) {
                return Err(fail());
            }
            triangles += count / 3;
        }
    }
    let materials = document["materials"].as_array().map_or(0, Vec::len);
    Ok((triangles, materials))
}

#[cfg(test)]
mod tests {
    use super::*;
    const MANIFEST: &str = include_str!("../../../assets/manifests/models.json");
    #[test]
    fn planned_manifest_is_honest_and_paths_are_safe() {
        let manifest = AssetManifest::parse(MANIFEST).unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let summary = manifest.check(&root, false).unwrap();
        assert_eq!(summary.approved, 0);
        assert_eq!(summary.planned, 22);
        assert_eq!(summary.blockout, 4);
        assert_eq!(summary.triangles, 1468);
        assert!(manifest.check(&root, true).is_err());
        for path in ["../a.glb", "/a.glb", "D:/a.glb", "a\\b.glb"] {
            assert!(!safe_path(path));
        }
    }
    #[test]
    fn rejects_duplicate_ids_and_missing_approved_provenance() {
        let mut value: Value = serde_json::from_str(MANIFEST).unwrap();
        let first = value["records"][0].clone();
        value["records"].as_array_mut().unwrap().push(first);
        assert!(AssetManifest::parse(&value.to_string()).is_err());
        let mut value: Value = serde_json::from_str(MANIFEST).unwrap();
        value["records"][0]["status"] = "approved".into();
        value["records"][0]["file"] = "a.glb".into();
        value["records"][0]["license"] = "pending".into();
        assert!(AssetManifest::parse(&value.to_string()).is_err());
    }
    #[test]
    fn glb_header_counts_and_external_references_are_checked() {
        fn fixture(external: bool) -> Vec<u8> {
            let mut doc = serde_json::json!({"asset":{"version":"2.0"},"accessors":[{"count":6}],"meshes":[{"primitives":[{"indices":0}]}],"materials":[{}]});
            if external {
                doc["images"] = serde_json::json!([{"uri":"external.png"}]);
            }
            let mut json = serde_json::to_vec(&doc).unwrap();
            while !json.len().is_multiple_of(4) {
                json.push(b' ');
            }
            let length = 12 + 8 + json.len() + 8 + 4;
            let mut bytes = b"glTF".to_vec();
            bytes.extend(2u32.to_le_bytes());
            bytes.extend((length as u32).to_le_bytes());
            bytes.extend((json.len() as u32).to_le_bytes());
            bytes.extend(0x4e4f534au32.to_le_bytes());
            bytes.extend(json);
            bytes.extend(4u32.to_le_bytes());
            bytes.extend(0x004e4942u32.to_le_bytes());
            bytes.extend([0; 4]);
            bytes
        }
        assert_eq!(glb_budget(&fixture(false)).unwrap(), (2, 1));
        assert!(glb_budget(&fixture(true)).is_err());
        assert!(glb_budget(b"glTF").is_err());
    }
}
