
void __cdecl system_onboarding::hint_focused_edit::hint_focused_edit_condition(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  longlong lVar4;
  int *piVar5;
  char *pcVar6;
  longlong *in_RCX;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong *in_RDX;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong lVar13;
  undefined8 *in_R8;
  ulonglong uVar14;
  longlong lVar15;
  longlong *in_R9;
  int *piVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  bool bVar19;
  bool bVar20;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  
  if (*(char *)(*in_RCX + 0x1f5) != '\0') {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(undefined1 *)(*in_RDX + 8) = 1;
    return;
  }
  lVar9 = in_R8[1];
  uVar10 = *(ulonglong *)*in_R8;
  uVar18 = 0;
  uVar14 = uVar10 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar14 = uVar18;
  }
  uVar3 = uVar10 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar3 = uVar18;
  }
  uVar10 = *(ulonglong *)(lVar9 + 0x10);
  uVar1 = *(ulonglong *)(lVar9 + 0x30);
  bVar19 = uVar1 < uVar3;
  uVar7 = uVar1 - uVar3;
  uVar8 = uVar7;
  if (bVar19) {
    uVar8 = uVar18;
  }
  bVar20 = uVar10 < uVar14;
  uVar11 = uVar10 - uVar14;
  uVar12 = uVar11;
  if (bVar20) {
    uVar12 = uVar18;
  }
  lVar15 = *(longlong *)(lVar9 + 8);
  uVar17 = uVar8 + uVar12;
  if (!bVar20 && uVar11 != 0 || !bVar19 && uVar7 != 0) {
    uVar17 = uVar18;
  }
  lVar4 = *(longlong *)(lVar9 + 0x28);
  *(ulonglong *)*in_R8 = *(longlong *)(lVar9 + 0x40) - uVar17;
  if ((bVar20 || uVar11 == 0) && (bVar19 || uVar7 == 0)) {
    return;
  }
  lVar15 = lVar15 + uVar14 * 0x28;
  if (uVar10 < uVar14) {
    lVar15 = 8;
  }
  piVar16 = (int *)0x0;
  if (uVar14 < uVar10) {
    piVar16 = (int *)(lVar15 + -0x28 + uVar12 * 0x28);
  }
  lVar4 = lVar4 + uVar3 * 0x28;
  if (uVar1 < uVar3) {
    lVar4 = 8;
  }
  piVar5 = (int *)(lVar4 + -0x28 + uVar8 * 0x28);
  if (uVar1 <= uVar3) {
    piVar5 = piVar16;
  }
  lVar9 = *in_RDX;
  if (*piVar5 - 1U < 2) {
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(undefined1 *)(lVar9 + 8) = 1;
  }
  else if ((*(byte *)(lVar9 + 8) & 1) == 0) {
    return;
  }
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar9 = *in_stack_00000030;
  lVar15 = *(longlong *)(lVar9 + 0x30);
  uVar2 = *(undefined8 *)(lVar9 + 0x40);
  if (lVar15 == *(longlong *)(lVar9 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar4 = *(longlong *)(lVar9 + 0x28);
  *(undefined8 *)(lVar4 + lVar15 * 0x10) = uVar2;
  *(undefined1 *)(lVar4 + 8 + lVar15 * 0x10) = 1;
  *(longlong *)(lVar9 + 0x30) = lVar15 + 1;
  *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
  lVar9 = *in_stack_00000028;
  lVar15 = in_stack_00000028[1];
  if (*(char *)(lVar9 + 0x108) != '\0') {
    lVar4 = 0;
    do {
      if (*(longlong *)(lVar9 + 0x100) << 2 == lVar4) {
        return;
      }
      uVar10 = (ulonglong)*(uint *)(*(longlong *)(lVar9 + 0xf8) + lVar4);
      lVar4 = lVar4 + 4;
    } while (*(longlong *)(*(longlong *)(lVar15 + 0x1a8) + 0x10 + uVar10 * 0x48) == 0);
    lVar15 = *(longlong *)(lVar15 + 0x1a8) + uVar10 * 0x48;
    if ((*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar15 + 0x40)) &&
       (uVar10 = *(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
       uVar10 != 0)) {
      pcVar6 = *(char **)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar10 * 0x30);
    }
    else {
      pcVar6 = (char *)0x0;
    }
    goto LAB_14227dcf2;
  }
  lVar4 = 0;
  do {
    if (*(longlong *)(lVar9 + 0x100) << 2 == lVar4) {
      return;
    }
    lVar13 = (ulonglong)*(uint *)(*(longlong *)(lVar9 + 0xf8) + lVar4) * 0xa0;
    lVar4 = lVar4 + 4;
  } while (*(longlong *)(*(longlong *)(lVar15 + 0x100) + 0x58 + lVar13) == 0);
  lVar13 = *(longlong *)(lVar15 + 0x100) + lVar13;
  uVar10 = (ulonglong)*(uint *)(lVar13 + 0x94);
  if (*(ulonglong *)(lVar9 + 0x110) <
      *(ulonglong *)(*(longlong *)(lVar15 + 0x1a8) + 0x40 + uVar10 * 0x48)) {
    lVar15 = *(longlong *)(lVar15 + 0x1a8) + uVar10 * 0x48;
    uVar10 = *(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8);
    if (uVar10 == 0) goto LAB_14227dce6;
    lVar9 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar10 * 0x30);
  }
  else {
LAB_14227dce6:
    lVar9 = 0;
  }
  pcVar6 = (char *)((ulonglong)*(uint *)(*(longlong *)(lVar13 + 0x50) + 8) + lVar9);
LAB_14227dcf2:
  if (*pcVar6 == '\x01') {
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar9 = *in_R9;
    lVar15 = *(longlong *)(lVar9 + 0x30);
    uVar2 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar15 == *(longlong *)(lVar9 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar4 = *(longlong *)(lVar9 + 0x28);
    *(undefined4 *)(lVar4 + lVar15 * 0x20) = 0x13;
    *(undefined8 *)(lVar4 + 0x18 + lVar15 * 0x20) = uVar2;
    *(longlong *)(lVar9 + 0x30) = lVar15 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
  }
  return;
}

