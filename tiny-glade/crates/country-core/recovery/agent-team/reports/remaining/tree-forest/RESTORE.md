# remaining/tree-forest RESTORE — tree-tool 剩余 + forest 动画/哥特帽骨

- 日期：2026-10-17（restorer-tree-forest-remaining）
- 方法：每函数 **fresh** 反编译（Ghidra `--worker shared` 只读）+ `disassemble_function`
  逐指令复核 + 常量/跳表字节实读；PDB `file_interface_candidates` 取真实系统签名，
  `.pdb.jsonl.gz` 的 line table/inline site 对齐源码行；未复用旧结论作为证据。
- 证据目录（本 agent）：
  `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/recovery/agent-team/work/restorer-tree-forest-remaining/`
  - `evidence/fresh-tree-tool.json` + `0x14226a260_hide_trees.c` / `0x142269970_place_tree.c` /
    `0x142269170_ui_place_tree.c` / `0x141bd2510_ui_delete_tree.c` / `0x1422689c0_tree_move_bend.c`
  - `evidence/fresh-forest.json` + `0x142279500_handplaced_tree_animation.c` /
    `0x14227a0d0_forest_tree_animation.c` / `0x1407eb500_get_gothic_hat_bones.c`
  - `evidence/disasm_{ui_place_tree,place_tree,ui_delete_tree,tree_move_bend,get_gothic_hat_bones}.txt`
    （Ghidra 全函数反汇编；handplaced 575 指令全量见 `evidence/handplaced_hp_step0.txt`）
  - `evidence/fresh-callees.json`（iter/position/Circle2d/Rectangle2d/time_continuous_lerp）、
    `fresh-*consts.json`（跳表/常量/字符串实读）
- 红线：country-core 只读（未改）；decorator crate 属他组，未改；只动
  `crates/systems/tree-tool/` 与 `crates/systems/forest/`。

## 结论总表

| 模块 | 函数 / VA | verdict | 剩余 / 分歧 |
|---|---|---|---|
| tree-tool | hide_trees 0x14226a260 | **未实现**（core-type-gap） | 两命令 shape/距离/删除门字节、可见性数组；候选源已更正为 `TreeStorage::iter()` |
| tree-tool | place_tree 0x142269970 | **重建** | InsertTreeCmd 0x48 的 variant/rotation/flags/height 无承载；random_tree_prefab 4 门控资源读取面未接线；rotation 下限已修 |
| tree-tool | ui_place_tree 0x142269170 | **重建** | widget volume size.y/record 所有权/唤醒 U；其余逐语句判等 |
| tree-tool | ui_delete_tree 0x141bd2510 | **重建** | 位置源 core 投影（TreeStorageEntry.position）；History scale/rotation 无承载；flow/事件/半径判等 |
| tree-tool | tree_move_bend 0x1422689c0 | **未实现**（近似体已登记） | `PrefabInstanceData@0` 位置与 `+0x24/+0x28` step 写面缺 core 字段；`time_continuous_lerp` 判等 |
| forest | handplaced_tree_animation 0x142279500 | **重建**（判等面 + 2 core gap） | `TreeAnimation+4` 进度以 Local 承载；`PrefabInstanceData+0x20` 打包写面未落 |
| forest | forest_tree_animation 0x14227a0d0 | **判等**（+2 缺口登记） | `PrefabInstanceData+0xc` 缓动写面；放置记录 `+0x44` 清标志/越界 panic |
| forest(实际 decorator) | get_gothic_hat_bones 0x1407eb500 | **未实现-需修**（他模块写权） | 现有实现 side/edge 两处公式与机器码不符（见 §8）；本报告给出精确 FIX |

三态计数：**判等 1 / 重建 4 / 未实现 3**（hide_trees、tree_move_bend、get_gothic_hat_bones）。

`cargo check`（2026-10-17 23:2x，workspace 根）：
- `cargo check -p system-tree -p forest` → **Finished，0 error**（两 crate 自身 0 warning；
  country-core 14 条既有 warning 非本批引入）
- `cargo test -p system-tree -p forest --lib` → **ok（0 test，编译通过）**

---

## 0. 关键纠错（推翻旧报告）

