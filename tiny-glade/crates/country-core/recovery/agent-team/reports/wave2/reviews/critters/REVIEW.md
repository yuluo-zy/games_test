# REVIEW — wave2 critters 状态跳表族与命中码语义落库核验

- reviewer：`reviewer-wave2-critters-anim`
- 日期：2026-10-17
- 依据：`reports/wave2/critters/RESTORE.md` + `reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md` §1/§6 + `evidence/`（handlers-batch1/handlers-butterfly/5 个 asm + 常量读 + 6 张跳表）。
- 方法：逐函数 **fresh 反编译**（ghidra `--worker shared` 批量 7 函数，结果 `evidence/results/review-fresh-7funcs.json`）+ **跳表字节实读解码**（0x14304b0f8/0x14304b178/0x14304b638/0x142f68008/0x142f68018 全部复算）→ 与 Rust `crates/systems/critters/src/{shared,sheep/sheep_walk_animation,sheep/sheep_pet_animation,butterfly,wren}.rs` 逐语句判等。
- 未改 src、未改库。

---

## 裁决总表

| 函数 | 地址 | 判等 | 需修 | 未实现(U，登记核验) |
|---|---|---|---|---|
| `move_critters` 命中码 | 0x142318fa0 | 判等（命中码语义） | 0 | 外层门 `(local_fc\|\|local_100)`、AL 标志（**补充登记**） |
| `critters_avoid_collisions` 命中码 | 0x1423179a0 | 判等（命中码语义） | 0 | water8 映射（登记准确）；墙 relate 循环语义（模块头已登记，RESTORE 未单列） |
| `sheep_walk_animation` | 0x14231a630 | 判等（主循环+2 handler 逐语句） | 0 | 音频槽写（登记准确） |
| `sheep_pet_animation` | 0x14231ac20 | 判等（5 handler 逐语句） | 0 | 音频槽写（登记准确） |
| `set_butterfly_velocities_towards_goals` | 0x141d867b0 | 判等（vel 表 4 handler 逐语句） | 0 | R9/RDX 槽写（登记准确） |
| `update_butterfly_states` | 0x141d84d40 | 部分判等（主循环/step_state/state1/3 结构） | 0 | state0/2、state1 水/墙/xmm1==xmm14 门、rng 槽写（登记准确）；state3 NaN 次差（补充登记） |
| `update_wrens` | 0x14231e390 | 部分判等（h0/h1/h2/h4 结构 + h3 公式） | **2** | 距离查询/音频 call 载荷/panic 路径（登记准确）；**anim_val ×7 未登记** |

**判等 4 全量 + 2 部分判等；需修 2（wren h3 anim_val、wren h3 移动 y/z 精确式）；新增登记 4 项。**

---

## 1. shared.rs 命中码语义

### 1.1 `move_critters`（0x142318fa0）
fresh 反编译（`review-fresh-7funcs.json`）与 RE-ANALYSIS §6.2 一致：
```c
legacy_spherecast();
if ((local_318 != 2) && (球距 < 1.0)) goto LAB_142319506;   // 命中码2不阻塞，球距<1.0才阻塞
```
Rust `shared.rs:144` `sphere.hit && sphere.hitcode != 2 && sphere.distance < 1.0` → **判等**。

spherecast 门（fresh 原文）：
```c
if (((local_fc & 1) != 0) || ((local_100 & 1) != 0)) {        // ①外层门
  if ((6 < *puVar29) || ((99U >> (*puVar29 & 0x1f) & 1) == 0)) // ②内层门
    { ... spherecast ... }
}
```
Rust `shared.rs:136` `(locomotion.0 > 6 || !locomotion.wall_hash_only())` 只复现 **②内层门**；**①外层门 `(local_fc||local_100)`（Duck/Sheep Option 存在标志，映射 U）被省略** → 二进制仅对带 Duck/Sheep 组件的实体 spherecast，Rust 对查询内全部实体 spherecast。RESTORE FIX① 声称的门 `(loc>6)||!wall_hash_only()` 不完整。**补充登记（非近似，是门条件缺失）**。

loc==2 岩石/水路径：fresh 同 §6.2 —— `(AL || rock>0) || (water<=0 & water2) → 阻塞`；Rust `rock || water<=0`（water2 以水<=0 近似、AL 未建模）。**登记准确**。

