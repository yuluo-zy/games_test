# LOGIC — walls 域逐函数逻辑判等（logic-verifier-walls，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。本报告仅以 **fresh 反编译**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`，
> 2026-10-17）为准；不复用此前 reviewer/restorer 描述。
> 判等标准：每个分支/循环/调用/数据流在 fresh 反编译中可逐条对应，无遗漏无编造。

## 0. 覆盖与结论概览

- 本轮 fresh 反编译 **146 个** walls 域业务函数（重点：decorator_storage 增删查/墙事件迁移、
  DecoratorInfo 读取、valid_enclosure contains/expand、snapping/eraser、shapes、WallCompoundHistoryEdit
  Builder、wall_spatial_hash）。
- 逐语句判等完成 **~55 个**；**确认 11 个函数为「未实现-需重做」**（含用户硬标准关注的
  on_wall_split、contains_shape、expand 等——**结构对上但关键内部逻辑是自造近似**）。
- 其余已实现业务函数（~240 个）**未在本轮 fresh 逐句判等**，按协议一律**不得标判等**，
  登记为「待判等」（见 §6）。绝不以报告/测试通过替代 fresh 复核。

## 1. 严重分歧（未实现-需重做，附 fresh 分歧点）

### 1.1 `DecoratorStorage::on_wall_split` 0x140a1f590（3410B）
- Rust body：对每个装饰统一 `d2s.remove + 子墙桶 get_or_insert + +0x30 注册表 insert removed`。
- fresh 反编译：**状态字节判别分支** `(b-2)<=2 ? b-1 : 0`（行 317-318 ✓ 与 Rust `side_from_state`
  一致），但：
  - **侧 0**（`lVar21==0`）走 `Curve::get_approx_u_from_pos<Vec2>` + clamp[0,1] + {low,high} f32
    对数组**包含搜索定子墙**；命中 → `per_wall[child].insert + Decorators::insert<DecoMoveToStorage>`
    （**直接拷回子墙桶**）；未命中才落注册表。
  - **侧 1**（`lVar21==1`）走 `rustc_entry<DecoratorStorageAddr,HashSet>` + set insert（注册表）。
  - **侧 ≥2** → `panic()`（unreachable ✓）。
- 分歧点：Rust **删除了侧 0 的 u 区间包含搜索与直接回插子墙**，把侧 0/侧 1 合并为同一
  removed-注册表路径；`children[side]` 直接下标**替代**了原 u 区间归属判定。
  → **未实现-需重做**（注释已登记"侧 0 原还经 u 区间包含搜索"但未实现）。

### 1.2 `Decorators::merge_with<DecoMoveToStorage>` 0x14155b170（1964B）
- Rust body：`for (id,info) in source: 冲突→removed_ids.insert; 否则内嵌 map 插入`——纯 map 合并。
- fresh 反编译：**三循环**（① 迭代 source 内嵌 map，目标无此 id → modified_ids 传播；② 迭代目标
  内嵌 map → **逐 id d2s remove（miss==3 panic）+ d2s insert(merged)**（`local_120==3` panic 行
  329、`extraout_RAX_00!=3` panic 行 348）；③ 迭代 source.removed_ids → d2s remove + removed_ids
  remove + modified insert + **DecoratorInfo 重插** + d2s update/insert（`local_e1==0x02` 分支，
  行 426 状态字节 0x02 break）。
- 分歧点：Rust merge_with **完全不含 d2s 重索引与 removed_ids 迁移**（原操作把这些内置于
  merge_with；Rust 挪到 on_walls_merged 调用方）。restorer 报告对 merge_with 的"源有自无→插入；
  冲突→标 removed"描述与机器码不符。
  → **未实现-需重做**。

### 1.3 `DecoratorStorage::on_walls_merged` 0x140a1ea40（2131B）
- fresh 反编译：双源 `remove_entry` 后，**仅当两源都存在**才调 `merge_with`（d2s 重索引在
  merge_with 内）；单源路径**逐装饰 d2s remove（miss==3 panic）+ d2s insert(merged)**，随后
  `per_wall.insert(merged, 源)`。
