# REVIEW-DEEP — walls 域深判等收口复验（reviewer-eco，2026-10-17）

复核对象：impl-walls-deep（planning/impl-walls-deep/REBUILD-DEEP.md，14 批）。方法：与 fresh
反编译/反汇编逐项判等。实测：cargo check 过（rhapsody 并发 WIP 瞬时报错重跑干净）；decorator_
storage 12 + snapping 8 + decorator_subtype 4 测试过。

## 逐项核对（协调者 4 点）

### 1. 5 实现 — ✅ 逐分支对上

- **spatial_hash 4 变体**：`query_with_segment`（get_cells_from_segment 覆盖格收集→到段最近点
  距离排序→相邻同墙去重 keep 最近）、`query_with_rectangle2d`（四角 AABB 收集排序去重）、
  `cell_center`（dims==0 panic + 中心换算）、`bounds_ws`（{−extent/2,+extent/2}）✓。
- **apply_decorator_change**（0x140a20890）：immutable 门→apply_decorator_remove→
  get_or_insert_default→insert（重插+d2s update）→get_mut（缺失 panic）→modified_ids.insert→
  **内嵌 map 逐字段更新**（u30 低 32 位==1→rank 判别 + u30 低=info.u30 高；u38&1→type 判别 +
  u30 高=info.u3c；判别非法 panic）✓；目标特殊分支（op blob+0x10==3）在 Rust 判别枚举无对应
  变体——登记判等-注意。
- **snapping_to_ends**（0x140a6ad80）：**去自造 `if snapped break`**（遍历全部墙，仅
  snapped==0 跳过当前墙 lVar23==lVar20）；当前墙 end 标志=flags bit0；3D 投影改
  get_approx_u_from_pos→coord_at_u(segment,t)→outline 3D 点按 t 插值（非 nearest_segment）✓。
- **can_be_on_corner**：`mov ecx,0x91008; bt ecx,eax; setc al` 位掩码成员测试
  （bit3/12/16/19 = 0x91008 = 2³|2¹²|2¹⁶|2¹⁹），restorer `==3` 近似已修正 ✓。
- **tracked_clone_from×2**：REBUILD-REMAINDER 已落地，fresh 复核一致 ✓。

### 2. 判等 ~45 — ✅ 抽核

inner_wall_state 4（is_fence/is_default_arch_height/get_enclosure_params + new_wall_w_params
归属修正）、public_wall_state 4（wall_space_length/wall_space/u_space/flat_roof_y）、
inner_walls 9（reset_max_id/update 族/_try_update_state×4/new_wall_with_id/remove/remove_all/
load_state）、cached_decorator_transforms 8（insert/swap/transform/position/try_*/get/
落缓存）、decorator_subtype 表族 13（LibKey/Collision/Insert/outline/get/subtype/bbx_ls）、
eraser archivist+ui_bulldozer+animate 6、archivist extract_and_restore_to_storage
（经 restore_archived_decorators 逐源对应）、shape_placement_params——均按 fresh 逐条判等。
判等-注意登记：DecoratorLibKey 参交换、DerivedDecoratorInfo::subtype 尾跳、try_get_subtype
Option 近似、archivist record_rewiring_event（RewireEnclosedWall 域边界）✓。

### 3. 属性疑点 2 — ✅ 登记

0x14085a370 = **InnerWalls::new_wall_w_params**（+0xc0 id 递增 + map/集合 insert，非
WallState::new）；0x140833250 = **DecoratorRank::new**（mov eax,ecx 字节直通，非 impl$7
into_decorator_subtype）——两处 Rust 注释 VA 归属有误已登记修正 ✓。

### 4. 剩余登记诚实 — ✅

~240 待判等（未 fresh 复核）、archivist 迭代链 5（extract_diff/extract_before_after/
archive_storage/archive_current×2）、eraser finish 11509B、telegraph/snap_to_roof_ridge
宿主边界、publish_public_wall_state（11198B 主逻辑在 recovered_building 域）——留待后续
分批，诚实不冒充 ✓。

## 结论与 DISPATCH 建议

- **walls 深收口（实现层面）= 部分闭合**：5 实现（spatial_hash 4 变体 + apply_decorator_change +
  snapping_to_ends + can_be_on_corner 位掩码）逐分支判等；~45 判等复核；2 属性疑点登记修正。
- **登记项（不阻塞，后续分批）**：archivist 迭代链 5、eraser finish 11509B、telegraph/
  snap_to_roof_ridge 宿主边界、~240 待判等、publish_public_wall_state 主逻辑域。
- 建议：REBUILD-DEEP(WALLS)=review_passed（限定）；archivist 迭代链 + eraser finish + ~240
  待判等交 coordinator 续派。
