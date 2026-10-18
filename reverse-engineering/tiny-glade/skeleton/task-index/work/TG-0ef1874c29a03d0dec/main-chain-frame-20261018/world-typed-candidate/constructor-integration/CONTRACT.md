# Boat / Stairs owned-delivery integration

Candidates only. These narrow patches reuse the already legally owned OS source deliveries; original claims remain untouched. The current production source is retained as exact before snapshots. No full-file stale replacement or change to current spawn/stairs algorithms is included. Tests reuse the existing ssbo MockDevice, not a new backend. No Cargo or production edits were made.

Boat is64B×4096/0x40000/NAME Boats/hash4f69c51e83e1d670, descriptor usage22/memory1/alignment16, InstanceBuffer with count0 on None; None forwards no payload. Some preserves actual bytes and element count. Original GPU BoatXform stride48 is a separate fact and does not shrink the native64B creation allocation. Unobserved CPU words remain opaque.

Stairs is48B×41000/0x1e0780/NAME Stairs transforms/hashc405372d6efc32d3, descriptor usage22/memory1/alignment4, TypedStorageBuffer without independent instance count; None forwards no payload. Some uses the existing fixed byte constructor, preserving length%48/pointer4/capacity checks. Canonical existing Affine3Packed supplies the48B record. The two original stairs passes and their real StairsCounter reference are unchanged; original counter producer is another owner responsibility.

The original full constructors/add_new wrappers and precise source hashes are in the independent remaining-seven-constructors pack. These patches establish fixed creation owners only; neither boat GPU spawn nor real stair terrain/input production is inferred from successful construction.
