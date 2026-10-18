# REVIEW — reviewer-B-eco：forest / tree-tool / critters / glade-border B 类逐函数独立复核

- reviewer：`reviewer-B-eco`
- 日期：2026-10-18
- 范围：`planning/2026-10-17-evidence-audit-ecstate.md` §2.B 中四个模块的全部 B 项 = **33 个函数**
  （glade-border 2、forest 0、tree-tool 8、critters 23）。
- 方法（硬标准）：对每个函数 **fresh Ghidra 反编译**（`--worker shared`，`program=tiny-glade.exe`），
  否则用 `disassemble_function` / `read_memory` 定点取证；独立复述原控制流后，与
  `crates/systems/{glade-border,tree-tool,critters}/src/**` 逐语句对照；逐条过
  `planning/2026-10-16-logic-verify-protocol.md` 的 **分歧检查表 8 条**。
- **未改 src。**
- 证据：`evidence/`（fresh 反编译 35 份、常量实读 `consts.json`、3 组反汇编 JSON）。
- 时间线注意：`reviewer-B-ecstate` 的 8 项复验报告写于 2026-10-17 23:32；其点名的
  `shared.rs`/`butterfly.rs`/`replay_tree.rs`/`process_tree_cmds.rs` 在 **23:33–23:36** 又被
  restorer 修改（mtime 实证）。本报告针对**当前源码**，因此多处闭合了 B-ecstate 报告的旧 FIX
  （纯墙路、spherecast 半径、state2 次态、Move-undo 旧位等）。本报告即为这些修改的 post-fix
  独立复验。

## 0. 三态结论

| 模块 | B 项 | 判等 | 需修 | 未实现 |
|---|---:|---:|---:|---:|
| glade-border | 2 | 2 | 0 | 0 |
| forest | 0 | 0 | 0 | 0 |
| tree-tool | 8 | 1 | 4 | 3 |
| critters | 23 | 4 | 18 | 1 |
| **合计** | **33** | **7** | **22** | **4** |

> forest 的 5 个函数在审计中为 4 C + 1 D，无 B 项，本轮无动作。

### 0.1 判等（7）

`display_glade_border`、`display_glade_border::closure$1`、`ui_place_tree`、
`despawn_fish`、`duck_splash_particle`、`sheep_floaties`、`sheep_umbrellas`。

### 0.2 需修（22）

`process_tree_cmds`、`replay_tree`、`place_tree`、`ui_move_tree`、`ui_delete_tree`、
`BoidFlock::remove`、`move_critters`、`critters_avoid_collisions`、`DuckSpawnLocations::new`、
`spawn_ducks`、`despawn_ducks`、`plan_duck_trajectories`、`move_ducks_along_trajectories`、
`ducks_react_to_being_inside_buildings`、`update_butterfly_states`、`sim_wren_boids`、
`update_wrens`、`update_sheep_goals`、`sheep_unstuck`、`react_to_being_inside_buildings`、
`sheep_reaction_emoji`、`sheep_input_record_replay`。

### 0.3 未实现（4）

`tree_move_bend`、`hide_trees`、`ui_pet_sheep`、`ui_pet_ducks`。

---

## 1. glade-border

### 1.1 `display_glade_border` @0x141be0ad0 — **判等**

fresh 反编译（`evidence/0x141be0ad0_display_glade_border.c`）逐段：

1. 模式门 `CMP dword[*AppMode],0x13; JZ`；
2. `ShapeParameters::as_curve2` + `from_iter<Vec3, Map<Iter<Vec2>, closure$0>>`（closure$0 见审计 C）；
3. 事件门（Events 双缓冲未读计数）；进入后 `DebugLine::new → color → line_width → set_ui_pass → show`，随后
   `*0x14362c098==1` 才写 demo overlay；
4. `config[+0x121]==0` → `perimeter()` + `format_inner`（piece 表 `0x142efa140` = {ptr 0x142efa130, len 1, 0xc}，
   实读字符串 `glade_border` **12B 无空格**；1 个 Display 参数 = `GameTime+0x2c`）+ `alloc 0x30`
   写 48B 记录 `{1,1,cap,ptr,len,perimeter@0x28}` + `SignifierFactory::new` + `UiSignifierStream::write`。

Rust 对照：模式/曲线/事件门/show/格式串/perimeter 门逐条一致；`from_closure` 的闭包 env =
`{cap@0,ptr@8,len@0x10,length@0x18}`，与 closure$1 读取的 `record+0x10` 视图逐槽一致。
登记 I（非行为）：48B 记录的 `tag/kind` 两 word 在 Rust env 中未单独承载（closure$1 不读）。

### 1.2 `closure$1` @0x141be1080 — **判等**

fresh 反汇编（`evidence/disasm-place-tree-closure1.json`）逐指令：

- `alloc 12*len + memcpy`（env `+8=ptr / +0x10=len`）；
- builder payload 4 个已初始化 qword `{0x8000000000000002, len, ptr, len}`（`MOV RAX,-0x7fff...ffe`；
  后 2 word 为调用者栈未初始化）；
- `set_ui_pass(0)`（R8D=0）、`animation_duration(0)`（XORPS XMM2）、
  `custom_shader("dashed_line",0xb)`、`build`；
