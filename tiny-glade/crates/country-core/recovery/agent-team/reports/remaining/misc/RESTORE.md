# remaining/misc RESTORE — forest / tree-tool / critters / glade-border 残余

- 日期：2026-10-17（restorer-remaining-misc）
- 方法：每函数 **fresh 反编译/反汇编**（Ghidra shared 只读 + radare2 字节级复核）→ 重建控制流 →
  Rust body 逐语句对照（判等协议 8 条分歧检查表）。
- **wave4 复核响应**：按 `reports/wave4/reviews/misc/REVIEW.md` 修复 4 项（wren h0 模式门/
  阈值、move water2 组件 Sheep、avoid Duck 两位置 4+4、butterfly dispatch flag 基线），
  并修正 near/sheep NaN 语义与相关注释/映射文档。
- 证据目录（本 agent 自取）：
  `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/recovery/agent-team/work/restorer-remaining-misc/evidence/`
  - `fresh-1.json` / `0x141bf16c0_manage_tree_prefabs.c` / `0x14226a260_hide_trees.c` /
    `0x14226be70_convert_preplaced_trees.c` / `0x141d84d40_update_butterfly_states.c` /
    `0x14231e390_update_wrens.c`
  - `fresh-2.json`, `disasm-0_141d84d40.txt`, `disasm-2_14231e390.txt`（+ r2 handler dump）
  - `fresh-4.json`, `0x142318fa0_move_critters.c`, `0x1423179a0_critters_avoid_collisions.c`,
    `0x140a12650_position.c`, `0x140d14770_despawn.c`
  - `fresh-6.json`, `0x14089d8b0_new.c`, `0x140b2cf90_write.c`, `0x141be1080_closure$1.c`,
    `disasm-display.txt`
  - `fresh-8.json`（`0x142923f10 = fmodf`、`0x142923fd0 = sinf`、`0x141625ae0 = uniform_f32`）
  - `avoid-full.txt` / `hide-full.txt`（r2 全函数反汇编）
- 跳表读回（r2，B EXE）：
  - `0x142f68008`（butterfly states）= `{0x141d85983, 0x141d85a22, 0x141d85ada, 0x141d85b08,
    0x141d86dd5, 0x141d86e29, 0x141d86df9, 0x141d86e1a}`（4-7 指向 set_butterfly_velocities）
  - `0x14304b638`（wren）= `{0x14231ebf8, 0x14231e930, 0x14231e9a9, 0x14231ec82, 0x14231edf4}`

## 结论总表

| 模块 | 原函数 / VA | Rust 位置 | verdict | 残余 / 分歧 |
|---|---|---|---|---|
| forest | `manage_tree_prefabs` 0x141bf16c0（**构造形状判定**） | `systems/forest/src/manage_tree_prefabs.rs` | **重建（该子判定判等）** | 记录 0x50B 布局/实体回写/SpawnPrefabCmd 0x50 仍 core-type-gap（原登记不变） |
| forest | `handplaced_tree_animation` / `forest_tree_animation`（核心写面） | `systems/forest/src/tree_animation.rs` | 算法判等；**两处 GPU 写面登记未实现** | `PrefabInstanceData[+0x20]`/`[+0xc]` 在 2026-10-17 权威布局修正后无承载字段 |
| tree-tool | `convert_preplaced_trees` 0x14226be70 | `systems/tree-tool/src/convert_preplaced_trees.rs` | **重建**（History 路径落库） | 位置源 Transform（PlacementDataSsbo 面）、scale/rotation 记录源、候选位图、rock buffer |
| tree-tool | `hide_trees` 0x14226a260 | `systems/tree-tool/src/hide_trees.rs` | **未实现（登记，core-type-gap）** | shape bit / gate byte / 可见性写数组 / Decorators→树映射 |
| critters | `update_butterfly_states` 0x141d84d40（state0/1/2/3） | `systems/critters/src/butterfly.rs` | **重建（判等；wave4 flag 基线已修）** | state4-7（异函数 handler）、audio u32 写值 |
| critters | `update_wrens` 0x14231e390（h0 距离查询 / h3） | `systems/critters/src/wren.rs` | **重建（判等；wave4 模式门/阈值已修）** | PlacementDataSsbo/debug 面、h1 音频槽 |
| critters | `move_critters` 0x142318fa0（water2） | `systems/critters/src/shared.rs` | **重建（判等；wave4 Sheep 组件已修）** | 纯墙体 relate 路径未实现；spherecast 半径参数；`PublicWalls::get` panic |
| critters | `critters_avoid_collisions` 0x1423179a0（water8） | `systems/critters/src/shared.rs` | **重建（判等；wave4 两位置 4+4 已修）** | Circle2d/LineInterval 形状近似；`PublicWalls::get` panic |
| glade-border | `display_glade_border` 0x141be0ad0（perimeter） | `systems/glade-border/src/display_glade_border.rs` | **重建（perimeter 写路径判等）** | `DebugLine::show`（0x140b04f70）仍缺（core debug_viz）；signifier closure 唤醒未接线 |

