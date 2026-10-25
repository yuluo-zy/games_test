# resources/roofs/commands.rs

> 文件作用：**屋顶编辑命令枚举**——`RoofCommand` 系（建造/修改/删除屋顶的指令，经 history roof_edit 执行）。
> 本文件仅含枚举的 Debug/fmt 派生（2 个 fmt），无独立业务函数；命令的消费在 systems 层与 resources/history/roof_edit.rs。

函数总计 2：业务 0 / 内联无独立符号 0 / 派生胶水 2

## 派生胶水（derive/宏生成；不计独立业务）

`fmt` 等 2 项（impl$2::fmt / impl$4::fmt，`#[derive(Debug)]`；impl$4 有 VA 0x14258e640）
