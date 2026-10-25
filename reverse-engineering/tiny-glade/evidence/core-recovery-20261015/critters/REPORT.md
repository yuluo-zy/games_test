# B sheep control implementation

- Original RNE SHA256: f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. `procedures.json`, `goal-procedures.json` retain exact PDB ownership.
- Serializer 1413ff520 and Debug 1416f2800 prove Idle{timeout_s} / Walking{timeout_s,goal_change_timeout,force_reach_goal}. No fabricated defaults.
- 1416f9eb0 writes position, goal, move_state; goal serialize_entry is Option<Vec3>. 1416f9fd0 writes sheep. Derived serde reuses the dependency.
- Capture 1412f064f..66f copies Transform.translation, CritterGoal and Sheep+30 movement state. Restore 1416f24f0 zips query order and stops at the shorter input; rotation, scale and other Sheep fields are untouched.
- Implemented resources/critters/sheep/control.rs capture/restore over borrowed fields. This is not a replacement whole Sheep component or a completed live sheep lifecycle. Full Sheep schema and spawn/movement/save query integration remain open.
- Claude snapshot-analysis returned a usable bounded schema analysis, checked against native instructions and PDB types. Independent wall agent review found no schema/copy/restore discrepancy.
- 2026-10-15: 2 focused control tests passed (wire variants and capture/restore preservation). Original full save/load acceptance not claimed.
