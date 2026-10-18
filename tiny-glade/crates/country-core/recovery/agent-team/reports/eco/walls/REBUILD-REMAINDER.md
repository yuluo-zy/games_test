# REBUILD-REMAINDER — walls 域待判等/未实现余量分批处理（impl-walls-remainder，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。方法：每函数 **fresh 反编译**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`，每批≤20）
> → 分类（宏胶水→标记；真业务→判等/实现）。落盘 `planning/impl-walls-remainder/decomp/batch1..5/`。
> 红线：只以 fresh 反编译为准；不删除已判等函数；不改他模块。

## 0. 本轮覆盖与改动

- fresh 反编译 **5 批 78 个** walls 域函数（decorator_storage 全访问器/迭代族、apply_* 三件、
  restore_snapshot 节格式、wall_compound_edit builder/fmt、valid_enclosure 几何族、
  wall_spatial_hash 全查询族、snapping_to_ends/telegraph、archivist 访问器）。
- **已实施修复 3 项**（fresh 证据支撑）：
  1. `apply_decorator_place`（0x140a20c70）state==2 强制 + d2s insert/update 分支（REVIEW §2 登记项）。
  2. `apply_decorator_remove`（0x140a20670）d2s 移除 miss==3 → panic（REVIEW §2 登记项）。
  3. `query_with_circle`（0x140aa5940）补圆心距离排序 + 相邻同墙去重（fresh 反编译 0x140aa5940
     含 smallsort/ipnsort + +0x30 同墙折叠；Rust 此前无序直收）。
- 编译：`cargo check -p country-core` **通过**（rhapsody 崩溃为并发 WIP 瞬时，重跑干净；
  本批改动文件 0 error）。

## 1. 已实施修复（附 fresh 分歧点）

### 1.1 apply_decorator_place 0x140a20c70（decorator_storage.rs）
- fresh：`get_or_insert_default()` + `Decorators::insert<DecoMoveToStorage>`（0x14155afe0）。
  insert 内：removed_ids remove + modified insert + 内嵌 map insert 后 **`if(state==2)`
  → d2s insert（冲突 panic）+ 写状态字节 2@+0x47；else → 拷贝输出 + d2s update**。
- 旧 Rust：`insert + 无条件 d2s.insert`（state 传什么存什么）。
- 修复：`info.state==2 → d2s.insert + placed.state=2；else → d2s.update`（逐分支与机器码对）。

### 1.2 apply_decorator_remove 0x140a20670（decorator_storage.rs）
- fresh：modified 移除 + removed 标记 + 内嵌 map 移除（miss fmt panic）+ **d2s remove_entry
  （miss==3 → panic，行 53/66）**。
- 旧 Rust：`let _ = d2s.remove()` 忽略 miss。
- 修复：`d2s.remove().expect(...)`（miss 同 panic）。

### 1.3 query_with_circle 0x140aa5940（wall_spatial_hash.rs）
- fresh：`get_cells_from_circle` → 收集 → **`sort_unstable_by`（<0x15 smallsort else ipnsort）**
  → **相邻 `*(seg+0x30)`（wall id）同墙去重 keep 最近**（行 108-150）。
- 旧 Rust：按 cell 直接遍历回调（无序、不去重）。
- 修复：收集→按圆心距离 total_cmp 排序→同墙去重→回调（保留 dedup-then-filter 顺序：
  同墙最近段在半径外则该墙不回调）。

## 2. 判等（fresh 逐条对应；含 LOGIC §5/§6 待判等项复核结论）

