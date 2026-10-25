# R6A device counters and queue construction

Original target SHA-256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. `rhapsody::raw::device::GraphicsDevice::create` is `0x140bf2560`, PDB procedure span `0x140bf2560..0x140bf84c0`. Retained decompile and MCP JSON are observations, not original Rust source. REA retrieval closed the persistent database with `save:false` successfully; no annotations or database saves were requested.

## Recovered initialization

The constructor chooses the first family whose flags have bit 0 set for the primary graphics queue (`0x140bf2c3d`). It also chooses the first family whose low two flag bits equal 2 for auxiliary queues (`0x140bf2e15..25`), i.e. compute capability without graphics. This second family may include other flags. A copied family record is 28 bytes: flags at 0, queue count at 4, and family index at 24. If no auxiliary family exists, the auxiliary count is zero; otherwise it is that family's queue count (`0x140bf3068` onwards).

At `0x140bf600e..40` the constructor builds `auxiliary_count + 1` counters. The compiled initializer instance `0x1418b7510` writes **1 to every contiguous 8-byte element** (scalar `0x1418b75f0`, SIMD constant `0x142ae0d00 = [1,1]`). No queue-dependent seed, pointer element, stride/interleave, or shared atomic is present. At `0x140bf6195`, `0x1418c9520` transfers the vector to a pointer/length pair; `0x140bf6628` writes the pointer to device `+0x668`, and `0x140bf6636` writes length to `+0x670`. Generic vector/box conversion mechanics are evidence for these use sites only; the Rust candidate reuses standard collections and atomics.

The primary queue is `{tag:0}` and begins with empty pending deques, sequential-disposal prefix 0, cached completed timeline 0. Auxiliary queue producer `0x1418b48f0` retrieves raw queue `(auxiliary_family_index, n)`, then writes queue `+0x68=1`, `+0x6c=n`, disposal prefix `+0x60=0`, cached completion `+0x80=0`, family index `+0xa8`, and a separate timeline semaphore at `+0x88`. Auxiliary identity index **is the queue ordinal within the auxiliary family**, not the family index. Device `+0x7f8` global submission timeline starts at 1: local `rbp+0x3220=1` at `0x140bf6150` is copied by `0x140bf6367` from `rbp+0x3140` to device `+0x718`, mapping source delta `0xe0` to `+0x7f8`.

## Connection to R5B buckets

`CommandBuffer::begin` (`0x140bed2d8..319`) uses slot 0 for an even queue tag, otherwise the **zero-extended wrapping u32** `aux_index+1`. It atomically adds 1 and stores the old value at CB `+8`. Thus primary, auxiliary queue 0, and auxiliary queue 1 all emit sequence 1 on their first successful begins. These are independent per-queue counters, **not globally unique sequences**.

R3/R5B observed a global device `+0xa50` bucket map keyed only by this `u64`, and `insert` replaces the old bucket before asserting that none existed. Therefore two live recording sequences from different queues with the same numeric value collide. This disproves a shared-global-atomic or queue-packed-sequence interpretation of this target. It does not prove that the game actually reaches the unsafe simultaneous multiqueue use: private executor queue selection, use restrictions, and other caller invariants remain unknown. Even serialized reuse cannot establish safety for a resource shared across queues merely from numeric `max(sequence)` tracking.

## Candidate and validation

`device-counters-r6` is an independent crate for family selection, original all-one per-queue counter initialization, atomic fetch-add, and global bucket-key collision witnesses. It contains no Vulkan implementation. `BucketKeyWitness` preserves replacement-before-assert side effect for analysis, not an invented production fix. Rust atomic ordering is a candidate choice (`Relaxed`); x86 locked `xadd` does not recover the original source-level memory ordering.

Six tests and strict Clippy pass. A native differential executes the untouched original CPU instruction slice `0x140bed2d8` until `0x140bed31d` or bounds failure `0x140bed3a8`: **2,840 cases, zero mismatches**. It verifies selected slot, old sequence, wrapping increment, CB tag and queue fields. Vulkan begin/reset, entry-state checks, bucket insertion, panic unwinding, concurrent races, device allocation, and complete device construction are excluded. Raw edge representations do not establish safe caller reachability.
