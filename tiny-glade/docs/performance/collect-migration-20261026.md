# Collect convergence

Single-pass and cross-pass Collect use the existing configured chunk size (64 draws by default), at most eight in-flight jobs, a 4,096-draw declaration bound and a 32MiB constants admission bound. Cross-pass work now splits into bounded pages, runs through the same task executor and commits original pass/page/draw order. The timing counter reports actual page tasks.

Grass object pushes, screen UI, world-space UI and water/ice raster loops now use worker-owned RasterPass collectors. UI stable sort and the unsorted world-space tail are preserved. Surface callbacks receive a worker-local trace; only owned trace Vecs return with batches for ordered merge. No Rc crosses threads, no unsafe Send or global locking was added. Fixed single-object MEI and dependent compute operations remain on their existing path.

Validation: country-core all-target check passed with CARGO_INCREMENTAL=0, -j1. One earlier incremental check failed in rustc after a 2MiB allocation failure while unrelated builds were also active. Draw-pass tests: 260 passed, zero ignored. Includes golden water/ice operation sequences, constants/state/bindings, UI image layers and sort order, real worker identity, uneven chunks, 0/63/64/65/513/4097-draw boundaries, and constant-budget splitting. Logs: .perf/collect-check-noincremental.log and .perf/collect-unit-tests-rerun.log. Mock image-view observations travel with test batches so the original layer assertions remain intact.

Window A/B, representative GPU surface pixels, cancellation stress and no-regression performance gates remain pending. This is a source candidate, not Windows <=16ms acceptance.
