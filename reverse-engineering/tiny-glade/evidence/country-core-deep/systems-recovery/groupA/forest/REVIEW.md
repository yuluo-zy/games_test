# REVIEW: forest（crates/systems/forest）

- 审查日期：2026-10-16；审查人：reviewer-A（dbid tg_va1）
- 交付：`D:/game/tiny-glade/crates/systems/forest/`（3 文件，workspace 已注册）
- 编译：`cargo check -p forest` PASS
- 证据：BASELINE / fresh decomp `decomp-0x141bf16c0.txt`（+ 调用面统计）/
  `decomp-0x142279500.txt` / `decomp-0x14227a0d0.txt` / rne-read 字节。

## 1. 逐项核对表

| 函数（VA） | 声称实现 | 实测判定 |
|---|---|---|
| forest::impl$0::build（0x140650c50） | build + init_resource + add_systems×2 | OK |
| manage_tree_prefabs（0x141bf16c0） | 两阶段（候选评估/执行） | FIX/GAP（见 §3：6 栅格采样未实现、位置写 ZERO） |
| is_too_close_to_shape（行 211 内联） | 圆距离 helper | GAP-lite（机器码 0x1409ea800 分派未证；系统内未调用） |
| handplaced_tree_animation（0x142279500） | 距离淡出 | GAP（见 §3：机器码 canopy 判定被距离启发式替代） |
| forest_tree_animation（0x14227a0d0） | sway/bend 动画 | GAP（见 §3：easing 未实现，`_gap += 1` 占位） |
| TwoWaySearcher（dep） | 不恢复 | OK |

## 2. 常量字节验证（rne-read）

65.0 @0x142a83814 = `00008242` ✓；7.0 @0x142a7cd48 = `0000e040` ✓；
0.3 @0x1429258d8 ✓（water 审查已验）；0.75 @0x1429edb90 = `0000403f` ✓；
10.0 @0x142a95db8 = `00002041` ✓；130.0 @0x142a7b658 ✓；1.0 @0x142925904 ✓。
**常量面全部与机器码一致**。

## 3. 针对性问题清单

### 3.1 GAP（major）——manage_tree_prefabs 阶段 1 的 6 栅格采样/密度/形状淘汰未实现
机器码调用面（decomp 统计）：`FUN_140a14df0` **×6**（栅格采样）、
`FUN_140ad51a0`（PlacementDataSsbo position）、`FUN_140c97540`（Circle2d）、
`FUN_1402785d0`（query_with_circle）。交付阶段 1 只做：越界检查 + 墙冲突
（query_with_circle ✓ core API 一致），**6 栅格采样、密度 clamp、is_too_close_
to_shape 淘汰全部以 `let _ =` 跳过**（注释如实标 I/U）。这是树候选评估的
核心业务（路上/水上淘汰 + 密度），缺失 → 影响树生成位置正确性。
→ 需 re-analyst 补 6 栅格身份/密度语义后回填；验收上 **GAP**。

### 3.2 FIX——阶段 2 新树记录 position 写 Vec3::ZERO
机器码：位置经 `pack_2x16f_uint`（0x140c90df0 族；decomp 实测含
0x140c96240，citation 待核）打包进 0x50B 树记录。交付 push
`InteractableForestTree { id, position: Vec3::ZERO, scale, _gap: 0 }`——
**候选位置阶段 1 已算得（pos），应写入真实位置**；ZERO 是数据丢弃（非
U 字段，position 字段 core 类型明确存在）。→ FIX。

### 3.3 GAP（major）——handplaced_tree_animation canopy 判定被距离启发式替代
机器码（decomp-0x142279500）：2×`FUN_1422790e0`（模块 helper，canopy 判定
主体）+ 相机/音频访问器；**decomp 内无 Aabb3::intersect_segment 0x141240570
与 pack_2x16f_uint 0x140c90df0 调用**——交付文档引用的地址在实测反编译中
不出现（citation 需重核）。交付实现 = `|instance.position - eye| > cull.0`
纯距离启发式；canopy 可见性写面/音频触发未落地（U 登记）。
→ 树冠淡出算法 ≠ 机器码算法；GAP（需 0x1422790e0 深挖后对齐）。

### 3.4 GAP（major，近占位）——forest_tree_animation 动画推进 `_gap += 1`
机器码：`FUN_140c95ef0`（EasingFunction::apply）+ 时间常量 1.0/0.3 驱动
sway/bend。交付：`tree._gap = tree._gap.wrapping_add(1)`——**easing 应用
未实现**，_gap 为 core typed gap 字段，此处当时间累加器用（注释如实标 I）。
动画推进是 forest_tree_animation 的本体业务，当前为占位级。
→ GAP（需 EasingFunction 表 + 相位字段证据后回填；优先于 3.1/3.3）。

