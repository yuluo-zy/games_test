# 核心生命周期契约 — systems/wall 根 13 单元

标注 **O/I/U**（见 00-overview）。地址均为 B patched EXE。

## 1. calc_wall_bend.rs（0x140a8a710）

**签名（O）**：
```rust
fn calc_wall_bend(reader: EventReader<OnWallChanged>, public_walls: Res<PublicWalls>,
                  writer: EventWriter<WallBendChangedEvent>)
```
**ECS in/out**：in `OnWallChanged`，in `PublicWalls`；out `WallBendChangedEvent { bend: f32 }`。
**业务（O，反编译 2466B）**：
1. puffin 作用域（`are_scopes_on`）。
2. 逐 `OnWallChanged`：`PublicWalls::get`（hashbrown 查找；缺失 → 无事件）。
3. 校验：public 源点计数 `>= 2` 且源曲线长 `> 0`，否则 panic。
4. 读发布曲线点（map value +0x48/+0x50，O）。若点计数 `> 5`：前 4 段法向 delta
   归一化 → 相邻 dot 累积 → `sum/(n-1)` → clamp[-1,1] → `acosf` → `min(1.0)`；
   否则 bend = 0.0。
5. 每墙写 `WallBendChangedEvent`（16B EventInstance 行，payload f32@+8）。
**GPU 效果**：无（CPU 事件 → 后段消费）。
**失败分支**：缺失墙静默跳过；非法 count/length → panic（原 panic 域）。
**typed gap**：发布曲线点字段名 U；bend 语义（弧度上限 1.0 的原因）U。

## 2. delete_wall.rs（0x140aac1e0）

**签名（O）**：
```rust
fn delete_wall(reader: EventReader<OnWallDeleted>, public_walls: ResMut<PublicWalls>,
               wall_plasters: ResMut<WallPlasters>, wall_entities: ResMut<WallEntities>,
               commands: Commands)
```
**业务（O，反编译 1095B）**：逐 `OnWallDeleted`：
- 从两个 `WallId→Entity` map（`WallEntities` + `PublicWalls.wall_entities_map`）`remove_entry`
  并 `Commands::despawn`。
- 从 `WallId→Vec<Entity>`（`WallPlasters`）移除并逐实体 despawn。
**typed gap**：内墙记录移除/附属对象级联属删除所有权（证据边界外，见 walls-audit）。

## 3. draw_wall.rs（0x140a7cba0，13557B 最大生命周期系统）

**签名（O）**：`ResMut<InnerWalls>, ResMut<InProgressWallResource>, ResMut<CurveNursery>,
tuple<(Res<SnappingMode>, …)>, Res<CursorTerrainRaycast>, Res<RawInputSmoothed>,
ResMut<InProgressCurve>, Local<…>`。
**业务（O，调用链）**：平滑输入空 → 返回；光标地形位 + 吸附（`SnappingMode::is_active/
position_to_snap_to`）；无进行中墙 → `InProgressWall::new`；`InProgressCurve::add_weight_to_the_last_point`；
`Curve::crashy_new_from_points` → `InnerWalls::update_wall_state_curve` + `update_freehand_ty`。
**GPU 效果**：无（墙曲线状态生产 → publish → 砖/渲染下游）。
**typed gap**：CurveNursery 字段、`Local` 载荷、吸附位置细节 U。

## 4. finish_wall.rs（0x140a800a0）

**签名（O）**：`Res<AppMode>, EventReader<FinishWall>, ResMut<InnerWalls>,
ResMut<InProgressWallResource>, ResMut<History>, …`。
**业务（O，调用链）**：`FinishWall` → 取进行中墙；`mark_as_non_enclosed` +
`update_wall_state_enclosure`；CompoundEditBuilder 记录历史（Create(seed)→StateChange）。
**typed gap**：enclosure snapshot 构造细节 U。

## 5. merge_walls.rs（0x140999920）

**签名（O）**：`Res<TerrainHeightsData<TerrainHeightSsbo>>, DecoratorArchivalResources,
ResMut<DecoratorArchivist>, ResMut<InnerWalls>, …`。
**业务（O，调用链）**：检测共线/相邻墙（地形采样），`Curve::crashy_new_from_points` 合并曲线，
`InnerWalls::new_wall`；decorator 归档资源读取。
**typed gap**：合并判据数值/相邻容忍度 U。

## 6. set_wall_style.rs（0x1409a2290）

**签名（O）**：`EventReader<SetWallStyleCmd>, EventReader<SetFreehandHeightTypeCmd>,
EventReader<SetPlasterPeelAmountCmd>, ResMut<InnerWalls>, Res<PublicWalls>,
ResMut<History>, EventWriter<…>`。
**业务（O，反编译 2642B）**：三读器 0x18/0x20/0x18 行：
- style：`update_style`；改变 → `CompoundEditBuilder::next()` + `from_iter` + `History::add()`。
- freehand ty：`update_freehand_ty` + `update_arch_type`。
- plaster peel：`update_plaster_peel`。
- style 后查 `PublicWalls`（校验 count/length），曲线坐标插值 → 写 0x20B
  `RequestWallMaterialAudio`-style 事件 `{tag=0x10, style, pos.xy, pos.z}`。
