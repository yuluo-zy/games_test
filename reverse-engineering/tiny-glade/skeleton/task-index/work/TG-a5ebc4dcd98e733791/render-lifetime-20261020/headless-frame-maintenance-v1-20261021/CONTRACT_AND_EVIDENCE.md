# 共享 headless 世界入口补齐帧首完成维护

## 变更范围

这是现有工程入口的一致性修复，不是重写原 renderer。root 明确委派现 source 窄候选；只改 render_headless_gameplay_frame 入口，在原 draw-list 计时与图声明之前调用现有 bridge.collect_completed_frames。生产 RenderHost、原 shader/counter/slot/range 算法、在飞帧上限、测试采样点、断言与阈值全部不改。不新建生产文件。

## 原调用合同及当前接缝

原 B render 的已恢复外层位于 country-core/src/systems/render_loop/render_frame.rs。其 render 第334行 host.begin_frame 后第336行 host.drain_finished_frames，再进入compile/execute。对应 ProductionRenderHost::drain_finished_frames（production_renderer.rs:1253）调用 ProductionRenderer::collect；后者对真实 App bridge.collect_completed_frames，再清完成present tickets/swapchain并做GC。acquire_swapchain_image另有一次collect，位于主图execute之后。

现 headless 公共producer声明自己使用同一production declare→compile→execute→export链，其调用者拥有App update与temporal推进；之前入口只从draw-list开始，没有帧首完成维护。测试在每次rows样本之后调用collect，而非每次public入口之前。VulkanGraphSession::begin_frame只有pending达到frames_in_flight时才wait，不会自动清理未达到上限但GPU已经完成的前一帧。

## 两批lease真实owner路径

- BindlessHeap::bind_command/bind_pass将每个实际BindlessLease Arc通过retain_recorded/retain_opaque存入native VulkanCommandBuffer.owners；PreparedBindlessParameters局部Drop不撤销这些命令owner。
- record_submission将真实SyncPoint同时记到command.last_use及其uses（包含lease.last_use）。
- collect_completed_frames取得device.poll snapshot，只在队头pending.execution.completion().is_reached(snapshot)时调用wait_frame(frame,0)。GraphExecution.completion来自Transaction累计的所有真实batch/auxiliary receipts的SyncSet，不能仅凭日志cached最后seq推断。
- wait_frame使用整个completion SyncSet。Timeline.wait会再次poll；已经达到时直接返回，正常单调时间线不会发vkWaitSemaphores。该入口没有idle/主动等待未完成帧。
- finish_frame observe每个receipt后chain.recycle，recycle_completed_commands调用raw.release_completed_owners清owners。其BindlessLease最后Arc Drop才使slot Live→Retiring；随后真实completed collect才能到Free。未完成的帧完全保留。

## 实际观察与尚未证明部分

/private/tmp/tg-highwater-world-thirty-20261021.log：frame2 live6022，其中images1381、storageimages136、buffers4433；后期约3043（images689/690、storageimages68、buffers2210）。这与两批实际命令owner同时保留一致；旧pool peak错误已修，新真实peak6022固定。frame2 constants pages4、inflight2，也支持前帧owner未recycle的假设。

日志retirement_stats.completed是cached值，无法独立证明“helper入口时前帧完整completion已到达”。本候选补齐已确认缺失的非阻塞维护，不声称必然让所有合法frames-in-flight下live样本完全相同。若前帧此时尚未完成，它继续活着；原30帧门仍必须照常执行，失败继续披露，不延后采样、不追加wait、不改steady起点。

root验证：原30帧所有原live/peak/memory/pixels断言保持，必要记录同一完整receipt集合与实际poll snapshot，而非cached最后提交值；真实WSI原门按root计划。作者只做源码/日志审核及rustfmt解析，不运行Cargo/GPU。此窄维护调用使用已有真实集成测试，不新增镜像实现的mock测试。