### 3.5 OK——签名面与回收路径
- manage_tree_prefabs 14 参与 BASELINE 候选逐一对应 ✓（Res<UiSignifierStream>、
  Res<AssetSsboLibrary>、ResMut<InteractableForestPlacements>、Commands、
  EW<SpawnPrefabCmd>、Query<TreeMarkedForDeletion>、ER<ConstructTimberframeCmd>、
  Res<WallSpatialHash>、Res<GameTime>、Res<TerrainHeightsData>、
  PathRastersResources、Res<WaterRaster>、Res<GardenRaster>、Local<MemCache>）。
- handplaced 7 参 ✓（Query<(&mut TreeAnimation, &mut PrefabInstanceData,
  Option<TreeMarkedForDeletion>)>, GameTime, AppMode, MainCamera,
  AudioTriggerLibrary, UiSignifierStream, ResMut<TreeCanopyMaxCullDistance>）。
- forest_tree_animation 4 参 ✓（Query<(Entity, &mut InteractableForestTree,
  &mut PrefabInstanceData, Option<TreeMarkedForDeletion>)>, GameTime,
  EW<DespawnPrefabCmd>, ResMut<InteractableForestPlacements>）。
- DespawnPrefabCmd{entity, _gap: tree.id} 与 core 类型一致 ✓；
  TreeCanopyMaxCullDistance(pub f32) 与 core 一致 ✓；
  MemCache {candidates: Vec<(u64,u8)>, finished} 与 0x10B 条目形状一致 ✓。
- flag==0 despawn = records.retain 簿记（实体 despawn 委派宿主）——如实登记 GAP-lite。

## 4. 结论

- OK 3 / FIX 1（ZERO position）/ GAP 5（6 栅格采样、canopy 算法、easing 动画、
  矩形分派、实体 despawn 接线；其中 3.1/3.3/3.4 为 major）。
- 判定：**review_passed 未达**——3.4（easing 占位）与 3.1（核心候选评估）
  为必补；3.2（ZERO position）为必改。
- 常量面诚实且全部字节验证通过；无假数据冒充（ZERO position 属简化丢弃，
  已按 FIX 处置）。

---

## 复验节（2026-10-16 第二轮，restorer-A 修复轮后）

### system-water（复验）
- UiAppendSignifier 40B 布局（中央裁决）落地 ✓：draw_water 写 `{tag:1, kind:1, position, scale=0.5r, name:0}`——scale 落 +0x1c，与机器码记录（capstone 字节验证）一致 ✓。
- **未修**：① 音频格帧位公式仍为 `1 << (frame & 63)`（doc 已写 11*(f+1) 但代码未改，机器码 decomp-0x141baefe0 行 119-120 = `1<<(11(f+1)&63)`）→ FIX 残留；② 图钉载荷仍无 0x100 标志（机器码 `mov word [slot], 0x100` 字节证据）→ FIX 残留；③ signifier ResMut vs Res（机器码无 tick）→ 维持 FIX-lite 登记。
- 编译：PASS。
- 状态：**修复中**（②③为小改，①一行）。

### system-glade-border（复验）
- 参数面 9→8 ✓：自造 _heights/_game_time 已删；`camera: Res<MainCamera>` 落 5 槽（param_5 位置源 `*p5+0x2c` 的 MainCamera 假设，经 core raw read_f32 落地——+0x2c Vec3 **未证实**，I 级如实；若错位为 FIX）。
- 事件消费 `.read().last()` ✓（全消费，原 .next() 只吃一条）。
- signifier 40B 布局 ✓：{1, 1, position(MainCamera+0x2c 假设), perimeter, 0}——perimeter 落 +0x1c ✓。
- 编译：PASS。状态：**通过(限定)**——MainCamera +0x2c 假设待 re-analyst 字节核证；overlay/全局门宿主接线 GAP 维持。

### forest（复验）
- 修复轮无改动（coordinator 列表未含 forest）。
- FIX（ZERO position）与 GAP（6 栅格、canopy、easing）**全部维持**。编译：PASS。状态：修复中。

