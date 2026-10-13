# WP16 — country-core misc resources + systems residual（Tiny Glade B）

本包恢复 country-core 渲染域的**剩余杂项资源与系统**：常春藤、软件 BVH 光线
追踪、窗光/颜色索引、backend/collision/debug/paths/steam/ivy 系统、非渲染
asset_libraries、components 残族，以及 geometry/utils 游戏自有工具残量。
交付为**骨架对齐 Rust + 业务逻辑合同**；机器码差分仅作数值/边界裁决，不作
实现手段（见项目工作流 `workflow_logic_then_skeleton.md`）。

## 目录

```
wp16/
  contracts/   6 份业务逻辑合同（O/I/U + typed gap）
  src/         骨架对齐 Rust（55 个 .rs）
    resources/   ivy / rt / window_lights / color_id
    systems/     backend / collision / debug / paths / steam / ivy
    asset_libraries/  meshwatch / configwatch / texture_watch / vao_library /
                      placement_data_library / core id
    components/  mod（45 标记组件）/ transient_mesh / geometry / utils
    boundary.rs  其它 WP 已有渲染类型（wp2/4/7）的接线面
    reused.rs    被复用依赖（hotwatch/rapier/steamworks/bracket-noise/…）的
                 游戏侧调用面（不重实现）
  Cargo.toml   独立 offline gate（edition 2024，rust 1.86）
  evidence.json / evidence/   反编译证据 + bundle 汇总
```

## 门禁

```sh
cargo check --offline     # 0 errors / 0 warnings（55 源文件）
```

只证明骨架对齐表面闭合；全部 `implemented_unverified`，无 runtime/GPU 验收。

## 与既有占位的对账

- `resources/window_lights.rs`、`resources/color_id.rs`（wp6 占位）：**扩展**为
  资源注册面（register_required_components）+ `ColorIds` 集合（`new` /
  `iter_nondefault` / `default_value`）+ 三个色 SSBO 载体。
- `asset_libraries/mod.rs`（wp4）：`Asset`/`Handle`/`HandleId` 已恢复，本包
  重述非渲染面并接线 watch/VAO/放置数据注册表。
- `components/drawable.rs`（wp6 nani 可绘制族）：**不重复**；本包补
  `transient_mesh` + 45 个标记组件 + geometry/utils 残量。
- `render/{vao,sparse_instance_buffer,world_raster,frame_constants,view_constants,
  mesh,ssbo_library}`（wp1-7）：以 `src/boundary.rs` 接线面引用，未重恢复。
- ivy/rt/window-lights/color_id 与 `systems/{backend,collision,debug,paths,steam,
  ivy}` 骨架文件一一对应（同名模块）。

## 集成说明

- 集成 owner 用整合 crate 真实类型替换 `src/boundary.rs` 门面，用真实依赖替换
  `src/reused.rs` 表面；两处均有 O 标注引导。
- 未知字段一律 `crate::Gap`（typed gap），未臆造。
