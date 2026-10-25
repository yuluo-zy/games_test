# REVIEW — B-ecstate：8 项 post-fix 独立复验（硬标准逐语句判等）

- reviewer：`reviewer-B-ecstate`
- 日期：2026-10-17
- 范围：ecstate 审计 B 类中「复核提 FIX 后 restorer 已改但无 post-fix 独立复验」的 8 项：
  `move_critters`、`critters_avoid_collisions`、`update_wrens`、`update_butterfly_states`、
  `replay_tree`、`process_tree_cmds`、`maintain_dormer_tiles`、`show_color_options`。
- 方法：**独立 fresh 反编译/反汇编**，不复用 restorer 结论。
  - B patched `tiny-glade.exe`（Ghidra shared 持久库，`program=tiny-glade.exe`）。
  - Ghidra `get_functions`（signature+decompiled_code，8 函数批）→ `evidence/raw-decompile.json`。
  - Ghidra `disassemble_function`（4 critter 函数全量）→ `evidence/raw-disasm-critters.json`、`evidence/*.asm`。
  - radare2 定点 `disassemble`：wren handler / butterfly handler / process_tree_cmds insert /
    show_color_options 分派与 case 0/1 → `evidence/*.r2.txt`、`evidence/radare-*.json`。
  - 常量字节实读：`0x142925984=0.1`、`0x142925870=0.5`、`0x142a16cb8=0.8`、`0x142925904=1.0`、
    `0x1429cd078=3.0`、`0x142934694=4.0`、`0x142a7cd48=7.0`、`0x142f9b080=-0.7`、
    `0x142934674=2π`、`0x14292e6b0=1.0`；butterfly 跳表 0x142f68008 8 项、
    color 跳表 0x14303ca70 39 项实读。
  - 限制：`show_color_options` 0x14229a7f0 的 Ghidra 反编译超时、ReVa 10s 超时；改用
    fresh **反汇编 + 跳表 + 字符串 blob 实读** 复核 case 0/1 载荷（非复用旧 `full.c`）。
  - **未改 src。**

## 0. 三态结论

| # | 函数 | 地址 | wave4/复核命名 FIX 的复验 | 函数级（硬标准） |
|---|---|---|---|---|
| 1 | `maintain_dormer_tiles` | 0x1407ce920 | fix_verified（游标写序、id 单槽 store、GPU 退役） | **判等** |
| 2 | `move_critters` | 0x142318fa0 | fix_verified（water2=Sheep 双采样、spherecast 门、blocked 三件套） | **仍有 FIX**（纯墙 relate 路缺失、半径、panic） |
| 3 | `critters_avoid_collisions` | 0x1423179a0 | fix_verified（Duck 两位置 4+4、hitcode==2 保留、loc<7 保留、relate 方向） | **仍有 FIX**（半径、relate 形状、零方向、panic） |
| 4 | `update_wrens` | 0x14231e390 | fix_verified（h0 模式门/阈值 1.0/3.0/0 与顺序） | **仍有 FIX**（h1 尾部写面） |
| 5 | `update_butterfly_states` | 0x141d84d40 | fix_verified（flag 基线 4/3、4/1） | **仍有 FIX**（**新发现**：state2 次态 3 vs 二进制 1） |
| 6 | `replay_tree` | 0x141bc34f0 | **未实现**（Move undo 旧位事件仍缺失） | **仍有 FIX** |
| 7 | `process_tree_cmds` | 0x142267430 | fix_verified（G4 selector=id&1；Move/Delete/Entities 门） | **仍有 FIX**（Insert T2/T4/T5 载荷） |
| 8 | `show_color_options` | 0x14229a7f0 | 部分 verified（case 0/1 主载荷） | **仍有 FIX**（**新发现**：plaster 副值 v24；case2..0x26/门源 GAP） |

**三态：判等 1 / 仍有 FIX 7。**
其中 4 项（move/avoid/wrens/process）**命名 FIX 本身已判等**，函数级仍 FIX 的原因是既有登记
缺口（缺路径/缺载荷写面）；3 项（butterfly/replay/show_color）存在未闭合项或新分歧。

