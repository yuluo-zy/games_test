# Core wall authority, first implementation batch

Date: 2026-10-14. User authorization: implement the non-rendering core plan, with
all business code inside `tiny-glade/crates/country-core`. Target: B only. Task
queue families include C metadata; this delivery does not claim C behavior.

## Result and source contract

The formal core now exports `resources::walls::{WallState, InnerWalls}` and the
core ECS consumer `systems::wall::wall_state::publish_public_wall_state::publish_wall_sources`.
The consumer reads actual `InnerWalls` and `WallPublicationTerrain` resources,
builds geometry through the existing `collect_raw_wall_publication`, and builds
regular heights through the existing `regular_wall_height_cache`. Its output is
`WallPublicationState`, a bounded CPU resource, not the entire native PublicWalls.

Source ownership, ID generation, explicit-ID insertion, edits, new/changed/deleted
sets and removal are real core code. Caller-created World/Schedule tests consume
them without tg-building-runtime. No upstream runtime code, manifest, lock,
frozen reconstruction or original binary was changed. Existing recovered geometry
and history-projection helpers were reused without changing their source hashes.

The state owns its curve and uses the retained source/history projection. The
projection omits source +0x66, so restoration requires a separate freehand-type
byte; `into_snapshot` returns both. No native Vec/map headers or enum names are
fabricated. Complete serde/compatibility and the old projection's entire native
layout are not accepted by this delivery.

## Original facts and corrections

Original instruction artifact: `ljxsj_92385/Tiny Glade/tiny-glade.rne`, SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The retained B PDB identity is `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`.

- `0x14085a370`: increments +0xc0 and uses the resulting ID for map insertion;
  initial +0xc0 is zero in `0x140859f90`. This is the ID counter, not an edit
  generation. The old wall DIGEST/REVIEW interpretation is superseded in this scope.
- `0x14085a550`: writes zero only to +0xc0; it does not clear wall rows/notices.
- `0x14085a4a0`: explicit-ID insertion does not change +0xc0. A displaced old
  row is dropped, then the original panics `Duplicate wall id: <id>`; the new row
  survives and no created notice is emitted. Proven statically by original bytes.
- `0x14085aa60`: checks existence before the removed-ID notice, then removes.
  Missing lookup panics `Remove failed: <id> doesnt exist`.
- Source stores: height +0x54, elevation +0x58, arch height +0x5c, plaster peel
  f32 +0x60, arch type byte +0x64, clutter version byte +0x65, **freehand type byte
  +0x66**, removed supports byte +0x67, style byte +0x68, demo-dependent byte +0x69.
  The old `freehand_ty @+0x60` table is wrong. The retained history metadata word
  at +0x60 is preserved as bits; it must not be modified to toggle freehand type.
- Updates notify +0x60 before unconditional stores, including identical values;
  missing rows warn without a notification/store. Non-enclosed writes only tag 2,
  preserving inactive shape payload. These notification branches are static facts.
- `0x1408c5690`: ordered equality against f32 bits `0x40333333` (2.8).

All edits preserve supplied NaN/signed-zero bits. Unsupported enum schemas use
observed bytes instead of invented variants. Curve kind is the retained CPU
collection adapter's shape tag, not an additional claimed native store.

## Verification boundary

`verify_native.py` checks the RNE digest before execution. Unicorn runs 80 actual
setter tails, 6 `new_wall_w_params` entries, 8 default-height comparisons and 3
reset entries. Constructor map insertion/notification are library bridges; setters
start AFTER lookup/notification. Constructor inactive shape payload and native
Vec headers are excluded. `tests/fixtures/wall-authority-b.json` contains original
outputs, and compiled Rust tests compare every modeled field against those bytes.
This does not execute the original map implementation or original full publisher.

Commands from the formal workspace:

```sh
cargo check -p country-core --locked --offline
cargo test -p country-core --locked --offline --test wall_authority
cargo test -p country-core --locked --offline --lib
cargo fmt -p country-core --check
cargo clippy -p country-core --all-targets --locked --offline --no-deps -- -D warnings -A clippy::chunks_exact_to_as_chunks
```

Results: check passed; 7 integration/native-fixture tests passed; 1349 existing
library tests passed; formatting passed. Host rustc is 1.98.1 aarch64 macOS.
Unqualified strict Clippy failed on the existing tg-history `unnecessary_map_or`
warning. Package-only strict Clippy then failed on 11 existing constant-size
`chunks_exact` suggestions in unrelated render/camera/startup/building code.
The last command checks core targets with only that existing style suggestion
disabled; it passed. No lint suppression was added to production source.

The World test covers create, publish, a clean update, height edit, marking
non-enclosed, delete and explicit restoration. Missing IDs, duplicate insertion
side effects, equal writes, counter reset/remove_all, failed publication and deletion
cleanup have separate tests. Publication failures remain observable through `get`;
old successful geometry is not served while that ID has a failed rebuild.

## Remaining consumers

- Full original PublicWalls record, RDP/simplified curves, spatial index, materials,
  entity maps, CPU instance production and GPU resources are not supplied here.
- Terrain-dirty subscriptions and lazy routing are not connected to this consumer.
- Freehand/gable height consumers, wall/roof associations, full history recording,
  replay, save schema and original scheduling remain open.
- `WallPublicationState` deduplicates source IDs using real sets and reports final
  authority state. Its failure retention and last-delta API are explicit CPU adapter
  behavior, not a recovered original event stream/failure contract.
- No original full-process/ECS/GPU test or independent reviewer acceptance occurred.
  Source/World integration is implemented; original verification is limited to the
  instruction ranges above. No overall NR1 or core completion percentage is updated.

Next core consumer: unify complete published wall state and spatial/roof change
consumers, preserving the authority and history schemas established in this batch.
