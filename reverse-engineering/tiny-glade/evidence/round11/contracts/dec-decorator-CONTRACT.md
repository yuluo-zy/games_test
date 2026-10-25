# 第 11 轮 DEC 组契约：`crates/systems/decorator`（B 版）

> 角色：逆向取证（只读源码、只写证据）。目标 crate：`D:/game/tiny-glade/crates/systems/decorator`
> （78 文件 / 9,288 行，0 `todo!`，62 未恢复空壳 + 18 无表象函数）。
> 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/**`
> （下文 `skeleton/...` 均指该根）。地址口径：B VA = `0x140000000 + 骨架 RVA`。
> 证据复用：`evidence/toolchain/decor-trunk/decompiled/*.c`、
> `evidence/country-core-deep/systems-recovery/groupD/{decomp,decorator_visual/decomp,ui_systems/decomp}/*`、
> `upper-modules-plan/systems/decorator/ANALYSIS*.md`、`upper-modules-plan/BATCH3-LOGIC-REVIEW-decorator.md`、
> `upper-modules-plan/STATUS-2026-10-19.md`。已用 `scripts/rne-read.py` 定点复核原文常量。
> 记号：**O** = 反编译/机器码直接观察；**I** = 由调用面推断；**U** = 证据不足，禁止默认值冒充。
> 范围排除：T13 反馈捕获（AGENTS.md 明确排除）；不改 `rhapsody`/`roof`/`wall`；不改 country-core。

---

## 0. 结论摘要（给 ② 的四条硬约束）

1. **STATUS 清单里有 4 处地址/归属错误必须先纠**（详见 §2.0、§3）：
   `compare_light_lists` 真地址是 `0x141257fa0/0x1412583d0`（`0x140beefa0` 是 `update_buffer_bytes`）；
   `load_collision_as_bbx` 是 `0x1407bb4b0`（`0x140b1aca0` 是 `load_mesh_into_library` 宿主）；
   `ui_change_stair_height` 三个处理器目标实为 `0x1407ade6f`（墙/楼梯共用）、`0x1407adfe2`（屋顶）、`0x1407ae060`（地形），
   当前 Rust 把屋顶/地形处理器的语义**对调**了；
   `ui_rotate_lantern::dst_addr` 不是原函数，`0x1407a8814` 是**楼梯**旋转处理器。
2. **18 个“完全没迹象”中有 6 个是真发射函数且反编译已落盘**（`clear_blueprints`、`copy_blueprints_to_prev`、
   `validate_blueprints`、`show_max_wall_y`、`instantiate_blueprint_colliders`、`instantiate_decorator_colliders`），
   其余 12 个为内联/泛型内联，不是“丢代码”。逐项见 §3。
3. **`playermade_stairs_assemble<true/false>` 仍是唯一硬 U**：PDB 两实例 0x948 栈帧、Ghidra 两次 timeout；
   其 4 个发射闭包 `closure$8/9<true>`、`closure$24/25<false>`（`0x140284b0/0x140275b0/0x140286d0/0x14027840`）
   与主体都无落盘反编译。实现前必须先补这 4 个闭包或 P-code，禁止用空体冒充。
4. **跨 crate 缺口已收敛为 7 个类型/API**（§4）；`replay_decorator` 的 core 缺口其实已闭合
   （`ReplayCommand::Decorator`/`DecoratorReplayCmd`/`ReplayBucket.timing`/`ReplayBucketAux.cull_allow` 都在），
   当前 `decorator_cmd_of` 恒返回 None 属实现未接线，不再是 core-type-gap。

---

## 1. 装饰器生命周期顺序（按调用顺序）

> 原版注册顺序未直接取证（`lib.rs` 的 `Plugin::build` 无独立符号，被内联进宿主 App）。
> 下表顺序 = 由数据依赖 + 机器码调用边得出的**可执行顺序契约**；O/I 标注；阶段间边界以资源写面为准。

### 阶段 0：启动装载（startup，只一次）

| 步骤 | 系统/函数 | 输入资源 | 输出资源 | 依据 |
|---|---|---|---|---|
| 0.1 | `load_decorators` 0x1407b66e0 | 装饰网格目录（`.json`，含 `outline.json`）、AssetMeshLibrary 宿主 | 网格库（`load_mesh_into_library` 宿主面） | O：readdir + 5 字节 `.json` 门 + `outline` 顶点色白 |
| 0.2 | `DecoratorBlueprint::set_colliders/set_interaction_mesh` 0x1408252b0/0x1408255e0 | 碰撞/交互网格数据 | 蓝图 +0x50/+0x80 区 | O |
| 0.3 | `derive_decorator_subtypes` 0x140812b20（首帧起每帧） | DecoratorStorage、PublicWalls | DerivedDecoratorSubtypes | O 结构；尾段 U |

### 阶段 1：创建/放置 → 交互 → 操作入队

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 1.1 | `calculate_decorator_dst::convert_raycast_hit_to_decorator_dst` 0x14127c990 | RaycastHit、UiState | `DecoratorDst`（0=Free/2=Wall/0xb=Terrain/0xd=StairTop/默认 Invalid） | O；payload 字段 U |
| 1.2 | `decorator_interaction_intent::calculate_decorator_interaction_intent` 0x14127cf70 | mouse、PublicWalls/Roof/Stairs、DecoratorStorage、DerivedSubtypes | `DecoratorInteractionIntent`（~0x60B） | O 头部；`find_closest_norm_curve_t` 内联 U（§3） |
| 1.3 | `ui_place_decorator` 0x1407a9c10 | UiState、dispatcher、dst/intent、TransientDecoratorIdGen | 新 DecoratorId、`DecoratorOp`、楼梯 endpoint/edge | O；body 细节 U |
| 1.4 | `ui_move_decorator` 0x1407a4850 / `ui_rotate_decorator` 0x1407a8580 / `ui_change_stair_height` 0x1407adea0 / `ui_change_chimney_height` 0x1407ae2e0 | 同上 + 拖动相位 | `DeferredDecoratorOpQueue`（0x28B，kind 0x21/0x12 等） | O 头部；处理器尾段见 §2.5 |
| 1.5 | `apply_decorator_op` 0x141276e30 | `DecoratorOp`、kind_flag、dst、DecoratorStorage、archivist、`DeferredDecoratorEditHistory`、events | 存储变更 + 历史记录（kind 3/4/5）+ `pending` 置位 + 0x88B 事件 | O 逐分支 |
| 1.6 | `handle_external_decorator_cmds` 0x14081a380 | 三组外部命令缓冲（0x18/0x28/0x30 步长） | `apply_decorator_op` / `History::add(5)` / `History::add(3)` | O；三组来源 U |

### 阶段 2：附着/锚点（墙形/高度变化后重定位）

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 2.1 | `decorator_on_wall_height_changed` 0x14081b5e0 | PublicWallState | `Info+0x10 = Info+4 - wall_attachment_height` | O |
| 2.2 | `decorator_on_move_shape` 0x140811fc0 | 形状新位 | `Info` 位置 +xvy 写 | O；尾段 U |
| 2.3 | `decorator_on_rectangle_edited` 0x140819c00 | Rectangle 边/角 | 沿周长重映射 u（角 `get_closest_u_from_pos`，非角投影 clamp[-1,1]） | O |
| 2.4 | `decorator_on_freehand_wall_edited` 0x14081bac0 / `decorator_on_circle_edited` 0x14081c060 | 自由墙/圆墙 | 新位置 + wall-space 坐标 | O |
| 2.5 | `move_decorators_following_anchors` 0x14081c790 | 已删/变更墙 | 全墙装饰重挂（0.05 余量、门底 +0x21） | O 结构；尾段 U |
| 2.6 | `remove_ivy_under_windows` 0x140814860 | 窗口 + 地形高 | ivy 移除 | O |
| 2.7 | `decorator_on_*` 的调用者（wall-constructor）触发 `apply_decorator_op` change | — | 历史记录 | I：调用边 |

### 阶段 3：变换缓存（每帧先清后建）

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 3.1 | `cache_decorator_transforms` 0x1408153d0 | DecoratorStorage、PublicWalls/PublicWallState、Roof query、TerrainHeightsData | `CachedDecoratorTransforms`（swap 双缓冲 + insert） | O 结构；墙/屋顶公式尾段 U |
| 3.2 | 同函数内墙分支调用 `calculate_adjusted_ladder_positions` 0x1407d8fe0 | 楼梯/墙锚 | 调整后梯位 | O；见 §2.2 #27 |

### 阶段 4：可视化（子类型 → 蓝图 → 实体 → GPU）

顺序（同一帧内由“清 → 差分 → 生成/重装 → 实例化 → seal”构成）：

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 4.1 | `derive_decorator_subtypes` 0x140812b20 | storage、walls | DerivedDecoratorSubtypes | O |
| 4.2 | `maintain_visual_entities::clear_out_visual_entities` 0x1407f9260 / `clear_out_wall_holes` 0x1407f98c0 | 旧 Persistent/Transient 实体表 | despawn + 洞缓冲清零 | O |
| 4.3 | 差分重装：`chimney_assemble_bricks` 0x1407f8db0 / `door_stairs_assemble_bricks` 0x1407ff130 → `WallAttachedDecoTracker::with_diffs` 0x1412ae660/0x1412af3a0 | 旧/新组装参数 | 差分日志 + 变化项重装 | O 壳；主体见 §2.3 #9/#10 |
| 4.4 | 视觉生成族：`generate_*`（arrow_slit/chimney/cottage_wall_window/corner_window/balcony_door/gothic_*/dormer/clutter/flag/lantern/trap_door/stairs/railings） | DecoratorStorage、CachedTransforms、MeshLibrary、DerivedSubtypes | `DecoratorBlueprints` 写入 + `WindowVisualEntityBuilder` 记录 + `DormerHoleSsbo` | O 主体；U 项见 §2 |
| 4.5 | `DecoratorBlueprints` 生命周期：`clear_blueprints` 0x1408260d0 → `instantiate_decorator_colliders` 0x140829850 → `instantiate_blueprints` 0x140827910 → `validate_blueprints` 0x140827130 → `copy_blueprints_to_prev` 0x140826d70 | 修改/移除 id 集、蓝图库 | RaycastWorld collider upsert/remove、实体重建+seal、错误事件 | O（6 个真缺失函数，见 §3） |
| 4.6 | 实体封口：`DecoratorPersistentVisualState::add_decorator_visual_entity` 0x1408119e0 + `seal_persistent_entities` 0x140811e80 | Entity | 持久实体表（seal 后禁止再加） | O |
| 4.7 | 瞬态：`DecoratorTransientVisualState::add_*` 0x140811c30 + `drain`（宿主 despawn） | autoclutter/cloth 子实体 | 瞬态表 | O |
| 4.8 | GPU 后置：`dormer_holes::upload_dormer_holes_to_gpu` 0x1407b1d40、`lantern::maintain_lantern_lights` 0x140790400、`window_lights` 0x1407b5980、`chimney_smoke_anim_thickness` 0x140818df0 | Ssbo/时间/会话 | GPU buffer 更新 | O；SSBO 布局 U（§5 后置） |
| 4.9 | 建造 UI 覆层：`ui_place_decorator::show_decorator_icon` 0x1407adb20、`ui_change_chimney_height::show_signifiers` 0x1407ae910、`ui_stairs_viz`/`ui_focus_stairs` | UiState | UiSignifierStream | O 结构 |

