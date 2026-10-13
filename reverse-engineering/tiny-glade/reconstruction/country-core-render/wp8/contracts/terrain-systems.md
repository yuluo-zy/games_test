# WP8 契约 4/4 — 地形系统（编辑笔触栅格化输入 / 导数与光照 / 鼠标射线与回读）

版本 **B**。数据源：skeleton bundles `systems/terrain/*.rs`、`systems/render_loop/compute_passes/
{rasterize_terrain_stroke,displace_rocky_terrain,terrain_lighting_texture}.rs`、`resources/
{terrain_editing,terrain_stroke,active_raster_stroke,mouse_terrain_raycast,cursor_terrain_raycast,
terrain_rock_mask}.rs` + 本轮 Ghidra 反编译（地址见 evidence.json）。ECS 签名来自 structures.json.gz 的
`FunctionSystem<void(*)(…)>` 记录（PDB 权威）。

标注：**O**=反编译/PDB；**I**=组合；**U**=未恢复（typed gap）。

## 1. 资源注册（O，insert_terrain_resources @0x1409457c0 / add_terrain_events @0x140945a20）

```
insert_resource<MouseActivityTimer>            // ui_edit_terrain
insert_resource<TerrainStrokeParamsCache>      // ui_edit_terrain
insert_resource<TerrainHeightsData<TerrainHeightSsbo>>
insert_resource<TerrainHeightsData<VisualTerrainHeightSsbo>>
insert_resource<HoveredTerrainStrokeId>        // terrain_hover_idx
insert_resource<ScheduledTerrainReadback>      // delayed_terrain_changed_msg
```
事件（register_event）：`EvTerrainStrokeChanged`（draw_terrain_stroke）、`EditTerrainStrokeCmd`/`SetTerrainStrokePtsCmd`
（edit_terrain_stroke）、`AddTerrainHighlightCmd`（highlight_active_terrain_stroke）、`RecalculateHeightMapCmd`
（rasterize_terrain_stroke）、`OnTerrainChange`（delayed_terrain_changed_msg）。

## 2. 编辑输入：ui_edit_terrain / ui_draw_terrain / ui_terrain_focus_mode / edit_terrain_stroke

### `ui_edit_terrain`（O，0x140aef1f0，行 34）
```
fn ui_edit_terrain(
  Res<AppMode>, Res<Modifiers>, Res<TerrainEditingData>, Res<UiState>,
  ResMut<TerrainStrokeParamsCache>, ResMut<PendingTerrainHistory>, ResMut<DecoratorArchivist>,
  EventWriter<EditTerrainStrokeCmd>, EventWriter<CursorCaptureCmd>,
  EventWriter<EnableArrowGizmoStretch>, EventWriter<AddTerrainHighlightCmd>,
  (EventWriter<NotifyHintSystemTerrain>, EventWriter<NotifyHintSystemTerrainSteepness>,
   Local<TerrainSteepnessEditAmount>),
  EventReader<MainCameraMoved>,
  (EventReader<CursorMovedScreenSpace>, Res<CursorTerrainRaycast>),
  Res<HoveredTerrainStrokeId>)
```
- 根据鼠标落点 + hover 笔画 id 生成 `EditTerrainStrokeCmd`/`SetTerrainStrokePtsCmd`（I：反编译多事件写入）。
- `MouseActivityTimer`/`TerrainStrokeParamsCache` 为 `ui_edit_terrain` 自有资源（O：insert 行 211/22）。

### `ui_draw_terrain`（O，0x140a9fdc0，行 40）
```
fn ui_draw_terrain(
  Res<AppMode>, Res<CursorTerrainRaycast>, Res<AssetSsboLibrary>, Res<WaterRaster>,
  ResMut<PendingTerrainHistory>, Res<UiState>, ResMut<TerrainEditingData>,
  ResMut<DecoratorArchivist>, EventWriter<EvTerrainStrokeChanged>,
  EventWriter<NotifyHintSystemTerrain>, EventWriter<NotifyHintSystemToolWasJustUsed>,
  (Res<GladeBorder>, EventWriter<DisplayOutOfBorder>, EventWriter<FeedbackInputOutOfBorder>),
  Local<DrawState>, Local<CacheMouseRaycastHeight>, EventWriter<PlayTerrainSound>)
```
- 高亮/编辑模式下点按绘制新笔触（`new_stroke`），发 `EvTerrainStrokeChanged`（I）。

