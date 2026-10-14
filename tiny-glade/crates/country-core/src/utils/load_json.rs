//! `country_core::utils::load_json` — JSON 网格加载器。
//!
//! 原版符号（B 构建）：`read_attribute_from_json`、`load_json_as_mesh`。流程由 EXE 字节核对
//! （`R6_utils_screen_space.md` 的 JSON 加载流程一节）：
//!
//! * 属性类型用 `format!("{}", json[name]["type"])` 的**文本**匹配：`["int",1]` → `Int`，
//!   `["float",1..4]` → `Float..Float4`；其余 panic `load_json_as_mesh: unrecognized attrib '{name}' with type '{ty}'`。
//! * `json[name]["buffer"]` 必须是数组。`Int` 元素 `as_i64().unwrap() as i32`，`Float` 元素
//!   `as_f64().unwrap() as f32`；`Float2/3/4` 的元素各自是数组，收集后 `try_into [f32; N]`。
//! * `load_json_as_mesh`：读文件失败 → `Couldn't find  {path}`；`log::trace!("Reading mesh {path}")`；
//!   解析失败 → `Couldn't parse  {path}`；`indices = json["indices"]["buffer"]`（PosInt 取 `u32`），
//!   `max_index` 取有符号最大值（空时为 −1）；`json["attributes"]` 是字符串数组；每个属性读出后
//!   `assert!(values.array_length() as i32 > max_index)` 再 `set_attribute`。

use glam::{Vec2, Vec3, Vec4};
use serde_json::Value;

use crate::render::mesh::{Mesh, VertexAttributeValues};

fn buffer_of<'a>(json: &'a Value, name: &str) -> &'a Vec<Value> {
    json[name]["buffer"].as_array().unwrap()
}

fn floats<const N: usize>(element: &Value) -> [f32; N] {
    element
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap()
}

/// 读取名为 `name` 的顶点属性。
pub fn read_attribute_from_json(json: &Value, name: &str) -> VertexAttributeValues {
    let ty = format!("{}", json[name]["type"]);
    let buffer = buffer_of(json, name);
    match ty.as_str() {
        "[\"int\",1]" => {
            VertexAttributeValues::Int(buffer.iter().map(|v| v.as_i64().unwrap() as i32).collect())
        }
        "[\"float\",1]" => VertexAttributeValues::Float(
            buffer.iter().map(|v| v.as_f64().unwrap() as f32).collect(),
        ),
        "[\"float\",2]" => {
            VertexAttributeValues::Float2(buffer.iter().map(|v| Vec2::from(floats(v))).collect())
        }
        "[\"float\",3]" => {
            VertexAttributeValues::Float3(buffer.iter().map(|v| Vec3::from(floats(v))).collect())
        }
        "[\"float\",4]" => {
            VertexAttributeValues::Float4(buffer.iter().map(|v| Vec4::from(floats(v))).collect())
        }
        _ => panic!("load_json_as_mesh: unrecognized attrib '{name}' with type '{ty}'"),
    }
}

/// 把 JSON 文本构建成网格（读文件之外的部分，便于测试）。
fn mesh_from_json(json: &Value) -> Mesh {
    let indices: Vec<u32> = json["indices"]["buffer"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    let max_index = indices.iter().map(|&i| i as i32).max().unwrap_or(-1);

    let mut mesh = Mesh::new("load_json_as_mesh");
    for name in json["attributes"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let values = read_attribute_from_json(json, name);
        assert!(values.array_length() as i32 > max_index);
        mesh.set_attribute(name, values);
    }
    mesh.set_indices(indices);
    mesh
}

/// 从磁盘加载 JSON 网格。
pub fn load_json_as_mesh(path: &str) -> Mesh {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Couldn't find  {path}"));
    log::trace!("Reading mesh {path}");
    let json: Value =
        serde_json::from_str(&text).unwrap_or_else(|_| panic!("Couldn't parse  {path}"));
    mesh_from_json(&json)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let mesh = mesh_from_json(&sample());
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
    #[should_panic(
        expected = "load_json_as_mesh: unrecognized attrib 'Bad' with type '[\"float\",5]'"
    )]
    fn unknown_type_panics_with_the_type_text() {
        let json = json!({"Bad": {"type": ["float", 5], "buffer": []}});
        read_attribute_from_json(&json, "Bad");
    }

    #[test]
    #[should_panic(expected = "assertion failed: values.array_length() as i32 > max_index")]
    fn index_beyond_attribute_length_asserts() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([0, 1, 3]);
        mesh_from_json(&json);
    }

    #[test]
    fn empty_indices_use_minus_one_as_max() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([]);
        assert!(mesh_from_json(&json).indices.is_empty());
    }

    #[test]
    #[should_panic(expected = "Couldn't find  /nonexistent/mesh.json")]
    fn missing_file_panics_with_the_path() {
        load_json_as_mesh("/nonexistent/mesh.json");
    }

    #[test]
    fn loads_from_disk() {
        let path = std::env::temp_dir().join("country_core_load_json_test.json");
        std::fs::write(&path, sample().to_string()).unwrap();
        let mesh = load_json_as_mesh(path.to_str().unwrap());
        assert_eq!(mesh.indices.len(), 3);
        let _ = std::fs::remove_file(path);
    }
}
