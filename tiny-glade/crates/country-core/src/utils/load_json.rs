//! `country_core::utils::load_json` — JSON 网格加载器。
//!
//! 原版符号（B 构建）：`read_attribute_from_json`（0x14083e210）、`load_json_as_mesh`（四个单态 0x140b10ed0 /
//! 0x140b11f50 / 0x140b0eeb0 / 0x140b0fe60）。逐条对照汇编（含 2026-10-15 的独立核对）：
//!
//! * 属性类型用 `format!("{}", json[name]["type"])` 的**文本**匹配：`["int",1]` → `Int`，`["float",1..4]` → `Float..Float4`。
//!   其余类型**返回** `Err`（anyhow 风格，文案 `load_json_as_mesh: unrecognized attrib '{name}' with type '{ty}'`），
//!   由调用方 `unwrap`（load_json.rs:43:64），所以 panic 文本以 `called `Result::unwrap()` on an `Err` value` 开头；
//! * `json[name]["buffer"]` 必须是数组，**在匹配到类型之后才取**（未知类型先于 buffer 检查失败）。
//!   `Int` 元素 `as_i64().unwrap() as i32`，`Float` 元素 `as_f64().unwrap() as f32`；`Float2/3/4` 的元素各自是数组，
//!   收集成 `Vec<f32>` 后按下标取前 N 个——**长度不足才越界 panic，多出的分量被忽略**（不是 `try_into [f32; N]`）；
//! * 每种类型收集完都做 `assert!(!v.is_empty())`（串 0x142ac6fdd，"assertion failed: !v.is_empty()"）：空 buffer 直接 panic；
//! * `load_json_as_mesh`：读文件失败 → panic `Couldn't find  {path}`；`log::trace!("Reading mesh {path}")`；
//!   解析失败 → panic `Couldn't parse  {path}`；`Mesh::new("load_json_as_mesh")` 之后**立即 `set_source_file(path)`**
//!   （0x140d61b00，与 `Mesh::set_source_file` 同码；没有它 `AssetMeshLibrary::add` 不会登记路径、也不会热重载）；
//!   `indices = json["indices"]["buffer"]`（PosInt 取 `u32`），`max_index` 取有符号最大值（空时为 −1）；
//!   `json["attributes"]` 先**整体**收集成字符串列表（任一元素不是字符串就先 panic），再逐个读属性；
//!   每个属性读出后 `assert!(values.array_length() as i32 > max_index)` 再 `set_attribute`。
//!   函数本身不返回 `Result`：所有失败都是 panic。
//!
//! 入口另有一段 puffin profile scope（`0x140c16a00`），属于仪表，不影响结果。

use glam::{Vec2, Vec3, Vec4};
use serde_json::Value;

use crate::render::mesh::{Mesh, SourceFile, VertexAttributeValues};

fn buffer_of<'a>(json: &'a Value, name: &str) -> &'a Vec<Value> {
    json[name]["buffer"].as_array().unwrap()
}

/// 原版收集后的非空断言（"assertion failed: !v.is_empty()"）。
fn non_empty<T>(values: Vec<T>) -> Vec<T> {
    assert!(!values.is_empty(), "assertion failed: !v.is_empty()");
    values
}

/// `Float2/3/4` 的一个元素：收集成 `Vec<f32>` 后取前 `N` 个分量，长度不足时按下标越界 panic。
fn float_components<const N: usize>(element: &Value) -> [f32; N] {
    let components: Vec<f32> = element
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect();
    std::array::from_fn(|i| components[i])
}