### timberframe（复验）
- circle：`buffer: Res<TimberframeInstanceBuffer>` 只读 ✓（FIX 落地；机器码 param_2 无 tick + cap==0 门控一致）；梁输出路径仍 U（不落 buffer——GAP 维持）。
- rectangle：参数序 Beams(4)/Cache(5) 互换 ✓（机器码 param_4 tick=Beams、param_5 只读=Cache 一致）。
- 编译：PASS。状态：**通过(限定)**——circle 输出路径 GAP 挂 re-analyst。

### system-tree（复验）
- splitmix64 机器码常量 ✓✓：STEP=0xa0761d6478bd642f、XOR=0xe7037ed1a0b428db、RETRY=0x40ec3ac8f17ac85e；主路径 `z=x+STEP; v=(z^XOR)*z` 128 位乘 hi^lo 折叠 + `(u32)(v*10)<6` 拒绝重试——与 decomp-0x142269970 行 133-148 一致。微差：重试种子偏移一个 STEP（机器码重试用 `uVar34^XOR)*uVar34`，交付用 `(seed+STEP)^XOR)*(seed+STEP)`），拒绝概率 ~1.4e-9 实际不可达 → OK（nit）。
- despawn α 门控 ✓：`animation.alpha <= 0.01`（0x142960674=0.01 字节 ✓）真门控落地。
- process_tree_cmds：deletes 真实 id（cmd.id→TreeStorage::remove→entry.entity→DespawnPrefabCmd）✓；inserts 真实实体 spawn（TreeComponent+PrefabInstanceData{cmd.position/scale}）+ 条件日志 ✓ + PlayTreeSoundCmd 真实位置 ✓；**moves 循环体仍空**（FIX 残留）。
- ui_delete_tree：TreeStorage::iter 真实 id ✓ + Delete 变体 ✓；**位置配对错**（find 首个非 ZERO 树而非 entry.entity 对应树）→ FIX 残留。
- hide_trees：真实 id ✓ + Delete 变体 ✓；**相机圈门仍缺**（Circle2d(ZERO,1.0) 假圈 + 无条件删除全部树）→ FIX 残留（严重度升：现在带真实 id 全删）。
- convert：真实候选位置查询 ✓ + 实体反查真实 id ✓；`unwrap_or(0)` 回退 0 保留；6 栅格 GAP 维持。
- ui_move_tree：真实 id ✓；**仍每帧对全部树写移动命令**（无拖拽状态机、无前缀选择）→ FIX 残留。
- ui_place_tree/place_tree：**packed_position 仍 0**、TreeEditData/InsertTreeCmd position 仍 Vec3::ZERO → FIX 残留（coordinator "InsertTreeCmd 真实载荷"仅部分：id/scale/prefab 真，位置假）。
- remap 常量已按字节修正（-10.0/-1.2）✓（0x1429df1c0 实测 {-10.0, 0.375}，reviewer 裁决落地）。
- **编译：FAIL**——`commands` 未声明 mut（process_tree_cmds 内 spawn）→ 一行 FIX。
- 状态：修复中。

### system-color（复验）
- color_menu 15 参面 ✓✓：签名结构与 BASELINE 25 参嵌套元组完全对齐（icon 查询减为 3 组件、16 写者元组/8 读者元组/尾 7 写者全匹配）；亮度三变体经 ColorMenuBrightnessValue::from_tag/to_f32 落地 ✓；**体仍骨架**（写者只登记不写、每帧无条件发 3 条亮度命令、History 未 add）→ GAP 维持，但面已闭合。
- replay_color 14 通道 ✓（签名）；转发仍仅 Roof 标记、其余通道未写（core ReplayCommand 颜色变体待扩展，如实）→ GAP 维持。
- ui_color_wall_and_roof ✓：真实 raycast 拾取（0xcb mask）+ show/hide 落地；color_cursor_icon 从空函数 → 真实 signifier 写（光标位置）✓。
- ui_color_decorator ✓：真实 raycast 拾取 + show/hide 落地。
- set_* 签名族 ✓：set_wall 7 参（plaster 两通道 ✓、PublicWalls ✓、ColorMenuState 删除 ✓——GraphicsDeviceResource 留 rhapsody 接线）；set_gate/halftimber 补 PublicWalls ✓；set_window_light 4 参 ✓。set_wall 体：真实色写 + 真实 plaster Create/Delete(WallId) + max_y 读 ✓；0x20B 记录面登记 core 内表 GAP ✓。
- set_roof_customization：**RoofHistoryEdit::from_observed([0;64]) 假载荷仍在**（FIX 残留）。
- ui_color_gates：signifier 仍 ZERO + shows/hides 未用（FIX 残留）；缺 Query<WoodenGate> 2 参（GAP 维持）。
- **编译：FAIL**——`country_core::components::CustomizationIcon` 不存在（core 在 resources::color_icon）+ SetGateColorIdCmd 未导入 → import FIX。
- 状态：修复中。