### 阶段 5：历史/备份/剔除

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 5.1 | `create_or_restore_decorator_backup` 0x1408238a0 | `+0x282` 状态（1=CREATE/2=RESTORE）、DecoratorStorage、StairsState、辅助状态 | 备份/恢复 + `+0x281` | O 状态机；辅助 0x30B 三表 U |
| 5.2 | `decorator_record_history` 0x140817850 | `+0x48` pending、0x20B 记录 Vec、locator 集合 | `History::add(EditType::DecoratorV5)`（kind 10；48B `{1,1,kind,f1,f2,f3}`） | O；`record_to_history_edit` 见 §2.4 #45 |
| 5.3 | `ui_decorator_inactive` 0x140791400 → `DecoratorBackup::clear` 0x140823660 | 非交互态 | 备份复位 | O |
| 5.4 | `auto_allow_decorator_culling` 0x14080d3d0 → `cull_oob_decorators` 0x14080d540 → `cull_stair_attached_decorators` 0x14080ec10 / `_on_invalid_stairs` 0x14080f040 | History 录制态、archivist、位置校验结果 | archive + remove op + 蓝图删除；重叠保护 | O；移除 info/变体 U |
| 5.5 | `merge_proposal_*`（0x142001880 / 0x142000e60） | 候选装饰、shape/perimeter | 合并提案（同族同型 + rank + 无重叠 + 最近距离） | O 结构；得分键/偏移符号 U |

### 阶段 6：删除/回放/恢复归档

| 步骤 | 系统/函数 | 输入 | 输出 | 状态 |
|---|---|---|---|---|
| 6.1 | `apply_decorator_op` kind=4 | remove op | `apply_decorator_remove` + 历史 kind 5 + 事件 | O |
| 6.2 | `replay_decorator` 0x14080c8d0 | ReplayBuffer 前桶、速度、time、archivist cull 集 | 命令推进/移除；cull_allow 注入 | O 结构；core 类型已具备（§2.4 #52） |
| 6.3 | `restore_archived_decorators` 0x140817a40 | DecoratorArchivist 全存储/全装饰、InnerWalls、StairsState | storage 恢复 + d2s 索引 + 楼梯 restore_backup | O 结构；步骤 5/6 U |
| 6.4 | `DecoratorStorage::restore_decorators/restore_decorators_diff`（core） | collider maps | 碰撞恢复 | O；见 §4 |
| 6.5 | `DecoratorBackup::clear`/`consume_after_processing` | — | 备份销毁 | `consume_after_processing` 内联，§3 |

---

## 2. 62 个空壳逐个（文件:行 / VA / 参数面 / 依赖 / 可复现常量 / O-I-U / 真实输出与消费者）

> “参数面”为 Ghidra 寄存器面（真实 Rust 类型多为 U）；“常量”均为 `rne-read.py` 复核到的小端值。
> 每项末尾给“消费者”，用于 §5 验收。

### 2.0 先纠四类清单错误

| 项 | STATUS/现注释 | 取证结论 |
|---|---|---|
| `compare_light_lists` VA | 0x140beefa0 | **0x141257fa0**（`closure_env$3`）与 **0x1412583d0**（`closure_env$5`）；0x140beefa0 = `GraphicsDevice::update_buffer_bytes` |
| `load_collision_as_bbx` VA | 0x140b1aca0 | **0x1407bb4b0**；0x140b1aca0 = `startup::load_mesh_into_library` |
| `ui_change_stair_height` 处理器 | 0x1407adba7/0x1407ad998/0x1407ad91a | 跳表 `0x142a832b0`（base=symbol）：idx0→**0x1407ade6f**（tag 0/1 与 tag 4 共用）、idx1→**0x1407adfe2**（tag 2 屋顶）、idx2→**0x1407ae060**（tag 3 地形）、idx3→0x1407ade6f；现 Rust `2→terrain, 3→stair` 对调 |
| `ui_rotate_lantern` 标记 | 0x1407a87a6=stair、0x1407a8814=dst_addr | 跳表 `0x142a82be8`：idx0→0x1407a91bb（墙）、idx1→0x1407a8766（**屋顶**）、idx2→0x1407a87a6（**地形**）、idx3→0x1407a8814（**楼梯**）。`dst_addr` 无原函数，应删除或改名登记 |

### 2.1 楼梯视觉层 `decorator_visual/stairs.rs`（7 项）

