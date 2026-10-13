# WP18 契约 — leaf_particle_draw.rs / path_pebble_shader.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b2）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## leaf_particle_draw.rs（编译单元 556）

| 项 | 值 |
|---|---|
| `leaf_particle_draw` | 0x140a8a090，size 435，行 13 |

绑定序（O）：资源 `0x3dce36237fb50dff`（U，unwrap）→
`read_storage_buffer<LeafParticleSsbo>`（=`_leaf_particles.cs.hlsl` W18 生产者）→
`TextureFromDisk(0x4bb0467de05d40d2)` → `TextureFromDisk(0xefa7a5fecb959f12)` →
`draw_instanced_w_gl_draw_flag`。

URI（O）：`/leaf_particle.raster.hlsl`（compiled `_leaf_particle`，双变体）。
typed gap：`0x3dce36237fb50dff` 语义；两磁盘纹理名 U。

## path_pebble_shader.rs（编译单元 498）

| 项 | 值 |
|---|---|
| `path_pebble_draw` | 0x1408e5a10，size 178，行 14 |

绑定序（O）：`read_storage_buffer<PebblePlacementSsbo>`（=`_path_pebbles.cs.hlsl`
W17 生产者）→ `TextureFromDisk(0x84b51135b59095ba)`（与 floor_shader 共用石材
纹理，I）→ `multi_draw_indexed_indirect`（间接参数源 = W17 写
`DrawIndexedIndirectBuffer`，I）。

URI（O）：`/path_pebble.raster.hlsl`（compiled `_path_pebble`，双变体）。
typed gap：`0x84b51135b59095ba` 纹理名 U（O：与 floor 共用）。

## 交付

`src/.../draw_shaders/leaf_particle_draw.rs`、`path_pebble_shader.rs`
（各 1 函数，`unimplemented!()` 骨架）。