- 2× `Commands::insert`：`DrawTopology=0x4000000000000001`（`Alt(2.0)`）、
  `DashedLineLength = env[+0x18] * [0x1429cd030]=0.7`；
- 返回 1 元素 `Vec<Entity>`（sret `{1,ptr,1}`）。

Rust 逐条一致。builder payload 尾 2 word 原生未初始化、Rust 取 0：登记 I。

---

## 2. forest

审计 §2.B 无 forest 项（4 C + 1 D），本轮不重复复核。

---

## 3. tree-tool

### 3.1 `process_tree_cmds` @0x142267430 — **需修**

fresh 反编译 906 行（`evidence/0x142267430_process_tree_cmds.c`）。已确认判等面：

- **Move**（槽 0x20）：`TreeStorage::get(index@+8)`；None → 日志
  `"MoveTreeCmd references a non-existing tree"`；Some → 定位放置记录写 x/y/z + 变更 tick，
  `inverse_lerp(0.5,0.9,scale)` → 预测点水采样 → `PlayTreeSoundCmd{kind=1, selector=water>0,
  value=inv_lerp, position}`（槽 0x20 逐偏移）。
- **Delete**（槽 0x10）：`storage.remove` → `Entities::contains(entity)` 门（当前 Rust 已落）→
  `TreeMarkedForDeletion` + `kind=2` 音效。
- **Insert**（槽 0x48）：5 个 naked 名门（长度 0x19/0x1a/0x27，decomp 字符串逐字节比对）、
  `ActiveWidget::is_dragging`、`deny_load`、`is_corrupted`、`max(scale,0.7)`、
  `alpha=r*0.75+(1-r)*0.55`、spawn 四组件、`TreeMoveBend{x,z}`、`TreeAmbience(x,y+scale*10,z)`、
  `MISSING_TREE_PREFAB_NAME` 兜底、`SpawnPrefabCmd` 0x50、`storage.insert` 冲突日志+Despawn、
  水采样 + `kind=0` 音效（`+0xa = naked&(deny|corrupt)`）。

**FIX（分歧点）**：

1. **is_dragging 门未建模**（分歧检查表 #2/#3）：原生对每个 Insert 事件调
   `ActiveWidget::is_dragging`（decomp 行 339）；拖动时 `local_120 = (event flags@+0x2c & 3)!=0`，
   非拖动为 0；`SpawnPrefabCmd.mode` = `naked&&deny ? 3 : (flags 字节!=0 ? 0 : local_120+1)`。
   Rust `mode` 恒为 1，且 `drag_started` 硬编码 false。
2. **is_corrupted 固定 false**（decomp `BackupSaveFileKind::is_corrupted` 行 524）；`kind=0`
   音效 `extra` 与 `random_tree_prefab` 第 4 参因此不同。
3. **SpawnPrefabCmd 载荷**：原生 `rotation@+0x28`、`variant@+0x2c` 直接取事件
   `puVar9[5]`；Rust 写 0（共享 `InsertTreeCmd` 无 rotation/variant/flags/height 承载）。
4. Move 的位置源：原生经 wall/decorator 表取 0x30B 放置记录；Rust 用 `Transform` Query 投影（I）。

### 3.2 `replay_tree` @0x141bc34f0 — **需修**

fresh 反编译 413 行（`evidence/0x141bc34f0_replay_tree.c`）。已确认判等面：

- `front_mut_bucket` None → return；事件 stride 0x30，tag 字节 8；
- `elapsed@+0x18 == 0.0 && !NAN` 才重发；
- disc 2=Delete（non-undo → DeleteTreeCmd id=edit+0x18；undo → Insert）、
  1=Move（non-undo → edit+0x20/+0x28，undo → **edit+0x14/+0x1c 旧位**，index=edit+0x30）、
  0=Place（non-undo → Insert；undo → Delete）；
- 时序 `flags@+8 bit0==0` 立即入移除集；否则 `delay=cmd+0xc/speed`、下钳 0.05、`delay<elapsed`
  入集、`elapsed += GameTime.delta`；移除集逆序 `Vec::remove`（swap-halves+memmove）。

**FIX**：InsertTreeCmd 0x48 重建仅落 name/position/scale/id；原生还写
`height@+0x4c`、`rotation/variant`（edit+0x44 区）、`flags@+0x50`（共享类型缺口 T2）。
Move-undo 旧位（B-ecstate 的旧 FIX）在当前源码已实现（`previous_position`），本轮判等。

### 3.3 `tree_move_bend` @0x1422689c0 — **未实现**

fresh 反编译（`evidence/0x1422689c0_tree_move_bend.c`）逐段：两次
`time_continuous_lerp(...,5.0)`（0x142925988=5.0 实读）、`dist>=0.1`、
`step=min(5,dist)*d/dist`、`bend=target+step` 打包 u64；变更检测比较
`PrefabInstanceData+0x24/+0x28`（step_x/step_z，含 NaN 语义），不同则写
`{step_x, step_z}` + 变更 tick。

