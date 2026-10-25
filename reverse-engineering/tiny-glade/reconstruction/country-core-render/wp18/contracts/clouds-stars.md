# WP18 契约 — clouds.rs / stars.rs

版本 **B**。数据源：skeleton bundle + Ghidra 只读反编译（b1/b2）+ shader-uri-list.json。
标注：**O**=反编译直接支持；**I**=组合；**U**=未恢复。

## 类型（O）

| 模块 | 项 | 值 |
|---|---|---|
| clouds.rs | 编译单元 | 479, 542, 1585 |
| clouds.rs | `impl$0::build` | 0x14085b8a0，size 8，行 19 |
| clouds.rs | `startup_clouds` | 0x14085b8b0，size 585，行 24 |
| clouds.rs | `closure$0` | 行 35（inlinee） |
| stars.rs | 编译单元 | 502, 1598 |
| stars.rs | `impl$0::build` | 0x140903c90，size 8，行 19 |
| stars.rs | `startup_stars` | 0x140903ca0，size 585，行 24 |
| stars.rs | `closure$0` | 行 35（inlinee） |

## 业务逻辑（O）

- **plugin build**：`SubApp::add_systems<Startup, startup_clouds / startup_stars>`。
- **startup 系统**（候选接口 O）：`(Res<GraphicsDeviceResource>,
  Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>,
  Res<ShaderWatch>, Commands)`。逻辑同 billboard_forest：quad 网格索引
  （TupleWindows 闭包，**clouds 闭包被 billboard/stars 复用**）→
  `load_mesh_into_library` + `load_shader_into_library<&str>` →
  `spawn_empty` + insert `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload,
  DesignatedRenderPass)`（0x88B 命令块；含 `DesignatedRenderPass`——云/星走指定
  render pass，I）。实体局部：`uStack_70` = 0x42a95191（clouds）/0x42aaa031
  （stars）——transform 平移常量（I，差异 = 高度）。

## Pass 组合 / shader URI（O）

| 模块 | URI | compiled |
|---|---|---|
| clouds | `/cloud.raster.hlsl` | `_cloud.raster.hlsl_{b903…,f0ad…}` |
| stars | `/star.raster.hlsl` | `_star.raster.hlsl_{b903…,f0ad…}` |

云/星 draw 本体在 `draw_lists.rs` 闭包族（WP5 命名边界；云动画时间 uniform）。
catalog `sky_cloud` 族另含 `_skydome`（`/skydome.raster.hlsl`，月盘，本模块外 U）。

## typed gap（U）

- quad 网格确切尺寸（closure 具体窗口数）；spawn 实体数。
- 云/星 draw 的 uniform 语义（时间/风向/星密度）。

## 交付

`src/.../draw_shaders/clouds.rs`、`stars.rs`（Plugin build + startup + closure，
`unimplemented!()` 骨架）。
