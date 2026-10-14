# utils/load_json.rs

> 文件作用：**JSON→Mesh 网格加载器 + JSON 属性读取**。`load_json_as_mesh`（4 泛型实例：
> `&str`/`PathBuf`/`String`/`&PathBuf`）用**自定义 JSON 解析器**（非 serde_json；断言
> `values.array_length() as i32 > max_index` @0x142ae8078），解析 `indicesbuffer/buffer`（u32 索引）
> 与 `attributes`（(name,descriptor) 表）→ bevy 风格 `Vertex_Position`/`Vertex_Color` Mesh。
> **共享 utils 无此函数**（RENDER_GAP_ANALYSIS §4.1 确认），68 调用方依赖 country-core 自有实现。
> 证据：4 实例反编译、`read_attribute_from_json` @0x14083e210（`type` 键 int/float tag 0..4）。

函数总计 19：业务 5 / 内联无独立符号 0 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `load_json_as_mesh<String>` | 0x140b10ed0 | 5 | JSON 串→Mesh（消费释放 String） | **O** | 自定义解析 FUN_141405930 + 键 indicesbuffer/attributes |
| `load_json_as_mesh<&PathBuf>` | 0x140b11f50 | 5 | 借用 PathBuf→Mesh | **O** | 同（借用不释放） |
| `load_json_as_mesh<&str>` | 0x140b0eeb0 | 5 | JSON 串→Mesh（68 调用方） | **O** | 读串 + 解析 + panic `Couldn't find  ` / `Couldn't parse  ` |
| `load_json_as_mesh<PathBuf>` | 0x140b0fe60 | 5 | PathBuf→Mesh | **O** | fs::read_to_string(FUN_140d59140)，缺失 panic |
| `read_attribute_from_json` | 0x14083e210 | 51 | 按名读属性→数值 | **O** | `type` 键格式化匹配 int/float(1..4) tag 0..4；panic `load_json_as_mesh: unrecognized attrib '` |

## 派生胶水（14 项：closure 等）
