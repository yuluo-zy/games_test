# resources/shader_refactoring_state.rs

> 文件作用：**着色器重构开关状态。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::shader_refactoring_state::ShaderRefactoringState::disabled | 0x140b1f210 | 37 | 0xb1f210：fn disabled(&self)->bool（[+0]==0；重建代码构造器模型错） | O |  |

## 实现方式（算法级）（O，decomp 手解）

**ShaderRefactoringState::disabled @0x140b1f210**：`[+0]==0 → bool`——**方法不是构造器**：
`fn disabled(&self) -> bool`（判当前是否处于禁用态）；重建代码构造器模型错误
（§4 #4）。O