**typed gap**：行内 flag 位精确语义 U。

## 7. wall_splitter.rs（0x140a74c20）

**签名（O）**：`Res<AppMode>, Res<PublicWalls>, DecoratorArchivalResources, Res<UiState>, …`。
**业务（O，调用链）**：光标拖过墙 → 计算 split u（`get_curve_segment_from_u` /
`get_u_at_coord`）；`ChildWall::new` 两子墙；`InnerWalls::new_wall`；命中处 `WallStyle::convert_to_stone_version`；
decorator 重挂（archival 资源）。
**typed gap**：子墙 range/位姿细节 U。

## 8. wall_highlight.rs（0x140981390）

**签名（O）**：`ResMut<WallHighlight>, ResMut<EditingHighlight>, EventReader<HighlightWallCmd>,
Res<WallEntities>`。
**业务（O，反编译 572B）**：`WallEntities::wall_entity`；命中 → `WallHighlight{entity, wall, ty}`；
`EditingHighlight` rgba 由 ty 选择（0x3d8f5c29/0x3da3d70b/0x3cf5c290/0x3dcccccd）。
**GPU 效果**：高亮色 → 渲染后段。

## 9. upload_instanced_wall_ssbo.rs（0x14082c170）

**签名（O）**：`Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>, Query<&mut InstancedWall>`。
**业务（O，反编译 594B）**：`get_mut<BrickTransformSsbo>`（缺失 unwrap panic）；逐
`InstancedWall` 脏区间两路径（+0x108 compact flag）→ `upload_to_storage_buffer`；
`SparseInstanceBuffer::gpu_flush<BrickTransformSsbo>`。
**GPU 效果**：墙砖 transform SSBO 上载/冲刷（W21/W27 墙 draw-list 前段）。

## 10. deferred_wall_history.rs（0x1408dc9f0 / 0x1408dccf0 / 0x1408dd1b0 / 0x140a876a0）

**签名（O）**：`history_record_wall_changed(ResMut<History>, ResMut<PendingWallHistory>)`；
`history_record_merge_walls(ResMut<History>, ResMut<PendingWallHistory>, Res<InnerWalls>)`。
**业务（O；复用 `reconstruction/building/wall_pending_changed.rs`，B-pending-wall-changed
256 全 RET 验证）**：`try_new` 投影前一墙首/末端点入下一曲线（`approximate_curve_u`，
平方段分母下限 1e-8、严格更小距离替换、并列取首段），clamp[0,1]，仅保留严格 low<high；
StateChange operation range 12B。`history_record_wall_changed` 取 pending 并记一个
ConcurrentEdits 组。`CompoundEditBuilder::wall_deleted`（0x140a876a0）记一个 Delete 组。
**typed gap**：merge 去重/回调细节 U。

## 11. structure_outline.rs（0x140959fc0，23774B）

**签名（O）**：`EventReader<OutlineWallCmd>, EventReader<OutlineStairsCmd>, Res<PublicWalls>,
Query<&Roof>, Res<EscMenuState>, ResMut<UiSignifierStream>, …`。
**业务（O，调用链）**：Esc 关闭时才画；逐墙：`ShapeParameters`（as_rectangle/as_circle/
as_curve2/center/expand/perimeter）+ 曲线采样 → 描边 signifier（custom_shader）。
**typed gap**：描边几何/阶梯 outline 细节 U。

## 12. ui_continue_wall.rs（0x140aadbb0 / 0x140aadd30）

**签名（O）**：`suppress_continue_wall(Res<Modifiers>, ResMut<SuppressContinueWall>,
Res<GameTime>, EventReader<CursorMovedScreenSpace>)`；
`ui_continue_wall(Res<AppMode>, Res<UiState>, tuple<(Res<InnerWalls>, Res<PublicWalls>)>,
ResMut<WallContinueState>, …)`。
**业务（O，调用链）**：光标接近已结束墙端点 → `InProgressWall::new` 续画；
`GladeBorder::is_pos_inside` + decorator cull；`WallContinueState::display_signifier/finish`。
**typed gap**：SuppressContinueWall 内部与续画判定阈值 U。

## 13. ui_draw_wall.rs（0x140aafe40）

**签名（O）**：`Res<AppMode>, ResMut<WallDrawState>, ResMut<CurveNursery>,
EventWriter<RecordMouseInputIntoWall>, EventWriter<FinishWall>, Res<UiState>,
Res<CursorTerrainRaycast>, Res<GladeBorder>`。
**业务（O，调用链）**：glade 内拖动且非清画布 → `RecordMouseInputIntoWall` 转发进
CurveNursery；释放 → `WallDrawState::finish` + `FinishWall`。
**typed gap**：nursery 缓冲细节 U。
