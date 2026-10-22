# Tiny Glade production render performance evidence

Measured on 2026-10-22 at 960x540 with Release builds, vsync disabled, and
no FPS cap.

## Compared revisions

- Baseline: `f98b40229`
- Latest fetched: `d5c49dafa`

The latest revision was tested from a detached sparse worktree because the
repository contains legacy recovery filenames that Windows cannot represent.

## Primary result

| Mode | FPS | Wall p50 | Wall p99 |
| --- | ---: | ---: | ---: |
| Baseline offscreen production graph, 300 frames | 2.46 | 404.65 ms | 464.69 ms |
| Latest offscreen production graph, 300 frames | 9.48 | 104.35 ms | 120.33 ms |
| Latest ECS 2 workers | 9.30 | 106.10 ms | 123.87 ms |
| Latest ECS 4 workers | 9.01 | 109.15 ms | 143.95 ms |
| Latest ECS 8 workers | 9.31 | 106.47 ms | 123.47 ms |
| Latest visible window, 300 gameplay frames | 8.63 | 107.00 ms | 288.00 ms |

Latest offscreen throughput is 3.86x the baseline (+286.1%). Wall p50 and p99
decreased by 74.2% and 74.1%.

## Hot path after optimization

Latest steady offscreen frame, in milliseconds:

| Stage | p50 | p95 | p99 |
| --- | ---: | ---: | ---: |
| CPU update | 3.2 | 4.7 | 6.2 |
| Graph declare | 10.0 | 12.6 | 13.8 |
| Graph compile | 1.1 | 1.9 | 2.1 |
| Command record | 6.4 | 7.9 | 9.8 |
| Queue submit | 15.6 | 18.0 | 20.4 |
| Graph execute total | 23.0 | 26.3 | 29.2 |
| GPU command spans | 103.211 | 114.172 | 119.156 |
| End-to-end wall | 104.354 | 114.717 | 120.334 |

The GPU is now the limiting domain: GPU busy time and end-to-end wall time are
almost equal. Before optimization, GPU time was already about 105 ms, while CPU
record p50 was 140.2 ms, update was 106.8 ms, and submit was 62.9 ms.

## GPU pass hotspots

The latest 300-frame run enables `pass_spans: true`. It produced 48,607 pass
spans across 110 pass names, plus 2,659 whole-command spans.

Steady-frame GPU pass ranking by mean grouped duration:

| Pass | Mean ms | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| G-buffer | 27.06 | 26.22 | 33.28 | 37.21 |
| G-buffer OC/2 | 16.58 | 15.69 | 20.99 | 23.79 |
| Depth of field | 12.40 | 11.81 | 16.64 | 19.32 |
| Shadows, 3 instances | 10.53 | 9.90 | 15.15 | 17.05 |
| Apply deferred lighting | 5.02 | 4.82 | 6.46 | 7.08 |
| Volumetrics | 3.88 | 3.32 | 6.71 | 11.30 |
| Blur buildings mask, 2 instances | 3.04 | 2.66 | 4.79 | 7.05 |
| Apply screen shadows | 2.94 | 2.58 | 4.80 | 8.08 |
| Trace GI candidates | 2.66 | 2.29 | 4.44 | 6.71 |
| TAA | 2.50 | 2.10 | 4.35 | 6.16 |

Rows with multiple instances contain the sum of that named pass's instances in
one frame. Pass spans overlap whole-command spans and must not be added to them.

## Multi-thread result

`ecs-multithreaded` with 2, 4, or 8 workers did not improve this scene. The
single-threaded executor remains fastest at 9.48 FPS. ECS update p50 increases
from 3.2 ms to roughly 6.4-7.0 ms due to scheduling and conflict costs.

## Visible window

The real window ran for 300 gameplay frames and queued 300 presents using:

- `TG_DIAGNOSTIC_RETAIN_WSI=1`
- `TG_GAME_THREAD=1`
- `TG_ECS_THREADS=4`
- `TG_FRAME_TIMING=1`
- `TG_FRAME_TIMING_EVERY=1`

The Intel UHD 630 driver lacks `VK_KHR_swapchain_maintenance1`, so strict
shutdown cannot prove the final present. The process exits through the existing
`RetainUnproven` diagnostic path after the measured 300 frames; frame capture is
complete. The unique window p99 spike is 288 ms and includes runtime hitches.

Counters `frame_throttle_ms` and `observed_completion_wall_ms` describe the
pipelined present path and are not additive.

## Detailed files

- `performance-summary.json`: machine-readable comparison and caveats.
- `latest-300-per-frame.csv`: one row per latest frame with CPU wall, update,
  declare, compile, record, submit, execute, GPU command count, GPU busy and
  largest GPU command span.
- `latest-300-gpu-command-spans.csv`: all 2,658 timestamped GPU command spans.
- `latest-300-gpu-pass-spans.csv`: all 48,607 GPU pass spans.
- `latest-300-gpu-pass-per-frame.csv`: per-frame grouped duration and instance
  count for each GPU pass.
- `latest-gpu-pass-summary.csv`: all steady-state GPU pass percentiles.
- `gpu-pass-summary.json`: top GPU passes and collection metadata.
- `window-300-render-stages.csv`: all 300 visible-window frames with draw lists,
  declare, compile, record/submit, throttle, acquire/present and completion-wall
  timings.
- `baseline-60-per-frame.csv`: baseline CPU and GPU per-frame comparison.
- `baseline-60-gpu-command-spans.csv`: all baseline GPU command spans.
- `baseline-300-cpu-per-frame.csv`: baseline CPU-only 300-frame detail.
- `latest-{2w,4w,8w}-300-cpu-per-frame.csv`: ECS multithread details.
- `latest-gpu-hotspot-300/gpu-command-timings.json`: raw GPU timestamp samples.
- `window-default-mt/stdout.log`: raw complete window timing log.
- `latest-gpu-hotspot-300/run.log`: raw complete offscreen timing log.

`pass-record-last-frame.txt` records CPU graph-recording duration per pass and is
separate from GPU pass timing. The newer `latest-gpu-pass-*` files contain actual
GPU timestamp spans grouped by frame receipt and pass name.