`cargo check`（workspace 根，2026-10-17 22:1x）：
- `cargo check -p forest` → **0 error**（Finished）
- `cargo check -p system-tree` → **0 error**（仅 process_tree_cmds.rs:209 既有 unused 警告，非本批）
- `cargo check -p system-critters` → **0 error**（8 既有警告，非本批）
- `cargo check -p system-glade-border` → **0 error**

> 并发注记：country-core 在 21:45 由其他 agent 改了 `PrefabInstanceData` 权威布局与
> `PathRastersResources::as_float_grid()` 签名；本批把受影响的 `forest/tree-tool/critters`
> 调用点按新 API 修正，未改 country-core。

---

## 1. forest — manage_tree_prefabs 构造形状判定（0x141bf16c0）

fresh 反编译 `evidence/0x141bf16c0_manage_tree_prefabs.c`（565 行）+ r2 复核。

**形状源（O）**：事件双缓冲（`+0x10` cap / `+0x18` len / `+0x28` data 与 `+0x30/+0x38/+0x28+0x10`）
各取**末条 0x28B 记录**（新缓冲有效优先，否则旧缓冲）。`0x141bf1d62/d64` 门：
`keep = !((buf2_empty && buf1_empty) | density_rock_blocked)` → **只有存在构造形状且
density==0 && rock==0 时运行形状判定**；形状判定失败时 `bVar33=0` **跳过**墙
`query_with_circle_callback`。

**判定体（O，0x141bf1d6a-0x141bf1dea）**：
- `tag & 1 == 0`：`center` 固定取记录 +4/+8（打包 2×f32）、`radius` +0xc；
  `dist = sqrt(dx²+dz²)`；`keep = radius+7.0 <= dist` → **过近 = `radius+7.0 > dist`**
  （`UCOMISS threshold,dist; SETA; XOR 1`；unordered → 不过近）。
- `tag & 1 == 1`：`Rectangle2d::closest_point_on_perimeter`（0x140c9ae20）→
  `dist < 7.0` 为过近。core 无同名方法，本实现以
  `Shape::Rectangle(rect).get_closest_u_from_pos` + `rect.get_pos_at_u(u)` 等价分解
  （记录 +4 axis / +0xc center / +0x14 size，与 `ShapeParameters::get_shape` 一致）。
  **地址勘误（wave4）**：0x140c9ae20 内部先 call 0x140c9a9d0（closest-u）；core 注释里
  `rectangle_closest_u_from_pos` 标 0x140c99530 **有误**（该地址实为 perimeter，
  0x140c99800 才是 rect `get_pos_at_u`）。分解拓扑一致；数值逐指令等价未验证 → 登记。

