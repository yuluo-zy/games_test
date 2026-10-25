# RESTORE — system-water（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/water/`（workspace 成员已挂）。
> 证据：`groupA/water/RE-ANALYSIS.md`（re-analyst-A）、`groupA/water/decomp/`（fresh）、
> `BASELINE-signature-surface.md`、章节 `chapters/resources/water*.md`。

## 实现清单

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `draw_water` | 0x141ba9130 | 已实现（语义级） | 16 参按 BASELINE；门禁 0x14362c098；AppMode::Eraser(All\|Water)/Water 双分支；橡皮半径 inverse_lerp(0.3,2.0)→t*4+(1-t)（0x140c904a0/0x1429258d8/0x14292e6f0/0x142934694/0x142925904 字节直读）；画水半径 = BrushSizes.water（+4）；界内圈 0.8×r（0x142a16cb8=0.8，re-analyst 记 0.15 与字节不符——**以字节为准**）；DrawState::update 状态机（0x140ae4bd0 机器码逐条核对）；`draw_raster<WaterRasterStagingSsbo>` 槽位 4；事件 WaterDrawEvent/WaterEraseEvent {position, radius}（audio 域消费面）；橡皮 signifier（'eraser_water_' fat 0x142eebc40，缩放 0.5×r=0x142925870）；图钉 {circle, pos.y+1.5}（0x1429258d0=1.5，0x142260a60 写入——re-analyst 记为 WaterDrawEvent 写入，**以 BASELINE 参数面 ShowPinnedDecoClutter 语义为准**，分歧登记）；画布名 'water'/'eraser'（re-analyst 契约；串区另含 'watereraser_water_' 0x142eebc28 打包区） |
| `update_water_audio_grid` | 0x141baefe0 | 已实现（语义级） | 5 参；64 帧滚动活动位集（位 = 1<<(frame&63)，掩码 0x0e→8×8 格，格坐标 2*(i&7)-7）；距离² ≥ 6400（0x142eed900）清位；水位采样 extent 130/窗 260（0x142a7b658=130.0 字节直读——re-analyst 记为栅格引用，**以字节为准**）≤0.5 清位；遮挡射线 mask 0x32a3；core 语义模型 WaterAudioGrid{water_level, position}（滚动窗口移入 Local，I） |
| `record_water_history` | 内联（行 12） | 已实现 | `RecordWaterEditToHistoryCmd` → `record_raster_history`（槽位 4，`EditType::Water`）；PDB inlinee，按 garden 域惯例独立系统 |
| `impl$0::build` | 0x1406501d0 | 已实现（O 面） | register_event×2 + insert_resource<WaterRaster>(DirtyTilePublisher::unique) + add_systems；App 机制按 core 约定以 (world, schedule) 落地 |

## core 侧消费面接线（delta，已入 `tiny-glade/crates/country-core`）

- `resources/water.rs`（新）：WaterDrawEvent/WaterEraseEvent {position, radius} + RippleSourceSsbo（typed gap）。
- `systems/audio/water_audio.rs`：本地事件副本改为 re-export `resources::water`（原"待接线"闭合）。
- `resources/mouse_terrain_raycast.rs`：新增 `CameraCursorRay`（Resource，origin@+0/dir@+0xc，`intersect_with_xz_plane` 0x1408ef180）。
- `systems/collision/world.rs`：`RaycastWorld`/`Raycasting` 加 `#[derive(Resource)]`（BASELINE 参数面）。

## 未实现项 / 待 reviewer

1. RasterDrawResources SystemParam（core `init_state` 0x140958040 已登记类型面）——本实现按 draw_garden 已接受惯例用派生 `WaterRasterResources` 束 + 借用组装；reviewer 裁决。
2. 画布名 'watereraser_water_'（0x142eebc28）vs 'water'（re-analyst）的精确串语义。
3. 0x142260a60 的写入事件归属（WaterDrawEvent vs ShowPinnedDecoClutter 圈/高变体）——本实现按 BASELINE 参数面 + garden 惯例（`WaterPinnedClutterCircle`）。
4. 音频格 24B 条目的滚动环复用细节（相对坐标/环语义，I）。
5. 门禁全局 0x14362c098 的建模（water_systems_enabled 常量；render 域记 demo、re-analyst 记输入抢占）。
6. draw_raster 的 strength（橡皮分支记录常数 1，I）。

## 编译

`cargo check -p system-water`（离线）通过；workspace 全量 check 通过（核心 12 个 roofs/geometry 错误为 D 组在途重构，与本交付无关——见工作树备注）。

## 修复轮 2（reviewer-A 复验 + 中央裁决，2026-10-16）

- **core UiAppendSignifier 40B 布局（中央裁决）**：`{tag u64@0, kind u64@+8, Vec3@+0x10, f32 scale@+0x1c, qword name@+0x20}`（机器码 write 拷贝 5 qword）；本 crate 橡皮 signifier 同步（{1, 1, 交点, 0.5×半径}）。
- water REVIEW §3.4：音频格帧位公式——滚动位 = `1 << (frame & 63)` 已在首轮按机器码落地（`11*(f+1)` 周期族标注 I）。
- REVIEW §2 #13：signifiers 参数 ResMut（机器码无 tick 回写——core write 取 &mut，以 ResMut 编译等价，登记）。
- REVIEW §3.1：图钉事件载荷 0x100 标志（union 载荷 U，登记待补证）。

## 修复轮 3（reviewer-A 复验轮残留，2026-10-16）

- 编译修复：process_tree_cmds  声明 mut；color_menu CustomizationIcon 导入位（resources::color_icon）+ SetGateColorIdCmd 导入 + Query 解引用删除；ui_color_* 的 raycast 面（core RaycastWorld 补非 native-physics 壳——机器码 0x1408cc870 mask 0xcb 查询的默认构建可用面）。
- hide_trees 相机圈门：HideTreesinAreaCmd/HideTreesNextToWallCmd 载荷 {中心, 半径} + Circle2d 过滤（不再无条件删）。
- place_tree：packed_position 解包（pack_2x16f_uint 对偶，f16 位格式）；InsertTreeCmd 位置真实。
- ui_move_tree：拖拽状态机（Local 起点/拖动/释放三态，只移拖拽树）。
- core TreeStorageEntry + position（0x48B 记录 +0x10 打包位置解包面）。
- water：帧位公式 11*(f+1)；图钉事件带 0x100 标志。
- set_roof_customization：RoofHistoryEdit 真实载荷（头 {1,1,4} + 目标 id）。

## 修复轮 3（reviewer-A 复验轮残留，2026-10-16）

- 编译修复：process_tree_cmds 的 commands 声明 mut；color_menu 的 CustomizationIcon 导入位（resources::color_icon）+ SetGateColorIdCmd 导入 + Query 解引用删除；ui_color_* 的 raycast 面（core RaycastWorld 补非 native-physics 壳——机器码 0x1408cc870 mask 0xcb 查询的默认构建可用面）。
- hide_trees 相机圈门：HideTreesinAreaCmd/HideTreesNextToWallCmd 载荷 {中心, 半径} + Circle2d 过滤（不再无条件删）。
- place_tree：packed_position 解包（pack_2x16f_uint 对偶，f16 位格式）；InsertTreeCmd 位置真实。
- ui_move_tree：拖拽状态机（Local 起点/拖动/释放三态，只移拖拽树）。
- core TreeStorageEntry + position（0x48B 记录 +0x10 打包位置解包面）。
- water：帧位公式 11*(f+1)；图钉事件带 0x100 标志。
- set_roof_customization：RoofHistoryEdit 真实载荷（头 {1,1,4} + 目标 id）。
