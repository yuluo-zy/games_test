# groupA RE-ANALYSIS — system_tree（树工具）

dbid: tg_xa1 · 2026-10-16 · decomp: `groupA/tree-tool/decomp/`

## 1. 覆盖盘点（13 业务）

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x14226be70 | convert_preplaced_trees | O | 预置树→游戏树转换 |
| 0x14226a260 | hide_trees | O | 相机距离内隐藏树冠 + 历史记录 |
| 0x140650520 | impl$0::build | O | TreeToolPlugin::build |
| 0x142267430 | process_tree_cmds | O | 树命令处理器（prefab 串分派） |
| 0x1422686d0 | despawn_trees | O | α≤0.01 树收集待删除 |
| 0x142268dc0 | recalc_tree_position_on_terrain_height_change | O | 地形高变化→树 y 重算 |
| 0x141bc34f0 | replay_tree | O | 历史回放（ReplayBuffer 前端） |
| 0x1422689c0 | tree_move_bend | O | 树移动/弯折插值 |
| 0x141bd2510 | ui_delete_tree | O | 删除树 UI（tree_erase widget） |
| 0x14226b1a0 | ui_move_tree | O | 移动树 UI（拖拽+历史） |
| 0x14226a170 | random_tree_prefab | O | 随机树 prefab 路径选择 |
| 0x142269170 | ui_place_tree | O | 放置树 UI（tree_canvas widget） |
| 0x142269970 | place_tree | O | 树实体生成（place 命令消费） |
| — | reset_tree_tool (line 4) | U(内联) | 重置逻辑并入调用点 |

## 2. 关键契约（restorer 可落码）

### 共享容器/数据面（与森林/墙域交叉）
- **树记录表 0x48B 条目**（lVar6+0x1a8 基址）: {Arc/String prefab 指针对@+0x18.., 位集索引表@+0x38, u64 id@+0x40}；**0xa0B 树条目**（lVar6+0x100 基址）: {……, u32 记录索引@+0x94, …@+0x50 组件表}。
- **id→索引表**（0x14B/条目 {u32 id_hi, u32 索引}@+4, 变体@+0xc, 子索引@+0x10）+ **存在位集**（{ptr@+0x28, len@+0x38, mask 字}）—— 与 manage_tree_prefabs 同一套。
- TreeStorage（chapters/resources/tree_storage.rs.md 已章节化）：iter 0x140a1b100 / get 0x140a38460 / insert 0x140a38560 / remove 0x140a385a0 / next_index 0x140a38600。
- **WallSpatialHash::query_with_circle_callback** 单态：0x1425defd0（convert_preplaced）、0x1425df320（hide_trees）—— 树查询墙域空间哈希。
- 历史命令 0x58B = {u32 ref, u32 ref, u32 变体, u64 树 id, u64, u64, u64 位置打包, u64, u32, u32, u8}：变体 0=放置(place_tree)/1=移动(ui_move_tree release)/2=删除(ui_delete_tree/hide_trees)；History::add = 0x140a84ec0，edit 类型 = 0xc（`uStack_e8 = 0xc` 模式）。回放端 replay_tree 用 ReplayBuffer::front_mut_bucket（0x1409f1a80）。

