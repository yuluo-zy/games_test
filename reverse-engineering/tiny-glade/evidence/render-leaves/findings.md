# Private rhapsody resource usage leaves

Target: original Tiny Glade `tiny-glade.rne` / retained byte-identical analysis executable, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. These are recovered behavior functions, not fetched original Rust text.

## Identity and bounds

| Function | Original range (end exclusive) | Identity / range evidence |
|---|---|---|
| `rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags` | `0x140bd4b00..0x140bd4b32` (50 bytes) | PDB module 676 GPROC + public alias, straight-line instructions through RET |
| `rhapsody::rg::resource_info::image_access_mask_to_usage_flags` | `0x140bd49d0..0x140bd4af4` (292 bytes) | PDB module 676 GPROC + matching PE exception-directory RUNTIME_FUNCTION |
| `vk_sync_fork::AccessType::is_write` | entry `0x140c10e10`, straight-line instructions end `0x140c10e1a` | PDB public alias; dependency, excluded from private Rust restoration |

Original file for both private leaves: `crates/rhapsody/src/rg/resource_info.rs`; PDB source checksum SHA256 `0f92cb3e59af073ef66c194a9ebbaafc349625df27209c37cb85962ac5539f4c`. That checksum identifies the author's file but does not reconstruct its contents. GPROC CodeView signature is `void()` type `0x1001`, so typed Rust signatures are behavioral reconstructions, not PDB-supplied original types.

PDB raw evidence: `../render-skeleton/raw/module-00676.txt.gz`. Actual RE-MCP operations including open/create/decompile/xrefs/close: `../render-architecture/mcp-leaf-resource-usage-report.json`. The initial `create_function` reports a one-byte database body for two leaves; those sizes are not trusted. Accurate ranges are corroborated independently above. `native-evidence.json` and adjacent `*-asm.txt` retain the original instructions and public aliases.

## Buffer conversion

The input type is an original renderer access mask. It is not directly an `ash::vk::AccessFlags` bitset. Exact consumed masks and resulting standard Vulkan buffer usage bits:

| Original access bits | Vulkan output usage |
|---|---|
| `0x1` | `INDIRECT_BUFFER` (`0x100`) |
| `0x2` | `INDEX_BUFFER` (`0x40`) |
| `0x4` | `VERTEX_BUFFER` (`0x80`) |
| `0x8` | `UNIFORM_BUFFER` (`0x10`) |
| any of `0x20`, `0x40`, `0x8000`, `0x10000` | `STORAGE_BUFFER` (`0x20`) |
| `0x800` | `TRANSFER_SRC` (`0x1`) |
| `0x1000` | `TRANSFER_DST` (`0x2`) |

Flags union together; all other input bits contribute nothing. This describes the leaf for arbitrary u32 inputs and does not assert that every bit combination is used by the game. Original input flag names and authoritative source type remain unknown.

## Image conversion

This leaf uses exact whole-mask matching, not independently unioned flags. It returns only for these inputs:

| Exact original mask | Vulkan output usage |
|---|---|
| `0x20` | `SAMPLED` (`0x4`) |
| `0x40`, `0x18000` | `STORAGE` (`0x8`) |
| `0x80`, `0x100`, `0x180` | `COLOR_ATTACHMENT` (`0x10`) |
| `0x200`, `0x400`, `0x600` | `DEPTH_STENCIL_ATTACHMENT` (`0x20`) |
| `0x220` | `DEPTH_STENCIL_ATTACHMENT | SAMPLED` (`0x24`) |
| `0x800` | `TRANSFER_SRC` (`0x1`) |
| `0x1000` | `TRANSFER_DST` (`0x2`) |

Every other mask reaches the original Rust panic entry `0x1428d9390` from the call at `0x140bd4aee`. For example `0`, `0x60`, `0x1800`, and `0x8000` panic. Restored Rust preserves panic occurrence and valid output bits; its diagnostic text is newly chosen, not recovered original formatting.

## Public dependency boundary

`0x140c10e10` is positively identified as `vk_sync_fork::AccessType::is_write::hf4d56949aed2e203`, not a private rhapsody predicate. Original instruction behavior on the low byte is wrapping subtraction by 40 followed by unsigned comparison with 16: true exactly for byte discriminants `40..=55`. We record it to validate the caller's write-class boundary and do not recreate this dependency. Original lock lists local/path `vk-sync-fork 0.4.0`; exact public upstream provenance and enum variant names require further confirmation.

## Verification and limits

`verification.json` records comparison of compiled Rust with original machine instructions in Unicorn. The buffer coverage includes all 1024 combinations of its ten consumed bits, all 32 single bits, full32 boundaries, and 5000 random u32 values. Image coverage exhausts all 131072 low17 masks and additionally exercises high bits and 5000 random u32 inputs. The original panic entry is observed and stops emulation; no conversion core is stubbed. The public dependency predicate is observed for all 256 byte values as a fixture only.

The Rust module directly uses original ash `0.38.0+1.3.281` output types/constants. A separately pinned renderer crate permits compilation without changing the established geometry reconstruction lockfile. Verification does not execute the game or any Vulkan driver/device and cannot establish render-graph integration, resource lifetime, synchronization correctness, image quality, or full renderer completion.
