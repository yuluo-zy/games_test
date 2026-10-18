# REVIEW — wave4 remaining-misc（forest / tree-tool / critters / glade-border）

- reviewer：`reviewer-wave4-misc`
- 日期：2026-10-17
- 被审：`reports/remaining/misc/RESTORE.md`（restorer-remaining-misc）
- 方法：**独立 fresh 反汇编/反编译**，不复用 restorer 的结论。
  - B patched EXE（SHA-256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`）
    → radare2 定点 `pd`（0x14231ec82/ebf8、0x140d14770、0x140a12650、0x142319b40-ca4、
    0x14231a0a0-242、0x1423183a0-820、0x141d857b4-85a20、0x141d85983/b08、0x141d852c0-f30、
    0x141bf1ba0-1ee0、0x140c9ae20/a9d0、0x141be0bc0-e97、0x141be10a0-1299）。
  - ghidra shared 只读 fresh 反编译：`0x140d14770`、`0x14011350`、`0x140d12d80`、
    `0x140a12650`、`0x14231e390`。
  - 常量字节实读：`0x14304b390`、`0x142925904`、`0x1429258d8`、`0x142f9b080`、
    `0x142934674`、`0x142925870`、`0x14292e6b0`、`0x142b2e410`、`0x14304b3a0`、
    `0x142a7cd48`、`0x142934690`、`0x1429cd078`、`0x14304b3b0`、`0x142f68008`（8 项）、
    `0x142efa130/140`。
  - **未改 src、未改 core。**

## 裁决总表

| 目标 | RESTORE 声称 | 本次复核 | 结论 |
|---|---|---|---|
| forest `manage_tree_prefabs` 构造形状判定 (0x141bf16c0) | 重建/判等 | 形状源末条 + 门控 + 点/矩形阈值/顺序逐语句吻合 | **判等** |
| tree-tool `convert_preplaced_trees` (0x14226be70) | 重建（History 路径落库） | Wait 门、6 路采样/冲突方向、Create/Delete 分支、`insert_starting_edit` 调用吻合 | **判等** |
| tree-tool `hide_trees` (0x14226a260) | 未实现（core-type-gap） | core 命令仍只有 `{center,radius}`；shape/gate 字节缺 | **未实现（登记属实）** |
| critters butterfly state0-3 (0x141d84d40) | 重建/判等 | 4 handler 实体逐语句判等；**dispatch flag 基线错** | **需修** |
| critters wren (0x14231e390) | 重建/判等 | h3 三轴/×7/despawn 真（旧结论已正确推翻）；**h0 模式门缺失** | **需修** |
| critters move water2 (0x142318fa0) | 重建/判等 | loc2/非 loc2 的 Duck 条件对；**双水采样组件取错（应 Sheep）** | **需修** |
| critters avoid water8 (0x1423179a0) | 重建/判等 | 分支/墙 relate 方向/Despawn 对；**Duck 栅格为两位置 4+4 采样，非单点 4 次** | **需修** |
| glade-border perimeter 写路径 (0x141be0ad0) | 重建/判等 | 48B 布局/字段序/工厂/write 5×qword 逐句吻合 | **判等** |
| glade-border `DebugLine::show` (0x140b04f70) | 未实现 | core `debug_viz::DebugLine` 仍无 `show` | **未实现（登记属实）** |

**三态：判等 3 / 需修 4 / 未实现 2。** RESTORE 自报“判等/重建 8、未实现 2”**高估 4 项**
（butterfly、wren、move、avoid 均有逐语句分歧，见 FIX 清单）。

---

## 1. critters — wren（0x14231e390）

### 1.1 h3 三条被推翻的旧结论：**全部核实成立**

fresh `pd` 0x14231ec82–0x14231ed62：

- `t = clamp01(progress/0.3)`；`smooth=t²(3-2t)`；
  `phase=0.5+0.5*sin(max(progress-0.7,0)*2π)`（0x3e99999a/0x1429cd078=3/0x142f9b080=-0.7/
  0x142934674=2π/0x142925870=0.5 逐位读回）。
- `xmm15=[smooth,phase]`；`xmm1=[vx,dvy]`，`vx=4(1-s)+10s`、`dvy=2(1-p)-0.6p`
  （0x142b2e410=(4,2)、0x14304b3a0=(10,0.6)）；两次 `shufps`（0x01,0xe2）后
  `mulps xmm0,xmm4`（xmm4={dir_x@+0x34, vx*dir_y}）。
- 落库结果：`pos.x += vx*dir_x*dt`；`pos.y += vx*dvy*dir_y*dt`；`pos.z += vx*dir_z*dt`
  （xmm3=dir_z*vx 再 `*dt` 加到 `[r13+0x18]`）。**三轴式正确**，wave2 的“合并式
  `y += vx*dt*(dvy*dir_y+dir_z)`、z 不动”确为误读，RESTORE 的推翻成立。
- `progress += dt`；`mulss xmm15,*[0x142a7cd48]=7.0` 在 `ucomiss progress,6.0` **之前**完成，
  `jbe` 直接到尾写 `[r12+0x44]=xmm15` → **anim_val = progress×7.0** 正确。
- `progress>6.0` 分支：ghidra fresh 证实 `0x140d14770` =
  `bevy_ecs::system::commands::EntityCommands::despawn`（函数体把
  `despawn::closure_env$0` 的 FnOnce + `bevy_ecs::error::handler::warn` + entity 压入
  `RawCommandQueue`，无 panic 格式化调用；`0x14304b620` 是 `track_caller` 的
  Location（file=`crates/systems/critters/src/wren.rs`、len 0x23），不是 panic 串）。
  **despawn 结论成立**（旧“panic”被正确推翻）。

### 1.2 【FIX】h0 距离查询缺模式门/阈值

fresh `pd` 0x14231ebf8–0x14231ec7d：

```
timer -= dt;  0 >= timer (ordered/unordered JAE) → h1_mid
ucomiss xmm7, 0;  jbe → 高度更新（跳过查询）        // 门：xmm7 > 0 才查询
call 0x140a12650 (CursorTerrainRaycast::position, ghidra fresh 证实)
dist = sqrt((cursor.x-pos.x)^2 + (cursor.z-pos.z)^2)
ucomiss xmm7, dist;  ja → h1_mid                   // xmm7 > dist 才进 h1
```

`xmm7` 在 0x14231e7d9–0x14231e80b 前导里由某个资源首字段（EDX∈6..20 的“模式”）算出：

- EDX∈{7,8,9,10} 或 EDX==15 → `xmm7=1.0`；
- EDX==6 且 `byte[res+4]-1 < 2` → `xmm7=3.0`；
- 其余 → `xmm7=0.0`（查询整段跳过）。

即：**阈值/门依赖模式（1.0 或 3.0，0=禁用）**，资源身份 U（AppMode/相机族候选）。
Rust `wren_spot_step` 状态 0 无条件用固定 `<1.0` 比较 → **缺门且阈值硬编码**。
RESTORE §5 “距离查询（U→关闭）”**高估**：`0x140a12650` 的符号身份确已关闭，但调用门/阈值未落库。

### 1.3 已核实正确的其余语句

- h1（0x14231e930→0x14231e964）：写音频 u32 槽、`[+0x1c/+0x20/+0x24]=1.0`、`[+0x30]=2`
  （qword store 清 timer）、`call 0x140a57300`；Rust 只落 `state=2` + 高度更新（残余已登记）。
- 尾 0x14231e969：`factor = (state==0) ? -50 : 5`（表 0x14304b3b0=(-50,5)）；
  `height=clamp01(height+dt*factor)`；`[+0x44]=xmm15` 原样回写。Rust 等价。
- 0x140a12650 ghidra fresh：`if Option.hit {pos=(+4,+0xc)} else {0}`，与 RESTORE 描述一致。

---

## 2. critters — shared（0x142318fa0 / 0x1423179a0）

### 2.1 【FIX】move_critters 双水采样取错组件（Duck→Sheep）

列映射（decomp `init<tuple<...>>` 顺序 + 7 个 offset 递增，实证）：
`+0x110 Transform, +0x118 Locomotion, +0x120 Velocity, +0x128 Option<Multiplier>,
+0x130 Option<Duck>, +0x138 Option<Sheep>, +0x140 Option<BirdSpots>`。

fresh 0x14231a0a0–0x14231a240：

- spherecast 半径：`[rbp+0x23c]`（=+0x138=**Sheep**）→0.8（0x142a16cb8），否则 0.5。
- 双水采样门：`test [rbp+0x23c]` + `loc!=4` → 与 **Sheep** 对齐
  （`andnps` 形式在 decomp 中为 `(local_fc & 1) && *puVar29 != 4`，local_fc=+0x138）。
- 采样 `water(pos)` / `water(pos+disp)`；`now<=0 && pred>0` → 阻塞（数学式与 Rust 相同）。

Rust `shared.rs:167` 用 `duck.is_some()` → **应改为 `sheep.is_some()`**。
RESTORE §6 的自相矛盾（映射行说 “+0x138=Sheep（双采样/半径 0.8）”，正文却写
“water2：Duck”）以二进制为准：**双采样=Sheep**。
以下条件经复核正确：loc2 `rock || water>0 || (water<=0 && Duck)`；非 loc2
`water<=0 && Duck`；spherecast 门 `(Sheep||Duck)` + `loc>6 || !bitset(loc)`；
命中码 2/距离 1.0 语义；纯墙 relate 路（未实现，登记属实）。

### 2.2 【FIX】critters_avoid_collisions Duck 栅格是两位置 8 采样

fresh 0x142318410–0x1423185c0（Duck 分支，`[rbp+0x15c]`=local_fc=+0x130=Duck）：

```
s0 = Σ_{4 路 grid} sample(pos.x,pos.z);  c0 = clamp(s0,0,1)
d  = (c0>0) ? sqrt(gx²+gy²+gz²) : 0.3      // g = goal.position(+4..+0xc)；否=0.3(xmm15)
p  = pos + dir*d                            // dir = normalize((goal-pos).xz)，零向量保护
s1 = Σ_{4 路 grid} sample(p);  c1 = clamp(s1,0,1)
if c1 > 0 → 清目标（抵达）
else → spherecast 路
```

即 **4+4=8 次 bilinear，两个位置**；“8 偏移采样系误读”这句本身不准确。Rust 只在 `pos`
采样 4 次并用 `s0>0` 决定清目标 → 判定位置与条件均不同（FIX）。
其余复核通过：Duck/Sheep/无 三分支；`loc==2` 在 `pos+dir*0.1` 同点采 water+rock；
墙 relate `do{...}while(relate!=0)`，**Intersection(disc 0)→清目标；遍历完无交→保留目标**
（Rust 方向正确）；`<0.4 && Duck → DespawnDuckCmd`；spherecast 保留/清目标方向；
`loc<7 && bitset → 保留` 门。

---

## 3. critters — butterfly（0x141d84d40）

### 3.1 handler 实体：**判等**

fresh 0x141d85983/a22/ada/b08 + 跳表 0x142f68008 字节实读：

- 表项 0-3 = 0x141d85983/0x141d85a22/0x141d85ada/0x141d85b08（基址差复算全对）。
- state0：`phase_ref<=0.5` 才 `flag=min((0.5-pr)*0.1/dt, flag)`；`t-=dt`；
  `t<=0||NaN||near → 3`，否则 0 —— Rust 一致（含 minss/NaN 语义）。
- state1：`phase==0 && goal.active && rock==0 && water<0.01` 门；`height>0.6 → (1,0.6)`，
  否则 `(0, rng*10+5)`，其余 `(2,t)` —— Rust 一致（0.6=0x3f19999a、0.01=0x3c23d70a）。
- state2：`t-=dt`；active → `2+(t<=0 && phase==0)`；否则 `t=max(t,5)` —— Rust 一致。
- state3：`t=height`；`height>=1.5 → (2, rng*5+5)`，NaN 保持 3 —— Rust 一致。
- 续段 0x141d852df：`[+0]=state`、`[+4]=xmm0`、`phase_ref=fmod(phase_ref+flag*dt,1)`
  （0x142923f10=fmodf）—— Rust 结构一致。

### 3.2 【FIX】dispatch 的 flag 初值基线错误

fresh 0x141d857b4–0x141d85a20。每路的 `xmm8` 进入 handler 并在续段参与
`phase_ref += flag*dt`；其计算为 `phase>0 ? 4.0 : baseline`，baseline 逐路不同：

| 派发路径 | flag_in (phase>0 / phase<=0) |
|---|---|
| `dwell<=0` 且 `dwell+dt<=0`（0x141d859ee） | 4.0 / **3.0** |
| `dwell<=0` 且 `dwell+dt>0`（0x141d85923） | 4.0 / **3.0** |
| `dwell>0 && vel.y>0`（0x141d8586a→85955） | 4.0 / **3.0** |
| `dwell>0 && (off<0.2 || off>=0.45)`（同 85955） | 4.0 / **3.0** |
| `off∈[0.2,0.45) && dwell-dt>0`（0x141d8581e） | 4.0 / **1.0** |
| `off∈[0.2,0.45) && dwell-dt<=0`（复位负 dwell，85955） | 4.0 / **1.0** |

Rust `flag = if phase>0 {4.0} else {0.0}` → phase<=0 时基线 0，**错**。
影响：state0 的 `min(candidate, flag)` 与续段 `phase_ref` 速率（0/1/3 vs 0）。
0.0/1.0/3.0/4.0 常量均为实读（xmm6=[0x142925904]=1.0、xmm12=[0x1429cd078]=3.0、
xmm15=[0x142934694]=4.0），`andnps` 语义 `DEST=~DEST & SRC` 已按字节编码复核。
（派发瞬间的 xmm0 对 state0-3 是死值——handler 各自重写；RESTORE 对“t 初值”的逐路重建
无行为意义，但 flag 基线必须修。）

### 3.3 登记（非本批 FIX）

- 近/避羊 NaN：二进制 `seta`（unordered→0）→ near=false；Rust `!(d>=th)` → true。
  RESTORE 注释“SETB/SETA 均为 0 → 近 = true”自相矛盾；距离为实数时不触发。
- state4-7 跳表项为 vel 表共享字节（非对齐 handler），RESTORE 表述成立（slot4 forwarder）。

---

## 4. forest — manage_tree_prefabs 构造形状判定（0x141bf16c0）

fresh 0x141bf1ba0–0x141bf1ee0：

- 边界 `|x|>65 || |z|>65 → keep=1`（0x141bf1c05 写 a8=1/sil=1）✔。
- 6 路采样：前 4 路 `ADDSS xmm11,xmm12..xmm14` 求和 → clamp[0,1]；后 2 路
  `ucomiss sample,0; ja → keep=0`（0x141bf1cf3/1cfc）。✔
- rock 0x1409ea800 单次布尔，`or sil,al`。✔
- 门控：`al = bit108(双缓冲空) | (density>0) | rock`；`cmp al,1; je 1dfc`。
  - `al==0`（有形状 && 无 density/rock）→ 0x141bf1d6a 形状判定；
  - `al!=0` 且 `!(density|rock)`（仅空）→ 跳过形状判定、直接墙圆查询 0x141bf1e05；
  - density|rock → skip（keep=0）。
  与 Rust `has_construct_shape` 分支 + `!blocked` 墙查询顺序**等价**；形状过近时
  （sil=0）`je 1c0f` 跳过墙查询 ✔。
- 点形状（bit0==0）：center=[+4/+8]，threshold=radius[+0xc]+7.0，
  `ucomiss threshold,dist; seta; xor 1` → `too_close = dist < threshold`；NaN→不过近 ✔。
- 矩形（bit0==1）：call 0x140c9ae20（closest_point_on_perimeter）→ `dist<7.0` ✔。
  该函数内部先 call **0x140c9a9d0**（4 边最近点/参数循环）；core 的
  `rectangle_closest_u_from_pos` 文档标注地址 **0x140c99530 有误**（该地址实为
  `perimeter` = 2*(size.x+size.y)，0x140c99800 为 rect `get_pos_at_u`）。分解式
  `closest_u + get_pos_at_u` 与二进制拓扑一致，数值等价性未逐指令对线 → **登记**。
- 形状源：双缓冲取末条（1e57–1ee5 区）→ Rust `EventReader::read().last()`（游标差异已登记 I）✔。

**结论：该子判定判等。** 0x50B 记录布局、`SpawnPrefabCmd` 0x50、实体/init 字段仍为
core-type-gap（与 RESTORE 登记一致，未虚报）。

---

## 5. tree-tool — convert_preplaced_trees（0x14226be70）

fresh decomp 复核：

- Wait 门：事件缓冲非空 → `flag=1,count=1`；否则 `flag` 置位 → `count'=count+1; 2<count'`
  → 清 flag 进入转换块。Rust `wait.count>2` 同义；双缓冲 vs EventReader 差异已登记。
- 候选：`PlacementDataSsbo::position()`（**非 Transform**；Rust 用 Transform 桥接，登记 ①属实）、
  `as_float_grid` 4 路 + 水 + 花园 + `sample_at_world_xz` 岩石；
  `(water>0 || garden>0) || rock → blocked`；否则 clamp 密度，`density<=0` 才
  `Circle2d::new(…,7.0)+query_with_circle_callback`；最终 `blocked → DeleteTreeCmd`，
  通过 → 0x48 TreeEdit（tag=0 Place）→ `from_iter<EditType, Map<Drain<TreeEdit>>>` →
  `History::insert_starting_edit`。
- Rust：冲突方向、墙查询嵌套、Delete/Place 分支、edits 收集 + `HistoryEntry::from_edits` +
  `insert_starting_edit(history.ledger_mut(), entry, fastrand::u64)` 与控制流一致。
  `ledger_mut()` 已存在于 `history_transactions.rs:135`，分歧 15 确已解锁。
- 残余属实：position 源、scale/rotation（记录 +0xc/+0x10）、候选位图
  （`Decorators::iter_removed_ids` vs Query）、rock buffer、事件载荷 stride 0x10/0x48 细节。

**结论：判等（含上述登记残余）。**

---

## 6. glade-border — perimeter 写路径（0x141be0ad0）

fresh 0x141be0d45–0x141be0eb2：

- `cmp byte[*config+0x121],0; je 写` ✔（core 字段 `border_signifier_enabled`）。
- `0x1411ff640`=perimeter → xmm6；格式化 `format!("glade_border{}", [GameTime+0x2c])`；
  字面量 0x142efa130 实读 `"glade_border"` len 12（0x142efa140 为 (ptr,len) 描述符）✔。
- alloc 0x30 → `[+0]=1, [+8]=1, [+0x10]=cap, [+0x18]=ptr, [+0x20]=len, [+0x28]=perimeter(f32)`
  （0x141be0e3a–e55；cap/ptr/len 由 0x141be0be7（0x90×12 alloc）/0xc18（0x88 memcpy src）佐证）✔。
- `SignifierFactory::new(label_ptr,len,record,0x142efa150)`（0x14089d8b0）+ 40B factory 拷贝后
  `write` 0x140b2cf90；所有权忘记（Rust `mem::forget`）与 restorer 说明一致。
- `closure_1`（0x141be1080）：env {ptr@8,len@0x10,f32@0x18}；alloc 12*len + memcpy；
  builder 载荷 `{0x8000000000000002, len, ptr, len}`（0x141be1163–0x116b）；
  `set_ui_pass(0)`/`animation_duration(0)`/`custom_shader("dashed_line",len 11)`/`build`；
  2× insert：`0x4000000000000001`（Alt+f32 2.0）与 `xmm6=env.length*0.7`（0x1429cd030）✔。
- `DebugLine::show`：core `utils/debug_viz.rs` 的 `DebugLine` 无 `show`（全文 grep 确认）→
  **未实现属实**，不伪造。

**结论：perimeter 写路径判等；`DebugLine::show` 未实现（登记属实）。**

---

## 7. FIX 清单（给 restorer，按优先级）

1. **`systems/critters/src/shared.rs:167`（move_critters water2）**
   `duck.is_some()` → `sheep.is_some()`（二进制 local_fc=+0x138=Sheep，loc!=4，
   now<=0 && pred>0 阻塞；Rust 其余式不变）。同步修正 RESTORE §6 “water2=Duck”的表述
   及映射表自相矛盾处。
2. **`critters_avoid_collisions` Duck 栅格两位置采样**：按 §2.2 的
   `pos`→`c0`→`d=(c0>0?|goal.position|:0.3)`→`pos+dir*d`→`c1>0` 决定清目标；
   当前单点 4 采样不足。
3. **`update_butterfly_states`/`step_state` dispatch flag**：`flag_in = phase>0 ? 4.0 :
   baseline`，baseline 按 §3.2 六路表取 3.0 或 1.0；不可用 0.0。
4. **`update_wrens` h0 距离查询门/阈值**：模式资源（EDX∈6..20，byte[res+4] 子模式）
   → 阈值 1.0/3.0/0；`threshold>0 && dist<threshold` 才进 h1。资源身份仍 U（AppMode/相机候选），
   落库时显式命名 + 登记。

## 8. 登记/文档修正（非 FIX）

- RESTORE §5 “wren 距离查询 U→关闭” → 应为“调用身份关闭；模式门/阈值为 U”。
- RESTORE §6 “water8 … 原 8 偏移采样系误读” → 实为 8 次采样（4 路×2 位置），需改写。
- RESTORE butterfly 注释 “`-(uint)(0<phase)&0x40800000` = 0/4” 与机器码不符（实为 4/3、4/1 选择）。
- 近/避羊 NaN：二进制 seta→false，Rust `!(>=)`→true（仅 NaN 次差）。
- forest 矩形分解：core `rectangle_closest_u_from_pos` 注释地址 0x140c99530 错；
  实际 closest-u=0x140c9a9d0、get_pos_at_u=0x140c99800、perimeter=0x140c99530。
  分解拓扑一致，数值等价未逐指令验证。
- `hide_trees` core-type-gap 与 `DebugLine::show` 未实现均属实；`tree_animation`
  （不在本批点名范围）仅确认 `PrefabInstanceData` 现布局（gpu_instances@+8/rt_meshes@+0x20）
  无可承载 `+0xc/+0x20` 写面，登记与 RESTORE 一致，未逐语句复验。

## 9. 三态回执

- **判等 3**：forest 构造形状判定；tree convert（含 History 落库）；glade-border perimeter。
- **需修 4**：wren（h0 门）；move water2（组件）；avoid water8（两位置）；butterfly（flag 基线）。
- **未实现 2**：hide_trees（core-type-gap）；DebugLine::show（core-api-missing）。
- **高估检查**：RESTORE 自报 8 判等/重建；本批点名 7 项中实为 3 判等 + 4 需修
  （第 8 项 tree_animation 不在本批点名范围，未复验）。具体高估：wren 距离查询“关闭”、
  water2=Duck、water8“同点 4 次”、butterfly state0-3“判等”（flag 基线）。被推翻的旧结论中，
  **wren h3 三轴、despawn（非 panic）、anim_val×7、water8=Duck（组件映射）均核实成立**；
  “water8 同点采样”过度纠正。
