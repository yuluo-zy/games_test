# WP18 契约 — billboard_forest.rs

版本 **B**（EXE `cf048e27…`，PDB `2d9686b8…`）。数据源：skeleton bundle
`billboard_forest.rs.{pdb,enriched,dependencies}`、Ghidra 只读反编译（job
`scripts/job_drawshaders_b1.json` → `result_drawshaders_b1.json`）、
`evidence/render-passes/pass-builder/shader-uri-list.json`。
标注：**O**=反编译/PDB 直接支持；**I**=调用链/命名组合；**U**=未恢复。

## 类型（O）

| 项 | 值 |
|---|---|
| 编译单元 | 499, 570 |
| `impl$0::build` | 0x140ae8070，size 8，源码行 18 |
| `startup_billboard_forest` | 0x140ae8080，size 578，源码行 27 |
| `closure$0` | 源码行 38（inlinee，IPI 0x6DC3） |

## 业务逻辑（O）

- **plugin build**：`SubApp::add_systems<Startup, startup_billboard_forest>`
  （`bevy_app::app::App::add_systems` 内联，源码行 301）。`impl$0` = bevy
  `Plugin` 实现（`BillboardForestPlugin`，I）。
- **startup 系统**（候选接口 O）：
  `(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>,
  Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`。
  - quad 网格索引：`from_iter<u32, FlatMap<TupleWindows<RangeInclusive<u32>,
    (u32,u32)>, [u32;6], closure>>`（每 `(i,j)` 窗口 → 6 索引，I：billboard
    quad 面片）。
  - `HashMap`（RandomState）创建。
  - `startup::load_mesh_into_library`（0x140b1aca0）+ `startup::
    load_shader_into_library<&str>`（0x140b3fe40）：billboard mesh/shader 入库。
  - `Commands::spawn_empty` → insert `(DrawableMeshBundle, DontCastShadow,
    KeepOnLevelUnload)`（0x78B 命令块，Transform 默认单位阵）。

## Pass 组合 / shader URI（O）

- draw 本体在 `draw_lists.rs` 闭包族（WP5 命名边界），非本模块。
- URI：`/billboard_forest.raster.hlsl` + `/billboard_plant.raster.hlsl`
  （compiled `_billboard_forest`/`_billboard_plant`，raster 变体
  `b903e43ffb3da915`/`f0adff76827e73ff`，全部 raster 双变体）。

## typed gap（U）

- billboard 具体 mesh 几何、shader URI 文件名、spawn 实体数。
- `startup_*` 系统参数经 `Gap` 承载（共享 crate 无 `ShaderWatch` 等类型）。

## 交付

- `src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs`
  （`BillboardForestPlugin::build` + `startup_billboard_forest` + `closure_0`，
  全部 `unimplemented!()` 骨架）。
