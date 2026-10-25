# Independent Signifier animation/lifetime review

Read-only review for the central owner and `save_migration_full`. No builder, animation, deletion source, or that agent's candidate was edited. No Cargo or new LLVM code generation was run. LLVM objdump only reads original instructions.

## Evidence identity and executable boundary

Original B RNE SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

`verify-signifier-independent.py` (Python313, existing pefile/Unicorn) writes `signifier-animation-independent-original.{asm,json}`. It retains complete original function bytes/hashes and eight typed PDB query/config records:

| Original function | Exact range | Bytes SHA256 |
|---|---|---|
| signifier_animation_t | 140a69340 +2276B | 2bb00baf1266bb9226c3f0ac795795a7ee07028fee152e37c04b7ac1e11221b0 |
| signifier_animation | 140a69c30 +2116B | 7745818a06ec61e940743c8ee3aae919aac2872c972ed86b9112f91fb1188905 |
| signfier_mark_for_delete | 140929160 +2350B | e0e629b6dd9baba4e130fb4ade5fb828e91b408261e348ef7d8b76a5b52e03d3 |
| despawn_marked | 140929a90 +1644B | d3cf36b3aa8482ed1c89526ab548cbcee9bfb97b37559592f6435b2182005c31 |

Unicorn executed **1008 actual original time/clamp cases** over140a69998..140a699fe, stopping before the EasingFunction call. Inputs cover duration±0, positive/negative finite, infinity, quiet/signalling NaN, subnormal; old t includes±0/NaN/infinity; delta includes±0, positive/negative finite, infinity and NaN; each direction is exercised. JSON retains exact input/output bits. This is an original-instruction oracle, **not a Rust candidate comparison or easing/ECS/schedule acceptance**.

## Observed: exact query and state ownership

PDB wrapper1417ca640/372B and config1417cc950/421B establish:

```
Query<(&mut SignifierAnimation,
       Option<&SignifierMarkedForDeletion>,
       Option<&mut SignifierMinimumLifespan>)>, Res<GameTime>
```

Animation state stride is0x1c; EasingFunction occupies the first8B, duration at10, t at14, eased at18. Optional minimum-lifespan stride is8B; elapsed time is its first f32. There is no native tick_acc field in SignifierAnimation. GameTime.delta is read at28. Reverse is determined by deletion-marker presence, not an animation byte or sign of duration.

## Observed: time arithmetic, zero duration and NaN

-140a69998..a2 compares duration to zero, taking arithmetic on JNE **or JP**. Thus `duration != 0.0` already includes NaN in Rust; `&& !is_nan()` is incorrect.
-140a699c4 reads positive GameTime.delta. Marker presence flips its sign bit140a699ce. Arithmetic140a699d2..e6 is **`(1.0 / duration) * signed_delta + old_t`**. Replacing it with `signed_delta / duration` changes f32 rounding.
-140a699eb..f5 runs `MAXSS destination=+0, source=raw` then `MINSS destination=1, source=that result`. The second operand wins on NaN/equality. Therefore raw NaN is preserved, and a raw negative zero is preserved. The retained high-level decompilation's ordered comparisons are wrong on NaN. Rust `clamp(0.0,1.0)` has the required NaN/-0 behavior; Rust `max/min` suppress NaN and must not replace these operations.
-For duration±0,140a699a4..ac directly sets t=1 if unmarked and t=0 if marked, even when old t/delta is NaN.
-Both branches write t, call the existing original EasingFunction::apply140c95ef0, then write eased at18. Zero-duration animation still needs this call/write. This is the direct reason a newly built scale-zero mesh remains invisible under the current production zero-duration no-op.

Selected actual oracle outputs:

| duration | old t | delta | marked | new t bits |
|---|---|---|---|---|
| +0 | +0 | +0 | false | 3f800000 |
| -0 | +0 | +0 | true | 00000000 |
| 1 | -0 | -0 | false | 80000000 |
| 1 | -0 | -0 | true | 00000000 |
| qNaN7fc01234 | +0 | finite positive | either | 7fc01234 |

## Observed: optional minimum lifespan always advances positively

