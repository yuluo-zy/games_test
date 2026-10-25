# REVIEW2 — systems/ui_mode(卡E-slice2 终验 → 闭合·signifiers+brush_preview)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md 二.卡E-slice2 节;`cargo check --lib` exit 0(独立复验,3 警告)。

## 核对

### ① signifiers 族 — PASS(结构,环形流近似诚实登记)
- `UiAppendSignifier`(0x28 B 记录:tag u8 + position Vec3 + extra [u8;12];Rust 结构 28B
  vs 原 0x28B 槽差异待取证——extra 字段语义 I,内部自洽)。
- `UiSignifierStream`:0x170 B 无锁环形流(read cursor@+0x160、write counter@+0x168、
  0x28 B 槽);`write`(0x140b2cf90)拷贝 0x28B 记录 + 自旋推进游标(36 callers 登记)。
  本实现以 `VecDeque` + 容量裁剪(8 槽)承载,**注释明确"重建写成 Vec 是错的,真实为无锁
  环形流"**——无锁语义登记为宿主并发接线待取证,诚实。
- `signifier_create`(0x140b2cfa0):排空环形流 → 逐条生成 [`SignifierSpawned`] 事件;实际
  spawn 指示符实体依赖实体机制(宿主),登记待接 ✓。

### ② brush_preview — PASS(结构)
- `BrushPreview` 组件标记 + `CLONE_BEHAVIOR_DISC=2`(0x140867080 `mov eax,2`)✓。
- `brush_preview`(0x140865050,26KB 结构):读 CursorTerrainRaycast/CursorPositionSS +
  工具 BrushSizes(7=Path/15=Water/17=Garden,fallback 1.0)→ BrushPreviewUpdate 事件。
  逐工具笔刷大小映射为结构推断(26KB 主体逐分支待取证,诚实)。
- `register_required_components`(0x140866800)空存根胶水 ✓。

### ③ 待取证登记诚实 — PASS
signifier_animation/mark_for_delete/despawn、spawn_brush_previews、path_brush_preview_size、
mode_manager 模式转换闭包族(check_if_can_switch)——全列下子卡,无冒充。

## 结论与 DISPATCH 建议

- 卡E-slice2 复验通过:**signifiers 族 + brush_preview 闭合(结构 + 事件面)**;环形流无锁
  语义与实体 spawn/动画回收为登记待接。
- 建议:slice2 → 闭合;slice3(mode_manager 转换/工具栏 check_if_can_switch)续派。
- 全 lib 编译 exit 0(3 警告,geometry/sphere 精度败属他组既有)。
