# T3 CONTRACT — ui_draw_terrain 结尾状态机 jumptable（&UNK_142adc498）分支定案

> 逆向 agent ① 交付。decomp-0x140a9fdc0 行 163-165：`(*(code*)(&UNK_142adc498 +
> *(int*)(&UNK_142adc498 + uVar11*4)))(param_7)`。状态字节 `uVar11 ∈ 0..3`（cvar9 点按 subtype +
> 上帧 prev + inside 折叠）。
> 证据：表 0x142adc498 4 条目（带符号偏移）+ capstone 分支反汇编。状态：**可直接落地**。

---

## 1. 跳表条目（O，rne-read + 符号偏移）

| uVar11 | 表偏移（signed） | 目标 | 行为 |
|---|---|---|---|
| 0 | -0x203bd2b | **0x140aa076d** | **函数 epilogue（restore+ret）→ 无动作** |
| 1 | -0x203c3af | **0x140aa00e9** | **点按/新笔画**：写 hit 记录 + 查地形资源 |
| 2 | -0x203c263 | **0x140aa0235** | **移动/续笔**：get_last_mut_stroke + insert_or_morph_last_point |
| 3 | -0x203c1d1 | **0x140aa02c7** | **释放/读笔**：get_last_mut_stroke + 遍历点集（12B/点） |

## 2. 分支行为（O，capstone）

- **state 0**：直接 `movaps restore; add rsp; pop; ret` = 本系统结束，无额外动作。
- **state 1（点按，0x140aa00e9）**：
  `[rcx+0x1c]→*[rcx+0x10]`（EventWriter 写回）→ `rcx=[rcx]`（stroke 资源）→ `call 0x140924a10`
  （点按判定）`cmp eax,1` → 命中：`*[rdi]=1, [rdi+4]=pos`（写 `HoveredTerrainStrokeId{tag=1,id}`）→
  `call 0x140969720(&UNK_142adc2e8? 0x218e2d8 相对)`（地形资源查找）→ 启动 new_stroke。
  = terrain-DELIVERY 已落地的**点按分支 new_stroke**（cvar9==3 对应）。
- **state 2（移动，0x140aa0235）**：`call 0x140924900` = `get_last_mut_stroke`；
  存在且 bl 标志 → `call 0x140a97880` = `insert_or_morph_last_point` → 续笔/morph 落点。
  = DELTA §4 "insert_or_morph_last_point（0x140a97880）消费待接" 的**消费点**。
- **state 3（释放/读笔，0x140aa02c7）**：`call 0x140924900`；stroke 存在 →
  `rax=[rax+0x10]`（点 len）× `0xc`（Vec3 stride）→ 遍历点集。= 完成笔画/回读路径。

## 3. ② 待办（与现实现对照）

- 现 `ui_draw_terrain.rs` 状态机已把点按分支（state1/new_stroke）落地；**state2（续笔 morph）
  与 state3（释放读笔）行为**若未实现，按上表补：
  - state2 → `get_last_mut_stroke`（0x140924900 语义）+ `insert_or_morph_last_point`
    （0x140a97880 语义，morph 由 `movss xmm2,[rip+0x1e85675]` 常量驱动）。
  - state3 → `get_last_mut_stroke` + 点集遍历（12B/点 = Vec3）完成收尾。
- state0 = 无动作（现实现 `_ => return` 等价）。

## 4. 证据索引
- decomp-0x140a9fdc0.txt 行 134-165（状态机 + 跳表调用点）。
- rne-read 0x142adc498 表 8 条目 + capstone 4 分支反汇编（本契约 §1 表）。
- 函数 0x140924a10/0x140924900/0x140a97880 = ui_draw_terrain 内联查询族
  （DELTA §4 已登记 get_last_mut_stroke/insert_or_morph_last_point）。