### 1.2 `critters_avoid_collisions`（0x1423179a0）
fresh 反编译：
```c
legacy_spherecast();
if ((local_258 == 2) || (球距 >= 1.0)) goto LAB_142317e4f;   // 命中码2 或 球距>=1 → 保留目标
if ((fVar25 < 0.4) && ((local_fc & 1) != 0)) write<DespawnDuckCmd>();  // <0.4 && water8
```
Rust `shared.rs:281-287` 与之一致 → **判等**。

water8/local_fc 映射 `!samples_rock_mask` 近似 → **登记准确**。墙 relate 循环：fresh 显示 `do{...}while(relate != 0)`（任一墙 relate==0 → 清目标；遍历完 → 保留），Rust 为"任一 relate != Disjoint → keep"——**模块头（shared.rs:32）已登记，RESTORE 残留表未单列，补注**。

---

## 2. sheep_walk_animation（0x14231a630）—— **判等**

- 主循环（fresh）：`speed = |target_pos - 记录位|/dt` 写 +0x48；`speed<=0.2：state==1→(2,0)` 否则保持；`>0.2：state=1`（state==1 保持 progress 否则 progress=speed）；`being_petted` 假 → 跳表 0x14304b0f8 分派后回主循环续下一羊。Rust `sheep_walk_animation.rs:58-91` 逐语句一致。
- **跳表实读** 0x14304b0f8：slot0/3=0x14231a729、slot1=0x14231aa29、slot2=0x14231aade（4 项，基址差复算全对）。
- **h1（0x14231aa29）**：`accum[+0x5c]+=dt` → `call fmodf(accum,1.25)`（0x142a8d600=0x3fa00000）→ `mulss ×24.0`（0x142934600=0x41c00000）→ cvttss2si 饱和序列（0x14295a0dc=0x5f000000 / 0x14295a0e0=0x5f7fffff）→ `[+0x28]=(idx+n*20+3) mod 30`（`n=[+0x20]`，除 30 魔数 0x8888888888888889 >>4）。Rust `sheep_walk_h1` 逐语句一致。
  - 文档修正：饱和序列是 **u64（fptoui）** 非 i64；accum∈[0,1.25) 时两值一致，无行为差。`n as i64` wrap vs 二进制 unsigned，仅 n≥2^63 才分歧（pet_target 种子小，实际不触发）。
- **h2（0x14231aade）**：同 accum/fmodf/idx/帧索引；`progress += dt/0.1`（0x142925984=0x3dcccccd）；`ucomiss; jae → state=3`（NaN → 保持 2 写 progress）。Rust `sheep_walk_h2` 逐语句一致（`progress>=1.0` NaN=false → state=2，同）。
- 音频槽写（`mov [r13],ebp` 查询列值）不建模 → **登记准确**。

---

## 3. sheep_pet_animation（0x14231ac20）—— **判等**

- 主循环（fresh）：`from_iter` 收集事件目标 → 取首羊 → `pet_target(+0x20)` 命中匹配标志 r11b → 跳表 0x14304b178 按 `phase(+0x18)` 分派，参数 (0x3e99999a,0x3f800000,0x3e4ccccd)=(0.3,1.0,0.2)。Rust `sheep_pet_animation.rs:43-55` 一致。
- **跳表实读** 0x14304b178：slot0-4=0x14231afe4/0x14231b0b3/0x14231b022/0x14231b068/0x14231b006（复算全对）。
- **h0（0x14231afe4）**：`test r11b; je` → 匹配才 `[+0x60]=1`、**`mov qword [r13+0x18],1`（qword store 同时清零 +0x1c progress）**、phase→1。Rust `advance_sheep_pet` phase0 写 control_flag=1、pet_phase=1、pet_progress=0 → **判等**。RESTORE 声称"二进制 h0 不写 +0x1c"**不准确**（qword store 确实清零 +0x1c），但结论（Rust 匹配、无行为差）成立——登记修正。
- **h1**（dt/0.3、>1→phase2/progress0、amplitude=progress 未钳制）、**h2**（dt/0.2、`!(progress>1)||matched` 保持、否则→3、amplitude=1.0）、**h3**（dt/0.3、>1→4 不写 progress、+0x60=0、amplitude=1-progress）、**h4**（phase→0、amplitude=0）：Rust 逐语句一致（含 NaN 分支：h1 `seta`、h2 `setbe`、h3 `ja` 均与 Rust `>` 比较 NaN 语义一致）。
- 音频槽写不建模 → **登记准确**。

---

