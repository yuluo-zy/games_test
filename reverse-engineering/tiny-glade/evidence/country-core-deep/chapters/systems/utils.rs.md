# systems/utils.rs

> 文件作用：**systems 层调度辅助。`add_stages` 用 `enum_iterator::All<GameSystemSet>` 枚举全部系统集并调用 `App::configure_sets(Update, ...)`。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::utils::impl$0::add_stages` | 0x140945ed0 | 12 | `fn add_stages(app: &mut App)`：`FUN_14130f350` 构造 `All<GameSystemSet>` 迭代器（`&UNK_142ab2438` 序列数据，判别基址 `0x8000000000000000` + 变体值），载入变体序号表（0x8=8 项，`_UNK_142ab24f0..`），`FUN_140b68190(app, &seq, &UNK_142ab2b10)` = `configure_sets(Update, all)`。 | O | decomp/decomp-0x140945ed0.txt |

## 实现方式（算法级）

**add_stages @0x140945ed0（O）**：
1. `FUN_14130f350` 构造 `All<GameSystemSet>` 迭代器：载入变体序号表（0x8=8 项，`_UNK_142ab24f0..`），判别基址 `0x8000000000000000` + 变体值。（O）
2. `FUN_140b68190(app, &seq, &UNK_142ab2b10)` = `App::configure_sets(Update, all)`（all=全部 GameSystemSet）。（O）

**数据结构（O）**：变体表 8 项与 GameSystemSet 8~9 变体量级一致；重构 `add_stages`（enum_iterator::All<GameSystemSet> → configure_sets(Update)）**完全证实**。（O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（add_stages 行 19 闭包）

## 备注

- 重构 `add_stages`（enum_iterator::All<GameSystemSet> → configure_sets(Update)）**完全证实**。
- 变体表 8 项（0x8）与 GameSystemSet 8~9 变体量级一致。
