# REVIEW: system-tree（crates/systems/tree-tool）

- 审查日期：2026-10-16；审查人：reviewer-A（dbid tg_va1）
- 交付：`D:/game/tiny-glade/crates/systems/tree-tool/`（11 文件，workspace 已注册）
- 编译：`cargo check -p system-tree` PASS
- 证据：BASELINE / fresh decomp（0x142267430、0x1422686d0、0x142268dc0、
  0x142269170、0x142269970、0x14226a260、0x14226b1a0、0x141bd2510 等）/
  rne-read 字节 / capstone。

## 1. 逐项核对表

| 函数（VA） | 声称实现 | 实测判定 |
|---|---|---|
| impl$0::build（0x140650520） | register + add_systems ×11 | OK |
| process_tree_cmds（0x142267430） | 命令消费 + 日志 + 转发 | **FIX/GAP**（moves 循环体空、insert 用 Entity::PLACEHOLDER、日志无条件、despawns 死代码；见 §3.1） |
| despawn_trees（0x1422686d0） | α≤0.01 回收 | **FIX**（α 门未实现，无条件回收；常量 `let _ =` 弃用） |
| convert_preplaced_trees（0x14226be70） | 候选评估 + 墙冲突 | **FIX/GAP**（墙查询中心 = Vec2::ZERO 假数据、DeleteTreeCmd id=0 假删除、6 栅格采样未实现） |
| hide_trees（0x14226a260） | 相机圈隐藏 + 历史 | **FIX/GAP（major）**（对全部树无条件 delete id=0 + Move 变体历史，机器码的圈判定/可见性写/双哈希全缺） |
| recalc_tree_position_on_terrain_height_change（0x142268dc0） | 界内重算 y | OK（本 crate 最佳：门/界/采样/写回全落地；采样 0x141246fb0×2 ✓、65.0 ✓ 字节） |
| replay_tree（0x141bc34f0） | ReplayBuffer 分派 | OK（签名 ✓；载荷 U 在 core，诚实） |
| tree_move_bend（0x1422689c0） | 位移插值 | **GAP（major，占位）**（`_gap += 1` + `let _ = &mut *data`；机器码 time_continuous_lerp×2 + clamp + 写新位置全缺） |
| ui_delete_tree（0x141bd2510） | 半径删除 | **FIX**（半径 ✓ 3.0 字节验证，但 DeleteTreeCmd id=0 假 id、历史用 Move 变体冒充删除） |
| ui_move_tree（0x14226b1a0） | 拖拽移动 | **FIX/GAP（major）**（无拖拽状态机/无 prefab 前缀选择/无 clicked 门控，每帧对全部树写 MoveTreeCmd{position.x bits 冒充 id} + History） |
| ui_place_tree（0x142269170） | 放置 UI | **FIX/GAP**（splitmix 常量错、PlaceTreeCmd 载荷 0、clicked 门缺失、边界反馈无条件、全局门缺） |
| place_tree（0x142269970） | 放置执行 | **FIX（major）**（splitmix64 用标准常量替换机器码常量；0x48B 树记录未推；TreeEditData::default 假载荷；packed_position 丢弃） |
| random_tree_prefab（0x14226a170） | 季节分派 | GAP（7 路径表只实现 2 条；halloween 分派未实现；机器码入参 = 树索引非变体） |
| remap_pos_y_if_in_water（内联） | 水下重映射 | **FIX**（1.5 常量字节不符：0x1429df1c0 = {-10.0, 0.375}；-1.2 ✓ 0x142a8cc64；max() 与记载的 lerp 公式不一致） |
| seed_cursor_icon（内联） | 空函数 | GAP-lite（并入调用点未落地，空 fn 无逻辑） |
| reset_tree_tool（内联行 4） | storage.clear() | OK-lite（I 级语义） |

## 2. 常量字节验证（rne-read）