Rust 的 bend 数学/rate/阈值/打包全部与反编译一致，但 **不写 +0x24/+0x28 与 tick**
（country-core 放置记录无这两个字段）。按分歧检查表 #8（字段缺失）→ **未实现**；
解锁条件 = core 补 native 0x30B 放置记录的 step 字段。

### 3.4 `hide_trees` @0x14226a260 — **未实现**

fresh 反编译 579 行确认两个分支均未落：

- **Area**（stride 0x30）：shape bit@+0 → `Circle2d::with_expanded_diameter(delta@+0x20)` /
  `Rectangle2d::with_expanded_size` + `contains_point`；距离门 f32@+0x1c、删除门 byte@+0x24；
- **NextToWall**（stride 0x18）：`TreeStorage::iter()` 候选 + 放置记录位置 + `Circle2d::new(pos,0.4)`
  + `WallSpatialHash::query_with_circle_callback`；命中写可见性
  `u32[id]=tick`、`byte[record+id*0x10+0xc]=1`，删除门 byte@+8 → `DeleteTreeCmd` + `History::add`。

Rust 为诚实空体（`let _ = (...)`），零业务实现 → **未实现**。原 B-ecstate 旧报告对候选源的
`Decorators::iter_removed_ids` 判断已被 fresh 反编译否证（实为 hashmap iter）；当前注释正确。

### 3.5 `ui_place_tree` @0x142269170 — **判等**

fresh 反编译（`evidence/0x142269170_ui_place_tree.c`）+ 反汇编
（`evidence/disasm-place-ui-umbrella.json`）：

- 门禁 `*AppMode==0x10 && demo==0`；
- `WidgetParams::new`（volume tag=5、`f32::MAX@+4`）→ `occlude_by_world(false)` →
  `UiSounds::empty()` → `UiState::add("tree_canvas")`（实读 11B）；
- `legacy_raycast`：返回槽 `+0x30 != 2` 才 `geo_src_to_edit_subject`，subject 非空 → 早退；
- `position()` → `GladeBorder::is_pos_inside`：界外 → `DisplayOutOfBorder`；`clicked` 再
  `FeedbackInputOutOfBorder`；
- 界内 `is_active` → `format!("decorator_{ss}")`（piece 表 len 10）+ 32B 记录
  `{1,1,ss(Vec2)@0x10, WindowSize::as_vec2().y@0x18, config+0xe8@0x1c}` +
  `SignifierFactory::new(marker 0x143035160)` + `write`；
- `clicked` → 再次 `position()`，`y<0 → (y/-2.5)*-1.2` → `PlaceTreeCmd{position}`。

反汇编实证：`XMM8=ss`（`MOVSD [RBP+0x1a8]`）、`XMM6=as_vec2().y`（`CALL 0x140949c10` 后
`MOVAPS XMM6,XMM1`）、`XMM7=[config+0xe8]`。Rust 逐字段一致。登记 I：volume `size.y`
原生未初始化（Rust 取 0）；signifier 记录所有权以 `Box::leak` 保持。

### 3.6 `place_tree` @0x142269970 — **需修**

fresh 反编译 + 反汇编（`evidence/disasm-place-tree-closure1.json`）。已确认判等面：
`next_index` → `fastrand::f32` r1 → splitmix 变体（常量 0xa076…/0xe703…/retry 0x40ec…、
拒绝 `(u32)(mixed*10)<6`、`variant=p>>32`）→ `fastrand::f32` r2 → `random_tree_prefab(id 低字节)` →
scale=`r1*0.9+(1-r1)*0.5` → History Place。

**FIX（新发现）**：

1. **旋转下限常量错**：原生 `rotation = r2*360 + (1-r2)*0.0`。反汇编
   `MOVSS XMM8,[0x142925870]=0.5` 后 `MULPS XMM0,XMM8`（MOVSS 清高 lane，lane1 乘 0）；
   decomp 也显示 `* 0.0`。Rust `TREE_ROTATION_MIN = 0.492_200_02` 错误（且实读
   0x142925874=0x3efc0fc1=0.49230769，与注释值也不符）。应改 0.0。
2. **InsertTreeCmd 0x48**：rotation/variant/flags/height 未承载（T2）。
3. 会话门 `deny_load/is_corrupted/drag_started` 硬编码（T4）。

### 3.7 `ui_move_tree` @0x14226b1a0 — **需修**

fresh 反编译（`evidence/0x14226b1a0_ui_move_tree.c`）。已确认判等面：
门禁 `mode∈{0x10,0} && !demo`；`position()` + 水下重映射；`format!("move_tree_{id}")` →
`dragged()`；`AL-2`、`>2→3`（state 0/1 亦钳 3，与 Rust `wrapping_sub` 一致）；
case1 记录 start+offset；case2 target=光标+offset、`y=max(h,0)`、界内 MoveTreeCmd、
界外 **双事件**；case3 同 + `History::add`（Move disc=1）。

**FIX**：

1. 候选源：原生 `Decorators::iter_removed_ids` 位图；Rust `storage.iter()`（D9）。
2. 原生按 prefab 名前缀判定可移动并取 widget 偏移（2.5/1.0）；Rust 无该门。
3. case1 原生还把 widget 名 + `x0y(offset)` 写入 UI widget 记录（local_180）；Rust 未建模。
4. History `TreeEditData` 仅旧/新位置+id；scale/rotation/prefab 未取（原生 live 记录亦只写这些，
   但 Rust 以默认值填充，登记 I）。

