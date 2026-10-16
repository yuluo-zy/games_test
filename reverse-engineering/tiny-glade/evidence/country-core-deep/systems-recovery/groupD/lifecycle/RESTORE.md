# 组 D · 生命周期/历史/备份族恢复报告（restorer D3，tg_d3）

日期：2026-10-16。范围：`tiny-glade/crates/systems/decorator/` 的
**apply_decorator_op / deferred_decorator_history / replay_decorator / backup /
restore_archived_decorators / load_decorators / merge_proposal_wall_decorators /
merge_proposal_roof_decorators / handle_external_decorator_cmds /
derive_decorator_subtype / decorator_resources / decorator_visual_state /
cache_decorator_transforms / decorator_on_freehand_wall_edited /
decorator_on_move_shape / decorator_on_rectangle_edited /
decorator_on_wall_height_changed**（17 文件）。

## 1. 证据

- fresh RE-MCP 反编译 28+8 函数落盘 `evidence/country-core-deep/systems-recovery/groupD/decomp/`
  （`job-lifecycle.json` / `job-aux.json` / `job-aux2.json` + `out-*.json`；dbid tg_d3）。
- 字符串 rne-read 实证（panic 串 / 归档标签 / file 路径）。
- 交叉核对：`history/DIGEST.md §3.2`（add_decorator_diff 0x140a854a0 4 参）、
  `chapters/resources/walls/decorator_archivist.rs.md`（all_storages/all_decorators/
  extract_diff/archive_storage_decorators）、`chapters/resources/walls/decorator_storage.rs.md`、
  `chapters/resources/walls/wall_attachment.rs.md`（WallCornerAttachment 0x140b256b0/6c0/6f0 手解）。

## 2. 实现清单（业务函数 → VA → 状态）

| 文件 | 函数 | VA | 状态 |
|---|---|---|---|
| apply_decorator_op.rs | apply_decorator_op | 0x141276e30 | O 结构（8 参面；归档别名集 U） |
| apply_decorator_op.rs | write_place/change/remove_op_history | 行 156/166/182 | O（内联） |
| deferred_decorator_history.rs | decorator_record_history | 0x140817850 | O（32B 记录 + tag 10 + HashSet 清空） |
| deferred_decorator_history.rs | DeferredDecoratorEditHistory::add / add_potentially_surviving_affected_decorator | 行 34/48 | O |
| backup.rs | DecoratorBackup::clear | 0x140823660 | O |
| backup.rs | create_or_restore_decorator_backup | 0x1408238a0 | O 状态机（CREATE/RESTORE/重置） |
| replay_decorator.rs | replay_decorator | 0x14080c8d0 | O 结构（duration/elapsed 推进 + cull-allow + 命令移除） |
| restore_archived_decorators.rs | restore_archived_decorators | 0x140817a40 | O 结构（mark_mutable/清追踪/逐装饰落位/stairs 恢复） |
| cache_decorator_transforms.rs | cache_decorator_transforms | 0x1408153d0 | O 调用面（墙/屋顶/地形；公式尾段 U） |
| decorator_on_wall_height_changed.rs | decorator_on_wall_height_changed | 0x14081b5e0 | O |
| decorator_on_wall_height_changed.rs | closure$0（adjust_wall_height_decorators） | 0x14081b920 | O（side 公式 + dst_payload 直写） |
| decorator_on_freehand_wall_edited.rs | decorator_on_freehand_wall_edited | 0x14081bac0 | O 结构（重定位公式尾段 U） |
| decorator_on_freehand_wall_edited.rs | decorator_on_circle_edited | 0x14081c060 | O 结构（side 分派 + 位置写偏移 O，值公式 U） |
| decorator_on_move_shape.rs | decorator_on_move_shape | 0x140811fc0 | O 结构（位置更新尾段 U） |
| decorator_on_rectangle_edited.rs | decorator_on_rectangle_edited | 0x140819c00 | O 结构（cull_allow_wall 等价；on_rectangle_edited U） |
| decorator_visual_state.rs | entities / add_decorator_visual_entity×2 / seal / remove / drain | 0x1408119d0/0x14067dd80/0x1408119e0/0x140811c30/0x140811e80 | O（seal panic "already sealed"） |
| decorator_resources.rs | to_widget_name_hash | 行 36 | I（xxh3 族候选；U 定点） |
| derive_decorator_subtype.rs | derive_decorator_subtypes / derive_decorator_subtype / is_half_dormer | 0x140812b20/0x140813d00/0x140814470 | O 分派 + 常量；跳表 case U |
| handle_external_decorator_cmds.rs | handle_external_decorator_cmds | 0x14081a380 | O（两命令流 + apply + History tag 10） |
| load_decorators.rs | load_decorators / load_collision_as_bbx | 0x1407b66e0/0x1407bb4b0 | O 结构（JSON 载荷/碰撞后端 U） |
| merge_proposal_wall_decorators.rs | propose_wall_decorator_merge_position_inner / find_best_wall_decorator_merge_position | 0x142001d40/0x142001880 | O 读面；公式尾段 U |
| merge_proposal_roof_decorators.rs | find_best_roof_decorator_merge_position / intersect_roof_decorators | 0x142000e60/0x141fe7280 | O 读面；公式尾段 U |

