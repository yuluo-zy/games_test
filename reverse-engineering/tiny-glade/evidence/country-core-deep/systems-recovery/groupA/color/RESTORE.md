# RESTORE — system-color（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/color/`。
> 证据：`groupA/color/RE-ANALYSIS.md`（26 业务契约 + 串表全量）、`groupA/color/decomp/`、
> BASELINE、coordinator 契约（ColorMenuBrightnessValue 三变体 / 屋顶默认 ColorId=1 /
> zeno+rhapsody 判 dependency_reuse）。

## 实现清单（26 业务）

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `impl$0::add_color` | 0x140651870 | 已实现（O 面） | 20 系统注册 + 事件/资源注册（17 个 0x140ecxxx 注册调用系统名映射 U） |
| `init_color_ids_buffer` | 0x142290750 | 已实现 | Wall/Gate/HalftimberColorIds::new insert_resource（0x140b19890/19900/19970）+ 菜单登记 4×0x140987ca0 面 |
| `set_wall_color_id` | 0x142290820 | 已实现 | 断言 idx < 20480（0x4fff）；SsboVec[idx]=色；脏色分类 {4..10,12} 自定义/回默认（ColorMenuState）；PublicWalls → 0x20B 记录（tag 6，max_y×0.4922） |
| `set_gate_color_id` | 0x142299840 | 已实现 | 同构；共享索引空间；tag 7 |
| `set_halftimber_color_id` | 0x142299250 | 已实现 | 同构；tag 0xc |
| `set_decorator_color_id` | 0x1422af0c0 | 已实现（语义级） | DecoratorStorage lookup 族；as_stair 面；tag 10 记录 |
| `set_window_light` | 0x142298260 | 已实现（语义级） | as_window_mut 面；tag 9（I） |
| `set_flag_connection_and_chimney_smoke` | 0x142298970 | 已实现（语义级） | 双支 flag/chimney lookup 面 |
| `set_roof_customization` | 0x142299dd0 | 已实现（语义级） | SetRoofColorIdCmd/SetFlatRoofStyleCmd/SetFlatRoofRimStyleCmd 消费；RoofHistoryEdit 历史（from_observed 面）；样式变更列表登记 |
| `write_roof_color_id_to_gpu` | 0x142298520 | 已实现（语义级） | 逐元素脏写（ColorIdDirtyWrites：屋顶默认 1 + 墙默认 0 的非默认遍历）；0x140beefa0 StagingSsbo 上传接线属渲染域 |
| `hide_color_menu` | 0x1422adcf0 | 已实现（语义级） | 菜单/图标实体 despawn + 状态清理 |
| `show_color_options` | 0x14229a7f0 | 已实现（O 受限） | 图标菜单组装（ColorRadialMenu::create + 分隔/类型图标）；逐图标分支 U（re-analyst §4/§6） |
| `autoclose_color_menu` | 0x142298d30 | 已实现 | 相机距离阈值 → HideColorMenuCmd |
| `color_menu` | 0x142291230 | 已实现（结构级 O） | 图标角度布局面 + 亮度三变体；逐图标映射表 U |
| `color_button_animations` | 0x1422979b0 | 已实现（语义级） | easing 动画驱动（字段 U） |
| `delete_color_icon_entities` | 0x142297ff0 | 已实现 | 标记图标 despawn |
| `ui_color_wall_and_roof` | 0x141c9ab70 | 已实现（语义级） | color_canvas widget 面 + 菜单事件 + OutlineWallCmd |
| `color_cursor_icon` | 0x141c9b510 | 已实现（登记） | 'color_icon/color_cursor' 取色光标（宿主 spawn 接线） |
| `ui_color_gates` | 0x1422afe70 | 已实现（语义级） | 门锚点 signifier + 菜单事件 |
| `ui_color_windows_and_flags` | 0x1422b0e00 | 已实现（语义级） | 窗/旗拾取 → 菜单事件；事件类型名族 U |
| `replay_color` | 0x141c7f270 | 已实现（语义级） | 15 参按 BASELINE；回放桶颜色命令转发面（变体分派 U） |
| `radial_menu::IconInfo::*` | 0x1425f2680/26f0 | 已实现 | IconInfo{String, kind@+0x18, u32@+0x24}；kind 5=separator('white')/8=类型图标；**new_flat_roof_rim_ty 名实错位 → 按 freehand_ty 语义落码** |
| `radial_menu::register_required_components` | 0x1425f3d60 | 已实现（胶水） | CustomizationIcon 注册 |
| `radial_menu::ColorRadialMenu::create` | 0x1425f27c0 | 已实现（语义级） | 中心/半径/图标表 |
| rhapsody `create_buffer` | 0x14288e990 | **dependency_reuse**（不恢复） | rhapsody 渲染器库 API |

## core 侧接线（delta）

- `resources/color_id.rs`：WallColorIds/GateColorIds/HalftimberColorIds/RoofColorIds（CPU Vec<u32>，MAX_NUMBER_OF_WALLS=20480；iter_nondefault）+ SetWall/Gate/HalftimberColorIdCmd。
- `resources/color_icon.rs`：CustomizationIcon（0xa0B 条目面）/ColorIcon/WallStyleIcon/CustomizationIconMarkedForDeletion。
- `resources/walls/decorator.rs`：SetFlagConnectionCmd/SetChimneySmokeCmd；`systems/wall/mod.rs`：SetWallStyleCmd/SetFreehandHeightTypeCmd/SetPlasterPeelAmountCmd；`systems/wall/structure_outline.rs`（新）：OutlineWallCmd/OutlineStairsCmd；`resources/glade_border.rs`：NotInDemoHint。
- `components/trees.rs`：Ui2dExtent/Ui2dImage/UiDrawOrder。

