# systems/ui_clutter/ui_focus_decoclutter.rs

> 文件作用：焦点模式 deco-clutter 移动/旋转（RequestMoveClutter/RequestRotateDecorator/LastClonedClutter，4 个 closure；wp20 合同）。

函数总计 6：业务 1 / 内联无独立符号 0 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter` | 0x1409a92c0 | 61 | 系统:周期性处理（ui_focus_decoclutter） | O：11.3KB（wp20 + 本域复核） | ui-input\decomp\decomp-0x1409a92c0.txt（已补；58KB 反编译，10 参数 ECS 系统） |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项

## 实现方式（算法级）

- `ui_focus_decoclutter` @0x1409a92c0（O，11.3KB，wp20 analysis 0x1409a92c0_ui_focus_decoclutter.txt）：焦点模式 deco-clutter 移动/旋转。
  - 消费 `RequestMoveClutter` / `RequestRotateDecorator` / `LastClonedClutter`（O：wp20 合同），每请求移动/旋转对应 clutter 实体。
  - 4 个 closure（O：闭包单态）——按请求变体分发：移动 gizmo 沿射线、旋转按光标角增量。
  - 与 ui_edit_focus 的 deco/clutter 路径联动（O：DIGEST 串区）。
