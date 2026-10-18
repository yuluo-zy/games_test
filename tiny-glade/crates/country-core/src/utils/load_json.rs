//! JSON 网格加载与发布前校验。解析复用 Serde，失败返回带路径和属性名的错误。

use anyhow::{Context, ensure};
use serde::Deserialize;
use serde_json::Value;

use crate::render::mesh::{
    ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh, SourceFile, VertexAttributeValues,
};

/// 解析一个属性；向量直接反序列化为 glam 类型，避免中间数组和第二次分配。
pub fn read_attribute_from_json(json: &Value, name: &str) -> anyhow::Result<VertexAttributeValues> {
    let attribute = &json[name];
    let descriptor = &attribute["type"];
    let Some([scalar_type, components]) = descriptor.as_array().map(Vec::as_slice) else {
        anyhow::bail!("属性 {name} 的 type 必须是 [标量类型, 分量数]");
    };
    let buffer = attribute
        .get("buffer")
        .with_context(|| format!("顶点属性 {name} 缺少 buffer"))?;
    let values = match (scalar_type.as_str(), components.as_u64()) {
        (Some("int"), Some(1)) => VertexAttributeValues::Int(Vec::deserialize(buffer)?),
        (Some("float"), Some(1)) => VertexAttributeValues::Float(Vec::deserialize(buffer)?),
        (Some("float"), Some(2)) => VertexAttributeValues::Float2(Vec::deserialize(buffer)?),
        (Some("float"), Some(3)) => VertexAttributeValues::Float3(Vec::deserialize(buffer)?),
        (Some("float"), Some(4)) => VertexAttributeValues::Float4(Vec::deserialize(buffer)?),
        _ => anyhow::bail!(
            "load_json_as_mesh: unrecognized attrib '{name}' with type '{descriptor}'"
        ),
    };
    ensure!(values.array_length() != 0, "顶点属性 {name} 不能为空");
    let finite = match &values {
        VertexAttributeValues::Int(_) => true,
        VertexAttributeValues::Float(values) => values.iter().all(|value| value.is_finite()),
        VertexAttributeValues::Float2(values) => values.iter().all(|value| value.is_finite()),
        VertexAttributeValues::Float3(values) => values.iter().all(|value| value.is_finite()),
        VertexAttributeValues::Float4(values) => values.iter().all(|value| value.is_finite()),
    };
    ensure!(finite, "顶点属性 {name} 含有非有限数值");
    let valid_type = match name {
        ATTR_POSITION | ATTR_NORMAL => matches!(&values, VertexAttributeValues::Float3(_)),
        ATTR_UV => matches!(&values, VertexAttributeValues::Float2(_)),
        ATTR_COLOR => matches!(
            &values,
            VertexAttributeValues::Float3(_) | VertexAttributeValues::Float4(_)
        ),
        _ => true,
    };
    ensure!(valid_type, "顶点属性 {name} 类型不符合网格约定");
    Ok(values)
}

/// 构造完整的可绘制网格。所有检查通过后才返回，调用方可安全替换旧资产。
fn try_mesh_from_json(json: &Value, path: &str) -> anyhow::Result<Mesh> {
    let indices: Vec<u32> = Vec::deserialize(&json["indices"]["buffer"])
        .context("网格 indices.buffer 必须为 u32 数组")?;
    let names = json["attributes"]
        .as_array()
        .context("网格 attributes 必须为属性名数组")?;
    let mut mesh = Mesh::new("load_json_as_mesh");
    for name in names {
        let name = name.as_str().context("网格属性名必须为字符串")?;
        ensure!(!mesh.has_attribute(name), "网格属性 {name} 重复声明");
        let values = read_attribute_from_json(json, name)
            .with_context(|| format!("解析网格属性 {name} 失败"))?;
        mesh.set_attribute(name, values);
    }
    let vertex_count = mesh
        .attributes
        .get(ATTR_POSITION)
        .context("网格缺少 Vertex_Position 属性")?
        .array_length();
    for (name, values) in &mesh.attributes {
        ensure!(
            values.array_length() == vertex_count,
            "属性 {name} 的顶点数量与 Vertex_Position 不一致"
        );
    }
    ensure!(
        indices.iter().all(|&index| (index as usize) < vertex_count),
        "顶点属性长度不足以覆盖网格索引"
    );
    mesh.set_indices(indices);
    mesh.set_source_file(SourceFile {
        path: path.to_owned(),
        stamp: 0,
    });
    Ok(mesh)
}

