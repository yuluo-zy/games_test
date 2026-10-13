# WP18 契约 — clearing_shaders.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b1）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## 类型（O）

| 项 | 值 |
|---|---|
| 编译单元 | 530 |
| `tree_trunk` | 0x1409c9830，size 1643，行 26 |
| `tree_canopy` | 0x1409ca350，size 1373，行 80 |

## 共享逻辑（O）

`DrawBuilder`（in_RCX，0xf8B 值传递）。`+0xf0` 状态字节非 0 → `+0xa8 |= 4`
（cull mode，I）。结尾 `nani_draw()`（0x140904ae0，`render::nani::draw_ext::
NaniDrawExt`）：锁 GpuWorld subset 槽 → 绑定 `NaniInstanceIndirectionSsbo` +
`NaniDrawListLengthSsbo`/`NaniDrawListSsbo` → `draw_indexed_indirect_raw/count`
（按 `NANI_FORCE_FINE_GRAINED_SORTING` 分派）。

## tree_trunk 绑定序（O）

| # | 绑定 | 证据 |
|---|---|---|
| 1 | 资源 type-id `0xd2f519263066413b`（U）unwrap | get_resource_id |
| 2 | 资源 type-id `0xbba451ee6a08a3ff`（=camera，O）unwrap | get_resource_id |
| 3 | `TextureFromDisk`×6：`0xce1615b9bd3e034`、`0x32dce6939b569f34`、`0x8215a488d99562a6`、`0x93201d69003f98d4`、`0xaa12b01311b77c4d`、`0x4dbb39acf1c7b7` | bind_texture_to_draw_builder |
| 4 | `write_uniform<f32>`（树干参数，I） | UniformBinder |
| 5 | `CameraMatrices::eye_position` → `write_uniform<Vec3>` | 0x14096b660 |
| 6 | `write_uniform<bool>` | 0x14096b850 |
| 7 | 资源 `0x1b4e385d376c1cc0`（U：TreeCanopyMaxCullDistance 邻类）→ `write_uniform<f32>` + `TexturePreset(0xe)` | get_resource_id + bind_texture |
| 8 | `nani_draw()` | 0x140904ae0 |

## tree_canopy 绑定序（O）

| # | 绑定 | 证据 |
|---|---|---|
| 1 | 资源 `0xbba451ee6a08a3ff`（camera）、`0xd2f519263066413b`（U）unwrap | get_resource_id |
| 2 | `TextureFromDisk`×5：`0x579f39ab91a01865`（=water_noise_texture，O）、`0x521d5b30024d9662`、`0x93201d69003f98d4`、`0xaa12b01311b77c4d`、`0x4dbb39acf1c7b7` | bind_texture_to_draw_builder |
| 3 | `write_uniform<f32>`×2（距离/高度，I） | 0x14096ba30 |
| 4 | `write_uniform<bool>` | 0x14096b850 |
| 5 | 资源 `0x1b4e385d376c1cc0` → `write_uniform<f32>` | get_resource_id |
| 6 | `nani_draw()` | 0x140904ae0 |

骨架 deps（O）：`TreeCanopyMaxCullDistance::get_dist`（tree_edit.rs:100）、
`MainCamera::lens`（camera_rig.rs:444）、`CameraModeStates::current_dyn`、
`lerp`、`RasterPipelineOpts::set_cull_mode(_checked)`（cull back，O）。

## Pass 组合 / shader URI（O）

URI：`/clearing/tree_trunk.raster.hlsl`、`/clearing/tree_canopy.raster.hlsl`
（compiled `_clearing_tree_trunk`/`_clearing_tree_canopy`，双变体）。catalog 另有
`_clearing_{distant_billboard,instanced_background_tree,meadow_flower,meadow_plant}`
（本模块外接线，U）。

## typed gap（U）

- 各 TextureFromDisk 名字哈希对应纹理资源名（除 water_noise_texture O 外）。
- `0x1b4e385d376c1cc0` 资源 type-id 语义；`nani_draw` kind 取值。

## 交付

`src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs`
（`tree_trunk`/`tree_canopy`，`unimplemented!()` 骨架）。
