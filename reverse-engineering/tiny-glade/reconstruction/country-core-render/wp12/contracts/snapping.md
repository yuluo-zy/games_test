# 吸附（snapping）契约 — snapping/（7 单元）

## 1. mod.rs — SnappingMode（0x140ade610）

`SnappingMode::new`（11B）：`{ mode u8 @0, snapped bool @1 }`；`is_active` = mode!=0；
`position_to_snap_to`。mode 1 = to segments，2 = to ends（O，`snapping_audio` 分支）。

## 2. snapping_init.rs（0x140ada600）

**签名（O）**：`ResMut<SnappingMode>, Res<InnerWalls>, Res<InProgressWallResource>,
Res<DebugGui>`。
**业务（O，反编译 680B）**：重置 mode=0/snapped=false；DebugGui 禁用或进行中墙空 → 返回；
进行中墙首段长 `<= 1.65` → 返回；后续点投影首段，`<= 1.5` 距离内 → mode=1 + snapped。

## 3. snapping_maintain.rs（0x140a6ad00 / 0x140a6a650 / 0x140a6a560）

- `check_for_disable_snapping`（0x140a6a560）：吸附中点下 → mode=0。
- `snapping_maintain`（0x140a6ad00，117B）：mod key 切换 + `NotifyHintSystemSnapping`。
- `snapping_signifiers`（0x140a6a650）：`Query<(&SnappingToSegmentPreview, &mut Transform),
Without<SnappingToEndPreview>>` + end 查询 → 段/端预览线（DebugLine/DebugSphere）。

## 4. snapping_to_ends.rs（0x140a6ad80）

**签名（O）**：`ResMut<SnappingMode>, Res<InProgressWallResource>, Res<CursorTerrainRaycast>,
Res<PublicWalls>, Res<InProgressCurve>`。
**业务（O，调用链）**：非 enclosure 公共墙；`InProgressCurve::first_raw_input_point` 投影
墙曲线（`get_pos_at_coord/get_pos_at_u/start_pos`）→ `find_closest_end_to_snap_to` 最近端
（u=0/1）→ mode=2。

## 5. snapping_to_segments.rs（0x140a6b5e0）

**签名（O）**：同上 + `Res<InProgressCurve>`。
**业务（O，调用链）**：`raw_input_curve_length_ws`；`closest_segment_to_snap_to`
（`get_curve_segment_from_u/get_tangent_at_u` + `distance_segment_point`）最近段 → mode=1。

## 6. snapping_audio.rs（0x140a6a480）

**签名（O）**：`Res<SnappingMode>, EventWriter<PlayWallUiSound>`。
**业务（O，反编译 223B）**：mode==2 且声音 flag 变化 → 写 0x20B `PlayWallUiSound`；
mode==3 且 snap-count<=1 → 写。

## 7. telegraph_walls_are_merged.rs（0x14086ad70）

**签名（O）**：`EventReader<PlayLineAcrossWall>, Res<PublicWalls>,
Query<(Entity, &mut LineAcrossWallSignifier, &Handle<Mesh>)>, ResMut<UiSignifierStream>, …`。
**业务（O，调用链）**：`PlayLineAcrossWall{a,b}` → `Curve::crashy_new_from_points` +
`crashy_slice` 连线两墙端点 → `LineAcrossWallSignifier`（`animation_duration/custom_shader`）。
**GPU 效果**：合并预告线 signifier。
