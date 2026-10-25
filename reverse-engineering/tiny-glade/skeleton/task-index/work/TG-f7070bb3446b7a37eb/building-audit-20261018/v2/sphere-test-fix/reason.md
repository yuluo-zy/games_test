# Sphere options regression: dependency oracle correction

Failure reviewed: root's core-lib-tests-r2.log, world.rs:3047, assertion abs(hit.toi-1.0)<1e-4. Current production world.rs is exactly frozen v2 candidate SHA256 0b00c2d8051e180f0b6a46e8c4393605e197ac6357c99ac22881f781256954af.

## Diagnosis

The options forwarding implementation has no demonstrated parameter error. The public with_options entry forwards origin, direction, radius, all four ShapeCastOptions fields and mask directly to the single Rapier core. The core uses the original radius as Ball geometry, the original origin/direction as Isometry/velocity, passes options unchanged to the first selected slot, and replaces only max_time_of_impact with a hit's returned TOI for subsequent slots. This agrees with the already retained original B world instructions (0x1408cd040/044/050/054 argument loads, 0x1408cd23f hit tightening).

The regression instead assumes analytic cuboid/sphere contact TOI with a fixed1e-4 tolerance. Pinned Parry0.17.0 DefaultQueryDispatcher routes Cuboid/Ball through support-map shape cast, then GJK directional_distance / minkowski_ray_cast. That algorithm returns iterative support-simplex estimates, with epsilon and last-chance exits; its public contract does not guarantee this hardcoded analytic tolerance. The adapter's responsibility here is preserving the dependency result and options, so making it force an analytic TOI would change original behavior.

Minimal candidate changes only this one test. The test directly casts each isolated slot using the pinned QueryPipeline, verifies slot0 is nearer, then requires exact equality between the adapter TOI and slot0's dependency TOI. It repeats that exact comparison with target_distance0.5 and verifies it advances contact, while retaining the maxTOI0.25 miss and initialized miss scalar check. Dropping target_distance, swapping radius/maxTOI or restoring the old last-slot-wins behavior still fails this test; no tolerance is relaxed. No production query code changes.

## Verification and limits

- sphere-test-oracle.patch: two hunks inside the failing test only.
- world.rs: complete candidate based on the frozen production file.
- manifest.json: source/candidate/patch SHA256.
- rustfmt parse passed; git apply --check passed.
- No Cargo invocation, Rust compilation or candidate execution by this reviewer.
- An attempt to inspect actual hit.toi in the existing c5336171ffcb459d test executable with LLDB loaded the correct symbol/line but macOS failed to attach/pause execution, both sandbox and approved host launch. The failed root log contains no numeric TOI, so this report does not invent its value. Root's rerun of the exact dependency comparison is the required dynamic confirmation.

Primary dependency source:
- /Users/yuluo/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/query/default_query_dispatcher.rs:272..335 dispatch.
- .../query/shape_cast/shape_cast_support_map_support_map.rs:23..35 options and GJK.
- .../query/gjk/gjk.rs:206..363 iterative TOI.
- /Users/yuluo/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/src/pipeline/query_pipeline/mod.rs:607..631 QueryPipeline forwarding.

