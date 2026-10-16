# RESTORE — input/UI/mode 族（卡E，order 6）

实现者: restorer-interaction
日期: 2026-10-16
范围: ui 资源/控件主闭包（widget_params/slider3d）+ AppMode 接线核对；mode 转换/工具栏细节下子卡。

## 新增实现

### systems/ui/resources/widget_params.rs（新建，O）
- `WidgetParams` 0xb0 B 参数块：默认 u32 组 `{0,8,0,1,0,2,3,3}`、`UiSounds` 4×24 B、
  `WidgetVolumeShape` 32 B {tag,layer,size}、u16@0xa8=0x101 + 整字节 occlude_by_world、
  flags@0xa9（bit0 no_history_recording、bit2 allow_negative_distance）。
- `UiFocus` u32 {None=0, Toolbar=1, Mei=2, World=3}（is_none/is_toolbar/is_mei/as_mei）。
- `WidgetVolumeShape::clearing_canvas` = {tag 4, layer, 130×130}。
- `WidgetFlags` 字节访问器。

### systems/ui/resources/slider3d.rs（新建，O 结构）
- `Slider3dState`（世界空间线段拖拽状态）+ `project_cursor_onto_line`（0x140a9bd40，
  游标→线段投影 t∈[0,1]）。

## AppMode 接线核对（裁决已定，crate 已对齐）

- crate `AppMode::native_tag`：Terrain=10 ✓、Replay=13（0xd）✓、Editing(None)=0 ✓。
- `AppMode::is_tool_selected` = disc {6..12,15..18}（Wall/Path/Cobblestone/Eraser/Terrain/
  Decorator/Color/Water/Tree/Garden/Fences），与裁决 0x3cfe 掩码一致 ✓。
- 裁决早期列表（4=Water/5=Garden/6=Tree）与 crate 的 15/16/17 差异——经 reviewer 已验
  地形门（0x421 bits {0,5,10}、mode==10）用 crate native_tag 正确，判定该列表为显示层
  序，非工具判别；is_tool_selected 掩码两方一致。核对通过，不改 mode_manager/types.rs。

## 登记待取证（下子卡）

- ui 顶层系统：ui_update/get_ui_volume、hover_intersection、debug_visualize_volumes。
- signifiers 族：signifier_create/signifier_animation/signifier_mark_for_delete、
  UiAppendSignifier 消费。
- brush_preview 系统（systems/brush_preview.rs：brush_preview/brush_size）、
  startup/spawn_brush_previews、paths/path_brush_preview_size。
- mode_manager 模式转换/工具栏细节（check_if_can_switch 闭包族）。

## 编译状态

`cargo check -p country-core --lib` 0 error（startup 组在途错误阻塞测试二进制，非本批）。

## 二.卡E-slice2 追加（coordinator 续派，2026-10-16）

1. **signifiers 族（新建 systems/ui/signifiers/）**：`UiAppendSignifier`（0x28 B 记录）+
   `UiSignifierStream`（无锁环形流，O：0x140b2cf90 write 自旋推进、read cursor@+0x160、
   write counter@+0x168）+ `signifier_create`（0x140b2cfa0，排空→生成指示符事件）。
   动画推进（signifier_animation）与回收（mark_for_delete/despawn）依赖实体机制，登记待接。
2. **brush_preview（新建 systems/brush_preview.rs）**：`BrushPreview` 组件标记 +
   `brush_preview`（0x140865050 结构：读光标+当前工具 BrushSizes → BrushPreviewUpdate
   事件）；clone_behavior 判别 2（0x140867080）登记。
3. **登记待取证**：signifier_animation/mark_for_delete/despawn、spawn_brush_previews、
   path_brush_preview_size、mode_manager 模式转换闭包族（check_if_can_switch）。

## 三.卡E-slice3（mode_manager 模式转换，coordinator 续派，2026-10-16）

**mode_manager 核对补全（既有已实质实现，本批复核通过，未加深）**：
- `check_if_can_switch` 转换闭包族 → `category_advance`/`select_app_mode`/`toggle_photomode`
  （mode_manager.rs 既有）：类别索引推进（current+1 % len、Toolbar.current=N）+ 请求模式
  写入 + Editing/CloneClutter 特判 + Photomode 快捷切换。
- `esc_menu_blocks`（esc_menu_canvas 悬停早退）、`send_mode_just_changed_event`
  （AppModeChanged {prev, current} 双 AppMode 事件）、`mode_manager` 主系统（排空请求双队列、
  History 录音门、时标/工具跳表分派）均既有且 6 测试通过。
- `AppMode::is_tool_selected` = {6..12,15..18} 与裁决 0x3cfe 掩码一致（types.rs 既有）。

**登记待接（实体机制项）**：spawn_brush_previews、path_brush_preview_size、
signifier_animation/mark_for_delete/despawn。

## 留给 reviewer 的核对点

1. WidgetParams 默认 u32 组与 @0xa8=0x101/flags@0xa9 位。
2. UiFocus 判别（0/1/2/3）。
3. slider3d project_cursor_onto_line 投影。
4. AppMode native_tag/is_tool_selected 与裁决对齐。
