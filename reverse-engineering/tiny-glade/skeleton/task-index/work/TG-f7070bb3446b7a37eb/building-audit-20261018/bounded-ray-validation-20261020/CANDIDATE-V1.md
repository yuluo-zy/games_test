# Bounded-ray candidate v1 — source review pending

Root accepted/applied overlap world f6cfabe02812b4e166f6141121ea48f5155907fa9c022d596268a0aebabe18ae (collision-query-integration-v1.json); this candidate is generated from that exact live source. It preserves the overlap hunk and all its tests byte-for-byte. Bounded design passed scoped review7738b14c93c4706ff4593237396c3d0915759ab4124b37435cc7ebef18118faa.

Only native World max_toi API, private helper float propagation, the two explicit MAX compatibility callsites, and four actual Rapier tests change. No nonphysics finite false stub, no implicit flush, no other physics/policy/API changes. The existing nonphysics compatibility stubs remain outside this bounded implementation.04 owns the incoming real cloth/flag call using original unit direction/distance-0.01/mask0x30db.

No Cargo/test/native target/GPU was run by the author. Root alone applies/builds/runs after source review. Suggested focused checks are bounded_ray_regressions and the already integrated decorator_overlap_regressions plus current physics regressions. This candidate does not establish upper consumer or game acceptance.
