# REVIEW-A1 — decorator 接线（卡A1，reviewer-eco，2026-10-16）

复核对象：restorer-decorator 卡A1（RESTORE-DECORATOR.md §八：wall 发布链接线 + ui_clutter
消费端 + archive_current_decorators）。实测：cargo check 过；cargo test --lib **1667/1**
（唯一败 geometry/sphere 他组）；decorator_wall_events 2/2 过。未改 src。

## 逐项核对（协调者 4 点）

### 1. wall 发布链接线 — ✅ 核对点①（防误删）通过

- `decorator_wall_events`：**OnWallDeleted → `DecoratorStorage::on_wall_removed`**（真实删除
  清理链），immutable 置位期间跳过（原 immutable 门控）✓。
- `register_wall_cpu_resources` 注册 `DecoratorStorage`；`add_wall_cpu_systems` 挂
  `decorator_wall_events`（WallCpuStage::Delete 与 delete_wall 同段）✓。
- **核对点①（分裂/合并防误删）**：当前只消费 `OnWallDeleted`（真实删除），
  `OnWallChanged` 事件读取器被忽略（`_changed` 不触发任何装饰操作）→ 分裂/合并**不会**
  触发 on_wall_split/on_walls_merged，也不会误删（无迁移动作）。on_wall_split/on_walls_merged
  需子墙 id + u + 0x28B 记录（拓扑 U），待 `wall_compound_edit.rs`（79 函数，卡A2）恢复后在
  `decorator_wall_change_pending` 接线点接入——登记 U，诚实。✓

### 2. ui_clutter 消费端 — ✅ 核对点②（主读路径与载荷）通过

- `ui_focus_decoclutter`：`tag==0` 门 → `lookup_storage(id)`（d2s 索引）→
  `decorator_info(id)`（双层 → 内嵌 map）→ **未中任一直接返回**（O：null 或
  [Decorators+0x48]==0 → return）→ `write(DecoClutterFocused)`。
- 载荷 `DecoClutterFocused{decorator_id, addr, info}` = DecoratorId + 所在存储 +
  DecoratorInfo——对 RE §4 O 主读路径（DecoratorId → d2s → get → 内嵌 map → info）✓。
- 实体/相机联动（clutter_mesh_ty_data/UiState::add/SignifierFactory/EventWriter×4）与实体生成
  （render nani 域）登记为宿主接线/待 render 域定位。✓

### 3. archive_current_decorators + archivist 边界 — ✅ 核对点③ 通过

- `DecoratorStorage::archive_current_decorators`：逐 per_wall + terrain + stairs 收集
  `(addr, Vec<(id, info)>)`——O 读路径（0x14123b5c0 逐墙 0x90B + 内嵌 map 拷贝）✓。
- `decorator_archivist.rs` re-export `recovered_building::wall_decorator_archivist::{
  DecoratorArchivist, DecoratorStorage, restore_archived_decorators}`——**与卡5 权威
  `decorator_storage::DecoratorStorage` 并存（路径不同，Rust 命名空间不冲突）**；garden
  draw_garden 消费前者。归档衔接经 archive_current_decorators → archivist backing，backing
  类型差异登记。✓

### 4. cached_decorator_transforms.rs — ✅ 登记

无 src 文件（B 骨架 9.7MB），属后续批次原文件，未在本接线范围，登记。✓

## 结论与 DISPATCH 建议

- **卡A1 = review_passed（限定）**：OnWallDeleted→on_wall_removed 真实删除链 + immutable 门控、
  ui_focus_decoclutter 主读路径 + DecoClutterFocused 载荷、archive_current_decorators O 读路径 +
  archivist 边界、cached_decorator_transforms 登记——全部核过。
- **登记项（不阻塞）**：on_wall_split/on_walls_merged 迁移接线待 wall_compound_edit.rs（卡A2）；
  实体生成/联动待 render 域；archivist backing 类型差异。
- 建议 DISPATCH：卡A1=review_passed（限定）；卡A 全部闭合。卡5/卡F 维持 review_passed。