/// 读取名为 `name` 的顶点属性；未知类型返回 `Err`（原版返回 tag 5，由调用方 `unwrap`）。
pub fn read_attribute_from_json(json: &Value, name: &str) -> anyhow::Result<VertexAttributeValues> {
    let ty = format!("{}", json[name]["type"]);
    // O 0x14083e210：`["buffer"]` 在每个匹配分支里才取，所以未知类型先于 buffer 检查失败。
    let buffer = || buffer_of(json, name);
    Ok(match ty.as_str() {
        "[\"int\",1]" => VertexAttributeValues::Int(non_empty(
            buffer()
                .iter()
                .map(|v| v.as_i64().unwrap() as i32)
                .collect(),
        )),
        "[\"float\",1]" => VertexAttributeValues::Float(non_empty(
            buffer()
                .iter()
                .map(|v| v.as_f64().unwrap() as f32)
                .collect(),
        )),
        "[\"float\",2]" => VertexAttributeValues::Float2(non_empty(
            buffer()
                .iter()
                .map(|v| Vec2::from(float_components(v)))
                .collect(),
        )),
        "[\"float\",3]" => VertexAttributeValues::Float3(non_empty(
            buffer()
                .iter()
                .map(|v| Vec3::from(float_components(v)))
                .collect(),
        )),
        "[\"float\",4]" => VertexAttributeValues::Float4(non_empty(
            buffer()
                .iter()
                .map(|v| Vec4::from(float_components(v)))
                .collect(),
        )),
        _ => anyhow::bail!("load_json_as_mesh: unrecognized attrib '{name}' with type '{ty}'"),
    })
}

/// 把 JSON 文本构建成网格（读文件之外的部分，便于测试）。`path` 写入 `source_file`。
#[cfg(test)]
fn mesh_from_json(json: &Value, path: &str) -> Mesh {
    try_mesh_from_json(json, path).unwrap()
}

/// 同上，但属性读取错误以 `Err` 传播（`reload_mesh_src` 所用的 Result 单态，0x140b0eeb0）。
fn try_mesh_from_json(json: &Value, path: &str) -> anyhow::Result<Mesh> {
    let mut mesh = Mesh::new("load_json_as_mesh");
    mesh.set_source_file(SourceFile {
        path: path.to_owned(),
        stamp: 0,
    });

    let indices: Vec<u32> = json["indices"]["buffer"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    let max_index = indices.iter().map(|&i| i as i32).max().unwrap_or(-1);

    // 先整体收集属性名，任一元素不是字符串就在读任何属性之前 panic。
    let names: Vec<&str> = json["attributes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect();
    for name in names {
        let values = read_attribute_from_json(json, name)?;
        assert!(values.array_length() as i32 > max_index);
        mesh.set_attribute(name, values);
    }
    mesh.set_indices(indices);
    Ok(mesh)
}

/// 从磁盘加载 JSON 网格；任何失败都是 panic。
pub fn load_json_as_mesh(path: &str) -> Mesh {
    try_load_json_as_mesh(path).unwrap()
}

/// Result 单态：读文件/解析失败仍 panic，只有属性读取错误返回 `Err`（O：0x140b0fa1b 读文件失败直接 panic）。
pub fn try_load_json_as_mesh(path: &str) -> anyhow::Result<Mesh> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Couldn't find  {path}"));
    log::trace!("Reading mesh {path}");
    let json: Value =
        serde_json::from_str(&text).unwrap_or_else(|_| panic!("Couldn't parse  {path}"));
    try_mesh_from_json(&json, path)
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
    #[should_panic(expected = "assertion failed: !v.is_empty()")]
    fn an_empty_attribute_buffer_panics_even_with_empty_indices() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([]);
        json["Tag"]["buffer"] = json!([]);
        mesh_from_json(&json, "m.json");
    }

    #[test]
    fn extra_float_components_are_ignored() {
        let json = json!({"P": {"type": ["float", 3], "buffer": [[1.0, 2.0, 3.0, 99.0]]}});
        assert_eq!(
            read_attribute_from_json(&json, "P").unwrap(),
            VertexAttributeValues::Float3(vec![Vec3::new(1.0, 2.0, 3.0)])
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn too_few_float_components_index_out_of_bounds() {
        let json = json!({"P": {"type": ["float", 2], "buffer": [[1.0]]}});
        let _ = read_attribute_from_json(&json, "P");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn a_non_string_attribute_name_panics_before_any_attribute_is_read() {
        let mut json = sample();
        json["attributes"] = json!(["Bad", 7]);
        json["Bad"] = json!({"type": ["float", 9], "buffer": []});
        mesh_from_json(&json, "m.json");
    }

    #[test]
    #[should_panic(expected = "assertion failed: values.array_length() as i32 > max_index")]
    fn index_beyond_attribute_length_asserts() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([0, 1, 3]);
        mesh_from_json(&json, "m.json");
    }

    #[test]
    fn empty_indices_use_minus_one_as_max() {
        let mut json = sample();
        json["indices"]["buffer"] = json!([]);
        assert!(mesh_from_json(&json, "m.json").indices.is_empty());
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