### 3.8 `ui_delete_tree` @0x141bd2510 — **需修**

fresh 反编译（`evidence/0x141bd2510_ui_delete_tree.c`）。已确认判等面：
demo 门 + `AppMode==Eraser(Tree)`（tag9/sub6）；`clearing_canvas_w_layer("tree_erase",0x0a,3)`；
`dragged()!=2` 取光标；`radius=brush*3.0`；`PathDrawEvent` `flag@+0x18==0` →
center=pos@+8/+0x10、radius=2×@+0x14、active；`sqrt(dx²+dz²)<radius` → DeleteTreeCmd + History Delete。

**FIX**：候选源 `storage.iter()` vs `iter_removed_ids`；位置源 `TreeStorageEntry.position`
vs 放置记录；History `scale/rotation/is_preplaced` 未填。`bVar30`（拖拽!=2 或有效路径事件）
门由 `active` 覆盖，判等。

---

## 4. critters

### 4.1 `BoidFlock::remove` @0x141dc2340 / 0x141dc2540 — **需修**

fresh 反编译（`evidence/0x141dc2340_remove_...c`）：索引 Vec 倒序；boid 0x18 槽
`swap_remove`（越界 → `alloc::vec::impl$1::swap_remove::assert_failed` panic）；aux 0x38 槽
`swap_remove`（越界 panic）；`Entities::contains` → despawn；结束 dealloc 索引 Vec。

Rust `flock_remove` 正常路径判等；**FIX**：boid 越界用 `break`（native panic，检查表 #1）。
aux 越界已 panic（一致）。正常态索引恒合法，影响低。

### 4.2 `move_critters` @0x142318fa0 — **需修**

fresh 反编译 1041 指令反汇编 + decomp。已确认判等面（wave4 FIX 全部 verified）：

- `y=max(y,0)`；multiplier 门（None→1.0、0.0→跳过位移）；
- 预测点水采样；loc==2 `rock>0 | water>0 | (water<=0 & Duck)`；非 loc2 `water<=0 & Duck`；
- Duck/Sheep → spherecast 内层 `loc>6 || !(99>>loc&1)`；**半径 = 0.8@0x142a16cb8 / 0.5@0x142925870**
  （反汇编 14231a139-14a 实证，Rust 已传）；`hitcode!=2 && dist<1.0` → blocked；
- Sheep 且 loc!=4 双水采样；blocked 跳位移/朝向/BirdSpots（`if !blocked`）；
- **纯墙路径**（反汇编 142319d00 起）：`WallSpatialHash::query` → `PublicWalls::get`
  （缺失 `unwrap_failed` panic，Rust `expect`）→ `loc!=3 && max_y_gable<pos.y` 跳过 →
  `Circle2d::new_rotated` + `LineInterval::line_segment` + `relate`；
- 朝向 `from_rotation_arc(dir_x0z,up)` + 符号翻转 nlerp；BirdSpots 偏移 (-0.495,1.567,0)。

**FIX**：墙 relate 形状——原生 `Circle2d::new_rotated` 后转 `LineInterval`，Rust 仅
`line_segment(pos,pos+dir)` 的 segment-vs-segment（登记近似，不是逐语句）。其余残留
（query 环境值写、变更 tick、spherecast 壳实现）为已登记 glue/ABI。

### 4.3 `critters_avoid_collisions` @0x1423179a0 — **需修**

fresh 反编译 850 指令反汇编。已确认判等面：`active` 门；loc2 在 `pos+dir*0.1` 同点采
water+rock；**Duck 两位置 4+4**（c0 clamp、`d=c0>0?|goal|:0.3`、c1 clamp>0 → 清目标）；
Sheep → spherecast；其它 → 墙 relate `do{}while(relate!=0)`（Intersection → 清目标、遍历尽 → 保留）；
`loc<7 && 99>>loc&1` 保留；半径 0.8/0.5；`hitcode==2 || dist>=1.0` 保留；`dist<0.4 && Duck`
→ `DespawnDuckCmd`；收尾清 active。

**FIX**：墙 relate 形状（同 4.2，`Circle2d::new_rotated` 缺失）；查询环境值写未建模（登记）。

### 4.4 `DuckSpawnLocations::new` @0x140f8ee90 — **需修**

fresh 反编译（`evidence/0x140f8ee90_new.c`）：外层 i=0..9 行公式写 x；
每行 10 个元素 `{Vec2{x=row, z=const}, used=0}`；资源尾 `+0x18=25.0`；100 项。
Rust 行/列均用 `(k/9)*52-(1-k/9)*52`。

**FIX（位级）**：原生 z 列是 10 个硬编码常量，其中
`j=2 = 0xc1e71c73`、`j=3 = 0xc18aaaa9`；Rust 公式算出 `0xc1e71c71`、`0xc18aaaab`
（**各差 2 ULP**，实算对拍）。其余 8 档逐位一致。应改为字面常量表。
（x 行值由公式计算，与原生一致。）