/// 启动期必需资产的便捷入口；需要可恢复错误时使用 try_load_json_as_mesh。
pub fn load_json_as_mesh(path: &str) -> Mesh {
    try_load_json_as_mesh(path).unwrap()
}

/// 读文件、解析和校验失败都返回 Err，不中断资产热重载。
pub fn try_load_json_as_mesh(path: &str) -> anyhow::Result<Mesh> {
    let text = std::fs::read_to_string(path).with_context(|| format!("读取网格失败：{path}"))?;
    let json: Value =
        serde_json::from_str(&text).with_context(|| format!("解析网格失败：{path}"))?;
    try_mesh_from_json(&json, path).with_context(|| format!("校验网格失败：{path}"))
}

#[cfg(test)]
fn mesh_from_json(json: &Value, path: &str) -> Mesh {
    try_mesh_from_json(json, path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec2, Vec3};
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "indices": {"buffer": [0, 1, 2]},
            "attributes": ["Vertex_Position", "Vertex_UV", "Tag", "Weight", "Colour"],
            "Vertex_Position": {"type": ["float", 3], "buffer": [[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,1.0,0.5]]},
            "Vertex_UV": {"type": ["float", 2], "buffer": [[0.0,0.0],[1.0,0.0],[0.0,1.0]]},
            "Tag": {"type": ["int", 1], "buffer": [7, -1, 3]},
            "Weight": {"type": ["float", 1], "buffer": [0.5, 1.0, 2.0]},
            "Colour": {"type": ["float", 4], "buffer": [[1.0,0.0,0.0,1.0],[0.0,1.0,0.0,1.0],[0.0,0.0,1.0,1.0]]}
        })
    }

    #[test]
    fn builds_every_attribute_kind() {
        let mesh = mesh_from_json(&sample(), "m.json");
        assert_eq!(mesh.label, "load_json_as_mesh");
        assert_eq!(mesh.indices, [0, 1, 2]);
        assert_eq!(
            mesh.attributes["Tag"],
            VertexAttributeValues::Int(vec![7, -1, 3])
        );
        assert_eq!(
            mesh.attributes["Weight"],
            VertexAttributeValues::Float(vec![0.5, 1.0, 2.0])
        );
        assert_eq!(
            mesh.positions_without_indices()[2],
            Vec3::new(0.0, 1.0, 0.5)
        );
        assert_eq!(mesh.uvs_without_indices()[1], Vec2::new(1.0, 0.0));
        assert_eq!(
            mesh.attributes["Colour"].array_length(),
            3,
            "float4 attribute"
        );
    }

    #[test]
    fn unknown_type_is_an_err_with_the_type_text() {
        let json = json!({"Bad": {"type": ["float", 5], "buffer": []}});
        let err = read_attribute_from_json(&json, "Bad").unwrap_err();
        assert_eq!(
            err.to_string(),
            "load_json_as_mesh: unrecognized attrib 'Bad' with type '[\"float\",5]'"
        );
    }

    #[test]
    fn unknown_type_fails_before_the_buffer_is_read() {
        let json = json!({"Bad": {"type": ["float", 5]}});
        assert!(read_attribute_from_json(&json, "Bad").is_err());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn unknown_type_in_a_mesh_panics_through_unwrap() {
        let mut json = sample();
        json["Tag"]["type"] = json!(["float", 9]);
        mesh_from_json(&json, "m.json");
    }

    #[test]
    fn loaded_meshes_remember_their_source_path() {
        let mesh = mesh_from_json(&sample(), "assets/meshes/rock.json");
        let file = mesh.source_file.as_ref().expect("source_file is set");
        assert_eq!(file.path, "assets/meshes/rock.json");
        assert_eq!(file.stamp, 0);
    }

    #[test]
    #[should_panic(expected = "不能为空")]
    fn an_empty_attribute_buffer_panics_even_with_empty_indices() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([]);
        json["Tag"]["buffer"] = json!([]);
        mesh_from_json(&json, "m.json");
    }

    #[test]
    fn extra_float_components_are_rejected() {
        let json = json!({"P": {"type": ["float", 3], "buffer": [[1.0, 2.0, 3.0, 99.0]]}});
        assert!(read_attribute_from_json(&json, "P").is_err());
    }

    #[test]
    fn too_few_float_components_returns_an_error() {
        let json = json!({"P": {"type": ["float", 2], "buffer": [[1.0]]}});
        assert!(read_attribute_from_json(&json, "P").is_err());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn a_non_string_attribute_name_panics_before_any_attribute_is_read() {
        let mut json = sample();
        json["attributes"] = json!(["Bad", 7]);
        json["Bad"] = json!({"type": ["float", 9], "buffer": []});
        mesh_from_json(&json, "m.json");
    }

    #[test]
    #[should_panic(expected = "长度不足以覆盖网格索引")]
    fn index_beyond_attribute_length_asserts() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([0, 1, 3]);
        mesh_from_json(&json, "m.json");
    }

    #[test]
    fn empty_indices_are_allowed() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([]);
        assert!(mesh_from_json(&json, "m.json").indices.is_empty());
    }

    #[test]
    #[should_panic(expected = "读取网格失败：/nonexistent/mesh.json")]
    fn missing_file_panics_with_the_path() {
        load_json_as_mesh("/nonexistent/mesh.json");
    }

    #[test]
    fn loads_from_disk() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("mesh.json");
        std::fs::write(&path, sample().to_string()).unwrap();
        let mesh = load_json_as_mesh(path.to_str().unwrap());
        assert_eq!(mesh.indices.len(), 3);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn invalid_semantic_attributes_are_rejected_before_publication() {
        let mut json = sample();
        json[ATTR_POSITION] = json!({"type": ["float", 2], "buffer": [[0,0],[1,0],[0,1]]});
        assert!(try_mesh_from_json(&json, "invalid.json").is_err());
        json["attributes"] = json!(["Tag"]);
        assert!(try_mesh_from_json(&json, "missing.json").is_err());
        json = sample();
        json["attributes"] = json!(["Vertex_Position", "Vertex_Position"]);
        assert!(try_mesh_from_json(&json, "duplicate.json").is_err());
        json = sample();
        json["Weight"]["buffer"] = json!([1.0, 2.0, 3.0, 4.0]);
        assert!(try_mesh_from_json(&json, "length.json").is_err());
    }

    #[test]
    fn malformed_type_descriptors_and_overflowing_floats_are_errors() {
        let json = json!({"P": {"type": ["float", 3, "extra"], "buffer": [[1,2,3]]}});
        assert!(read_attribute_from_json(&json, "P").is_err());
        let json = json!({"P": {"type": ["float", 1], "buffer": [1e100]}});
        assert!(read_attribute_from_json(&json, "P").is_err());
    }

    #[test]
    fn failed_hot_reload_preserves_the_previous_mesh_and_dirty_list() {
        use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};

        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("mesh.json");
        let previous = mesh_from_json(&sample(), path.to_str().unwrap());
        let mut library = AssetMeshLibrary::new();
        let handle = library.add(MeshAsset::new(previous, None));
        let mut invalid = sample();
        invalid[ATTR_POSITION] = json!({"type":["float",2], "buffer":[[0,0],[1,0],[0,1]]});
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(library.try_reload_mesh_src(&path).is_err());
        assert_eq!(
            library
                .get(handle)
                .unwrap()
                .positions_without_indices()
                .len(),
            3
        );
        assert!(library.modified_meshes.is_empty());
    }
}
