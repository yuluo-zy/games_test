# REVIEW-PENDING — walls 待判等/余量复验（reviewer-eco，2026-10-17）

复核对象：impl-walls-pending（REBUILD-PENDING.md，6 批）。方法：与 fresh 反编译/反汇编逐项
判等/重建核对。实测：cargo check 过（rhapsody 并发 WIP 瞬时报错重跑干净）；decorator::tests 6
+ shape_signifier_lerp 2 测试过。

## 逐项核对（协调者 5 点）

### 1. 判等 ~50 — ✅ 抽核

DecoratorType 8 谓词（max_rank/can_change_seed/is_window/spawns_clutter/can_place_on_roof/
can_place_on_stair_top/can_place_on_terrain/should_offer_snap_to_corner 逐字节对 disasm）、
TransientDecoratorIdGen（直写/偏移 generate）、inner_walls 余量（new/get/get_unwrapped/
update_arch_height/_try_update_state×8 探针逐条）、public_wall_state 8（wall_space/u_space/
flat_roof_y/get_wall_shape/is_rectangle/is_circle/corner_pos/get_wall_attachment_height）、
shapes edit_rectangle、decorator_storage lookup/remove/clear + wall_spatial_hash new
（4096 cells/extent 130）——均按 fresh 判等。判等-注意（new_wall_from_curve 并入 new_wall/
max_y 预计算标志/max_y_gable/attachment panic→Err）登记 ✓。

### 2. ⚠️ 重建 7 — ✅ 全对上（含 DecoratorExtraInfo 判别大纠偏）

- `from_xz_forward_vector` = `atan2(-x,-y)`（符号掩码 0x80000000×4）✓。
- `from_quat` = `atan2(2(wy+zx), w²+z²-x²-y²)`（通用公式，非纯 Y 近似）✓。
- `to_quat` = (x=0, **y=sin(a/2)**, z=0, **w=cos(a/2)**)（y/w 对调，roundtrip 测试过）✓。
- `to_projective_xz_mat2` = 列 {cos,sin}/{-sin,cos}（列转置修正）✓。
- `offset` = `fmod(a+amount, 2π)`（回绕）✓。
- **DecoratorExtraInfo 判别/载荷错位纠偏**（fresh disasm as_* panic 串逐字节证实）：
  **disc 4=烟囱{Chimney {f32,u8}}、5=旗{u8}、6=窗{Window{u32}}、7=灯{Lantern{f32}}**——
  Rust 原 Window=4/Lantern=6/Chimney=7 全错位已重建；From 地址 0x140839520=Chimney/
  530=Flag/540=Window/550=Lantern/560=Stair ✓（注释登记，测试新判别值 4/6/7 roundtrip）。
- `lerp_shape`：v1=(1-expf(-20t))·Δ（指数速率 20 非 cubic rate 10）、v2=sign(Δ)·min(2t,|Δ|)
  （保留符号）、增量=|v1|/|v2| 较大者——重建 ✓（2 测试过）。

### 3. 未实现-需重做 6 登记 — ✅ 诚实

create_rectangle（ridge_length_01 公式 5.25 分母/easing/(3+3eased)/12 + elevation/style
独立 _try_update_state 序 + building/roof 颜色查表）、get_building_and_roof_colors、eraser
finish 11509B（5 桶 WallHistoryEdit→ConcurrentEdits→History::add 链）、restore_decorators
占位 passthrough、unleash（tag==5 哨兵）、DecoratorOpEventKind::id（跳表字段读）——登记不冒顶 ✓。

### 4. 跨域登记 — ✅

archivist 迭代链 5（extract_diff/extract_before_after/archive×3）需 history 域
DecoratorStorageHistoryDiff/DecoratorStorageAddrHistoryState + collision 域 RaycastMesh/
OffsetColliderTransform 类型（不"因大"登记，按类型边界）；telegraph/snap_to_roof_ridge/
publish_public_wall_state 宿主边界（render-host/roof/recovered_building 域）——登记 ✓。
建议 coordinator 派 history/collision 域补类型后回派 walls 实现链。

### 5. 剩余 ~190 待判等 — ✅ 诚实

大 UI 系统函数（ui_rotate_circle/create_circle/ui_create_circle/ui_edit_circle/
show_full_circle_signifier/create_rectangle 前半等）待分批逐句——登记 ✓。

## 结论与 DISPATCH 建议

- **walls-pending（实现层面）= 部分闭合**：判等 ~50 复核、重建 7（含 DecoratorExtraInfo 判别
  大纠偏 + RotationXZ 5 件 + lerp_shape）逐分支对上；未实现 6 + 跨域 5 + ~190 待判等诚实登记。
- 登记项（不阻塞，后续/跨域）：create_rectangle ridge 公式 + eraser finish（需 history/
  collision 域类型）、restore_decorators/unleash/DecoratorOpEventKind::id、archivist 链 5、
  ~190 待判等。
- 建议：REBUILD-PENDING=review_passed（限定）；history/collision 域补 diff/collider 类型后回派
  walls 实现 create_rectangle ridge + eraser finish + archivist 链。