---

## 1. maintain_dormer_tiles 0x1407ce920 — 判等

fresh Ghidra 反编译（`evidence/raw-decompile.json` 0x1407ce920，77 行）与
`crates/systems/decorator/src/decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs` 逐语句：

| 二进制 | Rust | 判定 |
|---|---|---|
| `prev=*cursor`；`startA=prev-countA(@+0x18)` 钳 0；`startB=prev-countB(@+0x38)` 钳 0 | `saturating_sub(count_a/count_b)` | 判等 |
| `lenA=list.len(@+0x10)-startA`；`list.len<startA → 空哨兵 8,len=0`（同 B） | `if len>=start {len-start} else {0}` | 判等 |
| **循环前** `*cursor=(total@+0x40-lenB')-lenA'`；每瓦 `*cursor+=1` | `*cursor = total-len_b-len_a`，循环内 `+1` | 判等（末值=total） |
| `uVar2=[state+0x1c]`，`puVar6=[state+0x10]`；每瓦 `MOV [R13],EBP`（单槽 store，不递增） | `if !id_slot.is_null() { *id_slot = value }` | 判等（null guard 安全缝 I） |
| 每瓦 `hash_one<RandomState,&GlobalId>(元素+8)` + `RawTable::remove_entry`，返回值非空 → `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>` | `retire.remove(&global_id)` + `drop` | 判等 |
| A[startA..] 与 B[startB..] 的链式遍历（A 空→B，B 尽→return） | `skip().chain(skip())` | 判等（遍历语义等价） |

结论：**函数体判等**。审计注「rhapsody 注册表/RoofTileSsbo drop 未接线」属于宿主/注册面
（`add_decorator_systems` 未注册、退休表无生产者），不改变本函数体判定；`id_slot` null guard
为安全缝 I。

---

## 2. move_critters 0x142318fa0 — 仍有 FIX（命名 FIX 已 verified）

fresh 反编译 + 全量反汇编（`evidence/142318fa0.asm`）逐项：

### 2.1 已核实的 wave4 FIX

1. **water2 组件 = Sheep（+0x138）**：`0x14231a1bd TEST byte [RBP+0x23c],1`（列 +0x138=Sheep）
   且 `CMP [R13],4; JZ skip`；两次 `bilinear_sample_at_world_xz`：`pos` 与 `pos+disp`；
   `now<=0 && pred>0` → blocked。Rust `sheep.is_some() && locomotion.0 != 4` 判等。
2. **spherecast 内层门**：`0x142319da4 CMP loc,6; JA spherecast`；`BT 0x63,loc; JNC spherecast`
   （loc∉{0,1,5,6} 才 spherecast）。Rust `loc>6 || !wall_hash_only()` 判等。
3. **loc==2 岩石路**：`rock>0 || water>0 || (water<=0 && Duck)`（+0x238=Duck）；
   非 loc2：`!(water>0) && Duck`。Rust 判等（`not_water` 覆盖 NaN）。
4. **blocked 三件套**：blocked → `goto LAB_142319506` 跳过位移/朝向/BirdSpots；
   Rust `if !blocked {}` 判等。
5. **命中码**：`local_318 != 2 && dist<1.0` → blocked。Rust `hit && hitcode!=2 && dist<1.0` 判等。
6. 位移=`vel*dt*mult`；水面在预测点采样（`pos+disp`，`0x142319c22-c4f` 实证）。

### 2.2 仍未实现/不等（行为面）

1. **纯墙 relate 路径缺失**：0x142319cc3 后，无 Duck 且无 Sheep 的实体进入
   `WallSpatialHash::query`（0x140aa5730）→ 逐墙 `PublicWalls::get` → `max_y_gable<pos.y` 跳过
   （loc!=3）→ wall `Circle2d::new_rotated`+`LineInterval::line_segment`+`relate`；
   relate 判别 0 → 跳 `LAB_142319506`（阻塞），遍历尽 → `LAB_142319dbf`（位移）。
   Rust 对该类实体直接 `blocked=false` 位移，**不做任何墙判定** → 行为不同。
   （Rust 注释/台账已登记「纯墙 relate 未实现」，但按硬标准仍是 FIX。）
