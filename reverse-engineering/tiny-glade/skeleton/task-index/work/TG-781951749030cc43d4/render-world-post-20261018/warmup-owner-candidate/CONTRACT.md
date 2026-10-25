# Same-App background GPU warmup and main-thread history adoption

Candidate only, current architecture host adaptation; not an original ABI field or a new rhapsody implementation. No production edits/Cargo/GPU by this worker. Root owns full-world graph restoration and integration. The adapter accepts an actual fallible full graph builder and never substitutes only the prelude as a ready game.

## Current actual owner chain

GameLoop::new installs gameplay libraries from the original renderer bridge and captures that bridge's HostTables in original Startup producers. Its World contains buffer, texture, VAO and shader handles belonging to those exact tables. A new CoreRenderBridge::new(device,root,sameShaderLibrary) creates new BufferHost/TextureHost/ShaderHost tables and a new heap, so equal numeric handles would refer to different owners even on the same physical device. That is invalid warmup wiring.

On the normal loading switch, activate_application already saves the original gameplay ApplicationGpu in other_applications under its unchanged Arc<ApplicationShaderLibrary> pointer. The loading App has a separate Arc and independent tables. Returning the original SubApp therefore normally hits the saved gameplay bridge. The new guard rejects an actual cache miss when the World already owns AssetLibraryRuntime; it does not create a third namespace as fallback. New uninstalled Apps can still obtain their first real ApplicationGpu.

CoreRenderOwners privately snapshots only the existing same-App HostTables, Arc<RasterShared> (including the same DynamicConstants and VAO attachment/cache owners) and actual shader library. from_owners constructs a thread-local current VulkanGraphSession using the existing VulkanGraphBackend implementation. It does not recreate handle tables, heaps or asset libraries. CoreRenderBridge's Rc is not sent; no unsafe Send is added.

## Why owner snapshot alone is insufficient

A new worker-local session has its own existing named-resource cache. Dropping that cache after warmup would discard actual history contents/parity and make the main game's next frame incorrectly start fresh. Moving the whole VulkanGraphSession is not sound: it includes non-Send retirement callbacks in pending frame state. This candidate transfers only completed named data through a sealed, automatically Send-checked payload.

CompletedNamedResources contains the existing NamedResources value: its temporal state, parity and confirmed access/history-valid flags, actual GPU Arc owners and specifications. It also carries the actual native queue family for each retained physical token. It excludes callbacks, command pools, the Rc session, resource pools and optional result-cache bookkeeping. Those are not original gameplay history values.

`take_completed_named_resources(timeout)` requires a fully submitted, nonpartial completed frame with real submissions, performs the existing actual GPU wait, then rejects pending imports/work, device loss or uncommitted named requests. Only then does it detach that exact named cache. All resources remain owned by the payload, and the source session's normal cleanup can drop its independent references after GPU completion.

`adopt_completed_named_resources` accepts only the same exact device, an idle healthy destination and an empty named/history cache with no uncommitted graph. It validates each physical owner's device and restores its queue-family mapping and existing named data; no GPU content is copied or treated as newly initialized. It never overwrites/merges two histories. The next normal begin_frame rebinds the cache to the new graph ID and the current original temporal algorithm advances its preserved parity.

Tiny's CompletedGameplayWarmup uses Mutex<Option<CompletedNamedResources>> solely for a single actual ownership move through an immutable World reference. It is sent with the actual SubApp. `accept_gameplay_warmup` first requires Arc identity equality for buffer, texture, shader, raster and shader-library owners, then adopts the completed cache once. Repeated App activation sees the already consumed option and preserves current state. There is no ready boolean whose value can replace completion.

## Actual full-builder adapter and required caller changes

`warm_up_gameplay_with_world_builder(owners,world,build_world)` builds and sorts real DrawLists from that actual World, refreshes actual VAOs, and invokes the supplied typed Result builder on one CoreGraphFrame. The callback must be the recovered **complete compile_main graph**, including its real final22 export; a prefix or W47..54 segment is insufficient. The adapter prewarms the same ShaderHost, compiles and executes the graph, verifies its actual exported physical texture, waits for completion through the transfer API, releases the final export token and inserts the real completion payload into that World. An error never publishes a completion object.

Root must make these concrete two call-site changes when its full builder is available:

1. Before moving the gameplay SubApp in GameLoop::new, obtain `renderer.bridge().warmup_owners()?` and capture that value in the existing worker closure. Keep the original gameplay ApplicationGpu in the current per-App cache as today.
2. Production `warm_up_gameplay` receives those owners and calls warm_up_gameplay_with_world_builder using the same full typed builder as ProductionRenderHost::compile_world_graph, with no swapchain acquire/present. The original loader sends the SubApp only after this returns Ok, as its existing LoadingGameComplete gate already specifies. The supplied compile_main builder still runs its original outer final-post/Vermeer/Mei/UI behavior; it must not stop at WorldPostOutputs. activate_application's candidate adopts the completed cache after the original bridge cache hit.

The adapter intentionally does not implement or declare a missing full world builder. Root remains responsible for that concrete business closure. The current rendering entry must retain an explicit error until its required producers/consumers actually exist.

## GPU regression

`completed_background_warmup_preserves_actual_pair_owner_parity_and_pixels` reuses the existing actual original TAA fixture, not a new renderer or fabricated image. Its parameterized test helper keeps all previous shader/uniform/history/binding/submit/pixel assertions. A real background OS thread receives the actual owners and World, constructs its local existing bridge, renders frame0 with packaged TAA, completes actual GPU work and returns the actual World plus completion payload. The main thread retains the original bridge, adopts the payload, renders frame1 and must see valid history. `Arc::ptr_eq` must prove main frame1.previous is exactly worker frame0.current, in addition to actual output pixels. A repeated-activation check confirms the one ownership move is idempotent. Rust compilation of the thread spawn and Resource wrapper enforces Send without any unsafe exception.

Root must compile and run this fixture under the strict real GPU environment. Source syntax and patch applicability alone do not prove its Send constraints, GPU queue ownership or runtime behavior. `candidate-manifest.json` records actual source bases and those limited checks; it makes no full-game or original-version equivalence claim.
