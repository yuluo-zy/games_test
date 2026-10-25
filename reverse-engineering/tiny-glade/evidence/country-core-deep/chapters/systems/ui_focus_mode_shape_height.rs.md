# systems/ui_focus_mode_shape_height.rs

> 文件作用：焦点模式形状高度 gizmo：游标射线投影调整所选形状高度，LerpedGizmoPosition 记录 lerp 位置（wp20 合同）。

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height` | 0x140a623e0 | 0 | 系统:周期性处理（ui_focus_mode_shape_height） | O：9.2KB（wp20 + 本域复核一致） | ui-input\decomp\decomp-0x140a623e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项

## 实现方式（算法级）

- `ui_focus_mode_shape_height` @0x140a623e0（O，9.2KB，wp20 + 本域复核）：焦点模式形状高度 gizmo。
  - 游标射线投影确定抬升量，调整所选形状高度；`LerpedGizmoPosition` 记录 lerp 位置（I：结构名来自 wp20 合同）。
  - gizmo 发图走 `FUN_140d79f90`（signifier emit，8 调用点）；常量区 UNK_1429edb90（clamp 25.0 系，见 ui_camera_mode_gizmos 闭包）/UNK_142ad36xx。
  - 相机/资源读取经 `FUN_1405b0d70`（hashmap get 族）+ 变换常量 UNK_1429cd1/1429258。
  - 细节分支未逐行（U）：各 gizmo 分支的精确抬升算法留待 wall/history 域闭合。