2. **spherecast 半径未传**：`0x14231a139 XMM3=[0x142a16cb8]=0.8`（Sheep），否则
   `0x142925870=0.5`，作为第 4 参数传给 `RaycastWorld::legacy_spherecast`（0x1408ccc90）。
   Rust `legacy_spherecast(&raycast,*pos,dir)` 无半径参数 → 球距判定口径不同。
3. **PublicWalls::get 缺失的 panic**：`0x142319d31 TEST RAX; JZ 0x14231a371`（unwrap_failed）；
   Rust 纯墙路缺失，等价面也无 panic 分支（登记）。

结论：wave4 命名的 water2/门/三件套修正**全部 verified**；函数级因缺纯墙逻辑路与半径而
**仍有 FIX**。

---

## 3. critters_avoid_collisions 0x1423179a0 — 仍有 FIX（命名 FIX 已 verified）

fresh 反编译 + 全量反汇编（`evidence/1423179a0.asm`）。列映射照 wave4：+0x130=Duck、+0x138=Sheep。

### 3.1 已核实的 wave4 FIX（Duck 两位置 4+4）

- 0x142318410 `TEST [RBP+0x15c],1` = Duck 分支。
- 第一组 4 次 `bilinear`（0x142318440/45f/47f/49f）在 `pos`，求和 clamp；`UCOMISS c0,0`
  `JBE` → c0<=0 用 0.3，否则 `d=sqrt(goal.x²+goal.y²+goal.z²)`（0x1423184cb-e8）。
- 预测点 `pos+dir*d`（0x1423184ef-50b），第二组 4 次 bilinear（0x142318526/545/564/583），
  clamp 后 `JA 0x1423187f4`（c1>0 → 清目标）。Rust 两位置 4+4 逐语句判等。
- 其余核实：loc2 在 `pos+dir*0.1`（`0x142925984=0.1`，0x14231839e）同点采 water+rock；
  Duck 分支否则进 `0x1423185c6`；`loc<7 && 99>>loc&1` → 保留（0x1423185cd-dc）；
  `hitcode==2 || dist>=1.0` → 保留（0x14231869b/6e9）；`dist<0.4 && Duck` → DespawnDuckCmd
  （0x142318704-726）；墙 relate `do{}while(relate!=0)`，判别 0 → 清目标，遍历尽 → 保留
  （0x1423187e7-7f4/750）；`max_y_gable<pos.y` 跳过（0x142318794-79c）。
- 收尾 `*goal.active=0`（0x142318803 `*piVar16=0`）。

### 3.2 仍不等

1. **spherecast 半径未传**：0x14231866f `0x8` 若 `R12B`（Sheep 存在）否则 0x0.5，第 4 参数。
   Rust 无半径。
2. **relate 形状近似**：二进制对**墙段** `Circle2d::new_rotated`（0x140c97560）后再
   `LineInterval::line_segment`（0x14261f450），再 `relate`；Rust 用 `line_segment(pos,pos+dir)`
   与墙段 `line_segment` 的 **segment-vs-segment**。端点/半径面不同（登记为近似）。
3. **零方向处理分歧（未登记）**：二进制 normalize 非法时 `dir=(0,0)` 后**继续**（0x142318351-361）；
   Rust `dir.length_squared()<1e-9 → goal=0; continue`（提前清目标、跳过后续判定）。
   `goal.position==pos` 的实体会出现行为差。
4. **PublicWalls::get 缺失 panic**：0x142318773 `TEST RAX; JZ 0x142318945` unwrap_failed；
   Rust `continue` 兜底。
5. 查询环境值写 `*local_138=local_100`（每实体）未建模（登记 DIVERGE）。

结论：wave4 的 4+4 采样修正**verified**；函数级 **仍有 FIX**。

---

## 4. update_wrens 0x14231e390 — 仍有 FIX（h0 FIX 已 verified）

### 4.1 h0 模式门/阈值（wave4 FIX）— verified

