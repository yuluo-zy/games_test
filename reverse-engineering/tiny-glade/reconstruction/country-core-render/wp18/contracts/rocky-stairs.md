# WP18 契约 — rocky_terrain_draw.rs / stairs_draw.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b2）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## rocky_terrain_draw.rs（编译单元 530）

| 项 | 值 |
|---|---|
| `rocky_terrain_shader` | 0x1409cb4e0，size 501，行 16 |

绑定序（O）：置 `+0xa8 |= 4`（cull，I）→ `read_storage_buffer<RockyTerrainVertexSSBO>`
→ `read_storage_buffer<RockyTerrainTriangleSSBO>`（=W4 `displace_rocky_terrain` 写
顶点/三角 SSBO，I）→ `TextureFromDisk(0x84a037e697de0324)` → `TexturePreset(0xc)`
→ `write_uniform<bool>` → 资源 `0xfa44538b95ae76c6`（U：岩石网格实例/顶点计数，
unwrap）→ `draw_no_index_buffer`（无索引直绘，O）。

URI（O）：`/rocky_terrain/rocky_terrain.raster.hlsl`（compiled
`_rocky_terrain_rocky_terrain`，双变体；catalog §15 确认读
`vertices`/`triangles` SSBO + `rocky_terrain_texture`）。
typed gap：`0x84a037e697de0324` 纹理名 U；`0xfa44538b95ae76c6` 资源语义 U。

## stairs_draw.rs（编译单元 482）

| 项 | 值 |
|---|---|
| `stairs_draw` | 0x14086a780，size 1140，行 15 |
| `closure$0` | 行 23（inlinee，哈希表迭代闭包，哨兵 0x1d） |

逻辑（O）：`fetch_ref`（实体间接参数）→ 两张 RwLock 保护 hash map
（`+0x50`/`+0xb8`，读者计数 `+0x90`；键 `0x775c6361f3425a3a`/`0x3bad471c3be19d00`，
U）——楼梯实例查表（位掩码迭代，哨兵 0x1d）→ `Table::get_component`
（`StairsTransformSsbo`）→ 绑定：`read_storage_buffer<StairsTransformSsbo>` →
`TextureFromDisk(0x84a037e697de0324)`（与 rocky 共用岩石纹理，I）→
`write_uniform<i32>` → `write_uniform<bool>` → `TexturePreset(0xc)` →
`multi_draw_indexed_indirect`（间接参数源 = W20 `_stairs_stairs`/`_stairs_counter`
compute，I）。

URI（O）：`/rocky_terrain/stairs/stairs_indirect.raster.hlsl`（compiled
`_rocky_terrain_stairs_stairs_indirect`，双变体）。
typed gap：两 map 键/哨兵语义 U；`StairsTransformSsbo` 记录布局 U。

## 交付

`src/.../draw_shaders/rocky_terrain_draw.rs`、`stairs_draw.rs`
（各 1 函数 + `closure_0`，`unimplemented!()` 骨架）。