### 4.5 `spawn_ducks` @0x140f8f210 — **需修**

fresh 反编译（`evidence/0x140f8f210_spawn_ducks.c`）。已确认判等面：demo==0 门；
timer += dt，有活鸭再 +2dt，`>=30`；活鸭 `<15`；候选水采样 `>0.99` 跳过（写 used=0）、
地形高 `>0` 跳过；3 点轨迹（首点候选位 + k=1,2 `golden_spiral(k+u)`）；uniform 圆偏移；
push plan；used=1；timer=0。

**FIX**：

1. **is_corrupted 门缺失**（原生与 demo 门同时 `BackupSaveFileKind::is_corrupted()==false` 才进）。
2. **候选选择**：原生 `itertools::sorted_by<closure_env_0>` 后**从尾部向前**遍历；Rust 按下标
   正序取首个。排序比较器与本轮比较均未恢复（选择结果可不同）。
3. 水采样 `>0.99` 时 Rust 未把 `used` 置 0（原生显式清）。

### 4.6 `despawn_ducks` @0x140f8f8b0 — **需修**

fresh 反编译（`evidence/0x140f8f8b0_despawn_ducks.c`）：
state==2：`water<1e-5` 或 `pathsum.clamp(0,1)>0.99` → 离场；
state<2：轨迹 len≥2 时末点 `water<0.1` 或 `pathsum>0.9` → 离场；
state==3 不生成。

**FIX（新发现）**：

1. **原生不 despawn**：整个反编译中**没有** `EntityCommands::despawn`；离场只 push 1 点
   `TrajectoryPlan`（`{trajectories:[1点], offset:uniform_sample_circle}`），实体由计划执行方
   处理。Rust 直接 `commands.entity(entity).despawn()` → 跳过离场轨迹。
2. **plan key**：原生 plan `+0x20 = uVar8` = 该鸭实体 id；`+0x10 = 1`、`+0x14/+0x18 = circle`。
   Rust `key: 0` → `plan_duck_trajectories` 会把它当成全局出生计划（key==0 分支），
   **行为反向**。应写 `key = entity.to_bits()`。
3. 轨迹门以 `len>=2` 近似原生「len>0 且末点≠首点+0xc」（I）。

### 4.7 `despawn_fish` @0x140f90c10 — **判等**

fresh 反编译 191B：两段 Vec 存储 dealloc 后重写空布局（`0,4,0 / 0,8,0`），随后
`FishSpawnLocations::new()` 就地覆盖。Rust `*boids = FishBoids::default();
*locations = FishSpawnLocations::new()` 语义一致（无实体 despawn）。

### 4.8 `plan_duck_trajectories` @0x14230ff00 — **需修**

fresh 反编译确认：dev 热键 A/B（`developer_hotkeys_enabled` + `just_pressed<KeyCode>`）、
golden spiral / uniform circle、`legacy_raycast` 重采样、shape 采样 closure_env_4/5、
spawn bundle、`retain` 年龄上限（Rust 0x12d）。主体结构在 Rust 中已实现。

**FIX**：旋转弧两端向量未按寄存器闭合（U）；shape 采样闭包语义 U；spawn 10 组件 bundle
以近似 bundle 承载；`Duck::new` 前 audio 参数 U；`Frenet`/姿态面未完全重建。

### 4.9 `move_ducks_along_trajectories` @0x142312200 — **需修**

Rust 已实现：state==2 跳过、dir ±1、target、progress、chord、`CurveInternalCoord`、
姿态/速度、`f_f4`、grass push、音频分支、splash 事件、高度积分。

**FIX**：`from_rotation_arc` 端点 U；`set_volume`（WaterAudioGrid）未实现；两处
`AudioLoop::set_parameter` 参数名/值 U；reverse 的 `progress*progress` 曲线需按
反汇编逐点复核（登记 U）。

### 4.10 `duck_splash_particle` @0x142322650 — **判等**

fresh 反编译（`evidence/0x142322650_duck_splash_particle.c`）：
① `age += dt/2.5`，`<=1.0` clamp、否则 despawn；
② 事件实体 → Transform Query（查不到跳过）→ `get_handle_by_name("splash_particle")` +
`get_raster_handle_by_name`（缺失 `unwrap_failed`）→ `spawn<(DrawableMeshBundle,
SplashParticle, DontCastShadow)>`（transform = 事件实体变换）。
Rust 逐条一致；SplashParticle 初值/字段布局为 I。

### 4.11 `ducks_react_to_being_inside_buildings` @0x14231b140 — **需修**

原生（fresh decomp）：逐鸭（DuckState==2）→ `PublicWalls::get` 墙组
（count@+600<=1 且 water@+0x280<=1.5 才进）→ `get_wall_shape` 围合 + `max_y` +
`roof_entity` + `Roof::height` + `GladeBorder::is_pos_inside` → 水/形状/屋顶高度门 →
`AudioInstance::start_if_not_playing`。

Rust 用 `walls.all()` 全墙遍历 + 手写 2D 射线法 + `Roof::height` 上界；**FIX**：
墙组查询/过滤结构（+600/+0x280）、`get_wall_shape` 语义、`GladeBorder` 门顺序与
轮廓逐点 y 比较均不同（非 glue，属近似业务）。

