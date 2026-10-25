# Explicit test semantics decision, original right and current measurement

This addendum supersedes only CONTRACT.md's instruction to leave incompatible external expected assertions unchanged. Root explicitly authorized strict original921B f32 behavior plus corresponding expected-value revisions after07 review. All historical source/test files are preserved whole in before/. No case is deleted or ignored, and no tolerance is expanded. Left-extension assertions and direct measurement semantics stay unchanged. The original whole function and current before SHA identities remain design-v1.

Original instruction basis:141289917..91a rejects negative/unordered distance; there is no zero-distance return.141289a37..a45 computes f32 component differences, squared sum and sqrt.141289a51..a5f computes `(1-old_t)*length-distance`, in that order.141289a67..a6b accepts only ordered nonnegative residual.141289bfc..c09 divides residual/length and subtracts from1 without a finite or zero guard.141289ade..ae1 commits `(segment,1)` for negative/unordered residual;141289acb..ad8 adds the next f32 length before checking again. This precisely explains all seven changed input categories below. NaN payload/sign bits are not asserted, only is_nan.

| Original test input / old assertion | New right assertion | Right→current host measure combination |
|---|---|---|
| length1e-30, half distance / t=.5 | segment0,t=1 exactly | measure/length=1 exactly (old expected .5 was dependent on enhanced right) |
| MIN_POSITIVE, half distance / t=.5 | segment0,t=1 exactly | measure/length=1 exactly |
| length1e20, half distance / t=.5 | segment0,t.is_nan | measure.is_nan |
| lengthMAX, half distance / t=.5 | segment0,t.is_nan | measure.is_nan |
| [-MAX,+MAX], distanceMAX / t=.5 | segment0,t.is_nan | add measure.is_nan coverage to original right chain |
| [0,(1e-30,0),(1e-30,1)], distance5e-31 / segment0,t=.5 | segment1,t=0 exactly | add exact measure=1e-30 coverage of same retained input |
| repeated first zero edge, distance0 / unchanged coordinate | segment0,t.is_nan | add measure.is_nan for this same retained chain; then positive.5 on that NaN state exhausts to segment2,t=1, so preserve that continuation with its real new result |

The right results are demonstrated by the original body and bounded Python f32 lift, not native execution. The right→measure column is **current-host composition**, inferred from the unchanged current measure_length/iter_coords/sample_at_coord/metric_distance_f64 source, and additionally evaluated by compare_f32_lift_v2.py. It is not evidence for original B measurement behavior at these extreme coordinates. Tests will label this boundary explicitly; no whole original combination-equivalence claim is made.

The original positive .5 extension after a zero-distance zero-edge had a different state because the old method returned early. Keep that original sequence, updating its now-observable end to `(2,1)`; additionally create a fresh slice of the same unchanged curve and assert direct positive.5 still yields `(1,.5)`. This retains the positive extension coverage rather than silently resetting the original chain. Left .5 assertion remains literally unchanged.

Preserve the original measurement expectation `.5` on all four original lengths through an additional explicit coordinate range `(segment0,t0)..(segment0,t.5)`, then the same `measure_length()/length` assertion and tolerance. This additional independent measurement case supplements, and does not replace, the original right→measure chain above. Measurement implementation is unchanged; this packet does not generalize its f64 enhancement to the original right method.

Same-file additional tests may use exact finite bit equality for the known zero-distance 1057632207→1057632208 witness, inclusive stop and remap/predicate laziness. They will not rewrite the existing260 sample source, whose current tolerance already masks26 old-source bit differences. Existing oracle/fixtures source stays untouched. No external dependency, benchmark framework or enhanced API is added.

No Rust/Cargo execution by this worker. `numerical-observations-v2.json` is an actually executed scalar lift only; no “tests passed” claim follows. Central can execute the narrow focused tests and full utils tests after integrating the coherent candidate.07 review must see the original input/output tables, historical before tests, unchanged left code and actual retained composition assertions in the candidate.
