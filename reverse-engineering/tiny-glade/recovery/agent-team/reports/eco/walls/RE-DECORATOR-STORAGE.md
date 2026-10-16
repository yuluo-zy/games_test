# RE-DECORATOR-STORAGE: DecoratorStorage 实体机制证据包（生态组 · walls，卡5）

> 会话：eco re-analyst（2026-10-15）。范围：`resources/walls/decorator_storage.rs`（114 函数）、与 decorator.rs / decorator_subtype.rs / cached_decorator_transforms.rs 的关系、ui_clutter 消费端。
> 依据：章节 `chapters/resources/walls/decorator_storage.rs.md`（布局 O 已在位）+ 本轮新反编译（DecoratorInfo serde 实为 DecoratorId 十进制化）+ ui_focus_decoclutter(0x1409a92c0) 58KB decomp 消费路径 + S_PUB32 身份。
> 记号：O=机器码证实 / I=推断 / U=未恢复。**不重挖章节已 O 的布局，指认位置**。
> 不改 reconstruction 源码。

## 1. 结论一句话

**DecoratorStorage 是纯数据权威资源（存 DecoratorInfo 72B 记录，非 bevy Entity）**：`HashMap<WallId, Decorators>` 分墙桶 + `decorator_to_storage` 索引（DecoratorId→所在存储）+ terrain/stair 内联桶 + immutable 标志。**实体(bevy Entity)不存于 DecoratorStorage**——装饰实体由消费系统（ui_clutter / 渲染 / 音频）读 DecoratorInfo 后另行生成；实体↔装饰的关联靠 DecoratorId（实体携带含 DecoratorId/DecoratorInfo 的组件）。

## 2. 结构/字段布局（O，章节在位，指认 `chapters/resources/walls/decorator_storage.rs.md`）

| 结构 | 布局（章节 §算法级 全 O） | 关键地址 |
|---|---|---|
| DecoratorStorage | per_wall `HashMap<WallId, Decorators>`@+0x00（条目 0x98B={WallId u64@0, Decorators 0x90B@8}）+ `decorator_to_storage` 索引@+0x60（24B 条目 {u32 DecoratorId, pad, u64 判别, u64 payload}）+ terrain 内联@+0x90 + stairs 内联@+0x120 + **immutable u8@+0x1b0** | get_or_insert_default 0x140a1cf30、lookup_decorator_info 0x140a1c930、iter_terrain_decorators 0x140a1e280、iter_stair_decorators 0x140a1e2c0、mark_mutable 0x140a1c2b0 |
| Decorators（0x90B） | removed_ids 集合@+0x00 + **内嵌 map `HashMap<u32 DecoratorId, DecoratorInfo 0x48B>`@+0x30**（条目 0x50B）+ modified_ids 集合@+0x60 | iter_modified_ids 0x140a1b030、apply_decorator_remove 0x140a20670、tracked_clone_from 0x141256e40/0x1412573b0、merge_with 0x14155b170 |
| DecoratorInfo（72B，O 部分） | +0x22 目的地 tag、+0x2c id、+0x44 rank、+0x46/0x48 type、+0x4c/+0x50 u32、+0x55/+0x56 u8（apply_decorator_place 0x140a20c70 读面证实）；**含 DecoratorExtraInfo 联合（判别字节 +0x11：chimney/flag/lantern/stair/window）**；**无 bevy Entity 字段（本轮结论）** | apply_decorator_place 0x140a20c70、apply_decorator_remove 0x140a20670、decorator.rs 章 §判别访问 |

**实体机制（本轮结论，O 依据）**：DecoratorInfo 72B 的已知读面（dst tag / id / rank / type / ExtraInfo 联合 / 若干 u32+u8）**不含 8B Entity**；DecoratorStorage 无任何 Entity 存储（全为 DecoratorId/DecoratorInfo 数据记录）。装饰实体 = 消费系统按 DecoratorId 读 DecoratorInfo 后 `spawn`（含 Transform/含 DecoratorId 的组件）——ui_clutter 的 deco-clutter 即此类实体。

## 3. 关键函数清单与行为级结论（章节表 60 业务，O 为准）

分组（地址 → 行为）：
- **查询族**：`get` 0x140a1c2c0、`get_mut` 0x140a1cde0、`lookup_storage` 0x140a1c3e0、`try_lookup_storage` 0x140a1c510、`decorator_info` 0x140a1c5f0、`decorator_info_mut` 0x140a1c800、`lookup_decorator_info` 0x140a1c930（双层：索引→Decorators→内嵌 map）、`lookup_decorator_info_mut` 0x140a1cbf0。
- **插入族**：`get_or_insert_default` 0x140a1cf30（21KB，immutable 断言；terrain/stair 内联懒初始化，常数键 1/2@0x142ac9118/0x142ac9128）、`Decorators::insert` 0x14155afe0、`impl$8::insert` 0x140a1b4e0（d2s 索引插入，冲突 panic "d2s insert: already exists"）、`apply_decorator_place` 0x140a20c70。
- **删除族**：`impl$8::remove` 0x140a1b440（d2s 索引守卫 panic "d2s remove: no such {id}"）、`impl$10::remove` 0x140a1b700（断言 `self.map.remove(&id).is_some()`）、`apply_decorator_remove` 0x140a20670、`remove_storage` 0x140a1dba0。
- **墙事件族**：`on_walls_merged` 0x140a1ea40（21KB）、`on_wall_split` 0x140a1f590（46KB，装饰迁移主逻辑，0x28B 记录/子侧状态字节）、`on_wall_removed` 0x140a1f550（I）。
- **快照/追踪族**：`restore_snapshot` 0x140a20d70（26KB，16B XOR 掩码 0x1429cd080/090）、`take_snapshot`（内联）、`clear_modification_tracking` 0x141257900、`Decorators::mark_all_modified` 0x140a1b140、`merge_with` 0x14155b170（24KB）、`tracked_clone_from` 0x141256e40/0x1412573b0。
- **迭代族**：`iter` 0x140a1e5b0、`iter_mut` 0x140a1e630、`iter_walls(_mut)` 0x140a1b100/0x140a1e540、`iter_terrain_decorators` 0x140a1e280、`iter_stair_decorators` 0x140a1e2c0、`iter_removed_storages` 0x140a1e6e0、`Decorators::iter_modified(_ids/_mut)` 0x140a1b070/030/0b0、`wall_decorators(_mut/_ids)` 0x140a1e190/420/300。
- **关系（本章新证）**：`archive_current_decorators_in_storage` 0x14123b5c0（84KB，archivist 归档）——decorator_archivist.rs 通过它 + `get_or_insert_default` + `merge_with` 0x14155a950 消费。

