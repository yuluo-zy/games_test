# Full first-person footsteps candidate

Source is evidence-only under `proposed/`; production was not modified. No
Cargo/rustc/LLVM code generation was started. rustfmt parsing succeeded.
T13 remains excluded. No rhapsody/shared utility implementation was replaced.

## Observed original behavior

The complete PDB-sized firstperson_steps is140a889b0/1509B, original B SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The original PDB generic fixes **eight** inputs, in order:
AudioCore, MainCamera, AssetSsboLibrary, PathRastersResources,
FirstPersonViewAudioState, WaterRaster, GardenRaster, ActionSetStates.
The earlier six-parameter function and optional PlayerMoving have no original
contract or real movement producer and are removed by the candidate.

`native_oracle.py` executes the complete unchanged1509B, actual eye-position
helper and actual835B RasterBufferRef sampler. Its203 cases retain parameter
calls/order/float bits, raster sample order and loop start/stop request. Cases
cover all six camera modes, all five digital phases, zero/nonzero/NaN analog
pan, vertical modifier, both grid feature branches, six material outcomes,
scratch selection, nonterrain collider source and disabled Studio.

Only shared FMOD-facing parameter/AudioLoop calls, standard FrozenMap lookup,
typed mapped-view getter and CRT memcmp are bridged. No firstperson state
branch or sample result is substituted. The actual FMOD playback and AudioLoop
internals are boundaries here, rather than claimed executed. The original
sample code uses fixture-owned current/scratch buffers and mapped rock bits.

The native function proves these corrections to old source/comments:

1. FirstPerson is published every frame from actual camera mode3, independently
   of whether this frame is moving. The source byte is camera+4d0, not a ground
   material value.
2. Active walking requires mode3 and digital3 phase<3 OR analog2 length>0;
   analog length uses sqrt(y*y+x*x). No fabricated PlayerMoving resource.
3. GroundType is published **only** in the active-walking branch. On inactive
   frames the last FMOD value is retained, and the shared loop gets a stop
   request. The old function always published it and could start outside Walk.
4. Each active branch starts with material2. Only canonical source tag11 at
   camera+328 takes terrain sampling; other source tags publish2.
5. The typed mapped rock SSBO is resolved and its name verified before water
   sampling, even when water then short-circuits. Missing mapping is fatal.
6. Water>.1 produces3. Else the four distinct paths are sampled in original
   path/cobble1/cobble2/cobble3 order; sum is (((p0+p1)+p2)+p3), then SSE-style
   clamp0..1. A positive aggregate repeats the first path sample: p0>.1 gives1,
   otherwise5. These are four rasters, not four neighboring water/garden cells.
7. A nonpositive path aggregate takes the original rock bit. Set bit produces2
   and skips Garden; clear bit samples Garden, where **>.4** gives4, otherwise0.
   Previous comments incorrectly called the shared .4 constant .1.
8. Water/path/garden NaN comparisons preserve original unordered branch policy.
   Current/scratch selection comes from each canonical WorldRaster owner.
9. After active material publication, modifier byte+344==0 requests loop start;
   a set modifier requests stop. All inactive branches request stop.

The candidate reuses `TerrainRockMaskSsbo::sample_mapped_at_world_xz` from the
newly integrated canonical mapped adapter. It creates no mask copy or shadow
resource and does not bypass missing mappings with zeroes. Raster math remains
in the current utils crate. Existing AudioCore and AudioLoop own the actual
Studio and instances.

## Walking-loop initialization

Both original Walking literal callers are1414bc857 and1417e1a3e. The latter
sets descriptor count3 and delay50 before140a59220. Native instantiate_loop
increments the descriptor count and collects the actual original iterator;
it writes delay to AudioLoop+30, rather than treating50 as a soft recycling cap.

`native_init.py` executes that original constructor plus original collect/next
functions, bridging only allocation, Vec growth and CRT memory. Four disabled
Studio cases prove descriptor counts0/1/3/7 produce1/2/4/8 owned instances and
all retain50ms. The count+1 does not skip a first iterator entry. Native+59359
gets uses_loop_parameter from the first event's actual `loop` parameter probe;
started and explicit-stop flags are zero.

The candidate therefore creates four actual walking instances, probes the
existing backend for `loop`, then constructs the current shared AudioLoop with
50ms, detected parameter and explicit-stop=false. An instance creation error
panics as the original init caller's collected Result unwrap does; preceding
owned instances release during unwinding. A failed parameter query is logged
and treated as absent, following the nonfatal native probe path. This is a
bounded Walking initialization correction. The same shared AudioCore::
instantiate_loop count/threshold/probe discrepancy affects other callers and
was reported centrally; this candidate does not silently rewrite all callers.

The invented per-state cached GroundType is removed. The native resource owns
the loop, while FMOD owns the last published global material value.

## Candidate verification and integration

`proposed/tiny-glade/crates/country-core/src/systems/audio/firstperson_steps.rs`
contains four meaningful **unrun** Rust tests:

* All203 original fixture cases run the actual eight-parameter ECS function
  using six nonempty canonical raster owners, the actual mapped SSBO library,
  canonical actions, camera and a recording backend at the FMOD boundary.
* Native material sample order checks lazy branches and the repeated first
  path read independently of publication assertions.
* Original four-instance/50ms/dynamic-parameter initialization contract.
* Creation failure releases both preceding instances and builds no dummy loop.

The fixture must be copied with the source; its proposed path is
country-core/tests/fixtures/original-firstperson-b.json. `delivery.json` binds
exact production base bytes and both proposed files. Central integration may
apply only this file pair after SHA validation. The current audio group already
registers init->firstperson_steps, so no new event or substitute input feeder
is required. The eight original mandatory resources must exist when it runs.

Run only in the central compile window:

```
cargo test -p country-core firstperson_steps::original_tests --lib --locked --offline
```

No Rust result, actual audible playback, whole startup or camera-mode entrance
completion is claimed by this evidence-only candidate. A separate reviewer was
asked to independently check the original loop initialization interpretation.
