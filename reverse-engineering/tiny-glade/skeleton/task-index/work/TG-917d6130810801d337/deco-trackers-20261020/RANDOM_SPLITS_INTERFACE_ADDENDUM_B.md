# Shared helper interface addendum

RANDOM_SPLITS_DEPENDENCY_CONTRACT_B.md remains frozen at b3308a526870d3b8ff564b2fc515632edb0f44248e52e6a6f25419638a43429c.

Its `&mut Rng` spelling was a proposed caller interface, not a native/PDB proof of original Rust reference mutability. B ABI proves a Rng pointer and public fastrand f32 calls; it cannot distinguish `&Rng` from `&mut Rng` source spelling. The shared utils owner (04, TG-1d8b7844066af51d6b) may expose `(count, jitter, rng: &fastrand::Rng)` and corresponding into callback using pinned fastrand1.8.0's actual receiver contract. Mutable caller references coerce normally. This does not establish unsafe Sync or permit extending lifetimes.

The count, output append, float ordering, NaN/min, 0.495 coefficient and track-caller Location evidence remain unchanged. Shared implementation is restored once by04; this owner does not write utils or create a duplicate local helper.
