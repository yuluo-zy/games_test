# RESTORE — R6 余（roofs 视觉方法 + roof_ridge）+ R7（stairs 视觉/碰撞衔接）

> restorer-roofs-stairs，2026-10-16。承接 R6 剩余 + R7 专项（country-core 团队动态派发；
> 前 restorer-interaction 上下文将尽交接）。工作流：业务逻辑分析 → 整理 → 按 PDB 骨架实现 Rust。

## 一、R6 剩余（roofs 视觉方法 + roof_ridge）

### 1. `resources/roofs/roof_visual_shape.rs`（6 原函数全闭合）

| 原函数（地址） | Rust 位置 | 证据 |
|---|---|---|
| `Roof::get_height_01_unclamped_from_ws`（0x1408e4790） | `impl Roof::get_height_01_unclamped_from_ws`（roof_visual_shape.rs） | O 公式级：roofs-rt/ALGORITHMS.md §5.3 + decomp-0x1408e4790.txt（常量 −0.3 噪声系数/0.5 尺度/0.1 平顶阈值/12·0.4/16·9 高度基） |
| `Roof::get_roof_shape_from_pos_ws_clamped`（0x1408e4750） | `impl Roof::get_roof_shape_from_pos_ws_clamped`（同文件） | O：h01 = 上式 → clamp[0,1] → get_roof_shape(h01) |
| `Roof::get_roof_shape`（0x1408e4920） | `impl Roof::get_roof_shape`（同文件） | O = `system_roof::surface::section`（surface.rs get_roof_shape，3,004 案例 0 差异；q=1.5p+(1−p)0.5，t^(1/q) 混合） |
| `get_roof_expanded_top_and_bottom`（0x140a880c0） | `pub fn get_roof_expanded_top_and_bottom`（同文件） | O 反编译 `roof-tiles/0x140a880c0.c` + disassembly：矩形 bottom={0.28+dim+eave_ws}×2/top={dim·ridge+(1−ridge)·0.1, 0.1}(ridge_dir 选轴)；圆 bottom=((eave_ws·1.2+0.28)·0.5+r)/top=0.2 |
| `RoofBoundsDims::as_circle`（0x140a88200）/`as_rectangle`（0x140a88240） | `RoofBoundsDims::{as_circle, as_rectangle}`（同文件） | O 手解（rne-read --hex）+ roof-tiles/0x140a8820{0,40}.c |

**RoofBoundsDims 布局修正**：`get_roof_expanded_top_and_bottom` 输出 = `{tag@0, bottom@+4,
top@+8}`（矩形 bottom/top 各 Vec2，圆 bottom/top 各半径、top 恒 0.2）。旧章节把圆 +4/+8 记为
"圆心/半径"为误读，已按 `0x140a880c0.c` 纠正为 `Circle{bottom_radius, top_radius}` /
`Rectangle{bottom, top}`。

### 2. `resources/roofs/roof_ridge.rs`（新建；7 业务 + 1 胶水全闭合）

| 原函数（地址） | Rust 位置 | 证据 |
|---|---|---|
| `RoofRidgeDims::to_world_space`（0x1408e2750） | `RoofRidgeDims::to_world_space` | O = reconstruction/roof/ridge.rs `ridge_to_world_space_observed`（14,776 案例 0 差异），委托共享 `system_roof::ridge` |
| `RoofRidgeDims::get_inner`（0x1400885d0） | `RoofRidgeDims::get_inner` | O：原 `return *param_1` 首字段拷贝 |
| `RoofRidgeDims::max_dim`（内联行 27） | `RoofRidgeDims::max_dim` | O：SSE MAXSS 语义（NaN 优先） |
| `RoofRidgeDims::from`（0x1408e2690） | `RoofRidgeDims::from` | O = `ridge_dims_from_observed`（tag≠1 → {0.1,0.1}） |
| `RoofRidgeDims::calc`（0x1408e2700） | `RoofRidgeDims::calc` | O = `ridge_dims_calc` |
| `calc_roof_tip_center_ls`（0x1408e30b0） | `pub fn calc_roof_tip_center_ls` | O = `calc_roof_tip_center_ls_observed` |
| `inverse_ridge_01_param`（0x1408e30f0） | `pub fn inverse_ridge_01_param` | O = `inverse_ridge_01_param`（NaN→panic） |
| `to_world_space::closure$0`（0x1408e2eb0，胶水） | 已并入 to_world_space（闭包胶水，不计独立业务） | — |

