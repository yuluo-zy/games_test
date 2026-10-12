# 原作渲染源码骨架的 PDB 审计

原作 `tiny_glade.pdb` 与 `tiny-glade.rne` 的 SHA256 已重新核实，分别为 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`、`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配项目基线。没有使用 C / 1.16 构建，也没有修改游戏实现或把通用库实现当作游戏恢复成果。

本次实际读取 **389 个 rhapsody/country_core 编译单元**的文件、行号和符号记录；原始 LLVM 输出保存在 `raw/module-*.txt.gz`。该范围发现 **93 个原源码文件路径及 SHA256**：`crates/rhapsody/src` 55 个，`crates/country-core/src/render` 38 个。

`native-procedures.json` 保存 4,124 个被筛选的 native procedure 记录，其中 **432 个名称以 rhapsody 开头、765 个以 country_core::render 开头**；其余 2,927 个含内联源文件贡献、泛型实例或引用渲染器类型的依赖符号。后者不能计入私有渲染器源码恢复数量。记录保留模块、CodeView 记录偏移、section/offset、原机器码地址、准确 code size 和原行号贡献。

所有上述 native procedure 的 PDB 类型均为 **`0x1001 (void ())`**。实际独立读取该 TPI 类型，结果为 `LF_PROCEDURE: return void, # args = 0`，参数列表为空；见 `raw/native-procedure-debug-type.txt`。例如实际需要输入访问位掩码并返回 Vulkan usage 标志的函数，也使用这一空签名。因此它是调试记录丢失 Rust 签名信息的直接证据，不能据此生成 `fn foo()` 或声称参数、返回类型完全恢复。

`files.json` 按原文件记录所有观察到的 SHA256、编译单元和行号区间。`public-symbols.json` 则从既有 `evidence/render-architecture/public-symbols.json` 中筛出 860 条相关公开符号，保留符号原文及地址；它不是一次新的全 PDB 公开符号枚举。

已在 `reconstruction/render/original-skeleton/crates/` 建立对应的 **93 个 Rust 注释文件**。每个文件包括原路径、原 checksum 和贡献函数地址；没有发明 Rust 签名或实现，也没有加入 Cargo 编译。函数与文件的关系依据行号范围重叠，重叠可能来自内联，不能解释成确定的主定义文件。

## 能否称为“完全还原骨架”

目前可以称为“原 PDB 已编译贡献文件层级及局部 native 函数边界索引”，不能称为完整可编译源码骨架。下列信息仍需逐项恢复或补充其他证据：

- Rust 函数参数/返回类型、泛型约束、trait 关系、所有权与生命周期。
- 结构体、枚举布局和字段语义，以及调用边界的数据不变量。
- 内联函数、被优化掉的函数、未编译的文件与条件编译分支。
- 原始方法与多份泛型机器码实例、别名之间的一对多关系。
- 限定编译单元外的渲染调用者、私有辅助 crates 和着色器源码。
- 完整资源生命周期与 Vulkan 同步行为，以及实际渲染效果验证。

Vulkan/ash 公开 API 可以准确复用，PDB 的名称、原文件路径、行号和机器码边界也显著降低定位成本。它们仍不能自动补全上述游戏私有状态与签名。一遍编译可以证明代码通过 Rust 类型检查，不能独立证明它与原作行为一致。

## 本轮精确叶函数边界

| 原函数 | 地址 | 原 PDB code size | end exclusive | 模块 |
|---|---|---:|---|---:|
| ResourceInfo::link_up_next_access_chains | `0x140bd47d0` | 136 | `0x140bd4858` | 676 |
| ResourceInfo::calculate_lifetime_events | `0x140bd4860` | 359 | `0x140bd49c7` | 676 |
| image_access_mask_to_usage_flags | `0x140bd49d0` | 292 | `0x140bd4af4` | 676 |
| buffer_access_mask_to_usage_flags | `0x140bd4b00` | 50 | `0x140bd4b32` | 676 |
| PassResourceAccessType::can_be_concurrent_with | `0x140bece70` | 84 | `0x140becec4` | 700 |

前四个在 `crates/rhapsody/src/rg/resource_info.rs`，原 checksum 为 `0f92cb3e59af073ef66c194a9ebbaafc349625df27209c37cb85962ac5539f4c`。最后一个在 `crates/rhapsody/src/rg/recorder.rs`。原始 CodeView 记录以本索引的对应 raw dump 为准，不能采用 Ghidra 把多个函数合并后产生的更大函数范围。

可复现脚本：`scripts/render-skeleton/inventory.py`。脚本只是证据整理工具；93 个注释文件和 4,124 条 native 记录均不计为已实现源码。
