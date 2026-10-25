# REVIEW2 — systems/garden 二轮复核（reviewer-eco，2026-10-15）

复核对象：restorer 二轮（RESTORE.md + mod.rs build() 增补）。仅按协调者要求复核两点，
首轮 review_passed（限定）结论维持。

## 逐项复核

### draw_garden 半径公式 — ✅ 维持 OK

- 花丛（AppMode::Garden=17）半径：decomp `fVar17 = *(float *)(param_11[4]+8)` = **brush.garden 直取**；
  现实现 `radius = brush.1.garden`，garden 测试断言 radius=3.0（BrushSizes.garden=3.0）一致。
- 橡皮（AppMode=9）半径：decomp `inverse_lerp(0.3,2.0,brush)*4+(1-t)`；现实现一致。
- 章节"Circle2d 半径 = 笔刷 + 1.5"是**误读**——+1.5（0x1429258d0）实为引脚圆高度
  （`fStack_a0 = cursor.y + 1.5`，decomp 0x140ae4741 段），非半径。现实现 pin height=pos.y+1.5 正确。
- 界内圆半径 0.4*radius*0.8（`0x142925970*fVar17*0x142a16cb8`）一致。OK。

### build（原 systems/garden.rs::impl$0::build 0x140b6d290）— ✅ 等价注册已在 mod.rs

- inventory：`systems/garden.rs`（build 1 函数）为原文件；restorer 将 `build()` 放入
  `systems/garden/mod.rs:108`（register_garden_events + add_garden_cpu_systems：
  draw→replay→record 序 + GardenCpuStage/HistoryCpuStage 链配置）。
- decomp 0x140b6d290 是 Bevy App 注册循环（框架机器码），等价物以 Schedule 上的
  draw→record 顺序实现，可接受。**文件位置差异**（systems/garden.rs vs mod.rs）不影响业务，
  登记即可。
- 注：原 garden 插件可能还注册 audio（systems/audio/garden_audio.rs）与 compute_garden
  （render 域）——均不在 eco 批 draw_garden/record_garden_history 验收范围，作 remaining。

## 结论与 DISPATCH 建议

- **garden = review_passed（限定）**：模式/笔刷/UI/边界/绘制/历史四链 + build 等价注册均核过。
  remaining：全局 plugin 调度、pin 渲染、scatter/audio、compute_garden（库外/他组）。