1. **候选源不是 `Decorators::iter_removed_ids`。**
   0x140a1b100 是 ICF 折叠符号，别名集合 =
   `TreeStorage::iter` / `DecoratorArchivist::archived_walls` /
   `DecoratorStorage::iter_walls` / `Decorators::iter_removed_ids` / `PublicWalls::wall_ids`
   （`tree-tool/hide_trees.rs.dependencies.json`）。hide_trees 的调用实参是
   `RDX = *Res<TreeStorage>`（0x14226a340/0x14226a42a）、ui_delete_tree 同
   （0x141bd26f7 `MOV RDX,[RAX]`，arg6=Res\<TreeStorage\>）→ **两处都是
   `TreeStorage::iter()`**，旧报告 "Decorators 资源缺口" 不成立。
2. **hide_trees 两事件 stride**：Area = **0x30**（shape bit@payload+0、f32@+0x1c、
   删除门@+0x24）、NextToWall = **0x18**（删除门@payload+8）。与旧 hide_trees.rs 注释
   相反（旧文档写 Area 0x18 / NextToWall 0x30）。
3. **place_tree 的 selector** 是 `next_index()` 的**低字节**（调用点 RCX=seed，
   `random_tree_prefab` 内只用 `&1`），不是 splitmix 变体；旧实现传 `variant` 错。
4. **place_tree rotation 下限 = 0.49220002**（0x142925874，位型 0x3efc0fc1），
   不是 0.0；常量对 0x143035090 = {0.9, 360.0}、0x142925870 = {0.5, 0.4922}。
5. **handplaced 音频盒扩展** = min+(-2.5,-1.0,-2.5) / max+(2.5,1.0,2.5)
   （常量 0x143037e10={-1.5,-1.0,-1.5,1.5}、0x142ab2370={1.0,1.5}、0x14292e6c0={-1.0,1.0}、
   0x142925904=1.0 的 lane 组合），旧实现 (-1.5,-1.0,-1.5)/(1.0,1.5,0.0) 错。
6. **handplaced 全局状态**在 native 是第 7 参 `ResMut<TreeCanopyMaxCullDistance>` 的
   **值结构** `{progress@0, smoothstep@4}`（handler 写 `*[arg]+0/+4`、tick 写
   `+0x10/+0x1c`），core 该资源仅 `f32` → Local 承载是类型缺口，不是设计选择。
7. **forest_tree_animation 的 DespawnPrefabCmd 载荷只有 8 B（打包 Entity）**；
   core 类型多出的 `_gap` 原生不写 → 本波改为写 0（旧实现伪造 `tree.id`）。

---

## 1. hide_trees（0x14226a260）— 未实现（core-type-gap）

**签名（PDB）**：`Query<(&mut TreeAnimation,&PrefabInstanceData,&PrefabInstance,&TreeComponent)>,
Res<TreeStorage>, EventReader<HideTreesinAreaCmd>, EventReader<HideTreesNextToWallCmd>,
Res<WallSpatialHash>, EventWriter<DeleteTreeCmd>, ResMut<History>`。

**NextToWall 分支（事件 stride 0x18；0x14226a3ce-0x14226a8b7）**
1. `TreeStorage::iter()`；空 → 结束。
2. 条目 → entity/放置记录（Query 0x30 B 列）→ `PlacementDataSsbo::position()`（0x140ad51a0，
   复制 12 B）。
3. `Circle2d::new(pos.x, pos.z, 0.4)`（0x140c97540；XMM3=0.4）。
4. `WallSpatialHash::query_with_circle_callback`（0x1425df320）：遍历圆 AABB 覆盖的网格单元，
   对每墙段（0x38 stride）算圆心到线段最近距离，`<0.4` 时把闭包 flag 清 0 并 return。
5. flag==0 → 写 `u32[vis+id*4] = tick`、`byte[record+id*0x10+0xc] = 1`；
   事件 `byte@+0x10`（payload+8）!=0 → `DeleteTreeCmd{id}` + `History::add`
   （TreeEdit Delete 0x58 记录：disc=2@+0x10、id@+0x18、name String@+0x20、
   position/scale/rotation@+0x38..、is_preplaced byte@+0x50）。