| # | 文件:行 | 函数 | VA | 参数面 | 依赖 | 可复现常量 / 公式 | O-I-U | 真实输出与消费者 |
|---|---|---|---|---|---|---|---|---|
| 18 | `stairs.rs:78` | `find_alignment_preferences` | 0x14080beb0 | `(out Vec<0x48B>, nodeA, nodeB, f32, f32, storage, ctx)` | `DecoratorStorage::decorator_info`、`StairNodeAnchor::try_as_wall`、`PublicWalls::get/wall_space` | 归一化 `1/sqrt(p4²+p5²)`（0x142925904=1.0）；符号位 0x1429258f0=0x80000000；方向表 `[1.0, sqrt2-1=0.41421357]` @0x142a8d620；clip 0.5 @0x142925870 | O 结构；node 布局 U | 对齐候选 Vec（0x48B/条）→ `find_best_neighbor_alignments` |
| 19 | `stairs.rs:89` | `find_best_neighbor_alignments` | 0x14080c360 | `(out, candidates &[0x18B], count, mask:u8)` | 递归自身；候选 0x18B `{tag u8@0, vec4@4, vec4@0xc, f32@0x14}` | 初值 f32::MIN（0xff7fffff）；惩罚 -10.0（0xc1200000）；掩码不匹配加 -10 | O 全文 | 选定候选（写入对齐列表）→ `generate_playermade_stair_requests` |
| 20 | `stairs.rs:99` | `generate_playermade_stair_decorators` | 0x140803dd0 | 6 参（storage/transforms/requests/blueprints/PublicWalls/Roof/UiSignifierStream） | `DecoratorStorage::iter_modified`、`CachedDecoratorTransforms`、`PlayermadeStairsAssemblyRequests`、`DecoratorBlueprint::{new,set_colliders,set_interaction_mesh,insert}`、`DecoratorLibKey::new`、`Roof::is_flat/max_y` | 尺寸 0.65 @0x142a7fdf4 / 0.95 @0x142a7d8d0 / 1.0（0x142925904）；高度 `stair_h+0.1`（0x142925984=0.1）；墙饰回写 0x14B 步长 `{color_id,h,h0,0x41200000=10.0,4}`（O，decomp 行 1149-1153）；半木 0.76 @0x142925974、石 0.66 @0x142925978 | O 全文结构；分支载荷 U | 蓝图层（复合交互网格）+ 墙饰追加列表 → `instantiate_blueprints` / 后续 lane |
| 21 | `stairs.rs:110` | `generate_playermade_stair_requests<const X: bool>` | 0x140276e80 / 0x14027e470 | 7 参（storage/transforms/PublicWalls/TerrainHeightsData/StairsState/requests/UiSignifierStream） | `find_best_neighbor_alignments`、`AlignKind::outgoing_dir`、`derive_stair_node_params`、`walk_shape`、`Curve::try_new_from_points`、`VecDeque` 连通分量 | 阶段一清空 requests；阶段三闭合连通分量（细节 U） | O 三阶段骨架；`<true/false>` 语义 U | `PlayermadeStairsAssemblyRequests` → `playermade_stairs_assemble`、`generate_playermade_stair_decorators` |
| 22 | `stairs.rs:116` | `walk_shape` | 0x140809cc0 | 14 参（out, shape, u/v byte, 9×f32, u32×2） | `wall_space::subtract`、`project_clamp`、`lerp`、`Rectangle2d::circular_slice_segment_points`、`TupleWindows` | 采样数 `ceil(len)`；切线偏移 ±0.625；凹点叉积阈值 1e-8；无效哨兵 `0x8000000000000000`；`is_ccw_quad_convex` 内联 | O 全文 | `ShapeWalkOutput { points, heights, segment flags }` → `compute_stair_assembly_params` |
| 23 | `stairs.rs:124` | `maintain_stair_graph` | 0x140806110 | 8 参（storage, StairsState, collider map, history, cull allow, …） | `StairsState::graph/remove_node/endpoint/unset_endpoint`、`is_decorator_cull_allowed`、`HashMap<(StairGraphEdge,u32),ColliderMeshInstance>::retain`、`apply_decorator_op` | 邻居度==1 级联裁剪；桩边簿记 | O 结构 | 图删除/裁剪 + remove op → 历史/存储 |
| 24 | `stairs.rs:134` | `compute_stair_assembly_params` | 0x140807090 | 6 参（nodes/transforms/walls/terrain/out/…) | `lerp`、切线归一化、`StairSubSegment` smallvec、bitvec、`Curve::crashy_new_from_points` | 坡度分类：>2.5=3（0x40200000）、0.25~2.5=2、≤0.25=1（0x3e800000）；子段标记 `bVar22|0x10100`；输出 ~0x29×8B | O 结构；子段掩码 U | `PlayermadeStairsAssemblyParams` → `playermade_stairs_assemble` |

### 2.2 楼梯装配 `decorator_visual/stairs_assemble.rs`（16 项）

| # | 文件:行 | 函数 | VA | 参数面 | 依赖 | 可复现常量 / 公式 | O-I-U | 真实输出与消费者 |
|---|---|---|---|---|---|---|---|---|
| 25 | `stairs_assemble.rs:52` | `structural_graph_subscribe` | 0x1407d4990 | 2 参（subscribers &mut, ctx） | `RaycastWorld::subscribe_to_changes`? / `Publisher::subscribe<ColliderDiff>` | 944B；`insert_resource<StructuralGraphSubscription>` | O 结构；回调身份 U | 结构图碰撞变更订阅 → `calculate_distortion_from_structural_graph` |
| 26 | `stairs_assemble.rs:78` | `stairs_subscribe_lazy` | 0x1407cf210 | 4 参 | 3×`RaycastWorld::subscribe_to_changes` + 2×`Publisher::subscribe<DirtyTiles>` | insert `StairsLazySubscriber`；回调闭包 `{3,0x90000}` | O 结构 | 惰性脏瓦订阅 → `playermade_stairs_assemble` |
| 27 | `stairs_assemble.rs:89` | `calculate_adjusted_ladder_positions` | 0x1407d8fe0 | 8 参 `(out f32*, anchor, f32 a, f32 b, p5, p6, u32, u32)` | `ObservedStairNodeAnchor::try_as_wall` 0x140959ef0、`closure$0` 0x1407d9240 | 偏移 0.28（0x3e8f5c29 @0x142a80a4c）与 0.3（0x3e99999a @0x1429258d8）；方向 `1/sqrt(dx²+dz²)`；条件 `anchor+0x18/+0x38 == 1` | O 全文 | 调整后梯位 2×Vec3 → 缓存/装配 |
| 28 | `stairs_assemble.rs:103` | `construct_ladder` | 0x1407db3e0 | 8 参 `(out, Vec*, p3, float*, p5, byte*, p7, out*)` | `WoodenPlankSsbo`、fastrand 序列 | 档数 `ceil(len/0.65)`（0x142a7fdf4=0.65）；档高 `max(0.4, h-0.593)`（0x3ecccccd=0.4；0xbf17ced9=-0.593）；中轨 `b<5 && (0x16>>b)&1`；随机常量 0xa0761d6478bd642f / 0x40ec3ac8f17ac85e | O 全文 | 梯子木板变换 → 视觉 Ssbo |
| 29 | `stairs_assemble.rs:111` | `playermade_stairs_assemble<const X: bool>` | 0x140008c90 / 0x140018040 | 见骨架；两实例栈帧 0x948 | 调用 `calculate_adjusted_ladder_positions`/`construct_ladder`/`construct_stair_steps`/`create_stairs_collider`/`determine_ladder_support`/`gen_stair_floors`/`gen_stairs_collider_and_rt_geo` | 两函数开头 48B 逐字节相同；4 个发射闭包 `closure$8/9<true>` 0x140284b0/0x140275b0、`closure$24/25<false>` 0x140286d0/0x14027840 | **U（硬阻塞）**：Ghidra 两次 timeout；禁止空体 | 装配输出：梯步/梯子/楼板/碰撞体 → 蓝图/视觉/碰撞 |
| 30 | `stairs_assemble.rs:116` | `playermade_stairs_interpolate_and_compare` | 0x1407cf500 | 5 参（old, new, params, state, out） | `time_continuous_lerp_factor`、`SampledStairsPoints`、`HashMap<StairGraphEdgeIdentity,SampledStairsPoints>` | 位移阈值 5e-05；`USE_STAIRS_LAZINESS==1` 时未变跳过；`+0x148` 脏 | O 结构 | 插值后采样点 + 脏标记 → `propagate_dirty_flags` |
| 31 | `stairs_assemble.rs:124` | `playermade_stairs_assembly_propagate_dirty_flags` | 0x1407d16b0 | 4 参 | `SmallVec<StairSubSegment>` fold、两子世界脏瓦（0x90）、`DirtyTiles::{new_clear,take,extend,iter_tiles}` | `?`→`try_as_wall`（+0x284<=3.0）记受触墙 | O 结构 | 脏瓦/受触墙 → `playermade_stairs_add_wall_holes` |
| 32 | `stairs_assemble.rs:131` | `playermade_stairs_add_wall_holes` | 0x1407d1f80 | 2 参（wall_holes, dirty group） | `WallHoles::add` | 脏洞组 bitset 0x110，条目 0x18 非空 | O 结构 | 墙洞 → 墙渲染/碰撞 |
| 33 | `stairs_assemble.rs:139` | `create_structural_colliders_and_initial_graph` | 0x1407d20c0 | 6 参 | `StairsStructuralGraph::{upsert_span,add_edge_by_span,add_edge_by_collider}`、`RaycastWorld::upsert_collider<(StairGraphEdge,u32),IsoTransform>` | span 表 `+0x130=='3'`；墙锚 0.28/0.3 同 #27；位集去重 | O 结构 | 结构碰撞 + 初始图 → `calculate_distortion_from_structural_graph` |
| 34 | `stairs_assemble.rs:148` | `calculate_distortion_from_structural_graph` | 0x1407d49d0 | 6 参 | `Subscriber::take<ColliderDiff>`、`remove/disconnect_node_by_collider`、`iter_shape_overlaps`、`PriorityQueue` | 畸变 `clamp(0.046875-overlap, …, 1)`；kind ∈ {0x10,0x1a203,0xa203}；Dijkstra/BFS 衰减 0.046875（3/64） | O 结构；kind 枚举 U | 节点畸变 f32 → 碰撞体/视觉 |
| 35 | `stairs_assemble.rs:155` | `split_segment_into_supported_and_unsupported_subsegments` | 0x1407d94a0 | 5 参 | `wall_normal`、`raycast_simple`（向下）、`Empty(0x11)` | 净空 -1.17；`height*0.5 + 横向距`；输出 `Vec<(start,end)> + Vec<f32>` | O 全文 | 支撑/非支撑子段 → `construct_stair_steps`/`gen_stairs_structural_collider` |
| 36 | `stairs_assemble.rs:162` | `construct_stair_steps` | 0x1407da0b0 | 17 参 | `TerrainHeightFloatGrid::bilinear_sample`、fastrand、`random_splits`、`BrickTransformSsbo` | 步数 `floor(0.56/run)`（0x3f0f5c29）；`|Δh|≥0.6`（0x3f19999a）；砖数分母 0.46（ANALYSIS 观察值，需在 0x1407da0b0 内定位对应浮点槽）；辅助 0.15（0x3e19999a）、0.05（0x3d4ccccd） | O 全文 | 梯步砖变换 → 砖 Ssbo/视觉 |
| 37 | `stairs_assemble.rs:169` | `gen_stair_floors` | 0x1407dc170 | 13 参 | `Trimesh` builder、`spawn<NaniBundle<StairsFloorInstanceData>,StairsFloorMeshChunk>`、`floor_tri_count_to_atlas_idx` | 悬挑 `max(0,0.15-depth)`；每 quad 半悬挑 + 6 三角索引；失败日志 "Error spawning stair mesh" | O 结构；索引映射 U | 楼板三角/实体 → 渲染 |
| 38 | `stairs_assemble.rs:177` | `gen_stairs_collider_and_rt_geo` | 0x1407de8b0 | 10 参 | `FilterMap<TupleWindows<StepBy<Zip<...>>>>`、`RaycastMeshData` | 20 值三角索引常量表 `2,3,6,3,7,6,0,4,1,1,4,5,2,3,6,3,7,6,0,2,4,2,6,4,1,5,3,3,5,7,…`（O） | O 结构；表映射 U | RT 几何/碰撞 → RaycastWorld |
| 39 | `stairs_assemble.rs:185` | `gen_stairs_structural_collider` | 0x1407dec90 | 7 参 `(out, Vec*, Vec*, p4, p5, Vec*, Vec*)` | `new_convex` / `new_sphere`（0 点） | `x0y` swizzle ×2；中心 `+(-0.5)`（0x14295a0e8=-0.5）；半径系数 0.5/0.377（0x142925870/0x3ec10ffc）；`new_sphere` 当 0 点 | O 结构 | 结构碰撞体 → `StairColliderHandle` |
| 40 | `stairs_assemble.rs:192` | `floor_tri_count_to_atlas_idx` | 内联（源行 3550） | `tri_count: u32` | `gen_stair_floors` 内联 | 映射 U；入口 = `decomp-0x1407dc170` 的三角计数 → atlas 索引段 | I | 楼板图集索引 → 渲染 |

