
void __cdecl system_onboarding::hint_deselect_tool::hint_deselect_tool(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined4 *puVar6;
  undefined8 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  char extraout_AL;
  char extraout_AL_00;
  int iVar12;
  undefined8 *in_RCX;
  longlong lVar13;
  longlong *in_RDX;
  ulonglong uVar14;
  int *piVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong *in_R8;
  int *piVar19;
  undefined8 *in_R9;
  ulonglong uVar20;
  int *piVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong lVar24;
  int *piVar25;
  longlong *plVar26;
  int *piVar27;
  bool bVar28;
  bool bVar29;
  bool bVar30;
  bool bVar31;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 local_ec;
  undefined8 uStack_e4;
  undefined4 uStack_dc;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  uint6 uStack_5e;
  
  uVar11 = local_98;
  uVar10 = uStack_9c;
  uVar9 = uStack_a0;
  uVar8 = uStack_a4;
  if (*(char *)(*in_R8 + 500) != '\0') {
    return;
  }
  puVar3 = (ulonglong *)*in_stack_00000030;
  lVar4 = in_stack_00000030[1];
  uVar18 = *puVar3;
  uVar22 = 0;
  uVar20 = uVar18 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar20 = uVar22;
  }
  uVar17 = uVar18 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar17 = uVar22;
  }
  uVar18 = *(ulonglong *)(lVar4 + 0x30);
  uVar14 = uVar18 - uVar17;
  if (uVar18 < uVar17) {
    uVar14 = uVar22;
  }
  uVar5 = *(ulonglong *)(lVar4 + 0x10);
  uVar16 = uVar5 - uVar20;
  if (uVar5 < uVar20) {
    uVar16 = uVar22;
  }
  uVar22 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar22 - (uVar16 + uVar14);
  if (uVar5 >= uVar20 && uVar5 - uVar20 != 0 || uVar18 >= uVar17 && uVar18 - uVar17 != 0) {
    *puVar3 = uVar22;
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(undefined1 *)(*in_RDX + 9) = 1;
  }
  lVar4 = in_R9[1];
  uVar18 = *(ulonglong *)*in_R9;
  piVar21 = (int *)0x0;
  piVar27 = (int *)(uVar18 - *(ulonglong *)(lVar4 + 0x18));
  if (uVar18 < *(ulonglong *)(lVar4 + 0x18)) {
    piVar27 = piVar21;
  }
  piVar15 = (int *)(uVar18 - *(ulonglong *)(lVar4 + 0x38));
  if (uVar18 < *(ulonglong *)(lVar4 + 0x38)) {
    piVar15 = piVar21;
  }
  lVar24 = (longlong)piVar27 * 0x28 + *(longlong *)(lVar4 + 8);
  lVar23 = (longlong)piVar15 * 0x28 + *(longlong *)(lVar4 + 0x28);
  bVar28 = *(int **)(lVar4 + 0x10) < piVar27;
  piVar27 = (int *)((longlong)*(int **)(lVar4 + 0x10) - (longlong)piVar27);
  bVar30 = piVar27 == (int *)0x0;
  if (bVar28) {
    piVar27 = piVar21;
  }
  if (bVar28) {
    lVar24 = 8;
  }
  piVar25 = (int *)(lVar24 + -0x28 + (longlong)piVar27 * 0x28);
  if (bVar28 || bVar30) {
    piVar25 = piVar21;
  }
  bVar29 = *(int **)(lVar4 + 0x30) < piVar15;
  piVar15 = (int *)((longlong)*(int **)(lVar4 + 0x30) - (longlong)piVar15);
  bVar31 = piVar15 == (int *)0x0;
  if (bVar29) {
    lVar23 = 8;
    piVar15 = piVar21;
  }
  piVar19 = (int *)(lVar23 + -0x28 + (longlong)piVar15 * 0x28);
  if (bVar29 || bVar31) {
    piVar19 = piVar25;
  }
  piVar27 = (int *)((longlong)piVar15 + (longlong)piVar27);
  if (!bVar28 && !bVar30 || !bVar29 && !bVar31) {
    piVar27 = piVar21;
  }
  *(ulonglong *)*in_R9 = *(longlong *)(lVar4 + 0x40) - (longlong)piVar27;
  if (!bVar28 && !bVar30 || !bVar29 && !bVar31) {
    uVar2 = *(undefined4 *)((longlong)in_RDX + 0x1c);
    puVar6 = (undefined4 *)in_RDX[2];
    *puVar6 = uVar2;
    lVar4 = *in_RDX;
    *(undefined1 *)(lVar4 + 9) = 0;
    puVar3 = (ulonglong *)*in_stack_00000028;
    lVar23 = in_stack_00000028[1];
    uVar18 = *puVar3;
    uVar22 = 0;
    uVar20 = uVar18 - *(ulonglong *)(lVar23 + 0x18);
    if (uVar18 < *(ulonglong *)(lVar23 + 0x18)) {
      uVar20 = uVar22;
    }
    uVar17 = uVar18 - *(ulonglong *)(lVar23 + 0x38);
    if (uVar18 < *(ulonglong *)(lVar23 + 0x38)) {
      uVar17 = uVar22;
    }
    uVar18 = *(ulonglong *)(lVar23 + 0x30);
    uVar14 = uVar18 - uVar17;
    if (uVar18 < uVar17) {
      uVar14 = uVar22;
    }
    uVar5 = *(ulonglong *)(lVar23 + 0x10);
    uVar16 = uVar5 - uVar20;
    if (uVar5 < uVar20) {
      uVar16 = uVar22;
    }
    uVar22 = *(ulonglong *)(lVar23 + 0x40);
    *puVar3 = uVar22 - (uVar16 + uVar14);
    if ((uVar5 >= uVar20 && uVar5 - uVar20 != 0 || uVar18 >= uVar17 && uVar18 - uVar17 != 0) &&
       (*puVar3 = uVar22, *piVar19 == 0)) {
      *puVar6 = uVar2;
      *(undefined1 *)(lVar4 + 8) = 1;
      *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar4 = *in_stack_00000048;
      lVar23 = *(longlong *)(lVar4 + 0x30);
      uVar7 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar23 == *(longlong *)(lVar4 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar24 = *(longlong *)(lVar4 + 0x28);
      *(undefined8 *)(lVar24 + lVar23 * 0x10) = uVar7;
      *(undefined1 *)(lVar24 + 8 + lVar23 * 0x10) = 0;
      *(longlong *)(lVar4 + 0x30) = lVar23 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
    }
  }
  lVar4 = in_stack_00000050[1];
  uVar18 = *(ulonglong *)*in_stack_00000050;
  uVar22 = 0;
  uVar20 = uVar18 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar20 = uVar22;
  }
  uVar17 = uVar18 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar17 = uVar22;
  }
  bVar28 = *(ulonglong *)(lVar4 + 0x30) < uVar17;
  uVar17 = *(ulonglong *)(lVar4 + 0x30) - uVar17;
  uVar18 = uVar17;
  if (bVar28) {
    uVar18 = uVar22;
  }
  bVar30 = *(ulonglong *)(lVar4 + 0x10) < uVar20;
  uVar20 = *(ulonglong *)(lVar4 + 0x10) - uVar20;
  uVar14 = uVar20;
  if (bVar30) {
    uVar14 = uVar22;
  }
  uVar18 = uVar14 + uVar18;
  if (!bVar30 && uVar20 != 0 || !bVar28 && uVar17 != 0) {
    uVar18 = uVar22;
  }
  *(ulonglong *)*in_stack_00000050 = *(longlong *)(lVar4 + 0x40) - uVar18;
  if (!bVar30 && uVar20 != 0 || !bVar28 && uVar17 != 0) {
    plVar26 = (longlong *)*in_RDX;
  }
  else {
    lVar4 = in_stack_00000058[1];
    uVar18 = *(ulonglong *)*in_stack_00000058;
    uVar22 = 0;
    uVar20 = uVar18 - *(ulonglong *)(lVar4 + 0x18);
    if (uVar18 < *(ulonglong *)(lVar4 + 0x18)) {
      uVar20 = uVar22;
    }
    uVar17 = uVar18 - *(ulonglong *)(lVar4 + 0x38);
    if (uVar18 < *(ulonglong *)(lVar4 + 0x38)) {
      uVar17 = uVar22;
    }
    bVar28 = *(ulonglong *)(lVar4 + 0x30) < uVar17;
    uVar17 = *(ulonglong *)(lVar4 + 0x30) - uVar17;
    uVar18 = uVar17;
    if (bVar28) {
      uVar18 = uVar22;
    }
    bVar30 = *(ulonglong *)(lVar4 + 0x10) < uVar20;
    uVar20 = *(ulonglong *)(lVar4 + 0x10) - uVar20;
    uVar14 = uVar20;
    if (bVar30) {
      uVar14 = uVar22;
    }
    uVar18 = uVar14 + uVar18;
    if (!bVar30 && uVar20 != 0 || !bVar28 && uVar17 != 0) {
      uVar18 = uVar22;
    }
    *(ulonglong *)*in_stack_00000058 = *(longlong *)(lVar4 + 0x40) - uVar18;
    plVar26 = (longlong *)*in_RDX;
    if ((bVar30 || uVar20 == 0) && (bVar28 || uVar17 == 0)) {
      piVar27 = (int *)*in_RCX;
      iVar12 = *piVar27;
      goto joined_r0x0001422856dd;
    }
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  *plVar26 = *plVar26 + 1;
  piVar27 = (int *)*in_RCX;
  iVar12 = *piVar27;
joined_r0x0001422856dd:
  if (((iVar12 == 6) && ((char)piVar27[1] != '\0')) && (*plVar26 != 0)) {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(undefined1 *)((longlong)plVar26 + 9) = 1;
  }
  country_core::systems::mode_manager::AppMode::is_tool_selected();
  country_core::input::action_set::DigitalActionState::pressed();
  if (((extraout_AL != '\0') && ((char)plVar26[1] == '\0')) &&
     (extraout_AL_00 == '\0' && *(char *)((longlong)plVar26 + 9) == '\x01')) {
    hint_clutter_focus_edit::make_focus_or_camera_binding_hint();
    iVar12 = 6;
    if (*piVar27 == 6) {
      iVar12 = *(byte *)(piVar27 + 1) + 6;
    }
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar4 = *in_stack_00000040;
    uVar7 = *(undefined8 *)(lVar4 + 0x40);
    uStack_a4 = (undefined4)local_ec;
    uStack_a0 = (undefined4)((ulonglong)local_ec >> 0x20);
    uStack_9c = (undefined4)uStack_e4;
    local_98 = (undefined4)((ulonglong)uStack_e4 >> 0x20);
    lVar23 = *(longlong *)(lVar4 + 0x30);
    if (lVar23 == *(longlong *)(lVar4 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar24 = *(longlong *)(lVar4 + 0x28);
    lVar13 = lVar23 * 0x58;
    *(undefined8 *)(lVar24 + 0x50 + lVar13) = uVar7;
    puVar1 = (undefined8 *)(lVar24 + 0x40 + lVar13);
    *puVar1 = CONCAT44(uStack_7c,uStack_80);
    puVar1[1] = (ulonglong)uStack_5e << 0x10;
    puVar1 = (undefined8 *)(lVar24 + 0x30 + lVar13);
    *puVar1 = CONCAT44(uStack_8c,uStack_90);
    puVar1[1] = CONCAT44(uStack_84,local_88);
    puVar1 = (undefined8 *)(lVar24 + 0x20 + lVar13);
    *puVar1 = CONCAT44(uVar10,uVar9);
    puVar1[1] = CONCAT44(uStack_94,uVar11);
    puVar1 = (undefined8 *)(lVar24 + 0x10 + lVar13);
    *puVar1 = CONCAT44(uStack_dc,local_98);
    puVar1[1] = CONCAT44(uVar8,local_a8);
    *(undefined8 *)(lVar24 + lVar13) = CONCAT44(uStack_a4,iVar12);
    ((undefined8 *)(lVar24 + lVar13))[1] = CONCAT44(uStack_9c,uStack_a0);
    *(longlong *)(lVar4 + 0x30) = lVar23 + 1;
    *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  }
  return;
}