**Area 分支（事件 stride 0x30；0x14226a8bc-0x14226aedc）**
1. 候选同 `TreeStorage::iter()` + position。
2. `f32@+0x1c` 距离/寿命门：`value <= 0` → 跳过；否则 `value - delta <= 4.0` 才继续
   （delta = GameTime）。
3. `*record & 1`（payload bit0）：0 → `Circle2d::with_expanded_diameter`（0x140c980c0，
   `radius += 0.5*delta@+0x20`）+ `contains_point`（0x140c980f0）；1 →
   `Rectangle2d::with_expanded_size`（0x140c9a730，size += delta@+0x20，两 lane）+
   `contains_point`（0x140c9a630）。
4. 命中 → 同一可见性写；删除门 `byte@+0x24`==1。

**未实现原因**：core `HideTreesinAreaCmd`/`HideTreesNextToWallCmd` 仅 `{center,radius}`；
缺 shape bit、`f32@+0x1c`、`delta@+0x20`、删除门字节；可见性数组/索引也未建模。
本波把旧"近似体"（用 `storage.iter()` + 圆包含 + 无条件删除）改为**诚实空体**，
签名保持与 PDB 一致；解锁条件写入模块文档。

## 2. place_tree（0x142269970）— 重建

**逐语句（0x142269af0-0x142269e99）**
1. `TreeStorage::next_index()`（0x140a38600）→ id/seed。
2. `fastrand::Rng::f32`（0x140caf380）→ r1。
3. splitmix64（seed=id）：`z=seed+0xa0761d6478bd642f`；`a=z^0xe7037ed1a0b428db`；
   `mixed=hi^lo(a*z)`；`p=(u32)mixed*10`；`(u32)p<6` 重采（首轮 seed+0x40ec3ac8f17a_c85e，
   其后 z+=STEP）；接受后 **variant = p>>32**。Rust `tree_variant` 逐常量判等。
4. 事件位置：payload `{x@+8, y@+0xc, z@+0x10}`（EventInstance 8 B 头）。
5. `fastrand::Rng::f32` → r2。
6. `random_tree_prefab(selector=id 低字节, …)`；scale=`r1*0.9+(1-r1)*0.5`；
   rotation=`r2*360+(1-r2)*0.49220002`。
7. InsertTreeCmd 0x48 槽：
   `{name@+0, position@+0x10, scale@+0x20, {variant,rotation}@+0x28, flags@+0x30,
   id@+0x38, height@+0x40}`（0x142269d32-0x142269ddb 逐字段）。
8. `History::add`（0x140a84ec0）：Arc 0x58 TreeEdit Place（disc 0@+0x10、id@+0x18、
   name String、position/scale/rotation、tail byte）。

**已落库**：id/position/scale/rotation/prefab 与 History Place 全部按原值；selector 修正。
**未实现（登记）**：① InsertTreeCmd 缺 variant/rotation/flags/height（core 仅
`{id,position,scale,prefab_name}`）；② `random_tree_prefab` 的 deny_load /
is_corrupted / drag_started 在 native 是 `SessionLoadReadiness::deny_load()` /
`BackupSaveFileKind::is_corrupted()` / `DragState::is_started()` 宿主读取；本 crate
仅接线 halloween（`ActiveSession.meta.is_halloween_any()`），其余传 false（签名 I）。
③ History `is_preplaced`：native tail byte 来源 U（放置树语义应为 false，保留 false）。

## 3. ui_place_tree（0x142269170）— 重建

**逐语句**
1. 门禁：`AppMode.native_tag()==0x10 && DEMO==0`（0x1422691ac-0x1422691c2）。
2. `WidgetParams::new(volume, extra)`：volume 只写 `tag@0=5`、`f32@4=f32::MAX`
   （0x14226920f/216；其余 lane 未初始化 → U）；`extra_78 = [0,0]`（0x14226921a）。
3. `WidgetParams::occlude_by_world(false)`（0x14226923c `XOR R8D,R8D`；旧实现传 true 错）。
4. `UiSounds::empty()`（0x1409a8fb0）→ `.sounds(empty)`（0x1409a8e80）。
5. `UiState::add("tree_canvas")`（0x1425dedb0；字面量 0x143035108 len 0xb）。
6. `RaycastWorld::legacy_raycast`（0x1408cc680）；结果 +0x30 == 2 → miss；
   非 miss → `geo_src_to_edit_subject`（0x140977f50），`!=0` 早退
   （0x1422692e9 `CMP dword [rbp+0x178],0`）。