## 3. 机器码关键契约（O 证据摘要）

- **apply_decorator_op 8 参面**（handle_external 0x14081a380 调用点 O）：
  `(op, kind_flag u8, dst, storage, archivist Option, history Option, sink,
  events)`；assert "assertion failed: decorator_archivist.is_some()"（0x142c23510，
  line 0x2f）当 archivist 缺失且 kind≠0。op 判别 `(kind-3)<2` → 3=place/4=remove，
  其余 change。历史 record kind 5/4/3（remove/place/change）。
- **归档标签 = 字符串键**（rne-read O）："place preview"(13=0xd)、"remove
  preview"(14=0xe)、"move preview src/dst"(16=0x10)、"rectangle edit"(14=0xe)。
  **D2 曾把 0xd/0xe/0x10 误读为 History tag**——实为归档标签串长（本报告 §6
  overlay 冲突）。
- **decorator_record_history**：flag@+0x48 → 排空 0x20B 记录 → History::add
  （FUN_140a84ec0，**tag 10 = DecoratorV5**，载荷 Arc<32B>）→ HashSet 清空；
  kind==6 排空 break。
- **0x88B 事件**（DeferredDecoratorOpEvents::send 0x1408336b0）：首字段
  place=3/remove=4，尾部 dst 16B + kind 字节（O 放置；精确解码以消费端复核）。
- **replay_decorator**：ReplayBuffer 前端桶 → cull-allow（archivist+0x30 集合）
  → 0x30B ReplayCmd `{tag@0, flags@+8, duration@+0xc, id@+0x10, elapsed@+0x18,
  undo@+0x1c}`（tag 7=装饰；flags bit0==0 → 立即完成；required=max(duration/
  speed,0.1)；elapsed+=dt）→ 完成索引反转移除。
- **restore_archived_decorators**：mark_mutable → 逐 archivist 地址 get_mut +
  clear_modification_tracking → 逐装饰 get_or_insert_default + DecoMoveToStorage
  + 内嵌 map insert（状态字节==2 → d2s insert 否则 update）→ stairs
  restore_backup；colliders/interactions（FUN_141298110 身份 U）+ extract_diff
  冲刷（0x140b2cb30）登记 U。
- **备份状态机**：backup+0x282 ∈ {0,1=CREATE,2=RESTORE}；CREATE 拷存储桶 →
  archivist 桶（tracked_clone_from 等价）+ 延迟记录 + 辅助 0x30B×3 + 事件 Vec；
  RESTORE 反向；+0x280 重置。
- **decorator_visual_state**：seal_persistent_entities 已封存 panic
  "The persistent visuals are already sealed"（0x142a8ecd0）；HashMap<u32,
  0x28B 条目> 布局。

## 4. 显式 U / 未实现登记（不造假）

1. **History::add_decorator_diff**（core，0x140a854a0 4 参 `(&mut History,
   &DecoratorDiff 64B, &StairsDiffTable 96B, &RawTable remap)`）未在 core 实现；
   历史链以 `EditType::DecoratorV5(Arc<DecoratorHistoryEdit>)` + `History::add`
   模型接入（deferred_terrain_history 同模式）。32B 运行时记录 → 56B 存档
   模型映射仅取机器码证实位置（id ← +0x08），before/after 语义 U。
2. **core DecoratorArchivist** 缺 all_storages/all_decorators/extract_diff/
   archive_storage_decorators（章节 O）；systems 侧以模型字段等价迭代
   （archivist_all_storages/all_decorators + archive_decorator_state 墙路径），
   预览标签桶（special/string-keyed）U。
3. **DecoratorStorage 双 WallId**：resources::walls::WallId vs
   recovered_building::wall_state::WallId（`building_wall_state` 别名）——桥接
   `addr_from_address`/`WallId(w.0)`。
4. **decorator_on_\* 事件类型**为系统侧模型（EventReader 0x10B/0x28B/0x30B/
   0x58B 载荷）；原生事件族归属 core wall 系统（producer 接线 U）。
5. **decorator_on_\* 位置公式尾段**（freehand 曲线重定位 / move_shape Vec2 add /
   circle FUN_140c92090 / rectangle on_rectangle_edited / wall_height
   dst_payload 直写）U——机器码写偏移 O，数值公式待装饰消费端取证。