### 4.12 `update_butterfly_states` @0x141d84d40 — **需修**

fresh 反编译（`evidence/0x141d84d40_update_butterfly_states.c`）确认：
mode==3 → `sqrt` 距离 `<3.0`；否则平方距离 `<4.0`；任一羊平方距离 `<4.0`；
`phase=max(0,phase-0.5dt)`，near/sheep → 1.0；dwell/phase_ref 三参派发，
flag 基线 `phase>0 ? 4.0 : 3.0`（dwell<=0、vel.y>0、相位门外）与 `4.0/1.0`
（中段两路）；state2 次态 = `t<=0 && phase==0 ? 1 : 2`（当前 Rust 已改为 1）。

**FIX**：① **handler 的 t 源**：B-ecstate 独立反汇编确认 handler 读 `Butterfly+4
(state_t)`、派发 XMM0 为死值；当前 Rust 把本地计算的 `t_in` 传入 handler，需改为
`state_t` 语义（dwell 与 state_t 是两个字段）。② state4-7 跳表 handler 未建模（4 为
forwarder）。③ ButterflyAudio 每帧 u32 写（query 派生值）未建模。④ NaN：原生
`!(dist>3)&&!(dist==3)` 对 unordered 视为 near；Rust `<3.0` 为 false（登记差异）。

### 4.13 `sim_wren_boids` @0x142326a80 — **需修**

fresh 反编译确认：顶层门 `byte[*CritterControl+5]`；`dt!=0` 补满 80（x=u*130-65、
y=u*9+1、z=u*130-65）；保留半径 91.92388、y<=30；FreeSpot 池/认领；frame_update 参数
1.3/0.1169/7/14；摇树出生（0.8+u*5.2、1+u*5、(1.01) 半径、向上偏置 0.2）。

Rust 主体已实现；**FIX**：出生门精确条件、viable 向下探针、FreeSpot 认领/关联结构
（HashMap 生命周期）为近似；逐项 U（登记）。

### 4.14 `update_wrens` @0x14231e390 — **需修**

fresh 反编译确认：h0 timer/proximity（阈值 3.0/1.0/0.0，`threshold>dist` 才 H1）、
h3 smoothstep 移动/`progress*7`、state 2/4 扫描无写、音频 set_position。
Rust h0/h3 **判等**（B-ecstate 已反汇编确认）。

**FIX（h1 尾部）**：原生 `qword[record+0x30]=2`（state=2 且 timer 清零）、
`[+0x1c/+0x20/+0x24]=1.0`、音频槽 u32 写、`call 0x140a57300`（载荷）未建模。

### 4.15 `update_sheep_goals` @0x142315050 — **需修**

fresh 反编译确认主流程：behavior==1 → mode3 `eye+fwd*2` 否则 cursor；写 goal；
`timer-=dt`；`dist<0.2 → timer-=3dt`；`timer>=0` → 2 uniform → `(u1*12+4,u2*3+1)`、sub=1；
`timer<0` → state=0、`u*7+1`；共享刷新 keep=`两半>0 && goal.active && dist²>=0.16`；
换目标 `scale=active?0.8:0.5`、`shared[1]=u*3+1`、disk+rotation、demo>35、
`r_new<65 || r_new<r_old`、三次 bilinear 水面门；结尾 `flag_30=sub、shared=(t0,t1)`。

**FIX**：XMM7/XMM8 寄存器残留的结尾写值未确定性恢复（未跟随/未刷新羊的 t0/t1 沿用近似）；
`flag_31@+0x31` 恒 0 死分支未落字段；`0.2` 门的 NaN 语义（native `0.2<=dist` skip，
unordered → 减 3dt；Rust `<0.2` → 不减）。

### 4.16 `sheep_unstuck` @0x1423246b0 — **需修**

fresh 反编译确认：`legacy_spherecast`；命中码 !=2 → 记录 `(1,result)` 并跳过；
否则 rock/water；门 `rock_empty || state>6 || (0x54>>state&1)==0`；
`bucket=(state-2) clamp u32 3`；state2 水>0 → `is_corrupted?0:4`；state2 水<=0 →
`ground_h=max(h,0.5)`、`flag&1 && |(ground_h-rec)/dt| finite && >10 → state=0`；
state6 分支 `pos.y>h → 6`；岩石且 state∈{2,4,6} → state=0 清记录；尾部 state!=2 &&
`flag_30==0` → 清 shared timers。

**FIX**：① 跳过条件应为 **`hitcode != 2`**，Rust `!sphere.hit`（命中码 3/7 语义不同）。
② `rock_present` 恒 false、`save_is_corrupted` 恒 false（宿主资源未接线）。
③ `legacy_spherecast` ABI / rock 逐点采样近似（登记）。

### 4.17 `react_to_being_inside_buildings` @0x14231f610 — **需修**