### 3. 屋顶胶水（Bevy register_required_components / default / clone_behavior，不计业务）

- `resources/roofs/roof_tiles.rs`（新建）：`RoofTile` 组件声明（clone_behavior 0x140002370 = 共享空桩；register_required_components 0x140903f40 标准 Bevy 模式）。
- `resources/roofs/dormer_tiles.rs`（新建）：`DormerTiles` 资源（impl$1::default 派生胶水）。
- `resources/roofs/roof_buffers.rs`（新建）：`RoofBuffers` 资源（0 原函数，字段语义 U 待取证）。
- `resources/roofs/mod.rs`：登记 3 新模块 + pub use（roof_ridge/roof_tiles/dormer_tiles/roof_buffers + RoofBoundsDims/get_roof_expanded_top_and_bottom）。

## 二、R7（stairs 视觉/碰撞衔接）

### 4. `resources/stairs/structural_graph.rs`（新建；7 业务 + 1 胶水全闭合）

| 原函数（地址） | Rust 位置 | 证据 |
|---|---|---|
| `StairsStructuralGraph::clear`（0x140869a30） | `clear` | O decomp-0x140869a30（清图 + 双向映射 + 高 + connections + 边索引） |
| `StairsStructuralGraph::upsert_span`（0x140869d20） | `upsert_span` | O decomp-0x140869d20 + ALGORITHMS §3.4（旧 span 解绑→邻边入 connections→逆映射移除→remove_node 断言→高表移除；新 collider 挂接 4 表） |
| `StairsStructuralGraph::upsert_support_source_collider`（0x140869f90） | `upsert_support_source_collider` | 合同语义（Ghidra varnode 失败）：只加节点/高/visited |
| `StairsStructuralGraph::add_edge_by_span`（0x140869ff0） | `add_edge_by_span` | O decomp-0x140869ff0（两 span collider 加边，已存在早退；边索引写 0） |
| `StairsStructuralGraph::add_edge_by_collider`（0x14086a260） | `add_edge_by_collider` | O decomp-0x14086a260（排序 {slot,index,generation} + 索引写 0） |
| `StairsStructuralGraph::remove_node_by_collider`（0x14086a320） | `remove_node_by_collider` | O decomp-0x14086a320（删高→邻边入 connections→remove_node 断言→逆映射清理断言） |
| `StairsStructuralGraph::disconnect_node_by_collider`（0x14086a4c0） | `disconnect_node_by_collider` | O decomp-0x14086a4c0（枚举边移除，保留节点） |
| `impl$8::default`（胶水） | derive Default | — |

**节点键**：`StairColliderHandle`（= `SubworldColliderHandle` {index,generation,slot}，
排序 {slot,index,generation}，O add_edge_by_collider 排序）。`From<SubworldColliderHandle>`
与碰撞域互操作。

### 5. `resources/stairs/visual.rs`（新建；4 业务核心 + RtWorld/mesh 胶水登记）

