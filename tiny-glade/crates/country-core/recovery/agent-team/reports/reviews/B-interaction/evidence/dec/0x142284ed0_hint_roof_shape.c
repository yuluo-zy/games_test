
void __cdecl system_onboarding::hint_roof_shape::hint_roof_shape(void)

{
  longlong lVar1;
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  uint uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  char *pcVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  longlong lVar12;
  char extraout_AL;
  longlong lVar13;
  ulonglong extraout_RAX;
  ulonglong uVar14;
  undefined8 *in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  longlong *in_RDX;
  longlong lVar17;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong *in_stack_00000028;
  longlong *in_stack_00000050;
  int local_d8;
  int local_60;
  
  puVar6 = (ulonglong *)*in_R9;
  lVar7 = in_R9[1];
  uVar14 = *puVar6;
  uVar18 = 0;
  uVar15 = uVar14 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar15 = uVar18;
  }
  uVar19 = uVar14 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar19 = uVar18;
  }
  lVar13 = uVar15 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar14 = *(ulonglong *)(lVar7 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar15) {
    lVar13 = 8;
    uVar14 = uVar18;
  }
  lVar16 = uVar19 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar15 = *(ulonglong *)(lVar7 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar19) {
    lVar16 = 8;
    uVar15 = uVar18;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar15) - uVar14;
  lVar20 = uVar14 * 0x10 + lVar13;
  lVar17 = uVar15 * 0x10 + lVar16;
  uVar3 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  pcVar8 = (char *)*in_R8;
  puVar9 = (undefined4 *)in_R8[2];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar7 = *in_stack_00000028;
  puVar10 = (undefined4 *)in_stack_00000028[2];
  do {
    while ((lVar13 == 0 || (lVar13 == lVar20))) {
      if ((lVar16 == 0) || (lVar16 == lVar17)) {
        if ((((*(char *)(*in_RDX + 0x1ff) == '\0') && (*pcVar8 == '\x01')) && (pcVar8[1] != '\x01'))
           && ((((*(int *)*in_RCX == 0 &&
                 (country_core::systems::collision::world::RaycastWorld::legacy_raycast(),
                 local_60 != 2)) &&
                (country_core::systems::ui_edit_focus::geo_src_to_edit_subject(), local_d8 == 2)) &&
               (country_core::resources::walls::public_walls::PublicWalls::roof_entity(),
               extraout_RAX >> 0x20 != 0)))) {
          uVar14 = extraout_RAX & 0xffffffff;
          if ((uVar14 < *(ulonglong *)(in_stack_00000050[1] + 0x10)) &&
             (lVar7 = *(longlong *)(in_stack_00000050[1] + 8),
             *(int *)(lVar7 + uVar14 * 0x14) == (int)(extraout_RAX >> 0x20))) {
            uVar5 = *(uint *)(lVar7 + uVar14 * 0x14 + 4);
            uVar14 = (ulonglong)uVar5;
            if ((uVar14 != 0xffffffff) &&
               (((uVar14 < *(ulonglong *)(*in_stack_00000050 + 0x38) &&
                 ((*(ulonglong *)
                    (*(longlong *)(*in_stack_00000050 + 0x28) + (ulonglong)(uVar5 >> 6) * 8) >>
                   (uVar14 & 0x3f) & 1) != 0)) &&
                (country_core::resources::roofs::roof_shape::Roof::is_flat(), extraout_AL == '\0')))
               ) {
              make_lbm_drag_binding_hint();
              bevy_ecs::event::writer::EventWriter::
              write<system_onboarding::hint_orchestrator::DisplayHintRequest>();
            }
          }
        }
        return;
      }
      lVar1 = lVar16 + 0x10;
      lVar13 = 0;
      *puVar6 = *puVar6 + 1;
      cVar2 = *(char *)(lVar16 + 8);
      *puVar9 = uVar3;
      lVar16 = lVar1;
      if (cVar2 == '\0') goto LAB_142284fb0;
LAB_142285007:
      pcVar8[1] = '\x01';
      *puVar10 = uVar4;
      lVar1 = *(longlong *)(lVar7 + 0x30);
      uVar11 = *(undefined8 *)(lVar7 + 0x40);
      if (lVar1 == *(longlong *)(lVar7 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar12 = *(longlong *)(lVar7 + 0x28);
      *(undefined8 *)(lVar12 + lVar1 * 0x10) = uVar11;
      *(undefined1 *)(lVar12 + 8 + lVar1 * 0x10) = 7;
      *(longlong *)(lVar7 + 0x30) = lVar1 + 1;
      *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
    }
    lVar1 = lVar13 + 0x10;
    *puVar6 = *puVar6 + 1;
    cVar2 = *(char *)(lVar13 + 8);
    *puVar9 = uVar3;
    lVar13 = lVar1;
    if (cVar2 != '\0') goto LAB_142285007;
LAB_142284fb0:
    *pcVar8 = '\x01';
  } while( true );
}