fresh radare `0x14231ebf8`（`evidence/wren-h0.r2.txt`）逐指令：

```
timer [+0x34] -= dt
ucomiss 0, timer; jae H1_mid
ucomiss threshold(xmm7), 0; jbe height_update      // threshold<=0 或 NaN → 跳过查询
call 0x140a12650 (CursorTerrainRaycast::position)
dist = sqrt((cursor.x-pos.x)^2+(cursor.z-pos.z)^2)
ucomiss xmm7, dist; ja H1_mid                      // threshold>dist 才进 H1
```

Rust `wren_spot_step state0` 顺序/NaN 语义一致。阈值源 fresh 反汇编 0x14231e7d9：
`tag=mode[0]`；`RCX = (tag-6<15) ? tag-5 : 0`；`RCX∈2..5`（tag 7..10）→1.0；
`RCX==0xa`（tag 15）→1.0；`RCX==1`（tag 6）且 `byte[mode+4]∈{1,2}` →3.0；其余 0。
Rust `wren_proximity_threshold` 与 `AppMode` native_tag/WallShape 判别（Wall=6、
Path=7/Cobblestone=8/Eraser=9/Terrain=10/Water=15；Circle=1/Rectangle=2）**判等**。

### 4.2 h3 — 判等（顺带复验）

0x14231ec82：`t=clamp01(progress/0.3)`；`smooth=t²(3-2t)`；`phase=0.5+0.5*sin(max(0,progress-0.7)*2π)`;
`vx=4(1-s)+10s`、`dvy=2(1-p)-0.6p`；`pos.x+=vx*dir_x*dt / pos.y+=vx*dvy*dir_y*dt /
pos.z+=vx*dir_z*dt`；`progress+=dt`；`>6 → EntityCommands::despawn`；`anim_val=progress*7.0`。
Rust `wren_h3_move` 判等（含 NaN/常量）。

### 4.3 仍不等（h1 尾部写面）

fresh `0x14231e930`（`wren-h1.r2.txt`）：
```
xmm15=[+0x44]
eax=[rsp+0x30]（查询值）; [rcx]=eax          // 音频槽 u32 写
qword [r13+0x1c] = 0x3f8000003f800000        // [+0x1c]=[+0x20]=1.0
dword [r13+0x24] = 0x3f800000                // [+0x24]=1.0
qword [r12+0x30] = 2                         // state=2 且 +0x34(timer) **清零**
call 0x140a57300                              // 音频载荷
→ 0x14231e969 高度更新
```
Rust `wren_h1_mid` 只落 `state=2` + 高度更新；**qword store 顺带清 timer、1.0×3 写面、音频槽/
调用**未建模（审计注 h1 音频槽/实体映射/PlacementDataSsbo 已登记）。按硬标准仍为 FIX。

结论：h0 命名 FIX **verified**；函数级 **仍有 FIX**（h1 尾部）。

---

## 5. update_butterfly_states 0x141d84d40 — 仍有 FIX（含新发现）

### 5.1 dispatch flag 基线（wave4 FIX）— verified

fresh 反汇编 `0x141d857b4-0x141d85a20`（`evidence/141d84d40.asm`）：

- 常量：XMM6=1.0、XMM12=3.0、XMM15=4.0。
- 各派发点统一 `flag = phase>0 ? 4.0 : baseline`：
  - dwell<=0 两路（0x142318c2/859ee）：baseline **3.0**（`ANDNPS XMM8,XMM12`）✓
  - dwell>0 且 vel.y>0 / 相位门外（0x14231866→85955）：baseline **3.0** ✓
  - `|0.5-phase_ref|∈[0.2,0.45)` 两路（0x1423181e / 0x14231877→85955）：baseline **1.0**
    （`MOVAPS XMM0,XMM6` + `ANDNPS XMM8,XMM0`）✓
- Rust `flag_hi=4/3`、`flag_lo=4/1` 与该 6 条路径逐一对应判等。
- handler 实证：各 handler 的 `t` 读 `[+4]`（state_t），派发 XMM0 为死值（与 wave4 结论一致）；
  state0/1/3 与 Rust 逐项一致（0.5/0.1/0.6/rng*10+5/rng*5+5/NaN 语义）。