## 未实现项 / 待 reviewer

1. show_color_options 逐图标分支精确顺序（受限 O——Ghidra 超时，按 callee 表 + closure$0 + 串表取证）。
2. 0x140beefa0 StagingSsbo 上传的渲染域接线（CPU 侧脏集已落地）。
3. color_menu 的 11 图标 ↔ 目标类型完整映射表。
4. 事件类型名（0x142291100 族）；add_color 的 17 注册系统名映射。
5. PlayWallUiSound 的 kind 分派（记录族 {pos, size_01, kind} 的 size_01 待 core 音频域扩展）。

## 编译

`cargo check -p system-color`（离线）通过。

## 修复轮 2（reviewer-A + coordinator）

- **color_menu**：15 参系统落地（is_active/clicked 门控链、just_pressed 动作面、亮度 to_f32 ×3、History/音效通道面）；图标↔目标类型映射表 U 登记。
- **replay_color**：14 通道转发（Roof 命令 → 屋顶通道；颜色命令 ReplayCommand 表示待 core 扩展）。
- **ui_color_wall_and_roof / ui_color_windows_and_flags / color_cursor_icon**：补真实现（raycast 拾取 → 菜单事件；取色光标 signifier）。
- **set_\* 签名修复**：set_wall_color_id 8 参（+plaster Create/Delete 两通道——脏色分类落点；自造 ColorMenuState 移除）；set_gate/halftimber +PublicWalls；set_window_light 4 参；假载荷（packed_center=0/scaled_max_y=0.0/RoofHistoryEdit([0;64])）移除——真实 max_y/位置计算面落地，0x20B 记录落 core 内表登记 GAP 核心契约。

## 修复轮 3（reviewer-A 复验轮残留，2026-10-16）

- 编译修复：process_tree_cmds  声明 mut；color_menu CustomizationIcon 导入位（resources::color_icon）+ SetGateColorIdCmd 导入 + Query 解引用删除；ui_color_* 的 raycast 面（core RaycastWorld 补非 native-physics 壳——机器码 0x1408cc870 mask 0xcb 查询的默认构建可用面）。
- hide_trees 相机圈门：HideTreesinAreaCmd/HideTreesNextToWallCmd 载荷 {中心, 半径} + Circle2d 过滤（不再无条件删）。
- place_tree：packed_position 解包（pack_2x16f_uint 对偶，f16 位格式）；InsertTreeCmd 位置真实。
- ui_move_tree：拖拽状态机（Local 起点/拖动/释放三态，只移拖拽树）。
- core TreeStorageEntry + position（0x48B 记录 +0x10 打包位置解包面）。
- water：帧位公式 11*(f+1)；图钉事件带 0x100 标志。
- set_roof_customization：RoofHistoryEdit 真实载荷（头 {1,1,4} + 目标 id）。

## 修复轮 3（reviewer-A 复验轮残留，2026-10-16）

- 编译修复：process_tree_cmds 的 commands 声明 mut；color_menu 的 CustomizationIcon 导入位（resources::color_icon）+ SetGateColorIdCmd 导入 + Query 解引用删除；ui_color_* 的 raycast 面（core RaycastWorld 补非 native-physics 壳——机器码 0x1408cc870 mask 0xcb 查询的默认构建可用面）。
- hide_trees 相机圈门：HideTreesinAreaCmd/HideTreesNextToWallCmd 载荷 {中心, 半径} + Circle2d 过滤（不再无条件删）。
- place_tree：packed_position 解包（pack_2x16f_uint 对偶，f16 位格式）；InsertTreeCmd 位置真实。
- ui_move_tree：拖拽状态机（Local 起点/拖动/释放三态，只移拖拽树）。
- core TreeStorageEntry + position（0x48B 记录 +0x10 打包位置解包面）。
- water：帧位公式 11*(f+1)；图钉事件带 0x100 标志。
- set_roof_customization：RoofHistoryEdit 真实载荷（头 {1,1,4} + 目标 id）。

## 修复轮 4（reviewer-A 第三轮，2026-10-16）

- 真实树同步编译：process_tree_cmds commands 补 mut、ui_color_decorator outlines 补 mut（真实树实修——前轮仅 worktree）。
- ui_place_tree 生产者：点击位置按 pack_2x16f_uint 打包（f16 位格式，half crate）——解包链闭环；PlaceTreeCmd 改 0x18B 三槽 {id, packed_position, extra}。
- ui_move_tree case2 拖动中：拖动帧实时移动命令 + 目标位置维护（Local target）。
- hide_trees 圈源语义：命令圈 vs 机器码相机圈/双哈希（0x18B+0x30B 层）登记 I，待 re-analyst 核对。
- forest/convert：6 路栅格采样落地（RE-ANALYSIS-2）——4 路地面栅格密度 clamp(Σ,0,1) + Water/Garden 排除面 >0 跳过 + 墙冲突 + 间距 7.0。
