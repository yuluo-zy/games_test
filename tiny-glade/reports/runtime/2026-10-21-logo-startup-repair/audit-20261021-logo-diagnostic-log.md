# Independent audit of the compatibility Logo diagnostic log

Read-only audit of `D:/game/tmp/logo-startup-diagnostic-process.log`; no new GPU/test execution. Machine summary and log SHA256 in `audit-20261021-logo-diagnostic-log.json`.

**Supported conclusion:** on local Intel UHD Graphics630 with explicit `TG_DIAGNOSTIC_RETAIN_WSI=1` and480×270 window, actual load-completion→GPU warmup→Logo wait release→Logo finish→two-App owner handoff→60 gameplay frames/submissions/queued presents completed. It is accurate to call **“本机显式兼容模式的 Logo→gameplay 启动链已跑通”**, while keeping strict WSI/clean shutdown and whole-game business acceptance separate.

Evidence ordering:
- Lines3–6: explicit480×270/windowed policy, explicit RetainUnproven warning, actual Intel UHD630 device.
-150–151: actual LoadingGameComplete after3updates, warmup begins.
-177–181: actual world execution stats, warmup complete, prepared SubApp publication/receipt, releases Logo wait.
-183–184: Logo finished, actual owner handoff begins/completes.
-194–196: progress28→58→60 gameplay; final completed1202/gameplay60/submissions60/presents_queued60. Source GameFrameDriver increments these only after render_result and host_result succeed, not synthetic loading markers.
-197: first **fatal event-loop** error is final presentation proof Unsupported;198–203 are cleanup retries and actual WSI generation2 retained;204 final returned error preserves same cause.
- Zero `panic/panicked/device_ready/stack backtrace` matches. Zero progress/warmup/handoff/world-stats lines after line197. Thus this log has no observed erroneous redraw/render progress or device_ready panic after terminal failure. Source guards provide supporting explanation; silence alone is not a full queued-event instrumentation proof.

Limits that must remain in report:
- This is NOT error-free: line1 crash reporter unavailable (log-and-continue),146 missing intro snapshot with fallback,186/191/193 FMOD missing parameters warn; final shutdown Unsupported remains an actual error.
- `presents_queued60` proves real successful host presentation enqueue path for60frames, not60final scanout/completion proofs. No current PresentTicket proof/screenshot/pixel visual acceptance is in this log.
- Final shutdown did not succeed and generation retention is explicit. Exit1 and CloseMainWindow provenance are central process-run receipts; the merged process log itself does not encode an exit-code record or distinguish OS close from another close command, so this independent read cannot newly attest those mechanics.
- Parent supplies executable identity68C5AE58...; this process log does not embed executable hash/path, so hash-to-run binding depends on central launch receipt. Auditor did not rebuild/run or independently hash an unspecified exe.
- Measured timeline is about124seconds, loading/warmup/handoff around36seconds, then60gameplay frames around88seconds on this hardware. Do not interpret successful chain observation as performance target met.