### 2.3 楼梯扶手 `decorator_visual/stairs_railings_assemble.rs`（4 项）

| # | 文件:行 | 函数 | VA | 参数面 | 依赖 | 可复现常量 / 公式 | O-I-U | 真实输出与消费者 |
|---|---|---|---|---|---|---|---|---|
| 41 | `stairs_railings_assemble.rs:20` | `assemble_railings` | 0x1407f98e0 | 4+ 参（storage, blueprints, transient, 多组 in-stack） | `in_stack_58` 类型映射 + jumptable `0x142a8cc40` | 断言 len；分派表 U | O 结构 | 扶手几何/蓝图 → 视觉 |
| 42 | `stairs_railings_assemble.rs:30` | `railings_tips` | 0x1407fcc80 | 16 参（out, p2, Vec*, u64*, u32×2, u8, u64, i64, f32*, i64, f32*, i64, f32*, p15, p16） | `Affine3Packed::from_cols` 0x14089d500、`BrickSourceId::from_wall_id` 0x140858750、`Curve::try_resample`、`construct_wooden_railing` | 端点偏移 0.0975；类型 >2 → 木扶手（resample）；否则石端帽 `random_splits` + `BrickTransformSsbo`（0.46） | O 全文结构 | 扶手端头几何 → 蓝图/视觉 |
| 43 | `stairs_railings_assemble.rs:48` | `construct_wooden_railing` | 0x1407fdf90 | 8 参 `(out, u32, p3, f32, f32*, u64, i64, u64)` | `WoodenPlankSsbo` + 3×fastrand | 竖栏条件 `in_XMM3<1.6`；弦长 sqrt；0.65/0.24（0x142a7fdf4/df8） | O 结构 | 木扶手木板变换 → Ssbo |
| 44 | `stairs_railings_assemble.rs:62` | `add_corner_stone_railing_piece` | 0x1407febb0 | 14 参 | 点积选角、`BrickTransformSsbo` | push 1–2 块；几何常量见 `decomp-0x1407febb0` | O 结构 | 角石扶手件 → 砖 Ssbo |

### 2.4 差分/灯笼/旗帜/历史/装载/合并/杂项（15 项）