7. `CursorTerrainRaycast::position()`（0x140a12650）；`GladeBorder::is_pos_inside`
   （0x140ad2950）。
8. 界外：写 `DisplayOutOfBorder`（1 元素 8 B 槽）；`clicked()` 为真再写
   `FeedbackInputOutOfBorder`。
9. 界内：`is_active()` → 指示符：
   - label = `format!("decorator_{}", CursorPositionSS)`（字面量 "decorator_"
     @0x143035140，pieces 表 @0x143035150；1 个 display 参数）；
   - 20 B 记录（0x1422693f9-0x142269431）：`{1u64,1u64, ss.x@+0x10, ss.y@+0x14,
     WindowSize::as_vec2().y @+0x18, GameConfig.ui.icon_size_mult @+0x1c}`；
   - `SignifierFactory::new(label, record, 0x143035160)`（0x14089d8b0）→
     `UiSignifierStream::write`（0x140b2cf90，逐 5 qword）。
   本波按此落地（core `SignifierFactory::new` + `write_factory`）；所有权以
   `Box::leak` 保持载荷存活（原生把记录所有权移交流；core raw 面无 drop 面 → U）。
10. `clicked()` → `position()`（第二次采样）→ `y<0` 水下重映射
    `(y/-2.5)*-1.2+(1-y/-2.5)*0.0`（0x1422694d1-0x1422694fd）→
    `PlaceTreeCmd{position}`（事件 0x18 槽：`[+8]={x,y}`、`[+0x10]=z`）。

**残余 U**：widget volume 的 `size.y`（原生只写 lane 0）、`extra_78` 语义、signifier
记录所有权/唤醒（vtable 0x143035160 指向 rdata 零区，语义 U）、`legacy_raycast`
非 native 物理壳 seam（`toi bits==2 || is_miss`，登记 I）。

## 4. ui_delete_tree（0x141bd2510）— 重建

**逐语句**
1. `DEMO != 0 → return`；`AppMode == Eraser(Tree)`（tag 9、子字节 6）。
2. `UiState::clearing_canvas_w_layer_and_sound("tree_erase", 0x0a, 3)`
   （0x14092e340；name 实读 0x142ef5fe9 = `"tree_erase"` len 10；R9B=0x0a、
   栈 byte=3）→ `slider3d::into_drag_state`（0x14092df10）：
   `state != 2`（未拖动）才 `CursorTerrainRaycast::position()` 作初值。
3. `radius = EraserSizeMultiplier * 3.0`（0x1429cd078）。
4. `PathDrawEvent`（stride 0x20、payload 0x18）：`observed_flag`（事件 +0x18）
   **第一个** == false 的事件：center=`position.xz`（+8/+0x10）、
   `radius = 2*radius`（+0x14），置 active；否则保留光标中心。
5. `TreeStorage::iter()`（0x141bd26f7-0x141bd26fe）逐条：
   `sqrt(dx²+dz²) < radius`（0x141bd280?) → `DeleteTreeCmd{id}`（事件 0x10 槽）
   + `History::add`（Arc 0x58 TreeEdit Delete：disc=2、id、name、position 等）。

**已落库**：门禁、clearing canvas/drag state、首个 flag 事件覆盖、半径、
距离判据、DeleteTreeCmd、History Delete。
**登记缺口**：① 位置源：native 取 Query 的 0x30 B 放置记录 `+0/+8`；core
`PrefabInstanceData` 是 0x58 Vec 布局 → 用 `TreeStorageEntry.position` 投影
（去掉了旧实现的额外 `&Transform` 查询参数，签名回到 PDB 面）；② History
scale/rotation 在 native 来自放置记录，core 无字段 → 默认 0；③ prefab 名取
`PrefabInstance.name`（native 取自记录内 Arc，语义等价，登记 I）。

## 5. tree_move_bend（0x1422689c0）— 未实现（近似体已登记）