**Rust 落库**：新增 `construct_shape_of`（`ConstructTimberframeCmd.shape: Vec<u8>` 前 28 B →
`ShapeParameters::from_observed`）与 `is_too_close_to_constructed_shape`；阶段 1 改为
`shape 检查 → 通过才墙查询 → !blocked`，与二进制顺序/门控一致。`has_construct_shape`
由 `EventReader::read().last()` 承载（与原始双缓冲末条语义的差异：reader 游标 vs 原始
buffer，登记 I）。

**仍未实现（登记）**：放置记录 0x50B 的实体/init/+0x44 位、`SpawnPrefabCmd` 0x50 载荷、
`Pack2x16` 打包位、引用计数原子自增（原 RESTORE §2 登记不变）。

**附加修正（编译面）**：`PathRastersResources::as_float_grid()` 已改为无参返回
`[RasterBufferRef;4]`（core 2026-10-17 修正），forest 密度项按新签名循环四路采样；
`tree_animation.rs` 的 `PrefabInstanceData._gap/.scale` 写面在权威布局修正后不存在 →
保留计算、写面标 **未实现（core-type-gap）**，函数算法其余判等。

## 2. tree-tool — convert_preplaced_trees（0x14226be70）

fresh `evidence/0x14226be70_convert_preplaced_trees.c`。

- **Wait 门（0x14226be70 行 122-128）**：保持既有判等（事件帧 → count=1/flag=1；
  无事件帧 flag 置位 → count+1；`count+1>2` → 清 flag 执行一次）。
- **候选门**：4 路 path 栅格（`as_float_grid()` 无参新签名，同位置求和 clamp）+
  water + garden + rock；任一命中 → 发 `DeleteTreeCmd`（stride 0x10，payload=树 u64）。
  方向与历史复核一致。
- **History 路径（2026-10-17 落库）**：通过门禁的树组 `TreeEdit::Place(TreeEditData)`
  （position/id/prefab_name/is_preplaced=true），循环后
  `HistoryEntry::from_edits(edits)` + `insert_starting_edit(history.ledger_mut(), entry,
  fastrand::u64(..))`——对应二进制 `Vec<TreeEdit> drain → map(EditType::Tree) →
  History::insert_starting_edit 0x140a844f0`。core 已提供 `ledger_mut()`（分歧 15 关闭）。
- **残余（登记）**：① `PlacementDataSsbo::position()` 源 → 以 `Transform.translation`
  桥接（0x30-stride placement 记录不可得）；② scale/rotation 来自 placement 记录
  +0xc/+0x10 打包位，未取 → `TreeEditData::default()`（0.0）并显式标注；③ 候选源
  位图（Decorators::iter_removed_ids）vs `Query<ConvertedPreplacedTree>`；
  ④ rock mask buffer（空 buffer 占位）。

## 3. tree-tool — hide_trees（0x14226a260）— 未实现（登记）

fresh decomp + `evidence/hide-full.txt`（r2）复核，两条命令路：
- **NextToWall**（0x14226a5xx-0x14226a6df）：事件记录 **stride 0x18**；`PlacementDataSsbo::position()`
  → `Circle2d::new(center, radius)` + `WallSpatialHash::query_with_circle_callback`
  （回调 0x1425df320 翻转 flag）；命中后写隐藏数组（`u32[idx]=tick`、`byte[rec+idx*0x10+0xc]=1`）；
  **删除门 = 记录 +0x10 byte**。
- **Area**（0x14226ac50-0x14226ae3f）：事件记录 **stride 0x30**；`shape bit@0`：
  bit==0 → `Circle2d::with_expanded_diameter(2*delta@+0x20)`，bit==1 →
  `Rectangle2d::with_expanded_size(delta@+0x20)`（0x140c980c0/0x140c9a730）→
  `contains_point`（0x140c980f0/0x140c9a630）；另有 `f32@+0x1c` 距离/寿命门
  （`>0` 时与另一值比 4.0）；**删除门 byte@+0x24**。
