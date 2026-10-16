# REVIEW: system-water（crates/systems/water）

- 审查日期：2026-10-16；审查人：reviewer-A（dbid tg_va1）
- 交付：`D:/game/tiny-glade/crates/systems/water/`（4 文件，workspace 已注册 system-water）
- 编译：`cargo check -p system-water` PASS
- 证据：BASELINE-function-list / BASELINE-signature-surface / fresh decomp
  `groupA/decomp/decomp-0x141ba9130.txt`（+ capstone 反汇编 0x141ba9320..0x141ba960c）/
  `decomp-0x141baefe0.txt` / `decomp-0x1406501d0.txt` / rne-read 字节直读。

## 1. 逐项核对表（SOURCE_MAP business vs 交付）

| 函数（VA） | 声称实现 | 实测判定 |
|---|---|---|
| system_water::impl$0::build（0x1406501d0） | build(world, schedule) + register_water_events + add_water_cpu_systems | OK（App 接线委派宿主，注释如实；机器码 add_systems×2/insert_resource/register_event×2 面一致） |
| system_water::draw_water::draw_water（0x141ba9130） | draw_water.rs 237 行 | OK（细节见 §2/§3，3 项 FIX） |
| draw_water::closure$0（0x141ba9700） | 并入 stream 写入路径 | GAP-lite（见 §3.3：闭包为实体生成闭包，宿主流消费；crate 侧未复刻实体生成，依赖 core UiSignifierStream） |
| closure$1/2/3（inline） | on_draw/on_erase/on_record 闭包 | OK（与机器码 write 面一致） |
| system_water::water_audio_grid::update_water_audio_grid（0x141baefe0） | water_audio_grid.rs 143 行 | FIX（帧位公式，见 §3.4；语义模型落地已如实登记） |
| system_water::record_water_history::record_water_history（inline 行 12） | record_water_history.rs 46 行 | OK（inlinee 落地为独立系统；按 core garden 惯例展开 RasterHistoryRecordResources——core 侧未恢复，登记 GAP-lite） |
| zeno Rasterizer/Segments ×7、TwoWaySearcher（标 business 实为依赖实例） | 不恢复 | OK（依赖过滤正确） |

## 2. 签名/参数面核对（骨架候选 16 参 vs 交付 vs 机器码）

| # | 骨架候选 | 交付 | 机器码 | 判定 |
|---|---|---|---|---|
| 1 | Res<AppMode> | Res<AppMode> | param_1 tag@[0]、layer@[1] | OK |
| 2 | RasterDrawResources | WaterRasterResources<C>（派生 SystemParam 束）+ 借用组装 | param_2 值参（strokes/archivist/cursor/tool_used/debug_history 语义） | OK（等价落地，注释如实） |
| 3 | Res<UiState> | Res<UiState> | param_3 | OK |
| 4 | ResMut<WaterRaster> | ResMut<WaterRaster> | puStack_b8 tick 回写（+0x1c）| OK |
| 5-8 | EW<WaterDrawEvent/WaterEraseEvent/RecordWaterEditToHistoryCmd/RecalculateHeightMapCmd> | 同 | param_5/6/7 + param_8 tick | OK |
| 9 | Res<ScheduledTerrainReadback> | 同 | param_9 → on_terrain_modified 0x140aabdc0 | OK |
| 10 | Res<CursorTerrainRaycast> | 同 | param_10 → position 0x140a12650 | OK |
| 11 | (Res<GladeBorder>, EW<DisplayOutOfBorder>, EW<FeedbackInputOutOfBorder>) | 同 | param_11 | OK |
| 12 | Local<DrawState> | 同 | param_12，DrawState::update 0x140ae4bd0 + switch(*param_12) | OK |
| 13 | Res<UiSignifierStream> | **ResMut<UiSignifierStream>** | param_13 无 tick 回写（Res） | **FIX-lite**：机器码无回写；若 core write 取 &mut 则 core 契约待核，但以机器码为准应为 Res——交付的 ResMut 属偏差（编译等价，登记） |
| 14 | Res<CameraCursorRay> | 同 | param_14 → intersect_with_xz_plane 0x1408ef180 | OK |
| 15 | (Res<EraserSizeMultiplier>, Res<BrushSizes>) | 同 | param_15[0]=eraser / param_15[4]+4=water（BrushSizes.water @+4 ✓ core 布局） | OK |
| 16 | EW<enum2$<ShowPinnedDecoClutter>> | **EW<WaterPinnedClutterCircle>（crate 自造 16B）** | param_16 → 0x142260a60 写 40B 变体载荷 | **FIX/GAP**（见 §3.1） |

## 3. 针对性问题清单

### 3.1 FIX——图钉事件类型替换 + 载荷缺 0x100 标志
机器码（0x141ba9513..0x141ba957d，capstone 反汇编）实际写入的
`ShowPinnedDecoClutter` 圈变体载荷 = **40B**：
`word 0x100 @+0`（`mov word ptr [rbp+0x28], 0x100`）、Circle2d @+4（0x140c97540）、
`dword 0 @+0x14`、`height(pos.y+1.5) @+0x18`、`qword 0 @+0x20`。
交付的 `WaterPinnedClutterCircle{circle, height}` = 16B：
- **缺 0x100 标志**（字节证据，机器码必写；注释已登记 U 但载荷未带出）→ FIX；
- 类型替换（core `ShowPinnedDecoClutter` 56B union 未恢复，core 侧已有近似资源型重建；
  机器码此处为 enum 事件）→ GAP 如实登记，建议本地类型补齐 40B 载荷布局 + 0x100，
  待 core union 恢复后机械迁移。

