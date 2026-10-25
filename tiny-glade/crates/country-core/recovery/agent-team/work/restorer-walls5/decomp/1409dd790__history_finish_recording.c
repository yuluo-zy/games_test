// 0x1409dd790 history_finish_recording size=621 sig=void __cdecl history_finish_recording(void)
void __cdecl country_core::systems::history::history_recording::history_finish_recording(void)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  ulonglong uVar3;
  undefined4 *puVar4;
  bool bVar5;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar6;
  longlong lVar7;
  longlong *in_R8;
  longlong in_R9;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  bool bVar13;
  bool bVar14;
  longlong in_stack_00000028;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong in_stack_00000058;
  
  if (SBORROW8(0,*(longlong *)(*in_R8 + 0x30))) {
    return;
  }
  lVar7 = in_stack_00000038[1];
  uVar9 = *(ulonglong *)*in_stack_00000038;
  uVar6 = 0;
  uVar11 = uVar9 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar11 = uVar6;
  }
  uVar8 = uVar9 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar8 = uVar6;
  }
  bVar13 = *(ulonglong *)(lVar7 + 0x30) < uVar8;
  uVar8 = *(ulonglong *)(lVar7 + 0x30) - uVar8;
  uVar9 = uVar8;
  if (bVar13) {
    uVar9 = uVar6;
  }
  bVar14 = *(ulonglong *)(lVar7 + 0x10) < uVar11;
  uVar11 = *(ulonglong *)(lVar7 + 0x10) - uVar11;
  uVar10 = uVar11;
  if (bVar14) {
    uVar10 = uVar6;
  }
  uVar9 = uVar10 + uVar9;
  if (!bVar14 && uVar11 != 0 || !bVar13 && uVar8 != 0) {
    uVar9 = uVar6;
  }
  *(ulonglong *)*in_stack_00000038 = *(longlong *)(lVar7 + 0x40) - uVar9;
  bVar5 = true;
  if ((bVar14 || uVar11 == 0) && (bVar13 || uVar8 == 0)) {
    lVar7 = in_stack_00000040[1];
    uVar9 = *(ulonglong *)*in_stack_00000040;
    uVar6 = 0;
    uVar11 = uVar9 - *(ulonglong *)(lVar7 + 0x18);
    if (uVar9 < *(ulonglong *)(lVar7 + 0x18)) {
      uVar11 = uVar6;
    }
    uVar8 = uVar9 - *(ulonglong *)(lVar7 + 0x38);
    if (uVar9 < *(ulonglong *)(lVar7 + 0x38)) {
      uVar8 = uVar6;
    }
    bVar13 = uVar8 <= *(ulonglong *)(lVar7 + 0x30);
    uVar8 = *(ulonglong *)(lVar7 + 0x30) - uVar8;
    uVar9 = uVar8;
    if (!bVar13) {
      uVar9 = uVar6;
    }
    bVar14 = uVar11 <= *(ulonglong *)(lVar7 + 0x10);
    uVar11 = *(ulonglong *)(lVar7 + 0x10) - uVar11;
    uVar10 = uVar11;
    if (!bVar14) {
      uVar10 = uVar6;
    }
    bVar5 = bVar14 && uVar11 != 0 || bVar13 && uVar8 != 0;
    uVar9 = uVar10 + uVar9;
    if (bVar14 && uVar11 != 0 || bVar13 && uVar8 != 0) {
      uVar9 = uVar6;
    }
    *(ulonglong *)*in_stack_00000040 = *(longlong *)(lVar7 + 0x40) - uVar9;
  }
  lVar7 = *in_RCX;
  if (1 < *(byte *)(lVar7 + 0x18)) {
    bVar5 = (bool)(bVar5 | *(byte *)(lVar7 + 0x19) < 2 &
                           *(byte *)(*(longlong *)(lVar7 + 0x20) + 0xb9));
  }
  puVar2 = (ulonglong *)*in_stack_00000048;
  lVar7 = in_stack_00000048[1];
  uVar9 = *puVar2;
  uVar6 = 0;
  uVar11 = uVar9 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar9 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar11 = uVar6;
  }
  uVar8 = uVar9 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar9 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar8 = uVar6;
  }
  uVar9 = *(ulonglong *)(lVar7 + 0x30);
  uVar10 = uVar9 - uVar8;
  if (uVar8 > uVar9) {
    uVar10 = uVar6;
  }
  uVar3 = *(ulonglong *)(lVar7 + 0x10);
  uVar12 = uVar3 - uVar11;
  if (uVar11 > uVar3) {
    uVar12 = uVar6;
  }
  uVar6 = *(ulonglong *)(lVar7 + 0x40);
  *puVar2 = uVar6 - (uVar12 + uVar10);
  if (uVar11 <= uVar3 && uVar3 - uVar11 != 0 || uVar8 <= uVar9 && uVar9 - uVar8 != 0) {
    *puVar2 = uVar6;
    lVar7 = *in_RDX;
  }
  else {
    lVar7 = *in_RDX;
    if (bVar5 == false) goto LAB_1409dd9dd;
  }
  **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
  resources::walls::decorator_archivist::DecoratorArchivist::extract_diff();
  **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
  resources::stairs::stairs_state::StairsState::extract_diff_and_make_backup();
  uVar1 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar4 = (undefined4 *)in_R8[2];
  *puVar4 = uVar1;
  **(undefined4 **)(in_stack_00000058 + 0x10) = *(undefined4 *)(in_stack_00000058 + 0x1c);
  resources::history::History::add_decorator_diff();
  *puVar4 = uVar1;
  resources::history::History::finish_input_recording();
LAB_1409dd9dd:
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  *(undefined1 *)(lVar7 + 0x1b0) = 1;
  return;
}
