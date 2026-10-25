# reviewer-eco-treetool — system-tree REVIEW 报告

日期：2026-10-17（含 FIX 响应复核轮：2026-10-17，状态 **fix_verified（主体方向）**）
审核对象：`crates/systems/tree-tool`（package `system-tree`）restorer-eco-treetool 的 RESTORE.md
证据（本 reviewer **独立 fresh** 反编译/反汇编/内存直读，`--worker shared` 只读）：
`reverse-engineering/tiny-glade/upper-modules-plan/systems/tree-tool/.work/eco-reviewer-fresh/`
（`dec/REVIEW_*.c` 15 函数 fresh 反编译 + `result2.json` 3 函数反汇编 + `mem0/1/2` 常量/字符串直读）。

## 〇、审核方法（2026-10-16 硬标准）

1. 对 14 个模块函数逐一 **fresh 反编译**（本 reviewer 独立 job，非复用 restorer 输出），关键函数另取反汇编与常量/字符串内存直读；
2. **独立复述** 原函数完整控制流（每分支/循环/调用链/数据流），再与 restorer 落盘 Rust body 逐语句对照；
3. verdict：`判等` / `未实现-需重做(附分歧点)` / `胶水-非业务`。**凡控制流未完整还原到可判等的函数一律标未实现**，不因"部分子逻辑已重建"或"待取证登记"通过。

## 一、verdict 总表

| 函数 | 地址 | verdict | 核心分歧 |
|---|---|---|---|
| process_tree_cmds | 0x142267430 | **未实现-需重做** | 分歧 1/2/18 + Delete 子分支近似 |
| despawn_trees | 0x1422686d0 | **判等** | body 逻辑逐条对应（α@+4 布局属 country-core 分歧 5） |
| convert_preplaced_trees | 0x14226be70 | **未实现-需重做** | **事件方向反转**（新增）+ 分歧 12/13/14 |
| replay_tree | 0x141bc34f0 | **未实现-需重做** | 分歧 1/2/8 + elapsed 门控缺失 |
| recalc_tree_position_on_terrain_height_change | 0x142268dc0 | **判等** | — |
| tree_move_bend | 0x1422689c0 | **未实现-需重做** | 分歧 7（step 写面缺失）；bend 数学本身判等 |
| hide_trees | 0x14226a260 | **未实现-需重做** | 分歧 9/10/11（形状分派/墙哈希/门控字节/可见性写均未落地） |
| random_tree_prefab | 0x14226a170 | **判等** | 门控 + 7 串表逐分支吻合；分歧 4 在调用侧 |
| ui_place_tree | 0x142269170 | **未实现-需重做** | 分歧 3（raycast+geo_src 早退分支缺失）；主体流程判等 |
| place_tree | 0x142269970 | **未实现-需重做** | 分歧 2/3/4；splitmix/scale/rotation/History 判等 |
| ui_move_tree | 0x14226b1a0 | **未实现-需重做** | 分歧 1/15/16（状态机/载荷/候选源均近似） |
| ui_delete_tree | 0x141bd2510 | **未实现-需重做** | 分歧 9/15/17（中心/半径源=PathDrawEvent 被忽略） |
| reset_tree_tool | 内联 | **判等** | run_unsafe 覆写 default 一致 |
| TreePlugin::build | 0x140650520 | **胶水-非业务** | glue-legitimate（系统元组/资源/7 事件注册一致） |

统计：**判等 4 / 未实现-需重做 9 / 胶水 1**。restorer 自报"判等 3、重建 5"偏高；按硬标准，
凡存在未还原控制流分支/未实现写面的"重建(部分)"一律归**未实现-需重做**。

## 二、restorer 关键主张独立核验（全部通过）

以下主张本 reviewer 用独立反汇编/内存直读**逐项证实为真**：

- **PlaceTreeCmd = {position Vec3}（槽 stride 0x18）**：写侧 ui_place_tree
  `slot+8=CONCAT44(重映射y, x)`、`slot+0x10=z`；读侧 place_tree `uVar14=slot+8`、`uVar6=slot+0x10`。✓
