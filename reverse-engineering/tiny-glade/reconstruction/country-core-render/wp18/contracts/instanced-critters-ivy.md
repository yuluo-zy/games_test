# WP18 契约 — instanced_critters.rs / instanced_ivy.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b2/b3）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## instanced_critters.rs（编译单元 1615）

| 函数 | 地址/size/行 | 绑定序（O） |
|---|---|---|
| `draw_instanced_butterfly_shader` | 0x1415547e0 / 310 / 19 | `TexturePreset(9)`→10→`read_storage_buffer<ButterflySsbo>`→`TextureFromDisk(0x56463770c9cae8e1)`→`draw_instanced_w_gl_draw_flag` |
| `draw_instanced_firefly_shader` | 0x1415549b0 / 569 / 34 | 资源 `0xa2127765959c8b85`（U，密度 f32）非 0 → `TexturePreset(9)`→10→`write_uniform<f32>`→`read_storage_buffer<FireflySsbo>`→`draw_instanced_w_gl_draw_flag`；0 → `discard` |
| `draw_instanced_firefly_glow_shader` | 0x141554cf0 / 642 / 55 | 同 firefly + 置 `+0xa8 &= 0x7f`（blend，I）+ 读 `FireflySsbo`+`FireflyVisibilitySsbo`+`DofAutofocusSsbo` → flag |
| `draw_duck_shader` | 0x141555080 / 1719 / 80 | 间接参数（u64）+ `get_component`×3（U）→ `TexturePreset(9)`→10→`write_uniform<Mat4>`（四元数→矩阵，O）→`write_uniform<f32>`×2→`write_uniform<bool>`→`write_uniform<f32>`→`write_uniform<i32>`×2→`draw()` |
| `draw_wren_shader` | 0x141555ec0 / 289 / 124 | `TexturePreset(9)`→10→`TextureFromDisk(0xe69294dae8320741)`→`nani_draw`（鹪鹩走 nani GPU-driven，O） |

URI（O）：`/critters/{butterfly,firefly,firefly_glow,duck_animated,nani_wren}.raster.hlsl`
（compiled `_critters_*`，双变体）。`_critters_firefly_vis_query.cs.hlsl` =
`FireflyVisibilitySsbo` 生产者（O，catalog §18）。
typed gap：`0xa2127765959c8b85` 语义；duck 三组件/Mat4 来源；磁盘纹理名 U。

## instanced_ivy.rs（编译单元 497）

| 函数 | 地址/size/行 | 绑定序（O） |
|---|---|---|
| `draw_instanced_ivy_branch_shader` | 0x1408dbc50 / 855 / 19 | 资源 `0xdf356893cc4e98a8`（U，枝段 map）非空 → `read_storage_buffer<IvySegmentSsbo>`→`TexturePreset(9)`→10→8→`TextureFromDisk(0x579f39ab91a01865)`（=water_noise_texture，O）→`TexturePreset(5)`→`TextureFromDisk(0xedf425b62b653c03)`→`draw_instanced_w_gl_draw_flag` |
| `draw_instanced_ivy_leaf_shader` | 0x1408dc180 / 21 / 50 | → `ivy_leaf_shader`（wrapper） |
| `draw_instanced_ivy_flower_shader` | 0x1408dc1a0 / 21 / 60 | → `ivy_leaf_shader`（wrapper） |
| `ivy_leaf_shader` | 0x1408dc1c0 / 1555 / 70 | `CameraMatrices::eye_position`→`write_uniform<Vec3>`→`TexturePreset(9)`→10→5→`TextureFromDisk(0x579f39ab91a01865)`→`write_uniform<bool>`→叶：`read_storage_buffer<IvyLeafSsbo>`+FrozenMap assert（0x3d=61B 名）/花：`read_storage_buffer<IvyFlowerSsbo>`+assert（0x3f=63B 名）→`draw_instanced_w_gl_draw_flag` |

URI（O）：`/ivy/instanced_ivy_branch.raster.hlsl`、`/ivy/instanced_ivy_leaf.raster.hlsl`
（compiled `_ivy_instanced_ivy_*`）。compiled 另含 `_ivy_ivy.raster.hlsl`（无静态
URI 匹配，U：花/叶变体或运行时拼接）。
typed gap：`0xdf356893cc4e98a8` 语义；FrozenMap 断言名；`_ivy_ivy.raster` 接线。

## 交付

`src/.../draw_shaders/instanced_critters.rs`（5 函数）、`instanced_ivy.rs`
（4 函数）。