## 4. Butterfly（0x141d84d40 / 0x141d867b0）—— 部分判等

### 4.1 `update_butterfly_states` 主循环 + step_state
fresh 反编译与 Rust `update_butterfly_states.rs:99-217` 一致：相机 mode==3 → `sqrt(dist²)<3.0`，否则 `dist²<4.0`；避羊 `dist²<4.0`；`phase=max(0,phase-0.5*dt)`；`near||sheep_near → phase=1.0`；dwell 状态机（≤0 → +=dt 仍≤0 派发 / 否则 uniform*0.3+0.2 派发；vel.y>0 派发 a=3.0；`|0.5-phase_ref|∈[0.2,0.45)` 才 dwell-=dt 分支，否则 a=3.0）——逐语句判等。派发参数 `-(uint)(0<phase)&0x40800000` = phase>0?4:0 与 Rust flag 一致。

### 4.2 states 跳表 0x142f68008
实读 8 项解码：slot0-3=0x141d85983/0x141d85a22/0x141d85ada/0x141d85b08；slot4=0x141d86dd5（`lea r12,[0x142f68018]; movsxd rsi,[r12+rsi*4]; add; jmp` **forwarder**）；slot5-7=0x141d86e29/0x141d86df9/0x141d86e1a（**非对齐字节，非有效 handler**，实践中 state5-7 不触发）。RESTORE"state4-7 转发 vel 表"表述**略宽**——仅 slot4 是有效 forwarder（基址差 0x10 的机制成立：states slot4 与 vel slot0 同字节、不同基址）。

- **state1（0x141d85a22）**：Rust 结构判等（goal_active 门 + `height>0.6`→1 + `rng*10+5` 写槽(U)→0、`!goal_active`→2）。二进制另有 **xmm1==xmm14 门、水检查 0x1409ea800、墙高<0.01 门 0x140a14df0** → Rust 默认"非水无墙"跳过 → **登记准确（U）**。常量 0.6=0x3f19999a、10=0x1429345a8、5=0x142925988 逐位核验。
- **state3（0x141d85b08）**：Rust 结构判等（`height<1.5`→3、`>=1.5`→`rng*5+5` 写槽(U)→2）。常量 1.5=0x1429258d0=0x3fc00000、5=0x40a00000 核验。**次要补充登记**：NaN height 时二进制 `jae` 保持 3、Rust `height<1.5` NaN=false → 2（实际高度为实数，不触发）。
- **state0（0x141d85983）/ state2（0x141d85ada）**：`[+4]` 字段/xmm8/xmm14 阈值 U，Rust 空实现/保持 2 → **登记准确（U）**。

### 4.3 vel 跳表 0x142f68018 → `butterfly_anim_dispatch` —— **判等（4 handler 逐语句）**
实读 0x142f68018 解码（基址 0x142f68018 复算）：slot0-3=0x141d86de5/0x141d86e39/0x141d86e09/0x141d86e2a。
- state0（清零 direction+multiplier）、state1（x/z×0.5 + y=0xbf6147ae）、state2（`1.5>phase_ref → y=max(y,0x3ee147ae)`）、state3（y=0x3f6147ae）——Rust `butterfly.rs:444-470` 逐语句一致（含 NaN：state2 `jbe` 与 Rust `1.5>phase_ref` NaN=false 一致）。
- `set_butterfly_velocities` 主循环 `dir=normalize((dx,dy,dz))*1.76`、`multiplier=phase+1.0`（fresh `fVar16+1.0`）判等。
- R9/RDX 槽写不建模 → **登记准确**。

---

## 5. update_wrens（0x14231e390）—— 部分判等 + **需修 2**

