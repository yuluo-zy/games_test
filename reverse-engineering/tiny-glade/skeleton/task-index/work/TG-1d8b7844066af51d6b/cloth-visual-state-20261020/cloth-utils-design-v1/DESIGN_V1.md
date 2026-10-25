# Shared cloth/assembly utils design v1

Root claimed TG-1d8b7844066af51d6b for cloth-visual-state-20261020. This is an append-only semantic helper design for current crates/utils/src/lib.rs, preserving all module declarations/reexports, including pending user sampling. No dependency/Cargo changes, no sampling file, no local duplicate consumer helper. Original proof is B only, not C parity.

## Public host signatures

```rust
#[track_caller]
pub fn random_splits(count: usize, jitter: f32, rng: &fastrand::Rng) -> Vec<f32>;
#[track_caller]
pub fn random_splits_into(count: usize, jitter: f32, rng: &fastrand::Rng, out: &mut Vec<f32>);
pub fn xz_rotation_bad(direction: glam::Vec2) -> glam::Quat;
```

These are explicit current-host interfaces, not recovered PDB Rust ABI declarations. random_splits native hidden-return pointer plus count/jitter/Rng pointer and track_caller Location exactly support three business arguments. Original borrowed mutability spelling is unknown. Pinned fastrand1.8.0 defines Rng(Cell<u64>) and f32(&self), so accepting &Rng retains one real state stream and allows existing assembly mutable references to coerce. Do not make or reseed a new Rng per call.

## random_splits / into

Reuse05's immutable random-splits contract/manifest and original332+760B, with the exact native PE proof in the consumer projection pack as cross-reference. Do not reconstruct Vec allocation, random generator or dependency algorithms.

- Wrapper random_splits (0x140c90350/332B, PDB759:2988) rejects count<=1, allocates the actual count capacity and calls into. Preserve #[track_caller], which explains the extra native Location argument; it is not a fourth business parameter.
- Into (0x140c90050/760B, PDB759:904) appends, does not clear. For count<3 it appends exact0.0,1.0, even for0/1, and does not draw RNG. Thus wrapper and into empty-boundary behavior intentionally differ.
- For count>=3 cap is computed in native order `(1.0 / ((count-1) as f32)) * f32::from_bits(0x3efd70a4)` (0.495), not replacing with mathematically equivalent `.495/(count-1)`.
- Effective jitter is input when input<=cap; otherwise cap. NaN input selects cap. Negative finite/infinite jitter is not silently clamped. Exact original special-value policy must be preserved.
- The progression step independently uses `1.0 / ((count as f32) - 1.0)`. The cast/subtraction order differs from cap for very large counts; do not common-subexpression these denominators.
- Append0, then for index1..count-1 append `((rng.f32() + (-0.5)) * effective_jitter) + (index as f32)*step`, then append1. Exactly count-2 draws; endpoints consume none. No sort or renormalization.
- Real Vec checked capacity/append behavior is used. No fixed maximum count, fabricated row or empty-success fallback.

05's mutable-Rng contract spelling was an engineered consumer preference, not a contrary original type declaration;05 confirmed these shared signatures and will correct its frozen contract with a new note.

## xz_rotation_bad

Original0x140c90fd0/504B, PDB759:8908 is independently retained as complete original PE assembly. It receives two f32 coordinate values and returns a Quat through an out-pointer; host Vec2 is the observed semantic pair, not a claimed source declaration.

Normalize the x/z direction with the pinned Vec2 try_normalize policy: reciprocal length must be finite and positive, including positive subnormal reciprocals; otherwise use exact Vec2::X. Native finite-bit check0x140c91000..36 supports this. Degenerate/nonfinite/overflowed-length input falls back to +X, which yields a +X-facing rotation, not identity.

Create `forward=Vec3(direction.x,0,direction.y)`, `up=Vec3::Y`, then `right=up.cross(forward).normalize()`; the second normalization is present in native0x140c91080..a4 and must not be dropped merely because forward was normalized. Build Quat with pinned glam0.29.3 `Quat::from_mat3(Mat3::from_cols(right,up,forward))`. Native504B contains that basis-to-Quat branch calculation. Reuse this dependency conversion rather than copying its implementation or replacing it with atan2/from_rotation_y, which can choose a different quaternion sign branch. Observed rdata basis constant0x1429cd0c0=(0,0,1,0), with unit/half constants exactly1.0/0.5, confirms cross product and quaternion scaling.

## Candidate boundary and tests

After07 design review/root authorization, append only these helpers and a local test module to current lib beforeSHA. Existing module list/reexports must be byte-preserved. No other current helper implementation is changed and no two consumer-local copies are introduced. The actual cloth and assembly callers use this one shared interface only after root integrates it.

Planned meaningful CPU tests, unrun by this agent:

- Wrapper count0/1 panic; count2 returns[0,1] and leaves actual Rng seed unchanged.
- Into count0/1/2 appends[0,1] to a nonempty sentinel Vec without clearing and without consuming RNG.
- For count>=3, endpoints/count and exact count-2 Rng advances; zero-jitter boundaries use the separate step formula; native NaN/+infinity jitter selects same cap/result/state as explicit cap; negative jitter is preserved.
- xz cardinal forward mapping and quaternion branch through pinned from_mat3, non-cardinal directions, zero/NaN/infinity/overflow/tiny inputs mapping to +X fallback. Test the actual dependency types, no fake RNG/math facade.

No Cargo/typecheck, CPU tests or GPU has run. This design is a shared dependency closure for actual consumer recovery, not proof that ClothInfo producers/clothline output/Nani Cloth bindings are complete.