- 候选源 = `Decorators::iter_removed_ids()`（位图）→ 装饰/预制实例表（0x14/0x30/0x48
  stride 内部表）→ 树 id/实体。

**为何不落库**：country_core `HideTreesinAreaCmd`/`HideTreesNextToWallCmd` 只有
`{center,radius}`，缺 shape bit/stretch/门控字节；候选源需要 `Decorators` 内部映射与
`iter_removed_ids` 对应表（系统签名无 Decorators 资源）；可见性写目标数组未建模。
按硬标准 **未实现（core-type-gap）**，不改 country-core（他组并行编辑）。

## 4. critters — butterfly state0/1/2/3（0x141d84d40）

fresh decomp `0x141d84d40_update_butterfly_states.c` + r2 handler 反汇编
（`0x141d85983 / a22 / ada / b08`）+ 跳表读回 + 常量（`0.1@0x142925984`、`0.3/0.6/1.5/
5.0/10.0/2π/-0.7/0.5`、表 `0x142f68008`）。

**记录布局修正**：`Butterfly` 新增 `state_t @ +4`（`components.rs`），完整布局
`{state@0, state_t@4, goal_timer@8, dwell@0xc, phase@0x10, phase_ref@0x14}` = 0x18B。

**主循环（O）**：`phase = max(0, phase-0.5dt)`；`near || sheep_near → 1.0`
（near：mode3 `dist<3.0`，否则 `dist²<4.0`；sheep：任一 `d²<4.0`；均按 `SETA`
语义、unordered→false）。派发点 XMM0 的 `t` 初值按路径（新 dwell / 0.0 / mask&4 /
负 dwell）逐路重建。

**dispatch flag 基线（wave4 FIX）**：`flag_in = phase>0 ? 4.0 : baseline`，baseline
逐路不同（fresh 0x141d857b4-0x141d85981，常量 1.0@0x142925904 / 3.0@0x1429cd078 /
4.0@0x142934694）：
- `dwell<=0` 两路、`velocity.y>0`、相位门外（`off<0.2 || off>=0.45`）→ **3.0**；
- `|0.5-phase_ref|∈[0.2,0.45)` 的 dwell-dt>0 与随机重置两路 → **1.0**。
Rust 已按六路表落库（旧实现一律 0.0 系误读）。

**handler（O，逐指令）**：
- state0：`phase_ref<=0.5` → `flag = min((0.5-phase_ref)*0.1/dt, flag)`；`t -= dt`；
  `!(t>0) || near/sheep → 3`，否则 0。
- state1：`phase==0 && goal.active && rock==0 && water<0.01` 才进入；`height>0.6` →
  state1/t=0.6；否则 `t=uniform*10+5`、state0；其余 state2。
- state2：`t -= dt`；goal.active → `2 + (t<=0 && phase==0)`（NaN → 2）；否则
  `t=max(t,5.0)`、state2。
- state3：`t=height`；`height>=1.5` → `t=uniform*5+5`、state2（NaN 保持 3）。
- 续段（0x141d852df）：`state=r15`、`state_t=xmm0`、`phase_ref=fmod(phase_ref+flag*dt,1)`
  （`0x142923f10=fmodf` 已核）。

**残余（U）**：state4-7（跳表项指向 `set_butterfly_velocities` 的动画 handler，非本
上下文有效 handler）；`ButterflyAudio` 每帧 u32 写值（query 派生值）。

## 5. critters — update_wrens（0x14231e390）

fresh decomp + r2（`evidence/disasm-2_14231e390.txt`、h0/h3 dump、常量读回）。

