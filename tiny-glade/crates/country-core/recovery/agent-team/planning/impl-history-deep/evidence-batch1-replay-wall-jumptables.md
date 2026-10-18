# batch1 evidence — replay_wall 17-op jump tables (item 1)

fresh 反编译（2026-10-17，impl-history-deep，ghidra shared batch1）：
- `WallReplayEditCmd::process`（0x140abfb30）为一巨型函数（0x140abfb30..0x140ac0a36），
  含 17 操作分派 + 各操作 inline body。Ghidra/ReVa 均只在 DB 中建模 400B dispatch 壳。
- 分派：`done byte@+0x15`；`*&1` 定时位 → cubic_in_out + 跳表 0x142adf44c；
  瞬时 → 跳表 0x142adf408。op 号 = niche 解码（edit+0x10 < -0x7ffffffffffffff0 →
  lVar1 = v - 0x7fffffffffffffff；否则 0），lVar1 ∈ 0..0x10。
- 两表各 17 项（0x142adf408 与 0x142adf44c 相邻，各 68B）。

## 瞬时表（base 0x142adf408）→ 目标地址

| op | 目标 |
|---|---|
| 0 | 0x140ac0366 |
| 1 | 0x140ac02a2 |
| 2 | 0x140ac04b3 |
| 3 | 0x140ac042c |
| 4 | 0x140ac0253 |
| 5 | 0x140ac0612 |
| 6 | 0x140ac0667 |
| 7 | 0x140ac05b1 |
| 8 | 0x140ac0484 |
| 9 | 0x140ac0699 |
| 10 | 0x140ac0635 |
| 11 | 0x140ac06c8 |
| 12 | 0x140ac0224 |
| 13 | 0x140abfc80 |
| 14 | 0x140abfc80 |
| 15 | 0x140ac15b7 |
| 16 | 0x140ac05e3 |

## 定时表（base 0x142adf44c）→ 目标地址

| op | 目标 |
|---|---|
| 0 | 0x140abfcd3 |
| 1 | 0x140ac1576 |
| 2 | 0x140ac1576 |
| 3 | 0x140abfdbb |
| 4 | 0x140ac1576 |
| 5 | 0x140abfcb9 |
| 6 | 0x140abfe85 |
| 7 | 0x140abfecc |
| 8 | 0x140ac1576 |
| 9 | 0x140ac1576 |
| 10 | 0x140ac1576 |
| 11 | 0x140ac1576 |
| 12 | 0x140ac1576 |
| 13 | 0x140abfc2f |
| 14 | 0x140abfc2f |
| 15 | 0x140ac15b7 |
| 16 | 0x140ac1576 |

注：多数定时项指向 0x140ac1576（共享 handler）；两表首项（op 0）分别指向
0x140ac0366/0x140abfcd3 的独立 inline 体。op 13/14 共享 0x140abfc80（瞬时）/
0x140abfc2f（定时）。这些目标均为 process 函数内的 inline 代码，Ghidra 未单独建函数。

## 结论（分类）

17 操作分派为**真实业务**（多分支跳表 + 每 op 独立控制流）。待办项 1 的
`advance_wall_groups_observed 全 17 op` 需 recovered_building `WallHistoryEdit`
全变体（当前 src 仅 Create/Delete/EnclosureChange/StateChange 4 变体，op 号经
`observed_operation` 前 4B 传于 StateChange）。按红 line 不改他模块，本处登记
完整分派映射 + 目标地址证据，由 recovered_building 侧据此扩展变体（其
`advance_wall_transition` 已实现 op 0/1/2，见 src/recovered_building/
wall_history_transition.rs）。

证据落盘：
- results/batch1-wall-jumptables.json（read_memory 两表 + process/replay_wall decomp）
- results/batch1e-find-wall-funcs.json（process 全 disasm 349 指令）
- decomp/0x140abfb30-reva-full.txt（process ReVa 定点，仅 400B 壳）
- decomp/0x140abfb30-disasm.txt（全区域 disasm，含 op bodies inline）