- **MoveTreeCmd 槽 0x20 = {id@+0, index u64@+8, position Vec3@+0x10}**：process_tree_cmds Move 分支
  `local_268=slot+8`、`uVar3=slot+0x10`、`uVar16=slot+0x18`；ui_move_tree case2/3 写 `slot+8=index、slot+0x10=x、+0x14=y、+0x18=z`。✓
- **InsertTreeCmd 槽 0x48**（name String@+0、id@+0x38、position/scale/rotation/variant/flags 中段）：place_tree 逐槽写 + process_tree_cmds/replay_tree 逐槽读。✓
- **DeleteTreeCmd 槽 0x10（id@+8）**：replay_tree/hide_trees/convert 均 `slot+8=tree id`。✓
- **tree_move_bend lerp rate = 5.0（0x142925988=0x40a00000），非 3.22**：反汇编
  `142268a4f MOVSS XMM6,[0x142925988]` 兼作 `142268d14 MINSS XMM3,XMM6` 位移上限；
  0x14292598c=3.22（0x404e147b）确属其它调用点。✓
- **place_tree scale=lerp(0.5,0.9)、rotation=r*360、变体=(u32)v*10 高 32**：
  `fVar28=r*0.9+(1-r)*0.5`、`fVar29=r*360.0+(1-r)*0.0`、`uVar23=(uVar25&0xffffffff)*10>>0x20`。✓
- **despawn 严格 `<0.01`**（`<=0.01 && !=0.01`）；**recalc 边界 `<=65`（含）**。✓
- **random_tree_prefab 7 项路径表 0x1430351d0 区直读**：branchy2/branchy1(22)、
  autumn/branchy2/branchy1(29)、naked1/naked2(26)、naked1_no_mushrooms(39)，
  字节与 restorer 表逐串吻合。✓
- cargo check -p system-tree **通过**（country-core 13 个既有警告非本组）；ivy_spawner.rs:203
  `pub(crate)` 可见性修复在库中已存在。✓

## 三、逐函数独立复述 → Rust 对照 → verdict

### 1. process_tree_cmds @0x142267430 — 未实现-需重做
**原逻辑复述**：处理顺序 **Move(stride 0x20) → Insert(stride 0x48) → Delete(stride 0x10)**。
- Move：逐事件读 index@+8、position@+0x10；`TreeStorage::get(index)`；None 记日志
  `"MoveTreeCmd references a non-existing tree"`；Some 按 {prefab 下标 u32, entity u32} 定位
  `PrefabInstanceData` 写新位置 Vec3；`inverse_lerp`+`bilinear_sample` 判入水；发
  `PlayTreeSoundCmd`（kind=1，slot+9=入水布尔、slot+0xc=inverse_lerp 值）。
- Insert：`ActiveWidget::is_dragging` 门控 + prefab 名 SIMD 匹配（"MISSING_TREE_PREFAB_NAME" 0x18 等）
  + deny_load/is_corrupted 门控 → `Commands::spawn_empty` 插 `TreeComponent`/`TreeAnimation`（值
  `r*0.75+(1-r)*0.55`）/`TreeMoveBend`/`TreeAmbience`；名=="MISSING_TREE_PREFAB_NAME" → random_tree_prefab
  兜底；发 `SpawnPrefabCmd`（stride 0x50）；`TreeStorage::insert`；水面采样；`PlayTreeSoundCmd`（kind=0）；
  insert 返回高 32 非零 → 日志 + `DespawnPrefabCmd`（stride 0x10）。
- Delete：逐事件 `TreeStorage::remove`；`Entities::contains(entity)` 才继续 → Commands 插
  `TreeMarkedForDeletion`；经 wall/decorator 表读位置发 `PlayTreeSoundCmd`（kind=2）。

**Rust 对照**：Move 分支循环体**为空**（分歧 1）；Insert 分支循环体**为空**（分歧 2）；
Delete 分支：`storage.remove→Some` 后**无条件** `Commands::entity(...).insert(TreeMarkedForDeletion)`，
缺 `Entities::contains` 门控（分歧 18，以 Commands::entity 兜底），且缺真实查询的
`unwrap_failed` panic 路径与 prefab/bitmask 过滤（以 `instances.get(entry.entity)` 近似）。