- Rust body：恒建 `target=default`，两源各先 `assert_exists+d2s.remove` 再 merge_with，最后对
  target 全键 `d2s.insert`。
- 分歧点：① 结构不逐条对应（原 merge 入 A + d2s 内置于 merge_with；Rust 分离）。② **重复 id
  跨源**时 Rust 第二个源的 `assert_exists` 会 panic，而原 merge_with 对冲突走 removed 去重
  （不 panic）。③ 单源路径原不 panic，Rust 亦不 panic ✓，但 d2s 时机不同。
  → **未实现-需重做**。

### 1.4 `DecoratorStorage::restore_snapshot` / `take_snapshot` 0x140a20d70（607B，同址）
- fresh 反编译：前置 6 断言（+0x1b0 immutable / per_wall / +0x90 terrain / +0x120 stairs /
  removed +0x48 / d2s +0x78）✓ 与 Rust 一致；随后**逐 0x28B 记录**：`get_or_insert_default` →
  **状态判别跳表**（`&DAT_142ac9bdc/142ac9bfc` switch，行 206-283 读 0x28B 记录各字段）→
  `removed_ids.remove + modified_ids.insert + HashMap::insert<DecoratorId, DecoratorInfo>`（**恢复
  DecoratorInfo 入内嵌 map**）→ 状态字节分支 → `d2s insert`（冲突 panic）。
- Rust body：仅 `removed_ids.insert(id) + d2s.update`，**不恢复 DecoratorInfo**；`take_snapshot`
  只断言返回空。
- 分歧点：Rust 是 16B `DecoratorAddrSnapshot` 近似模型（注释已登记"与原生 0x28B/0x16B 节格式
  不符"），fresh 证实原**确实回填内嵌 map 并逐状态还原**。按协议 → **未实现-需重做**。

### 1.5 `ShapeParameters::contains_shape` 0x1411ff330（720B）
- fresh 反编译四路：
  - 自=圆 / 他=圆：**6 次** `Circle2d::get_pos_at_u + contains_point`（6 采样点）。
  - 自=圆 / 他=矩形：**4 次** `Rectangle2d::get_point + Circle2d::contains_point`（4 角）。
  - 自=矩形 / 他=圆：6 次 get_pos_at_u + Rectangle2d::contains_point ✓。
  - 自=矩形 / 他=矩形：4 次 get_point ✓。
- Rust body：`Self::Circle` 分支**只取 `other.get_pos_at_u(0.0)` 单点**（圆他/矩形他共用）。
- 分歧点：自=圆分支应 6 采样（圆他）/ 4 角（矩形他），Rust 只 1 点。restorer §11.3 的
  "u=0 边界点纠偏"修复与机器码不符（原非单点）。
  → **未实现-需重做**。

### 1.6 `ShapeParameters::expand` 0x1411ff680（105B）
- fresh 反编译：tag==0（圆）→ **仅 r(radius) += amount**；+0x14/+0x18 **保持原值不变**
  （`~uVar7 & (2a+fVar5) | fVar5 & uVar7`，uVar7 圆时为全 1 → 保留原 fVar4/fVar5）。
  tag≠0（矩形）→ +0x14/+0x18 覆写 `2a`。
- Rust body：圆分支 `for [0x14,0x18]: value = self.float_at(offset) + twice`——**把 +0x14/+0x18
  加了 2a**（应原值不变）。
- 分歧点：restorer 对圆车道"+0x14/+0x18 车道 +=2a"的描述与机器码相反（原为保留）。
  → **未实现-需重做**。

### 1.7 `snapping_maintain` 0x140a6ad00（27B）
- fresh 反编译：按 mode 跳表（case0/1/2/3）读 snapped/snap/wall/end，**整体发布到 +0x20 区**
  （`pbVar2[0x20]=mode、[0x21]=snapped、[0x24]=snap、[0x30]=wall、[0x38]=end`）。
