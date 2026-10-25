
void __cdecl toolbar::ui_interaction::hide_ui(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  char cVar3;
  byte bVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  undefined4 *puVar11;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte bVar12;
  longlong lVar13;
  longlong lVar14;
  longlong *in_RDX;
  longlong lVar15;
  byte *pbVar16;
  longlong lVar17;
  longlong *in_R8;
  uint *puVar18;
  ulonglong uVar19;
  byte *pbVar20;
  ulonglong uVar21;
  int *piVar22;
  ulonglong uVar23;
  bool bVar24;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong local_70;
  longlong local_60;
  longlong local_58;
  
  country_core::resources::gui::EscMenuState::is_closed();
  if ((extraout_AL == '\0') ||
     (country_core::systems::ui::resources::widget_params::UiFocus::is_mei(), extraout_AL_00 == '\0'
     )) {
    bVar12 = 0;
  }
  else {
    bVar12 = *(byte *)*in_stack_00000030 ^ 1;
    if (*(byte *)*in_stack_00000030 == 0) {
      piVar22 = (int *)*in_stack_00000028;
      goto LAB_14233ebdd;
    }
  }
  piVar22 = (int *)*in_stack_00000028;
  country_core::systems::ui::resources::widget_params::UiFocus::is_toolbar();
  if (extraout_AL_01 != '\0') {
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    piVar22[4] = 0;
    piVar22[5] = 0;
  }
LAB_14233ebdd:
  lVar7 = *in_stack_00000038;
  lVar8 = in_stack_00000038[1];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000038 + 0x14);
  puVar18 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar7 + 0x100);
  cVar3 = *(char *)(lVar7 + 0x108);
  lVar9 = *in_RDX;
  lVar10 = *in_R8;
  uVar6 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  puVar11 = (undefined4 *)in_stack_00000028[2];
  local_58 = 8;
  lVar17 = 0;
  uVar21 = 0;
  uVar23 = 0;
  local_60 = 8;
  bVar24 = false;
  while( true ) {
    if (cVar3 == '\0') {
      if (uVar21 == uVar23) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar23 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
          if (uVar23 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar15 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar18 * 0xa0;
        if (*(ulonglong *)(lVar7 + 0x120) < *(ulonglong *)(lVar15 + 0x88)) {
          bVar24 = *(longlong *)(*(longlong *)(lVar15 + 0x80) + *(ulonglong *)(lVar7 + 0x120) * 8)
                   != 0;
        }
        else {
          bVar24 = false;
        }
        lVar13 = *(longlong *)
                  (*(longlong *)(lVar8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar15 + 0x94) * 0x48)
        ;
        lVar14 = ~*(ulonglong *)
                   (*(longlong *)
                     (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar15 + 0x94) * 0x48) + *(longlong *)(lVar7 + 0x110) * 8)
                 * 0x30;
        lVar17 = *(longlong *)(lVar13 + 0x10 + lVar14);
        in_RDX = *(longlong **)(lVar13 + 0x28 + lVar14);
        puVar18 = puVar18 + 1;
        local_58 = *(longlong *)(lVar15 + 0x50);
        uVar21 = 0;
      }
      local_70 = *(longlong *)(local_58 + uVar21 * 0x10);
      uVar19 = (ulonglong)*(uint *)(local_58 + 8 + uVar21 * 0x10);
    }
    else {
      if (uVar21 == uVar23) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar23 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
          if (uVar23 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar15 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
        if (*(ulonglong *)(lVar7 + 0x120) < *(ulonglong *)(lVar15 + 0x40)) {
          bVar24 = *(longlong *)(*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar7 + 0x120) * 8)
                   != 0;
        }
        else {
          bVar24 = false;
        }
        lVar13 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar7 + 0x110) * 8) *
                 0x30;
        lVar17 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + lVar13);
        in_RDX = *(longlong **)(*(longlong *)(lVar15 + 0x18) + 0x28 + lVar13);
        puVar18 = puVar18 + 1;
        local_60 = *(longlong *)(lVar15 + 8);
        uVar21 = 0;
      }
      local_70 = *(longlong *)(local_60 + uVar21 * 8);
      uVar19 = uVar21 & 0xffffffff;
    }
    pbVar16 = (byte *)(uVar19 * 3 + lVar17);
    puVar2 = (undefined4 *)((longlong)in_RDX + uVar19 * 4);
    uVar21 = uVar21 + 1;
    if (!bVar24) break;
    if ((bVar12 == 0) || (pbVar20 = (byte *)(lVar10 + 10), *(byte *)(lVar9 + 1) == 0))
    goto LAB_14233ee00;
LAB_14233ee1a:
    bVar4 = *pbVar20;
    *puVar2 = uVar5;
    *pbVar16 = bVar4 ^ 1;
    if (bVar4 != 0) {
LAB_14233ee72:
      if ((piVar22[4] == 1) && (*(longlong *)(piVar22 + 6) == local_70)) {
        *puVar11 = uVar6;
        piVar22[4] = 0;
        piVar22[5] = 0;
      }
      if ((*piVar22 == 1) && (*(longlong *)(piVar22 + 2) == local_70)) {
        *puVar11 = uVar6;
        piVar22[0] = 0;
        piVar22[1] = 0;
      }
    }
  }
  pbVar20 = (byte *)(lVar9 + 1);
  if (bVar12 != 0) goto LAB_14233ee1a;
LAB_14233ee00:
  *puVar2 = uVar5;
  *pbVar16 = 0;
  goto LAB_14233ee72;
}


