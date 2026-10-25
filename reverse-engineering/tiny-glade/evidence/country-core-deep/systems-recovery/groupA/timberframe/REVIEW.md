# REVIEW: timberframe（crates/systems/timberframe）

- 审查日期：2026-10-16；审查人：reviewer-A（dbid tg_va1）
- 交付：`D:/game/tiny-glade/crates/systems/timberframe/`（4 文件，workspace 已注册）
- 编译：`cargo check -p timberframe` PASS
- 证据：BASELINE / fresh decomp `decomp-0x1421b6610.txt`（audio）、
  `decomp-0x1421b67a0.txt`（circle）、`decomp-0x141a41ba0.txt`（rectangle）/
  rne-read 字节。

## 1. 逐项核对表

| 函数（VA） | 声称实现 | 实测判定 |
|---|---|---|
| impl$0::add_timberframe（0x140630a20） | build + register + add_systems | OK |
| timberframe_beam_count_change_audio（0x1421b6610） | 4 参 + (current−previous==1)&&non_empty → PlayTimberframeSound | OK/FIX-lite（计数布局 I，见 §3.3） |
| construct_timberframe_circle（0x1421b67a0） | 5 参 + 梁构建 push buffer | **FIX**（param_2 应为 Res 只读；push 输出路径与机器码矛盾，见 §3.1/§3.2） |
| construct_timberframe_rectangle（0x141a41ba0） | 5 参 + 四边梁构建 | **FIX**（参数序 Beams/Cache 互换，见 §3.4） |
| timberframe_on_glade_unload（行 50 内联） | 未单独落地 | GAP-lite（内联回调，宿主接线登记） |

## 2. 常量字节验证（rne-read）

0.2 @0x142a8c6f0 = `cdcc4c3e`×2 ✓（BEAM_SPACING，圆/矩形共用）；
0.4922 @0x142925874 = `c10ffc3e` = 0.49219… ✓（PLANK_LENGTH_SCALE）；
-1.0/-0.5 @0x14295a0e4 = `000080bf 000000bf` ✓（HEIGHT_SIGN/HEIGHT_HALF）；
0.5 @0x142925870 ✓（NORMALIZE_HALF）。
**未验证**：RECT_SCALE=0.63 引 0x1429258e0——该址实测为 `0.5×2`（`0000003f×2`），
0.63 无字节支持（impl 已标 I，登记待 re-analyst 补证）。

## 3. 针对性问题清单

### 3.1 FIX（签名）——circle param_2 = Res（只读），交付误用 ResMut + push
机器码（decomp-0x1421b67a0）：param_2 **无 tick 回写**、`if (*(longlong *)*param_2
== 0) goto …` 以 cap==0 门控、`unaff_R15 = ((longlong *)*param_2)[1]` 读条目基址
+ 元素 +0x18 LOCK/INC（Arc 读面）——**只读**；BASELINE 候选亦为
`Res<timberframe::TimberframeInstanceBuffer>`。交付用 `ResMut<…>` 并向
`buffer.0` push 梁实例——**与机器码读写面直接矛盾**。→ FIX（改 Res；梁输出
路径见 3.2）。

### 3.2 GAP——circle 梁构建的输出路径未解出
机器码几何环（0x140c97ba0 get_pos_at_u / 0x140c975a0 get_normal_at_u /
0x140c9cdc0 打包 / 0x140c9cde0 写盘族 + u 域校验 panic）构建梁实例——但
param_2 只读，输出不落 buffer。候选输出面：param_3 ssbo 查询
（FUN_1421b5d00，AssetSsboLibrary）或 param_4 cache。交付假设"push buffer"
已被 3.1 否证 → 输出路径需 re-analyst 补证后回填。GAP（major，与 3.1 联动）。

### 3.3 FIX-lite/GAP——TimberframeBeamCount 布局与读取面
audio（0x1421b6610）：param_2 = ResMut（tick 行 102-103 回写）✓ 交付槽位 2
一致；但机器码另有 `*(int*)(*param_3 + 0x2c)` 读面（param_3 无 tick = Res——
BASELINE 候选为 Res<GameTime>，则 +0x2c int ≠ 梁数，计数真源待核）。
交付 `{current, previous, non_empty}` 为 I 级布局，推进簿记
（`beams.previous = current`）位置未与机器码核对（impl 自注）。→ re-analyst
核对 +0x2c 读取面身份。

### 3.4 FIX（参数序）——rectangle 的 Beams/Cache 槽位互换
机器码（decomp-0x141a41ba0 行 314-317）：**param_4 = tick（ResMut）= BeamCount**、
**param_5 = 只读（Res）= Cache**；BASELINE 候选同为 (…, ResMut<BeamCount>,
Res<Cache>)。交付 rectangle：(…, `_params`(cache) 4 位, `beams` 5 位)——
**互换**。circle 侧 (…, Cache, BeamCount) 与机器码一致 ✓。→ FIX（rectangle
参数序交换）。注：Bevy 调度不受参数序影响，但验收面要求与机器码一致。

### 3.5 OK——其余面
- audio：ER 双缓冲 0x28B 条目 ✓（机器码 stride 0x28）；(ER, ResMut<Beams>,
  Res<GameTime>, EW<PlayTimberframeSound>) 与 BASELINE ✓；
  PlayTimberframeSound{kind:u8, position:Vec3} core 类型 ✓（kind=6 为机器码
  记录 I；size_01 字段 core 待扩展，登记）。
- rectangle：param_2 ResMut + tick ✓；写面（直接元素写 + FUN_141a3ce70 旧数据
  释放族）与交付 Vec push 布局等价 ✓。
- 两构建系统 `_library` 未用——机器码两处 `FUN_1421b5d00`（ssbo 查询）→
  WoodenPlankSsbo 写面 GPU 后置（dispatch 约定），登记 GAP-lite ✓。
- lib.rs：insert_resource×2 + register_event + add_systems 面 OK。

## 4. 结论

- OK 4 / FIX 3（circle Res 只读、rectangle 参数序、audio 计数布局核对）/
  GAP 3（circle 输出路径、ssbo 写面、unload 回调）。
- 判定：**review_passed 未达**——3.1（circle 读写面矛盾）为必改（会连带 3.2）；
  3.4 参数序为必改；常量面诚实（0.63 无字节支持已标 I）。

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