- **跳表实读** 0x14304b638：slot0-4=0x14231ebf8/0x14231e930/0x14231e9a9/0x14231ec82/0x14231edf4（slot5 起为 /rustc 串，表长=5，复算全对）。
- **h0（0x14231ebf8）**：`timer[+0x34]-=dt`；`0>=timer (ucomiss jae)` → H1；否则 0x140a12650 位置查询 → `dist<1.0` → H1（**查询目标身份 U，Rust 不建模，登记准确**）；否则高度更新尾部。Rust `wren_spot_step` state0 一致。
- **h1（0x14231e930）**：`[+0x44]` 保存 → 音频槽写/pos 记录 1.0（U）→ **`mov qword [r12+0x30],2`（qword store 同时清零 +0x34 timer）** → `state→2` → `call 0x140a57300`（U 载荷）→ 高度更新尾部。Rust `wren_h1_mid` 只置 state=2 → 功能等价（timer 后续仅 `<=0` 测试），登记准确。
- **h2/h4（0x14231e9a9/0x14231edf4）**：实体/嵌套扫描续（无状态写），Rust no-op 判等。
- **h3（0x14231ec82）公式判等**（常量逐位核验）：`t=clamp01(progress/0.3)`（0x3e99999a）；`smooth=t²(3-2t)`（3=0x1429cd078）；`phase=0.5+0.5*sin(max(0,progress-0.7)*2π)`（-0.7=0x142f9b080、2π=0x142934674、0.5=0x142925870）；`vx=4(1-smooth)+10*smooth`（4,2=0x142b2e410、10,0.6=0x14304b3a0）；`dvy=2(1-phase)-0.6*phase`（lane1，Rust 符号正确）。
- **高度更新尾部（0x14231e969）**：`clamp01(height + dt*(state==0?-50:5))`（0x14304b3b0=(-50,5)）逐语句判等。
- **需修 1 —— 移动精确式**：二进制 SIMD 合并 `pos.y += (dvy_sub*dir_y*vx + dir_z*vx)*dt`、`pos.z` 不变（xmm0 lane0=vx、lane1=dvy_sub 经 shufps 重排后 mulps xmm4=(dir_x,dir_y*vx)，addss 把 dir_z*vx*dt 并入 y）。Rust `pos.y += dvy*dir_y*dt`、`pos.z += vx*dir_z*dt`。RESTORE 已登记"lane-1 swizzle 近似"，**核验确认差异真实**，精确式：`pos.y += vx*dt*(dvy*dir_y+dir_z)`，pos.z 不动。待 restorer 按此修正。
- **需修 2（未登记）—— anim_val 尾部写**：h3 尾部 `[r12+0x44] = progress*7.0`（`mulss xmm15, XMM12`，XMM12=[0x142a7cd48]=0x40e00000=7.0，fresh 常量核验）。Rust `wren.rs:224` `spot.anim_val = spot.progress` 缺 ×7。**RESTORE 残留表未登记**，须登记并修正为 `spot.anim_val = spot.progress * 7.0`。
- **>6.0 panic 路径**：`progress>6.0`（0x142934690=6.0）→ `call 0x140d14770`（panic，串 "crates/systems/critters/src/wren.rs"）；Rust 复位 progress/state 承接 → **登记准确（U）**。

---

## 6. 残留登记核验（任务点名）

| 残留 | 登记 | 核验 |
|---|---|---|
| wren 0x140a12650 距目标查询 | U | 准确（h0 中段分支不建模） |
| wren h3 `pos.y += dvy*dir.y*dt` | 近似 | 准确（精确式已确认：`pos.y += vx*dt*(dvy*dir_y+dir_z)`） |
| butterfly state0/2 | U | 准确（`[+4]`/xmm14/标志 U，Rust 空实现/保持 2） |
| shared water2(local_100)/water8(local_fc) | 近似 | 准确（`!samples_rock_mask`/`水<=0`） |

**新增登记（RESTORE 遗漏）**：
1. wren h3 anim_val = progress×7.0（Rust 缺 ×7）。
2. move_critters spherecast ①外层门 `(local_fc||local_100)`（Rust FIX① 门不完整）。
3. butterfly state3 NaN 高度次差（二进制保持 3、Rust →2）。
4. critters_avoid_collisions 墙 relate 循环语义（仅模块头登记，RESTORE 未单列）。

---

## 证据

- fresh 反编译：`evidence/results/review-fresh-7funcs.json`（7 函数全量，ghidra shared，job `evidence/jobs/review-fresh-7funcs.json`）。
- 跳表字节核验：复用 `reports/re-analysis/hardpoints-ecstate/evidence/results/ghidra-read-tables.json`（0x14304b0f8/0x14304b178/0x14304b638）+ `ghidra-bfly-tables.json`（0x142f68008/0x142f68018）→ 全部基址差复算通过。
- 常量核验：`ghidra-wrens-consts.json`/`ghidra-wrens-consts2.json`/`ghidra-wave2-consts.json`（0x142a7cd48=7.0、0x14304b3b0=(-50,5)、0x14304b3a0=(10,0.6)、0x142f9b080=-0.7、0x142934674=2π 等逐位）。
