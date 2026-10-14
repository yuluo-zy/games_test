# systems/mod_key.rs

> 文件作用：Modifiers 修饰键状态与吸附禁用开关：update_mod_key_state 系统读 ButtonInput<KeyCode>，按 7 组键跳表计算 ctrl/shift 组合 + 工具栏 Decor 模式区间（0x38..0x3f）判定，写 Modifiers{u32@0, byte@1=disable_snapping}（含 f32 阈值/按键沿触发）。

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::mod_key::Modifiers::disable_snapping` | 0x1408c5680 | 31 | 谓词/校验 | O：Modifiers::disable_snapping（ICF 折叠单行，与 ActiveInputMode::is_controller 同码；语义=读位标志组） | wall\decomp\decomp-0x1408c5680.txt |
| `country_core::systems::mod_key::Modifiers::is_disable_snapping_just_pressed` | 0x140af9bf0 | 40 | 谓词/校验 | O（手解）：`0f b6 41 01 c3` = 返回 byte [rcx+1] —— 位标志组第 1 字节某位（SOURCE_MAP 归名 is_disable_snapping_just_pressed） | ui-input\decomp\decomp-0x140af9bf0.txt |
| `country_core::systems::mod_key::update_mod_key_state` | 0x140af9c00 | 55 | 系统:周期性处理（update_mod_key_state） | O（手解）：ResMut<Modifiers> tick 回写；读键态 @+0x78 计数与 f32@+0x7c 阈值；键码索引 0..6 跳表 7 handler；Decor 模式 0x38..0x3f 区间判定并入；写 {u32@0, byte@1}。SOURCE_MAP 地址（重建代码写的 0x140af9000 = Toolbar::new，错） |  |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
