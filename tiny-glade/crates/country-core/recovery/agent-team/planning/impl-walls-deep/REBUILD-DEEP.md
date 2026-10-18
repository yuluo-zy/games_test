# REBUILD-DEEP — walls 域深判等收口（impl-walls-deep，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。方法：每函数 **fresh 反编译**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`，每批≤20）
> → 分类（宏胶水→标记；真业务→判等/实现）。落盘 `planning/impl-walls-deep/decomp/batchN/`。
> 红线：只以 fresh 反编译为准；宏胶水标记不重建；不删除已判等；不改他模块。

## 状态

- 待办：① 未实现-需重做 10（apply_decorator_change、tracked_clone_from×2、
  snapping_to_ends、telegraph_walls_are_merged、archivist extract/archive 6、
  spatial_hash native-only 4）；② 未覆盖文件（decorator_subtype/inner_walls/
  public_wall_state/inner_wall_state/cached_decorator_transforms/shapes-eraser/
  publish_public_wall_state）；③ ~240 待判等；④ 判等-注意 7。
- **已实施修复**：spatial_hash 4 变体、apply_decorator_change（remove/reinsert +
  逐字段更新判别跳表）、snapping_to_ends（snapped 门双路径 + get_approx 投影 +
  outline 插值）、decorator_subtype can_be_on_corner（位掩码 0x91008 修正）。
  tracked_clone_from×2 与 merge_with 已在 REBUILD-REMAINDER 落地（fresh 复核一致）。
- **判等**：inner_wall_state（is_fence/is_default_arch_height/get_enclosure_params）、
  public_wall_state（wall_space/u_space/wall_space_length/flat_roof_y）、inner_walls
  （new_wall/new_wall_w_params/new_wall_with_id/remove/remove_all/load_state/
  update_* 族）、cached_decorator_transforms（insert/swap/transform/position/
  try_*）、decorator_subtype 表族、eraser（archivist 访问器、ui_bulldozer 条件/
  入队、animate_wall_bulldoze 骨架）、shapes shape_placement_params、
  archivist extract_and_restore_to_storage（经 restore_archived_decorators）。
- **登记-注意（结构差异/宿主边界）**：telegraph_walls_are_merged（render-host
  实体链）、archivist extract_diff/extract_before_after_impl/archive_*（迭代链
  待分批）、eraser finish 11509B、snap_to_roof_ridge（roof 域）。
- **属性疑点**：0x14085a370 = InnerWalls::new_wall_w_params（非 WallState::new）；
  0x140833250 = DecoratorRank::new（非 impl$7）。

## 各批节


### 批 1 — wall_spatial_hash native-only 4 变体（实现，2026-10-17）

依据：`decomp/batch1/0x140aa5850/0x140aa5ba0/0x140aa5c70/0x140aa6600` +
`decomp/batch2/get_cells_from_*`（fresh）。文件：`systems/wall/wall_state/wall_spatial_hash.rs`。

| VA | 函数 | verdict | 实现 |
|---|---|---|---|
| 0x140aa5850 | query_with_segment | 未实现→已实现 | get_cells_from_segment 覆盖格收集 → 排序（到查询段最近点距离）→ 相邻同墙去重 keep 最近；与 query_with_circle 同骨架 |
| 0x140aa5ba0 | query_with_rectangle2d | 未实现→已实现 | Rectangle2d 四角世界坐标 → AABB 覆盖格收集 → 排序去重 |
| 0x140aa5c70 | cell_center | 未实现→已实现 | dims==0 panic + index/dims 拆分 + (coord+0.5)*cell-0.5*extent 中心换算 |
| 0x140aa6600 | bounds_ws | 未实现→已实现 | 读 extent（+0x50）→ {-extent/2,+extent/2}；GRID_SIDE/WORLD_SIZE 常量对应 |

编译：`cargo check -p country-core` 通过（rhapsody 并发 WIP 瞬时报错重跑干净；本批文件 0 error）。

### 批 2 — apply_decorator_change（实现，2026-10-17）

依据：`decomp/batch1/0x140a20890-apply_decorator_change.txt` + fresh disasm
（`/tmp/walls-deep-disasm-result.json`，0x140a20890）+ 调用方 apply_decorator_op
disasm（0x141276e30）。文件：`resources/walls/decorator_storage.rs`。