### 5.2 新发现 FIX — state2 次态（0x141d85ada）

fresh 字节 `0f28d0 410fc2d602 410fc2ce00 0f54ca 66410f7ecf 4183c702`：
```
xmm2 = (state_t - dt) <= 0 ? -1 : 0
xmm1 = (phase == 0) ? -1 : 0
andps; movd r15d, xmm1; add r15d, 2
```
即 **条件成立 → 0xFFFFFFFF+2 = 1；否则 2**。Rust：
`if t<=0.0 && st.phase==0.0 { 3 } else { 2 }` — **3 应为 1**。
（wave4 复核写作 `2+(t<=0 && phase==0)`，未处理 SSE 掩码为 -1，属未闭合项。）
state2 其余（`!active → t=max(t,5)`、t-=dt）判等。

### 5.3 仍不等

- state2 次态 3 vs 1（上述，**新发现**）。
- state4-7：跳表项 4=forwarder（0x141d86dd5），5-7 非对齐字节；Rust `_ => 保持` 未建模
  forwarder（登记）。
- ButterflyAudio 每帧 u32 写（query 派生值）未建模（登记）。

结论：flag 基线 FIX **verified**；函数级 **仍有 FIX**（state2 次态 + state4-7/audio）。

---

## 6. replay_tree 0x141bc34f0 — 仍有 FIX

fresh 反编译（`evidence/raw-decompile.json` 0x141bc34f0，412 行）逐语句：

### 6.1 判等面

- `front_mut_bucket()` None → return；事件 `stride 0x30`，`tag==8` 才处理。
- `elapsed@+0x18 == 0.0 && !NAN` 才重发。
- kind=2=Delete：non-undo → DeleteTreeCmd(0x10, id=edit+0x18)；undo → Insert(0x48)。
- kind=1=Move：non-undo → MoveTreeCmd(0x20, id@+8=edit id、position@+0x10=edit+0x20)；
  undo → **读 edit+0x14/+0x1c 旧位**。
- kind=0=Place：non-undo → Insert(0x48)；undo → Delete。
- 时序：`flags@+8 bit0==0 → 立即入移除集`；bit0==1 → `delay=cmd+0xc/speed` 钳 0.05，
  `delay<elapsed` 入集，`elapsed += GameTime.delta`。
- 移除集**逆序** `Vec::remove`（swap-halves + memmove），tag switch 释放载荷。
Rust 时序/Move non-undo/Delete/Place/逆序删除均判等。

### 6.2 仍不等

1. **Move undo 旧位事件缺失（G2/T3，点名 FIX 未实现）**：二进制 `edit+0x14/+0x1c`
   （undo 分支 0x141bc3xx）重建 MoveTreeCmd；Rust 该分支为空（仅注释），且
   `TreeEditData` 仍无 previous position → **整条事件不发**。
2. **Insert 0x48 载荷 T2**：二进制写 name/position/scale/rotation/variant/flags/id/height
   8 字段；Rust 仅 name/position/scale/id。
3. 查询环境值写 `*in_RCX[2]=[RCX+0x1c]` 未建模（登记级）。

结论：**仍有 FIX**（点名 FIX=Move undo 未落地；T2 未落）。

---

## 7. process_tree_cmds 0x142267430 — 仍有 FIX（G4 已 verified）

fresh 反编译 + radare 定点（`evidence/ptc-insert.r2.txt`）。

### 7.1 判等面

- **Move**（0x20）：`storage.get(index)`；None → log "MoveTreeCmd references a non-existing tree"
  （0x142267b.. 日志面）；Some → 写变换位置、`inverse_lerp(0.5,0.9,scale)`、预测点水采样、
  `PlayTreeSoundCmd{kind=1,selector=water>0,value=inv_lerp,position}`（槽 0x20：+8/+9/+0xc/+0x10/
  +0x18）。Rust 判等。
