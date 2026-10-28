# GPU pass diagnostic evidence (2026-10-27)

This directory preserves the raw diagnostic data used by
[the GPU pass-level performance plan](../gpu-pass-diagnostic-plan-20261027.md).
It is diagnostic evidence only: the isolated runs serialise GPU passes and are
not production-frame latency or a performance acceptance result.

## Source and binary identity

- Diagnostic source: `ec3ea62dabf77add2ae4ba50422b650b12d1e676`.
- Frozen Release EXE SHA256:
  `e4fe726fe4949bab3c645197db15760307ac03bd440dd2877b9ecc291246f44d`.
- GPU: Intel UHD Graphics 630.
- Output: 1280x720, Release, mimalloc, `cpu-probe`.
- Both isolated sessions use 200 warmup frames and 120 clean observations.
- Vulkan validation found no diagnostics; timestamp pending and capacity skips
  were zero; pipeline statistics were present for every recorded pass.

## Runs

| Scenario | Mode | Session sample count | Isolated pass total, mean |
| --- | --- | ---: | ---: |
| static | pass spans + isolate + statistics | 52,644 | 154.692ms |
| many_draws | pass spans + isolate + statistics | 52,644 | 394.270ms |
| many_draws | command-span baseline | 2,344 commands | 352.0ms command-span p95 |

`session.json` is retained losslessly as `session.json.gz`. The adjacent
`session.json.sha256` records the SHA256 of the original uncompressed file.
`run.json` binds the session, frozen EXE/PDB/runtime DLLs, controller and build
metadata. `fixture.log` preserves the exact test output.

## Key pass results

| Pass | static mean | many_draws mean |
| --- | ---: | ---: |
| G-buffer | 32.779ms | 219.821ms |
| Depth of field | 21.355ms | 22.598ms |
| G-buffer OC/2 | 19.852ms | 21.412ms |
| Shadows, three cascades total | 13.382ms | 59.997ms |
| Apply deferred lighting | 8.542ms | 8.982ms |
| Volumetrics | 6.168ms | 6.629ms |
| Trace GI candidates | 5.080ms | 5.509ms |

The detailed `*-ranking.json` files retain every named pass with mean, p50,
p95, p99, max, per-frame occurrence totals and the available pipeline
statistics. `analyze_gpu_passes.py` reproduces those reports from a session.

For `many_draws`, G-buffer recorded 6,243.2k input vertices, 2,081.1k input
primitives and 26.90M fragment invocations: 29.19 fragment invocations per
1280x720 pixel. Depth of field and Volumetrics each recorded about 942.1k
compute invocations. This separates geometry-heavy, pixel-heavy and
compute-heavy pass families.

## Exclusions

The frozen EXE, PDB, FMOD DLLs, Cargo target tree and other binaries are not
committed because they are large generated artifacts. Their identities remain
in `build/build.json`, each `run.json`, and `sha256-manifest.json`.