| VA | 函数 | verdict | 实现 |
|---|---|---|---|
| 0x140a20890 | apply_decorator_change | 未实现-需重做→已实现 | immutable 门 → `apply_decorator_remove`（旧条目 d2s 注销）→ `get_or_insert_default` → `Decorators::insert`（重插 + d2s update）→ `get_mut`（缺失 panic）→ `modified_ids.insert` → **内嵌 map 逐字段更新**（rank/ty/decorator_id/u45/u30 写面；`u30 低 32 位==1` → rank 判别 + u30 低=info.u30 高；`u38&1` → type 判别 + u30 高=info.u3c；判别非法 panic） |

分歧点对照：Rust 此前仅 `get_mut → insert+modified` 简化（无 remove/reinsert 链、
无逐字段更新、无判别 panic）；目标特殊分支（原生 op blob+0x10==3）在 Rust
`DecoratorStorageAddr` 判别枚举（0/1/2）无对应变体——登记判等-注意，正常路径可达。

编译：`cargo check -p country-core` 通过（0 error）。

### 批 3 — snapping_to_ends（实现，2026-10-17）

依据：`decomp/batch1/0x140a6ad80-snapping_to_ends.txt`（fresh 反编译）+ 既有
REBUILD-REMAINDER §4.3 分歧登记。文件：`systems/wall/snapping/snapping_to_ends.rs`。

| VA | 函数 | verdict | 实现 |
|---|---|---|---|
| 0x140a6ad80 | snapping_to_ends | 未实现-需重做→已实现 | ① 去掉自造 `if snapped break`（原遍历全部墙，仅 snapped==0 跳过当前墙 lVar23==lVar20）；② 当前墙 end 标志 = flags bit0（非 0 常量）；③ 3D 投影改为 **游标投影到简化 2D 曲线**（get_approx_u_from_pos 语义：逐段投影取最近 u → coord_at_u (segment,t)）→ **outline 3D 点按 t 插值**（非 nearest_segment） |

分歧点对照：REBUILD-REMAINDER §4.3「投影链 native 用 get_coord_at_u+插值 vs Rust
nearest_segment(outline)」「snapped 门双路径次序」——已重建。编译通过。

### 批 4 — archivist extract/archive（判等/登记，2026-10-17）

依据：`decomp/batch1/0x140b2c4b0/0x140b2cb30/0x140b2be70/0x1412b2a70/0x14123b5c0/
0x1412b4f10`（fresh）。文件：`recovered_building/wall_decorator_archivist.rs` +
`resources/walls/decorator_archivist.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x140b2c4b0 | extract_and_restore_to_storage | 判等（经 restore_archived_decorators） | fresh 结构 = 清 2 碰撞 map → memcpy walls/special1（take）→ chain（walls Drain + special1/2 IntoIter，extract_archived_decorators 闭包）→ 每源 get_or_insert_default + tracked_clone_from\<DecoMoveToStorage\>。Rust `restore_archived_decorators` + `restore_one`（含已重建 tracked_clone_from 快/慢路径 + d2s insert）逐源对应，序 walls→special1→special2 一致 |
| 0x140b2cb30 | extract_diff | 未实现→登记（待分批） | fresh = RawTable clear×2 + chain(Drain + IntoIter×2) filter_map → from_iter\<DecoratorStorageHistoryDiff\>；Rust 无对应（restore 单遍简化） |
| 0x140b2be70 | extract_before_after_impl | 未实现→登记（待分批） | fresh = extract_archived_decorators 闭包 + 历史状态 map from_iter + DecoratorExtraInfo panic 判别；Rust 无对应 |
| 0x1412b2a70 | archive_storage_decorators | 未实现→登记（待分批） | fresh 2902B 归档链；Rust 无对应 |
| 0x14123b5c0 | archive_current_decorators_in_storage | 未实现→登记（待分批） | fresh 6741B 归档读路径；Rust `archive_current_decorators` 为简化单遍（登记） |
| 0x1412b4f10 | archive_current_decorators_in_storages | 未实现→登记（待分批） | fresh 3824B 多存储归档；Rust 无对应 |

编译：`cargo check -p country-core` 通过（本批无改动文件）。

### 批 5 — telegraph_walls_are_merged（登记，2026-10-17）

依据：`decomp/batch1/0x14086ad70-telegraph_walls_are_merged.txt`（fresh，4723B）。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x14086ad70 | telegraph_walls_are_merged | 未实现-需重做（render-host 边界登记） | fresh = 事件驱动：遍历 public wall 顶（+0x118/+0x110/+0x120 三索引点）→ curve try_new_from_points/slice_into + easing + despawn + UiAppendSignifier 实体链。Rust 单墙近似（注释已登记待接线）。**实体链为 render-host 接线边界**（AGENTS 外部依赖分类），算法部分（曲线偏移 + kind-5 signifier 推送）可闭合但本轮按边界登记不改他模块 |

### 批 6 — inner_wall_state.rs（判等，2026-10-17）

依据：fresh 反编译 0x1408c5680/0x1408c5690/0x1408c56b0 + disasm 三者机器码。文件：
`resources/walls/inner_wall_state.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x1408c5680 | StructureType::is_fence | 判等 | `movzx eax,[rcx]; ret` 字节0 直通；Rust `!=0` 等价（调用方 ==0 判别） |
| 0x1408c5690 | is_default_arch_height | 判等 | `cmp_eq_ss value,[0x142a8cc60]`（有序等值，NaN 语义同） |
| 0x1408c56b0 | get_enclosure_params | 判等 | `cmp [rcx+0x38],2; cmovc ptr` = tag<2 → Some(+0x38) |
| 0x14085a370 | WallState::new（归属疑点） | 登记 | fresh 反编译实际名 = **InnerWalls::new_wall_w_params**（+0xc0 id 递增 + map insert + 集合 insert）——Rust 注释将该 VA 归属 WallState::new 有误（属性疑点） |

