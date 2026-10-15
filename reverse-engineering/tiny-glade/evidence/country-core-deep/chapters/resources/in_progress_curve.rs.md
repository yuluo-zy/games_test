# resources/in_progress_curve.rs

> 文件作用：**编辑中曲线（徒手墙原始输入曲线）：raw 输入点累积、验证与终结曲线计算。wall 域（freehand 墙）为主消费者。**

函数总计 9：业务 3 / 内联无独立符号 5 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve` | 0x140ad41a0 | 0 | 由原始输入点计算终结曲线（重采样/拟合，16KB） | O | terrain-water\decomp\decomp-0x140ad41a0.txt
| `country_core::resources::in_progress_curve::InProgressCurve::new` | 0x140ad3070 | 62 | 构造空曲线（Vec 初始化，17KB 含验证分支） | O | terrain-water\decomp\decomp-0x140ad3070.txt
| `country_core::resources::in_progress_curve::InProgressCurve::try_add_raw_input_points` | 0x140ad3f50 | 134 | 先调 FUN_141466860 输入验证（返回 3 为合法），非法分支清字段；合法则批量加入原始输入点 | O | terrain-water\decomp\decomp-0x140ad3f50.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point`（行 126）— 最后一个原始输入点
- `country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point`（行 130）— 第一个原始输入点
- `country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws`（行 141）— 原始输入曲线世界长度
- `country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point`（行 146）— 给末点加权重（曲线权重累积）
- `country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid`（行 220）— 原始输入曲线有效性谓词

## 派生胶水（derive/宏生成；不计独立业务）

`clone` 等 1 项

## 实现方式（算法级）

- `new`（0xad3070）：空曲线 Vec 初始化。
- `try_add_raw_input_points`（0xad3f50）：先调 FUN_141466860 输入验证（返回 3 为合法），非法分支清两字段；合法则批量加入原始输入点。
- `compute_final_curve`（0xad41a0）：由原始输入点计算终结曲线（重采样/拟合，16KB）。
- 辅助（内联）：last/first_raw_input_point、raw_input_curve_length_ws、add_weight_to_the_last_point、is_raw_input_curve_valid。wall 域 freehand 墙为主消费者。
