# Independent nonempty gameplay acceptance review

Scope: B only, T13 excluded, current rhapsody preserved. No Cargo was run and no production file was changed by this worker. The candidate parses through rustfmt (edition 2021); this is not typechecking or runtime acceptance.

## What the existing test actually exercises

`original_gameplay_chain::shipped_session_through_actual_app_and_production_world_graph` installs the real `run_game_thread` App, platform events, device runtime and bridge-owned asset libraries. `GameFrameDriver::new` installs asset maintenance and the canonical egui context. It then manually calls `App::update` with egui begin/end, loads the actual intro, settles 90 updates, checks three authoritative walls, and invokes `warm_up_gameplay`.

This is a useful startup/geometry/upload/world-graph acceptance path. It does **not** call `GameFrameDriver::frame`, so it does not execute its normal outer render, frame-return accounting, limiter or production WSI presentation. `warm_up_gameplay` is a real compile/execute/export operation with the recovered world/tail subset; the function's own documentation says complete compile_main equivalence is still open. A successful exported texture does not prove correct visible walls or pixels.

The central helper now checks wall publication/construction failures, each authoritative wall's owning `InstancedWall`, no pending CPU update, at least one allocated chunk, seven global wall proxies, and their seven shadow/seven G-buffer/four OC2 admissions. Those assertions are substantially stronger than the previous `walls == 3` plus printed `lists.total_len()`.

## Concrete missing proof addressed by this candidate

`geometry::runtime::sync_wall_bricks` clears each wall's upload-pending flag after copying instances into the sparse CPU shadow. Its subsequent device flush may fail and set `WallBrickUploadStatus.gpu_ready = false`; therefore pending=false plus an allocation is insufficient. `nonempty-uploads.patch` checks the real publisher's ready/error/capacity status, actual nonzero sparse draw ranges, no remaining dirty chunks, the canonical sparse buffer's physical owner, and every wall proxy's nonempty VAO/index-buffer owner on the same bridge. It changes only the existing acceptance helper and its call.

These checks establish production publication and physical handle admission. They do not compare uploaded GPU bytes with CPU shadow. The production sparse buffer requests STORAGE|TRANSFER_DST, without TRANSFER_SRC, so copying it to a staging buffer would require an explicit diagnostic allocation-usage extension. Direct `mapped_slice_mut` or `Buffer::read_bytes` is invalid for device-only allocations. The existing `geometry_gpu` test explicitly uses a `ReadbackDevice` wrapper to add TRANSFER_SRC; that synthetic-geometry test does not replace original-intro acceptance and must not be cited as a real gameplay readback.

## Current production assembly versus stale central state

The current `gameplay_startup.rs` already installs real per-session walls, sky, roof, full clearing and meadow producers in ordered OnEnter sets, and `run_game.rs` no longer preloads bare meshes. Do not reapply the old pending startup patches blindly. `GameplaySessionStartup::Critters` and `::Sheep` appear in the ordered set tuple, but this snapshot has no corresponding producer registration in that file; integration owners are separately completing those candidates.

The latest completed original-gameplay runtime log available for review remains `round12/startup-baseline-v20.log`: B0002 in the old `ui_place_decorator` placeholder before any nonempty assertion or GPU graph. Its source differs from the current resource-only assembly and cannot diagnose current gameplay. `round13/check-v1-result.json` exited 4294967295 with five source changes during the check, not a compiler/runtime pass. No current nonempty gameplay result was available at this review point. CPU App startup must execute first before claiming that the remaining blocker is specifically graph/GPU.

## Minimal executable acceptance order

Central owner only, after coherent source freeze, from `D:/game/tiny-glade`, with `TG_INSTALL_ROOT=D:/game/ljxsj_92385/Tiny Glade` and `RHAPSODY_REQUIRE_VULKAN=1`:

1. `cargo test -p tiny-glade --test original_gameplay_chain shipped_session_through_actual_app_and_production_world_graph --locked --offline -j 2 -- --nocapture --test-threads=1`
2. `cargo test -p tiny-glade --test original_loading_present --locked --offline -j 2` for the other GPU worker's actual `GameFrameDriver::frame` loading WSI fixture. It proves loading-frame/present behavior, not nonempty gameplay.
3. A nonempty gameplay WSI run must use the real production `GameLoop`/`ProductionRenderer` and finish real intro loading, perform repeated normal `GameFrameDriver::frame` calls, require no renderer error, require successful presents, and retain the same content assertions. Replacing normal render with repeated warm-up or a headless fake `RenderHost` would not meet that scope.

Bind each result to start/end source fingerprints and actual executable identity. Do not promote source candidates or rustfmt success to runtime completion.