| 原函数（地址） | Rust 位置 | 证据 |
|---|---|---|
| `MagicSupports::clear`（0x140aa8210） | `MagicSupports::clear`（存储核心：清空集合） | O decomp-0x140aa8210（RtWorld/mesh atlas 句柄释放胶水已登记） |
| `StairsVisualState::remove_edge`（0x140aa8620） | `remove_edge`（删边 + 脏字节置位） | O decomp-0x140aa8620（RtMesh 撤除/Arc drop 胶水登记） |
| `StairsVisualState::reset_edge`（0x140aa8a60） | `reset_edge` | O decomp-0x140aa8a60（v1 force_rebuild 名错，本实现重置存储） |
| `StairsVisualState::force_rebuild`（内联行 132） | `force_rebuild` | I：= reset_edge 存储语义 |
| `impl$6::register_required_components`（0x140aacd30，胶水） | `StairsVisualComponent` 声明 | — |

**登记待接线（胶水）**：remove_edge/reset_edge 的 RtWorld RtMesh 删除、mesh atlas 句柄
释放与 DirtyCow 交互属 render/rt 域（库内未建 `resources/rt`），本文件只实现边视觉存储的
增删核心 + 脏字节（+0x30，O remove_edge 处理末置 1）。

### 6. `resources/stairs/stairs_collider.rs`（新建；1 胶水）

`StairsCollider` 标记资源（rapier 边界；实际注册/撤除在 collision 域 R1 与 structural_graph）。

### 7. `resources/stairs/stairs_removed_supports.rs`（新建；1 业务全 serde 胶水）

`StairsRemovedSupports` = `HashMap<u32,u32>`（键=edge1、值=edge2，O replay_stairs
remove_entry(edge1,edge2)）；serde derive（serialize 0x140b3bad0）；record/erase/contains。

### 8. `resources/stairs_walls.rs`（新建；1 业务 + 4 胶水）

`StairsPillarProposals`（HashMap<u64, Vec<[u8;44]>>）：`StairsPillarProposals::add`
（0x140901270，O 反编译 7KB）命中 key append 44B 记录 / 未命中插入单元素桶。44B 字段语义 U。

### 9. `systems/history/replay_stairs.rs`（新建；1 业务）

| 原函数（地址） | Rust 位置 | 证据 |
|---|---|---|
| `replay_stairs`（0x140af75b0） | `pub fn replay_stairs` | O decomp-0x140af75b0 + history/ALGORITHMS §2.6（tag 10 前桶过滤；动画门控 elapsed<max(duration/speed, 下限)→推进；undo==0 → StairsRemovedSupports insert、!=0 → remove_entry；查附挂装饰 + CachedDecoratorTransforms → PlayStairsAudioCmd；完成命令逆序移除） |

**命令模型**：`StairsReplayCmd`（0x30B：tag/flags@+8/duration@+0xc/elapsed@+0x18/undo@+0x1c +
边记录 edge1/edge2）新增于 `resources/history/stairs_history_edit.rs`；`ReplayCommand::Stairs`
变体 + `ReplayBucket::expand`（EditType::Stairs → before−after 差集边）新增于
`resources/history/replay_cmd_buffer.rs`（共享基础设施最小增改，报告显式标注）。

## 三、依赖边界（共享/他域）

- **roof 几何**：`get_roof_shape`/height/ridge 数值委托共享 `system-roof`（surface/ridge/parameters），
  country-core 只留组件投影 + 谓词 + 噪声（roof_noise 已在卡A3 建）。
- **replay_stairs**：消费 `resources/walls/decorator_storage::DecoratorStorage`（lookup_decorator_info +
  DecoratorInfo +0x11 判别过滤）、`cached_decorator_transforms::CachedDecoratorTransforms`
  （键=edge1）、`systems/audio::PlayStairsAudioCmd` 事件；StairsRemovedSupports 由
  `register_history_events` 初始化。
- **structural_graph**：`systems/collision::world::SubworldColliderHandle` 转换（不改碰撞域文件）。
- **replay_pop_bucket 完整版**：当前 `pop_empty_replay_bucket` 只弹空桶；应用楼梯 diff
  （0x140b38250 装饰/楼梯 diff 恢复）为共享 history 域残留（卡C 已登记），本批登记不实现。

