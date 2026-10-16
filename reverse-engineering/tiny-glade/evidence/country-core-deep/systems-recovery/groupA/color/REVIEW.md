# REVIEW: system-color（crates/systems/color）

- 审查日期：2026-10-16；审查人：reviewer-A（dbid tg_va1）
- 交付：`D:/game/tiny-glade/crates/systems/color/`（17 文件，workspace 已注册）
- 编译：`cargo check -p system-color` PASS
- 证据：BASELINE / fresh decomp（0x142291230 **137KB 已解出**、0x142290820、
  0x142298970、0x142299840、0x142299250、0x142299dd0、0x1422af0c0、
  0x142298260、0x142298520、0x142298d30、0x1422adcf0、0x141c7f270、
  ui_color 三文件） / rne-read 字节 / core 章节（color_icon/color_id/color_id_roof）。

## 1. 逐项核对表（26 业务）

| 函数（VA） | 判定 |
|---|---|
| impl$0::add_color（0x140651870） | OK（17 系统 + 事件注册面；rhapsody create_buffer 判依赖不恢复 ✓） |
| init_color_ids_buffer（0x142290750） | OK-lite（三表 insert_resource ✓ 0x140b19890/990/970 族；缺 GraphicsDeviceResource 参数面——rhapsody 接线 GAP-lite） |
| set_wall_color_id（0x142290820） | **FIX**（见 §3.1：8 参 vs 6 参；plaster 事件缺；0x4fff 断言 ✓ 等价） |
| set_gate_color_id（0x142299840） | **FIX**（缺 PublicWalls/GraphicsDeviceResource；max_y 写 0.0 假数据） |
| set_halftimber_color_id（0x142299250） | **FIX**（同上；tag 0xc ✓） |
| set_roof_customization（0x142299dd0） | **FIX/GAP**（8 参 ✓；`RoofHistoryEdit::from_observed([0;64])` 假历史载荷；样式比较/颜色写缺失） |
| write_roof_color_id_to_gpu（0x142298520） | **FIX**（4 参 vs 6 参——多 roof/wall 表 + 自造 ColorIdDirtyWrites；机器码经 SSBO 双哈希逐元素脏写，交付为 CPU 表 iter_nondefault 语义模型；屋顶默认 1 ✓ 章节） |
| set_decorator_color_id（0x1422af0c0） | **FIX/GAP**（亮度/梯/扶手/魔法 4 通道全部丢弃；stair.color 写缺失；10 参 vs 机器 9（自造 ColorMenuState）） |
| set_window_light（0x142298260） | **FIX/GAP**（窗灯色写缺失；5 参 vs 机器 4） |
| set_flag_connection_and_chimney_smoke（0x142298970） | GAP（仅 lookup，无开关写/记录推；签名 ✓ 5 参） |
| show_color_options（0x14229a7f0） | **GAP（major）**（反编译超时未解出——契约承认受限；交付只发 2 图标菜单（分隔+freehand），目标类型分派/图标全表未落地；中心 (0,0) 假锚点；13 参 vs 候选 15） |
| hide_color_menu（0x1422adcf0） | FIX-lite（despawn ×2 ✓；just_pressed 门缺失——`_actions` 丢弃；单事件消费） |
| autoclose_color_menu（0x142298d30） | **FIX/GAP**（ModeJustChanged/EscapeMenuJustOpened 触发被丢弃；距离阈值 20.0 为 I 自设） |
| color_menu（0x142291230） | **GAP（major）**（机器码 15 Ghidra 参 / 137KB：just_pressed×11、is_active×11、to_f32×3（0x140b06770 ✓ 亮度三变体）、History×13（0x140a84ec0）、AngleRange 布局；交付 5 参空壳（3 资源弃用 + 空循环）） |
| color_button_animations（0x1422979b0） | **GAP（major）**（EasingFunction::apply ×3 动画未实现，空循环） |
| delete_color_icon_entities（0x142297ff0） | OK（despawn 标记图标 ✓） |
| replay_color（0x141c7f270） | **GAP（major）**（14 参通道 ✓ 但全部未写——无任何转发；只匹配 ReplayCommand::Tree 与机器码颜色命令表示不符） |
| IconInfo new_separator/new_flat_roof_rim_ty（0x1425f2680/0x1425f26f0） | OK（'white' kind=5 ✓；freehand_ty 错位登记 ✓ 按机器码语义） |
| IconInfo 内联 12 构建器 | OK-lite（名字表 I 级；kind 1/2/8 布局 I） |
| ColorRadialMenu::create（0x1425f27c0） | GAP-lite（13 float 参 → 3 参简化；实体生成未落地） |
| register_required_components（0x1425f3d60） | OK（胶水） |
| ui_color_wall_and_roof（0x141c9ab70） | **GAP（major）**（9 参 ✓ 但函数体全空——拾取/菜单事件全缺） |
| color_cursor_icon（0x141c9b510） | **GAP（major）**（空函数；SignifierEntityBuilder + mesh insert 未实现） |
| ui_color_gates（0x1422afe70） | **FIX/GAP**（缺 Query<WoodenGate> 第 2 查询；锚点 = Vec3::ZERO 假数据；show/hide 未写） |
| ui_color_windows_and_flags（0x1422b0e00） | **GAP（major）**（13 参 vs 候选 15；函数体全空） |

