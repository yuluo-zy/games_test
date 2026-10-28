# crates/frozen/src

状态：依赖/归属边界。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：dependency_or_unresolved_local_ownership。

先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。

## 接线与验证

接线：not_established。

依赖实现不计游戏逆向覆盖。

## 已恢复的函数和类型范围

原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。

## 明确缺口

- 来源/必要分叉差异待核实，公共实现复用

下一闭包：来源/必要分叉差异待核实，公共实现复用

## 证据

- [DEPENDENCY_FILTERING.md](../../docs/current/DEPENDENCY_FILTERING.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/frozen/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