## 四、待取证（U，诚实标注）

1. `RoofBoundsDims` 圆字段消费端语义（旧章节"圆心/半径" vs 本批 bottom/top）——按
   `get_roof_expanded_top_and_bottom` 反编译定案为 bottom/top，系统消费端待 render 域对账。
2. `StairsStructuralGraph` +0x120/+0x180 两附加表语义、+0x48 边索引表精确类型
   （本批建模 `HashMap<(Collider,Collider),u8>` 写 0 语义）；collider 句柄在 upsert/remove
   面的字节宽（decomp 8B 读 vs add_edge_by_collider 12B 排序，本批统一 12B）。
3. `visual.rs` MagicSupports/StairsVisualState 的 RtWorld RtMesh/mesh atlas 句柄字段与释放链。
4. `stairs_walls.rs` 44B 提议记录字段语义。
5. `StairsReplayCmd::DEFAULT_DURATION`（0.1）与 `REPLAY_DURATION_FLOOR`（0.1，= `_UNK_142930d70`）
   原值；PlayStairsAudioCmd 事件载荷逐字节映射（decomp @+8 写入 undo 派生字节，语义待音频域）。
6. `stairs_collider.rs`/`roof_buffers.rs` 实际注册面（渲染域接线登记）。
7. `replay_stairs` 完成命令移除索引语义（本批按索引逆序 remove；原 memmove 压缩 + 判别 drop）。

## 五、测试/编译

- `cargo check -p country-core`：**通过**（7 警告全在他域：wall/collision/audio/save/meta_commander，
  非本模块；期间 wall 组在途瞬时错误已恢复）。
- 全库 `cargo test --lib` 当前被 wall 组在途测试错误打断（calc_shape_height/snapping_*/ui_bulldozer
  等，非本模块）；本模块逻辑经临时集成测试验证后删除：
  - roof_visual_shape（height_01 有限性/rect 截面/expanded rect+circle）✓
  - roof_ridge（from 轴选择/最小维/inverse 夹紧/world point）✓
  - structural_graph（upsert 绑定/同 span 解绑/remove 逆映射/disconnect 保留节点）✓
  - visual + removed_supports + pillar proposals ✓
  - replay_stairs（redo record / undo erase / 动画门控 / 桶 pop 链）✓
- 测试文件 `crates/country-core/tests/temp_r6r7_check.rs` 验证后已删除。

## 七、reviewer 终验 FIX 吸收（2026-10-16，REVIEW-R6R7.md 5 项全修）

| FIX | 裁决/修复 | 落点 |
|---|---|---|
| 1. `flat_roof_height_is_zero_cap`（NaN） | **测试断言错**——0x1408e4790 平顶不进入 cap 分支，分母恒 `(0+floor)−floor=0` → 原版确产 ±inf/NaN（"unclamped" 语义）；测试改为断言非有限 + clamped 入口仍出截面 | roof_visual_shape.rs |
| 2. `inverse_ridge_clamps_and_rejects_nan`（ok>1.0） | **测试断言错**——`(5−0.1)/(4−0.1)=1.256` 夹紧 1.0；改为断言 ==1.0/==0.0/NaN→Err | roof_ridge.rs |
| 3. `disconnect_keeps_node_removes_edges`（edge_index expect panic） | **实现忠实原版**（add_edge_by_span 未命中索引 panic @0x142a96bd0）；新增 `register_edge_index`（填充源 U，供装配面预置）+ 测试预置 | structural_graph.rs |
| 4. `redo_records...`（buffer.is_empty()） | **bucket 弹出归属** = 独立系统 `pop_empty_replay_bucket`（原版 replay_pop_bucket 0x140b38250 前桶空时）；测试接线 `(replay_stairs, pop_empty_replay_bucket).chain()` | replay_stairs.rs 测试 |
| 5. `animation_gating...`（off-by-one） | **原版"先判后加"**：`if (elapsed <= required) { elapsed += dt; }`（O decomp-0x140af75b0）——`advance` 判式 `<=` 修正 + 测试 4 帧（0.0/0.2/0.4 → 0.6>0.5 处理） | stairs_history_edit.rs `advance` + 测试 |