- **wren 距离查询（wave4 FIX）**：`0x140a12650` =
  `CursorTerrainRaycast::position`（反编译 17 行：hit bit → 返回 +4/+0xc，否则 0）。
  h0：`timer -= dt`；`timer<=0 → h1_mid`；**阈值 `proximity` 由 AppMode 产生**
  （fresh 0x14231e7d9-0x14231f5fb：`AppMode::Wall(Circle|Rectangle)` → 3.0；
  Path/Cobblestone/Eraser/Terrain/Water → 1.0；其余 → 0.0 关闭查询），
  `proximity > 0 && dist < proximity` 才 h1_mid；否则高度更新。
  Rust 已把 `Res<AppMode>` + `Res<CursorTerrainRaycast>` 接入 `update_wrens`，
  `wren_proximity_threshold` 按 tag/子字节逐分支落库（旧实现恒 `<1.0`、无门；
  资源身份按 native_tag 集合 {7..10,15} 与 `byte[+4]`=WallShape 判别取 AppMode，
  强候选但仍登记 U）。
- **h3 移动式修正（推翻 REVIEW "合并式"）**：fresh 0x14231ed04-0x14231ed62：
  `pos.x += vx*dir_x*dt`；`pos.y += vx*dvy*dir_y*dt`；`pos.z += vx*dir_z*dt`
  （dir_x=+0x34/timer、dir_y=+0x38、dir_z=+0x3c；xmm2/xmm3 分列）。旧实现
  `y += vx*dt*(dvy*dir_y+dir_z)` 且 z 不变是错的。
- **progress>6.0**：`0x140d14770` 实为 `bevy_ecs::EntityCommands::despawn`（反编译
  30 行，非 panic）→ `commands.entity(entity).despawn()`；不再重置 progress/state。
- 尾部 `anim_val = progress*7.0` 保持。

**残余**：PlacementDataSsbo::to_affine3a / DEBUG_SHOW_* 调试绘制 / h1 音频槽写
（query 派生值）/ 实体映射（crate 以 wren 实体承载 spot）——登记。

## 6. critters — water2 / water8 映射（move_critters / critters_avoid_collisions）

fresh `0x142318fa0_move_critters.c`、`0x1423179a0_critters_avoid_collisions.c` +
`evidence/avoid-full.txt`（r2）。

**映射（已关闭，wave4 修正）**：
- move 查询 `(Transform, Locomotion, Velocity, Option<Multiplier>, Option<Duck>,
  Option<Sheep>, Option<BirdSpots>)`；cursor 列 `+0x128/+0x130/+0x138/+0x140` →
  **+0x130 = Duck（水面阻塞 water2 标志）**、**+0x138 = Sheep（双水采样 + 半径 0.8）**。
- avoid 查询（本轮修正）`(Entity, Transform, Goal, Locomotion, Option<Sheep>,
  Option<Duck>)`：**+0x128 = Sheep（0x68B 记录）**、**+0x130 = Duck（water8）**。

**move_critters 修正（O）**：
- 水面采样改在**预测位置** `pos+disp`；`loc==2`：`rock>0 || water>0 || (water<=0 && duck)`
  阻塞；非 loc2：`water<=0 && duck` 阻塞。
- Duck/Sheep → spherecast 路；**water2：Sheep**（`[rbp+0x23c]`=+0x138）且 `loc!=4` →
  `water(pos)` 与 `water(pos+disp)`，`pos<=0 && disp>0` → 阻塞（wave4 FIX：旧写 Duck）。
- 残余：纯墙体 relate 路径（无 Duck/Sheep 实体）未实现；spherecast 半径
  （Sheep 0.8@0x142a16cb8 / 其它 0.5@0x142925870）crate helper 无参数；`PublicWalls::get`
  缺失的 `unwrap_failed` panic 以 continue 兜底。

**critters_avoid_collisions 修正（O）**：
- `water8` = **Duck**（不是 `!samples_rock_mask`）；**两位置 4+4 采样**（wave4 FIX）：
  `c0=clamp(Σ4 sample(pos))`；`d=(c0>0)?|goal.position|:0.3`；
  仅 `c1=clamp(Σ4 sample(pos+dir*d))>0` → 清目标（旧实现单点 4 采样判 c0）。
- `locomotion==2` 前置门（0x1423183c4-0x14231840a）：`pos+dir*0.1` 同时采 water 与
  rock，任一 >0 → 清目标。
