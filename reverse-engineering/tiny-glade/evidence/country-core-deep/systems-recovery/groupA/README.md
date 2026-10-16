# 组 A 审查基线（reviewer-A，2026-10-16）

> 独立审查 agent。不改 src；只写证据与报告。审查对象：restorer-A 交付的
> `tiny-glade/crates/systems/{water,forest,glade-border,timberframe,tree-tool,color}/`。
> 判定标准：**机器码/骨架 VA/章节 O 事实为准，不盲信 RESTORE.md**。

## 专属 database id

- 任务指定 `tg_vA1`，但 RE-MCP 校验 `[a-z][a-z0-9_]{0,31}`，大写被拒
  （实测 `Invalid database_id 'tg_vA1'`）。
- 实际使用 **`tg_va1`**（小写、唯一、不与 playbook tg_a1..tg_a8 冲突）。所有 job 用此 id。

## 权威基线文件（本目录）

- `BASELINE-function-list.md` — SOURCE_MAP-systems.json 提取：6 crate 全部符号
  （business/glue/inline/dep）+ 每个 business 函数 VA。**缺失检测对照表**。
- `BASELINE-signature-surface.md` — 骨架 B 各文件「文件级接口候选」+ 函数符号 + 首条
  VA 提取。**签名/参数面对照表**（候选接口经 caller 类型引用绑定，比 PDB 占位可信）。
- `decomp/` — 审查用 fresh RE-MCP 反编译落盘。
- `jobs/` `out/` — RE-MCP job 与结果。

## 审查清单（每 crate 一份 REVIEW.md）

1. **签名/参数面**：实现函数参数 vs BASELINE-signature-surface 候选接口 + fresh decomp
   （Ghidra 参数个数/寄存器面）。揪出自造签名（顺序错、Res/ResMut 反、缺参、多参）。
2. **接口契约**：调用的 core 类型字段/布局 vs 章节证据（water.rs.md/color_id.rs.md/
   tree_storage.rs.md/glade_border.rs.md 等）；未知字段必须诚实标注，不得填假数据。
3. **偷懒检测**：空壳/占位/todo!/假实现（如恒返回 default、空循环）。
4. **缺失检测**：BASELINE-function-list business 函数 vs 实现逐一对齐；区分
   依赖实例（zeno/rhapsody GraphicsDevice 实例标 business 属分类噪音，不要求还原）。

## 判定

- **OK**：与机器码/接口证据一致，业务逻辑闭合。
- **FIX**：有偏差但可修（签名错/字段错/逻辑简化过度），列具体修法。
- **GAP**：未实现/未调查清楚/证据不足，需 restorer 补或 re-analyst 深挖。

## 评审模板（REVIEW.md 结构）

```md
# REVIEW: <crate>
审查日期 / 交付 SHA（git log -1 <crate>）/ 审查人 dbid
## 0. 交付面
## 1. 逐项核对表（函数 / SOURCE_MAP VA / 声称实现 / 实测判定 OK-FIX-GAP）
## 2. 签名/参数面核对（实现 vs 骨架候选 vs fresh decomp）
## 3. 接口契约核对（core 类型字段 vs 章节证据）
## 4. 偷懒/缺失清单
## 5. 结论与 FIX 项（发 main 转 restorer-A）
```