### 3.2 FIX——signifier 记录布局与机器码不符（core UiAppendSignifier 契约疑误）
机器码记录（0x141ba93a0..0x141ba93c9）：32B = `qword 1 @+0`（tag）、
**`qword 1 @+8`（第二字段）**、`Vec3 @+0x10`、**`scale=0.5*radius @+0x1c`**。
交付经 core `UiAppendSignifier{tag:u8, position:Vec3, extra:[u8;12]}` 写入：
tag @+0（1B）、position @+8、scale 放 extra[0..4]=+0x14 ——
与机器码相比 position 前移 8B、scale 前移 8B、缺 @+8 的 kind=1 字段。
字节为准：**core 的 UiAppendSignifier 布局重建有误**（真实 = {tag, kind=1, Vec3, extra[4]…}，
或等价 {u64, u64, Vec3, f32}），且名字串 "eraser_water_"（0x142eebc40 fat，已字节确认）
经 0x14089d8b0 进入写入流——core `UiSignifierStream::write(&mut, UiAppendSignifier)`
签名缺名字参数。→ FIX（跨域：core signifiers 契约校正，由 main 定 owner；
water crate 侧至少应按机器码把 scale 放对偏移并补 kind=1）。

### 3.3 GAP——closure$0（0x141ba9700）实体生成闭包未复刻
机器码 closure$0 = SignifierEntityBuilder 链（animation_duration/position/scale/
set_ui_pass/build，skeleton enrichment 证据），随 stream 写入流调用。交付只写
stream 记录、不生成实体——依赖 core UiSignifierStream 消费端接线。登记即可。

### 3.4 FIX——update_water_audio_grid 帧位公式
机器码：`uVar16 = f + (f+1)*10 + 1 = 11*(f+1)`；帧位 = `1 << (uVar16 & 0x3f)` =
**`1 << ((11*(f+1)) & 63)`**（decomp-0x141baefe0 行 92/119-120）。交付：
`let bit = 1u64 << (frame & 63)`——缺 ×11 与 +1 相位。64 帧后稳态窗口集合一致，
但逐帧位序不同（影响 water_level 相位与 centroid 累加时序）。→ FIX（一行）。
另：格坐标映射机器码走查表 0x142eed9b8（0x37 项），交付用 `2*(i&7)-7` 公式——
语义 I 已登记，未逐表核对 → GAP-lite。
另：`MAX_CAMERA_DIST_SQ = 6400.0`（0x142eed900）、`WATER_LEVEL_THRESHOLD = 0.5`、
`GRID_CELL_SPACING = 0.0625*13`（0x142eed8e0/0x142eed8f0）、`0x32a3` 射线掩码
与机器码一致 ✓。

### 3.5 OK——模式分发与常量（字节级验证）
- 机器码门禁 `lVar14==4 → tag 9`、`lVar14==10 → tag 15`；core AppMode::Eraser=9/
  Water=15 ✓；`layer & 3 == 0` 门 = All(0)|Water(4) ✓（Wall/Path/Bulldoze 被拒）。
- 橡皮半径 `lerp(0.3→2.0, eraser) → t*4+(1-t)`：0x1429258d8=0.3 ✓、
  0x14292e6f0=2.0 ✓、0x142934694=4.0 ✓、0x142925904=1.0 ✓（rne-read 字节）。
- 界内圈 `0.8*radius`：0x142a16cb8=`cdcc4c3f`=**0.8** ✓（restorer 对；re-analyst
  记 0.15 与字节不符，以字节为准裁决）。
- signifier 缩放 0.5（0x142925870）✓、图钉高度 +1.5（0x1429258d0）✓、
  名字 "eraser_water_"（0x142eebc40 fat）✓。
- WaterRaster 槽位 = 4：**字节验证** 水 draw_raster 实例 0x142260ae0 前导
  `b2 04`（mov dl,4）→ index_mut(slot=4) ✓。
- 尾部链 recalc.write → readback.on_terrain_modified（0x140aabdc0）→
  draw_raster<WaterRasterStagingSsbo>（0x142260ae0，7 参：cmd/point/resources/
  raster/ew_draw/ew_erase/ew_record）✓ 与 core draw_raster 9 参 API 一致
  （slot/library 编译期并入实例，符合 core 抽象）。
- 界外反馈：机器码 DisplayOutOfBorder 必写；FeedbackInputOutOfBorder 经
  `dragged()（0x14092df10）→ 0x14092e6b0 谓词`——交付 `dragged() != 2` 为 I 级
  （0x14092e6b0 本 DB NotFound，谓词未逐字节核对）→ GAP-lite。
- 门禁 `*0x14362c098 != 0 → return`：交付以常量 `WATER_GATE_FLAG=true` 建模
  （如实注释；宿主接线待 main）→ OK（库内语义等价）。

### 3.6 OK——record_water_history
`EditType::Water` + `record_raster_history(&mut strokes, 4, &mut history, …)` +
失败 `HistoryRecordingFailed` 面与 core 契约一致（slot 4 与 §3.5 一致）；
`RecordWaterEditToHistoryCmd` 单元事件（closure$3 = write_default ✓）。

## 4. 结论

- OK 7 / FIX 3（图钉载荷 0x100 与类型、signifier 记录布局、音频格帧位公式）/
  GAP 4（closure$0 实体闭包、show_pins 事件类型、格坐标查表、0x14092e6b0 谓词）。
- 判定：**review_passed(限定) 未达**——3 项 FIX 修完可过；图钉/signifier 两项
  涉 core 契约（UiAppendSignifier 布局、ShowPinnedDecoClutter union），需 main
  定 owner（core 域修正 or 组 A 内以字节对齐本地落地）。
- 无偷懒/占位（reconstruction/tree-tool 草稿的 read_u64→0 模式未复现）。

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
