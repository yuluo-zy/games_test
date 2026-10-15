# systems/ui_move_shapes.rs

> 文件作用：移动形状 gizmo：widget 名 'move_shape'；ui_move_shapes 主系统（12.8KB）处理 gizmo 拖拽→形状位置；move_gizmo_and_trigger_vol 施加 delta 并触发 volume 重算；MoveShapeCache 记录在途移动。

函数总计 8：业务 2 / 内联无独立符号 0 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_move_shapes::ui_move_shapes` | 0x14097a060 | 0 | 系统:周期性处理（ui_move_shapes） | O：12.8KB 主系统（wp20 + 本域复核；'move_shape' widget @0x142ab6c90） | ui-input\decomp\decomp-0x14097a060.txt |
| `country_core::systems::ui_move_shapes::move_gizmo_and_trigger_vol` | 0x14097d280 | 594 | 施加 delta + volume 触发 | O：move_gizmo_and_trigger_vol（round3 round2 补证：证据文件在 reconstruction/country-core-render/wp20/analysis/，11KB 反编译与该址 PDB 吻合，非缺失） | wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt（reconstruction/country-core-render/ 下） |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 6 项

## 实现方式（算法级）

- `ui_move_shapes` @0x14097a060（O，12.8KB，wp20 + 本域）：移动形状 gizmo 主系统。
  - widget 名 'move_shape'（@0x142ab6c90，串区 UNK_142ab6cxx）；处理 gizmo 拖拽 → 形状位置更新。
  - 5 个 jumptable（match 分发，按 GeoSource/形状变体）；事件输出经 `UNK_142c3a148`（bevy Events::send）。
  - `move_gizmo_and_trigger_vol` @0x14097d280（O）：施加 delta + 触发 volume 重算（11KB 反编译，wp20）。
  - `MoveShapeCache` 记录在途移动（I：由 move/freehand 两章同名缓存推断）。
