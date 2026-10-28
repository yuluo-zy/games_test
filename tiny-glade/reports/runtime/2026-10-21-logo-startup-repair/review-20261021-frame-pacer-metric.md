# Frame pacer metric independent review — 2026-10-21

Read-only source and existing original-evidence audit. No Cargo/GPU run. W34 result is separately recorded in review-w34-late-target.md.

## Finding: production host supplies a different measurement

ProductionRenderHost::last_gpu_frame_ns reads VulkanGraphSession::last_known_frame_duration_ns and treats nonzero as Some. Current rhapsody rg/vulkan.rs:3063 / 3089 assigns PendingSessionFrame.started_at=Instant::now only after chain.execute_prepared_validated has executed/recorded/submitted the graph. finish_frame:3202–3302 publishes started_at.elapsed only after actual completion polling/wait, command/resource recycling, native-resource pool handling and retire callbacks. The host calls collect at next frame drain and again before acquisition. Thus this measures post-submission-to-late-harvest wall time, including intervening CPU work, delayed polling, and reclaim/retire callbacks. It can miss GPU work that began before the final host submission returned. It also updates for each graph (main and presentation), not one original top-level main+presentation frame bracket.

Original evidence does not support equating it to GPU timestamp duration. Existing 0x140beaae0.asm.txt in evidence/render-recovery/parallel-batch-02/executor-temporal proves: mapped buffer accessor @140beaaec; two-u64 subtraction @140beab04/08; device float timestampPeriod @140beab33 and mulsd @140beab3f; Option Some discriminator eax=1 and ns payload rdx @140beab95. Existing begin/end-frame evidence/findings describe two top-level timestamp queries, opening timestamp before execution and bottom-of-pipe closing timestamp + readback at end_frame. This is a GPU-clock bracket, not Instant elapsed since submission.

Core FramePacer itself retains the original four-sample formula: max(min(gpu seconds) - max(cpu seconds),0). CPU sample is previous frame end→postcompile begin plus execution segment. Source original render.asm calls last_known at @1404ea096; the later min/max and wait control are retained. The wrong host measurement can inflate or understate trim budget. It should not be labeled original GPU frame time.

## Why this is not yet a low-FPS root cause

Current wait_gpu_queue_trim uses the true oldest pending frame completion SyncSet and calls queue timelines.wait with the budget. It returns immediately if complete; Timeout becomes Ok(false). It does not sleep for the full budget regardless of GPU completion. A wrong budget alone therefore does not prove a long actual wait or explain reported GPU underutilization. Main→presentation graph ownership and PresentTicket::wait on frames_in_flight still protect real GPU/presentation work; do not remove them. Need actual phase timings: compile/prewarm, execute/main, acquire including ticket wait, presentation execute, native present, collect, trim budget and actual trim elapsed. Report main-vs-presentation graph identity and pending queue depth beside samples.

## Narrow safe adaptation

Keep the original FramePacer algorithm, user input_lag_reduction option, and mandatory actual-completion waits. RenderHost already returns Option<u64>; when no true frame-bracket GPU timestamp is available, returning None is semantically truthful instead of injecting this wall-clock value or record_ns/submit_ns. This should be documented as missing backend measurement, not a recovered frame timing algorithm or performance fix. If adopted, independently measure effect before attributing a FPS improvement.

Existing VulkanGraphBackend configure_gpu_timing / poll_gpu_timings / drain_gpu_timings are nonblocking real query paths. Their public GpuCommandTiming contains per-physical-queue command duration (and optional overlapping pass spans), queue, recording epoch and completion counter; it does not expose absolute endpoints or an application frame ID. Summing overlapping queues, taking max of commands, or adding overlapping pass spans cannot reconstruct the original top-level frame bracket. Keep these for explicit diagnostics; do not fabricate an equivalent production GPU frame sample from them. The current begin_frame also reconfigures profiling per frame; introducing persistent collection requires a separate host lifecycle design, respecting configure rejection while queries are in flight. No rhapsody algorithm changes are recommended in this narrow repair.

## Concurrent host correction observed

After the findings above were sent, central source now returns None from ProductionRenderHost::last_gpu_frame_ns (currently line 1720) with explicit missing-original-GPU-span documentation. It retains the original core pacer and actual completion waits. New diagnostic RenderStageTiming keeps observed_completion_wall_ns separately from GPU duration and measures trim budget / actual elapsed plus present wait. This addresses the semantic input mismatch at the host boundary; it is not restoration of the missing top-level timestamp span or proof of the reported FPS root cause. No execution verification was performed by this reviewer.

