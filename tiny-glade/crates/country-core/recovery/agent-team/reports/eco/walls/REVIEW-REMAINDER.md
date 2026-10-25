# REVIEW-REMAINDER — walls 余量分批处理复验（reviewer-eco，2026-10-17）

复核对象：impl-walls-remainder（REBUILD-REMAINDER.md，5 批 78 函数 fresh 证据）。
方法：与 fresh 反编译逐函数判等/分类。实测：cargo check 过（rhapsody 并发瞬时崩溃重跑干净）；
walls 单测 26 过（decorator_storage 13 + valid_enclosure 等）。

## 逐项核对（协调者 4 点）

### 1. 3 修复 — ✅ 逐分支对上

1. `apply_decorator_place`（0x140a20c70 / insert 0x14155afe0）：`state==2 → d2s.insert（冲突
   panic）+ placed.state=2；else → d2s.update`——源码逐分支与机器码对 ✓（REVIEW §2 登记项闭合）。
2. `apply_decorator_remove`（0x140a20670）：`d2s.remove().expect("assertion failed: …")`
   （miss==3 panic，非忽略）✓（REVIEW §2 登记项闭合）。
3. `query_with_circle`（0x140aa5940）：收集→**sort_unstable_by（smallsort/ipnsort 圆比较器）**
   →相邻同墙去重 keep 最近→半径筛后回调（dedup-then-filter 序保留）✓。

### 2. 判等 ~55 / 注意 7 — ✅ 抽核

- 判等族（fresh 复核一致）：decorator_storage 访问器/迭代族（iter_modified/mark_all_modified/
  clear_modification_tracking/insert/d2s 三件/as_wall/decorator_info 双层/get/remove_storage/
  on_wall_removed/clear_modification_tracking）、restore_snapshot、CompoundEditBuilder、
  valid_enclosure 几何族（as_curve2/3/get_normal/get_pos/perimeter/get_closest_u/center）、
  spatial_hash（add_segment/remove_wall_id/query/cell 换算族/cell_center）、archivist cull
  谓词（cull_allow_any/is_cull_allowed）——均对。
- 判等-注意 7（登记）：lookup_storage（native panic_fmt vs Rust Option，调用方按 Option 用）、
  d2s update（native miss panic vs Rust Option，调用方先 remove 后 update miss 不可达）、
  tracked_clone_from×2（简化，缺 d2s remove 旧条目）、apply_decorator_change（简化）、
  WallHistoryCompoundEdit::fmt/WallHistoryEdit::fmt（derive glue）、snapping_to_ends
  （投影 get_coord_at_u vs nearest_segment）、telegraph（单墙近似）——诚实。

### 3. 分类 — ✅ 诚实

宏胶水 ~12+（serde visitor/derive/注册/0x14003bb80 空函数族）不重建；未实现-需重做 10 登记：
apply_decorator_change、tracked_clone_from×2、snapping_to_ends、telegraph_walls_are_merged、
archivist 5 大函数（extract_diff/extract_and_restore/archive_storage/archive_current/
extract_before_after）。✓

### 4. 未覆盖留后续 — ✅ 登记

decorator_subtype 20、inner_walls 26、public_wall_state 12、inner_wall_state 11、
cached_decorator_transforms 7（已建但未在本批）、shapes 其余、eraser archivist finish/
ui_bulldozer/animate_wall_bulldoze、publish_public_wall_state（11198B）——留待后续分批。✓

## 结论与 DISPATCH 建议

- **walls 余量（实现层面）= 闭合**：3 修复逐分支判等、~55 判等/7 注意复核、分类诚实、
  未覆盖登记。
- **登记项（不阻塞，需重建/后续）**：apply_decorator_change、tracked_clone_from×2、
  snapping_to_ends、telegraph、archivist 5 大函数 = 未实现-需重做（下批）；decorator_subtype/
  inner_walls/public_wall_state 等 = 后续分批。
- 建议：REBUILD-REMAINDER=review_passed（限定）；未实现-需重做 10 项与未覆盖文件交 coordinator
  续派。