- **Delete**（0x10）：`storage.remove(id)`；Some 且 **`Entities::contains(entity)`**（0x14226834 区）
  才 `insert TreeMarkedForDeletion` + 变换查询 + `kind=2` 音效；否则跳过。Rust 判等（补门属实）。
- **Insert**：名称门（5 个 naked 名，长度 0x19/0x1a/0x27 实证）、`deny_load`+`is_corrupted`
  门、`max(scale,0.7)`（0x142267bf3/0x1429cd030）、α=`inv_lerp*0.75+(1-r)*0.55`、
  spawn+四组件（TreeComponent/TreeAnimation/TreeMoveBend{x,z}/TreeAmbience{x,y+scale*10,z}）、
  `storage.insert` 冲突日志 + DespawnPrefabCmd、水采样 + `kind=0` 音效（+8/+9/+0xa/+0xc/pos）。
- **G4 selector**：`0x142268146 MOV RCX,[RDI]`（RDI=event+0x38=id）→ `call 0x14226a170`
  （random_tree_prefab，内部 `in_CL & 1`）。Rust `(cmd.id & 1)` **判等**。

### 7.2 仍不等

1. **T2**：Insert 事件真实 0x48 载荷 rotation/variant/flags/height 未承载（共享
   `InsertTreeCmd` 只有 name/position/scale/id）。
2. **T4**：`is_corrupted`（`BackupSaveFileKind::is_corrupted`）以 `false` 固定；
   `drag_started`（0x14229? 区 `ActiveWidget::is_dragging`，decomp 行 339）以 `false` 固定 →
   `random_tree_prefab` 第 5 参不同。
3. **T5**：`SpawnPrefabCmd` 0x50 载荷（prefab/entity/pos/scale/rotation/variant/flags/mode）
   未落库，Rust 只发 `default()` → Insert 的下游实体生成不可观察。

结论：G4 命名 FIX **verified**；函数级 **仍有 FIX**（T2/T4/T5）。

---

## 8. show_color_options 0x14229a7f0 — 仍有 FIX（case 0/1 主载荷基本 verified + 新发现）

方法：Ghidra 反编译超时、ReVa 10s 超时 → fresh radare 反汇编 + 跳表/字符串实读。

### 8.1 结构/跳表（复核）

- `0x14303ca70` 39 项实读：kind0→0x14229b725、kind1→0x14229bde9、kind2→0x14229af43、
  kind3→0x14229b5fd、kind5/8/0xc→0x14229aa6e…（与 Rust 注释一致）。
- 事件 stride 0x20；kind 字节取事件首字节；demo 门 `[0x14362c098]→[0x143634d90]`，
  当前 0 → 非 demo（Rust `demo_mode=false` 与当前构建一致）。
- case 0 非 demo 入口 0x14229cc37：先 `PublicWalls::get`（0x140a91ef0），空则 return。

### 8.2 case 0 主载荷（verified 部分）

非 demo 路径 fresh 反汇编（`evidence/color-case0-nondemo.r2.txt`）13 个 kind-3 swatch，
`mov dword [rbx+k*0x28+0x1c]` 值：0,1,3,0xb,2,4,0xc,5,0xa,8,9,6,7（0xb/0xc 以字节实读
`c7 83 94.. 0b` / `c7 83 0c01.. 0c` 复核）；名称 blob 实证 bricks_beige/yellow/light/grey/dark
+ plaster_colors×8。Rust `push_case0_main_swatches` 的 name/kind/v1c **判等**。

- plaster 门：`get_instruction` 取值后 `lea ecx,[rax-4]; cmp ecx,7; jae` → `(v-4)<7`，另一支
  `==0xc`。Rust 以「任一 id∈4..=10 或 0xc」近似（门源 GAP，登记）。
- plaster_peel 块（0x14229d2ec 起）：separator(kind5) + low/medium/high（kind 0xc，
  `+0x1c=0 / 0x3f000000 / 0x3f800000`，`+0x24=0`）→ **判等**。