| # | 文件:行 | 函数 | VA | 参数面 | 依赖 | 可复现常量 / 公式 | O-I-U | 真实输出与消费者 |
|---|---|---|---|---|---|---|---|---|
| 9 | `deco_differential_state.rs:104` | `chimney_reassemble` | 0x1412ae660（31KB） | 5 参（context a/b/c/e/g） | `DecoratorStorage::iter_modified`、`DecoDiff::current`、`WallAttachedDecoTracker<ChimneyAssemblyParams>`、砖列装配链 | `with_diffs`：每装饰旧参数 vs 新参数，仅变化项重装；96B 记录头 74.48f32（0x4294f6e0）/77.5f32（0x429b0010） | O 结构（反编译全文已落盘）；砖列主体 U | 重装后的烟囱砖列 → 碰撞/视觉 |
| 10 | `deco_differential_state.rs:115` | `door_stairs_reassemble` | 0x1412af3a0（26KB） | 3 参（a,b,d） | 同 #9，`DoorStairsAssemblyParams` | 同构；仅变化项重装 | O 结构；主体 U | 门梯砖列 → 碰撞/视觉 |
| 11 | `deco_differential_state.rs:125` | `compare_assembly_params` | 0x1412ae1c0 / 0x1412add20（12KB×2） | 3 参 | `FUN_1412accf0`×2、`DecoratorStorageAddr::as_wall` 0x140a1c240 | 40B `Chimney/DoorStairsAssemblyParams` 逐字段比较（布局 U，可从 decomp 读出） | O 结构；比较准则 U | `bool` → `with_diffs` |
| 12 | `flag_connections.rs:41` | `greedily_connect` | 0x1407e1430（25KB） | 9 参 `(out, Vec* a, Vec* b, p4, u64, char, f32,f32,f32)` | `RaycastWorld::intersects_any`、候选 0x50B、`HashMap<(usize,usize),()>` | 容量 3.0（0x40400000）；最小距² 0.02（0x3ca3d70a）；上限 12.0（0x41400000）；遮挡/`2|dz|<=水平距`；`dist>0.02` | O 全文结构 | 连接边 Vec → `clothline` |
| 13 | `flag_connections.rs:49` | `clothline` | 0x1407e2510（10.7KB） | 14 参 | `WindowVisualEntityBuilder::build<DecoratorTransientVisualState>` 0x1412ab270 | `n=max(2,ceil(dist/spacing))`；下垂曲线 `(1-t²)*0.08*len`（0x142a88890=0.08）；0.5/1.0/-1.0 常量 | O 结构；10 参语义 U | 布绳实体 → 瞬态视觉表 |
| 16 | `lantern.rs:67` | `maintain_lantern_lights` | 0x140790400（24KB） | 8 参 | 灯笼 SSBO、屋顶瓦位图、`GladeType`、`GraphicsDevice::update_buffer_bytes` 0x140beefa0、`compare_light_lists` | 点亮窗条件 `0.45 + 0.2*(0.14*rng + 0.01) <= t < 0.72`（0x3ee66666=0.45；0x3e4ccccd=0.2；0x3e0f5c29=0.14；0x3c23d70a=0.01；0x3f3851ec=0.72）；记录上限 0x8000；记录 0x1cB；排序 `impl$7::cmp` 行 245 | O 结构；记录布局 U | GPU 灯光双缓冲 → 渲染 |
| 17 | `lantern.rs:78` | `compare_light_lists` | 0x141257fa0 / 0x1412583d0 | `(out u32*, new_len, old, old_len, &5×ctx)` | 0x1cB 记录；`impl$7::cmp` | 逐条比较 `{+0x10 u64 ColorId,@+0x18 f32}`；差异 push 记录 tag 0xe（28B 元素） | O 全文结构 | GPU buffer 增量记录 → `update_buffer_bytes` |
| 45 | `deferred_decorator_history.rs:175` | `record_to_history_edit` | 0x140817850（宿主） | `&DeferredDecoratorRecord -> DecoratorV5Edit` | `History::add(EditType::DecoratorV5)` | 原 48B `{u64 1, u64 1, kind, f1, f2, f3}`；kind 6 终止 | O 布局；两个前导 1 常量需证明恒真 | `DecoratorV5Edit` → History/replay |
| 46 | `derive_decorator_subtype.rs:85` | `is_half_dormer` | 0x140814470（~8KB 反编译全文已落盘） | `(u32 ty, char side, p3, wall_state, roof?, roof_map*)` | `Roof::is_flat/is_gable`、`Rectangle2d::get_closest_edge_id`、`DecoratorLibKey::new` + `HashMap<DecoratorLibKey,DecoratorCollision>`（0x48B）、`Curve::get_approx_coord_from_pos` | 族掩码 `0x160600 >> ((ty-0x38&3)<<3)`；墙形 tag `+0x258 > 1 → false`；flat → false；gable 边命中两端 → false；最终 `lerp(u) < ((max-min)+y)*0.5 + base` | O 全文（decomp 已落盘） | bool → `derive_decorator_subtype` |
| 47 | `load_decorators.rs:85` | `load_collision_as_bbx` | 0x1407bb4b0 | `(asset_lib, name, subtype)` | `AssetMeshLibrary::get_by_name`、`Mesh::collision_bbx`、`box_trigger_volume`、`RaycastMeshData::new_cuboid` | 断言 `Vec2==(1.0,0.0)`；输出 `[0..7]` bbx 组合；0x90B 对象 | O 结构；name 串 U | 碰撞 bbx → `DecoratorCollision` |
| 48 | `merge_proposal_roof_decorators.rs:34` | `find_best_roof_decorator_merge_position` | 0x142000e60 | 5 参（info, derived, storage, shape, Rectangle2d） | `dst_as_roof`、`max_rank`、`subtype/dims_wall_space`、`perimeter/as_rectangle`、`edge_length/get_closest_edge_id`、`intersect_roof_decorators` | 偏移 `(w_m-w_f)*0.5`；非角 `/perimeter`；角 `*0.25`；距离 SQRT；最近写入 `+0xd..+0x1a` | O 结构；符号/权重 U | `RoofMergePosition` → merge 执行 |
| 49 | `merge_proposal_roof_decorators.rs:57` | `try_merge_decorators` | 内联（lib.rs 行 77 同源） | `(candidates, rank)` | `DecoratorInfo::eq`、`Iterator::any/sum` | 删除本地 `fn try_merge_decorators(_a,_b)->false`，统一走 `lib.rs::try_merge_decorators` | I（重复实现应删除） | 合并布尔 → merge 执行 |
| 50 | `merge_proposal_wall_decorators.rs:57` | `find_best_wall_decorator_merge_position` | 0x142001880 | 4 参（candidates, info, storage, PublicWalls） | `propose_wall_decorator_merge_position_inner` 0x142001d40、`intersect_wall_decorators` | 逐个 propose + `min_by` 距离；无 → `+0x5f=2` | O 结构；得分键 U | `WallMergePosition` → merge 执行 |
| 51 | `merge_proposal_wall_decorators.rs:69` | `snapping_direction` | 内联（lib.rs 行 63） | `(WallSpace, WallSpace) -> f32` | `Vec2::dot`、`subtract/project` | 方向由两墙空间坐标差符号得 -1/0/1；0 = 居中 | I（本地重复实现应删除） | 吸附方向 → propose |
| 52 | `replay_decorator.rs:118` | `decorator_cmd_of` | 内联 | `&ReplayCommand -> Option<DecoratorReplayCmd>` | core 已有 `ReplayCommand::Decorator`、`DecoratorReplayCmd`、`ReplayBucket.timing`、`ReplayBucketAux.cull_allow` | 应 `match ReplayCommand::Decorator(cmd) => Some(cmd.clone())`；时序用 `bucket.timing[index]`，移除命令用 `bucket.remove_command(index)`；cull_allow 用 `take` 注入 archivist | I/O（core 缺口已闭合） | 装饰回放命令 → 存储/历史 |
| 61 | `utils/mod.rs:129` | `clutter_mesh_ty_data` | 0x1408b8e90 | `() -> u8`（原为 core helper） | core 未恢复；`generate_decorator_clutter` 消费 | 原函数在当前数据库未反编译；需定点 `re_mcp_ghidra` 0x1408b8e90 取 subtype→mesh 类型表 | **U** | 杂乱网格类型 → `generate_decorator_clutter` |
| 62 | `window_lights.rs:157` | `rng_seed` | 无原函数 | `() -> u64` | `fastrand::Rng::f32()` 全局 | 原版直接调用全局 `fastrand::Rng::f32()`（decomp 行 287），无 `init_rng`/seed 函数；`timer = 2*rng` | O（现实现以 seed 0 固定，错误） | 灯态重计时 → 亮度插值 |

### 2.5 老虎窗/阳台门/窗/旗帜/UI 处理器（剩余 20 项）

| # | 文件:行 | 函数 | VA | 参数面 | 依赖 | 可复现常量 / 公式 | O-I-U | 真实输出与消费者 |
|---|---|---|---|---|---|---|---|---|
| 1 | `blueprint.rs:80` | `DecoratorBlueprint::new` | 内联 | `()` | `Default` | 全零 0x140B 结构；`+0x80` 判别 0 = 未设交互网格 | I | 空蓝图 → 各 generate |
| 2 | `blueprint.rs:138` | `brick_source` | 内联 | `(&self) -> Option<BrickSourceId>` | `BrickSourceId` | 原字段在蓝图 `+?`；精确偏移 U | U | 砖源 → 砖列装配 |
| 3 | `cottage_balcony_door.rs:68` | `add_door_autoclutter` | 0x1407f5e70（5.8KB） | 6 参 `(info, quat f32*, ctx, u32, p5, Vec*)` | `FUN_14128cc30`×2、`FUN_14078ca00`×2、`GladeType::is_autumn` | 记录 0x50B push；`param_1+0x45 > 3` 门控；随机位 `(hash&3)==0`；`0xa0761d6478bd642f ^ 0xe7037ed1a0b428db` | O 结构；autumn 载荷 U | autoclutter 子实体 → 瞬态视觉 |
| 4 | `assemble_dormer_roof_tiles.rs:22` | `dormer_roof_height` | 内联（源行 389） | `(&DecoratorInfo)->f32` | `generate_dormer_roofs` 内联 | 由 `Info` 高度/屋顶形状得；精确式 U（入口 decomp-0x1407cb1a0） | U | 屋顶高 → 瓦片装配 |
| 5 | `assemble_dormer_roof_tiles.rs:27` | `is_bay_window` | 内联（源行 381） | `(&DecoratorInfo)->bool` | 同上 | 凸窗判别；`type`/subtype 掩码 U | U | 跳过凸窗 |
| 6 | `collision_geo.rs:30` | `get_collision_mesh` | 0x14128a0b0（50KB） | 7 参 `(out f32*, p2, p3, p4, f32, p6, HashMap*)` | `DormerKey` 缓存、`inverse_lerp`、x0y swizzle、`new_convex`、递归自调用 | `DormerKey` 量化 `round(dist*5)`（0x40a00000=5.0）；缓存键 `{i32 x,i32 z,i32 count,i8 roof_idx}`；三角带采样 | O 结构；网格数学 U | `Arc<RaycastMeshData>` → 蓝图碰撞 |
| 7 | `cottage_dormer.rs:76` | `get_dormer_panel_xforms` | 内联（源行 172） | `(&DecoratorInfo)->Vec<IsoTransform>` | `generate_cottage_dormers` 内联 | 面板尺寸/位置；U | U | 面板变换 → 视觉/碰撞 |
| 8 | `cottage_wall_window.rs:116` | `add_lintels` | 0x1407ef990（2.6KB） | **12 参**（当前 Rust 只有 4） | `FUN_14082b3b0`×2（0x160B 元素 push）、`OverhangSupportFinderCache::new` 0x140c925d0、`DecoratorLibKey::new`、`dst_as_wall` | `(y+1.43)*0.5`（0x3fb70a3f=1.43，0x3f000000=0.5）；scale 首尾 0.82/中部 0.64（0x3f51eb85/0x3f23d70a）；`flag bit0 * height/0.9`（0x3f666666=0.9）；mesh `"decorators/setdressing_window_lintel"`（len 0x24）/shader `"nani_solid_vertex_color_window"`（0x1e） | O 常量/结构；12 参语义 U | 楣块视觉 → 窗族蓝图 |
| 14 | `gothic_balcony_door.rs:25` | `check_for_door_stairs` | 0x1407f3340（1330B） | 5 参 `(p1, p2, float* p3, p4, p5)` | `FUN_140257ba0` 墙空间、`bilinear_sample_at_world_xz<TerrainHeightFloatGrid>` 0x140a14df0、`RasterBufferRef<512>` | 门高/接地判据；地形 bilinear；`fVar12 = (p4+0x1c - p4+0x18)*(-0.5) + p3[5]`（0x14295a0e8=-0.5）；容差 `_UNK_1429df16c` | O 全文结构 | 门前楼梯信息 `/ u64` → 门蓝图/楼梯 |
| 15 | `gothic_wall_window.rs:311` | `get_window_holes` | 内联（源行 421） | `(id/payload)->Vec<u8>` | `generate_gothic_wall_windows` 内联 | 洞几何；U | U | `DormerHoleSsbo`/蓝图 |
| 53 | `handle_splitting_roof_decorators.rs:68` | `handle_moving_out_roof_decorator_part` | 0x14127bab0 | 6 参（storage,id,info,addr,part,stream） | `FracturedPart`、`TransientDecoratorIdGen::generate`、`apply_decorator_op` ×6、`FUN_14127c7f0` 内层 | u 回绕 `[0,1)`；分离件记录 | O 结构；细节 U | 6 次 op（remove+place）→ 历史/存储 |
| 54 | `handle_splitting_wall_decorators.rs:63` | `handle_moving_out_wall_decorator_part` | 0x141279410 | 6 参 | `is_attached` 角落特例、碰撞、逐部件沿墙新位置、`apply_decorator_op` | 部件布点 `(i/(n-1)-0.5)*width*(1-1/n)`；内层 0x14127a480 | O 结构 | op → 历史/存储 |
| 55 | `ui_change_chimney_height.rs:151` | `preview_ratio` | 内联（0x1407ae2e0 内） | `(ratio) -> f32`（当前签名不足） | 光标高射线、`param_13+0x28` 参考高度 | `abs(Info+0x14 - (cursor + Info+0x24)) / reference`；参考高度来自 `param_13+0x28` | O 公式（decomp 行 201-214）；`param_13` 身份 U | 比例 → 预览高度/op |
| 56 | `ui_change_stair_height.rs:63` | `change_stair_height_wall` | 尾段 **0x1407ade6f**（tag 0/1）与 tag 4 共用 | `(storage, info)` | 键位/墙空间 | 见 §2.0 跳表；当前地址注释需改 | O 目标；尾段细节 U | 墙/楼梯高度写回 |
| 57 | `ui_change_stair_height.rs:66` | `change_stair_height_terrain` | 尾段 **0x1407ae060**（tag 3） | 同上 | 地形 | 见 §2.0 | O 目标；细节 U | 地形高度写回 |
| 58 | `ui_rotate_lantern.rs:73` | `rotate_wall_handler` | 尾段 0x1407a91bb | `(local, storage, info, dragged)` | 墙空间角度 | 跳表 idx0；墙空间角度计算 U | O 目标 | 旋转角写回 |
| 59 | `ui_rotate_lantern.rs:101` | `rotate_stair_handler` | 尾段 **0x1407a8814**（tag 4） | 同上 | 楼梯拓扑 | 跳表 idx3；当前注释写成 dst_addr 需改 | O 目标；变换 U | 楼梯旋转 |
| 60 | `ui_rotate_lantern.rs:112` | `dst_addr` | **无原函数** | `(info)->Option<DecoratorStorageAddr>` | — | `0x1407a8814` 是楼梯旋转处理器；存储地址查找在 dispatch 前用 `FUN_140a1c3e0`（`DecoratorStorage::lookup_storage` 等价） | I（应删除或改名登记） | 无消费者（spurious） |