- Sheep → 直接 spherecast（跳过墙路）；其它 → 墙 relate：**任一 Intersection → 清目标；
  遍历完无 Intersection → 保留目标**（`do{...}while((int)relate != 0)`，Intersection
  判别 0；原 Rust "非 Disjoint → 保留" 方向相反，已修）。
- `<0.4 && Duck` → `DespawnDuckCmd`。
- 残余：Circle2d::new_rotated + line_segment 的 relate 形状以 segment-vs-segment 近似；
  `PublicWalls::get` panic 路径。

## 7. glade-border — display perimeter 写路径（0x141be0ad0 / 0x141be1080）

fresh `disasm-display.txt`（246 条）+ `0x14089d8b0_new.c`。

- **perimeter 写路径（已落库）**：`alloc 0x30` 写 48B
  `{tag=1, kind=1, cap@+0x10, ptr@+0x18, len@+0x20, perimeter@+0x28}`
  （disasm 0x141be0e3a..0x141be0e55；三 qword 顺序由 0x141be0be7/0xc18 证：
  `[+0x90]=len`（`12*len` 分配）、`[+0x88]=ptr`（memcpy src）、`[+0x80]=cap`）→
  `SignifierFactory::new(label, record, 0x142efa150)`（0x14089d8b0：
  `memcpy(dst, src=rdx, size=r8)`）→ `UiSignifierStream::write_factory`（0x140b2cf90 5×qword）。
  所有权按原版移交：`mem::forget(factory/points)` 保持 label/记录/折线存活。
- **label**：`format!("glade_border{}", game_time.frame_count)`（无空格，12B 格式串）。
- **DebugLine::show（0x140b04f70）仍未实现**：core `debug_viz::DebugLine` 仍无 `show`
  （doc 明示 show 系列留组件/ECS 包）→ 按任务前提"若 core debug_viz 已落"判定为
  **未落**，登记 country-core 任务，不伪造。
- closure$1（0x141be1080）保持既有落库；唤醒路径仍为 core signifier 消费者 gap。

---

## 改动文件（绝对路径）

- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/forest/src/manage_tree_prefabs.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/forest/src/tree_animation.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/tree-tool/src/convert_preplaced_trees.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/critters/src/components.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/critters/src/butterfly.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/critters/src/wren.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/critters/src/shared.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/critters/src/duck.rs`（`as_float_grid` 新签名适配）
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/glade-border/src/display_glade_border.rs`
- 未改：`process_tree_cmds.rs` / `replay_tree.rs`（W1 consume-types 独占）、country-core。

## 三态回执

- **完成**：forest 构造形状判定（形状源+点/矩形+门控顺序）；tree-tool convert History
  写路径；critters butterfly state0/1/2/3（wave4 flag 基线修正）、wren 距离查询
  （wave4 AppMode 门/阈值）/h3 精确移动/despawn、water2=Sheep（wave4）/water8 两位置
  4+4（wave4）与墙 relate 方向；glade-border perimeter 写路径。四 crate `cargo check`
  均 0 error。
- **登记**：hide_trees（core 命令 shape/gate 字节、候选源映射、可见性数组）；
  convert 的 position 源/scale-rotation/位图候选/rock buffer；forest 记录 0x50B 布局与
  `SpawnPrefabCmd`；critters state4-7、move 纯墙路、spherecast 半径、PublicWalls panic；
  glade-border `DebugLine::show`（core debug_viz）与 closure 唤醒。
- **阻塞**：无（均以登记收敛；country-core 由他组并行编辑，未改动）。
- **wave4 复核后计数**：判等 3（forest/manage 形状、tree convert、glade perimeter）/
  需修 4（butterfly flag、wren h0 门、move water2 组件、avoid water8 两位置）已全部修复 →
  修复后 **判等 7 / 未实现 2**（hide_trees、DebugLine::show）。