After easing,140a69a17 tests the optional-lifespan pointer. If present,140a69a20..36 adds **the original positive GameTime.delta** to lifespan.t, even when animation is reversing. It does not derive elapsed life from animation.t, does not reverse elapsed age, and does not reset it on every request. Framework change ticks are updated through the actual mutable component owners.

## Observed: mark and despawn gates

Mark wrapper14086ee90/425B has:

```
Res<UiState>,
Query<(Entity, &SignifierId, Option<&SignifierMinimumLifespan>),
      Without<SignifierMarkedForDeletion>>,
Commands
```

The native function builds its alive set only when UiState byte18 is active, using the current widget id at38 plus each optional signifier name from the active widget's0x28B entries. It uses existing calculate_hash<(WidgetId,Option<&str>)>14145a710 and standard HashSet behavior. Matching hashes skip deletion. For a withdrawn ID, optional lifespan is checked140929800..80d: `ucomiss min_lifespan, elapsed; JA skip`. Exact expired predicate is `!(min_lifespan > elapsed)`, so NaN in either field allows marking. Current `min_lifespan <= elapsed` incorrectly inhibits it.

Despawn wrapper14086f040/148B and config140870020/421B establish the exact tuple:

```
Query<(Entity, &SignifierMarkedForDeletion, Option<&SignifierAnimation>)>, Commands
```

Native140929f51..6e directly despawns if animation is absent. If present, it reads t at14 and permits despawn only for **ordered t==0.0** (both signed zeros qualify). NaN and any nonzero t remain. Bevy EntityCommands::despawn140d14770 owns actual deferred removal. The current production immediate despawn of every marker truncates the complete exit animation.

## Inferred implementation mapping, sent to candidate owner

- Preserve the real typed t query; use GameTime delta and the existing full typed EasingFunction implementation, keeping its exact algorithm/variant behavior.
- Remove the extra tick_acc projection; update the canonical optional lifespan instead.
- Mark's narrow numeric correction is `life.is_none_or(|life| !(life.min_lifespan > life.t))`.
- Despawn uses the real tuple and `animation.is_none_or(|animation| animation.t == 0.0)`.
- A With<Marked> filter would be semantically equivalent to the component tuple, but the real original tuple is available and avoids an unnecessary design divergence.

These mappings are review guidance. Only the candidate owner edits/integrates them.

## Unknown/unclosed boundaries

1. Current production mark reads `UiSignifierStream.active_ids`, whereas original reads actual UiState/current widget/name identities. A parallel stream alive set cannot be called a complete original mark implementation without proving it carries the exact original alive identities and lifecycle. Original query identity payload stride/GlobalId projection also merits full source-owner validation; do not infer every field from the u64 hash helper's ICF aliases.
2. `signifier_animation` has UiState-driven translation/parent volume cases beyond the scale path. They are excluded from this narrow numeric/lifetime review and remain unverified here. Do not use a correct t/eased update to claim those branches restored.
3. Existing EasingFunction::apply is a game-owned utility with multiple variants. This review establishes the original call and payload location, not bitwise differential execution of all easing variants or NaN payload propagation through every easing algorithm.
4. Current signifier schedule is a flat chain in source. This review does not certify it matches the complete original registration/schedule/deferred boundaries. In particular, forward/update, marking, reversed exit and final removal must be accepted with actual multi-frame ECS tests.

## Required meaningful candidate regressions

- Actual builder-produced mesh with duration0: first time update writes t1 and eased1, then actual transform becomes nonzero; delete marker writes t0/eased0 and permits deferred despawn.
- Nonzero duration: reach t1, withdraw request, marker appears, entity survives at positive reversed t, despawns only after reaching ordered zero.
- Optional minimum lifespan: elapsed age grows positively while forward/reversing; no mark before min age; expired threshold then marker; withdrawal/re-request lifecycle uses the original ID owner.
- NaN duration enters arithmetic; NaN lifespan is eligible for mark; NaN animation.t prevents despawn. Include negative duration and signed-zero cases rather than silently normalizing them.

`save_migration_full` received exact addresses, query signatures, numeric corrections and these boundaries directly. Final candidate compilation and runtime acceptance remain with central integration.