6. **FUN_1412b4a40 / FUN_1412b4f10 别名集**（apply/rectangle 的归档调用）
   身份 U（<&str>/<String> 实例 = 0x1412b2a70/0x1412b3910，core 章节 O）。
7. **FUN_141298110**（restore 的 colliders/interactions 落位）身份 U。
8. **load_decorators** JSON 载荷格式 + 目录路径 + RaycastMeshData 碰撞构造
   （core `native-physics` feature 关闭时 new_convex/new_cuboid 不可用）U。
9. **derive_decorator_subtype** 跳表 case（ty 全值域分派）与 is_half_dormer
   公式尾段 U；常量 case（0x10/0x18/0x19/0xd/0x16/6/0/7/1）O。
10. **merge_proposal_\*** 得分键（min_by）与吸附公式尾段 U。
11. **cache_decorator_transforms** 墙/屋顶位置旋转精确公式 U（61KB decomp
    已备，调用面 O）。
12. **replay_decorator** core `ReplayCommand` 无 Decorator 变体（展开
    DecoratorV5 → UnrestoredReplayOwner）——系统按 0x30B 命令模型推进，
    变体接线交集成 owner。

## 5. 与既有合同/代码差异

- core `DecoratorHistoryEdit`（56B 存档模型）≠ 原生运行时 HistoryEdit 载荷
  （32B）；映射已登记（§4.1）。
- core `Decorators::insert` 语义含 removed 撤销 + modified 标记；备份/恢复
  复用一致。
- `DecoratorStorage::get_mut` immutable 门控：restore 先 mark_mutable（机器码
  O：0x140a1c2b0）。
- `SNAPSHOT_ID_MASK`（_UNK_1429cd080/_UNK_1429cd090）在 restore 哈希面出现
  （机器码 XOR 掩码），core 常量值为 0（U：数值待取证）。

## 6. overlay 冲突（D2，需集成 owner 裁决）

- **apply_decorator_op.rs**：D2 曾以 4 参签名 `(storage, id, request,
  history)` + `EditType::Decorator(Default::default())` 空载荷 + "History tag
  0xd/0xe/0x10" 覆盖。机器码实证（本批次 fresh decomp + rne-read）：
  ① 8 参面（handle_external 调用点）；② 0xd/0xe/0x10 = 归档标签串长（非 tag）；
  ③ 历史链 tag = 10（decorator_record_history 的 FUN_140a84ec0 调用）；④
  `EditType::Decorator(Default)` 为假数据（历史/备份链禁止）。**本文件按机器码
  恢复，D2 ui_systems 消费（ui_change_chimney_height/ui_move_decorator 等）
  需按其调用点适配 8 参面**。
- **deferred_decorator_history.rs**：D2 曾把 UI 延迟操作队列（0x28B 记录
  {seq, kind 0x12/0x21, payload}）命名为 `DeferredDecoratorEditHistory`（与
  历史链异构）。机器码证明历史链权威布局 = `{Vec<0x20B 记录>@+0x00, HashSet
  @+0x18, flag@+0x48}`（0x140817850 排空面 + 0x141276e30 push 面）。**本文件
  保留两者**：`DeferredDecoratorEditHistory`（历史链）+ `DeferredDecoratorOp`/
  `DeferredDecoratorOpQueue`（UI 操作队列，D2 证据）。`ui_systems/mod.rs` 的
  `init_resource::<DeferredDecoratorEditHistory>()` 仍编译（Resource 派生保持）。

## 7. 编译状态（2026-10-16）

- `cargo check -p system-decorator`：**D3 17 文件 0 hard error / 0 warning**。
- 剩余 49 hard error 全部在 D1/D2/D4 在途文件（decorator_visual/*、audio/*、
  cull_oob、remove_ivy、dormer_holes、move_decorators_following_anchors、
  utils/visual_entity_builder、lib.rs 跨文件）与 country-core 在途
  （systems/time_of_day、collision/world）——按中央协调登记，非 D3 缺码。

## 8. 跨领域线索

- History 链（tg_a1）：add_decorator_diff / History::add 核心 0x140a84ec0。
- DecoratorStorage/archivist（tg_a2）：apply_decorator_place/remove/change、
  get_or_insert_default、tracked_clone_from、all_storages/all_decorators/
  extract_diff、WallCornerAttachment。
- visual/ui（D1/D2）：DeferredDecoratorOpEvents 0x88B 事件消费、0x28B 操作
  队列、DecoratorVisualState（seal/transient）。
- StairsState::restore_backup（tg_a7 楼梯域）：backup/restore 链。
