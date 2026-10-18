# Review handoff: loading Startup canonical MEI

Candidate delivery only; no production edits, no Cargo/GPU execution. All six
source base hashes still match at final handoff. Four relevant Rust candidates
parsed successfully with rustfmt; this does not establish compilation.

Read CONTRACT.md first. Original native source facts, parameter/ownership
boundaries and exact stage evidence are in
../loading-startup-proof/LOADING-CANONICAL-TEXT-CONTRACT.md and manifest.json.

| Narrow patch | SHA256 |
| --- | --- |
| `core-minimal-startup.patch` | `094cc18fa2bc2e1be8672ac61c1c69d552749a486d0088a2e889ec36ae736944` |
| `game-loop-loading-startup.patch` | `932fdcf49a77295e1ce7e7a37409a2d67f0e291632467b28d983160937670360` |
| `loading-canonical-startup-gpu.patch` | `8d16769b712781b26fcb2feeac9b3f4f3c40ff7b034ff247b27a1dcee0be6e6e` |
| `loading-present-canonical-producer.patch` | `4367841ed6cca5dd350289c4f958e09bff3bc7a03eecaa088ca96b06c7fd2442` |
| `loading-startup-adapter.patch` | `44bc019571cba910a97939ae658a24a75097cfe7ba47695d7081b439b52fd95d` |
| `mei-canonical-library-owner.patch` | `1359e0b553f796d376cf953a3dafeaa9b9a7e9215d1173a7c73cd95de9496725` |

Apply in the sequence documented in CONTRACT.md: core producer/export, MEI
owner, loading adapter, GameLoop callsite, actual loading GPU fixture, WSI
fixture callsite. Old canonical-mei-loading-gpu.patch is superseded; do not
apply that private-host-then-replace test candidate.

Original gameplay/core resources ownership is not taken over. The new startup
file is an organization adapter for original inlined minimal resources logic.
Root is the authorized shared-source integrator and handles owner coordination.
The original five Startup callbacks are not artificially chained: only
canonical two-buffer producer -> MEI attachment has a new explicit host data
dependency. Initialization errors stay fatal before Update, with no implicit
buffer creation in flush and no gameplay missing-library fallback.

Pending validation targets for the integrator:

- core unit `minimal_loading_startup_creates_only_two_ordered_canonical_owners_without_upload`
- amended `original_loading_gpu` target: actual App Startup + Update44 readiness,
  real glyphs/Logo/Vermeer/readback, canonical graph-id identity and live slots
- amended `original_loading_present`: same minimal pure producer followed by
  callable helper, existing actual WSI presents/shutdown
- normal GameLoop construction and first real Startup; this candidate makes
  no complete gameplay/world consumer or async-ready assertion.

The existing loading asset texture filter/font/atlas adapter behavior is
preserved and has not been newly verified as native five-system equivalence.