原生：墙空间哈希逐墙 `PublicWalls::get`（`count>1 || water@+0x280>1.5` 门）→
`max_y` + `roof_entity` → `Roof::height` → `GladeBorder::is_pos_inside` → 羊在
`[墙高, 墙高+屋顶]` → rng 拒绝采样 emoji 索引（`(idx*7)<4` 重试）→ 表
DAT_14304b8c8/b8e8 → decorator 记录 → `spawn<(Ui2dExtent,Ui2dImage,SheepReactionEmoji)>`。

Rust 仅保留 `walls.all()+2D 射线+roof.height` 与简化的 emoji 实体；**FIX**：墙组查询/
过滤、GladeBorder 门、rng 拒绝采样、emoji 表与 UI bundle 均未还原。

### 4.18 `sheep_reaction_emoji` @0x14231ff70 — **需修**

fresh 反编译：state2 fade t=age/0.4 → ease → scale*=ease，t==1&&!NaN → despawn；
state1 `age>=1.0` → state2、age=0；state0 t=age/0.2 → ease → scale*=ease，t==1 → state1、age=0；
每帧 `WindowSize::as_vec2` + `world_space_to_screen_space` 写屏幕位。
Rust 三态转移/阈值逐条一致；**FIX**：屏幕投影写面缺失（`*puVar16 = screen`）；
EasingFunction 变体未取证（U）。

### 4.19 `sheep_floaties` @0x142320cf0 — **判等**

fresh 反编译确认：holder `+0x14==0` → spawn bundle（mesh/shader 来自 holder，transform=羊）；
配件整份 Transform 复制；**双向音频**（state4&&!started、!state4&&started 各播
`event:/SFX/Sheep/UmbrellaClose` + set_position + start_if_not_playing）；
`progress=clamp(progress±2*dt,0,1)`；`BackOut(progress)*羊 scale.x` 三轴；visible=state4。
Rust 逐条一致。登记 I：DrawableMeshBundle/mesh/shader 由宿主注入。

### 4.20 `sheep_umbrellas` @0x142321980 — **判等**

fresh 反编译 + 反汇编（`evidence/disasm-place-ui-umbrella.json`）：
`mask = (0x1a >> clamp_u32(state-2,..,3)) & 1`（`ADD ECX,-2; CMP ECX,5; CMOVNC`
= **无符号**，state 0/1 回绕→3，与 Rust `wrapping_sub` 一致）；双向音频；`progress±2dt`；
`BackOut*scale.x`；visible=mask。Rust 逐条一致。登记 I：bundle 句柄宿主注入。

### 4.21 `sheep_input_record_replay` @0x14231b720 — **需修**

fresh 反编译：会话档 = `resource[+0x40] ^ 0x8000…` 取 min 档；`<2` → 记录
（两 `from_iter` 收集全羊 position/yaw → `InputStream::last_frame_mut`）；
否则 `档-3>1`（无符号）且输入门（`+0xfb` 或 `*in_R8==1`）→ 回放逐羊写
translation + `from_euler_angles` 旋转。

Rust 数据流（positions/yaws 帧）一致；**FIX**：会话门以 `SheepInputReplayCmd` 事件替代
resource+0x40 与 `+0xfb` 输入字节（身份未取证，DIVERGE）；`from_euler_angles` 的
Euler 变体/输出轴需按寄存器闭合（当前 `XYZ(0,yaw,0)`，U）。

### 4.22 `ui_pet_sheep` @0x142324f80 — **未实现**

原生：UI 状态资源 gate（0x13+mode3 / 0xd 跳过 / 6+子字节）；`format!("pet_sheep{id}")` +
WidgetParams/UiSounds/no_history_recording + `UiState::add` + `intersecting/is_active` +
`ws_to_cs` + SignifierFactory + write；`into_drag_state`：state3 → `[+0x34]>0.5` 音频；
state4 → 拖拽 signifier + 相机 mode3 → 宠物事件 + **写 Sheep+8=1、+0xc=14.0**。
Rust 仅保留 eye 距离门（`dist>5.0` 跳过）与 target 记录，UI/drag/audio/宠物写入全部缺失 →
**未实现**（部分骨架）。

### 4.23 `ui_pet_ducks` @0x142322e30 — **未实现**

原生：① pet timer `Duck+0xf8` 按 `±dt/0.4` clamp[0,1]，`frozen && >=1.0` → 释放；
② UI gate/`format!("pet_duck{entity}")`/WidgetParams/UiState/intersect/is_active/eye>6/
SignifierFactory + closure "pet_hand"/"duck_head" + `LookAtCamera` + `into_drag_state`
（state3 `frozen=1` + 音频；state4 拖拽 signifier）。
Rust 仅实现 ①（逐条判等）；② 全部缺失 → **未实现**。

---

## 5. FIX 清单（按函数，均附分歧点）

