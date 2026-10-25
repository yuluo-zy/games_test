# R6：E 包第一部分（utils 屏幕空间）

日期：2026-10-14。落点：`tiny-glade/crates/country-core/src/utils/{project_cursor_onto_ray.rs,mod.rs}`。每项用 EXE 汇编核对。

| 项 | 地址 | 级别 |
|---|---|---|
| `ProportionalScreenSpace::from_window_size` | 0x140ac8420 | O：16 B `{size, aspect=w/h, 1.0}`，`u32→f32` 零扩展 |
| `ProportionalScreenSpace::transform_screen_space` | 0x140ac8450 + helper 0x140c906d0 | O：`x' = a/size.x*2 + (-1.0)`、`y' = (1-b)/size.y*2 + 1.0`，再 `*aspect`、`*1.0`（DIGEST 记 helper 为“未吃透”，已读全；y' 的形式照原样保留） |
| `ProportionalScreenSpaceCoord::project_onto_ray` | 0x140ac84a0 | O：近点=起点、远点=起点+方向，各过两个矩阵（第二次不重取 w）；`t = ((c-near)·d)/max(d·d, 0x1e3ce508)`；`f32::clamp`（`min>max` panic）；透视校正插值 `s = lerp(0/wn,1/wf,t) / max(lerp(1/wn,1/wf,t), 0x2edbe6ff)`；结果 `near*(1-s)+far*s` |
| `utils::world_space_to_screen_space` | 0x140ac8780 | O：两矩阵 + 透视除 + `(x*w, (1-y)*h)` |

两个矩阵在原版相机结构里的字段身份（+0x80 / +0x00）是 I。测试 +16（全量 1300，默认与 `native-physics` 相同）。

## 未做
`load_json_as_mesh` / `read_attribute_from_json`：流程已从汇编读出（见下），尚未落码——
- `read_attribute_from_json(json: &Value, name)`：`json[name]["type"]` 做 `format!("{}")`（紧凑 JSON），匹配 `["int",1]`→Int，`["float",1..4]`→Float…Float4，否则 panic `load_json_as_mesh: unrecognized attrib '{name}' with type '{ty}'`；`json[name]["buffer"]` 必须是数组；Int 用 `as_i64().unwrap() as i32`，Float 用 `as_f64().unwrap() as f32`，Float2/3/4 的每个元素是数组再 `try_into [f32;N]`。
- `load_json_as_mesh`：`read_to_string`（失败 `Couldn't find  {path}`）→ `log::trace!("Reading mesh {path}")` → `serde_json::from_str::<Value>`（失败 `Couldn't parse  {path}`；DIGEST 说“自定义解析器”是错的，0x141405930 就是 `serde_json::de::from_str`）→ `json["indices"]["buffer"]`（元素必须是 PosInt，`as u32`）、`max_index`（有符号最大值，空为 -1）→ `json["attributes"]` 是**字符串数组**（逐个元素必须是 String），每个名字 `read_attribute_from_json(&json, name)`，`assert!(values.array_length() as i32 > max_index)`，`mesh.set_attribute`。`Mesh::new("load_json_as_mesh")`。