| 批 | VA | 函数 | 判等依据 |
|---|---|---|---|
| 1 | 0x140a1b030/070/0b0/100 | iter_modified(_ids/_mut)/iter_removed_ids | modified/removed 集合迭代器构造（+0x60/+0x0）+ 闭包捕获自指针；与 Rust `impl Iterator` 语义一致 |
| 1 | 0x140a1b140 | mark_all_modified | 遍历内嵌 map 逐 id `insert<tuple<>>`（modified 集合）；= Rust `modified_ids.extend(keys)` |
| 1 | 0x141257900 | clear_modification_tracking(clear) | 清 modified 集（memset 0xff+len0+cap 重算）+ 清内嵌 map@+0x30；= Rust `modified_ids.clear()+decorators.clear()` |
| 1 | 0x14155afe0 | insert\<DecoMoveToStorage\> | 前三步 removed remove+modified insert+map insert ✓；state 分支由 1.1 修复承担 |
| 1 | 0x140a1b440/4e0/5a0/700 | d2s remove/insert/update/impl$10 | 存在性守卫/冲突 panic/探针 update/断言 remove；= Rust assert_exists/insert/update |
| 1 | 0x140a1c240/290 | as_wall/try_as_wall | disc==0 → 载荷/None；✓ |
| 2 | 0x140a1c2c0/1cde0 | get/get_mut | per_wall/terrain/stair 三路 + immutable 门；LOGIC §2 已判等，fresh 复核一致 |
| 2 | 0x140a1c5f0/1c800/1c930/1cbf0 | decorator_info(_mut)/lookup_decorator_info(_mut) | d2s 24B 条目探针 → per_wall 0x98 步长 → 内嵌 map 0x50B 双层查表；= Rust Option 链 |
| 2 | 0x140a1c510 | try_lookup_storage | Option 版（无 miss panic）；= Rust |
| 2 | 0x140a1dba0 | remove_storage | immutable 门 → per_wall/内联桶移除 + d2s 索引清理；✓ |
| 2 | 0x140a1e190/280/2c0/300/420/540/5b0/630/6e0 | wall_decorators(_ids/_mut)/iter_terrain/stair/iter_walls_mut/iter/iter_mut/iter_removed_storages | 迭代器构造族；= Rust |
| 2 | 0x140a1e720 | DecoratorStorage::clear_modification_tracking | per_wall+terrain+stairs 逐桶 clear；= Rust |
| 2 | 0x140a1f550 | on_wall_removed | immutable 门 + remove_storage；= Rust |
| 3 | 0x140a20d70 | restore_snapshot（REBUILD 已重建） | 前置 6 断言 + get_or_insert_default + removed/updated/内嵌 map/d2s insert；fresh 复核一致 |
| 3 | 0x140a82b20/2b40/2d50 | CompoundEditBuilder new/next/build | REBUILD §1.10-12 已判等；fresh 复核一致（next alloc 0xf8+0x108、grow_one、push 无 is_empty 门） |
| 3 | 0x140a82e20 | panic_deprecated_operation | panic_fmt 恒定；Rust `panic!`（文案差异登记，判等-注意） |
| 3 | 0x1411ff2b0/2f0/600/620/640/660/6f0/740 | valid_enclosure as_curve2/3、get_normal/get_pos/perimeter/get_closest_u、as_circle/as_rectangle | 位 0 判别分发 ✓（LOGIC §2 已判等，fresh 复核一致） |
| 3 | 0x1411ff2a0 | center | 15B 载荷读取（payload[tag*8] 双 f32）；= Rust |
| 4 | 0x140aa5dd0 | add_segment | +0x58 id 递增 + get_cells + 逐格 push + wall_segments/segment_cells insert + segments insert；= Rust |
| 4 | 0x140aa5360 | remove_wall_id | wall_segments remove_entry → 逐格 member 删除（0x38B 步长）+ cell len 减；= Rust |
| 4 | 0x140aa5730 | query(单点) | world→grid→cell 索引→member 列表指针；Rust 经 recovered_building cell_index_of_world_point/segments_in_cell 等价暴露 |
| 4 | 0x140aa6520/65d0/65e0/6600 | world_space_to_grid_coord_unclamped/grid_coord_to_cell_index/cell_size/bounds_ws | floor((ws+0.5*extent)/cell)、y*dims.x+x、{+0x50/dims}、{-extent/2,+extent/2}；= recovered_building wall_spatial_index |
| 4 | 0x140aa5c70 | cell_center | dims==0 panic + index/dims 拆分 + 中心换算；= Rust 换算族 |
| 5 | 0x140b2bbf0 | decorator_cull_allow_any | 写 +0x1e0=1 标志；= Rust allow_any=true |
| 5 | 0x140b2bd90 | is_decorator_cull_allowed | 标志短路 + +0x30 集合成员探针；= Rust allow_any \|\| contains |

