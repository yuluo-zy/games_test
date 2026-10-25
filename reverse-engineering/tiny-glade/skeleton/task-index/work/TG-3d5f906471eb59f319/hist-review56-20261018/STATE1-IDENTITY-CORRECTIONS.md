# Corrections to the frozen fourth-batch state1 review

The previous state1 numerical probe remains valid. Its historical UI/placement
labels do not identify the native owners. Keep all original frozen artifacts;
this note explicitly withdraws the following mistaken identity interpretations.

1. Original ui_draw stack frame+0x128 is the eighth function parameter,
   ResMut<DecoratorArchivist>. UiState is the sixth parameter at+0x118. The
   original prologue pushes64 bytes, allocates0x128 and sets rbp=rsp+0x80, so
   rbp=entry_rsp-0xe8 and entry argument5 becomes rbp+0x110. Therefore
   0x140aa053b..0x140aa0547 dereferences DecoratorArchivist and sets its+0x1e0
   culling.allow_any, not a terrain UI-active flag. Current
   LiveDecoratorArchivist::decorator_cull_allow_any is the existing correct owner.
   The fourth-batch TerrainEditUiState.active write must be removed. The separate
   original TerrainEditingData active flag at0x140aa0117 remains valid.
2. The state1 last parameter at frame+0x160 is PlayTerrainSound. The original
  40-byte event tag5 carries XYZ and the desired height before clamp (xmm7), while
   the stroke receives clamped xmm0. The old stairs identity and clamped event
   value are rejected. The independently recaptured36 payloads in the previous
   review directory retain exact raw evidence.
3. ui_edit hover0x140aef3b4..0x140aef401 writes Terrain notification tag1 via its
   arg12 tuple's first writer, on valid hoveredID and cursor.tag==1. Its release
   frame+0x50 port is CursorCaptureCmd, not Terrain. The tuple's+0x20 writer is
   NotifyHintSystemTerrainSteepness; release sends0 and horizontal accumulated
   change>0.3 sends1. The prior statement assigning Terrain tag1 to release is
   withdrawn. Terrain notifications have only the observed first-byte tag; the
   second byte read in hints belongs to hint-state resources, not an event bool.

Root and the terrain author were notified immediately upon each independent
correction. These corrections do not invalidate original instruction execution,
stroke-height bits, typed mapped storage or typed pending-history facts. They
prevent treating byte-level fixture success as acceptance of a guessed consumer.
