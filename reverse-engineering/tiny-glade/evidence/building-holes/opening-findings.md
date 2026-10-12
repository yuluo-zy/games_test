# Wall decorator hole producers

## Provenance and dependency boundary

The four producers are game-owned `system_decorator::decorator_visual` functions,
from `crates/systems/decorator/src/decorator_visual/{cottage,gothic}_{wall_window,balcony_door}.rs`.
Three-version function addresses/source checksums and binary identities are retained
in `opening-three-version-map.json` and `opening-provenance.json`. All four source
files differ across A/B/C. The Gothic window's two embedded four-float vectors
per size are present in all three binaries; this is static shared parameter evidence,
not proof of the complete A/C producer behavior. These are not dependencies merely
because they have shared parameters.

`std::HashMap`, `Vec`, `Arc`, Bevy Commands and glam transforms stay upstream.
The collision library lookup is an asset-metadata boundary: reconstruct the game's
argument/selection policy separately from container probing. Decompiler aliases are
not authoritative: the raw-hole Vec growth call is mislabeled as
`rhapsody::ResourceAccessInfo`; the 20-byte stores and destination blueprint field
identify the actual game record. `WindowVisualEntityBuilder::build` creates visual
components and entities, but does **not** produce the raw wall holes.

## Recovered B production policy

`reconstruction/building/opening_rules.rs::generate_wall_opening` receives the
authoritative wall-space `DecoratorInfo::dst_as_wall` center, derived size byte
1/2/3, decorator family, resolved collision XZ extents/vertical bounds and host wall
shape. It creates raw hole records, rather than accepting final hole arrays.

- Cottage window (`1407ebfd0`) contributes one kind-0 rectangle. Width is the
  collision X extent, height is `maximum_y - minimum_y`, center is the wall-space
  decorator center. Circle host tag 0 additionally adds width adjustments
  -0.05/-0.1/-0.15 for sizes 1/2/3. Other host shapes have no width adjustment.
  This condition is **wall shape**, not wall style or half-timbering.
- Cottage balcony door (`1407f3da0`) contributes one rectangle with the same
  collision extent/height and unmodified center. No circle shrink applies.
- Gothic window (`1407e87b0`) and balcony door (`1407f0900`) each contribute
  **three separate rectangles**. Width, height and center-Y offset come from
  size-specific original scalar tables; center-X stays identical. They do not
  merge overlapping rectangles or normalize wrapping at production time.
- Kind byte is 0 for all these records. Tail padding bytes are not written.
- Invalid size variants take original panic paths; Rust reports
  `OpeningRuleError::InvalidSizeVariant` at this external boundary.

Periodic duplicates, ordering and fingerprint caching remain the
`WallHoleStorage::finalize` consumer. The producer itself adds no independent
dimension/height clamp.

## Original instruction verification

`scripts/multi-version/verify_opening_rules.py` executes all production branches:

| Family | Original B range |
|---|---|
| Cottage window | `1407ed546..1407ed659` |
| Gothic window | `1407ea272..1407ea4e4` |
| Cottage balcony door | `1407f4adb..1407f4bba` |
| Gothic balcony door | `1407f16c4..1407f1969` |

3,072 cases cover four families, all three sizes, four host shape tags, shifted
centers, negative widths and signed-zero/threshold inputs. All f32 output fields,
kind bytes, record ordering, original Vec count changes and untouched padding
match. Original artifact digest and relevant Rust/candidate binary/lock hashes
are in `opening-B-verification.json`.

Explicit bridges: resolved `dst_as_wall` accessor; supplied successful collision
map slot for the cottage door; standard allocator/free; adequate standard Vec
capacity. No expected hole output is injected. This is complete **resolved
production**, not the entire generator's asset/visual/morph execution.

The first run exposed a real reconstruction error: Gothic-window size 2 offset
is `bdc8b43a`, while spelling it as Rust `-0.098` yields `bdc8b439`. The source now
retains the original f32 bits; the eight mismatches are preserved in
`opening-B-decimal-literal-failure.json`.

## Integration contract and remaining boundary

The runtime must store authoritative decorator placement/family/size plus resolved
collision metadata. On creation/edit/host changes, regenerate contributions;
on deletion remove the contribution and rebuild the previous host wall. Persistent
decorators still contribute on frames in which their visuals are not rebuilt.
Missing host or collision metadata must stay explicit, not produce guessed holes.

The parent Bevy chain integrates this producer with RecalculateWallHoles →
OnWallChanged → decorator state/cache regeneration → WallHoleStorage finalize
and deletion. Roof-aware placement, Gothic hat/morph bones, dormer SSBOs, visual
asset selection and GPU builders preceding/following these blocks remain separate
work; passing a resolved raw-production oracle does not establish their recovery.

Reproduce:

```
cd reconstruction
cargo build --release --locked --bin opening-probe
cd ..
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_opening_rules.py
```
