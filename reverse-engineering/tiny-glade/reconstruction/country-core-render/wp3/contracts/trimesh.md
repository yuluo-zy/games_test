# Trimesh 契约 — trimesh.rs（2 procs）

标注：**O**=反编译直接支持；**I**=推断；**U**=未恢复。地址 B patched EXE。

## 1. `TrimeshBuilder`（O）

CPU 侧三角网格增量构造器。`attrib<T>`（0x1412a90c0）显示布局：
- 索引 [0..4]：`{ vertex_count 相关, ... }`（O：`*in_RDX = 1` 由 `positions` 写“positions 已写”标志）。
- [5]/[6]：attrib Vec {cap, ptr}（`RawVec::grow_one<( &str, VertexAttributeValues )>`）。
- [7]：`attrib_len`（当前 attrib 数）。
- [8]：`expected_vertex_count`（0 表示未定）。
- [9]：`vertex_count`（首个 attrib 设值，后续必须相等，`assert_failed<usize,usize>`）。

## 2. `attrib<T>`（0x1412a90c0，行 ~29）

```
fn attrib<T>(&mut self, name: &str, data: &[T]) -> Self
```
1. `data` 空 → `panic`；`data.len() < expected_vertex_count` → `panic`（首个 attrib 前 expected=0 不校验）。
2. 若 `vertex_count==0` → 置为 `data.len()`；否则必须与已定 count 相等（`assert_failed`）。
3. attrib 记录 = 0x18B `{ name: &str(8B); byte_len = data.len()*element_size(8B); element_size u16; element_type u8 }`；`<Vec3>` 的 element_size=0xc、type=4（O）。push 到 attrib Vec。
4. 按值返回自身（builder 模式，0x58B 复制回写，in_RCX 为返回缓冲）。

## 3. `positions`（0x14129e890，行 ~9）

```
fn positions(&mut self, positions: &[Vec3]) -> Self
```
置 `positions_written = true`（`*in_RDX = 1`），然后 `attrib<Vec3>("positions", positions)`。

## 4. Unknown

- `TrimeshBuilder::new`、`build`（产出最终 mesh/Vao）等其余方法未在 PDB 独立过程出现（内联或属其他文件）。
- attrib 的 `element_type` 编码（4 = 3×f32 的判别值）与上游 mesh 属性枚举映射 U。
- positions 在 attrib 列表中的排序/去重规则 U。
