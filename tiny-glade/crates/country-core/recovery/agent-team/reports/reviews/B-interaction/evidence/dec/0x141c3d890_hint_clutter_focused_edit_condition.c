
void __cdecl system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_condition(void)

{
  ulonglong uVar1;
  byte *pbVar2;
  undefined8 uVar3;
  ulonglong uVar4;
  longlong lVar5;
  int *piVar6;
  char *pcVar7;
  longlong *in_RCX;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 *in_RDX;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  undefined8 *in_R8;
  ulonglong uVar15;
  longlong lVar16;
  longlong *in_R9;
  int *piVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  bool bVar20;
  bool bVar21;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  
  if (*(char *)(*in_RCX + 0x200) != '\0') {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(undefined1 *)*in_RDX = 1;
    return;
  }
  lVar10 = in_R8[1];
  uVar11 = *(ulonglong *)*in_R8;
  uVar19 = 0;
  uVar15 = uVar11 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar15 = uVar19;
  }
  uVar4 = uVar11 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar4 = uVar19;
  }
  uVar11 = *(ulonglong *)(lVar10 + 0x10);
  uVar1 = *(ulonglong *)(lVar10 + 0x30);
  bVar20 = uVar1 < uVar4;
  uVar8 = uVar1 - uVar4;
  uVar9 = uVar8;
  if (bVar20) {
    uVar9 = uVar19;
  }
  bVar21 = uVar11 < uVar15;
  uVar12 = uVar11 - uVar15;
  uVar13 = uVar12;
  if (bVar21) {
    uVar13 = uVar19;
  }
  lVar16 = *(longlong *)(lVar10 + 8);
  uVar18 = uVar9 + uVar13;
  if (!bVar21 && uVar12 != 0 || !bVar20 && uVar8 != 0) {
    uVar18 = uVar19;
  }
  lVar5 = *(longlong *)(lVar10 + 0x28);
  *(ulonglong *)*in_R8 = *(longlong *)(lVar10 + 0x40) - uVar18;
  if ((bVar21 || uVar12 == 0) && (bVar20 || uVar8 == 0)) {
    return;
  }
  lVar16 = lVar16 + uVar15 * 0x28;
  if (uVar11 < uVar15) {
    lVar16 = 8;
  }
  piVar17 = (int *)0x0;
  if (uVar15 < uVar11) {
    piVar17 = (int *)(lVar16 + -0x28 + uVar13 * 0x28);
  }
  lVar5 = lVar5 + uVar4 * 0x28;
  if (uVar1 < uVar4) {
    lVar5 = 8;
  }
  piVar6 = (int *)(lVar5 + -0x28 + uVar9 * 0x28);
  if (uVar1 <= uVar4) {
    piVar6 = piVar17;
  }
  pbVar2 = (byte *)*in_RDX;
  if (*piVar6 == 3) {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *pbVar2 = 1;
  }
  else if ((*pbVar2 & 1) == 0) {
    return;
  }
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar10 = *in_stack_00000030;
  lVar16 = *(longlong *)(lVar10 + 0x30);
  uVar3 = *(undefined8 *)(lVar10 + 0x40);
  if (lVar16 == *(longlong *)(lVar10 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar5 = *(longlong *)(lVar10 + 0x28);
  *(undefined8 *)(lVar5 + lVar16 * 0x10) = uVar3;
  *(undefined1 *)(lVar5 + 8 + lVar16 * 0x10) = 9;
  *(longlong *)(lVar10 + 0x30) = lVar16 + 1;
  *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
  lVar10 = *in_stack_00000028;
  lVar16 = in_stack_00000028[1];
  if (*(char *)(lVar10 + 0x108) != '\0') {
    lVar5 = 0;
    do {
      if (*(longlong *)(lVar10 + 0x100) << 2 == lVar5) {
        return;
      }
      uVar11 = (ulonglong)*(uint *)(*(longlong *)(lVar10 + 0xf8) + lVar5);
      lVar5 = lVar5 + 4;
    } while (*(longlong *)(*(longlong *)(lVar16 + 0x1a8) + 0x10 + uVar11 * 0x48) == 0);
    lVar16 = *(longlong *)(lVar16 + 0x1a8) + uVar11 * 0x48;
    if ((*(ulonglong *)(lVar10 + 0x110) < *(ulonglong *)(lVar16 + 0x40)) &&
       (uVar11 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
       uVar11 != 0)) {
      pcVar7 = *(char **)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar11 * 0x30);
    }
    else {
      pcVar7 = (char *)0x0;
    }
    goto LAB_141c3db12;
  }
  lVar5 = 0;
  do {
    if (*(longlong *)(lVar10 + 0x100) << 2 == lVar5) {
      return;
    }
    lVar14 = (ulonglong)*(uint *)(*(longlong *)(lVar10 + 0xf8) + lVar5) * 0xa0;
    lVar5 = lVar5 + 4;
  } while (*(longlong *)(*(longlong *)(lVar16 + 0x100) + 0x58 + lVar14) == 0);
  lVar14 = *(longlong *)(lVar16 + 0x100) + lVar14;
  uVar11 = (ulonglong)*(uint *)(lVar14 + 0x94);
  if (*(ulonglong *)(lVar10 + 0x110) <
      *(ulonglong *)(*(longlong *)(lVar16 + 0x1a8) + 0x40 + uVar11 * 0x48)) {
    lVar16 = *(longlong *)(lVar16 + 0x1a8) + uVar11 * 0x48;
    uVar11 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8);
    if (uVar11 == 0) goto LAB_141c3db06;
    lVar10 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar11 * 0x30);
  }
  else {
LAB_141c3db06:
    lVar10 = 0;
  }
  pcVar7 = (char *)((ulonglong)*(uint *)(*(longlong *)(lVar14 + 0x50) + 8) + lVar10);
LAB_141c3db12:
  if (*pcVar7 == '\t') {
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar10 = *in_R9;
    lVar16 = *(longlong *)(lVar10 + 0x30);
    uVar3 = *(undefined8 *)(lVar10 + 0x40);
    if (lVar16 == *(longlong *)(lVar10 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar5 = *(longlong *)(lVar10 + 0x28);
    *(undefined4 *)(lVar5 + lVar16 * 0x20) = 0x13;
    *(undefined8 *)(lVar5 + 0x18 + lVar16 * 0x20) = uVar3;
    *(longlong *)(lVar10 + 0x30) = lVar16 + 1;
    *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
  }
  return;
}

