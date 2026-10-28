# Toolbar independent gate

Status: static review and OS-input harness prepared; this file does **not** assert a module pass. Cargo and gameplay execution are owned by the central agent.

## Static findings

- MEI menu registration now follows the bounded original body at `mei-interactive-registration-original.asm`: IGNORE_INPUT rejects, FORCE_INTERACTIVE bypasses the content-vtable test, and the resulting interactivity byte gates registration after hover calculation. Empty forced-interactive menu roots therefore register; registration is not conditional on cursor containment. Current `mei/systems.rs` matches that specific gate. The original bounds remain multiplied separately before addition.
- Production set6 orders picker/UI update, create, mark, animation time, animation transform, despawn, and MEI preparation with `chain_ignore_deferred`. Creation/mark visibility retains the ordinary schedule boundary. Native geometry consumed by picking belongs to the previous frame; moving producers in front would change behavior.
- `signifier_create` consumes owned Factory records only while UI is active, enforces the captured unique WidgetParams owner, and uses a frame-start existing-entity hash snapshot. It attaches full WidgetId alongside the hash after real factory wake. `mark` derives live identities from the same captured owner. NaN expiry and zero-animation deletion behavior are represented explicitly.
- Toolbar Replay builds the real `deny_input_toolbar` closure, Ui2dImage `icon_cross` and mouse-follow marker. The production follow system is registered in set7. Empty dynamic builder meshes retain TransientMesh receipts and are eligible for actual mesh/VAO maintenance after entity deletion. The existing replay-chain test exercises the original UI raster pass and dropped transient receipt cleanup, but that is not WSI acceptance.
- Current toolbar events are registered with `add_event` rather than resource initialization alone. This static finding does not independently prove their frame-maintenance registrations; central schedule/maintenance regression remains required.
- The numeric Raw signifier compatibility surface remains explicitly rejected by the recovered consumer; downstream producers belong to later module contracts. This is a known failure boundary, not a complete implementation of those later flows.

## Executable OS gate

New `tests/toolbar_window_chain.rs` is a harness=false test; it assembles the existing `run_game_thread` and real `GameLoop`, reads actual App resources through existing getters, and never writes business state after startup. Its Python fixture sends XTest events to a unique PID-specific window title. It does not operate on other windows.

Expected phases:

1. Full original loading/gameplay handoff, nonempty production world, native present; actual Welcome Showing.
2. Physical click behind Welcome leaves authoritative AppMode/Toolbar/history unchanged.
3. Physical Return closes actual Welcome and sets the actual profile dismissal flag.
4. Physical click on actual emitted category geometry changes authoritative AppMode and Toolbar, selects the actual button, and preserves edit history.
5. Start fresh frame/native-present baselines only after real Welcome closure and successful tool selection. At least 30 additional production gameplay receipt frames and native present increments are required. Capture entity/mesh/transient-delete/RSS resource counts at the beginning and end, plus main-loop redraw wall-time samples. Genuine WM_DELETE_WINDOW then causes production shutdown and releases save-thread and preparation ownership.

Snapshots print `TOOLBAR_WINDOW` JSON with world identity, mode, category, nav, menu, profile, history, entity count and save owner. Real WindowEvent mouse/Enter releases are counted. Screenshots are not used to infer state. New window and account are test-only adapters; rendering and input consumers remain production implementations.

Central build command: `cargo test --offline -p tiny-glade --test toolbar_window_chain --no-run`.

Run the resulting test executable with `DISPLAY=:1`, `TG_INSTALL_ROOT` pointing to the existing local packaged assets, `TG_FRAME_TIMING=1`, and `TG_FRAME_TIMING_EVERY=1`. Optional `TG_TOOLBAR_WINDOW_TIMEOUT_SECONDS` defaults to 240; `TG_TOOLBAR_WINDOW_FRAMES` defaults to 30 and cannot be less than 12. All writes use a fresh `/tmp/toolbar-window-account-*` directory, held until after the real save-thread shutdown. No user account is read or modified.

For the regular host binary, equivalent isolation uses `--override-save-root <local temporary parent> --override-save-root-user-id 0`; Paths appends `Tiny Glade/Steam/0`. Use `--window-size 960x540`. `TG_SMOKE_FRAMES=N` exits via production shutdown after real counted frames; avoid a hard timeout kill as normal acceptance. `TG_STARTUP_UI_CAPTURE=<absolute PNG>` queues a real screenshot after reveal completion and an additional 0.5 seconds.

Performance must separate Welcome/menu phases: the original menu policy limits FPS to 60. Compute gameplay percentiles only after real menu closure and stable tool selection. XTest key hold and child-process creation are validation overhead, not the steady-state sample.

The new harness prints a `steady_observation` summary with mean/p95 main-loop redraw wall time and `performance_verdict=not_evaluated`. This includes waits and pacing, not isolated CPU execution duration. Existing `TG_FRAME_TIMING` logs provide stage attribution; neither observation alone establishes the target-hardware FPS gate.

## Performed here

- Read-only X11 enumeration: `DISPLAY=:1 python3 /tmp/toolbar-x11-input.py list` succeeded. It found an existing unrelated 480x270 validation window; no input was sent to it.
- Python fixture syntax compilation succeeded. No Cargo build or gameplay run was executed by this agent.

## Source fingerprints at review

| Source | SHA-256 |
| --- | --- |
| toolbar MEI systems | `04e24251246aece1a10b85edd2bc9887d5397f5ca2baedaafd3e40a1f1b41bc6` |
| production toolbar signifier registration | `12fb04755c21b620acc26e6e955ec1bb1a809cfd2b4acba94b5e4b78c1293cd8` |
| signifier create | `170bccbd3000e6606bee0fe122bcaccefc5bf06a03e3beb6c68e3209ff0c5d0c` |
| signifier mark/despawn | `3b407c99def41aed184c979752ef22be546e93faa2c81c7bd39e795e1e5ca82a` |
| toolbar window test | `4f851b7f1f0ed389dc2076671df82809b54ced4dd404c24df56d9f61c7fa315f` |
| XTest fixture | `1f5122426e6d768cb2596a6f67434004e62154bcb3204df16a1fb3ca2fc82528` |

Changes to the reviewed gates, owners, schedules, startup adapters or the test invalidate corresponding conclusions and require central revalidation.
