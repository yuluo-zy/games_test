
void __cdecl
system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance
          (void)

{
  undefined8 uVar1;
  undefined1 uVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  char extraout_AL;
  ulonglong uVar16;
  undefined8 *in_RCX;
  longlong lVar17;
  ulonglong uVar18;
  undefined8 *puVar19;
  ulonglong uVar20;
  undefined8 *puVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong *in_R9;
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  bool bVar29;
  bool bVar30;
  undefined8 *in_stack_00000060;
  undefined4 uStack_4d0;
  
  country_core::resources::gui::EscMenuState::is_open();
  if (extraout_AL != '\0') {
    return;
  }
  uVar3 = *(uint *)*in_RCX;
  lVar17 = 0;
  if (uVar3 - 6 < 0xf) {
    lVar17 = (ulonglong)uVar3 - 5;
  }
  if (lVar17 == 0) {
    if ((uVar3 < 5) && ((0x19U >> (uVar3 & 0x1f) & 1) != 0)) goto LAB_1407af4d6;
  }
  else if ((lVar17 == 6) || (lVar17 == 0xf)) {
LAB_1407af4d6:
    country_core::systems::ui::resources::widget_params::WidgetParams::new();
    country_core::systems::ui::resources::widget_params::UiSounds::empty();
    country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
    uVar22 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
    country_core::systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
    lVar17 = in_stack_00000060[1];
    uVar6 = *(ulonglong *)*in_stack_00000060;
    uVar18 = uVar6 - *(ulonglong *)(lVar17 + 0x18);
    if (uVar6 < *(ulonglong *)(lVar17 + 0x18)) {
      uVar18 = uVar22;
    }
    uVar16 = uVar6 - *(ulonglong *)(lVar17 + 0x38);
    if (uVar6 < *(ulonglong *)(lVar17 + 0x38)) {
      uVar16 = uVar22;
    }
    uVar6 = *(ulonglong *)(lVar17 + 0x10);
    uVar7 = *(ulonglong *)(lVar17 + 0x30);
    bVar29 = uVar7 < uVar16;
    uVar25 = uVar7 - uVar16;
    uVar26 = uVar25;
    if (bVar29) {
      uVar26 = uVar22;
    }
    bVar30 = uVar6 < uVar18;
    uVar27 = uVar6 - uVar18;
    uVar28 = uVar27;
    if (bVar30) {
      uVar28 = uVar22;
    }
    uVar20 = uVar26 + uVar28;
    if (!bVar30 && uVar27 != 0 || !bVar29 && uVar25 != 0) {
      uVar20 = uVar22;
    }
    lVar23 = *(longlong *)(lVar17 + 8);
    lVar24 = *(longlong *)(lVar17 + 0x28);
    *(ulonglong *)*in_stack_00000060 = *(longlong *)(lVar17 + 0x40) - uVar20;
    if (!bVar30 && uVar27 != 0 || !bVar29 && uVar25 != 0) {
      lVar24 = lVar24 + uVar16 * 0x70;
      lVar23 = lVar23 + uVar18 * 0x70;
      if (uVar6 < uVar18) {
        lVar23 = 0x10;
      }
      puVar21 = (undefined8 *)0x0;
      if (uVar18 < uVar6) {
        puVar21 = (undefined8 *)(lVar23 + -0x70 + uVar28 * 0x70);
      }
      if (uVar7 < uVar16) {
        lVar24 = 0x10;
      }
      puVar19 = (undefined8 *)(lVar24 + -0x70 + uVar26 * 0x70);
      if (uVar7 <= uVar16) {
        puVar19 = puVar21;
      }
      uVar8 = puVar19[2];
      uVar9 = puVar19[6];
      uVar15 = puVar19[7];
      uStack_4d0 = (undefined4)((ulonglong)*(undefined8 *)((longlong)puVar19 + 0x3c) >> 0x20);
      uVar4 = *(undefined4 *)((longlong)puVar19 + 0x44);
      uVar5 = *(undefined4 *)(puVar19 + 9);
      uVar11 = puVar19[3];
      uVar12 = puVar19[4];
      uVar10 = puVar19[5];
      uVar2 = *(undefined1 *)(puVar19 + 0xb);
      uVar13 = *puVar19;
      uVar14 = puVar19[1];
      puVar21 = (undefined8 *)*in_R9;
      uVar1 = *(undefined8 *)((longlong)puVar19 + 0x4c);
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      puVar21[1] = 0;
      *puVar21 = 1;
      puVar21[2] = uVar13;
      puVar21[3] = uVar14;
      puVar21[4] = uVar11;
      puVar21[5] = uVar12;
      puVar21[6] = uVar10;
      puVar21[7] = uVar9;
      puVar21[8] = uVar15;
      puVar21[9] = CONCAT44(uVar4,uStack_4d0);
      *(undefined4 *)(puVar21 + 10) = uVar5;
      *(undefined8 *)((longlong)puVar21 + 0x54) = uVar1;
      puVar21[0xc] = uVar8;
      *(undefined1 *)(puVar21 + 0xd) = 1;
      *(undefined1 *)((longlong)puVar21 + 0x69) = uVar2;
      return;
    }
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    uVar6 = *(ulonglong *)*in_R9;
    lVar17 = 0;
    if ((((ulonglong *)*in_R9)[1] - 1) + (ulonglong)(1 < uVar6) < (ulonglong)(uVar6 - 2 < 5)) {
      lVar17 = uVar6 - 1;
    }
                    // WARNING: Could not recover jumptable at 0x0001407af699. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)&DAT_142a83790 + (longlong)(int)(&DAT_142a83790)[lVar17]))
              ((code *)((longlong)&DAT_142a83790 + (longlong)(int)(&DAT_142a83790)[lVar17]));
    return;
  }
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar21 = (undefined8 *)*in_R9;
  puVar21[1] = 0;
  *puVar21 = 6;
  return;
}