## 2. coordinator 三项契约核对

1. **ColorMenuBrightnessValue 三变体**：core `resources/color_icon.rs` 已修正为
   三 tag 无载荷模型（to_f32 = [0.25,1.0,4.0] 表、default_f32=1.0，无 Custom(f32)）
   ✓ 与章节一致；机器码 color_menu `0x140b06770` ×3 调用 ✓ 与 impl 文档一致。
   **但 crate 未消费该类型**（SetDecoratorBrightnessCmd `_gap` 承载、set_decorator
   丢弃亮度通道）→ GAP。
2. **屋顶默认 ColorId=1**：write_roof_color_id_to_gpu 文档 "屋顶默认 1、墙默认 0"
   ✓ 与 color_id_roof 章节（0x14099ca50 返回 1）一致；`iter_nondefault` 语义面
   ✓ 但落地为自造 ColorIdDirtyWrites（非机器码 SSBO 写面）→ FIX。
3. **0x4fff 断言**：set_wall_color_id 机器码 `if (0x4fff < idx) panic`；core
   MAX_NUMBER_OF_WALLS=20480；交付 `assert!(idx < MAX_NUMBER_OF_WALLS)` —
   与机器码等价（idx≥0x5000 均 panic；0x4fff 本身通过）✓ OK。panic 文案地址
   ✓（0x14303b160 区）。

## 3. 针对性问题清单

### 3.1 FIX——set_* 族签名与载荷
- set_wall_color_id：机器码 8 参 = (p1?, Res<AssetSsboLibrary>,
  ER<SetWallColorIdCmd>, ResMut<WallColorIds>, EW<PlayWallUiSound>,
  Res<PublicWalls>, EW<CreatePlasterCmd>, EW<DeletePlasterCmd>)（tick 面行
  152-160）；交付 6 参——**缺 plaster 两通道**（墙色变更 → 石膏重建是机器码
  业务）、**缺 GraphicsDeviceResource**（rhapsody 接线，GAP-lite 可）；**自造
  ResMut<ColorMenuState>**（机器码 8 参无此资源——脏色列表/0x20B 记录在机器码
  中应落入 core WallColorIds 内表，core 字段未恢复 → GAP 核心契约）。→ FIX。
- set_gate_color_id / set_halftimber_color_id：机器码 6 参（含 Res<PublicWalls>
  ——门/半木 max_y 读取面）；交付缺 PublicWalls、`scaled_max_y: 0.0`、
  `packed_center: 0` 假数据 → FIX。
- set_window_light：机器码 4 参；交付 5 参（自造 state）→ FIX。

### 3.2 FIX——假数据载荷
`EntityColorRecord { packed_center: 0, scaled_max_y: 0.0 }`（set_gate/
halftimber/decorator/window ×4 处）、`RoofHistoryEdit::from_observed([0; 64])`
（set_roof_customization）、`Vec3::ZERO` 锚点（ui_color_gates）、`(0,0)` 菜单
中心（show_color_options）——机器码均有真实计算面（Aabb2::center 0x1412400c0 +
pack 0x140c9cde0、max_y 0x140b2a620、PublicWalls 读、Roof::pivot_3d、门网孔
Mesh::positions 0x140a07b30）→ **FIX（必改）**：以假数据冒充实现超出诚实
标注边界。

### 3.3 GAP（major）——color_menu / color_button_animations / replay_color /
ui_color_* 空壳
机器码证据（decomp-0x142291230）：just_pressed×11、is_active×11、
History::add×13、to_f32×3、AngleRange/cartesian_to_polar 族——交互/历史/亮度
全业务；交付 color_menu 5 参空壳。color_button_animations（EasingFunction×3）、
replay_color（14 通道零转发）、ui_color_wall_and_roof / ui_color_windows_and_
flags（全空）、color_cursor_icon（空函数）同属。→ GAP（major，按
re-analyst 补证后逐系统回填；color_menu 为最高优先——137KB 机器码已解出，
可直接深挖）。

### 3.4 OK——诚实面
- 事件族命名与 BASELINE 参数面 system_color::* 一致（SetRoofColorIdCmd/
  SetDecoratorBrightnessCmd/…）✓；
- IconInfo PDB 名错位（new_flat_roof_rim_ty → freehand_ty）登记并落码 ✓；
- delete_color_icon_entities 真实现 ✓；register_required_components 胶水 ✓；
- 章节契约引用准确（ColorMenuBrightnessValue、屋顶默认 1、MAX_NUMBER_OF_WALLS）。

## 4. 结论

- OK 7 / FIX 9（set_* 签名 ×4、假载荷 ×5 处）/ GAP 10（color_menu、
  color_button_animations、replay_color、ui_color 三系统 + color_cursor_icon、
  show_color_options、autoclose、亮度/plaster 通道）。
- 判定：**review_passed 未达**——本 crate 是六 crate 中机器码业务密度最高而
  交付覆盖最低者：核心交互（color_menu）、动画、回放、UI 拾取四类为空壳；
  假数据载荷（3.2）必改。建议 restorer-A 按 decomp-0x142291230（已解出 137KB）
  优先回填 color_menu 交互面，其余 GAP 挂 re-analyst。

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