**verdict：未实现-需重做**（restorer 诚实标记 未实现，认同；Delete 子分支为近似非判等）。

### 2. despawn_trees @0x1422686d0 — 判等
**原逻辑复述**：Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)> 逐实体（两 archetype 分派 =
bevy 查询内部，胶水）；`alpha(+4) <= 0.01 && != 0.01`（严格小于，排除 NaN）→
`DespawnPrefabCmd` 槽 stride 0x10，`slot+8=entity`（非树 index）。

**Rust 对照**：`for (entity, animation, _m) in trees.iter_mut()`，`if animation.alpha < 0.01` 与
`<=0.01 && !=0.01` 对全部输入等价（0.01/NaN 均排除）；`DespawnPrefabCmd{entity:Some(entity),_gap:0}`
payload@+8 正确。bevy 查询/事件缓冲 = 胶水。
**注意**：真实 TreeAnimation 浮点在组件 **+4**，共享型 `alpha@+0`——这是 **country-core 布局分歧 5**
（非本 body 错误），但修复前编译产物读 +0，字节级不等。**判等（body 逻辑），登记布局分歧 5**。

### 3. convert_preplaced_trees @0x14226be70 — 未实现-需重做（新增反转）
**原逻辑复述**：SpawnPreplacedTrees 事件到达 → `Wait{count=1, flag=1}`；无新事件帧 count+1，
`count>2` 清 flag 执行转换。转换：`Decorators::iter_removed_ids` 位图候选树 → wall/decorator 表定位
PrefabInstanceData → 6 路 `bilinear_sample`（4 路径+水面+花园）+ `terrain_rock_mask::sample_at_world_xz`
（FrozenMap<SsboId,BufferState> 取用 + assert 名 len 0x3f）→ 水/岩/花园任一 >0 → **跳过**；
否则 density=clamp(Σ路径,0,1)，density<=0 时 `Circle2d::new`+`WallSpatialHash::query_with_circle_callback`。
**转换（通过全部门禁）** → 组 stride-0x48 `TreeEdit` 记录 → `History::insert_starting_edit`。
**未转换（水/岩/花园/路径/墙任一命中）** → 发 stride-0x10 事件（payload=树 u64，arg2 EventWriter）。

**Rust 对照**：① **事件方向反转（新增分歧，restorer 报告未覆盖）**：Rust 对**通过门禁**的树发
`DeleteTreeCmd{id}`，而原只对**未转换**树发 stride-0x10 事件；通过者应进 History 而非删除。②
`History::insert_starting_edit` 完全缺失（`_history` 未用）。③ Wait 标志位缺失（分歧 12，无 flag
门控，事件持续到达时原会重置 count=1，Rust 不会）。④ 岩石掩码 `sample_at_world_xz(&[],...)` 空 buffer
占位（分歧 13）。⑤ 候选源 = `converted` Query + `storage.iter().find(entity)`，非
`Decorators::iter_removed_ids` 位图（分歧 9/14）。

**verdict：未实现-需重做**（restorer 诚实标记未实现，认同；但需追加"事件方向反转"到分歧清单）。

### 4. replay_tree @0x141bc34f0 — 未实现-需重做
**原逻辑复述**：`front_mut_bucket` 空返；读 ReplayBuffer+0x28（f32 倍速除数）。逐 ReplayCmd
(stride 0x30)：tag@+0==8 才处理；Arc<TreeEdit>@+0x10，disc@+0x10：**2=Delete / 1=Move / 0=Insert**。
仅 `elapsed@+0x18 == 0.0`（非 NaN）时重发：Delete 非 undo（@+0x1c==0）→ DeleteTreeCmd(stride 0x10,
id=TreeEdit+0x18)；Delete undo → 重建 InsertTreeCmd(stride 0x48, name@+0x18, position@+0x38/0x40/0x44,
height@+0x4c, scale@+0x50)；Move 非 undo → position@TreeEdit+0x20/+0x28、index@+0x30；Move undo →
position@+0x14/+0x1c（旧位）→ MoveTreeCmd(stride 0x20)；Insert 非 undo → 重建 InsertTreeCmd；
Insert undo → DeleteTreeCmd。时序：flags@+8 bit0==0 → 记待移除；否则
`delay=cmd@+0xc/speed`（下钳 0.05），`delay<elapsed` → 待移除，否则 `elapsed += GameTime delta`；
移除集逆序 `Vec::remove` + tag switch(0-9) 释放负载（case 8=drop_slow<Arc<TreeEdit>>）。