## 4. ui_clutter 消费端（本轮新挖，O）

**`ui_focus_decoclutter`（0x1409a92c0，58KB decomp）——deco-clutter 焦点模式移动/旋转，DecoratorStorage 主读路径**：
```
1. 按 DecoratorId 查 decorator_to_storage 索引（hashbrown 24B 条目：{u32 id@-0x18, u64 ptrA@-0x10, u64 ptrB@-8}）→ 得到所在存储
2. FUN_140a1c2c0 = DecoratorStorage::get → Decorators 集合 ptr；null 或 [Decorators+0x48]==0 → return
3. FUN_1405b0ed0(Decorators+0x50, id) 哈希 DecoratorId → 读 Decorators 内嵌 map ptr@+0x30
4. hashbrown 探针 0x50B 条目 → DecoratorInfo
（callee 面证实：SignifierFactory::new 0x14089d8b0、clutter_mesh_ty_data 0x1408b8e90、UiState::add 0x1409231f0、box_trigger_volume 0x140ada0e0、EventWriter::write ×4 0x141327xxx、AppendOnlyVec::push 0x14146c880、DebugRectangle3d::show 0x140b02ed0、DecoratorId Display 0x1408331d0）
```
**其他 ui_clutter**：
- `decoclutter_show_pins`（0x1412357c0，6.7KB）：遍历 pinned deco-clutter 集合，按 ShowPinnedDecoClutter 状态切图钉实体可见性——不直接调 DecoratorStorage helper（走 pinned 集合/Query）。
- `ui_convert_autoclutter_to_decoclutter`（0x14099ca60，8.1KB）：右键 auto-clutter→deco-clutter（InstaCreateDeco/InstaCreateStairPoint/StartPlaceDecoClutterInteraction/EvRightClickedAutoClutter）——直接读 DecoratorStorage 的路径未在本函数出现（经事件/ClutterMeshTy 数据驱动；`FUN_1408b83d0`×4 = clutter_mesh_ty_data 变换，推 0x48B 事件）。
- `handle_rmb_autoclutter`（0x140976ca0，1.4KB）：右键移除 auto-clutter，hashbrown 实体位集 + 朝向计算 → EvRightClickedAutoClutter。

## 5. 地址与调用链 / wall 发布链接口

```
wall_state/publish_public_wall_state (0x140a8b530) — 发 OnWallChanged/OnWallDeleted（16B 行 EventInstance）
  → DecoratorStorage::on_wall_split (0x140a1f590) / on_walls_merged (0x140a1ea40) / on_wall_removed (0x140a1f550)
     → per_wall 探针(FUN_1405b0d70@+0x20 + FUN_140b78400) → 0x90B Decorators memmove → merge_with(0x14155b170)
     → 子墙插入(get_or_insert_default 0x140a1cf30) → d2s 索引更新(impl$8 insert/remove 0x140a1b4e0/440)
ui_clutter → DecoratorStorage::get (0x140a1c2c0) + decorator_to_storage 索引 → Decorators 内嵌 map → DecoratorInfo
decorator_archivist → archive_current_decorators_in_storage (0x14123b5c0) + get_or_insert_default + merge_with
history replay_pop_bucket (0x14155ba30) / replay_stairs → restore_decorators 链
save/snapshot → take_snapshot/restore_snapshot (0x140a20d70)
```

## 6. 未决（U，不臆造）

1. DecoratorInfo 72B 完整字段表（+0x00..0x20 与 +0x24..0x44 逐字段语义；ExtraInfo 联合载荷）。
2. decorator_to_storage 索引 24B 条目判别值（0/1/2=Wall/Terrain/Stair）payload 语义（ptr 或内联 id）。
3. 装饰实体的具体生成系统（渲染 nani / ui_clutter spawn 的确切 VA）——本包确认"实体不存于 DecoratorStorage"，生成器待 render 域定位。
4. on_wall_split 0x28B 装饰记录 u 区间/子侧状态字节完整语义。

## 7. 证据

- 章节：chapters/resources/walls/decorator_storage.rs.md（布局 O 全表）、decorator.rs.md、decorator_archivist.rs.md、cached_decorator_transforms.rs.md、decorator_subtype.rs.md。
- 本轮：ui-input/decomp/decomp-0x1409a92c0.txt（ui_focus_decoclutter 58KB 消费路径）；eco-reanalyst/decomp/decomp-0x1402ddd20.txt（DecoratorId serialize=u32 十进制化，非 DecoratorInfo serde）；wall/decomp/decomp-0x140a20c70.txt（apply_decorator_place 读面）。
- S_PUB32：0x140a1c2c0=DecoratorStorage::get、0x1408331d0=DecoratorId Display、0x1408b8e90=clutter_mesh_ty_data、0x1409231f0=UiState::add。
