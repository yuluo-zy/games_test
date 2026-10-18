# Snapshot serializer identity correction

The frozen69 native pack descriptive filenames bucket-serialize-value and snapshot-serialize-value are reversed. Use original types/callees, not these labels. No frozen inputs or candidate changed.

- 0x141560e90/source1234 passes key142c6e780/len6 (states) to1415946b0, whose exact PDB type is Serialize_field<Vec<(DecoratorStorageAddr,DecoratorAddrSnapshot)>>. It is the whole StorageSnapshot serializer.
- 0x141560fc0/source1245 passes key142c6e7ad/len5 (state) to1415997d0, whose exact PDB type is Serialize_field<Vec<(DecoratorId,DecoratorHistoryState)>>. It is the per-address bucket serializer.
- Typed to_value caller1415a4870 serializes tuple<(Addr,AddrSnapshot)>, calls Addr serializer141560650 and bucket141560fc0, independently confirming the inner identity.

Original native JSON: {states:[[addr,{state:[[id,historyState],...]}],...]}. Addr wire is {Wall:u64}, Terrain unit string, Stairs plural unit string. Exact source method field names in reconstructed Rust can remain states; the save owner provides original wire adapter. All proof static only, no original or Rust test execution.