> 另：`ui_rotate_lantern.rs:78/84` 的 `rotate_roof_handler`/`rotate_terrain_handler` 虽然不在 62 清单，但按 §2.0 应分别对应 0x1407a8766/0x1407a87a6；现注释地址已正确，保留。

### 2.6 老虎窗屋顶装配的“非 62 但语义未闭合”项（实现时一并处理）

这些函数当前有体但只写入空蓝图，**不能算已恢复**；不在 STATUS 的 62 里，但属于 ② 必须闭合的真实业务：

- `generate_dormer_roofs` 0x1407cb1a0（57KB）：子类型掩码 `0x800186`、`subtype==0x17` 分支（decomp 行 412/1161）、gothic(8) → `gothic_dormer_profile_curve` → `calc_gothic_half_dormer_profile_ws` + `get_gothic_hat_bones` → `assemble_gothic_dormer_tiles`；其余 → `assemble_dormer_roof_tiles`。
- `assemble_dormer_roof_tiles` 0x14128db20（54KB，938 行反编译）与 `assemble_gothic_dormer_tiles` 0x141290b30。
- `generate_cottage_dormers` 0x1407c7ac0、`generate_gothic_dormers` 0x1407c5650、`get_collision_mesh` 0x14128a0b0。
- `gothic_wall_window::calc_gothic_half_dormer_profile_ws` 0x1407eb160（反编译打散，0x70B×8 分配 + 两次 `FUN_1407e7dd0`；需 P-code）。
- `generate_gothic_wall_windows` 0x1407e87b0、`generate_cottage_wall_windows` 0x1407ebfd0。
- `generated_*` 的 mesh/shader 名静态表（DAT_142a8xxxx）仍 U，禁止编造字符串。

---

## 3. 18 个“完全没迹象”函数：分类与深挖入口

> 判据：骨架有 `// 函数符号` 但无 `// 编译实例 RVAs` → 内联/泛型内联（无独立符号）；有 RVA → 真发射函数，属真缺失。

### 3.1 真缺失（6 个，有独立符号、反编译已落盘，必须补实现）

| 函数 | VA | 反编译证据 | 行为要点（O） | 消费者 |
|---|---|---|---|---|
| `blueprint::clear_blueprints` | 0x1408260d0 | `evidence/toolchain/decor-trunk/decompiled/1408260d0_clear_blueprints.c` | puffin；`DecoratorStorage::iter_mut`；对 modified/removed id 与 removed storages：SipHash13 → `RawTable::remove_entry<DecoratorId,Arc<DecoratorBlueprint>>`，Arc 归零 drop | 每帧蓝图清理 → `instantiate_*` |
| `blueprint::copy_blueprints_to_prev` | 0x140826d70 | `140826d70_copy_blueprints_to_prev.c` | `hashbrown clone_from<HashMap<DecoratorId,Arc<DecoratorBlueprint>>>` 当前→prev；拷贝 `+0x20/+0x28` | 差分基线 → `instantiate_blueprints` |
| `blueprint::validate_blueprints` | 0x140827130 | `140827130_validate_blueprints.c` | 先 `clear_modification_tracking`；对 `HashSet<DecoratorLocator>` `try_fold(all)`；通过且 `+0x281` 置位按 8 分支跳表；错误累积 `+0x282=2` + push 事件 | 错误事件/放置门控 |
| `blueprint::show_max_wall_y` | 0x140827350 | `140827350_show_max_wall_y.c` | `from_iter zip(Vec3,f32)` 合成墙顶折线；`self+0x258 != 2` 再 push 一点；`DebugLine dashed/line_width/show` | 调试/放置预览 |
| `blueprint::instantiate_blueprint_colliders` | 0x140827510 | `140827510_instantiate_blueprint_colliders.c` | puffin + 写计数器后直接转发 `instantiate_decorator_colliders` | `instantiate_blueprints` |
| `blueprint::instantiate_decorator_colliders` | 0x140829850 | `140829850_instantiate_decorator_colliders.c` | removed 集 `remove_collider<DecoratorId>` + Arc 减引；蓝图 `+0x60==0` 用 `+0x50 Vec3/+0x58 Quat/+0x64/+0x6c/+0x70` 构造 `RaycastMesh::new` → `upsert_collider<DecoratorId,(Vec3,Quat)>`；`+0x90==0` 第二碰撞 upsert；`+0x90!=1` panic | RaycastWorld/碰撞 |

### 3.2 内联/泛型内联（12 个，不是丢代码；实现时按宿主函数内联落位）

| 函数 | 骨架源行 | 宿主（深挖入口） | 说明 |
|---|---|---|---|
| `backup::DecoratorBackup::consume_after_processing` | 46 | `create_or_restore_decorator_backup` 0x1408238a0 | 处理完清备份；需在 `decomp-0x1408238a0-backup_create_or_restore.txt` 确认调用次序 |
| `blueprint::DecoratorBlueprintCollider::as_transformed_collider` | 131 | `instantiate_decorator_colliders` 0x140829850 | 碰撞体 → 变换后碰撞 |
| `blueprint::DecoratorBlueprints::iter_maybe_modified` | 315 | `clear_blueprints`/`validate_blueprints` 内联实例 | 泛型内联迭代器；语义 = 仅 modified 项 |
| `blueprint::show_intended_placement` | 810 | `instantiate_blueprints` 0x140827910 内联 | 预览放置 |
| `arrow_slit::arrow_slit_profile_curve` | 246 | `generate_arrow_slits` 0x1407bbee0 | 硬编码剖面曲线；从 decomp 提取点表 |
| `stairs::is_ccw_quad_convex` | 2114 | `walk_shape` 0x140809cc0 | 凹点剔除判定（叉积 1e-8） |
| `stairs_assemble::resample_curve` | 2650 | `railings_tips` 0x1407fcc80 / `construct_ladder` | `Curve::try_resample` 等价 |
| `restore_archived_decorators::overwrite_decorator_state_from_archivist` | 80 | `restore_archived_decorators` 0x140817a40 | 逐装饰覆盖状态 |
| `decorator_interaction_intent::find_closest_norm_curve_t` | 408 | `calculate_decorator_interaction_intent` 0x14127cf70 | 曲线最近 t；4 个内联闭包 0x14127... |
| `ui_decorator_affordance_dispatch::decorator_raycast_flags` | 818 | `ui_dispatch_decorator_affordance` 0x1407af340 | raycast 层/掩码 |
| `ui_move_decorator::highlight_child_autoclutter` | 1254 | `ui_move_decorator` 0x1407a4850（closure$0 0x1407a82f0） | 子 autoclutter 高亮 |
| `ui_stairs_viz::find_rightmost_position_for_gizmo` | 1152 | `visualize_ui_stairs` 0x140791510 | 最右位置；gizmo 绘制 |