**复验实测**：`cargo test -p country-core --lib` = **1709 过/1 败**（唯一失败
`geometry/sphere::tests::uv_formula_matches_the_closure` 为历史 pre-existing，非本模块）。
本模块 16 测试全过（roof_visual_shape 4 / roof_ridge 4 / structural_graph 4 / replay_stairs 2 /
stairs visual 2）。`cargo check -p country-core` 通过（6 警告全他域）。

## 八、库内可闭合缺口闭合（2026-10-16，coordinator 派 3 项）

### 8.1 roof_shape 剩余 ~13 谓词/几何（借 system_roof + utils，不重复实现）

| 原函数（地址） | Rust 位置 | 实现/委托 | 证据 |
|---|---|---|---|
| `GableRoofStyle::is_stepped`（0x140833230）/`is_overhang`（0x1408c5680） | `GableRoofStyle` 枚举 + 谓词（roof_shape.rs） | tag==0 / tag!=0（serde "Matching"/"Override"） | O 手解 + wall 域共享去重对照 |
| `FlatRoofStyle::is_wooden`（0x1408e31a0）/`pattern_id`（内联 86） | `FlatRoofStyle([u8;12])` + is_wooden/pattern_id | tag0→other、tag1→false、tag2→field@+1 | I 手解（0x1408e31a0 NotFound，章节语义） |
| `Roof::profile_curve_normalized`（0x1408e3420） | `Roof::profile_curve_normalized` | 委托 `system_roof::surface::profile_curve_normalized` | O 209 案例 |
| `Roof::roof_tip_offset_xz`（0x1408e3780） | `Roof::roof_tip_offset_xz` | 委托 `system_roof::ridge::roof_tip_offset_xz_observed` | O（roof-tiles .c） |
| `Roof::profile_curve_ws`（0x1408e3d40） | `Roof::profile_curve_ws` | 委托 `system_roof::surface::world_profile_curve` | O 1,512 全链案例 |
| `Roof::rotation_yaw_radians`（0x1408e3b10） | `Roof::rotation_yaw_radians` | rect `atan2(−frame.z, frame.x)`；circle `−π/2 − CircularF32(center_y)`（常量 _UNK_142a10ff0=−π/2 rne-read） | O decomp + 常量 rne-read 0xbfc90fdb |
| `Roof::roof_shape_at_intersection_y`（0x1408e3820） | `Roof::roof_shape_at_intersection_y` + `RoofIntersectionSection` | y==floor→t=0 截面；越界→OutOfBounds(tag 2)；否则 `t=(y−floor)/cap` 截面 | O decomp（y==floor 直接拷帧体近似登记） |
| `ridge_length_01_from_rectangle_ratio`（0x1408e3b60） | `ridge_length_01_from_rectangle_ratio` | ratio=max(w/h,h/w)、u=clamp((max_dim−2.25)/0.94736844)、blend=(1.5·smooth+ratio−1.5)/1.5、smoothstep(clamp) | O decomp + 常量 rne-read（_UNK_142aa50dc/1429df180） |
| `roof_height_ws_from_wall_shape`（0x1408e3c70） | `roof_height_ws_from_wall_shape` | 尺寸归一 → `EasingFunction::SineInOut`（tag2 @0x142aa5938）→ 高度带（circle 10/4、rect 6/3）钳[0,12] | O decomp + 常量 rne-read（FUN_140c95ef0=utils easing apply） |
| `roof_height_01_from_wall_shape`（内联 444） | `roof_height_01_from_wall_shape` | = height_ws/12 钳[0,1] | I（inline） |
| `RoofShapeParams::eave_length_ws`（0x1408e3c30） | `RoofShapeParams([u8;28])` + eave_length_ws | 委托 `system_roof::surface::eave_length_ws` | O 2,500 案例 |
| `RoofShapeParams::height_ws`（内联 394） | `RoofShapeParams::height_ws` | `h·12+(1−h)·0.4`（= Roof::height 上游） | I（inline） |
| `RoofShapeParams::from_wall_shape`（内联 418） | — | 依赖墙形状→参数转换（wall 域），**待取证登记** | — |
| `roof_uv_at_ws_pos`/`roof_uv_and_point_at_ws_pos`（0x1408e4040/4090） | — | 碰撞体注册表管线（RaycastWorld 注册表→arena→raycast→重心），**待接线登记**（R1 collision 域） | O decomp（全管线已在 ALGORITHMS §6.2） |