**Rust 对照**：仅 Delete 非 undo、Place undo 两条 → DeleteTreeCmd 形状正确；但**两条都缺
`elapsed==0.0` 门控**（原只在 elapsed==0 时发）。Move 分支空（分歧 1）；两处 InsertTreeCmd 重建空
（分歧 2）；时序记账/移除集/降序删除全部空（分歧 8，共享 ReplayCommand 无 elapsed/delay/flags 字段）。
另注意原 undo 判定 = ReplayCmd@+0x1c 字节，Rust 读 `cmd.undo`（TreeReplayCmd 字段）——共享结构非原布局。

**verdict：未实现-需重做**。

### 5. recalc_tree_position_on_terrain_height_change @0x142268dc0 — 判等
**原逻辑复述**：EventReader 无新事件 → 早退；逐 PrefabInstanceData（stride 0x30）：先写变更 tick
（bevy `&mut` 胶水）；`ABS(x)<=65.0 && ABS(z)<=65.0`（含边界，NaN 为假）→
`sample_at_world_xz` 取 h；`y!=h || NAN(y) || NAN(h)`（f32 不等含 NaN）→ `y = (h<=0 ? 0 : h)`。

**Rust 对照**：`changed.read().next().is_none()` 早退 ✓；`!(x.abs()<=65 && z.abs()<=65)` continue ✓
（NaN 均跳过）；`data.position.y != h`（Rust f32 不等即含 NaN）与 `!= || NAN() || NAN()` 等价 ✓；
`if h<=0 {0} else {h}` = max(h,0) ✓。

**verdict：判等**。

