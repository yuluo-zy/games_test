# WP18 契约 — draw_shaders 域总览

版本 **B**（EXE `cf048e27…`，PDB `2d9686b8…`）。范围：
`country_core::systems::render_loop::draw_passes::draw_shaders/*`（14 个骨架单元）。
方法：业务逻辑（Ghidra 只读反编译 + PDB 骨架 enriched/dependencies + 调用链）→
合同 → 骨架实现。机器码差分仅用于关键数值/边界裁决，不作实现手段。

标注：**O**=反编译/PDB 直接支持；**I**=调用链/命名组合；**U**=未恢复。

## 1. 每个 draw 模块的通用 pass 组合（O）

raster draw 以 `DrawBuilder`（wp1，0xf8B，值传递链式）组合：

1. **纹理绑定**：`bind_texture_to_draw_builder<TexturePreset>(preset_byte)` /
   `<TextureFromDisk>(name_hash)`（wp1 impl$3，0x140908c50/0x140908610）。
2. **SSBO 读**：`DrawBuilder::read_storage_buffer<T>`（wp1，0x141629e90 族）。
3. **uniform 写**：`UniformBinder::write_uniform<T>`（0x14096b660 族）——`f32`/
   `i32`/`bool`/`Vec3`/`Mat4` 等。
4. **draw 模式**（判别 1/3/4/5/7/8）：
   - `draw()`（0x140906f40）：直绘（floor/duck）。
   - `draw_instanced_w_gl_draw_flag`（0x140907490）：实例 + GL draw flag（critters/
     ivy/leaf）。
   - `multi_draw_indexed_indirect`（0x140907990）：索引间接（grass LOD0/pebble/
     stairs/bricks）。
   - `multi_draw_indirect`（0x1409076b0）：非索引间接（grass LOD1）。
   - `draw_no_index_buffer`（0x1409083f0）：无索引直绘（rocky）。
   - `nani_draw`（0x140904ae0，`render::nani::draw_ext::NaniDrawExt`）：nani
     GPU-driven 间接（clearing 树/鹪鹩/instanced roof）。
5. **守卫/变体**：`+0xf0` 状态字节（discard/变体选择）；`+0xa8` cull/blend 位。

## 2. 模块 ↔ shader URI ↔ compiled-shaders 交叉核对（O）

| 模块 | URI（shader-uri-list.json 匹配） | compiled `.bin` |
|---|---|---|
| billboard_forest | `/billboard_forest.raster.hlsl`、`/billboard_plant.raster.hlsl` | `_billboard_forest`、`_billboard_plant` |
| clearing_shaders | `/clearing/tree_trunk.raster.hlsl`、`/clearing/tree_canopy.raster.hlsl` | `_clearing_tree_trunk`、`_clearing_tree_canopy`（另 `distant_billboard/instanced_background_tree/meadow_*` 本模块外） |
| clouds | `/cloud.raster.hlsl` | `_cloud` |
| stars | `/star.raster.hlsl` | `_star` |
| draw_bricks | `/wall/wall_brick_lod0.raster.hlsl`、`/wall/wall_brick_lod1.raster.hlsl` | `_wall_wall_brick_lod{0,1}` |
| floor_shader | `/terrain_editing/floor.raster.hlsl` | `_terrain_editing_floor` |
| grass_shader | `/grass.raster.hlsl`、`/grass_lod1.raster.hlsl` | `_grass`、`_grass_lod1` |
| instanced_critters | `/critters/{butterfly,firefly,firefly_glow,duck_animated,nani_wren}.raster.hlsl` | `_critters_*`（+`_critters_firefly_vis_query.cs.hlsl` compute） |
| instanced_ivy | `/ivy/instanced_ivy_branch.raster.hlsl`、`/ivy/instanced_ivy_leaf.raster.hlsl` | `_ivy_instanced_ivy_*`（另 `_ivy_ivy.raster` 无 URI 匹配，U） |
| leaf_particle_draw | `/leaf_particle.raster.hlsl` | `_leaf_particle`（+`_leaf_particles.cs.hlsl` compute 生产者） |
| path_pebble_shader | `/path_pebble.raster.hlsl` | `_path_pebble`（+`_path_pebbles.cs.hlsl` compute 生产者） |
| rocky_terrain_draw | `/rocky_terrain/rocky_terrain.raster.hlsl` | `_rocky_terrain_rocky_terrain` |
| stairs_draw | `/rocky_terrain/stairs/stairs_indirect.raster.hlsl` | `_rocky_terrain_stairs_stairs_indirect` |

全部 raster 双变体 `b903e43ffb3da915`/`f0adff76827e73ff`（compiled-shaders 全名单
`evidence/render-passes/pass-builder/compiled-shaders-full-list.txt` 核验，O）。

## 3. 磁盘纹理名字哈希（TextureFromDisk）

| 哈希 | 语义 | 状态 |
|---|---|---|
| `0x579f39ab91a01865` | water_noise_texture | **O**（wp7 water_planes_draw） |
| `0x84a037e697de0324` | 岩石纹理（rocky/stairs 共用） | I（名 U） |
| `0x84b51135b59095ba` | 石材纹理（floor/path_pebble 共用） | I（名 U） |
| `0x7db9e7483fa24078` | 砖纹理（draw_bricks） | U |
| `0xce1615b9bd3e034`…`0x4dbb39acf1c7b7`（clearing） | 树干/树冠 | U |
| `0x4a2b687d0c1ecf58`…`0x954b7e4974bbcd6b`（floor 余项） | 地面 | U |
| `0x4bb0467de05d40d2`/`0xefa7a5fecb959f12`（leaf） | 叶粒子 | U |
| `0x56463770c9cae8e1`/`0xe69294dae8320741`（critters） | 蝴蝶/鹪鹩 | U |
| `0xedf425b62b653c03`（ivy） | 常春藤叶 | U |

## 4. 主要 typed gap（U）

1. 各 TextureFromDisk 名字哈希 → 纹理资源名（除 water_noise_texture O 外）。
2. ECS 资源 type-id 语义：`0xbba451ee6a08a3ff`（camera，O）、`0xd2f519263066413b`、
   `0x1b4e385d376c1cc0`、`0x8226cc786e679355`、`0x850dbd51eafef756`、
   `0xfa44538b95ae76c6`、`0x3dce36237fb50dff`、`0xa2127765959c8b85`、
   `0xdf356893cc4e98a8`。
3. `bind_brick_erase_highlight`/`bind_wall_color_customization`/`bind_stone_floor_textures`
   O 为 `DrawBuilder::bind_*` 方法；共享 crate 建模在 `CommonDrawParams`
   （集成裁决）。
4. duck 三组件/Mat4 uniform 来源；`nani_draw` kind 取值（tree/wren）。
5. `_ivy_ivy.raster.hlsl` 接线；`<1>` 砖变体 case 3/4 语义；TexturePreset 数值槽
   语义表。

## 5. 交付索引

- 14 个骨架单元 → `src/systems/render_loop/draw_passes/draw_shaders/*.rs`。
- 本总览 + `billboard-forest.md`/`clearing-shaders.md`/`clouds-stars.md`/
  `common-includes.md`/`draw-bricks.md`/`floor-grass.md`/`instanced-critters-ivy.md`/
  `leaf-pebble.md`/`rocky-stairs.md`。
- 构建门禁：`cargo check --offline` / `cargo clippy --offline`（0 错误/0 警告，
  wp18 crate）。