### 8.2 stairs invalidate.rs（新建，1 业务）

`invalidate_dirty_stair_artifacts`（0x140b37180，O）：脏边 drain（StairsState +0x148）→ 每边
`contains_edge_or_stub` 判定 → 边仍存在 `reset_edge` / 已移除 `remove_edge`；碰撞体撤除×3 +
Arc drop + RtMesh 删为胶水登记（collision/rt 域）。已注册 `stairs/mod.rs` + 1 测试。

### 8.3 RoofBoundsDims 圆字段消费端（U 项登记）

`RoofBoundsDims` 消费端（`assemble_circular_roof`/`calculate_roof_trigger_volumes`/
`structure_outline`/`half_timbering`）全在 system_roof / system_decorator /
system_wall_constructor 库内，country-core 无直接消费面——**圆字段 bottom/top 语义已按
`get_roof_expanded_top_and_bottom` 反编译定案，渲染/装配侧接线待 render 域对账（登记）**。

### 8.4 复验实测

- `cargo check -p country-core`：通过（6 警告全他域）。
- `cargo test -p country-core --lib` = **1720 过/1 败**（唯一失败 geometry/sphere 历史 pre-existing）。
- 本模块新增测试全过：roof_shape 7 / roof_visual_shape 4 / roof_ridge 4 / structural_graph 4 /
  invalidate 1 / replay_stairs 2 / stairs visual 2。
- **紧急编译阻断已解除**：invalidate.rs 测试 `drain_dirty_edges` 可变借用
  （`resource()` → `resource_mut()`）已修，`cargo test --lib` 测试目标恢复编译。

### 8.5 待取证（新增）

- `RoofShapeParams::from_wall_shape`（墙形状→参数，wall 域依赖）。
- `roof_uv_at_ws_pos`/`roof_uv_and_point_at_ws_pos`（碰撞注册表管线，R1 接线）。
- `roof_height_ws_from_wall_shape` 的墙形状精确类型（当前按 decomp 偏移 +0xc/+0x14/+0x18）。
- `roof_shape_at_intersection_y` y==floor 帧体直接拷贝的字节布局（本实现以 t=0 截面近似）。

## 六、inventory 状态建议

- `roof_visual_shape.rs` / `roof_ridge.rs`：no_formal_mapping → **present（6/6、7/7 业务闭合）**。
- `roof_tiles.rs` / `dormer_tiles.rs` / `roof_buffers.rs`：**present（胶水登记）**。
- `stairs/visual.rs`（4 核心）、`stairs_collider.rs`、`stairs_removed_supports.rs`、
  `stairs/structural_graph.rs`（7/7）、`stairs_walls.rs`（add）、`systems/history/replay_stairs.rs`：
  **present（核心闭合，胶水登记）**。
- 开放项：visual RtWorld/mesh 胶水接线、replay_pop_bucket 完整楼梯 diff 应用（共享 history 域）、
  `ReplayCommand::Stairs` 枚举扩展为共享基础设施（需 reviewer/协调者确认归属）。
