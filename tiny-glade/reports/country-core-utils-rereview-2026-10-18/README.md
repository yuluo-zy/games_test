# utils 重构后再次审查（2026-10-18）

结论：**changes_required**。确认 3 个可复现问题：1 项 P1、2 项 P2。本轮只审查、记录并运行反例，没有修改业务实现。

范围为当前 `country-core/src/utils` 全部文件，以及滑块、网格资产、虚线绘制回调、指示符创建/删除等消费路径。以当前产品正确性为准，不以原作逆向行为作为保留错误的理由。

## R1 · P1：虚线实体与绘制回调使用不同的 ECS 组件

位置：[debug_viz.rs:274](../../crates/country-core/src/utils/debug_viz.rs#L274)。`DebugLine::show` 写入 `components::DashedLineLength`，但 `dashed_line` 着色器对应 `Env57` 的 [read_env57:595](../../crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a.rs#L595) 强制读取 `DashedLineLengthView`。后者是 [独立派生 Component 的新结构体](../../crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs#L63)，不是同一类型的别名。

触发：提交一条合法虚线 → 用真实 `signifier_create` 唤醒 → 将生成实体交给虚线回调的真实读取入口。网格和原组件均存在，但组件 TypeId 不同，`comp::<DashedLineLengthView>` 的 expect 必然 panic。上一轮只断言原组件存在，未走实际绘制读取端，因此漏检。

建议：绘制回调再导出/使用规范的同一个组件，消除平行类型；增加工厂生成实体后调用真实读取入口的回归测试。给实体同时补两个同义组件会留下状态同步风险。

反例：`actual_dashed_draw_callback_cannot_read_the_spawned_component`，捕获到了 `draw_callbacks_a.rs:166` 的缺失组件 panic。无需真实 GPU 即可到达失败点。

## R2 · P2：滑块外推能越过透视奇点，返回相机背后的点

位置：[project_cursor_onto_ray.rs:103](../../crates/country-core/src/utils/project_cursor_onto_ray.rs#L103)。`total_weight.abs()` 只排除了接近零的分母，负权重仍被接受。两个采样端点的 w 都为正，不代表外推后的世界点也在前方。

触发：正常 90° 右手透视、100×100 窗口，射线起点 `(0,0,-1)`、方向 `(0.2,0,-1)`，光标 `(62.5,50)` 在窗口内。真实 `Slider3dState::project_cursor_onto_line` 使用 ±1e10 的外推范围：屏幕参数约 2.5、总权重约 -0.25，返回世界点约 `(-1,0,4)`，其裁剪 w 约为 -4。编辑点越过视点跳到相机后方。

建议：拒绝非正透视权重，或把参数限制在可见分支，并检查最终点的裁剪 w；调用层应明确处理投影失败。现有测试仅覆盖采样点间的插值，没有覆盖透视消失点之外的窗口内光标。

反例：`visible_cursor_can_extrapolate_a_slider_point_behind_the_camera`，直接调用真实滑块入口，断言当前返回位置有限但裁剪 w 为负。

## R3 · P2：调试线进入现有删除链后被立即回收

位置：[debug_viz.rs:259](../../crates/country-core/src/utils/debug_viz.rs#L259) 与 [signifier_mark_for_delete.rs:54](../../crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs#L54)。新实体携带 `SignifierId`，但当前删除系统无条件建立空 live 集合；实体又没有最短寿命组件，expired 为 true。

触发：UiState 保持活动，提交并通过真实 `signifier_create` 创建普通调试线，再执行现有 mark/delete。实体马上被标记并 despawn，网格进入回收队列。依赖宿主顺序，可能在第一次绘制前消失，或反复创建/销毁；没有形成稳定的活动请求生命周期。

建议：创建与删除共享本帧/当前配置的有效指示符集合，并建立明确的系统顺序；验证“请求持续时保留、撤销后删除、网格只回收一次”。仅添加一个延迟寿命不能修复空 live 集合。问题在现有消费者中早已存在，本次接入真实 Drawable 后成为可视化链的实际阻断；不将其全部归因为新引入的局部算法错误。

反例：`current_signifier_cleanup_immediately_despawns_an_active_debug_line`，运行真实创建、标记、删除系统，证明活动请求刚创建的实体被销毁并入回收队列。

## 执行与限制

本轮 3 个定向反例全部复现，Cargo 退出 0；测试断言的是当前问题能够观察到，**通过不表示修复完成**。投影最终使用真实 `Mat4::perspective_rh_gl`；初步检查使用的简化矩阵记录保存在 initial 日志中。

测试对 shader 编译边界复用项目现有 `render/test_device.rs`，业务创建、滑块、绘制读取和删除代码使用当前真实实现。没有执行真实 GPU 绘制或 FMOD 设备，也没有原二进制等价验收。本轮未发现足够证据支持新增栅格/JSON/索引/曲线问题，但不据此宣称这些路径全输入正确。

复验命令（从 `games_test/tiny-glade` 根）：

```sh
python3 reports/country-core-utils-rereview-2026-10-18/run_probes.py
```

[probes.rs](probes.rs) · [运行脚本](run_probes.py) · [最终日志](probes.log) · [验证与源码指纹](verification.json)。脚本独占创建临时集成测试文件并在结束后删除；输入源码前后指纹相同。保存的探针不进入默认测试集。
