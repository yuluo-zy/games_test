# Original world lighting: candidate integration contract

This delivery targets B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. All files are candidates, not production or compiled/GPU-verified results. Original consumer semantics run through the existing refactored rhapsody frame/session/ownership architecture.

## Callable boundary

`begin_depth_history(frame, depth)` declares the original `reprojection.prev_depth` before W1, using the real depth description with format100/usage12. `build_world_lighting(frame, world, lists, WorldGeometryOutputs, ColorHistoryMode)` connects the continuous W35..W43 segment. It must execute inside the single existing `CoreGraphFrame::with_main_prelude` scope, after real W1..W34. It does not install a second FrameSetup, create a separate session, synthesize geometry, initialize history contents, compile/submit/present, or publish the gameplay App.

The typed geometry input is `{ GBuffer{gbuffer,depth}, velocity, color, previous_depth:TemporalTexture }`. Output keeps this input plus `{ half:{depth,normals}, reprojection, ssgi, history:{current,previous,valid}, RestirTextures{radiance,ray_dir,reservoir}, gi_radiance, LightBinning{grid,indices}, direct_lights, shadows }` for W44..W54. Every ID is returned by the same frame's actual RgResources.

The canonical `DeferredRasterResources` per-App owner is required. Used material handles must be Some and resolve in the current App shader library before any new lighting declarations. No lookup by filename substitutes for the real original startup producer. `startup_deferred_raster@140a97a30` registers four shaders in declaration order1,3,0,2; slot4 is not written. Its candidate and producer evidence are in TG-5e434a71f2d6ff903b/render-nani-main-20261018. The exact original stage is gameplay Startup's eight-item tuple, established by its typed System vtable → into_configs → SubApp::add_systems<Startup> → add_game_startup_systems call chain. It is not in PreStartup10.

## Original value flow

Full `render_world@1404ec110` 4314-byte instructions are retained as render-world-full-original.asm. This table follows actual argument registers and caller stack, not old variable names.

| Original call | Business entry | Real value/result |
|---|---|---|
| 1404ec260 | temporal previous depth | original depth.desc format100/usage12; flag0=new, flag1=history valid |
| 1404ec660 | half-res | GBuffer pair → half depth + half normals |
| 1404ec6d9 | reprojection | full depth + velocity + earlier previous depth → map + copy-depth side effect |
| 1404ec708 | SSGI | half pair + GBuffer + reprojection → SSGI texture |
| 1404ec84a | color history pair | format122/sample/storage; TAA=color extent, TGSR=actual display extent |
| 1404ec986 | RT GI | GBuffer + half pair + prev color + valid + reprojection + SSGI → three textures |
| 1404ec9ac | evaluate GI radiance | half pair + SSGI + three RT GI textures + reprojection |
| 1404ec9c1 | bin lights | canonical Lantern data → grid and indices buffers |
| 1404ec9e5 | direct lights | half pair + reprojection + both light buffers |
| 1404eca03 | screen shadows | GBuffer pair + reprojection |
| 1404eca67 | deferred raster | exact typed mapping below |

Deferred original incoming r8 points at `{gbuffer,depth,velocity}` at rbp240; incoming r9 points at the original scene color. Inside deferred: 1409d2216..2298 constructs color tag0 from r9 and read-only depth tag1 from r8+24, with viewport from that depth. Current source had those attachments swapped. `deferred-attachments.patch` fixes that independently; it also updates the existing mock regression to distinguish real texture IDs, access intent and viewport. The original U_DEPTH_DIMS binding still reads the incoming color description at1409d2a83; the narrow patch deliberately keeps that actual expression.

Deferred caller mapping: argument3 pair={gbuffer,depth}; argument4=color; argument5=previous color history; argument6=history valid; argument7=SSGI; argument8=half depth; argument9=three RT GI textures; argument10=GI radiance; argument11=screen shadow; argument12=direct-light reservoir; argument13={light grid,light indices}. The historical core `DeferredInputs` fields depth/gbuffer/rt_buffers have misleading names; this compatibility mapping is confined to the final local constructor. The public new stage inputs/results use semantic names.

## Core FramePasses type correction

`typed-render-world.patch` replaces raw arrays with current existing canonical types and updates the sole CPU recorder caller. Single pass results use RgTexture; HZB uses Hzb; light bins use LightBinning; RT GI uses RestirTextures; DoF preparation uses DofPrepared. No unsafe byte casting or new opaque native ABI is introduced.

Original dof_prepare at1404ecc26 produces72B, passed intact to calc_dof at1404ecd09. Original final output at1404ed01e..03d copies only the first36B (CoC) at output+24; the other two returned textures are depth at+0 and TAA/TGSR at+48. Thus the candidate returns `dof_prepare.coc`, preserving all internal outputs without incorrectly expanding the original outer result to four textures. The recorder checks the three distinct RT textures, two distinct light buffers and both DoF inputs before their consumers. Canonical graph handles have no fabricated Default, so the unused default derive is removed from RenderWorldOutputs.

## Remaining integration and actual validation

Root handles W2..W34 ingress and production entry integration; this candidate does not pretend those prerequisites exist. Real SSBO/texture producers, loaded mesh atlas, original DrawLists and frame constants remain mandatory. W44..W54 and compile_main's final-post/UI/cache branches still require separate exact consumers.

Only first-frame reprojection has current binding-only metadata backed by original shader CFG and actual GPU evidence. SSGI/RTGI/radiance/DI/shadow history first-frame branches have not been validated by this new continuous graph. If the strict graph rejects an inactive history, collect its actual shader branch and failure, then use the existing narrow contents-unused capability. No fake clear or global uninitialized-read exemption is authorized. Light histogram/scan partial writes may need original-preserving metadata through the now-existing current API; do not infer all raw37 accesses are read-modify-write.

Existing root-run v37 results do validate the true Nani master with real one-instance atlas/indirect/indirection/visibility readback, six live raster uploads over two frames and original first-frame reprojection+depth-copy. They do not validate this complete lighting segment, real scene imagery, or gameplay-ready publication. The actual production startup still reports the world consumer gap. No Cargo was run for this delivery.