### 3.3 深挖入口（可直接执行的取证动作）

1. `playermade_stairs_assemble`：对 4 个闭包 `0x140284b0`、`0x140275b0`、`0x140286d0`、`0x14027840` 逐一定点反编译（RE-MCP/Ghidra，program=`tiny-glade.exe`）；若仍 timeout，改 P-code/原字节手解。主体两实例 0x140008c90/0x140018040 用同一 job 的 `get_function_pcode` 或 `read_memory` 分段。
2. `clutter_mesh_ty_data` 0x1408b8e90：定点反编译，取 subtype→mesh 类型表（当前证据目录无该地址）。
3. `calc_gothic_half_dormer_profile_ws` 0x1407eb160：已落盘但打散；用 P-code 还原两次 `FUN_1407e7dd0(&ctx, 0/1)` 的闭包参数（0x70B 元素、8 字节对齐）。
4. `is_half_dormer` 0x140814470：已落盘完整；实现时逐分支复刻（decomp 在 `groupD/decomp/decomp-0x140814470-is_half_dormer.txt`）。
5. `generate_dormer_roofs` 0x1407cb1a0 / `assemble_dormer_roof_tiles` 0x14128db20：已落盘 57KB/54KB；先提取瓦片间距与 f16 打包常量，再落 Rust。
6. `ui_move_decorator`/`ui_place_decorator`/`ui_focus_stairs`/`visualize_ui_stairs` 的闭包：`groupD/ui_systems/decomp` 已覆盖主函数，闭包地址见 BATCH3 复核表（0x1407a7a50、0x1407ad8c0/0x1407ad9f0、0x14079af50 等）。

---

## 4. 跨 crate 契约（只列需求，不改 country-core）

### 4.1 已存在、可直接消费（无需新增）

- `DecoratorStorage`：`get/get_mut/get_or_insert_default/lookup_storage/decorator_info(_mut)/iter_walls(_mut)/wall_decorators(_mut)/iter_terrain_decorators/iter_stair_decorators/iter_removed_storages/mark_mutable/clear_modification_tracking/on_wall_removed/on_wall_split/on_walls_merged/apply_decorator_place/change/remove/archive_current_decorators/restore_decorators(_diff)`。
- `Decorators`：`iter_maybe_mut/iter_modified(_ids)/iter_removed_ids/mark_all_modified/clear_modification_tracking/insert/get/get_mut/remove/tracked_clone_from`。
- `DecoratorInfo`（`+0x2c id/+0x44 rank/+0x46 ty/+0x47 state/+0x22 dst tag`）、`DecoratorId`、`DecoratorLocator`、`DecoratorExtraInfo`（Stair/Chimney/Flag/Window/Lantern）。
- `CachedDecoratorTransforms`：`swap_buffers_and_clear/insert/get/try_transform/try_position`。
- `DecoratorArchivist<C>`：`walls/special1/special2/culling.allowed_ids`；`TrackedDecorators`、`ObservedDecoratorInfo([u8;72])`、`DecoratorStorageAddress`。
- `History<EditType>`：`is_recording/pending/add(EditType)`；`EditType::DecoratorV5(Arc<DecoratorV5Edit>)`；`DecoratorV5Edit{kind, fields:[u64;3]}`。
- `ReplayBuffer/ReplayBucket/ReplayCommand::Decorator/DecoratorReplayCmd/CommandTiming/ReplayBucketAux{cull_allow}`。
- `StairsState`：`graph/endpoint/contains_edge_or_stub/iter_edges_and_stubs/drain_dirty_edges/make_backup/restore_backup/extract_diff_and_make_backup/apply_diff/restore_snapshot`。
- `StairsStructuralGraph`：`upsert_span/upsert_support_source_collider/add_edge_by_span/add_edge_by_collider/remove_node_by_collider/disconnect_node_by_collider/register_edge_index/collider_to_span/span_to_collider/collider_to_height/connections`。
- `StairWallAnchor/ObservedStairNodeAnchor/AlignKind/PreferredNodeAlignment`（24B/20B 布局已锚定）。
- `DecoratorLibKey/DecoratorCollision/DecoratorMeshLibrary/DecoratorRank/DerivedDecoratorInfo/DerivedDecoratorSubtypes`。
- `DormerHoleSsbo`（48B）、`WallHoles`、`PublicWalls/PublicWallState`、`Roof/RoofTriggerVolume/ShapeParameters/Rectangle2d/Circle2d`、`TerrainHeightsData`。
- `DecoratorColliderMaps`（仅 `native-physics` feature）：`world + collision_a/b`。

### 4.2 缺失/需补的 7 项（上报中央，不由 DEC 改 core）

1. `PlayermadeStairsAssemblyRequests`：本 crate 与 wall-constructor 都引用，core 无定义；需携带每边 `(AdjustedStairEdge, AlignKind)`、阶段三闭合状态。
2. `AdjustedStairEdge`：`StairsState::graph` 边 → 调整后节点/方向；core 无。
3. `StairSubSegment`：`compute_stair_assembly_params` 输出 smallvec 元素（≥0x20B）；core 无。
4. `OverhangSupportFinderCache`：`add_lintels`/`generate_wall_chimneys` 消费；现仅在 wall-constructor crate 本地定义（`construct_elevation_supports/find_overlap_segments.rs`），应上收或共享。
5. `StairsLazySubscriber` / `StructuralGraphSubscription`：订阅资源；core 无，需在 stairs 资源域提供。
6. `SampledStairsPoints` + `HashMap<StairGraphEdgeIdentity, _>`：`interpolate_and_compare` 输出；core 无。
7. `DecoratorBlueprint` 完整 0x140B TPI 布局：`+0x50/+0x58 pos/rot`、`+0x60/+0x80` 两 `Arc<RaycastMeshData>`、`+0x120 door_stairs [u32;8]`、`+0x128/+0x130 tracker`、`+0x138/139` 标志。当前 crate 本地模型是消费面子集，字段语义 U；需与 core 碰撞/交互 API 对齐。

### 4.3 行为契约（core 供方必须保证）

- `DecoratorStorage::archive_current_decorators` 与 `apply_decorator_place/change/remove` 的 `modified/removed` 追踪语义（供 `clear_blueprints`/cull/backup 消费）。
- `DecoratorStorage::restore_decorators` 的 collider 移除顺序（`collision_a` 后 `collision_b`，各减一次 Arc）。
- `History::add` 在 `is_recording()==false` 时的失败语义（`decorator_record_history` 当前只 `log::warn`，需确认原版是否 panic/忽略）。
- `ReplayBucket::remove_command` 必须同步压缩 `timing`（core 已实现，DEC 侧禁止直接 `commands.remove`）。
- `DecoratorInfo` 的 `+0x45`（rank/type 间字节）与 `+0x47 state` 语义：`is_half_dormer`/`cull`/`backup` 读取，当前 core 标 U，需保持原样不发明。

---

## 5. 验收门（每个函数的“真实输出 + 消费者”判据）

### 5.1 通用判据

一个空壳算“闭合”必须同时满足：

1. **真实输出**：函数返回/写入的是原版业务产物（Vec/Entity/碰撞/历史记录/GPU 记录），不是 `Vec::new()`、`0`、`false`、`() `、`Vec3::ZERO`、空蓝图。
2. **真实消费者**：该产物被至少一个已接线的下游系统读取（见 §2 每行“消费者”）；测试必须断言下游可见的效果，而非只断言函数被调用。
3. **常量/分支**：§2 列出的可复现常量全部落地；反编译中存在的 panic/边界分支（空输入、越界、非有限）保留，不得用默认值吞掉。
4. **O/I/U**：U 字段保留原始字节/占位并显式登记，不得伪造语义；U 分支必须能在测试中区分“未实现”与“实现错误”。
5. **无重复实现**：`merge_proposal_*` 的本地 `try_merge_decorators`/`snapping_direction`、`ui_rotate_lantern::dst_addr` 等 spurious/重复项删除或统一到 `lib.rs`/core。

### 5.2 逐组验收重点