- 0x142960674 = `0ad7233c` = **0.01** ✓（DESPAWN_ALPHA_THRESHOLD）；
- 0x1429cd078 = `00004040` = **3.0** ✓（DELETE_RADIUS_SCALE）；
- 0x142a80ce0 = {0.0, 0.5} ✓（widget 定位偏移）；
- 0x1430354e8 = 'clearing/naked_tree/large' + 'move_tree_' ✓（前缀匹配串存在）；
- **0x1429df1c0 = `000020c0 0000c03e` = {-10.0, 0.375}——非 1.5**（WATER_REMAP_DIVISOR 常量引用错误）；
- 0x142a8cc64 = `9a9999bf 000080c0` = {-1.2, -4.0} ✓ -1.2。

## 3. 针对性问题清单

### 3.1 FIX（major）——process_tree_cmds 占位级缺陷
- moves 循环体**空**（机器码 0x20B MoveTreeCmd → 实例位置更新全缺）→ GAP；
- inserts：`storage.insert(…, TreeStorageEntry { entity: Entity::PLACEHOLDER })`——
  **假实体**；`spawns.write(SpawnPrefabCmd { _gap })` 载荷仅 id；日志
  "Tree already exists" **无条件**（机器码仅替换冲突时打）；`let _ = &mut despawns;`
  死代码（机器码删除走 DespawnPrefabCmd）→ FIX；
- deletes：`storage.remove(TreeStorageIndex(cmd._gap))` 面 ✓ 但随后对 instances
  的 `let _ = &mut *data; break;` **无操作循环**（占位气味）→ GAP。

### 3.2 FIX（major）——splitmix64 机器码常量被标准常量替换
机器码（decomp-0x142269970 行 133-148，字节证据）：
`z = x + 0xa0761d6478bd642f; v = (z ^ 0xe7037ed1a0b428db) * z`（128 位乘
hi^lo 折叠）；拒绝采样 `(u32)(v*10) < 6` 重试（重试种子 `x + 0x40ec3ac8f17ac85e`，
步进 +0xa0761d6478bd642f）。
交付：**标准 splitmix64 常数对**（0x9e3779b97f4a7c15 / 0xbf58476d1ce4e5b9 /
0x94d049bb133111eb + 移位）——与机器码完全不同，变体序列错。→ FIX（按机器码
常数与折叠/拒绝结构重写；re-analyst 记录的常数对已获字节证实）。

### 3.3 FIX（major）——ui_place_tree / place_tree 载荷假数据
- `place_cmds.write(PlaceTreeCmd { packed_position: 0, _gap: 0 })`——机器码
  0x18B 命令 = {u64 id?, u64 打包位, u32}；packed_position=0 即**点击位置被丢弃**
  （函数内 position 已算得）→ FIX；
- `place_tree`：`let _ = cmd.packed_position;`——0x48B 树记录**未推**；
  `TreeEditData::default()` 假载荷入 History；`InsertTreeCmd{_gap: id}` 仅 id。
  → FIX（真实位置/prefab/缩放入载荷；记录推入面待 core TreeStorage 补全）；
- PlaceTreeCmd 类型 16B ≠ 机器码 0x18B（缺 id u64）→ FIX-lite。

### 3.4 FIX——despawn_trees α 门未实现
机器码：`α ≤ 0.01`（0x142960674，字节 ✓）门控回收（α = TreeAnimation 消退
进度，字段 U）；交付**无条件** despawn 全部标记树，阈值常量弃用
（`let _ = DESPAWN_ALPHA_THRESHOLD;`）。→ FIX（读 TreeAnimation α 字段——
字段偏移需 re-analyst 补证）。

### 3.5 FIX（major）——hide_trees / convert_preplaced_trees / ui_delete_tree 假删除
- hide_trees：机器码 = 相机圈（with_expanded_diameter 0x140c980c0 +
  contains_point 0x140c980f0）+ TreeStorage::iter（0x140a1b100）+ 可见性写 +
  History（FUN_140a84ec0）变体 2 删除；交付对**每棵** Query 树无条件
  `deletes.write(DeleteTreeCmd { _gap: 0 })` + Move 变体历史——**id=0 假删除、
  每帧刷历史** → FIX（按 TreeStorage 索引发真实 id；变体按机器码 2）；