### 批 7 — public_wall_state.rs（判等，2026-10-17）

依据：fresh 反编译 0x140b2a760/0x140b2a740/0x140b2a770/0x140b2a6e0 + disasm 机器码。
文件：`resources/walls/public_wall_state.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x140b2a760 | wall_space_length | 判等 | `movss [rcx+0x28c]` = original_curve_length |
| 0x140b2a740 | wall_space | 判等 | `cmp [rcx+0x258],2; setnz` + [0x28c]（tag!=2 才返回） |
| 0x140b2a770 | u_space | 判等 | setnz + movss 1.0 常量（tag!=2 → 1.0） |
| 0x140b2a6e0 | flat_roof_y | 判等 | `test byte[0x108],1; jz panic`（高度未算 panic，同 MISSING_HEIGHT）+ `[0x10c]+(-0.79)` = max_y-0.79 |

### 批 8 — inner_walls.rs（判等，2026-10-17）

依据：fresh 反编译 0x14085ad60/0x14085ad40/0x14085a4a0/0x14085a730/0x14085aa60/
0x14085a550/0x14085ad80 + _try_update_state 单态（0x140aaaca0/0x140aa99e0/
0x140aa9eb0/0x140aa9800）。文件：`resources/walls/inner_walls.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x14085a550 | reset_max_id | 判等 | `*(rcx+0xc0)=0` 清计数器 ✓ |
| 0x14085ad60 | update_wall_state_height | 判等 | 转发 `_try_update_state` 闭包族；Rust try_update 同构 |
| 0x14085ad40 | update_arch_type | 判等 | 同上 |
| 0x140aaaca0/99e0/9eb0/9800 | _try_update_state（height/freehand/arch/supports 闭包） | 判等 | 存在性门 + updated 通知 + 闭包更新；Rust try_update |
| 0x14085a4a0 | new_wall_with_id | 判等 | map insert（冲突 panic）+ created 通知 + 不推进 +0xc0 |
| 0x14085aa60 | remove | 判等 | 存在断言 + removed 通知 + 移除权威行 |
| 0x14085a730 | remove_all | 判等 | 全键 removed 通知 + 清空 |
| 0x14085ad80 | load_state | 判等-注意 | 前置断言（walls/各内层空）+ 逐记录解码（O 字段 + 曲线区待①，Rust 已登记） |
| 0x14085a370 | new_wall_w_params | 判等 | +0xc0 递增（替换旧 id 也递增）+ map insert + 集合 insert（fresh 0x14085a370 证实） |

### 批 9 — decorator_subtype.rs（判等/修复，2026-10-17）

