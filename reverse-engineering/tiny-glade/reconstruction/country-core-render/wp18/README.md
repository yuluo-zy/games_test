# WP18 — draw_shaders 域（per-shader raster draw 模块）

恢复 country-core **draw_shaders 域**（`systems/render_loop/draw_passes/draw_shaders/*`，
14 个骨架单元）。B 版本：EXE `cf048e27…`，PDB `2d9686b8…`。方法遵循项目流程：
业务逻辑（Ghidra 只读反编译 + PDB 骨架 enriched/dependencies + 调用链）→
合同 → 骨架实现；机器码差分仅用于关键数值/边界裁决。

## 交付

- `contracts/`
  - `draw-shaders.md` — 域总览：通用 pass 组合（DrawBuilder 纹理/SSBO/uniform/
    draw 模式）、模块↔URI↔compiled-shaders 交叉核对表、磁盘纹理名字哈希表、
    主要 typed gap 清单。
  - `billboard-forest.md` / `clearing-shaders.md` / `clouds-stars.md` /
    `common-includes.md` / `draw-bricks.md` / `floor-grass.md` /
    `instanced-critters-ivy.md` / `leaf-pebble.md` / `rocky-stairs.md` —
    每个 draw 模块的 pass 组合、URI(s)、绑定序、typed gap（O/I/U）。
- `src/` — 骨架对齐源码（镜像 `reconstruction/country-core` 装配布局）：
  - `systems/render_loop/draw_passes/draw_shaders/{billboard_forest, clearing_shaders,
    clouds, common_includes, draw_bricks, floor_shader, grass_shader,
    instanced_critters, instanced_ivy, leaf_particle_draw, path_pebble_shader,
    rocky_terrain_draw, stairs_draw, stars}.rs`（14 单元）。
  - `resources/{texture_presets, critters, color_id_roof}.rs` — 共享 `country-core`
    尚未恢复、draw_shaders 需要的 typed-gap 载体。
- `scripts/` — Ghidra 只读 job/result（b1/b2/b3）+ 提取脚本 + decomp dump。
- `evidence.json` — 地址表、skeleton SHA、URI 交叉核对、构建验证、typed gap。

## 构建验证

`reconstruction/country-core-render/wp18`（独立证据束 crate；path 依赖共享
`country-core` crate，离线缓存遵循原发行 Cargo.lock 基线）：

```
cargo check --offline  → 0 errors / 0 warnings（wp18；country-core 依赖保留其
                         既有 176 个骨架警告）
cargo clippy --offline → 0 errors / 0 warnings
```

全部标记 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据。

## 主要恢复成果

- **14 个 draw 模块全部映射**：billboard_forest/clouds/stars 为 plugin+startup
  （quad 网格 + mesh/shader 入库 + spawn `DesignatedRenderPass`）；clearing
  tree_trunk/canopy 走 `nani_draw`；draw_bricks LOD0/1 走
  `multi_draw_indexed_indirect`；floor 直 `draw`；grass LOD0/1 走
  index/non-index indirect；critters/ivy/leaf 走 `draw_instanced_w_gl_draw_flag`；
  path_pebble/stairs/rocky 走间接/无索引绘制。
- **shader URI ↔ compiled-shaders 交叉核对完成**（255 URI/404 .bin）：
  全部 14 模块命中静态 URI，raster 双变体 `b903…`/`f0ad…`。
- **磁盘纹理名字哈希**：`0x579f39ab91a01865`=water_noise_texture（O，wp7）；
  `0x84a037e697de0324`（岩石）/`0x84b51135b59095ba`（石材）跨模块共用（I）。
- **`nani_draw` 分派**（0x140904ae0）：GpuWorld subset 锁 + NaniDrawList* +
  draw_indexed_indirect_raw/count（NANI_FORCE_FINE_GRAINED_SORTING 编译期开关）。
- **`bind_*` helper**（common_includes）：terrain_heightmap/lighting 内联 + 
  water_object_push（DrawBuilder/ComputePass 双实例）+ garden_exclusion 四槽。

## 复用裁定（prior work）

- 共享 `reconstruction/country-core`（wp1–wp16 装配）作为 path 依赖：DrawBuilder/
  ShaderUri/SSBO 类型/FrameConstants 直接复用；`nani_draw` 复用 wp3
  `render::nani::draw_ext::NaniDrawExt`。
- `evidence/render-passes/`（INDEX.md、shaders/catalog.md、pass-builder
  shader-uri-list.json、compiled-shaders-full-list.txt）作为 URI/compiled 交叉核对
  数据源。
- wp7 `water_planes_draw.rs` 的 `bind_water_object_push` 边界复用。

## 主要缺口（如实登记）

- TextureFromDisk 名字哈希→纹理资源名（除 water_noise_texture O 外全 U）。
- ECS 资源 type-id 语义（9 个未解析，见 evidence.json）。
- `bind_brick_erase_highlight`/`bind_wall_color_customization`/`bind_stone_floor_textures`
  O 为 `DrawBuilder::bind_*`，共享 crate 建模在 `CommonDrawParams`（集成裁决）。
- `_ivy_ivy.raster.hlsl` 接线；duck Mat4 uniform 来源；TexturePreset 数值槽语义。
- startup 系统参数（ShaderWatch 等）以 typed gap 承载。
