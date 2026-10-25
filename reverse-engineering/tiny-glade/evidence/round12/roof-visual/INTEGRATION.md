# Roof production integration follow-up

The seven candidates in REPORT.md are now applied to production, each checked
against its exact original base SHA and proposed SHA before mutation. See
applied-manifest.json. The report's original "production not modified" statement
is historical to the candidate delivery. This follow-up does not overwrite that
evidence. Latest source bytes are bound by production-source-manifest.json.

`roof_trigger_volume.rs` now transcribes the complete original triangle, UV and
normal producer. It uses the canonical event's actual entity, fails on a missing
query row, restores both shapes and all flat/pitched/gable/axis/cap branches,
and transfers the resulting vectors in original triangles -> UVs -> normals
order. No PublicWalls re-resolution or fake trigger mesh remains in that system.

Independent oracle: native_trigger_oracle.py executes the unchanged original
1421a3490..1421a5a90 geometry/noise range after the ECS lookup. It maps the full
RNE and reuses its actual helpers. Only allocator/free/realloc, memcpy/move/set
and exact imported ucrt f32 math are bridged; it never launches the game. The
body is not replaced with a Python geometry model. Native fixtures cover 16
full shape combinations and 12 extra threshold/profile/floor/rotation/tip cases.
Original import floorf was initially missing, causing an unmapped fetch. Adding
the genuine import bridge corrected the oracle; there was no game-byte patch.
The original B whole-file SHA is checked before every oracle run.

Subtle original facts preserved: rectangle bottom AND top each receive +.4;
circle bottom AND top each receive +.13; rectangle tip-center call passes two
zero scale arguments; circular ring translation uses tip_offset * profileU;
circle top cap remains on the original center. Rectangle cap is emitted after
every accepted side (four hip / two gable), with UV V2.0/2.1. Do not deduplicate
these triangles: first UV match is observable. Normals precede vertex noise.

Native counts: circle flat135/pitched285; rectangle flat160/hip pitched320/gable
pitched160. These facts are independently generated from original instructions.
A production Rust test compares every position/UV/normal against the frozen
16-case binary with <=2e-5 absolute tolerance and prints unequal bit patterns.
It has not yet been run by this agent; the central owner controls Cargo. The
extra12 original fixtures are retained separately until first comparison passes.
The numerical tolerance is not a claim of f32 bit equivalence.

`roof-registration.integration.rs` provides the original set20 exact nested
chain/chain_ignore_deferred tuple, deletion21, unchained visual29, history34,
replay40 and flush44 for central integration. Most production CPU roof systems
were previously misplaced together in40; merely adding29/44 is insufficient.
The full production installer and shared history_runtime are owned by root.

Still pending: central Rust comparison/compile; real wall -> arches -> create /
edit / delete / undo -> Sparse GPU readback -> DrawLists -> physical rendering
and Nani ownership release; snapshot's other roof spawn site; UI17/18 actual
widgets/history/drag interactions (current files still openly contain no-op
skeletons). Generic old batching vs callback-on-generation panic side effects
remain unverified for invalid geometry. No ordinary input branch is silently
skipped to hide these gaps. T13 is excluded by user instruction.

RoofSupportBeam is independently investigated by roof_support_producer; read
../roof-support/REPORT.md. Its compiled B plugin has no fourth visual producer,
so do not invent a nonempty generator from wall lintels just to meet a presumed
acceptance condition. Keep the original dense buffer and draw consumer intact.