## 3. 已判等-注意（结构对应，差异登记）

| VA | 函数 | 差异 |
|---|---|---|
| 0x140a1c3e0 | lookup_storage | native **miss → panic_fmt**（infallible）；Rust 返回 Option。native 有独立 try_ 变体（0x140a1c510）；Rust 将两者合一为 Option。调用方（ui_focus/save）按 Option 使用，维持登记不改签名（改他模块红线）。 |
| 0x140a1b5a0 | d2s update | native 探针 **miss → panic_fmt**（0x140a1b5a0 尾部）；Rust `update` 返回 Option 不 panic。调用方（merge_with/on_wall_split）均在 d2s.remove 后调用，条目必在，miss 不可达——登记不改。 |
| 0x141256e40/0x1412573b0 | tracked_clone_from ×2 | native **慢路径**：清 modified → 当前 decorators 逐 id 标 removed（insert tuple<>）→ 清 map → 旧条目 d2s remove（state==2 break）→ 源 removed_ids 回填 → 源 decorators 重插（removed remove + modified insert）→ d2s insert。Rust 简化为 clone_from+clear+d2s.insert，**缺 d2s remove 旧条目与 removed 标定** → 未实现-需重做（见 §4.2） |
| 0x140a20890 | apply_decorator_change | native：目标特殊（ppcVar3==3）走单独分支；否则 **apply_decorator_remove + get_or_insert_default + insert\<DecoMoveToStorage\>**，随后内嵌 map 逐字段更新（含 DecoratorExtraInfo/rank/type 判别 panic 跳表）。Rust 仅 `get_mut → insert+modified` 简化 → 未实现-需重做（见 §4.1） |
| 0x140a87970 | WallHistoryCompoundEdit::fmt | `debug_tuple_field1_finish`（derive Debug 胶水，单字段 tuple）；Rust derive Debug ✓ → glue-legitimate |
| 0x140a82eb0 | WallHistoryEdit::fmt | niche 跳表 0x142ad888c 判别 Debug；Rust derive Debug（枚举 shape 为 4 变体 vs native 17-op，REBUILD 登记） → derive 存在即 glue，shape 差异另行登记 |
| 0x140a6ad80 | snapping_to_ends | LOGIC §5 待判等大函数；fresh：`if(*pcVar4!=1) return` 模式门 + snapped==0 跳过当前墙（lVar23==lVar20）双路径 + get_coord_at_u 3D 投影 + 1.5 阈值 + 哨兵 1e8 + mode=2 写回。Rust 结构对（阈值/哨兵/snapped/mode 写），但 **投影用 nearest_segment（Rust）vs get_coord_at_u+插值（native）**，且当前墙跳过条件与 Rust `if snapped break` 次序不同 → 未实现-需重做（见 §4.3） |
| 0x14086ad70 | telegraph_walls_are_merged | native 大函数（事件流 + curve try_new_from_points/slice_into + easing + despawn + signifier insert 链）；Rust 简化为逐墙 UiAppendSignifier 近似（注释已登记待接线）→ 未实现-需重做（见 §4.3） |
| 0x140ac1f20 | adjust_wall_height_impl | native 0x30B 命令 + clamp 回退 + 地形采样 + `_try_update_state`×2 + 事件；Rust adjust_wall_height 委托 recovered_building wall_height_command（clamp/sample/assign×2/feedback 逐条对应）→ 判等（结构），算法在 recovered_building 域 |
| 0x1409dd790 | history_finish_recording | native 多事件游标 + extract_diff×2 + finish_input_recording + +0x1b0 置位；Rust history_recording 为 Wall 域事件接线（record_wall_history）→ 非 wall 域，LOGIC §6 已列待判等，维持登记 |

## 4. 未实现-需重做（fresh 分歧，需重建）

### 4.1 decorator 余量
- **apply_decorator_change 0x140a20890**：见 §3。目标特殊分支 + remove/reinsert 链 + 内嵌 map
  逐字段更新（ExtraInfo/rank/type 跳表 panic）Rust 全缺。

