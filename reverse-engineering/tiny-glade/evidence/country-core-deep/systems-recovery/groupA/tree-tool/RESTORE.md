# RESTORE — system-tree（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/tree-tool/`。
> 证据：`groupA/tree-tool/RE-ANALYSIS.md`、`groupA/tree-tool/decomp/`、BASELINE。

## 实现清单（13 业务）

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `impl$0::build` | 0x140650520 | 已实现（O 面） | 系统族注册 + 事件注册（8 个 TypeId 检查面 U） |
| `ui_place_tree` | 0x142269170 | 已实现（语义级） | 门禁 mode==0x10；界内（is_pos_inside）→ PlaceTreeCmd（0x18B）；界外 Display/Feedback；remap_pos_y_if_in_water（1.5/-1.2 常量） |
| `place_tree` | 0x142269970 | 已实现（语义级） | TreeStorage::next_index → splitmix64 确定性变体（floor(10*frac)）→ random_tree_prefab → 缩放 lerp(0.4922, 1.0)×2（第二轴 ×0.9）→ EditType::Tree 历史 + InsertTreeCmd |
| `random_tree_prefab` | 0x14226a170 | 已实现 | 7 prefab 路径表（0x14303526a 族）；bit0 交错；季节/is_halloween 三级分派 I |
| `process_tree_cmds` | 0x142267430 | 已实现（语义级） | 命令消费 + TreeStorage insert/remove + log "Tree  already exists…"（0x143034f4f）+ SpawnPrefabCmd + PlayTreeSoundCmd kind 0；prefab 前缀分派全表 U |
| `despawn_trees` | 0x1422686d0 | 已实现 | 标记树 → DespawnPrefabCmd；α ≤ 0.01 阈值（0x142960674）登记 |
| `recalc_tree_position_on_terrain_height_change` | 0x142268dc0 | 已实现 | OnTerrainChange → \|x\|,\|z\| ≤ 65 → 地形高采样 → y = max(h, 0) |
| `replay_tree` | 0x141bc34f0 | 已实现 | ReplayBuffer::front_mut_bucket → ReplayCommand::Tree 分派（Place→Insert/Move→Move；撤销放置→Delete，I） |
| `tree_move_bend` | 0x1422689c0 | 已实现（语义级） | time_continuous_lerp 位移插值面（常量区 U） |
| `ui_move_tree` | 0x14226b1a0 | 已实现（语义级） | 门禁 mode==0xb；界内 → MoveTreeCmd + EditType::Tree 历史；拖拽状态机细部 U |
| `ui_delete_tree` | 0x141bd2510 | 已实现（语义级） | 门禁 Eraser(Tree)；删除半径 = brush × 3.0（0x1429cd078）；光标圈内树收集 → DeleteTreeCmd + 历史 |
| `hide_trees` | 0x14226a260 | 已实现（语义级） | 相机圈判定面 + DeleteTreeCmd + 历史；可见性写面 U |
| `convert_preplaced_trees` | 0x14226be70 | 已实现（语义级） | SpawnPreplacedTrees 消费 + 墙冲突圈（7.0）+ DeleteTreeCmd；6 栅格密度面登记 |

## core 侧接线（delta）

- `resources/path_stroke.rs`（新）：PathDrawEvent {position, radius}。
- `resources/history/replay_cmd_buffer.rs`：`ReplayCommand::Tree(TreeReplayCmd)` 变体 + expand() + make_instant() 匹配臂。
- `resources/color_id.rs` 等（color 域见 color RESTORE）。

## 未实现项 / 待 reviewer

1. process_tree_cmds 的命令变体全表（0x19/0x1a/0x27 串分派精确分支）。
2. ui_place_tree 的 widget 'tree_canvas' 单击状态机（UiWidgetContext::clicked 0x14092e5f0 面）。
3. 0x141246fb0 地形采样输入面；tree_move_bend 常量区。
4. splitmix64 常量对（re-analyst 记 0xa0761d6478bd642f/0xe7037ed1a0b428db 族 vs 标准对，I）。

## 编译

`cargo check -p system-tree`（离线）通过。

## 修复轮 2（reviewer-A 必改 3.2/3.5/3.6 + coordinator）

- **splitmix64 机器码常量**：`z = x + 0xa0761d6478bd642f`、`v = (z ^ 0xe7037ed1a0b428db) * z`（128 位乘 hi^lo 折叠）、拒绝采样 `(u32)(v*10) < 6` 重试（种子 += 0x40ec3ac8f17ac85e）——替换标准 splitmix 常数对。
- **假删除链修复**：hide_trees/convert_preplaced_trees/ui_delete_tree 改真实 TreeStorage 遍历 + 索引（DeleteTreeCmd{id} 真实）；ui_move_tree 真实 id（去掉位置位冒充）。
- **core TreeEdit 补 Delete 变体**（0x58B 变体 2）+ TreeEditKind::Delete + 访问器；replay_tree 按 kind 分派（撤销放置=删除）。
- **真实载荷**：InsertTreeCmd{id, position, scale, prefab_name}；process_tree_cmds 真实 spawn（树组件束）+ insert 真实实体；despawn_trees α≤0.01 门控（TreeAnimation.alpha 字段）；remap 常量按字节（0x1429df1c0={-10.0,0.375}——reviewer 纠错）与公式（`(y/d)*t + (1-y/d)*0.0`）。

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
