# Dormer-hole scalar provenance V1

This package resolves one input to the seven original 48-byte record writes. It does not restore their visual generators, change the already frozen uploader/constructors, or claim a running scene. All six original producer bodies and the two small helper bodies were checked instruction by instruction against B RNE f507a6ce; the matching PDB is 2d9686b8. No original target, Cargo, GPU or tests were executed.

The original 48-byte record word at byte+0x0c is the low32 bits of the current storage wall key, a WallId. It is not the current DecoratorId. The proof is caller data flow rather than selection of one shared-address symbol name:

| Original writer | Wall-key save | Getter and incoming constructor scalar |
| --- | --- | --- |
| arrow slit | 1407bc347 loads the outer0x98-stride wall-entry key; 1407bc34f saves stack+140 | 1407bd8b3 passes its address; 1407bd8ba calls885d0; return is retained in RSI, then ESI goes to EDX at1407bda04 before gothic ctor1407bda14 |
| trap door | 1407beb3f loads the wall-entry key; 1407beb47 saves stack+188 | 1407bf78a / 1407bf791 get the qword, EAX goes to EDX at1407bf84b before cottage ctor1407bf85b |
| gothic dormer | 1407c5a3d loads the wall-entry key; 1407c5a45 saves stack+188 | 1407c6c86 / 1407c6c8d get it, save stack+170, reload RDX at1407c6cda before gothic ctor1407c6cec; ctor consumes/stores the low32 bits |
| cottage dormer | 1407c7eac loads the wall-entry key; 1407c7eb4 saves stack+100 | 1407c962f / 1407c9636 get it, EAX goes to EDX at1407c978a before cottage ctor1407c979a |
| gothic window | 1407e8b46 loads the wall-entry key; 1407e8b4e saves stack+c8 | 1407ea5b1 / 1407ea5b8 get it, EAX goes to EDX at1407ea6a5 before gothic ctor1407ea6b5 |
| cottage window first | 1407ec3cd loads the wall-entry key; 1407ec3d5 saves stack+1f0 | 1407ed97f / 1407ed986 get it, RAX is retained in RSI, ESI goes to EDX at1407ed9f5 before cottage ctor1407eda01 |
| cottage window second | same wall key and stack+1f0 | 1407edb16 / 1407edb1d get it, EAX goes to EDX at1407edb60 before cottage ctor1407edb6d |

Each outer iteration comes from DecoratorStorage::iter_walls at140a1b100, with the exact PDB record540:65936. The map entry's first qword is copied separately from the Decorators value, whose inner modified-item iterator is then used. Cottage/gothic dormer also pass the same saved key to PublicWalls::roof_entity, independently confirming this is the wall identity. `NATIVE_TRANSFERS.json` preserves the outer iterator-to-key transfer and the complete getter-to-constructor instruction ranges for all seven writes, source hashes, original helper bytes and selected competing aliases.

The getter at1400885d0 is exactly4 bytes `48 8b 01 c3`: load the pointed-to qword and return. PDB aliases include both WallId::get_inner (585:30752) and DecoratorMaybeMut::get (540:19100), plus unrelated getters. Calling it does not itself establish either type; the saved outer key does. The constructor's first scalar is passed through the x64 integer parameter register RDX and stored as EDX. The host equivalent at these callers is therefore `wall.get_inner() as u32`, preserving native truncation rather than a checked conversion, pointer value or DecoratorId conversion.

The already integrated canonical constructors deliberately retain the neutral argument name native_u32_0c. This evidence does not require changing their public signature or reinterpreting arbitrary external callers as WallId. It constrains actual restored producers. The current old generator path still calls the legacy utils::DormerHoleSsbo with a DecoratorId and default geometry, then discards the result. That is a real source gap, not a valid adapter to the canonical48-byte type. Appending that old record, or changing only its ID, would not restore the full producer inputs, conditions, geometry or blueprint output.

Complete retained write-back evidence remains in seven-record-writer-instruction-transfers-v1.json, including the cottage dormer's backward jump from1407c97b4/97ca to1407c8ff0 and the actual three16-byte stores there. This new package proves scalar provenance only. Semantic shader use of this value, other constructor inputs and each producer's complete eligible domain remain separate checks before implementation.
