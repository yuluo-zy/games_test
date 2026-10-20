# B reverse-engineering lessons from independent audit and archival restoration

1. Native call arguments and values decide behavior. sync_channel EDX1 is a
   positive1-capacity channel; deadline niche1e9 selects None, not1second. Reuse
   std's channel instead of reconstructing its queue. Test before/after completion,
   cancellation, disconnection and one-consumer ownership with actual current files.
2. Follow every control-flow successor, including cold blocks outside a short
   window. The window migration's additional Decorators loop is reached through
   f6d8, and was missed by reading only ec9c..ee0a. A local absence is not a proof
   of no producer. Record and withdraw disproved findings promptly.
3. Read the source and returned value of a discriminator. Restore compares the
   returned HashMap::insert Option niche2, not incoming DecoratorInfo.state.
   PDB calls named clear<T> are not clear_tracking; drop_inner_table is not a
   history flush. Incorrect wrapper names must not become algorithm evidence.
4. Separate physical Rapier handles from global SlotMap mesh keys. Two subworlds
   can both have physical index0. Mesh registry user_data + exact generation,
   actual RaycastMesh Arc, shape and transform must survive live collider deletion.
   Compare archived Arc identity in a nonempty two-channel/multiwall test.
5. Reuse the dependency owner and audit its boundary behavior. slotmap1.0.6 and
   current1.1.1 both normalize FFI version|1, so exact native generation validation
   must precede KeyData::from_ffi. Preserve newer user baseline until central
   dependency audit, rather than blindly downgrading or inventing a side table.
6. Restore whole lifecycle and address semantics. Per-wall, terrain and stair
   archives are independent before buckets, not cross-address before/after
   overlays. Follow mark_mutable -> event gate -> clear/d2s -> authoritative wall
   existence -> item insertion -> both collision channels -> stairs -> extract/drop.
   Fast paths and partial writes before panic are observable contracts too.
7. Keep proof levels explicit. Independent original audit, author candidate,
   formatting, metadata type-check, actual tests and whole-app interaction are
   different gates. No PDB physical producer after a complete bounded scan still
   does not prove no anonymous/inlined writer without additional evidence.
8. Give one owner production integration/Cargo. Other agents can produce scoped
   evidence and candidate patches against declared base hashes. New source must
   be independently reviewed and exercised with real nonempty state, not merely
   satisfy tests that mirror a reduced implementation.
9. Freeze coherent source before central application. A document-only script
   incorrectly used the trailing pub mod changes declaration as a header boundary
   and truncated a candidate. Production remained untouched; the task's exact
   previous tool patches restored it. Keep backups, use the actual first code
   declaration, assert file/function counts, normalize diff line endings and
   roundtrip the exact patch to proposed bytes. frozen-v2 records these checks.
10. Do not fill a recovered type with imagined fields because a consumer seems to
    need them. Original AddrHistoryState is a48B map; a current extra removed set
    lacks native production evidence and needs a field/serializer audit. Similarly,
    omitted RaycastGeoSource payloads cannot be replaced by Arc pointers or tags
    while claiming original source identity/equality.