- Rust body：仅 `mode.last_published_mode = mode.mode`（1 字节）。
- 分歧点：原发布**全状态副本**到 +0x20..0x38（SnappingMode 布局 = 活动区@+0x0..0x1c +
  发布区@+0x20..0x38）；Rust 只存模式字节。`snapping_audio` 去重逻辑仍等价（只比模式字节），
  但发布区其余字段未重建。
  → **未实现-需重做**。

### 1.8 `snapping_to_segments` 0x140a6b5e0（941B）
- fresh 反编译：`query_with_aabb_clamped` → `from_iter<SnapProposal, filter_map<closest_segment_to_snap_to
  ::closure_env_0>>` → **`sort_by`（smallsort/driftsort）** → 取 `*local_a8`（**排序后最近**）→
  `SQRT(dist) < 1.5` 才写 mode=3。
- Rust body：遍历 candidates 取**第一个** distance<1.5（无排序）。
- 分歧点：原取最近候选，Rust 取任意首个；空间哈希返回序非距离序时结果不同。
  → **未实现-需重做**。

### 1.9 `WallSpatialHash::query_with_aabb_clamped` 0x140aa5a10（394B）
- fresh 反编译：`get_cells_from_aabb` 后 **`sort_unstable_by<...query_with_circle::closure_env_1>`
  排序 WallSegment 列表**（复用圆查询比较器）再收集。
- Rust body：直接遍历 cell 收集，**无排序**。
- 分歧点：原返回按距离比较器有序；Rust 无序。与 1.8 联动（消费者依赖最近优先）。
  → **未实现-需重做**。

### 1.10 `CompoundEditBuilder::new_edit_sequence / next / build` 0x140a82b20/2b40/2d50
- fresh 反编译：`new_edit_sequence` 只写 Vec {ptr=0, **cap=8**, len=0}（无 started 标志位）；
  `next` alloc 0xf8 + memcpy **单条 edit** → `grow_one` → push 新组（无 is_empty 门）；`build`
  直接 `from_iter<ConcurrentEdits, ...>` 收集（无 None/空判据）。
- Rust body：`started=true + current.clear()`；`next` `if !current.is_empty() { groups.push(take) }`；
  `build` `if !started || groups.is_empty() { None }`。
- 分歧点：Rust 引入的 `started` 标志与 `next` 空门在机器码中无对应；`build` 的 None 分支无对应。
  → **未实现-需重做**（builder 精确语义重建待机）。

## 2. 已判等（fresh 逐条对应，无遗漏）