依据：fresh 反编译 0x140b25b50/0x140b25c90/0x140b25ca0/0x140b25b60/0x140b262a0/
0x140b26690/0x140b267f0/0x140b26970/0x140b26980/0x140b25ef0/0x141fdc7b0/
0x140833250/0x140833ba0 + disasm 小函数。文件：`resources/walls/decorator_subtype.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x140b25b50 | can_be_on_corner | 未实现→已修复 | fresh disasm：`mov ecx,0x91008; bt ecx,eax; setc al`——**位掩码成员测试**（bit3/12/16/19），非 restorer 的 `==3` 近似——已重建 |
| 0x140b25c90 | DecoratorLibKey::new | 判等-注意 | `mov eax,edx; mov edx,ecx` 两参交换（PDB 注释已登记同址交换）；字段序语义 U |
| 0x140b25ca0 | DecoratorCollision::empty | 判等 | 0x50+ 字节全零初始化（Rectangle3d+cuboid+IsoTransform 三连） |
| 0x140b25b60 | DerivedDecoratorSubtypes::get | 判等 | map 探针 + 未命中 panic（Rust panic 文案一致） |
| 0x140b262a0 | DecoratorMeshLibrary::insert | 判等 | Arc 克隆后插 4 表（collision/interaction/outline/collision_outline） |
| 0x140b25ef0 | insert_collision_and_interaction_as_same | 判等 | collision+interaction 插同一值 |
| 0x140b26690 | get_outline | 判等 | map3@+0x60 查询 |
| 0x140b267f0 | get_collision_outline | 判等 | map4@+0x90 查询 |
| 0x140b26970 | DerivedDecoratorInfo::subtype | 判等-注意 | `mov rcx,[rcx]; jmp get` 尾跳（读首字段后转发）；Rust 直读字段等价 |
| 0x140b26980 | try_get_subtype | 判等-注意 | Option 门 + alt 语义 U（Rust `Some(subtype)` 近似登记） |
| 0x141fdc7b0 | bbx_ls | 判等 | bbx 差值库空间（13KB 大函数，结构对应） |
| 0x140833250 | impl$7 into_decorator_subtype | 判等-注意 | 实际名 = **DecoratorRank::new**（`mov eax,ecx; ret`）——Rust 注释归属 impl$7 有误（属性疑点） |
| 0x140833ba0 | impl$6 into_decorator_subtype | 判等 | 尾跳转发取 subtype |

编译：`cargo check -p country-core` 通过。

### 批 10 — cached_decorator_transforms.rs（判等，2026-10-17）

依据：fresh 反编译 0x140a1ada0/0x140a1aeb0/0x140a1aa20/0x140a1a9f0/0x140a1ac80/
0x140a1ab80/0x140a1af40/0x14152aee0。文件：`resources/walls/cached_decorator_transforms.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x140a1ada0 | insert | 判等 | 3 组有限性校验（translation/rotation/scale，0x7f7fffff NaN/Inf 门）+ 落缓存 |
| 0x140a1aeb0 | swap_buffers_and_clear | 判等 | 双缓冲 0x30B 逐字段互换 + 清一册（0xff 填充 + cap 重算） |
| 0x140a1aa20 | transform | 判等 | 读取册查表 + 未命中 panic |
| 0x140a1a9f0 | position | 判等 | 转发 transform 取 translation |
| 0x140a1ac80 | try_transform | 判等 | Option 版查表 |
| 0x140a1ab80 | try_position | 判等 | Option 版 position |
| 0x140a1af40 | storage get | 判等 | 探针头查表（未命中 None） |
| 0x14152aee0 | insert\<Transform\> 落缓存 | 判等 | map insert（依赖容器） |

### 批 11 — eraser archivist.rs（判等-注意，2026-10-17）

