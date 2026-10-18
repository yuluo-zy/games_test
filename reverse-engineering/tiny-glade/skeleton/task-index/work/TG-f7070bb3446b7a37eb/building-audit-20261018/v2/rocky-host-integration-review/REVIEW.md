# Rocky / full72 current host integration review

Point-in-time read-only source review, after Rocky147 freeze and during parallel owner integration. Sixteen source files were captured with no drift during capture. No production code, Cargo, original execution or JIT was performed here. Canonical resource acceptance and real host/GPU acceptance remain separate.

## Current positive wiring

`tiny-glade/src/game_loop.rs:260` installs the actual renderer bridge's libraries before registering full72 at264–265 in **PreStartup**. This registration is on the gameplay App, before it may be moved into the async preparation SubApp. That matches the original full72's PreStartup nested tuple, and differs from Rocky's original Startup8 slot4.

`render_bridge.rs:96` uses init_resource<AssetSsboLibrary>, preserving an existing same-World owner. Lines103–108 install GraphicsDeviceResource with the bridge's actual GraphicsDevice and BufferHost. Main graph access at473 borrows that World resource and FrameWorldAccess forwards the same reference; no second SSBO registry is inserted for the full producer.

`startup/init_populate_ssbo_library.rs` contains 68 unique direct canonical add_new type paths, three add_new_with_data calls for actual Nani data, then exactly one existing create_custom_cursor_ssbo helper. It contains no local surrogate carrier declarations and no Rocky entries. The real EventWriter<CustomCursorSsboCreatedCmd> publishes the mapped cursor pointer after the real canonical registrations. This review confirms current type identity and lifecycle location; it does not independently revalidate all 72 per-type native factory algorithms/capacities or claim the full72 Rust/GPU run passed.

The loading App stays separate: run_game.rs:745 creates its own AssetSsboLibrary; install_asset_libraries preserves it. loading_graph.rs:64/68 registers its minimal producer in **Startup**, with its actual attachment after minimal_startup_ssbo_system. It does not run full72. Sharing only the underlying graphics device is intentional and does not merge the libraries.

## Explicit current Rocky gaps

The inspected host/startup sources contain **zero** calls to RockyTerrainMesh::load, into_vertex_count, runtime RockyTerrainMeshVertCount installation, or the actual rocky JSON path. The current startup_terrain file still contains its manual grid helpers. Root's separate resource147 integration/tests do not create this missing runtime producer. The owner handoff remains required: real resolved assets/data/rocky_terrain.json → existing canonical device/library → deferred count insertion in **Startup**. Original stage evidence and exact count/asset identity are already frozen in147 STARTUP-OWNER-HANDOFF.md; no original probe was repeated.

Pending current W4/W5 sources also still need their existing architecture adapters:

- pass_kit.rs:445–454 has separate RockyTerrainVertexBuf/TriangleBuf foreign_carrier definitions instead of the current canonical resources::rocky_terrain types.
- displace_rocky_terrain.rs:36/53 consumes RockyTerrainMeshHead, while the real resource is the one full usize vertex count. W4 should divide that once by three before narrowing.
- draw_callbacks_b.rs:241/552 consumes separate u32 RockyTerrainMeshVertCountView. It must read the same canonical count owner and narrow the actual count for W5.
- ssbo_tags.rs:218/225 hardcodes original uppercase SSBO full type names. Current library registrations retain existing Rust `...Ssbo` names and enforce full type identity; using only the matching short-name hash is insufficient.

These are current, observed unintegrated interfaces, not permission to overwrite their owners. This review changed none of them. Later owner changes should be reviewed as deltas to the captured sources.

## Additional current full72 notification lifetime gap

The actual mapping and producer are correctly wired, but the current async handoff does not retain the one-shot Created notification through arbitrary preparation duration. run_game.rs:818 updates the gameplay SubApp repeatedly until LoadingGameComplete. The producer sends Created only once in PreStartup. GameLoop consumes it only after gameplay is handed back (`game_loop.rs:495–505`); a successful handoff merely resets the EventCursor at575/686 and does not rebind the current canonical buffer or retain its original notification.

Pinned Bevy0.16.0 App::default installs event maintenance in First; Events::update swaps buffers and clears the oldest. EventRegistry performs the second cleanup even if there was no new write. Thus **if gameplay preparation runs two event-maintenance cycles after Created publication before returning**, the new frontend EventCursor cannot recover that event. TimePlugin gates maintenance on FixedPostUpdate signals; the precise trigger is two maintenance cycles, **not a fixed number of SubApp::update calls**. This is a source-proven conditional host lifecycle gap, not original B behavioral execution or an executed counterexample. Its owner needs to preserve the actual notification/lease across handoff or reattach that same canonical buffer through the current API; do not allocate a duplicate cursor buffer, fabricate zero contents or rerun full72 into an already populated library.

## Root compilation delta and bounded acceptance

Root rocky-r1 failed on E0283 in new test assertions, then changed exactly two rhs calls to `bytemuck::cast_slice::<_, u8>` in the actual-asset test. Independent full-file comparison confirms **no other delta** from frozen147; all bytes before cfg(test), assertions, native expected words and input fixtures are identical. This resolves target-type inference and does not mask production behavior.

Root rocky-r2 logs/results were read directly: command `cargo test -p country-core --lib --features native-physics --locked --offline resources::rocky_terrain::tests`; exit0, five named tests passed, zero failed/ignored, 87.955 seconds total, 5058 tracked files and source_drift[]. This includes the actual30,998,513B asset/library regression, signed JSON overflow, raw pack, None descriptors and invalid input boundaries. No Rust execution was initiated by this reviewer. Root's assessment explicitly excludes full Startup, complete startup_terrain, W4/W5 and GPU completion.

At root's request, resource claim TG-7e388fa8b16d3b9fc2 was released only for that bounded accepted closure; claim-history `TG-7e388fa8b16d3b9fc2-20261018T132039331471Z.json` records those limits. Frozen147 remains unchanged.
