# Bounded reclamation domain verification

- CPU: `cargo test -p rhapsody --all-features --lib --locked --offline -j 2`, 263 passed.
- Native: identity-bound copied test executables, strict Vulkan requirement and validation/synchronization layers, 14 passed, zero validation errors.
- Reclamation: configured per-maintenance-tick scan/destruction limits, persistent cursor fairness, no implicit submitted-owner drop collection; explicit completed-object draining and shutdown fully collect eligible objects.
- Completion ordering: all six three-queue permutations under small scan/destruction budgets, plus three deterministic four-queue randomized schedules. Every returned object must have all dependencies reached, return once, and eventually drain.
- The real fixture queues 161 completed submitted samplers: two ticks destroy exactly three each, the explicit drain releases the remaining 155. In-flight dependencies remain protected even for explicit draining.
- Unsubmitted EMPTY objects retain immediate destruction for construction/failure cleanup. The quota does not claim to bound arbitrary unsubmitted drops or the sum of multiple maintenance ticks in one frame.
- Ready storage uses fixed inline batches of 64 with no spill/admission allocation; native destruction occurs after releasing the retirement lock. No Device ownership was added to deferred owners.
- Full cleanup callers and their accounting regressions use the explicit endpoint. Normal FrameLoop/Session maintenance remains quota-limited.

This domain report does not replace the final unified, source-frozen matrix. Exact copied executable identities and checked source hashes are in `manifest.json`.