依据：fresh 反编译 0x14092e6e0/0x14092ea50/0x14092e8e0/0x14092eff0/0x14092f990。
文件：`systems/wall/eraser/archivist.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x14092e6e0 | new | 判等 | hashmap 随机键初始化（双 map） |
| 0x14092ea50 | cache_if_original_wall | 判等 | entry-or-insert 快照 + range 更新 |
| 0x14092e8e0 | record_rewiring_event | 判等-注意 | native 插 **HashMap\<OriginalWallId, RewireEnclosedWall\>**（重接线 map）；Rust 写入 affected 快照 map——结构差异登记（RewireEnclosedWall 域在 recovered_building 墙重接线边界） |
| 0x14092eff0 | shrink_wall | 判等 | 既有 range 合并 min/max + 缺失插 |
| 0x14092f990 | finish（11509B） | 未实现-需重做（登记） | fresh 11509B 大函数（RoofDelete + Wall compound history + ChildWalls 重接线配对）；Rust `finish_wall_eraser` 为简化单遍（record_wall_bulldoze 转发）——Entity/ChildWalls 重接线为宿主边界，算法闭合待分批 |

### 批 12 — eraser ui_bulldozer + animate_wall_bulldoze（判等，2026-10-17）

依据：fresh 反编译/disasm 0x1409de170/0x1409de190/0x140ac27e0/0x1412f6a20。
文件：`systems/wall/eraser/ui_bulldozer.rs` + `animate_wall_bulldoze.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x1409de170 | wall_eraser_bulldoze_condition | 判等 | disasm：`[*rcx]==9 && [*rcx+4]==3`（AppMode tag 9 / sub 3）= Rust `AppMode::Eraser(EraserLayer::Bulldoze)` |
| 0x1409de190 | wall_eraser_bulldoze | 判等 | 命令入队 + 动画缓冲 WallId 去重 + InnerWalls 快照 |
| 0x140ac27e0 | animate_wall_bulldoze | 判等-注意 | phase 推进 dt/0.2 + CubicInOut ease 切片曲线 + phase==1 移除权威行 + 152B 命令记录；Rust 结构对应（ease 语义 I 已登记） |
| 0x1412f6a20 | retain finished | 判等 | Vec retain（finished 行丢弃） |
| 0x140a82b20 | new_edit_sequence（同址） | 判等 | `{ptr=0,cap=8,len=0}`（REBUILD §1.10 已判等，复核一致） |

### 批 13 — shapes shape_placement（判等-注意，2026-10-17）

依据：fresh 反编译 0x14097f970/0x140980db0/0x140ac80e0。文件：
`systems/wall/shapes/shape_placement.rs` + `calc_shape_height.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x14097f970 | shape_placement_params | 判等-注意 | 游标地形位 → query_with_aabb ±8 → 1.5 内取最近墙 → 法线（premade shape 法线 / freehand 切线法线）→ 落 placement；结构对应（法线推导细节 I 登记） |
| 0x140980db0 | snap_to_roof_ridge | 未实现-需重做（登记） | fresh 1494B：Roof::pivot_3d / RoofRidgeDims::calc 山脊几何步行；Rust 为占位近似（roof 域 host wiring 边界） |
| 0x140ac80e0 | calc_shape_height | 判等（LOGIC §2 复核一致） | 采样求和/最大 + 基高 6/8 + clamp[0.7,14]（lerp_rate 常量近似登记） |

### 批 14 — publish_public_wall_state（登记，2026-10-17）

依据：LOGIC §5 登记 + 文件结构（11198B 大函数）。文件：
`systems/wall/wall_state/publish_public_wall_state.rs`。

| VA | 函数 | verdict | 依据 |
|---|---|---|---|
| 0x140a8b530 | publish_public_wall_state | 判等-注意（登记） | Rust 为 `consume_with_paths` 包装（恢复路径栅格 → public.consume + dirty 清 + OnWallChanged/Deleted 事件）；主逻辑在 recovered_building wall_collection 域（11198B 主体不在 wall_spatial_hash 域内） |

编译：`cargo check -p country-core` 通过。

## 汇总（2026-10-17）

- **已实施修复 5**：spatial_hash 4 变体（query_with_segment/rectangle2d/cell_center/
  bounds_ws）、apply_decorator_change（remove/reinsert + 逐字段更新判别跳表：
  u30低32位==1→rank判别/u38&1→type判别）、snapping_to_ends（snapped 门双路径 +
  get_approx 投影 + outline 插值）、can_be_on_corner 位掩码（0x91008 修正）。
- **判等 8 批**（~45 函数）：inner_wall_state/public_wall_state/inner_walls/
  cached_decorator_transforms/decorator_subtype 表族/eraser 访问器+系统骨架/
  archivist extract_and_restore_to_storage/shapes shape_placement_params。
- **判等-注意/登记**：telegraph（render-host）、archivist extract_diff 等 5（待分批）、
  eraser finish 11509B、snap_to_roof_ridge、publish_public_wall_state。
- **属性疑点 2**：0x14085a370=InnerWalls::new_wall_w_params、0x140833250=DecoratorRank::new。
- **编译/测试**：`cargo check -p country-core` 通过；decorator_storage 12 测试 +
  snapping 8 + decorator_subtype 4 通过（test 目标的 rhapsody 并发 WIP 报错与本批
  文件无关，见 REBUILD-REMAINDER 同款登记）。
- **剩余**：~240 待判等（未 fresh 复核）、archivist 迭代链 5、eraser finish、
  telegraph/snap_to_roof_ridge 宿主边界——留待后续分批。

<!-- 每批追加一节 -->