### 函数行为
- **ui_place_tree (0x142269170)**：门禁 `mode==0x10`(16) && 全局 0x14362c098==0；widget 'tree_canvas'（0x143035108）；RaycastWorld::legacy_raycast（0x1408cc680）+ geo_src_to_edit_subject（0x140977f50）拒绝无效编辑；CursorTerrainRaycast::position；界内（0x140ad2950）→ clicked → push 0x18B 放置命令 {u64 id?, u64 打包位置, u32} 到 ResMut<Vec>（grow 0x140562db0）；界外 → is_active 时显示 signifier（format 串 UNK_143035150）；**水下重映射**（内联 remap_pos_y_if_in_water）：`y<0 → y = lerp(y/1.5, -1.2, 0)`（1.5=0x1429df1c0, -1.2=0x142a8cc64）。
- **place_tree (0x142269970)**：消费 0x18B 命令：`TreeStorage::next_index` → id；**确定性变体**：splitmix64(id)（常数 0xa0761d6478bd642f/0xe7037ed1a0b428db）拒绝采样 → `变体 = floor(10*frac(v))`；`FUN_14226a170(低字节)`（random_tree_prefab）选 prefab 路径 String（Arc 计数管理：LOCK/inc/dec）；`fastrand::f32` ×2 → 缩放 `lerp(0.4922, 1.0)`×2（0x142925870=0.4922, 0x14292e6b0=1.0；0x143035090=0.9 用于第二轴）；push 0x48B 树记录；History::add（0x58B, 变体 0, edit 0xc, 位置 0x1430351b8）。
- **random_tree_prefab (0x14226a170)**（byte param_1 + SessionMeta）：prefab 路径选择（返回 String）：
  - `0x140a9fa70`（季节标志 1）+ `is_halloween_any`（0x140b46890）→ `0x140ae8320`（标志 2）→ 三级分派：
  - 表：naked1_no_mushrooms(39B 0x14303526a) / naked1(26B 0x143035236) / naked2(26B 0x143035250) / branchy2(22B 0x1430351d0) / branchy1(22B 0x1430351e6) / autumn/branchy2(29B 0x1430351fc) / autumn/branchy1(29B 0x143035219)。
  - param_1 bit0 在 naked 族选 1/2；branchy 族选 2/1（交错）。
- **hide_trees (0x14226a260)**：双空间哈希遍历（0x18B + 0x30B 层）+ TreeStorage iter；树记录→位置；`Circle2d::with_expanded_diameter`（0x140c980c0）+ `contains_point`（0x140c980f0）判相机圈内 → 写可见性（组件字节 @uStack_a8+0xc+idx*0x10=0/1，u32 @puStack_a0+idx*4=滴答）；首次隐藏 → push 0x10B {u64,u64} 到隐藏列表（ResMut，grow 0x140562630）+ History::add（0x58B 变体 2, 位置 0x143035328/0x143035340）。
- **despawn_trees (0x1422686d0)**：param_1 = 树索引列表 {u32*@+0xf8, end@+0x100, flag@+0x108}；对每树取 0xa0B 条目→0x48B 记录→组件表；`α ≤ 0.01`（0x142960674=0x3c23d70a）→ push {u64,u64} 到输出 Vec（ResMut 0x10B）。**α 阈值 0.01**。
- **process_tree_cmds (0x142267430)**（37KB）：mode 门禁 `0x14092e6a0`(ActiveWidget::is_dragging 反) + `0x140a9fa70`/`0x140ae8320` 标志；对命令串做 **prefab 路径前缀匹配**（0x19/0x1a/0x27 长串 vs 'clearing/naked_tree/naked1_no_mushrooms'(0x143034f28 区) / 'clearing/naked_tree/naked1'(0x143034d30 区) / 'naked2'(0x143034d50 区) 等）→ 分派 spawn/替换/删除；log：**"Tree  already exists. Replacing it with a new tree"**（0x143034f4f 区）；TreeStorage::insert/remove + Entities::contains（0x140ceafa0）+ Commands::spawn_empty + random_tree_prefab 调用。
- **recalc_tree_position_on_terrain_height_change (0x142268dc0)**：|x|,|z| ≤ 65（0x142a83814, 掩 0x14292e980）→ `FUN_141246fb0`（地形高采样）→ `y = max(h, 0)` 写回组件（u32 打包）。
- **tree_move_bend (0x1422689c0)**：time_continuous_lerp（0x140c904e0）×2 → 位移 `clamp(dist, fVar9, fVar10)` 段（fVar9/fVar10 常量区 U）；写新位置 + 滴答。
- **ui_move_tree (0x14226b1a0)**：门禁 `mode==0xb`(11) || (mode==0 && widget 态 0)；树记录 prefab 路径前缀匹配（0x1430354e8 = 'clearing/...' 25B）判定可移动树；widget 定位 = 树位 + {0, 0.5}（0x142a80ce0）；UiSounds（hover/drag start/drag release/click 全接）；拖拽状态机（UiWidgetContext::clicked 0x14092df10 → switch）：case1 记录起点 / case2 拖动中（地形高 y=max(h,0)，界内 push 0x8B 命令）/ case3 释放 → push 0x20B 移动命令 {id, u64, x, y, z} + History::add（0x58B 变体 1, 位置 0x143035520）。
- **ui_delete_tree (0x141bd2510)**：门禁 `mode==9 && submode==6`；clearing_canvas_w_layer（0x14092e340，层 10）+ widget **'tree_erase'**（0x142ef5fe9）；**删除半径 = brush_size × 3.0**（0x1429cd078=3.0）；空间哈希圆内（0x20B 条目 {flag@+0x18, pos@+8, r@+0x14}）→ TreeStorage 全遍历收集 `dist < r×3` → push 删除列表 + History::add（变体 2, 位置 0x142ef6040）。
- **replay_tree (0x141bc34f0)**：ReplayBuffer::front_mut_bucket（0x1409f1a80）+ 0x140949170（buffer 前端）；重放树命令（spawn/移动/删除；内部复用 process_tree_cmds 的 bundle 构造 0x140b5cfc0/0x140b5d010/0x140b5f9f0 族 —— tree-tool crate 内部命令应用器）。
- **convert_preplaced_trees (0x14226be70)**：预置树数据（PlacementDataSsbo::position 0x140ad51a0）+ 6 栅格采样（0x140a14df0，同 forest）；`密度=clamp(Σ4,0,1) <= 0` 且附加栅格 0 且非形状近（0x1409ea800）→ `WallSpatialHash::query_with_circle_callback`（0x1425defd0，r=7.0 圈）无墙 → 转 0x48B 树记录（转换），History 相关（0x140a84ec0 链路）。

