# WP18 契约 — floor_shader.rs / grass_shader.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b1）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## floor_shader.rs

| 项 | 值 |
|---|---|
| 编译单元 | 530 |
| `draw_floor_shader` | 0x1409cacb0，size 1363，行 14 |

绑定序（O）：置 `+0xa8 |= 8`（渲染标志，I）→ `TexturePreset(9)` → 10 → 6 →
`0x13` → `TextureFromDisk(0x4a2b687d0c1ecf58)` → `bind_stone_floor_textures`
（0x14090c300）→ `bind_wall_color_customization`（0x14090bb70）→
`TextureFromDisk(0x84b51135b59095ba)` → `TextureFromDisk(0xb484366d00600ae5)` →
`TextureFromDisk(0x59b98d76c4e95bd1)` → `TexturePreset(0x15)` → 7 → 0xc → 0xe →
`0x14` → `TextureFromDisk(0x954b7e4974bbcd6b)` → `write_uniform<bool>` → `draw()`
（直 draw，0x140906f40）。

URI（O）：`/terrain_editing/floor.raster.hlsl`（compiled `_terrain_editing_floor`）。
typed gap：preset 槽/磁盘纹理名语义（9/10/6/0x13/0x15/7/0xc/0xe/0x14 = terrain
纹理族，I）；`0x84b51135b59095ba` 与 path_pebble 共用（I）。

## grass_shader.rs

| 项 | 值 |
|---|---|
| 编译单元 | 589 |
| `draw_common` | 行 18（内联共享宿主） |
| `draw_grass_shader` | 0x140b4af70，size 412，行 37 |
| `draw_grass_lod1_shader` | 0x140b4b1d0，size 410，行 47 |

**draw_common**（O，内联）：`TexturePreset(9)` → 10 → 8 →
`read_storage_buffer<GrassBladeDataSsbo>` → `read_storage_buffer<AtomicCounterBuffer>`
→ `write_uniform<i32>`。

**draw_grass_shader**（O）：draw_common + `multi_draw_indexed_indirect`
（草 LOD0 索引间接）。
**draw_grass_lod1_shader**（O）：draw_common + `multi_draw_indirect`
（LOD1 非索引间接）。

URI（O）：`/grass.raster.hlsl`、`/grass_lod1.raster.hlsl`（compiled `_grass`/
`_grass_lod1`）。间接参数源 = W29 grass compute（`_generate_grass`/`_grass_*`）
写 `DrawIndexedIndirectBuffer`/`DrawIndirectBuffer` + `AtomicCounterBuffer`（I）。
typed gap：preset 9/10/8 槽语义（I）。

## 交付

`src/.../draw_shaders/floor_shader.rs`（`draw_floor_shader`）、
`grass_shader.rs`（`draw_common`/`draw_grass_shader`/`draw_grass_lod1_shader`）。
