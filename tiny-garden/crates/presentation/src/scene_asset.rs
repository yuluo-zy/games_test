//! 离线景观资产合同与校验；网格只存在表现层，不作为可编辑建筑的领域状态。
//! Blender 将静态展示场景按共享材质编译一次，加载时复用核心 MeshData 检查。
use crate::catalog::CatalogError;
use garden_generation::mesh::MeshData;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneFile {
    schema_version: u32,
    units: String,
    up_axis: String,
    front_axis: String,
    camera: SceneCamera,
    materials: BTreeMap<String, SceneMaterial>,
    batches: BTreeMap<String, SceneBatch>,
}

/// 以米制世界坐标记录的参考视角；运行时镜头变化不写回源资产。
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneCamera {
    pub eye: [f32; 3],
    pub target: [f32; 3],
    pub fov_degrees: f32,
}

/// 基础颜色为线性值；纹理路径相对于 assets，基础色图用 sRGB，N/ORM 用线性。
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneMaterial {
    pub base_color: [f32; 4],
    pub roughness: f32,
    pub metallic: f32,
    pub emissive: [f32; 3],
    pub emissive_strength: f32,
    pub textures: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneBatch {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

/// 已验证的静态展示资源；不包含实体、句柄、输入设备或建筑聚合。
pub struct SceneAsset {
    pub camera: SceneCamera,
    pub materials: BTreeMap<String, SceneMaterial>,
    pub batches: BTreeMap<String, MeshData>,
}

impl SceneAsset {
    /// 拒绝未知版本、坏网格、无材质批次、非有限参数及逃出资产目录的路径。
    pub fn parse(json: &str) -> Result<Self, CatalogError> {
        if json.len() > 64 * 1024 * 1024 {
            return Err(CatalogError("景观编译件超过 64 MiB".into()));
        }
        let file: SceneFile =
            serde_json::from_str(json).map_err(|e| CatalogError(e.to_string()))?;
        if file.schema_version != 1
            || file.units != "meter"
            || file.up_axis != "+Y"
            || file.front_axis != "+Z"
            || file.batches.is_empty()
            || file.batches.len() > 20
            || file.materials.len() > 20
        {
            return Err(CatalogError("景观版本、坐标或批次数无效".into()));
        }
        let camera = &file.camera;
        if camera
            .eye
            .iter()
            .chain(&camera.target)
            .any(|v| !v.is_finite() || v.abs() > 500.0)
            || !(20.0..=100.0).contains(&camera.fov_degrees)
            || camera
                .eye
                .iter()
                .zip(&camera.target)
                .map(|(a, b)| (a - b) * (a - b))
                .sum::<f32>()
                < 0.01
        {
            return Err(CatalogError("景观相机无效".into()));
        }
        for (role, material) in &file.materials {
            if !valid_role(role)
                || material
                    .base_color
                    .iter()
                    .chain(&material.emissive)
                    .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
                || ![material.roughness, material.metallic]
                    .iter()
                    .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
                || !material.emissive_strength.is_finite()
                || !(0.0..=50.0).contains(&material.emissive_strength)
                || material.textures.iter().any(|(slot, path)| {
                    !["base_color", "normal", "orm"].contains(&slot.as_str()) || !safe_texture(path)
                })
            {
                return Err(CatalogError(format!("景观材质无效：{role}")));
            }
        }
        let mut batches = BTreeMap::new();
        let mut triangles = 0;
        for (role, batch) in file.batches {
            if !file.materials.contains_key(&role) {
                return Err(CatalogError("景观批次缺少材质".into()));
            }
            let mesh = MeshData {
                positions: batch.positions,
                normals: batch.normals,
                uvs: batch.uvs,
                indices: batch.indices,
            };
            mesh.validate()
                .map_err(|e| CatalogError(format!("{role}: {e}")))?;
            if mesh.positions.is_empty() || mesh.positions.iter().flatten().any(|v| v.abs() > 500.0)
            {
                return Err(CatalogError("景观网格范围无效".into()));
            }
            triangles += mesh.triangles();
            if triangles > 250000 {
                return Err(CatalogError("景观超过 25 万三角形预算".into()));
            }
            batches.insert(role, mesh);
        }
        Ok(Self {
            camera: file.camera,
            materials: file.materials,
            batches,
        })
    }

    /// 离线网格库的三角形总数；不表示 GPU draw call 或实测帧率。
    pub fn triangles(&self) -> usize {
        self.batches.values().map(MeshData::triangles).sum()
    }
}

fn valid_role(role: &str) -> bool {
    !role.is_empty() && role.chars().all(|c| c.is_ascii_lowercase() || c == '-')
}
fn safe_texture(path: &str) -> bool {
    path.ends_with(".png")
        && !path.contains(['\\', ':'])
        && path
            .split('/')
            .all(|p| !p.is_empty() && p != "." && p != "..")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> serde_json::Value {
        serde_json::json!({"schema_version":1,"units":"meter","up_axis":"+Y","front_axis":"+Z",
            "camera":{"eye":[0,2,5],"target":[0,0,0],"fov_degrees":58},
            "materials":{"stone":{"base_color":[1,1,1,1],"roughness":0.9,"metallic":0,"emissive":[0,0,0],"emissive_strength":0,"textures":{}}},
            "batches":{"stone":{"positions":[[0,0,0],[1,0,0],[0,1,0]],"normals":[[0,0,1],[0,0,1],[0,0,1]],"uvs":[[0,0],[1,0],[0,1]],"indices":[0,1,2]}}})
    }
    #[test]
    fn rejects_bad_assets_before_upload() {
        assert_eq!(
            SceneAsset::parse(&fixture().to_string())
                .unwrap()
                .triangles(),
            1
        );
        let mut missing = fixture();
        missing["materials"] = serde_json::json!({});
        assert!(SceneAsset::parse(&missing.to_string()).is_err());
        let mut invalid = fixture();
        invalid["batches"]["stone"]["indices"] = serde_json::json!([0, 2, 1]);
        assert!(SceneAsset::parse(&invalid.to_string()).is_err());
        let mut path = fixture();
        path["materials"]["stone"]["textures"]["normal"] = "../secret.png".into();
        assert!(SceneAsset::parse(&path.to_string()).is_err());
        let mut camera = fixture();
        camera["camera"]["target"] = camera["camera"]["eye"].clone();
        assert!(SceneAsset::parse(&camera.to_string()).is_err());
    }
    #[test]
    fn polished_scene_obeys_the_runtime_contract() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let text =
            std::fs::read_to_string(root.join("themes/warm-stone/cliff-castle-v8/scene.json"))
                .unwrap();
        let scene = SceneAsset::parse(&text).unwrap();
        assert!(scene.triangles() > 20000 && scene.triangles() <= 250000);
        assert!(scene.batches.contains_key("water") && scene.batches.contains_key("glow"));
        for material in scene.materials.values() {
            for path in material.textures.values() {
                assert!(root.join(path).is_file(), "{path}");
            }
        }
    }
}
