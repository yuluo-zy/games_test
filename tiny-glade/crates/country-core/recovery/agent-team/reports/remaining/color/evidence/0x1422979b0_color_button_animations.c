
void __cdecl system_color::color_menu::color_button_animations(void)

{
  undefined8 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char cVar6;
  uint uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar13;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  float extraout_XMM0_Da;
  float fVar17;
  float fVar18;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  longlong local_208;
  longlong local_200;
  longlong local_1f8;
  longlong local_1f0;
  longlong local_1e8;
  longlong local_1e0;
  ulonglong local_1d0;
  longlong local_1c0;
  uint *local_190;
  uint *local_188;
  longlong local_170;
  ulonglong local_160;
  ulonglong local_158;
  longlong local_150;
  longlong local_140;
  undefined4 local_134;
  longlong local_130;
  longlong local_120;
  undefined4 local_114;
  longlong local_110;
  longlong local_100;
  undefined4 local_f4;
  byte local_e8;
  byte local_e0;
  
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_mut_<country_core::resources::color_icon::CustomizationIcon>,ref_mut_<country_core::components::Ui2dExtent>,ref_mut_<country_core::components::UiDrawOrder>,enum2_<core::option::Option<ref_<country_core::resources::color_icon::CustomizationIconMarkedForDeletion>_>_>_>,tuple_<>_>
            ();
  local_1c0 = local_170;
  local_1d0 = local_158;
  local_1e0 = local_150;
  local_1e8 = local_140;
  local_1f0 = local_130;
  local_1f8 = local_120;
  local_200 = local_110;
  local_208 = local_100;
  lVar10 = *in_RDX;
  do {
    if ((local_e0 & 1) == 0) {
      if (local_1d0 == local_160) {
        do {
          if (local_190 == local_188) {
            return;
          }
          uVar7 = *local_190;
          local_190 = local_190 + 1;
          local_160 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar7 * 0xa0);
        } while (local_160 == 0);
        lVar11 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar7 * 0xa0;
        if (*(ulonglong *)(lVar8 + 0x128) < *(ulonglong *)(lVar11 + 0x88)) {
          local_e8 = *(longlong *)(*(longlong *)(lVar11 + 0x80) + *(ulonglong *)(lVar8 + 0x128) * 8)
                     != 0;
        }
        else {
          local_e8 = false;
        }
        lVar12 = *(longlong *)
                  (*(longlong *)(lVar9 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar11 + 0x94) * 0x48)
        ;
        lVar13 = *(longlong *)
                  (*(longlong *)(lVar9 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar11 + 0x94) * 0x48)
        ;
        lVar14 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
        local_1e0 = *(longlong *)(lVar12 + 0x10 + lVar14);
        local_1e8 = *(longlong *)(lVar12 + 0x28 + lVar14);
        lVar14 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
        local_1f0 = *(longlong *)(lVar12 + 0x10 + lVar14);
        local_1f8 = *(longlong *)(lVar12 + 0x28 + lVar14);
        lVar13 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
        local_200 = *(longlong *)(lVar12 + 0x10 + lVar13);
        local_208 = *(longlong *)(lVar12 + 0x28 + lVar13);
        local_1c0 = *(longlong *)(lVar11 + 0x50);
        local_1d0 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(local_1c0 + 8 + local_1d0 * 0x10);
    }
    else {
      if (local_1d0 == local_160) {
        do {
          if (local_190 == local_188) {
            return;
          }
          uVar7 = *local_190;
          local_190 = local_190 + 1;
          local_160 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar7 * 0x48);
        } while (local_160 == 0);
        lVar11 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar7 * 0x48;
        lVar12 = *(longlong *)(lVar11 + 0x38);
        if (*(ulonglong *)(lVar8 + 0x128) < *(ulonglong *)(lVar11 + 0x40)) {
          local_e8 = *(longlong *)(lVar12 + *(ulonglong *)(lVar8 + 0x128) * 8) != 0;
        }
        else {
          local_e8 = false;
        }
        lVar11 = *(longlong *)(lVar11 + 0x18);
        lVar13 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
        local_1e0 = *(longlong *)(lVar11 + 0x10 + lVar13);
        local_1e8 = *(longlong *)(lVar11 + 0x28 + lVar13);
        lVar13 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
        local_1f0 = *(longlong *)(lVar11 + 0x10 + lVar13);
        local_1f8 = *(longlong *)(lVar11 + 0x28 + lVar13);
        lVar12 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
        local_200 = *(longlong *)(lVar11 + 0x10 + lVar12);
        local_208 = *(longlong *)(lVar11 + 0x28 + lVar12);
        local_1d0 = 0;
      }
      uVar15 = local_1d0 & 0xffffffff;
    }
    puVar1 = (undefined8 *)(local_1e0 + uVar15 * 0x24);
    puVar2 = (undefined4 *)(local_1e8 + uVar15 * 4);
    puVar3 = (undefined4 *)(local_1f8 + uVar15 * 4);
    puVar16 = (undefined8 *)(uVar15 * 0x10 + local_1f0);
    fVar18 = *(float *)(lVar10 + 0x28);
    *puVar2 = local_134;
    fVar17 = fVar18 / -0.15;
    if ((local_e8 & 1) == 0) {
      fVar17 = fVar18 * 4.0;
    }
    local_1d0 = local_1d0 + 1;
    *(float *)(puVar1 + 3) = fVar17 + *(float *)(puVar1 + 3);
    *(undefined4 *)(local_208 + uVar15 * 4) = local_f4;
    *(ulonglong *)(local_200 + uVar15 * 8) = (ulonglong)((byte)~local_e8 & 1);
    fVar18 = *(float *)(puVar1 + 3);
    if (1.0 <= fVar18) {
      fVar18 = 1.0;
    }
    *puVar2 = local_134;
    *(float *)(puVar1 + 3) = fVar18;
    uVar4 = *puVar1;
    ::utils::easing_functions::EasingFunction::apply();
    *puVar3 = local_114;
    puVar16[1] = CONCAT44(extraout_XMM0_Da_00 * (float)((ulonglong)uVar4 >> 0x20),
                          extraout_XMM0_Da_00 * (float)uVar4);
    if ((local_e8 & 1) == 0) {
      ::utils::easing_functions::EasingFunction::apply();
      fVar18 = (1.0 - extraout_XMM0_Da_01) * 0.8 + extraout_XMM0_Da_01;
      uVar4 = puVar1[1];
      uVar5 = puVar1[2];
      *puVar3 = local_114;
      *puVar16 = CONCAT44(fVar18 * (float)((ulonglong)uVar4 >> 0x20) +
                          (1.0 - fVar18) * (float)((ulonglong)uVar5 >> 0x20),
                          fVar18 * (float)uVar4 + (1.0 - fVar18) * (float)uVar5);
      cVar6 = *(char *)(puVar1 + 4);
      *puVar2 = local_134;
      fVar18 = 0.15;
      if (cVar6 == '\0') {
        fVar18 = -0.15;
      }
      fVar18 = fVar18 + *(float *)((longlong)puVar1 + 0x1c);
      fVar17 = 0.0;
      if (0.0 <= fVar18) {
        fVar17 = fVar18;
      }
      fVar18 = 1.0;
      if (fVar17 <= 1.0) {
        fVar18 = fVar17;
      }
      *(float *)((longlong)puVar1 + 0x1c) = fVar18;
      *puVar3 = local_114;
      ::utils::easing_functions::EasingFunction::apply();
      fVar18 = extraout_XMM0_Da * 0.2 + (1.0 - extraout_XMM0_Da) * 0.0 + 1.0;
      puVar16[1] = CONCAT44(fVar18 * (float)((ulonglong)puVar16[1] >> 0x20),
                            fVar18 * (float)puVar16[1]);
    }
  } while( true );
}

