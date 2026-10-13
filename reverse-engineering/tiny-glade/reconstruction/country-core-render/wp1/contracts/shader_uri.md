# shader_uri.rs 业务契约

标注：**O**=反编译直接支持；**I**=调用链/行号组合；**U**=未恢复。

## 1. `ShaderUri`（O，48B = 3×16B &str 对；行 20）

`ShaderUri::parse(uri: &str)`（0x140968960）：
1. `memrchr(b':')` → 截断整串至 `:`；entry = 串尾切片（ptr@+0x20, len@+0x28）。
2. 在截断串内 `memrchr(b'/')` → name 起点 = 最后一个 `/` 之后（ptr@+0x10, len@+0x18）。
3. 在 name 段内 `memrchr(b'.')` → name 截止最后一个 `.`；**无 `.` → `unwrap_failed`**。
4. 输出：whole = 原串截断至 `:`（ptr@+0x00, len@+0x08）；name = 去扩展名段；entry = 冒号后段。

示例：`/nani/cull_and_bucket.cs.hlsl:cs_main` → whole `/nani/cull_and_bucket.cs.hlsl`、name `cull_and_bucket`、entry `cs_main`。

## 2. trait `AsComputeShaderSource`（O，行 48/52）

```rust
pub trait AsComputeShaderSource {
    fn source_hash(&self) -> u64;          // 行 48；impl$1::source_hash<&str> 0x1408a4300（37B）
    fn as_compute_pipeline(&self) -> ...;  // 行 52；impl$1::as_compute_pipeline<&str> 0x1408a4330；<String> 0x1405879f0(module15)
}
```
- `ComputeShaderSourceHash`：`compute`（inline 22）——计算源码 hash 的辅助。
- `as_compute_pipeline` 产出 `ShaderUri` + shader def 值（`AsShaderDefValue`），供 `AssetShaderLibrary::get_compute_by_name` 缓存键。
- `AsShaderDefValue` trait（impl$3/impl$4，行 130 附近）：`as_shader_def_value` —— 把计算 pass 结构体转成编译期特性值（与 INDEX.md 的“两 16-hex 变体对”编译特性对应，I）。

## 3. 18 个游戏实现 = compute pass 清单（O，PDB + shader-uri-list 匹配，见 pass-declaration.md §2.1）

| 实现类型 | URI |
|---|---|
| `render::nani::cull_shaders::NaniCullAndBucketMain` | `/nani/cull_and_bucket.cs.hlsl:cs_main` |
| `NaniCullAndBucketBush` | `/nani/cull_and_bucket_bush.cs.hlsl:cs_main` |
| `NaniCullAndBucketCurvedHalftimber` | `/nani/cull_and_bucket_curved_halftimber.cs.hlsl:cs_main` |
| `NaniCullAndBucketFlowers` | `/nani/cull_and_bucket_flowers.cs.hlsl:cs_main` |
| `NaniCullAndBucketPlaster` | `/nani/cull_and_bucket_plaster.cs.hlsl:cs_main` |
| `NaniCullAndBucketRoofTile` | `/nani/cull_and_bucket_roof_tile.cs.hlsl:cs_main` |
| `NaniCullAndBucketSmoke` | `/nani/cull_and_bucket_smoke.cs.hlsl:cs_main` |
| `NaniCullAndBucketStairsFloor` | `/nani/cull_and_bucket_stairs_floor.cs.hlsl:cs_main` |
| `NaniCullAndBucketWoodenPlank` | `/nani/cull_and_bucket_wooden_plank.cs.hlsl:cs_main` |
| `systems::render_loop::compute_passes::bin_lights::BinLights` | `/light_binning/bin_lights_to_cells.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResDof` | `/dof/fullres.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResAllLod0Dof` | `/dof/fullres_all_lod0.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResAllLod1Dof` | `/dof/fullres_all_lod1.cs.hlsl:cs_main` |
| `compute_passes::dof::HalfResDof` | `/dof/halfres.cs.hlsl:cs_halfres` |
| `compute_passes::dof::HalfResAllDof` | `/dof/halfres.cs.hlsl:cs_halfres_all` |
| `compute_passes::rasterize_terrain_stroke::GaussBlur` | `/gauss_blur.cs.hlsl:cs_main` |
| `compute_passes::taa::TgsrMain` | `/tgsr.cs.hlsl:cs_main` |
| `vermeer::GenerateStrokes` | `/vermeer/generate_strokes.cs.hlsl:cs_main` |

`as_compute_pipeline` 单态化地址（module 549 DoF / module 1634 nani / module 487 等）见 evidence。

## 4. Display / Hash / Eq（O）

- `impl$18::fmt`（0x140939420，行 214）：`ShaderUri` Debug/Display（输出 URI 全串）。
- `impl$5::hash`、`impl$7::eq`（inline）：`ShaderUri` 作 hash map 键（`HashMap<ShaderUri,…>`，供 `AssetShaderLibrary` 缓存与 `get_compute_by_name`）。

## 5. 通用 T 实现（I）

`impl$2::source_hash` / `as_compute_pipeline`（inline 1）：为 `&str`/`String` 提供的 `AsComputeShaderSource`，即 `ComputePass::new<&str>`/`new<String>` 的默认路径（无需自定义 def 值）。
