# Independent review: B rocky canonical resource147

Accepted as the bounded resource/constructor delta after the original signed-field correction. No remaining blocking finding in this scope. Candidate has not been integrated and Rust tests have not been run by this reviewer. This is not acceptance of the full Startup or W4/W5/GPU chain.

## Identity and independent validation

Target B PE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Upstream frozen147 SHAe5b66476ec87a38c434f3a4a429fa0516af4e64ac09a2f54c01bc75657bd7d22. All147 input hashes/sizes checked. All29 original ranges30002B/7082 instructions were re-decoded from the PE; raw hashes, individual instruction bytes, addresses and disassembly texts match. Both key switch tables and16 typed call targets match their original PE bytes. Candidate patch applies to an isolated copy of frozen base and produces candidate.rs byte-for-byte. final-verification.json retains details. No native execution/JIT, Cargo, production edits or cross-chat writes occurred.

The actual30,998,513-byte original asset SHA0f63a7250df5a3382017dbdc3652748f33899750e3a65c5aef3727415b9183b7 was independently parsed/packed by copying the author's inspection script into this review directory. The signed version checks every row and matches all source fixtures and both packed hashes exactly:7142112 vertex bytes d91d4d1d4422a7552974e21da127a7a671b2c64771c17cb0e2f58c8682e1499c;1587136 triangle bytes dbbbb6f464fc08f71ed6cf013c4e5a25ea2b7b5d994b00b2e2a3703d87996a1d. This Python execution inspects data only, not the original machine code or Rust candidate.

## Field finding and correction

The earlier unfrozen draft's threeu32 DTO fields were not justified by the all-positive shipped asset. The reviewer requested direct key-to-source-offset binding. The author supplied the original from_str→root→VecVisitor→next_element_seed→record visitor chain and corrected all three to i32 before freeze. This finding is closed by the final147 candidate and its signed/overflow regression source.

Independently read original key visitor1417c3650 yields position0,bby1,cell_id2,dir3,is_corner4. Table142c5dbe8 resolves to c497,c5f3,c545,c59c,c4ee,c64a. Thus cell_id2 enters14142c545→c696, callc6ab→14140d270 i32, local-2c and payload+16; is_corner4 entersc4ee→c660, callc675→samei32, local-24 and payload+32. position/dir call141413820 f32[3], bby calls14140cfd0 f32. Original success locals/Result+4 payload handling were checked, including the four-byte Result discriminant rather than mistaking it for source data. Triangle key neighbours at142ce2039 maps to i32[3] parser141413560; is_top at142ce2043 maps to i32 parser14140d270 and payload+12. LLVM independently symbolizes the scalar helpers asi32/f32; exact array-generic PDB identities are retained in the frozen original calls metadata.

Current serde derives use these proven fields; rawu32 is introduced only at the GPU transport cast. Negative-1/i32MIN acceptance and2147483648/4294967295 rejection tests have distinct JSON objects/keys, so rejection is not accidentally caused by duplicate-key errors. Integer-domain fixes preserve actual shipped asset packed hashes. No original serde dependency was reimplemented.

## Packing, storage and count

Load1408bc090..0d4 copies the36-byte source into48-byte GPU rows with the exact duplicateposition, bby, cell_id, dir, corner order; both signed integers are preserved as raw words. Triangle1408bc1b0..238 copies four signed words and explicitly zeroes16 trailing bytes in even pairs and odd tail. Candidate packers match these stores and preserve float bits including negativezero/NaN in direct-record tests.

Typed factories141611ba0/579B and1416144a0/531B prove48x148794 and32x49598 fixed capacities, sizes0x6cfae0/0x1837c0, names Rocky Terrain Vertices/Triangles, usage0x22/memory1/alignment4. Some validates byte stride then pointer4 then capacity; None passes no initial upload. Existing fixed-byte TypedStorageBuffer factory is reused unchanged. Library registration remains vertex creation/upload first at1408bc0f8, then triangle packing/upload. Partial failure therefore retains the same earlier vertex registration side effect; no private library or second buffer identity was added.

Original successful load returns vertex count, and the original startup command thunk inserts one8-byte RockyTerrainMeshVertCount. The candidate preserves the current inspectable CPU mesh/load API and adds the canonical count/into_vertex_count() adapter without inventing a default. triangle_groups is explicitly a compatibility derived count/3, not the original resource or an alternative count owner.

## Registration and acceptance limits

Original actual loader identity is the plain name bound by14176a423 inside14176a370. The eight-tuple at14099b8b0 calls this helper as slot4 (14099baad); exact PDB-typed Startup helper140b67b50 calls that tuple at140b67bda, invoked from14094117f. The earlier PreStartup ten-tuple is a different branch and does not establish this loader's stage. STARTUP-OWNER-HANDOFF.md correctly keeps this distinction and preserves the current explicit asset-root Path API.

The original resource constructor can now be reviewed/integrated independently, but actual Startup must still call it with the host-resolved original path and shared device/library, propagate load failure visibly, insert its count via Commands, and feed both current W4 displacement(count/3 once) and W5 drawing(fullcount low32). Canonical pass aliases must reference these current types. Those owner-controlled adapters are not supplied or certified by this resource-only patch. No default count, mesh-layout cast, private library, fake asset or copied serde engine is authorized by this acceptance.

Five proposed Rust tests cover signed schema/overflow, raw GPU packing, exact None construction, invalid data before device call, and the actual asset through real AssetSsboLibrary. Their API/assertions were reviewed but not run. GPU shader behavior, upload completion/synchronization, whole startup_terrain5382B and whole72producer/frame closure remain explicit boundaries.
