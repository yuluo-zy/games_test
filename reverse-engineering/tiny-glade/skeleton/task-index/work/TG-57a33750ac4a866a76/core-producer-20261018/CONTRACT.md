# B canonical dormer storage and actual roof consumer

Owner core-producer-20261018; delegated root claim TG-57a33750ac4a866a76. Only work candidates were written. Target B original PE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911.

## Original evidence

Four complete PDB ranges were independently re-decoded from the PE and every Capstone address/instruction checked against the provided original assembly. Typed constructors 14161a760/451B and1416159e0/451B; add_new wrappers1412bf170/627B and1412c3f80/627B. Total511 instructions/2156B; raw SHA and RIP strings in evidence/native-verification.json. These are static verification, not native execution. The constructor ranges include their PDB exception code.

DormerHoleSsbo constructor: r9 row count<=11250; size0x83d60=540000 (11250x48); byte data size computed rows*48; NAME Dormer Holes from142c8014b length12. Count constructor: r9<=225; size0x384=900 (225x4); bytes rows*4; NAME Dormer Hole Count from142c7f86a length17. Both BufferDesc usage0x22,memory1,alignment4; absent source ptr passes None to create_buffer rather than allocating/uploading zeros. Some data retains its exact supplied byte length. Device failure reaches original unwrap panic.

The add_new wrappers call those typed constructors with null source, write instance_count0 after the call, allocate a48B InstanceBuffer and register it. Original full type names are exactly country_core::resources::walls::dormers::DormerHoleSsbo (55B) and country_core::resources::walls::dormers::DormerHoleCountPerCellSsbo (67B); their RIP source pointers and contents were independently verified. Original NAME hashes0x13c9c799697a906c and0x96675839a35d7e8f match existing draw-reader slots. Some-row InstanceBuffer counting reuses the already recovered InstanceBuffer contract.

## Current design adaptation

Add the canonical module beneath existing resources::walls. DormerHoleSsbo is repr-transparent twelve rawu32 words:48B/alignment4. Record field meaning was not established by this bounded constructor evidence, so no semantic CPU fields or fabricated defaults are introduced. Count is repr-transparentu32,4B/alignment4. Both are Pod/Zeroable to permit existing library byte upload; StorageElement::default_value remains None.

LibrarySsbo Storage is InstanceBuffer<Self>. A private adapter for these two types uses existing TypedStorageBuffer::new_storage_buffer fixed-byte policy, which already checks stride, pointer4alignment and capacity, builds size fixed, usage0x22/memory1/alignment4, and preserves None without upload. It publishes initial-byte-count/row-size or0. This does not call InstanceBuffer::new_storage_buffer(None), whose current generic Typed::new(None) would zero-upload; no shared generic None policy is modified.

Pass resource SsboType hash derives from the canonical library SsboId. Shared draw ssbo_tags reexports the two canonical types under existing DormerHoleTag/DormerHoleCountTag API names and implements DrawSsbo with the original slots and full names. Existing roof draw callbacks are reused without replacement. No parallel tags/storage owner remains for these two identities.

## Validation sources and bounds

Four test sources: actual library None creation validates fixed sizes/full descriptors/no upload/count0/canonical alias identity+hash; actual library Some preserves exact raw bits/count2/fixedcapacity; both capacities accept their final row and reject overflow/badstride/unaligned slices before device creation; actual existing instanced roof shader reader queries library-derived leases with full canonical type-name checks and shared RenderGraph import cache, then submits its draw packet. That test uses current Rig only for unrelated textures/reflection/other buffers, not dormer storage. Its two imports are observed real library owners701/702. These are CPU contract tests; no GPU execution or mapped completion is asserted.

Rustfmt parsed the new module and shared tags. Both patches apply in sequence to isolated captured bases and produce every candidate byte-for-byte. Cargo was not run; root owns compilation and tests. No original executable probes were attempted, including no retry after usage-limit approval rejection.

This bounded change does not restore the missing dormer geometry producer, GPU shader semantics of the48B record, whole roof pipeline, or all72 startup producers. Startup must register these canonical owners from the existing producer orchestration; that registration belongs to root. The current canonical factory must not be counted as a complete dormer business module.
