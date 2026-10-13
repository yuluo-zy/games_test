# 合同：resources::{window_lights, color_id}（窗光 / 颜色索引 CPU 状态）

源 SHA256（B skeleton）：`resources/window_lights.rs`、`resources/color_id.rs`
Rust：`src/resources/window_lights.rs`、`src/resources/color_id.rs`

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## window_lights

- PDB 实体 4：`WindowLight` / `GlobalWindowLightsControl` / `LightInfo` /
  `GlobalSmokeControl`（O 命名实体，均无自有非噪声过程）。
- `WindowLight` 组件（O：decorator 系统给窗实体挂载；glass 绘制
  `nani_update_solid_vertex_color_window_glass` 读 `&WindowLight`；延迟光照分箱
  `bin_lights` 以窗灯为光源 —— deferred light_binning lantern 源）。字段布局 U
  （灯颜色/功率/范围 typed gap）。
- `impl$2::register_required_components`（0x14086cac0）+ `impl$2::clone_behavior`
  （0x2370，空）—— 注册伴随组件集合；伴随清单 U。
- `LightInfo`（O 实体；bin_lights 光源描述）与 `GlobalSmokeControl`（O 资源；
  烟系统读）字段 U。

## color_id

- `ColorId` = u32 newtype（O：wp6 已恢复；serde newtype `visit_newtype_struct`）。
- `WallColorIds` / `GateColorIds` / `HalftimberColorIds`（O 资源）：
  `new`（0x140b19890 / 0x140b19900 / 0x140b19970）建空表；
  `iter_nondefault`（O：save/snapshot 0x17a0430 内联）滤默认色；
  键类型 I：u64（墙/门/半木 id）。
- `impl$3::default_value`（0x14099ca50）→ 默认 `ColorId`（0）。
- `{Wall,Gate,Halftimber}ColorIdSsbo`（O：AssetSsboLibrary `add_new`/
  `get_rg_buffer` + draw_builder `read_storage_buffer`）—— 上传非默认色集。
- `Set{Gate,Halftimber,Wall}ColorIdCmd`（O 实体）—— 修改命令。
- `impl$12::register_required_components`（0x140b1cf80）/ `clone_behavior`
  （0x140b1d670）—— 组件注册面。

## 未知（U）

- `WindowLight`/`LightInfo` 字段布局；`ColorIds` 的键精确类型与内表结构；
  `iter_nondefault` 上屏顺序；SSBO 元素打包（假定 u32 每 id）。