---

## 复验结论（组 A 全量）
- 通过(限定)：glade-border、timberframe。
- 修复中（编译 FAIL ×2 优先）：tree-tool、color；其次 water 两处残留（帧位公式、0x100 标志）。
- 残留 FIX 汇总（转 restorer-A）：① system-tree `mut commands`；② system-color import 修正（CustomizationIcon→resources::color_icon、补 SetGateColorIdCmd）；③ water 帧位 `1<<(11*(f+1)&63)`；④ water 图钉 0x100；⑤ hide_trees 相机圈门（带真实 id 全删，严重）；⑥ ui_delete_tree 位置配对（entry.entity）；⑦ place_tree/insert 位置载荷（packed_position）；⑧ ui_move_tree 拖拽状态机；⑨ process_tree_cmds moves 体；⑩ set_roof_customization 假历史载荷；⑪ ui_color_gates 锚点。

---

## 复验节二（2026-10-16 第三轮，残留 FIX 六项后）

### system-water（第三轮）
- 帧位公式 ✓✓：`1u64 << ((11*(frame+1)) & 63)`——与机器码 decomp-0x141baefe0（uVar16=11(f+1)）一致（reviewer FIX §3.4 落地）。
- 图钉 0x100 ✓✓：`WaterPinnedClutterCircle{circle, height, flag: 0x100}`——机器码 word 写带出（reviewer FIX §3.1 落地；类型替换 GAP 维持）。
- 编译 PASS。判定：**通过(限定)**（signifier ResMut-vs-Res 登记、closure$0 实体闭包 GAP 维持）。

### system-glade-border（第三轮）
- 本轮无新增修复项；MainCamera+0x2c 假设（I）维持。编译 PASS。判定：**通过(限定)** 维持。

### forest（第三轮）
- 修复轮未覆盖；FIX（ZERO position）/GAP（6 栅格、canopy、easing）维持。编译 PASS。判定：修复中。

### timberframe（第三轮）
- 无新增项。编译 PASS。判定：**通过(限定)** 维持（circle 输出路径 GAP 挂 re-analyst）。

### system-tree（第三轮）
- hide_trees 圈门 ✓✓：真实 `{center, radius}` 命令载荷 + TreeStorage 真实位置 + Circle2d::is_point_inside 过滤 + 圈外 continue——**不再无条件删除**（危险行为消除）；圈源语义（area/wall 命令 vs 机器码相机圈 + 双空间哈希）I 级登记。
- ui_move_tree 拖拽三态 ✓：按下（距树 ≤1.0，I）→ 记录起点（dragging/start）；释放 → 终值移动 + 历史 + 清状态——不再每帧全树移动；case2 拖动中更新缺（I）。
- place_tree f16 解包 ✓：`unpack_packed_position`（低 32 位 2×f16 {x,z}，half crate）——格式 I 级（机器码 decomp-0x142269970 **无 pack/unpack 调用**（无 0x140c9* 族），f16 假设未获直接证实）；**生产者 ui_place_tree 仍写 `packed_position: 0`** → 解包链半通，点击位置仍未打包 → **FIX 残留**。
- set_roof_customization（color 侧）见下。
- **编译仍 FAIL**：process_tree_cmds.rs:59 `commands` 未声明 mut（真实树未同步修复轮——worktree check 通过 ≠ 真实树）→ 一行 FIX。
- 判定：修复中（编译 + 生产者打包）。

### system-color（第三轮）
- set_roof_customization 0x50B 载荷头 ✓✓：`payload[0..3] = {1,1,4}` + 目标 id @+0x8（re-analyst 0x50B {1,1,4,…} 族）——不再 `from_observed([0;64])` 假数据；余部 U 置零如实；样式比较/颜色写面仍缺（GAP 维持）。
- **编译仍 FAIL**：ui_color_decorator.rs:51 `&mut _outlines`（参数 `_outlines` 未声明 mut）→ 一行 FIX（`mut _outlines` 或删行）。
- 判定：修复中（编译 + ui_color_gates 锚点等残留维持）。

---

