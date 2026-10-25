# Loading Startup + canonical MEI owner candidate

Status: candidate only. No production files changed. No Cargo or GPU run.
The three changed Rust bodies/new producer parse with rustfmt --emit stdout;
that is a syntax check, not type checking or original/runtime verification.
manifest.json records the exact six production base hashes at delivery.

## Original facts

See ../loading-startup-proof/LOADING-CANONICAL-TEXT-CONTRACT.md and its native
manifest. MinimalStartupPlugin is directly identified through the loading
seven-plugin tuple's vtable. build@140b1ac90 -> add_systems<Startup>@140b65f80
-> five-tuple@1413fb240 -> config@1408ae5b0 -> run_unsafe@1408ac4a0.
Business @1408ac544 registers TextPrimSsbo; @1408ac553 registers ShapeStreamSsbo.
A separate full548B run_without_applying_deferred@1414f8ff0 has the same calls.
Both are add_new(None), without initial upload. This is the two-entry minimal
subset, not gameplay's full72. Original other tuple members include color,
load_ui_shaders, startup_blit and LoadOnlyBootupTextures. Tuple registration
order does not prove a configured execution chain.

Original flush_text_render@141e9e1f0 reads both canonical library entries;
Update44 flush runs after actual MEI/text producers. It does not own private
replacement SSBOs. Existing actual RenderGraph session and managed ownership
remain the only backend/command resource chain.

## Patches, in integration order

1. core-minimal-startup.patch: add an existing-startup-module producer using
   the actual canonical typed add_new calls. A separate ECS wrapper reads the
   actual GraphicsDeviceResource / AssetSsboLibrary. The original function was
   inlined in resources; this source file organization is an adapter, not a
   fabricated new original native function. startup/mod.rs adds one export.
2. mei-canonical-library-owner.patch: from_library resolves the real entries
   through the same BufferHost; validates actual device/generation/size/usage;
   holds physical ManagedBuffer leases. Library consumers do not release the
   authoritative slots on drop. Explicit private new remains available for
   deliberately standalone callers; there is no missing-library fallback.
3. loading-startup-adapter.patch: split the unchanged asset/font initializer
   from canonical host attachment. install_loading_startup registers asset
   initialization and the two-entry producer in Startup without an overall
   chain. Only attach.after(minimal_startup_ssbo_system) is imposed, an explicit
   host data dependency. The same HostTables clones hold the same actual Arcs;
   no second device/library/cache/session is created.
4. game-loop-loading-startup.patch: replace the immediate loading initializer
   call with Startup registration after the actual library installation.
   Asset and canonical MEI resources are ready before the first Update44.
5. loading-canonical-startup-gpu.patch: update the existing actual loading GPU
   fixture to call the same install_loading_startup on an actual App; preserve
   its original App World/Schedules while installing inputs. Update44 asserts
   the real typed entries and attached host. Existing actual glyph+Logo+
   Vermeer execution/readback remains; graph resource ids must equal imports
   of those canonical owners, and host drop must leave library slots live.
6. loading-present-canonical-producer.patch: the existing WSI fixture is an
   explicitly supplied World, so it directly runs the same pure two-entry
   producer before the still-callable initialization helper. Existing actual
   swapchain production render/present/shutdown assertions are unchanged.

The previous ../canonical-mei-loading-gpu.patch is superseded: it temporarily
created a private host then replaced it in the fixture. The new fixture enters
through actual Startup and never makes those private buffers.

## Failure/lifecycle contract and remaining scope

- install_loading_startup validates actual device/library installation and
  rejects preexisting text entries instead of overwriting or running minimal
  again into a full gameplay library. It does not manufacture successful state.
- The producer preserves actual typed factory/library failure behavior. Fatal
  Startup callbacks currently use explicit expect messages, consistent with
  existing logo startup; such a failure aborts Startup before Update/render.
  The directly callable initialize_loading_resources still returns Result.
- That helper requires the caller to supply canonical entries first; it never
  invokes minimal/full producers implicitly and has no private fallback.
- Assets and attachment are independent of each other: text CPU initialization
  completes during Startup and attachment only needs its real SSBO owners.
  No glyph is consumed during registration/attachment; consume remains once
  per genuine Update44, outside graph retries.
- Gameplay remains bound to its full72 producer and must use from_library when
  its MEI path is installed. This patch changes only the present loading path;
  it does not claim that gameplay's entire UI, world graph or async ready gate
  has been restored.
- The preexisting asset loading body remains byte-for-byte, including its
  should_load(name,demo) call. This narrow change does not pretend to newly
  verify that old predicate against LoadOnlyBootupTextures or restore all five
  plugin callbacks as native ECS systems. Font/atlas host adapter limits in
  ../CONTRACT.md remain explicit.
- CPU contract test in the new module proves only canonical constructor order,
  exact descriptors, None payload and exactly two library owners. Actual
  source type checking and the amended GPU/WSI fixtures are pending root runs.

## Ownership/integration

All output is confined to our existing TG-0ef1874c29a03d0dec/main-chain-frame
candidate work directory. No claims were taken from the external core systems/
resources session. The new startup module is a source organization seam for
original inlined resources behavior. Root integrator must apply the shared
startup export and GameLoop/fixture changes after owner coordination; the
parent explicitly authorized this candidate delivery. Do not copy whole
snapshots over newer source; use the narrow patches and inspect base hashes.