| VA | 函数 | Rust 位置 | 判等依据 |
|---|---|---|---|
| 0x140a1c2b0 | mark_mutable | decorator_storage.rs:412 | 写 +0x1b0=0 ✓ |
| 0x140a1c2c0 | get | decorator_storage.rs:392 | disc 分支 + per_wall/terrain/stair 门 ✓ |
| 0x140a1cde0 | get_mut | decorator_storage.rs:402 | +0x1b0 panic + 三路分支 ✓ |
| 0x140a1cf30 | get_or_insert_default | decorator_storage.rs:459 | 墙探针(0x98) + 注册表 take + terrain/stair 懒初始化(+0x90/+0x120) ✓（fresh 行 127/171/316/412/432） |
| 0x140a1b430 | DecoMoveToStorage::new | decorator_storage.rs:293 | 3 字段 {r8,r9,rdx} ✓ |
| 0x140a1b440 | d2s remove | decorator_storage.rs:80 | 未中 panic ✓（Rust 拆 assert_exists+remove 等价） |
| 0x140a1b4e0 | d2s insert | decorator_storage.rs:87 | 冲突 panic ✓ |
| 0x140a20c70 | apply_decorator_place | decorator_storage.rs:731 | !immutable + get_or_insert_default + insert + d2s ✓（状态字节 2 写入缺失见 §3） |
| 0x140833260..470 | DecoratorInfo dst_as_wall(_mut)/roof/terrain/stair | decorator.rs | +0x22 tag 判别 + panic ✓（panic 文案差异见 §3） |
| 0x1408334d0/4e0/540 | DecoratorDst is_wall/as_roof/as_wall(_mut) | decorator.rs | ✓ |
| 0x1411ff2b0/2f0 | as_curve2/as_curve3 | valid_enclosure | bit0 判别双路 ✓ |
| 0x1411ff600/620/640/660 | get_normal_at_u/get_pos_at_u/perimeter/get_closest_u | valid_enclosure | 判别双路 ✓（get_closest_u 为重构，见 §3） |
| 0x1411ff6f0/740 | as_circle/as_rectangle | valid_enclosure | bit0 判别 + panic ✓ |
| 0x140ade610 | SnappingMode::new | snapping/mod.rs:37 | mode=0 + +0x20=0 ✓（Rust 多初始化 snapped/snap/wall/end，原不初始化，良性） |
| 0x140a6a560 | check_for_disable_snapping | snapping_maintain.rs | mode{2,3}×flag 矩阵全 5 分支逐条对应 ✓ |
| 0x140a6a480 | snapping_audio | snapping_audio.rs | 去重 + kind1/2 + xvy swizzle ✓ |
| 0x140ada600 | snapping_init | snapping_init.rs | 重置→debug(+0xfd)→no-progress→len≤1.65→投影步行→mode=1+snapped ✓（投影表达式次序等价，见 §3） |
| 0x140ac80e0 | calc_shape_height | calc_shape_height.rs | 采样求和/最大 + 基高 6/8 + -0.46 + clamp[0.7,14] + 写+0x40 + 0==0→-2.5 + tag=1/cur/prev/delta ✓（lerp_rate 常量近似，见 §3） |
| 0x140a8a710 | calc_wall_bend | calc_wall_bend.rs | 末 4 方向 + 平均 + 双 clamp + acos ✓ |
| 0x140accab0 | wall_eraser | wall_eraser.rs | height=0.75+13.25t（`fVar*14+(1-fVar)*0.75` 行 1249 ✓）+ min-current + -0.79 平顶补偿（结构 ✓；ChildWall 分裂待接线） |
| 0x14092e6e0 | WallEraserArchivist::new | archivist.rs | 双 map 初始化 ✓ |
| 0x14092ea50 | cache_if_original_wall | archivist.rs | entry-or-insert ✓ |
| 0x140981390 | wall_highlight | wall_highlight.rs | 清 kind + last 未读命令 + entity 解析 ✓ |
| 0x140aac1e0 | delete_wall | delete_wall.rs | 事件→remove_roof/entity/plaster 清链 ✓ |

## 3. 已判等-注意（主结构判等，含登记差异）

| VA | 函数 | 差异点 |
|---|---|---|
| 0x14155afe0 | Decorators::insert | removed_ids.remove+modified+内嵌 map ✓；原另写**状态字节 2@+0x47** 与 d2s insert/update，Rust 由调用方承担（apply_decorator_place 缺状态字节写，见下） |
| 0x140a20c70 | apply_decorator_place | 原 insert 内 `if(state==2) d2s insert + *(rcx+0x47)=2`；Rust 不强制 state=2（调用方传什么存什么）——放置后状态字节可能≠2 |
| 0x140a20670 | apply_decorator_remove | 结构 ✓；原 d2s remove **miss==3 → panic**（行 53/66），Rust `let _ = d2s.remove()` 忽略 |
| 0x1411ff640 | get_closest_u_from_pos | 原调 `Rectangle2d::get_closest_edge_pt_indices` + 4 路 jumptable；Rust 手工 4 边投影重构（语义等价，callee 链不同） |
| 0x140ada600 | snapping_init | 原 t 用 `(point-p0)·(p1-p0)/dist_sq`（=1-t_rust）与插值 `t*p1+(1-t)*p0`；Rust 次序相反（代数等价，FP 舍入不同） |
| 0x140ac80e0 | calc_shape_height | `time_continuous_lerp`（0x140c904e0）rate 常量 Rust 以 `rate*0.1 clamp` 近似（t 恒=1 退化），原函数体未逐句判等 |
| 0x140833260/2c0 | dst_as_wall(_mut) | panic 文案原为 "internal error: entered unreachable code"（0x142a92798）；Rust 用自定义文案（控制流判等，文案差异） |