### `ui_terrain_focus_mode`（O，0x1409a4fb0）
- 签名见 evidence（含 `Local<TerrainFocusModeOrigin>`、`OutlineWallCmd`、`PublicWalls` 等）；行 94/175/261/324/441/481/531 七处闭包。主要为进入/退出地形聚焦模式的 UI 状态机（I），细节 U。

### `edit_terrain_stroke`（O，0x14094e000，行 0）
```
fn edit_terrain_stroke(
  EventReader<EditTerrainStrokeCmd>, EventReader<SetTerrainStrokePtsCmd>,
  ResMut<TerrainEditingData>, EventWriter<PlayTerrainSound>,
  Res<ScheduledTerrainReadback>, EventWriter<RecalculateHeightMapCmd>, Res<GameTime>)
```
- 按命令 id 在 `TerrainEditingData` 中查笔触；缺失 → panic
  `"Was requested to edit a terrain stroke  that doesnt exist"`（O，行 156）。
- 应用 falloff/height 调整（prior `tg-terrain::edit::apply_resolved`：Absolute 直写、Delta 限幅
  falloff∈[0,1]、height Absolute≥0 / Delta≥0.1——O 原机器码案例，I 挂接），发 `RecalculateHeightMapCmd`
  与 `PlayTerrainSound`（O 签名）。

### `TerrainEditingData` / `TerrainStroke`（资源方法，O）

- `TerrainStroke::insert_or_morph_last_point(&mut, p)`（O，0x140a97880，行 42）：
  len==0→push；len==1 且 dist(p,last)>0.001→push（否则不变）；len≥2 且 dist(last,prev)≥spacing→push
  （len<0x400=1024），否则 **覆盖 last 点（morph）**。prior `tg-terrain::stroke.rs` 已逐条恢复此规则（O 原机器码）。
- `TerrainEditingData`（O，0x140924900/0x140924a10）：字段观察 `{hashmap@{+8 数据/+0x10 掩码/+0x20 计数}, +0x38 last_stroke_id}`；
  `new_stroke(p)`（行 91）：`last_stroke_id < 0x3fffff` 则 +1 并 `HashMap<TerrainStrokeId, TerrainStroke>::insert`，
  否则 None；`get_last_mut_stroke`（行 83）按 last_stroke_id 查。`last_stroke_id` 行 87、`set_stroke` 行 110、`clear` 行 120。
  `TerrainStrokeId = u32 newtype`（prior schema；serde transparent）。

## 3. 笔画变化→重算：draw_terrain_stroke / deferred_terrain_history / delayed_terrain_changed_msg

### `terrain_stroke_changed`（O，0x140a03ba0，行 17）
```
fn terrain_stroke_changed(
  EventReader<EvTerrainStrokeChanged>, Res<ScheduledTerrainReadback>,
  EventWriter<RecalculateHeightMapCmd>, EventWriter<PlayTerrainSound>)
```
- 有 `EvTerrainStrokeChanged` 事件时发 `RecalculateHeightMapCmd` + `PlayTerrainSound`（O）。

### `history_record_terrain`（O，0x140ac9770，行 16）
```
fn history_record_terrain(ResMut<History>, ResMut<PendingTerrainHistory>, EventWriter<DebugHistoryMsg>)
```
- `PendingTerrainHistory` 置脏标记（`0x8000000000000002` sentinel，O）；首次变化时构建历史记录并 `History::add`，
  发 `DebugHistoryMsg`（I）。之后不再重复记录（O）。