### build (0x140650520)
TreeToolPlugin::build：注册全部上述系统（标签 'crates/systems/tree-tool/src/lib.rs'）+ 资源 init（0x1400dd6f0 哈希 ×8 区 —— 8 个 TypeId 检查/插入；类型名 U）。

## 3. 小函数（手解）

- **0x140c904e0 time_continuous_lerp()**：无参，返回基于全局时间戳的插值因子（tree_move_bend 用）。
- **0x140a38600**：TreeStorage::next_index（`mov rax,[rcx+0x30]; lea rdx,[rax+1]; mov [rcx+0x30],rdx` —— 与章节一致；place_tree 中作 RNG 种子源）。
- **0x140c980c0 Circle2d::with_expanded_diameter** / **0x140c980f0 contains_point** / **0x140c9a630/0x140c9a730 Rectangle2d::contains_point/with_expanded_size**：utils 域几何（章节化）。
- **0x140caf380 fastrand::Rng::f32**（依赖）。
- **0x141246fb0**：地形高度采样（tree y 重算/移动共用），U 精确输入（网格/地形资源）。

## 4. panic / 串

| 地址 | 内容 |
|---|---|
| 0x142960674 区 | 0.01f32（despawn α 阈值） |
| 0x143034f4f 区 | "Tree  already exists. Replacing it with a new tree"（log，双空格为 %s 格式） |
| 0x14303526a 族 | 7 个 prefab 路径（见 random_tree_prefab 表） |
| 0x143035108 | 'tree_canvas'（放置 widget 名） |
| 0x142ef5fe9 | 'tree_erase'（删除 widget 名） |
| 0x1430351b0 区 | 'seed_cursor_icon'（seed 光标图标名） |
| 0x142ef5e98/0x142ef5e78 | ui_delete_tree 的 unwrap panic 位置（'crates/systems/tree-tool/src/ui_delete_tree.rs'） |
| 0x1430350f0/0x1430352e0 | place_tree/hide_trees 的 alloc 错误位置 |

## 5. 消费者线索

- 树命令 → history EditType 0xc（与 replay_tree 对偶）；audit：hide_trees/place_tree/ui_move_tree/ui_delete_tree 四系统写 history，变体 2/0/1/2。
- WallSpatialHash 三单态（0x1422785d0/0x1425defd0/0x1425df320）—— 树域复用墙域空间哈希；restorer 应把 query_with_circle 归墙域 API。
- 0x14362c098 全局门禁共用。
- place_tree 的 splitmix64 变体选择 → 树 prefab 变体（naked1/naked2/…）与随机缩放 {0.4922..1.0} —— 与 forest manage_tree_prefabs 的随机缩放 {0.3..0.75} **不同**（手放 vs 森林自动）。

## 6. U 项

- process_tree_cmds 命令变体全表（0x19/0x1a/0x27 串分派的精确语义分支）。
- tree_move_bend 的 fVar9/fVar10 常量区地址。
- 0x141246fb0 地形采样输入面。
- reset_tree_tool 内联体。
