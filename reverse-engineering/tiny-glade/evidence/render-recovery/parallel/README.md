# Rhapsody parallel recovery batch 01

This directory is the integration entry for the first parallel graph-compiler recovery batch. The target is the shipped Tiny Glade binary with SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Work packages

| Package | Recovered boundary | Validation |
|---|---|---|
| `resource-info` | `ResourceInfo::calculate`, access chains, lifetime endpoints/events, usage aggregation | 512 original-entry semantic snapshots: 507 normal, 5 panic, 0 mismatch |
| `registry-pool` | exact descriptor pool reuse, origin ownership, registry mapping, consuming exports | 6 candidate tests; static original control-flow and ownership evidence |
| `compiled-state` | compiled layout, batch ranges/final flag, initial/pre/export transition decisions | 1600 original-entry BatchCompiler cases, 0 failure; remaining graph/GPU paths static |

## Unified path

`RenderGraph declarations -> ResourceInfo analysis -> acquire-event registry population -> physical backing mapping -> CompiledRenderGraph snapshot -> batch recording/barriers`

Observed shared layout:

- `CompiledRenderGraph` is `0x218` bytes in this build.
- `RenderGraph` occupies `+0x00..+0x8f`; `Vec<RecordedPass>` is at `+0x90` with `0x40`-byte elements.
- `ResourceRegistry` occupies `+0xa8..+0x1af` and is `0x108` bytes.
- `ResourceInfo` is `ResourceRegistry+0x48`, therefore `CompiledRenderGraph+0xf0`, and is `0xc0` bytes.
- `SynchronizationScope1` is at `+0x1b0`; pending initial-transition indices are at `+0x200`.
- Lifetime event bit 16 distinguishes release from acquire. Registry population consumes acquire events and skips release events.

The current evidence does not establish graph-local alias reuse: the local reuse maps are initialized empty and the observed population function skips release events. Persistent cross-graph `ResourcePool` reuse is independently established and uses exact descriptor keys with LIFO movement.

## Automation

Run `python scripts/render-recovery/validate_parallel_graph_contract.py` from the project root. It parses the three ledgers and validation reports, checks embedded offsets, shared record layouts, event polarity and scoped oracle results, then writes `integration.json`. This turns cross-agent reconciliation into a deterministic gate rather than another model pass.

## Remaining path

The next recovery batch should close queue submission/retirement, identify the producer or deliberate absence of graph-local reuse, recover temporal-state schema, and connect the three candidates behind one compiled-graph snapshot oracle. Full Vulkan execution, shaders and scene rendering remain outside this batch.