### 6. tree_move_bend @0x1422689c0 — 未实现-需重做（分歧 7）
**原逻辑复述**：逐 (TreeMoveBend 8B, PrefabInstanceData)：`time_continuous_lerp(bend.x, target.x,
delta, 5.0)`（反汇编 142268cae：XMM3=rate=5.0 来自 0x142925988）、`lerp(bend.y, target.z, delta, 5.0)`；
dist>=0.1（0x142925984）时 `step=min(5.0,dist)*{dx,dz}/dist`，`bend=target+step` 打包 u64 写回；
变更检测：PrefabInstanceData **+0x24**/**+0x28** {step_x,step_z} 与 sx/sz 不等（含 NaN）→ 写 tick +
`pack(sx, sz<<32)` 到 +0x24。time_continuous_lerp 本体 = `a+(b-a)*(1-expf(-delta*rate))`（反汇编
XORPS 取负→MULSS×rate→expf→1-F→差乘→加 a）。

**Rust 对照**：lerp 公式/rate=5.0/step=min(5,dist)/bend 打包 u64 全部判等；`_gap` 位解包正确；
但 **PrefabInstanceData +0x24/+0x28 step 写 + 条件 tick 未实现**（分歧 7：共享型布局只到 0x20，
step 字段缺失）。该写面影响 GPU 变更同步，属真实缺失。

**verdict：未实现-需重做（分歧 7）**；bend 数学部分判等。

### 7. hide_trees @0x14226a260 — 未实现-需重做
**原逻辑复述**：两命令读者 Area(stride 0x18)/NextToWall(stride 0x30)；候选树 =
`Decorators::iter_removed_ids` 位图。Area：`cmd@+0 字节 bit0==0 → Circle2d::with_expanded_diameter +
contains_point`、==1 → `Rectangle2d::with_expanded_size + contains_point`（另有 @+0x1c 半径/位置条件
门控）。NextToWall：`Circle2d::new + WallSpatialHash::query_with_circle_callback`。命中 → 写
**可见性字节 @+0xc=1** + tick；`cmd 门控字节`（Area @+0x24==1 / NextToWall @+0x10!=0）→
DeleteTreeCmd(stride 0x10, 树 id@+8) + `History::add`（TreeEdit Delete disc=2）。

**Rust 对照**：Area 仅实现圆分支（`with_expanded_diameter(0.0)` 参数错，原取命令字段；矩形分支空，
分歧 10）；NextToWall 用命令圈对树位置 `is_point_inside` 近似（原走墙哈希查询，行为不同，分歧 10）；
**可见性字节 @+0xc 写缺失**（分歧 11）；**门控字节未建模——Rust 无条件写 DeleteTreeCmd**（原需
@+0x24==1/@+0x10!=0）；候选源 = storage.iter() 非位图（分歧 9）。

**verdict：未实现-需重做**。

### 8. random_tree_prefab @0x14226a170 — 判等
**原逻辑复述**（反汇编逐分支）：selector=in_CL，bit=selector&1。门控：
- `is_corrupted && bit==0` → **naked1_no_mushrooms**（0x14303526a,39）——deny/hal 无关，两入口同；
- `(deny_load || is_halloween_any)` 且 corrupted==0 且 bit==0 → **naked1**（0x143035236,26）；
- 其余（含 deny/hal&&corrupt&&bit==1）→ `DragState::is_started` 路径：started → autumn/branchy[bit]
  （0x1430351fc/0x143035219,29），否则 branchy[bit]（0x1430351d0/0x1430351e6,22）。
- naked2（0x143035250）在 `BL=selector^1` 约束下**不可达**（编译器保留）；autumn/branchy1 经
  started&&bit==1 **可达**。

**Rust 对照**：`if is_corrupted && bit==0 → NO_MUSHROOMS` ✓；`if (deny||hal) && bit==0 → NAKED[0]` ✓
（corrupt&&bit==0 已被前条拦截）；`table[bit]`（started?autumn:branchy）✓。7 项路径串与内存直读逐串吻合。
**注**：restorer 注释"autumn/branchy1 分支不可达"**错误**（反汇编 0x1ee→0x22b 可达），但代码正确。
**verdict：判等（逻辑）**；分歧 4 在调用侧接线（见 place_tree）。

### 9. ui_place_tree @0x142269170 — 未实现-需重做（分歧 3）
**原逻辑复述**：门禁 `*AppMode==0x10 && DEMO==0`；WidgetParams::new/occlude_by_world/sounds +
`UiState::add("tree_canvas")`；`RaycastWorld::legacy_raycast`，结果判别 !=2 → `geo_src_to_edit_subject`，
返回非 0 → **早退**；`position()`；`GladeBorder::is_pos_inside`：界外 → `DisplayOutOfBorder`（stride 8）
+ clicked 才 `FeedbackInputOutOfBorder`；界内 → is_active 时 SignifierFactory+format 串写 signifier；
clicked → 二次 `position()` → `y<0 → (y/-2.5)*-1.2 + (1-y/-2.5)*0` → `PlaceTreeCmd{position Vec3@+8..}`。

**Rust 对照**：门禁/界内外/DisplayOutOfBorder/FeedbackInputOutOfBorder/水重映射/PlaceTreeCmd 写**逐条
对应**；signifier 以 `UiAppendSignifier::new(0,0,pos,1.0)` 近似（原 format 串面 U）；**raycast+
geo_src_to_edit_subject 早退分支缺失**（分歧 3，以光标可读直接前进，原在编辑主题无效时返）。

**verdict：未实现-需重做（分歧 3）**；主体流程判等。

### 10. place_tree @0x142269970 — 未实现-需重做（分歧 2/3/4）
**原逻辑复述**：消费 PlaceTreeCmd(0x18) → `TreeStorage::next_index` → **splitmix 变体**（
`z=seed+0xa0761d6478bd642f`、`a=z^0xe7037ed1a0b428db`、`mixed=hi^lo(a*z)`、`p=(u32)mixed*10`；
`p32<6` 拒绝 → 首轮 `z=seed+0x40ec3ac8f17ac85e` 后每轮 `z+=STEP`；通过后 **变体=p>>32**）→
`random_tree_prefab(variant, session)` → fastrand×2 → `scale=r1*0.9+(1-r1)*0.5`、`rotation=r2*360` →
`History::add`（TreeEdit Place disc=0）→ InsertTreeCmd(stride 0x48 全字段)。

**Rust 对照**：splitmix 变体/拒绝采样/scale/rotation/History Place **全部判等**；但 ①
InsertTreeCmd 用共享 {id,position,scale,prefab_name}(0x28) 写，缺 rotation/variant/flags/height/index
（分歧 2/3）；② 会话门控 deny_load/corrupted/drag_started **硬编码 false**（分歧 4），运行时永远走
branchy/autumn 路径，不会产出 naked1/naked1_no_mushrooms——与真实会话状态行为不同。

**verdict：未实现-需重做（分歧 2/3/4）**。

### 11. ui_move_tree @0x14226b1a0 — 未实现-需重做
**原逻辑复述**：门禁 `mode_native==0x10（uVar8-6<0xf 且 uVar8-5==0xb）&& DEMO==0`；
`Decorators::iter_removed_ids` 位图候选树 → wall 表定位；PrefabName 前缀匹配判定可移动
（widget 偏移 2.5 vs 1.0）；WidgetParams+4 个 UiSounds+signifier+`UiState::add<String>`；
`position()` 水重映射；`slider3d::into_drag_state` **3 case 状态机**：case1 记起点到 drag 记录；
case2 拖动 target=光标+offset → 高度采样 → 界内 → MoveTreeCmd(0x20 全载荷)，界外 → 2 个 stride-8 事件
（DisplayOutOfBorder+FeedbackInputOutOfBorder）；case3 释放 → 界内 → MoveTreeCmd + `History::add`
（TreeEdit Move disc=1，含旧/新位）。

**Rust 对照**：门禁等价；但状态机以 `Local<TreeMoveCache>` 距离 ≤1.0 探测近似（**非原 case1/2/3**，
分歧 15）；`MoveTreeCmd{_gap:id}` 只发 id 缺 index/position（分歧 1）；候选源=storage.iter()（分歧 9）；
case2 界外只写 DisplayOutOfBorder 即返回，缺 FeedbackInputOutOfBorder（分歧 16）；无 prefab 前缀判定。

**verdict：未实现-需重做**。

### 12. ui_delete_tree @0x141bd2510 — 未实现-需重做（分歧 9/15/17）
**原逻辑复述**：DEMO!=0 → 早退；`*AppMode==9 && sub==6`（Eraser(Tree)）；`clearing_canvas_w_layer` +
`slider3d::into_drag_state`（drag 状态 !=2 才采样光标）；radius=`brush*3.0`；遍历 `PathDrawEvent`
(stride 0x20)：`flag@+0x18==0` → **center=pos@+8/+0x10、radius=2×@+0x14**（替换光标中心/半径）；
`Decorators::iter_removed_ids` 候选树 → 位置 → `sqrt(dx²+dz²)<radius` → DeleteTreeCmd(0x10, 树 id@+8) +
`History::add`（TreeEdit Delete disc=2）。

**Rust 对照**：DEMO+Eraser(Tree) 门禁 ✓；`brush.0*3.0` ✓；距离命中写 DeleteTreeCmd+History Delete 形状 ✓；
但 **PathDrawEvent 被完全忽略**（`_strokes` 未用），中心/半径用光标——真实交互以拖拽路径圆为准
（分歧 17）；候选源 storage.iter() 非位图（分歧 9）；drag 状态机/clearing_canvas 缺失（分歧 15）。

**verdict：未实现-需重做**。

### 13. reset_tree_tool — 判等
**原逻辑复述**：全内联进 run_unsafe(0x141bb19d0)：`TreeStorage::default()` = 清空 hashbrown 头
（map ptr@+0/+8、len@+0x10）+ 写随机 KEYS（hashmap_random_keys）+ next_index=0（+0x30）+ 置变更 tick。

**Rust 对照**：`*storage = TreeStorage::default()`；共享 TreeStorage `#[derive(Default)]` = 空
HashMap + next_index=0，与 run_unsafe 逐字段一致。hashbrown 头/随机键属 std 内部（胶水）。

**verdict：判等**。

### 14. TreePlugin::build @0x140650520 — 胶水-非业务
**原逻辑复述**：`add_systems<OnExit<AppLoadingState>, fn(ResMut<TreeStorage>)>`（reset_tree_tool）；
`add_systems` Update（in_set<GameSystemSet>+chain_ignore_deferred，元组含 replay_tree 等全部 CPU 系统）；
`insert_resource<TreeStorage>()`；register_event ×7：**PlaceTreeCmd / MoveTreeCmd / DeleteTreeCmd /
InsertTreeCmd / SpawnPreplacedTrees / HideTreesinAreaCmd / HideTreesNextToWallCmd**（TypeId 守卫）。

**Rust 对照**：`build` = register+add_systems，7 事件 init 一致 ✓；系统元组齐全（顺序在 bevy
非冲突系统中无语义，元组栈槽顺序不可读）✓；`register_tree_tool_events` **额外 init**
PathDrawEvent/SpawnPrefabCmd/DespawnPrefabCmd/OnTerrainChange——原 build 不注册这 4 个（属其它插件域，
bevy register_event 幂等故无害），建议从本 build 移除或注明为其它插件预注册。

**verdict：胶水-非业务（glue-legitimate）**。

## 四、FIX 清单

按优先级（F=函数级必须重做；T=类型级阻塞；M=次要）：

1. **[F] convert_preplaced_trees**：反转事件方向——**通过门禁 → History::insert_starting_edit**、
   **未转换 → 发事件（payload=树 u64）**；补 Wait 标志位语义；岩石掩码 buffer 接线。
2. **[T] country-core InsertTreeCmd 扩为真实 0x48 载荷**（name String@+0、position/scale/rotation/
   variant/flags/index）：解 process_tree_cmds Insert / place_tree / replay_tree 重建面。
3. **[T] country-core MoveTreeCmd 扩为 {id, index u64@+8, position Vec3@+0x10}**：解 process_tree_cmds
   Move / replay_tree Move / ui_move_tree 载荷。
4. **[T] country-core PrefabInstanceData 补 +0x24/+0x28 step 字段**：解 tree_move_bend 变更写面。
5. **[F] replay_tree**：补 elapsed==0 门控（含已实现的两条 DeleteTreeCmd）、Move 分支、Insert 重建、
   时序记账（delay=cmd/speed 下钳 0.05）与逆序 Vec::remove。
6. **[F] process_tree_cmds**：Move/Insert 分支按真实载荷实现；Delete 分支补 Entities::contains 门控。
7. **[F] ui_delete_tree**：中心/半径改 PathDrawEvent（flag@+0x18==0 → pos@+8/+0x10、radius=2×@+0x14）。
8. **[F] ui_move_tree**：按 slider3d::into_drag_state case1/2/3 重建；MoveTreeCmd 全载荷；case2 界外双事件。
9. **[F] hide_trees**：Area 圆/矩形分派；NextToWall 走 WallSpatialHash 查询；@+0xc 可见性写；门控字节。
10. **[F] place_tree**：InsertTreeCmd 全字段 + 会话门控（deny/corrupted/drag）接线。
11. **[F] ui_place_tree**：补 raycast+geo_src 早退分支（依赖 country-core geo_src_to_edit_subject）。
12. **[M] lib.rs build**：移除非本 build 的 4 个额外事件 init（或注明属其它插件）。
13. **[M] random_tree_prefab 注释**：更正"autumn/branchy1 不可达"（可达）。

## 五、过度断言检查

- 无"全功能已实现"式断言。restorer 对 5 个函数诚实标未实现并附分歧；但其对 convert 的描述
  （"通过者组 stride-0x48 条目并经事件发射"）**未指出事件方向与通过者相反**，且未提 History
  insert_starting_edit 完全缺失——这两点需登记。
- restorer 标"重建(部分)"的 5 个函数（tree_move_bend/ui_place_tree/place_tree/ui_delete_tree/
  random_tree_prefab）按硬标准：前 4 个存在未还原写面/分支 → **归未实现**；random_tree_prefab 逻辑
  确凿判等 → **升判等**。
- cargo check 通过、ivy 可见性修复均核验为真。
- 关键常量/载荷/槽 stride 主张 100% 复核通过，无夸大。

## 六、协作登记

- 本 reviewer 全部独立 fresh 反编译走 `--worker shared` 只读，无 ReVa 脚本/写库；证据落盘
  `.work/eco-reviewer-fresh/`。
- 分歧 5（TreeAnimation α@+4 vs 共享 +0）、分歧 7（PrefabInstanceData step 字段）为 country-core
  类型合同缺口，建议由集成 owner 裁决统一修复后，tree-tool 侧补写面。

## 七、FIX 响应复核（restorer 本轮修改，2026-10-17）

复核依据：本 reviewer 独立反汇编/内存直读（不依赖 restorer 描述），对照更新后源码逐项核对。

### 4 处函数级修复 — 主体方向正确，残留近似已诚实登记

1. **convert_preplaced_trees（事件方向反转）✓**：现为 `blocked（水/岩/花园/路径/墙）→
   DeleteTreeCmd`、`通过 → History（显式未实现占位）`——与 decomp 0x14226be70 行 382-468 逐分支吻合
   （未转换树走 stride-0x10 事件写、通过者组 0x48 记录进 History）。**残留**：Wait 标志位一次性子
   门控仍缺失（分歧 12）——`count` 无事件帧持续自增，`>2` 后**每帧重跑**转换块，原逻辑清 flag 后
   每事件簇只转一次；事件从未到达时 Rust 仍会在 3 帧后空转（原不触发）。
2. **ui_delete_tree（PathDrawEvent 中心/半径）✓**：`strokes.read().last()` → center=事件位置、
   radius=事件 radius×2 与 decomp 行 123-128 方向一致（`flag@+0x18==0 → pos@+8/+0x10、radius=2×@+0x14`）。
   **残留**：`flag@+0x18` 字段缺失用"事件存在"近似（分歧 17）；`bVar30 = drag_state!=2 || 有效路径事件`
   门控未建模——原在既不拖拽又无有效事件时不删，Rust 以光标/brush 兜底仍删（分歧 15）。
3. **lib.rs build（移除 4 个非本 build 事件）✓**：`register_tree_tool_events` 现只注册 7 事件
   （PlaceTreeCmd/MoveTreeCmd/DeleteTreeCmd/InsertTreeCmd/SpawnPreplacedTrees/HideTreesinAreaCmd/
   HideTreesNextToWallCmd），与 build 0x140650520 的 7 个 `register_event`（TypeId 0xb02b…/0x69b9…/
   0x6fc4…/0x1717…/0xfdf3…/0x73b6…/0x8fee…）**逐一对应**。
4. **random_tree_prefab 注释 ✓**：已更正 `started&&bit==1 → autumn/branchy1（0x143035219）可达；
   仅 naked2 不可达`——与反汇编 0x1ee→0x22b 分支吻合。

### 待接线/集成登记 — 准确，代码无过度断言

- 6 项 country-core 集成任务（MoveTreeCmd 0x20 / InsertTreeCmd 0x48 / PrefabInstanceData step /
  ReplayCommand elapsed/delay / TreeAnimation α@+4 / History insert_starting_edit）与本题核验的共享型
  缺口一一对应；待接线函数（process/replay/tree_move_bend/hide/place/ui_place/ui_move）代码均以显式
  `未实现`/占位标注，不再以近似充当已实现。
- **verdict 不变**：判等 4 / 未实现-需重做 9 / 胶水 1。
- cargo check -p system-tree **通过**（0 警告；country-core 13 个既有警告非本组；audio 组并发
  in-flight 阻断已解除，当前可编译）。

### 残留 FIX（非新增，均为已登记分歧，待 country-core 类型面闭合）

- convert Wait 一次性子门控（分歧 12）+ History::insert_starting_edit（分歧 15）。
- ui_delete_tree flag@+0x18 / bVar30 门控（分歧 17/15）。
- 其余函数级 FIX（分歧 1/2/3/4/7/8/9-11/16/18）维持原 FIX 清单，依赖 T 项。

**状态：fix_verified（主体方向）；仍有 FIX（9 函数未实现，依赖 country-core 集成任务）。**
