# B steepness-hint consumer candidate

Exclusive owner terrain-hints-20261018. Scope: existing hint_terrain_steepness reads
NotifyHintSystemTerrainSteepness.tag instead of seam0; replaces Query<Entity>
with the PDB-bound existing HoveredTerrainStrokeId.tag gate. Tag0 sets first
condition byte, every nonzero tag sets second byte and emits completion4.
Existing condition/API/display builder is retained. Second state byte is not
an extra event boolean.

Shared full contract and42 original sequence/9 gate fixtures:
../../TG-004776afc6745c08bc/terrain-hints-20261018/CONTRACT.md.
Original0x142283640 bytes retained in original-hint-terrain-steepness.asm.
Schema owner supplies NotifyHintSystemTerrainSteepness {tag:u8}. Root integrates
and compiles; no production edit or compilation was made by this actor.