## 4. 属性疑点（VA/归属）

- `0x140ad2950`：fresh 反编译名 = `GladeBorder::is_pos_inside`（非 ShapeParameters::contains_point）。
  报告/inventory 对该 VA 的归属错误；Rust `Shape::contains_point` 的圆/矩形判别结构与之语义一致，
  但 **PDB 名/归属需对账**。
- `contains_shape` 四路中**圆他=6 点、矩形他=4 角**为机器码直接计数（12×get_pos_at_u + 8×get_point），
  restorer §11.3 的单点修正不成立（见 §1.5）。

## 5. 未判等-需 fresh 复核（已反编译但未完成逐句判等）

以下函数已 fresh 反编译（`/tmp/lw-decomp/` 落盘），本报告未给最终 verdict，按协议**不得标判等**：
- decorator_storage：iter_modified_ids/iter_modified/iter_modified_mut/mark_all_modified/iter_*、
  wall_decorators(_ids/_mut)、lookup_storage/try/decorator_info(_mut)/lookup_decorator_info(_mut)、
  remove_storage、clear_modification_tracking、iter_removed_storages、restore_decorators、
  tracked_clone_from ×2（0x141256e40/0x1412573b0，377-380B 待逐句）。
- decorator：DecoratorType 8 谓词位运算、TransientDecoratorIdGen、DeferredDecoratorOpEvents、
  DecoratorRotationXZ、DecoratorExtraInfo From 变体。
- snapping：snapping_to_ends（2077B 大函数，已确认 1.5 阈值/哨兵/snapped 结构，per-wall 细节待
  逐句）、telegraph_walls_are_merged。
- eraser：archivist finish（1881B）、ui_bulldozer（431B）、animate_wall_bulldoze。
- shapes：shape_placement_params、preview_snapping_line、shape_signifier_lerp、create_circle/
  create_rectangle/edit/finish/ui_* 系列。
- wall_spatial_hash：add_segment（1168B）、replace_wall、segments_*、query_with_circle。
- publish_public_wall_state（11198B 大函数，Rust 为 consume_with_paths 包装，主逻辑在
  recovered_building wall_collection 域外）。

## 6. 待判等清单（未 fresh 复核，本轮不计判等）

已实现但本轮未 fresh 反编译的业务函数（~240 个），按协议标「待判等」，不得以测试/报告通过替代：
- decorator_subtype.rs 20、decorator_archivist.rs 22（多为 re-export 胶水）、public_wall_state.rs 12、
  inner_wall_state.rs 11、inner_walls.rs 26、cached_decorator_transforms.rs 7、
  eraser 其余、shapes 其余、wall_spatial_hash 其余、wall_state/publish 其余、adjust_wall_height、
  history_recording、delete_wall/calc_wall_bend/wall_highlight（已判等见 §2）。

## 7. 总结（回传主会话）

- **判等 ~26 / 判等-注意 7 / 未实现-需重做 11 / 待判等 ~240**。
- 严重分歧集中在**用户硬标准点名区域**：on_wall_split（侧 0 区间搜索/回插缺失）、merge_with 与
  on_walls_merged（d2s 重索引位置/去重语义不符）、restore_snapshot（不恢复 DecoratorInfo）、
  contains_shape（自=圆应 6 点/4 角，Rust 单点）、expand（圆车道应保留原值）、snapping_maintain
  （只发模式字节非全状态）、snapping_to_segments/query_with_aabb_clamped（原排序取最近，Rust 首
  个）、CompoundEditBuilder（started/is_empty/None 为自造）。
- 部分函数 **fresh 反编译推翻 restorer 报告描述**（merge_with 语义、expand 车道、contains_shape
  单点、snapping_maintain 发布区、query 排序）——需 restorer/reviewer 据本报告重做。