**签名（PDB）**：`Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>`。
- `TreeMoveBend` = 8 B `{x@0, y@+4}`；`PrefabInstanceData` = 0x30 B，
  `position@0`、`step@+0x24/+0x28`。
- 逐实体（0x142268c86-0x142268d3c）：
  `bx=time_continuous_lerp(bx, target.x, delta, 5.0)`；
  `by=time_continuous_lerp(by, target.z, delta, 5.0)`；
  `dist=sqrt((bx-target.x)²+(by-target.z)²)`；`dist<0.1` 保留 lerp；否则
  `step=min(5.0,dist)·{dx,dz}/dist`、`bend=target+step`。
- 变更检测（0x142268d47-0x142268d68）：与 `+0x24/+0x28` 比（NaN 语义），
  变化则写 `{step_x,step_z}` 并打 tick。
- `time_continuous_lerp`（0x140c904e0）逐指令判等：`a+(b-a)*(1-expf(-delta·rate))`；
  rate 0x142925988=5.0、阈值 0x142925984=0.1、步长上限同为 5.0。

**未实现原因**：core `PrefabInstanceData` 无 `position@0` / `step@+0x24/+0x28`。
本实现以 `Transform.translation` 投影目标位置、不写 step；verdict 明确登记为
**未实现（近似体）**。解锁：core 落 native 0x30 B 放置记录（或补字段）。

## 6. handplaced_tree_animation（0x142279500）— 重建（判等面 + 2 core gap）

**全函数 575 指令逐段核对**（`evidence/handplaced_hp_step0.txt`）
- AppMode 跳表 0x143038050（16 项）：
  {6,7,8,11,12,15,17,18,19}→0x142279812（加）、{9,10,13,14,16,20}→0x142279848（减）、
  index0→0x142279837（tag 0/3 减、其余加）。Rust `subtract` 集合判等。
- 加/减 handler：`*progress ± delta`，写 `[state+0x10]=tick`、`[state+4]` 稍后写 smoothstep。
- 公共段：`MAXSS 0 / MINSS 1.0` → `progress`；`t²(3-2t)`（3.0 @0x1429cd078）→
  `state+4`。`f32::clamp` 与 MAXSS/MINSS 在 NaN 上同为 NaN 传播，判等。
- 相机：`eye_position`(0x140c96d40)/`eye_direction`(0x140c96d80)；端点
  `eye + dir*4.5`（3.0×1.5）。判等。
- 音频盒：记录 stride 0x1c、`tag@+0x18`（bit0 置位跳过、==0 提前结束）、
  AABB 扩展 **min+(-2.5,-1.0,-2.5)/max+(2.5,1.0,2.5)**、`Aabb3::intersect_segment`
  (0x141240570)。本波已修常量；`tag` 过滤 core `boxes()` 无位 → U。
- 逐树：`step = marked||flag@+0xc ? delta/-0.4 : delta/[anim+0]`；
  `progress=clamp(+step)`；`at_max=(progress==1.0)`（SETZ/SETNP）；
  淡出：`!(subtract & !hit) & at_max → fade+=delta(min1)`，否则 `fade-=delta(clamp01)`；
  变化时 `pack_2x16f_uint(progress, 1-(1-fade)²)` 写 `PrefabInstanceData+0x20`。
  逐分支判等。
**core gap**：① `TreeAnimation+4` 进度位（以 `Local<Vec<f32>>` 承载，登记 I）；
② `PrefabInstanceData+0x20` 写面（core 无字段，未实现）。

## 7. forest_tree_animation（0x14227a0d0）— 判等（+2 缺口）

**逐语句**
- 行 stride 0x18：`amplitude@+8`、`progress@+0xc`、`timer@+0x10`；marked 由
  Option 存在位。
- 标记树：`step=delta/-0.3`、growing=false；否则 timer>0 → 打 tick、`timer-=delta`、
  step=0、growing=true；timer≤0 → `step=2*delta`。
- `progress += step`；**分支钳制** `0<=x?x:0`、`r<=1?r:1`（NaN→0）。本波按分支复刻。
- 进度变化（含 NaN）：`EasingFunction::CubicOut.apply(progress) * amplitude` →
  `PrefabInstanceData+0xc`；同时打 PrefabInstanceData tick。
