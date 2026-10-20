# Complete photomode toolbar producer

Original binary B SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

* setup_photomode_toolbar: 1426246e0, PDB size 9053; executable edition-tree construction captured at 14262472d..142625ad8 for demo=false/true.
* All 11 category records, every nested row/column, exact labels, sizes, show_only_fpv flags and every custom enum payload retained in photomode-original-trees.json.
* Child custom vtable names prove PhotomodeButton closure (PDB function142623540), not inferred from icon labels.
* Root callback dispatch 142623ff4..1426242ee executed without hooks: photo-category-callbacks.json captures all22edition/category cases and original deferred command function pointers/payloads. Type identities resolved from procs.tsv.
* Root source 142623cb0 inserts common ShowOnlyInPhotomode/cooldown/expand/hide; existing generic factory reused for geometry and common lifetime. Only take-picture category17 has Some(1.25), base137.5; others110.
* FOV/DOF/EV/Artistic root UiLocked is present in both editions. LUT demo branch deliberately inserts neither action nor lock. Children still created as original. No guessed repair to original behavior.
* Root callback writes:11TimeOfDayButtonV2;12Photo4;13Photo5;15Photo15;17Photo9;19Photo11(full);21Photo17. All other photo roots locked.
* Production uses owned ToolbarItem trees/closures and existing real Taffy/Bevy factory, with all11menu focus parents and cyclic nav from actual Entity bits. No old raw union runtime added.

Central Cargo validation passed (parent log): complete original-tree snapshots including executed real ECS custom callbacks, plus both editions × four alignments root payload/markers/sizes/nav/menu focus/slider owner/gated-leaf tests. Native capture hooks only allocator/memcpy/temporary storage drop in tree construction; callback dispatch needs no hooks. This is producer restoration; full photomode_buttons external screenshot/platform branches are separate consumer work, not claimed complete by this setup.

Production frozen SHA256 setup_photomode_toolbar.rs 130a03d0965482a45d9feb6280e46b1ea77ce1e7e3b70e2758e8a9d2744201c4.
Test newline-normalized by central coordinator; final SHA256 setup_photomode_toolbar_tests.rs 33ee4534f012f7114cae2567d49767e9e68d71952ba86699daf201aca3695aa6.