| # | 函数 | FIX |
|---|---|---|
| FIX-B1 | `process_tree_cmds` | Insert 补 `is_dragging` 门（拖动 flags 字节 → SpawnPrefabCmd.mode）；`is_corrupted` 接宿主资源；SpawnPrefabCmd `rotation/variant` 取事件 `+0x28` |
| FIX-B2 | `replay_tree` | InsertTreeCmd 0x48 重建补 `height/rotation/variant/flags`（依赖 core 类型 T2） |
| FIX-B3 | `tree_move_bend` | 补 `PrefabInstanceData+0x24/+0x28 = {step_x,step_z}` + 变更 tick（依赖 core 放置记录字段） |
| FIX-B4 | `hide_trees` | 按模块文档逐语句重建：Area 圆/矩形分派、NextToWall 墙哈希、可见性 `+0xc`、删除门、History |
| FIX-B5 | `place_tree` | `TREE_ROTATION_MIN` 改 **0.0**（反汇编 lane1×0.0）；InsertTreeCmd 全字段；会话门接线 |
| FIX-B6 | `ui_move_tree` | 候选源改 `iter_removed_ids`；补 prefab 前缀/可移动门与 widget 偏移；case1 UI widget 记录写 |
| FIX-B7 | `ui_delete_tree` | 候选源/位置源改放置记录；History 补 scale/rotation/is_preplaced |
| FIX-B8 | `BoidFlock::remove` | boid 越界改为 `swap_remove` panic 语义（当前 `break`） |
| FIX-B9 | `move_critters` | 墙 relate 形状补 `Circle2d::new_rotated`（当前 segment-vs-segment 近似） |
| FIX-B10 | `critters_avoid_collisions` | 同上墙 relate 形状 |
| FIX-B11 | `DuckSpawnLocations::new` | z 列改字面常量表（`j=2 0xc1e71c73`、`j=3 0xc18aaaa9`，当前差 2 ULP） |
| FIX-B12 | `spawn_ducks` | 补 `is_corrupted` 门；`sorted_by` 比较器 + 反向遍历；水采样跳过时 `used=0` |
| FIX-B13 | `plan_duck_trajectories` / `move_ducks_along_trajectories` | 旋转弧两端向量、shape 采样闭包、set_volume/2×audio 参数、Frenet/姿态面 |
| FIX-B14 | `ducks_react_to_being_inside_buildings` | 墙组查询（+600/+0x280）、get_wall_shape、GladeBorder 门与轮廓 y 比较 |
| FIX-B15 | `update_butterfly_states` | handler t 源改 `Butterfly.state_t(+4)`（派发浮点为死值）；state4-7 handler；audio u32 写；NaN near 语义 |
| FIX-B16 | `sim_wren_boids` | 出生门精确条件、viable 向下探针、FreeSpot 认领/关联结构 |
| FIX-B17 | `update_wrens` | h1 尾部 `[+0x1c/+0x20/+0x24]=1.0`、音频槽 u32、`call 0x140a57300`、timer 清零（qword store 已覆盖） |
| FIX-B18 | `update_sheep_goals` | XMM7/XMM8 结尾写值、`flag_31@+0x31`、0.2 门 NaN 语义 |
| FIX-B19 | `sheep_unstuck` | 跳过条件改 `hitcode != 2`；rock 逐点采样、is_corrupted 接线 |
| FIX-B20 | `react_to_being_inside_buildings` | 墙组查询/GladeBorder、rng 拒绝采样、emoji 表、UI bundle |
| FIX-B21 | `sheep_reaction_emoji` | 补 `WindowSize::as_vec2 + world_space_to_screen_space` 写面；Easing 变体取证 |
| FIX-B22 | `sheep_input_record_replay` | 会话资源 `+0x40/+0xfb` 门身份；Euler 变体闭合 |
| FIX-B23 | `ui_pet_sheep` / `ui_pet_ducks` | 补 UI 命中/signifier/drag/audio；`ui_pet_sheep` 的拖拽 4 分支写 Sheep+8=1/+0xc=14.0 |
| FIX-B24 | `despawn_ducks` | 去掉直接 despawn；离场 plan `key` 写鸭实体 id（当前 0 会被当作出生计划）；补 plan 其余字段 |

## 6. 登记（不改变主判定）

- `CritterGoal` 共享布局（`position@0` vs native `active@0/pos@4`）影响 move/avoid/goals 的
  组件读面（country-core 类型合同，I）。
- `legacy_spherecast` crate helper 的 `slot_query` 为宿主壳（当前返回 None）；`hitcode`
  在 helper 中取槽号而非 native 结果结构 `+0x1c` 的岩石/墙语义（ABI 登记）。
- 各系统的 query 环境值写/变更 tick（bevy 派生值）不建模（DIVERGE，非业务）。
- `TreeAnimation.alpha@+4`、`PrefabInstanceData` 0x30B 放置记录等共享类型布局差异按既有登记。

## 7. 方法与证据索引

- fresh 反编译：`evidence/0x*.c`（35 份；Ghidra shared，`program=tiny-glade.exe`）。
- 常量实读：`evidence/consts.json`（0x142925870=0.5、0x142925874=0x3efc0fc1、
  0x142925984=0.1、0x142925988=5.0、0x1429cd030=0.7、0x142a16cb8=0.8、
  0x142efa130="glade_border"、0x142efa140 piece 表、0x143035140="decorator_" 等）。
- 反汇编：`evidence/disasm-place-tree-closure1.json`、`disasm-move-avoid.json`、
  `disasm-place-ui-umbrella.json`。
- 未改 src；所有 verdict 以 fresh 反编译/反汇编/内存实读为准，未采信 restorer 描述。