- halftimber/skirt 块（0x14229d55e 起）：separator + halftimber(kind4,b19=2) +
  skirt(4,1) + no_skirt(4,0)（`word [..+0x40]=0x204/0x104/0x004` 即 kind/b19），`+0x24=0`
  → **判等**。门 `*(puStack_250+600)<2` 源未落库（Rust 固定 0，登记）。
- is_flat 门 separator + `new_flat_roof_rim_ty` 结构经前次复核判等（本次未重复展开）。

### 8.3 case 1（verified）

非 demo 入口 0x14229d754，分配 `0x1b8=11*0x28`；名称 blob 顺序
ochre/red/purple/blue/light_blue/turquoise/green_light/green/dark_wood/dark/grey；
首项 `+0x18=3, +0x1c=8`，第二项 `+0x44=0`；Rust 11 项表（v1c 8,0,10,2,9,3,6,4,1,5,7）**判等**
（v24 均为 0）。

### 8.4 仍不等

1. **新发现：plaster_colors 副值 v24（+0x24）未建模**。13 项中后 7 项二进制显式写
   `+0x24 = 7,1,6,4,5,2,3`（前 6 项写 0）：`0x14229d03e` 等 `mov dword [rbx+0xec],0`…
   `0x14229d07d …0x114=7`、`0x13c=1`、`0x164=6`、`0x18c=4`、`0x1b4=5`、`0x1dc=2`、`0x204=3`。
   Rust `icon_color` 只落 `v1c`，`v24` 恒 0 → 7 个 plaster swatch 的副值丢失（硬标准 FIX）。
2. **case 2..0x26**：逐 case 精确表未实现（Rust `_ =>` 回落到 roof 色），登记 GAP。
3. **demo 分支**：二进制 demo 分支真实存在（0x14229b744 起的 demo 路径已见），当前全局 0，
   Rust 不实现（登记/GAP）。
4. **门源 GAP**：plaster 门（`get_instruction` 索引）、halftimber 计数（+600）、
   `MainCamera::eye_position` SIMD 偏移/radius 均为近似或常量（登记）。
5. 非平顶 else 块（5 图标）与事件写者族未实现（登记）。

结论：case 0/1 **主载荷**已按 wave2 REVIEW 修正且 fresh 复核基本成立；但因 **v24 漏写**（新发现）
与 case2..0x26/门源/demo GAP，函数级 **仍有 FIX**。

---

## 9. 与审计 B 清单的对照

| 审计 B 注 | 本次结论 |
|---|---|
| move_critters：water2 Sheep 已改 | verified；纯墙路/半径/panic 仍缺 → FIX |
| critters_avoid：water8 4+4 已改 | verified；半径/relate 形状/零方向/panic → FIX |
| update_wrens：h0 模式门/阈值已改 | verified；h1 尾部写面 → FIX |
| update_butterfly：dispatch flag 基线已改 | verified；**state2 次态 3→1**（新） + state4-7 → FIX |
| replay_tree：Move undo 旧位缺失 | 未实现，维持 FIX |
| process_tree_cmds：Insert T2/T4/T5 | G4 verified；T2/T4/T5 维持 FIX |
| maintain_dormer_tiles：G3 已修 | 判等 |
| show_color_options：case0 载荷已修 | 主表 verified；**v24 副值漏写**（新）+ GAP → FIX |

## 10. 证据索引

- `reports/reviews/B-ecstate/raw-decompile.json`（8 函数 Ghidra fresh；show_color_options decompile_failed=timeout）
- `reports/reviews/B-ecstate/raw-disasm-critters.json` + `evidence/142318fa0.asm`、`1423179a0.asm`、
  `14231e390.asm`、`141d84d40.asm`
- `evidence/wren-h0.r2.txt`、`wren-h3.r2.txt`、`butterfly-h{0..3}.r2.txt`
- `evidence/color-dispatcher.r2.txt`、`color-case0-nondemo.r2.txt`、`color-case0-plaster.r2.txt`、
  `color-case1-nondemo.r2.txt`
- `evidence/ptc-insert.r2.txt`、`radare-ptc2.json`
- `evidence/consts.json`、`read_jt.json`、`read_color_jt.json`、`read_color_bytes.json`、`names.json`