### `ScheduledTerrainReadback` + `delayed_terrain_changed_msg`（O，0x140aabdd0，行 66）
```
struct ScheduledTerrainReadback {  // 布局部分 O，字段名 I
  counter: AtomicUsize,   // +0x00；on_terrain_modified 增量（O）
  flag: bool,             // +0x08；read_terrain_data 置位“本帧有变更”（O）
  delays: Vec<u32>,       // 延迟计数列表（I；delayed 主循环逐项递减）
}
fn on_terrain_modified(Res<ScheduledTerrainReadback>)  // +0x00 自增（O，0x140aabdc0，行 40）
fn delayed_terrain_changed_msg(
  Res<AssetSsboLibrary>, ResMut<ScheduledTerrainReadback>, Local<DelayedUpdateWallsMsgBuffer>,
  EventWriter<OnTerrainChange>, ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
  ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>)
```
- 每帧：递减 `delays` 各项；某项到 0 → `FrozenMap::get<SsboId,Box<BufferState>>`（断言 SSBO 尺寸 0x3a/0x40）
  → cpu_readback → `TerrainHeightsData::copy_from_gpu<TerrainHeightSsbo>`（两次：高度+视觉，O）；每帧至多一次。
- `flag` 置位时追加新项 `delays.push(2)`（延迟 2 帧后回读，O）；随后压实（移除已完成的 0 项，O 紧凑循环）。
- 并发安全：counter 用 `LOCK()/UNLOCK()`（std atomic/互斥，O）。

## 4. 鼠标射线与回读：mouse_terrain_raycast / cursor_terrain_raycast / terrain_hover_idx + compute 回读

### `CameraCursorRay` / `CursorPositionSS`（O，resources/mouse_terrain_raycast.rs）
- 布局（O，mouse_terrain_raycast@0x1408ef320 + intersect_with_xz_plane@0x1408ef180）：
  `CameraCursorRay { tag: u32(+0x00=1有效), pos: Vec3(+0x04), dir: Vec3(+0x10), length: f32(+0x1c) }`（32B）。
- `intersect_with_xz_plane(ray, y)`：`t=(y-pos.y)/dir.y; pos+t*dir`（O 浮点）。
- `mouse_terrain_raycast`（O，0x1408ef320，行 63）：
  若 `CameraCursorRay` 已有效→xz 平面；否则 `TerrainHeightsData::intersect_ray<TerrainHeightSsbo>`（O）；
  命中→写 pos/dir/length。闭包 `closure$0`（0x1408efa70）额外对实例网格求交（I，逐 mesh 最近点）。

### `CursorTerrainRaycast`（O，resources/cursor_terrain_raycast.rs）
- 布局 `{tag u8(+0x00), pos Vec3(+0x04)}`（O，position@0x140a12650 读 +4/+0xc）；`position()` 行 35、
  `update` 行 20、`new` 行 39。Option<Vec3> 语义（I）。

### `terrain_hover_idx`（O，0x140aff720，行 17）
```
fn terrain_hover_idx(
  Res<AppMode>, (Res<AssetSsboLibrary>, Res<WaterRaster>, Res<CursorTerrainRaycast>),
  ResMut<HoveredTerrainStrokeId>)
```
- 仅特定 AppMode（位掩码 0x421 = bits{0,5,10}，O）。
- 读鼠标回读 SSBO（`FrozenMap::get`，断言 0x6c）→ 笔画 id（+0x10，O）；采样水位 ≤0 或光标无效或
  `HoveredTerrainStrokeId` 距离≤0.5 时不变；否则写 `HoveredTerrainStrokeId {tag=1, id}`（O）。

### `HoveredTerrainStrokeId`（I）
`{ Option<u32 stroke_id>, 帧 tick }` 形状（O 写 0/+1）。

