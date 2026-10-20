# Decorator interaction restoration, 2026-10-20

The work restores bounded interaction leaves. The surrounding affordance,
place, move, rotate, split and interaction-intent systems remain partial.
No whole decorator completion or percentage increase is claimed.

## Source and ownership

- `decorator_resources.rs` and the two widget factories were integrated by the
  central owner from the reviewed two-file patch and SHA manifest. The canonical
  `DecoratorHandleName` is a `(u64, u8)` key. B14125d180 passes those two values
  to B14128c8e0, which hashes exactly eight bytes then one byte with std
  SipHasher13. The old string key/terminator changed widget identity.
- The widget factories store their XMM3 argument as the tag-5 volume's float at
  +4. It is not a canvas layer. They preserve constructor occlusion=true and
  flag bit0, OR bit1 and optional bit3, and copy all four sound slots. The
  previous implementation discarded the flags and forced occlusion=false.
- `calculate_move_dst` now executes all four native candidate branches and
  writes through canonical `DecoratorInfo::set_destination`. The wall branch
  uses its actual storage wall, published attachment height and world position.
  Missing storage/wall/cache remains an error. Its old input was incorrectly
  described as DecoratorInfo: original input is the selected interaction
  candidate, whose tags are roof/wall/terrain/stair, distinct from stored tags.
- Grab-offset now preserves the original Option<Vec2> output, all type/destination
  surface gates, both stair IDs, world/trigger anchor choice, the rotated 0.315
  fallback, and the original f32 correction sequence. Host projection, query and
  premade trigger lookup are mandatory accessors. The outer move UI and actual
  premade trigger accessor are not recovered by supplying these leaf adapters.
- Offset raycasting uses canonical screen unprojection, Raycasting and terrain
  owners. It performs the terrain query even after a physical hit; a physical
  hit retains priority. Misses use terrain, then the unclamped y=0 plane. The
  old epsilon guard changed horizontal-ray infinities/NaNs and was removed.
- The fake 88-byte ray record was replaced by the canonical logical physics
  result alias. This is a semantic adapter; it does not claim that the original
  moved-enum ABI and the canonical physical-query ABI share discriminant offsets.

## Original evidence and exact limits

`native-manifest.json` records the actual B image SHA
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, six
raw function slices, their hashes, and matching PE unwind boundaries. The
original installation path no longer exists; the verified original mirror is
`workspace/input/tiny-glade.exe`. No database or dependency implementation was
changed. The std hash implementation is reused rather than reconstructed.

`collect_native.py` actually executes complete original function bodies with
bounded dependency hooks:

| Native probe | Cases | Boundary excluded from the claim |
| --- | ---: | --- |
| handle hash | 25 | none of the hash algorithm or std writes are hooked |
| both widget factories | 24 | registry insertion, NodeContext destructor, CRT memcpy |
| move destination | 8 | std hashing, wall-height/world-coordinate callee values |
| grab offset | 400 | projection, physics query, public-wall lookup; native wall fixtures use freehand shape |
| offset ray | 24 | unprojection, physics/terrain samplers; masks and query count recorded |

The physics/terrain query masks are 12459 and 35. Every offset case queried
terrain exactly once. Native fixtures include horizontal-ray IEEE values.
Premade trigger lookup branches in grab-offset have static instruction evidence,
but the 400 native cases do not independently execute Bevy query storage or
that trigger-volume implementation. Camera, terrain and physics libraries are
not validated by hooks that supply their results.

## Actual Rust verification

`handle-hash-actual-source.rs` directly includes the actual production owner;
only its WidgetId scalar wrapper is represented at the boundary. Its test
passes all 25 native hash outputs. No Cargo was run by this agent.

`ui_systems/native_interaction_goldens.rs` is registered in the production
crate's lib tests. It consumes original receipts, not a rewritten expected
algorithm: 8 full 72-byte move records (including preserved bytes and both wall
sides), 400 grab cases, and 24 offset-query cases. The move tests create real
InnerWalls and PublicWalls and supply a post-initialization freeform cache;
production lookup/height/world-coordinate functions remain actual. The widget
tests use actual UiState registries and all sound slots. Central Cargo execution
is pending; authored tests and successful native execution are not Rust passes.

## Independent review of the visual owner's door-stair leaf

Reviewed the actual source, transform/dimension reads, current WorldRaster
selector, callback coordinate order, 32-byte allocation/replacement and old-box
preservation. The actor's first 2000 cases only injected terrain NaN in lanes
0/2. Added an independent 192-case probe covering all four terrain lanes plus
coverage, width, bounds, positions and quaternion inputs with NaN, infinities,
signed zero and finite boundaries. The first probe script changed `case['q']`
without updating its separate `q` variable: that first quaternion coverage claim
was invalid and is withdrawn. After the visual owner identified it, the script
was corrected, native fixtures regenerated and actual-source Rust rebuilt.
The corrected run compares every output float bit and every sample-coordinate
bit/order: **192 cases, 0 mismatches**. See the final fixture/source hashes and
`door-stairs-independent-result.txt`. This proves the geometry leaf at sampler
boundaries; it does not prove the actual shared sampler or the surrounding
`generate_gothic_balcony_doors` visual producer.

## Remaining interaction gaps

`ui_dispatch_decorator_affordance`, `calculate_decorator_interaction_intent`,
actual move/place/rotate/height consumers, roof/wall splitting, full stair
interaction and their host resource lifecycle remain incomplete. Restored
leaves have not been substituted for those systems, and no noop/default/removed
branch is counted as a completed interaction chain. The obsolete module-header
notes referring to a widget layer are superseded by the exact factory contracts
and original receipts above.