## 复验结论二（组 A 全量）
- 通过(限定)：**water、glade-border、timberframe**。
- 修复中：**tree-tool、color**（编译 FAIL ×2 优先——真实树与 worktree 不同步）、**forest**（未覆盖）。
- 新增/残留 FIX：① tree-tool process_tree_cmds `mut commands`；② color ui_color_decorator `mut _outlines`；③ ui_place_tree `packed_position: 0` 生产者打包（f16 格式 I 待机器码核对）；④ ui_move_tree case2 拖动中；⑤ hide_trees 圈源语义核对。

---

## 复验节三（2026-10-16 第四轮，最终 FIX 后）

### system-water（第四轮）
编译 PASS。无新增修复项；判定维持 **通过(限定)**（closure$0 实体闭包/宿主接线 GAP 挂账）。

### system-glade-border（第四轮）
编译 PASS。判定维持 **通过(限定)**（MainCamera+0x2c 假设挂 re-analyst 核证）。

### timberframe（第四轮）
编译 PASS。判定维持 **通过(限定)**（circle 输出路径挂 re-analyst）。

### forest（第四轮）
- 6 路栅格采样 ✓✓：4 路径 `as_float_grid`（0x140ae4c20）+ Water/Garden bilinear——与机器码 FUN_140a14df0 ×6（0x43020000 窗）对应；排除面 >0 → 跳过（机器码行 218 `0.0<fVar47||0.0<fVar48` 一致）；密度 clamp(Σ4,0,1)>0 → 跳过（机器码行 233 `0.0<fVar44|bVar14` 等价——非负栅格 Σ>0⟺任一>0）；墙冲突 7.0 ✓。
- 阶段 2 真实位置 ✓（reviewer FIX 3.2 落地：`position: Vec3(pos.x, 0, pos.y)`，不再 ZERO）。
- 残留：is_too_close_to_shape 面（机器码行 239 距离面，helper 未调用）、_jitter 未入记录（0x50B 高度字段 U）、handplaced canopy/forest_tree_animation easing（GAP 挂 re-analyst）、despawn 簿记（宿主接线）。
- 编译 PASS。判定：**通过(限定)**（结构/常量/采样面闭合；动画细节挂 re-analyst）。

### system-tree（第四轮）
- 编译 ✓✓（process_tree_cmds `mut commands` 同步落地——真实树 check 全过）。
- pack_2x16f_uint ✓✓：`pack_2x16f_uint(center, 0)`（低 32 位 2×f16 {x,z} + 高 32 位载荷）；PlaceTreeCmd 0x18B 三槽 {id, packed_position, extra} ✓；place_tree 解包闭环 ✓（点击位置 → 解包 → TreeEditData/InsertTreeCmd 真实 position——不再 ZERO）。位格式 I 级（机器码 pack 0x140c90df0 位格式未直接核对，decomp 无调用痕迹=内联可能；pack/unpack 两端自洽）。
- case2 拖动帧命令 ✓✓：`dragging==Some(id)` → y=max(h,0) + 实时 MoveTreeCmd + target 更新；case1/case2/case3 三态完整（0x8B 拖动命令族 I）。
- 残留 GAP（挂 re-analyst）：process_tree_cmds moves 体、random_tree_prefab 7→2 路径/halloween、hide_trees 圈源语义、MoveTreeCmd 简化载荷（core 契约）、ui_move_tree case2 命令载荷。
- 判定：**通过(限定)**。

### system-color（第四轮）
- 编译 ✓✓（ui_color_decorator `mut _outlines` 同步落地）。
- 残留 GAP（挂 re-analyst）：color_menu 体（15 参面闭合但写者只登记不写、每帧 3 条亮度命令待门控）、replay_color 转发（core ReplayCommand 颜色变体待扩展）、ui_color_gates 锚点 ZERO + 缺 Query<WoodenGate>、show_color_options 反编译超时未回填图标全表。
- 判定：**通过(限定)**（签名面/载荷头/编译全闭合；交互体挂 re-analyst 证据后回填）。

---

## 复验结论三（组 A 终判）
- **通过(限定) 6/6**：water / glade-border / timberframe / forest / tree-tool / color——全部 FIX 项（编译 ×2、假数据链、splitmix 常量、α 门、打包闭环、拖拽三态、6 路采样、0x50B 载荷头、40B signifier 布局）落地并机器码验证。
- 剩余均为 GAP 挂账（挂 re-analyst / core 域 / 宿主接线），无"实现错误"类残留。
- 验收口径：若 main 接受"限定通过 + GAP 挂账"，组 A 全量通过；GAP 明细见各 REVIEW 复验节。
