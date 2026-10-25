# systems/wall/shapes/rectangle/finish_edit_rectangle.rs

> 文件作用：矩形编辑收尾：UiState80 快照先发；维度 state1/朝向 state2；改变判据 abs(acos(prev·cur))*57.29578>0.01 或维度差>0.01；记 ConcurrentEdits + niche8000000000000004；无变化到 DecoratorArchivist 恢复（192 全原调用验证）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle` | 0x141464a60 | 22 | finish_edit_rectangle（0.01 阈值双判据 + ConcurrentEdits + 恢复） | O | wall\decomp\decomp-0x141464a60.txt |

## 实现方式（算法级）

**finish_edit_rectangle（O，0x141464a60）**：9 参系统（param_7 等带 +0x1c 回写 = ResMut<ConcurrentEdits> 等）。处理链：

1. **UiState 变体**：`*piVar10`（param_2 指向的编辑状态）∈ {1(维度), 2(朝向), 0(无)}：
   - 非 0 时先 push **0x58B 快照**到 ConcurrentEdits Vec（`{variant u32, id u32, u64, 4×u32, 2×ptr, u64, 16B, u64, u64, u64}`）；
   - 变体 2 → 场偏移表 {0x44,0x40,0x34,0x30,0x2c,0x28,0x20,0x1c,0x18,8}；变体 1 → {0x4c,0x48,0x3c,0x38,0x34,0x30,0x28,0x24,0x20,0x10}；否则 **panic**（`FUN_1428d9430(&UNK_142c6305a, 0x28, &UNK_142c63088)`，"internal error" + finish_edit_rectangle.rs）。
2. **变更检测（双判据）**：
   - **朝向判据**：`|func_0x142923de0(prev·cur 点积类) | × 57.29578 <= 0.01`（acos→度；57.29578=0x42652ee1@0x142b1d78c、阈值 0.01=0x3c23d70a@0x142960674、abs 掩码 0x7fffffff@0x14292e980）；
   - **维度判据**：`SQRT(Δdims 平方和) <= 0.01`；
   - 两者都不超 → **无变化** → `FUN_140b2c4b0(uVar8, *puStack_108)`（**DecoratorArchivist::extract_before_after_all 恢复**）并返回。
3. **有变化**：`FUN_140a82b40(&uStack_100, ...)`（CompoundEditBuilder 开始）→ `FUN_141696f50(&puStack_238)`（步骤）→ `FUN_140a84ec0(uVar8, &puStack_238, &UNK_142c630b8)`（**record，路径 finish_edit_rectangle.rs**）→ 逐事件写墙曲线/enclosure（复用 edit_rectangle 的 _try_update_state 族）。
4. **事件迭代**：reader 差异钳 0（与 edit_rectangle 相同游标逻辑）；无事件 → 早退。

**常量表**：阈值 0.01 @0x142960674、度换算 57.29578 @0x142b1d78c、abs 掩码 0x7fffffff @0x14292e980。
**跨模块**：`FUN_140b2c4b0` = DecoratorArchivist（decorator 域）；`FUN_140a82b40/141696f50/140a84ec0` = history CompoundEditBuilder 族。
**边界/panic**：变体非 0/1/2 → panic 行 40（"internal error: entered unreachable code"）。
