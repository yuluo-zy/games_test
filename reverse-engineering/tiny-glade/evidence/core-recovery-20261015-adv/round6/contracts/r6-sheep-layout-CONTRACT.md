# Round6 CONTRACT — sheep 完整布局 + movement（stride 0x68）

> 逆向 agent ① 交付。证据：sheep-SPEC §2.1/§2.6 + snapshot-analysis + 各 decomp 读面。
> 状态：**已证偏移可直接落地；+0x00..0x2f 等区间待取证（动画/SSBO 后置候选）**。

---

## 1. Sheep 布局（stride 0x68 = 104B，O 读面）

| 偏移 | 宽 | 语义（O） |
|---|---|---|
| +0x30 | 12B | **SheepMoveState**（判别@+0x30：0=Idle、1=Walking；+0x31 force_reach_goal bool、+0x38 timeout_s f32） |
| +0x58 | 4B | **being_petted f32**（谓词 0x14089d4c0 = `[+0x58] > 0.01`，常量 0x142960674=0.01） |
| +0x00..0x2f | — | **U**（可能动画态/SSBO 后置） |
| +0x3c..0x57 | — | **U** |
| +0x5c..0x67 | — | **U** |

- capture/restore：`apply_sheep_snapshot` 0x1416f24f0、capture iterator 0x1412f03f0（`uVar*0x68` 步进）。
- `SheepControlSnapshot` = `{"sheep":[...]}`（0x1416f9fd0 写键 "sheep" len5）；字段序 position, goal,
  move_state（0x1416f9eb0 依次 serialize_entry）。
- ② 现源 Sheep{move_state@+0x30, being_petted_timer@+0x58} 与已证偏移一致；未知区间不填。

## 2. movement 状态机 — **待取证（中优先）**

- Idle↔Walking 转移不在 country-core sysres map（SPEC §2.6）；sheep_controls 0x1416f11a0 可见
  写 move_state tag=Walking+force_reach_goal 与 timeout_s；picking 分支（move_state=0x101）待证。
- ② 保持不造；spawn/movement 随布局完整后接。

## 3. X4 羊 Query（sync save 0x140a3f470 sheep 捕获段）— **待取证（中优先）**

- 活羊 → SheepControlSnapshot 的 Query 面未在 into_configs 现形；拆 decomp-r5-0x140a3f470 中
  sheep 捕获段可定（SheepControlSnapshot 字段序 O：position, goal, move_state）。

## 4. 证据索引
- sheep-SPEC §2.1（stride 0x68 读面表）/§2.6（movement 待证）；snapshot-analysis（判别字节）。
- decomp：0x14089d4c0（being_petted）、0x1412f03f0（capture iterator）、0x1416f24f0（apply）。
- crate resources/critters/sheep.rs（Sheep 现源）、sheep/control.rs（SheepControlSnapshot）。