| 组 | 必验效果 | 反例（禁止） |
|---|---|---|
| 楼梯视觉/装配 | `walk_shape` 输出点数/凹点剔除数；`compute_stair_assembly_params` 的坡度 1/2/3 分段与子段合并；`construct_stair_steps` 步数 `floor(0.56/run)` 与砖数；`construct_ladder` 档数/档高；`gen_stair_floors` 三角数与 atlas idx | 返回空 Vec、恒 0、跳过 0 点球碰撞、把 `<true/false>` 合并成一个空体 |
| 楼梯扶手 | `railings_tips` 0.0975 偏移与木/石分支；`construct_wooden_railing` 1.6 竖栏条件；角石件 1–2 块 | 只写蓝图不产几何 |
| 老虎窗/门/窗 | `get_collision_mesh` 的 DormerKey `round(dist*5)` 缓存命中与三角带；`assemble_dormer_roof_tiles` 的 f16 打包与瓦片数；`add_lintels` 0.82/0.64/1.43；`check_for_door_stairs` 门高/接地判据 | 用空蓝图/`Vec3::ZERO` 冒充；漏掉 0x17/gothic 分支 |
| 差分重装 | `with_diffs` 仅变化项重装；`compare_assembly_params` 40B 逐字段；烟囱/门梯砖列在变化前后数量差 | 无条件重装或永不重装 |
| 灯笼/窗灯 | `maintain_lantern_lights` 的 `0.45+0.2*(0.14*rng+0.01)<=t<0.72`；`compare_light_lists` 0x1cB 差分 push tag 0xe；`window_lights` 用全局 `fastrand::f32()` 且 `timer=2*rng` | 固定 seed 0、恒 false 比较、不更新 GPU buffer |
| 旗帜/布条 | `greedily_connect` 容量 3、0.02 最小距、遮挡；`clothline` 等分点数与 0.08 下垂 | 空链、单点布绳 |
| 历史/备份/回放 | `record_to_history_edit` 48B `{1,1,kind,f1,f2,f3}`；`decorator_record_history` kind 6 终止与无条件清空；`replay_decorator` 用 `timing` 推进并按 `cull_allow` 注入；`create_or_restore` 状态机 1/2/0 | 返回 None、静默跳过命令、备份只清不恢复 |
| 合并/剔除 | `find_best_*` 的最近距离/min_by 与无重叠；cull 的重叠保护（重叠保留）与归档删除 | 恒 None、删除重叠装饰 |
| UI 处理器 | 旋转跳表 tag 映射（0/1/2/3/4 → 墙/屋顶/地形/楼梯）；楼梯高度 `2→0x1407adfe2, 3→0x1407ae060`；`preview_ratio` 公式；split 的 `TransientDecoratorIdGen` + 多次 op | 地址/标签错配、空体 handler |

### 5.3 GPU/渲染后置项（单列，不得以“后置”为由删逻辑）

- `dormer_holes::upload_dormer_holes_to_gpu` 0x1407b1d40：`DormerHoleSsbo` 布局（48B）与 `update_buffer_bytes` 调用面。
- `maintain_lantern_lights` 的 GPU 灯光缓冲（`update_buffer_bytes` 0x140beefa0、名字 hash 0x35、容量 0x8000）。
- `window_lights`/`chimney_smoke_anim_thickness` 的 GPU 消费（当前仅 CPU 状态）。
- 所有 `WindowVisualEntityBuilder::build` 的 mesh 名/shader 名静态表（DAT_142a8xxxx）当前部分 U；CPU 记录必须真实，GPU 绑定可后置但字符串/长度需保留证据。
- `maintain_dormer_tiles` 的 `SparseInstanceBufferAllocation<RoofTileSsbo>` 退役路径。

---

## 6. ② 实现清单（≤18 步，按依赖顺序）

1. **纠错落地**：按 §2.0 修正 4 处地址/标签；删除 `dst_addr`、`merge_proposal_*` 的本地重复 `try_merge_decorators`/`snapping_direction`；`ui_change_stair_height` 的 2/3 映射对调。
2. **楼梯基础**：`walk_shape`（含 `is_ccw_quad_convex` 内联）+ `compute_stair_assembly_params`（0.25/2.5、子段合并）。
3. **楼梯请求**：`find_alignment_preferences` + `find_best_neighbor_alignments`（f32::MIN、-10 惩罚）+ `generate_playermade_stair_requests` 三阶段。
4. **装配阻塞取证**：先反编译 `playermade_stairs_assemble` 的 4 个闭包（0x140284b0/0x140275b0/0x140286d0/0x14027840）；未取证前保持 U，不写空体。
5. **梯子/踏步/楼板**：`calculate_adjusted_ladder_positions`（0.28/0.3）+ `construct_ladder`（0.65/0.4/-0.593）+ `construct_stair_steps`（0.56/0.6/0.46）+ `gen_stair_floors`（0.15 + `floor_tri_count_to_atlas_idx`）。
6. **结构图**：`create_structural_colliders_and_initial_graph` + `calculate_distortion_from_structural_graph`（0.046875）+ `split_segment_*`（-1.17/0.5）+ `gen_stairs_structural_collider` + `gen_stairs_collider_and_rt_geo`（20 值索引表）。
7. **脏标记/订阅**：`playermade_stairs_interpolate_and_compare`（5e-05、USE_STAIRS_LAZINESS）+ `propagate_dirty_flags` + `add_wall_holes` + `stairs_subscribe_lazy`/`structural_graph_subscribe`。
8. **扶手**：`assemble_railings` + `railings_tips`（0.0975、木/石分支、`resample_curve`）+ `construct_wooden_railing`（1.6）+ `add_corner_stone_railing_piece`。
9. **楼梯装饰视觉**：`generate_playermade_stair_decorators`（0.65/0.95/1.0、0.76/0.66、0x14B 回写）+ `maintain_stair_graph`（级联裁剪/apply op）。
10. **老虎窗碰撞/屋顶**：`get_collision_mesh`（DormerKey round(dist*5)）+ `generate_dormer_roofs`（0x800186/0x17/gothic）+ `assemble_dormer_roof_tiles`/`assemble_gothic_dormer_tiles` + `dormer_roof_height`/`is_bay_window`/`get_dormer_panel_xforms`。
11. **门窗**：`add_door_autoclutter`（0x50B、is_autumn）+ `check_for_door_stairs`（地形/门高）+ `add_lintels`（1.43/0.82/0.64/0.9）+ `get_window_holes`。
12. **差分重装**：`compare_assembly_params`（40B）+ `chimney_reassemble` + `door_stairs_reassemble`（仅变化项）。
13. **灯笼/窗灯**：`maintain_lantern_lights`（0.45..0.72、0.14/0.01、0x8000、28B 记录）+ `compare_light_lists`（0x1cB、tag 0xe、双缓冲）+ `window_lights` 改全局 `fastrand::f32()`。
14. **旗帜/布条**：`greedily_connect`（容量 3/0.02/12.0/遮挡）+ `clothline`（等分 + 0.08 下垂）。
15. **子类型/装载**：`is_half_dormer`（0x160600、flat/gable、最终 lerp 不等式）+ `load_collision_as_bbx`（0x1407bb4b0、Vec2==(1,0) 断言）。
16. **蓝图 6 真缺失**：`clear_blueprints` → `instantiate_decorator_colliders` → `instantiate_blueprint_colliders` → `instantiate_blueprints` → `validate_blueprints` → `show_max_wall_y` → `copy_blueprints_to_prev`（顺序按 §3.1）。
17. **历史/回放/合并**：`record_to_history_edit`（48B，证明前导 1 常量）+ `decorator_record_history`（kind 6）+ `replay_decorator`（用 core timing/cull_allow）+ `find_best_roof/wall_decorator_merge_position` + `try_merge_decorators` 统一。
18. **UI 尾段 + 收口**：`handle_moving_out_*`（TransientDecoratorIdGen + op）+ `rotate_*`（跳表目标）+ `change_stair_height_*`（修正映射）+ `preview_ratio`（公式）+ `clutter_mesh_ty_data`（反编译 0x1408b8e90）；跑 `cargo check -p system-decorator` 与 crate 测试，逐项对照 §5 判据。

---

## 7. 证据索引（绝对路径）

- 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/`
- 主干反编译：`D:/game/reverse-engineering/tiny-glade/evidence/toolchain/decor-trunk/decompiled/`
- visual/楼梯/ui 反编译：`D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/systems-recovery/groupD/{decomp,decorator_visual/decomp,ui_systems/decomp}/`
- 现有实现：`D:/game/tiny-glade/crates/systems/decorator/src/`
- 分析：`D:/game/reverse-engineering/tiny-glade/upper-modules-plan/systems/decorator/ANALYSIS*.md`
- 复核：`D:/game/reverse-engineering/tiny-glade/upper-modules-plan/BATCH3-LOGIC-REVIEW-decorator.md`
- 清单：`D:/game/reverse-engineering/tiny-glade/upper-modules-plan/STATUS-2026-10-19.md`
- 原字节读取：`D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py`（BIN=`workspace/input/tiny-glade.exe`）
- MCP 入口：`D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`
