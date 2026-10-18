# Original loading App: canonical text storage producer

## Identity and bounded original evidence

This is B original native evidence, not source-progress statistics. Oracle and
complete PDB-bounded native ranges are in manifest.json. No Ghidra database was
opened or changed. Current production source and RG ownership remain in place.

The loading seven-plugin tuple @1400cd450 is retained in sibling boot work:
TG-c459c7f73345e24709/main-chain-boot-20261018/loading-plugin-tuple-rne.asm.
Its first plugin vtable @142937738 is loaded @1400cd478 and passed to
App::add_boxed_plugin @1400cd48c. The vtable type_id @+18 and name @+60 are
MinimalStartupPlugin; build @+40 is 140b1ac90. Thus the following registration
is specifically the independent loading App's plugin, not an inferred gameplay
callback with a similar parameter signature.

## Exact schedule and registered callbacks

MinimalStartupPlugin::build @140b1ac90 (8 B) forwards its App argument to
SubApp::add_systems @140b65f80 (316 B). The complete PDB instance names
bevy_app::main_schedule::Startup. Native @140b6600a invokes the five-entry
into_configs @1413fb240 (883 B). The original tuple contains:

| Entry | Config factory | Exact original stored name |
| --- | --- | --- |
| 0 | 1413fb780 | country_core::startup::init_color_library::init_color_library |
| 1 | 14179f4e0 | country_core::startup::load_shaders::load_ui_shaders |
| 2 | 14165b110 | country_core::startup::startup_blit::startup_blit |
| 3 | 140a2c790 | country_core::startup::load_textures::load_textures<country_core::startup::load_textures::LoadOnlyBootupTextures> |
| 4 | 1408ae5b0 | country_core::resources::init_populate_minimal_startup_ssbo_library |

The five names are exact pointer+length slices recovered from the native
factory instructions; names, vtables and their PDB instances are retained in
minimal-factory-identities.json. Registration order in this table is tuple
order. There is no chain/configured order between these independent systems
in this evidence, so this document does not invent one.

## Minimal SSBO producer is exactly two library entries

The fifth config's System vtable @142a9ca10 points to:

- run_unsafe @1408ac4a0, full 393 B. Its only successful business calls are
  @1408ac544 -> AssetSsboLibrary::add_new<TextPrimSsbo> @1412c2b50, then
  @1408ac553 -> add_new<ShapeStreamSsbo> @1412be4f0.
- run_without_applying_deferred @1414f8ff0, full 548 B, independently contains
  the same two calls @1414f90d6 / @1414f90e5.
- System::run @141501460 forwards to that latter wrapper.

All remaining calls in these full wrappers are ECS/assert/expect failure paths.
Both add_new invocations receive the same resolved AssetSsboLibrary and same
GraphicsDeviceResource payload. This is the loading App's own library; the
separate App libraries share the physical device, not SSBO ownership.

The producer invokes add_new, so both constructors receive None, with **no
initial upload**. Their native constructors and current restored canonical
contracts are already provided by the UI constructor patch:

| canonical element | owner wrapper | capacity | bytes | native descriptor |
| --- | --- | --- | --- | --- |
| TextPrimSsbo(tg_text::Primitive) | TypedStorageBuffer (40 B) | 131072 x 48 | 0x600000 | usage 0x22, memory 1, alignment 4 |
| ShapeStreamSsbo(u32) | TypedStorageBuffer (40 B) | 1048576 x 4 | 0x400000 | usage 0x22, memory 1, alignment 4 |

This loading subset is **not** init_populate_ssbo_library's full 72 entries.
It must not invoke the full gameplay producer or manufacture missing entries.

## Consumer and source integration boundary

Original MeiPlugin is also present in the same loading plugin tuple. Existing
MEI evidence confirms Update set44 runs (mei_post_update, flush_text_render)
with chain semantics and after render_text_fragments. flush_text_render
@141e9e1f0 reads these same two typed library entries, then consume callback
@140fac810 uploads the actual primitive/shape streams. Startup executes before
that Update consumer. Full text startup/sizing/layout registration remains a
separate producer contract in TG-c80526daecad7e4117/meihost-20261018/CONTRACT.md.

Current loading_graph::initialize_loading_resources still creates MeiHost::new
with two private allocations before App::update. That explicit host adapter
was sufficient for earlier loading-only fixture GPU tests; it does not match
this recovered canonical library ownership. The reviewed from_library candidate
must only attach to the actual same-App TextPrim/ShapeStream entries after the
real minimal Startup producer. Missing or stale entries are an error for both
loading and gameplay; never fallback to private new on missing library data.

A faithful narrow integration has these boundaries:

1. Once the actual same-device libraries exist, register the two-add_new
   minimal callback in loading Startup (or restore the canonical resources
   function through its existing owner). It should receive the actual
   GraphicsDeviceResource / AssetSsboLibrary, not test-only globals.
2. Split the current CPU/font/texture preparation from canonical MeiHost
   attachment as necessary. Attach the host after the actual producer has run
   and before the first Update text flush; do not access a nonexistent library
   entry at GameLoop construction time. The host-only attachment ordering is an
   explicit source adapter, not a claim of original tuple chain order.
3. Gameplay uses its existing full72 producer's same two entries, with no
   private fallback or independent second SSBO cache. Do not call minimal again
   into a library whose full72 producer has already installed these entries.
4. For either App, real library-owned ManagedBuffer leases survive command
   recording/submission, while dropping MeiHost does not invalidate the shared
   BufferHost slots. Existing candidate does not change consume/upload/atlas
   business algorithms.

Only bounded original native evidence was added here. No production files were
changed, no Cargo/GPU test was run, and no gameplay-ready/present claim follows
from this investigation or from the candidate test's explicit registrations.