- convert_preplaced_trees：机器码 6 栅格密度评估（0x140a14df0 族）→ 墙查询
  （0x1425defd0）在**候选树位置**；交付 `query_with_circle(Vec2::ZERO, …)`
  （原点查询 = 假数据）+ `DeleteTreeCmd{_gap:0}` 假删除 → FIX；
- ui_delete_tree：机器码删除半径 = brush×3.0 ✓（3.0 字节验证）但树索引取自
  **TreeStorage 全遍历**；交付 Query 遍历 + `_gap: 0` 假 id + Move 变体历史
  → FIX（真实索引 + 删除变体）。
- 注：core `TreeEdit` 仅 Place/Move 变体（章节证据），机器码 0x58B 变体 2 =
  删除在 core 类型中缺失 → 删除语义需 core 域补 Delete 变体（或登记等价编码），
  不得以 Move 静默冒充（replay 会把删除回放成移动）。

### 3.6 FIX/GAP（major）——ui_move_tree 无拖拽状态机
机器码：prefab 前缀匹配（0x1430354e8 'clearing/naked_tree/…'）选可移动树 +
widget clicked 状态机（case1 起点/case2 拖动中 y=max(h,0)/case3 释放 →
0x20B 命令 {id, 打包位, x, y, z} + History 变体 1）；交付：**对全部树每帧**
`moves.write(MoveTreeCmd { _gap: data.position.x.to_bits() })`（位置 x 位当 id =
假数据）+ Move 历史（id=0、prefab_name 空）→ FIX（状态机 + 真实 id/载荷）。

### 3.7 GAP-lite——全局门禁缺失
机器码：ui_place_tree / ui_delete_tree / ui_move_tree 均含 `*0x14362c098`
早退（decomp 各 ×1 证实）；交付三函数均未建模（water crate 已建模同门）。
→ FIX-lite（与 water 同法）。

### 3.8 OK——recalc / replay / build
- recalc：OnTerrainChange 门 ✓、65.0 界 ✓（0x142a83814 字节）、采样
  `FUN_141246fb0`×2 ✓、y=max(h,0) I 级 ✓、写回 ✓——本 crate 唯一完整业务；
- replay_tree：签名 ✓（ReplayBuffer + 3 EW + GameTime），ReplayCommand::Tree
  分派面 ✓，撤销-放置→删除 I 如实；载荷 U 在 core；
- build：init_resource + 11 系统注册面 OK。

## 4. 结论

- OK 3 / FIX 8（splitmix 常量、α 门、PlaceTreeCmd 载荷、假删除 ×3、MoveTreeCmd
  载荷、remap 常量）/ GAP 7（moves 处理、栅格评估 ×2、拖拽状态机、随机树
  分派、树记录推入面、删除变体 core 缺、seed_cursor_icon）。
- 判定：**review_passed 未达**。最严重：3.2 splitmix 常量替换（机器码证据确凿）、
  3.5 假删除链（id=0 数据会污染回放/历史）、3.6 每帧假移动命令。此三项必改，
  且 3.5/3.6 涉及 core TreeEdit/TreeStorage 契约（id 来源、Delete 变体），
  建议 restorer-A 修复 + re-analyst 补树记录字段后复验。
- 无偷懒标注隐瞒：多数空实现有注释，但 `Entity::PLACEHOLDER`、`Vec2::ZERO`
  查询、`_gap: 0` 删除、`_gap += 1` 动画为**以假数据/占位冒充实现**，超出
  "诚实标注 U" 的边界（reconstruction/tree-tool 旧草稿的 read_u64→0 模式
  在新交付中仍局部存在）。

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