### 4.2 decorator_storage tracked_clone_from
- **tracked_clone_from 0x141256e40/0x1412573b0**：见 §3。native 慢路径对自目标先标 removed、
  d2s 逐旧条目 remove（state==2 break），再回填源 removed 与 decorators；Rust clone_from 近似不维护
  d2s 移除与 removed 标定。恢复语义（history replay_pop_bucket 消费）需按 native 三阶段重建。

### 4.3 wall 系统余量
- **snapping_to_ends 0x140a6ad80**：投影链 native 用 `get_coord_at_u`+曲线点插值（3D snap 点），
  Rust 用 `nearest_segment(outline)`；snapped 门双路径次序需对齐。
- **telegraph_walls_are_merged 0x14086ad70**：native 事件驱动曲线构造+easing+despawn+signifier
  实体链；Rust 单墙近似。实体链为 render-host 接线边界（AGENTS 外部依赖分类），算法部分可闭合。
- **wall_spatial_hash native-only 变体**（fresh 0x140aa5850/0x140aa5ba0/0x140aa5c70/
  0x140aa6600）：`query_with_segment`、`query_with_rectangle2d`、`cell_center`、`bounds_ws`
  在 Rust 无实现（仅 ivy_grower 注释引用 query_with_segment 语义）——native 均为
  get_cells_from_* + 排序/去重骨架，需按 query_with_circle 同法重建。

### 4.4 archivist 大函数（decorator_archivist.rs 22 待判等）
- extract_diff 0x140b2cb30 / extract_and_restore_to_storage 0x140b2c4b0 /
  archive_storage_decorators 0x1412b2a70 / archive_current_decorators_in_storage(s)
  0x14123b5c0/0x1412b4f10 / extract_before_after_impl 0x140b2be70：fresh 结构 = 多闭包
  filter_map/chain/Drain + tracked_clone_from + 碰撞 clear。Rust `restore_archived_decorators`
  为简化单遍；**未逐句判等**（待单独分批，本轮登记不重建）。

## 5. 宏胶水（glue-legitimate，不重建）

- decorator_storage：derive Clone/Debug/Default、serde visitor 族（0x141560920/8f0/650/c80/dc0 等）、
  `impl$15/16::from`（xor eax 零构造/单字段）。
- wall_compound_edit：serde visitor 族（0x141401960/0x140314a00/0x1414022c0/0x141402680/
  0x141402790 及 ~40 visit_str/visit_map/deserialize monomorph，章节已列）、derive clone/fmt。
- valid_enclosure：serde visitor 族（0x1416ebaf0/eb640/ebea0/ebf70/ec140/eca30 等）。
- archivist：bevy SystemParam init_state/validate_param/register_required_components、
  `0x14003bb80` 空函数族（decorator_subtype::queue 同址 no-op）。
- wall_spatial_hash：`0x1409ef870/9eff80/9f08f0/9f0e80/9f12e0/9ef210` get_cells_from_* 闭包入口。

## 6. 自检与结论

- **判等/判等-注意**：~66 个（5 批 fresh 复核；含 LOGIC §2 已判等项复核）。
- **已修复**：3（apply_decorator_place state==2、apply_decorator_remove d2s panic、query_with_circle 排序去重）。
- **未实现-需重做（本轮登记不重建）**：apply_decorator_change、tracked_clone_from×2、
  snapping_to_ends、telegraph_walls_are_merged、archivist 5 大函数（extract/archive 族）。
- **宏胶水**：~12+（serde/derive/注册/空函数）。
- 编译：`cargo check -p country-core` 通过；walls 域单测 26 passed（decorator_storage 12 含修复后 roundtrip）。
- 未覆盖：decorator_subtype.rs 20、inner_walls.rs 26、public_wall_state.rs 12、
  inner_wall_state.rs 11、cached_decorator_transforms.rs 7、shapes 其余、eraser archivist finish/
  ui_bulldozer/animate_wall_bulldoze、publish_public_wall_state（11198B）——留待后续分批。