- `!growing && progress==0.0` → `DespawnPrefabCmd`（payload = 打包 Entity，8 B；
  事件 0x10 槽）+ `placements.records[id].+0x44 = 0`（越界 `panic_bounds_check`）。
**判等面**：算法/常量/缓动变体（0x14303802c=0x06=CubicOut）逐语句。
**缺口**：① `PrefabInstanceData+0xc` 无 core 承载位（计算保留、写面未落）；
② 放置记录 `+0x44`/0x50 stride/越界 panic 未建模；DespawnPrefabCmd 载荷形状
（core `Option<Entity> + _gap`）登记。

## 8. get_gothic_hat_bones（0x1407eb500）— 未实现-需修（他模块写权）

> 归属：`system_decorator::decorator_visual::gothic_wall_window`（inventory 明确）。
> 按红线"不改他模块"，本波**未改 decorator src**；以下为 fresh 复核结论，供
> decorator owner 落地。

**已判等**：rank gate（+0x44 ∈ {1,2,3} 否则 panic；`wall+0x258==0` 时先
`inverse_lerp(0.95,1.6,wall+0x264)` 再 gate）；`project_clamp(u∓half)` 两路、
中心点不钳制；三点 `get_approx_u_from_pos.clamp01` → `get_coord_at_u`；
段索引 `+1` 越界 `panic_bounds_check`；`first/center/second` 三 Vec3 写面
（+0x10/+0x1c/+0x28）；half 调整表（rank1 -0.05；rank2 `-(1-clamp01(t))*0.21-0.1`；
rank3 `-(1-clamp01(t))*0.158-0.15`；flag!=0 时 rank1/2 -0.05、rank3 -0.1）；
`half*=0.5`。

**需修 2 处（现有实现与机器码不符）**：
1. `side[4]`（`[RSI]` 16 B）应为
   `[-dy1*inv1, dx1*inv1, -dy3*inv3, dx3*inv3]`
   （0x1407eb8dd-0x1407eb8fd：SHUFPS 0x55/0xd4/0x20/0x82 + MULPS；即段 1、段 3
   单位切向分量）。现有 Rust `[-dy1*inv1, 0.0, -dy1*dx3, inv3²]` 错。
2. `edge[2]`（`[RSI+0x34]` 8 B）应为 `[-dy2*inv2, dx2*inv2]`（段 2 单位切向；
   0x1407eb901-0x1407eb90e：MOVSLDUP XMM8(1/len2) × UNPCKLPS(XMM13{-dy2,-dy2},XMM7{dx2,dy2})）。
   现有 Rust `[-dy2*inv2, b2.x*inv2]` 用 `b2.x` 而非 `dx2`，错。

其余（`GothicHatWallView`/`GothicHatParams`/`GothicHatBones` 布局、调用点）与
机器码一致。解锁：decorator owner 应用上述两行 FIX（不涉及 core/类型变更）。

---

## 改动文件（绝对路径）

- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/tree-tool/src/hide_trees.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/tree-tool/src/tree_move_bend.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/tree-tool/src/ui_delete_tree.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/tree-tool/src/ui_place_tree.rs`
- `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/systems/forest/src/tree_animation.rs`

未改：country-core（只读）、decorator crate（他模块）、process_tree_cmds / replay_tree /
ui_move_tree / convert_preplaced_trees（他路）。

## 三态回执

- **完成**：place_tree（重建，selector/rotation 修正）、ui_place_tree（重建，指示符
  5-qword 面落地）、ui_delete_tree（重建，事件/半径/History 面）、
  handplaced_tree_animation（重建，音频盒常量修正 + 分支复刻）、
  forest_tree_animation（判等，NaN 钳制与载荷修正）。`cargo check -p system-tree
  -p forest` 均 0 error。
- **登记（未实现）**：hide_trees（命令 shape/门控/可见性数组 core gap；
  候选源更正为 TreeStorage::iter）、tree_move_bend（PrefabInstanceData 0x30
  位置/step 字段 core gap）、get_gothic_hat_bones（decorator 模块写权 + side/edge
  两处 FIX 已给出）。另登记 handplaced/forest 的 core 写面（+0x20/+0xc/+0x44）。
- **阻塞**：无（country-core 与 decorator 均未改动；全部以登记收敛）。
