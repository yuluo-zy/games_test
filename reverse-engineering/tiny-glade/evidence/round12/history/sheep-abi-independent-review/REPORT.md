# Independent SheepControl ABI and snapshot review

Read-only production review. No Cargo or core compilation. The implementation owner is independent_ui_acceptance; this reviewer did not author its candidate. Fresh original RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. `original-byte-evidence.json` binds9 complete original ranges, raw bytes, hashes and exact PDB aliases; `.original.asm` files preserve disassembly. PDB's void() placeholders were not used as ABI.

## Second Option snapshot is closed by its typed producer

Runtime1416f166f creates pointer SheepControl+0x18;1416f16a7 stores it in outer GUI closure environment+0x30. The control's base pointer goes to environment+0x28 at1416f16a3. The nested GUI1416ee410 copies outer environment+0x28..0x3f into a three-pointer snapshot closure at1416ee52c..556. Therefore child environment+8 is **SheepControl+0x18**, not a selected Entity vector or pick-point vector.

The snapshot GUI1416eeec0 uses original string `Take snapshot` at142c90480,length13. Its clicked branch calls141304bd0 at1416eefed. That callee's exact PDB specialization returns `Vec<country_core::resources::critters::sheep::control::SheepSnapshot>` from the actual `(Entity,&mut Transform,&mut Sheep,&mut CritterGoal)` query mapped through capture_sheep_snapshot::closure_env$0. This establishes the business element type independently of size inference. Then1416eeff2 reads child environment+8,1416ef003..015 frees the previous buffer at capacity×40/alignment4 when allocated, and1416ef01a..02c replaces the full Vec header at the second Option snapshot. Native SheepControl drop140891e40/99B independently frees both first and second fields with the same40-byte stride/alignment4.

Thus the second field is the **manual Option<SheepControlSnapshot>**, with native24-byte Option<Vec<SheepSnapshot>> ownership; the first field is the independent frozen snapshot. Both reuse the canonical snapshot/row type. Do not replace the second with Vec<Entity>, a12-byte position row, an opaque byte buffer or a boolean.

## Restore also synchronizes an existing freeze

On `Restore snapshot` (142c9048d,length16),1416ef0d9..0e0 passes the actual query and manual second-snapshot pointer to1416f24f0, exact PDB `debug::sheep_controls::apply_sheep_snapshot`. Then1416ef0e5 loads child environment+0, the first frozen Option snapshot;1416ef0e8..0ee skips cloning when it is None. When already frozen,1416ef0f4..15c allocates manual length×40/alignment4 and1416ef170..1d4 clones typed row fields;1416ef246..279 frees/replaces the first frozen snapshot. Restoring manual data while frozen must update that frozen owner as well, or the next frame restores the old freeze again. Restoring while unfrozen must not turn freeze on.

This review proves the ownership and branch order, not full GUI file save/load correctness. The remaining save/load buttons and original snapshot serialization should remain in the implementation owner's complete reconstruction and tests.

## Picking enum has direct GUI writers

Outer GUI environment+0x10 points to SheepControl+0x48, established at runtime1416f166b/1697. `Set goal` clicked branch1416ee62f..633 writes byte1 through that pointer. `Set position`1416ee6c2..6c6 writes byte2. Runtime reads the byte1416f1e18..22 and skips picking when zero; it checks byte2 at1416f2387/2391 for the position branch, using the other reachable value1 for goal. Successful consumption resets the same byte to0 at1416f2179..2180. Therefore native ABI is **0=None,1=Goal,2=Teleport**, with direct writer evidence, not an inference from runtime's else branch alone. Invalid bytes are not reachable from these GUI producers; don't claim that testing byte255 makes it a valid enum variant.

The selected identity at SheepControl+0x40 is a u64 SheepId, not an Entity. Its GUI button1416ee281..2a5 applies the native wrapping-add modulo3 cycle, producing0→1→2→0. Entity bit patterns must not be substituted into that owner.

## Bounded original execution

`verify-native-abi.py` uses Unicorn to execute actual original instruction slices, not rewritten Rust/model algorithms:

-12 writes: Goal1, Teleport2 and consumed0 from four initial bytes; every other control byte stays unchanged.
-6 selected SheepId cycles, including0/1/2 and wrapping boundaries.
-12 manual snapshot replacements across old Some(empty), allocated capacities1/3 and None, new lengths0/1/3; only the actual allocator free boundary is hooked, recording exact pointer/capacity×40/alignment4. The primary frozen snapshot is byte-for-byte unchanged by taking a manual snapshot.

All30 native fixtures passed and are preserved in `native-abi-fixtures.json`. Scope is strictly those original slices. No actual Rust implementation, Egui clicks, complete Bevy system or save/load execution was run. These fixtures and the typed producer proof are ready for the implementation owner's nonempty regression tests and central acceptance.
