# systems/save/load_completion_checker.rs

> 文件作用：读档完成检查：SessionLoadReadiness（is_ok/deny_load 常量折叠）、is_session_ok_to_load / check_load_completion = 会话文件存在性检查（save_folder_path + File::create 探测）。

函数总计 5：业务 4 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::load_completion_checker::check_load_completion`` | 0x140a9f810 | 23 | 谓词/校验 | O：check_load_completion = save_folder_path + join + FUN_140d69590 + File::create×2（文件存在性/就绪检查；精确条件 I） | session-save\decomp\decomp-0x140a9f810.txt  |
| ``country_core::systems::save::load_completion_checker::SessionLoadReadiness::is_ok`` | 0x140833230 | 73 | 谓词/校验 | O：SessionLoadReadiness::is_ok（is_initial 同址折叠） | session-save\DIGEST.md  |
| ``country_core::systems::save::load_completion_checker::SessionLoadReadiness::deny_load`` | 0x140a9fa70 | 78 | 待推断 | O：deny_load = tag==1 判定（与 GladeType::is_olden 同址折叠） | session-save\DIGEST.md  |
| ``country_core::systems::save::load_completion_checker::is_session_ok_to_load`` | 0x140a9fa80 | 85 | 谓词/校验 | O：is_session_ok_to_load = save_folder_path×2 + join×2 + File::create×4（文件存在性检查） | session-save\decomp\decomp-0x140a9fa80.txt  |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