### compute 回读 pass（O 反编译，帧外层调用者同 @0x1404ec350）
| pass | 地址 | 行为（O） |
|---|---|---|
| `read_terrain_data` | 0x14098c680 | RasterPass 绘制 `terrain_shader`+`rocky_terrain_shader` → compute：读高度（`TerrainHeightSsbo`+`VisualTerrainHeightSsbo`）、岩石掩码（`TerrainRockMaskSsbo`）、水（`WaterMaskSsbo`+`WaterExcludeClutterMaskSsbo`，非 demo）、花园（`GardenMaskSsbo`）到 SSBO；置 `ScheduledTerrainReadback.flag` |
| `mouse_read_terrain_stroke_index` | 0x1409768b0 | `write_to_cpu_mapped_storage_buffer<MouseTerrainRaycastSsbo>`；读 hover 资源位→写入 cpu 映射 SSBO；dispatch(1,1) |
| `mouse_depth_compute` | 0x140a01620 | `write_to_cpu_mapped_storage_buffer<MouseDepthValueSsbo>`；uniform Vec2(uv)/Vec2+f32；dispatch(1,1) |
| `highlight_active_terrain_stroke` | 0x140a58450 | 高亮 active 笔画：`AddTerrainHighlightCmd` 事件/模式分支 → 高亮 compute + 两次模糊（write_uniform<bool,f32>）；preset 0xb/0xc |

## 5. 导数与光照：rasterize_terrain_stroke / displace_rocky_terrain / terrain_lighting_texture

### `rasterize_terrain_stroke`（O，0x1408c7120，行 65）
- 守卫：active 笔画非空（`Vecs@+0x10/+0x30` 或 未栅格化 flag@+0x40）才执行（O）。
- 遍历 active 笔画（16 位掩码迭代，每笔画 0x30B：id/宽度/点列），过滤 `width>0.01`、点数非空（O）。
- 每笔画收集 16B 动态常量 `{dynamic_const_addr, (id<<10)|index, extra}`（O）；`DynamicConstants::allocate` +
  `push_slice`（O）。
- `ComputePass::new`（`/terrain_editing/rasterize_terrain_stroke.cs.hlsl`）：uniform i32 / array<u32,2> / f32，
  bind preset 9/0xb/0x13/0xe/0xd，`write_storage_buffer<NormalDebugSsbo>`，`dispatch_with<closure_env$1>`（行 165）（O）。
- 随后两遍 `GaussBlur`（`/gauss_blur.cs.hlsl:cs_main`，`impl$0::as_shader_def_value` 行 43 / `impl$5::hash` 行 35）；
  每遍 uniform bool+f32、bind preset 9 + 新建纹理句柄；dispatch（O）。
- 结束时置 active flag=1（本帧已栅格化，O）。

### `displace_rocky_terrain`（O，0x140a3ee80，行 14）——见 rocky-terrain.md §3。

### `calc_terrain_lighting_texture`（O，0x140a51440，行 19）
- `AssetTextureLibrary::get_rg_texture`（输出光栅，O）。
- `ComputePass::new`（`/terrain_lighting_texture.cs.hlsl`）：`CameraMatrices::eye_position()` 写 uniform Vec3，
  bind preset 8/9/10/0x11，uniform bool，dispatch（O）；`generate_mips<TexturePreset>`（O）。
- 以 `std RwLock` 读计数保护纹理库查找（O，`in_RDX+0x90` 读写计数）。

## 6. O/I/U 摘要

- **O**：全部系统地址/行号/ECS 签名；笔触插入规则、回读调度、射线布局与求交、各 compute pass 声明与 URI。
- **I**：UI 系统内部状态机；`closure$0` 实例求交语义；`PendingTerrainHistory` 历史记录内容。
- **U**：`TerrainEditingData`/`TerrainStroke`/`ActiveRasterStroke(s)` 精确布局与 `resample_and_smooth`/`solidify_fluid_into`
  （行 74/143）算法；`ui_terrain_focus_mode` 闭包细节；`MouseTerrainRaycastSsbo`/`MouseDepthValueSsbo` 记录格式；
  `TerrainStrokeParamsCache`/`MouseActivityTimer` 字段。→ typed gap。
