
void __cdecl system_onboarding::hint_display_system::hint_position_update(void)

{
  uint *puVar1;
  char cVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *puVar6;
  uint uVar7;
  longlong *in_RCX;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined8 *in_RDX;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  uint *puVar16;
  float extraout_XMM0_Da;
  uint extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  uint extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  uint extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  undefined4 extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  undefined4 extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  undefined4 extraout_XMM0_Da_11;
  undefined4 extraout_XMM0_Da_12;
  float extraout_XMM0_Db;
  float extraout_XMM0_Db_00;
  float extraout_XMM0_Db_01;
  undefined4 extraout_XMM0_Db_02;
  undefined4 extraout_XMM0_Db_03;
  float extraout_XMM0_Db_04;
  float extraout_XMM0_Db_05;
  undefined4 extraout_XMM0_Dc;
  undefined4 extraout_XMM0_Dc_00;
  undefined4 extraout_XMM0_Dd;
  undefined4 extraout_XMM0_Dd_00;
  float fVar17;
  uint uVar18;
  uint uVar19;
  undefined1 in_XMM1 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  float fVar24;
  uint uVar25;
  float fVar26;
  uint uVar27;
  float fVar28;
  uint uVar29;
  uint uVar31;
  undefined1 auVar30 [16];
  float fVar32;
  undefined8 uVar33;
  float fVar36;
  float fVar37;
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  longlong local_240;
  longlong local_188;
  undefined4 *local_178;
  longlong local_170;
  undefined8 *local_160;
  longlong local_120;
  longlong local_118;
  
  country_core::resources::window::WindowSize::as_vec2();
  auVar20._4_12_ = in_XMM1._4_12_;
  fVar17 = in_XMM1._0_4_ / 1000.0;
  fVar24 = fVar17 * 20.0;
  auVar20._0_4_ = fVar17 * 30.0;
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  uVar3 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar16 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar16 + *(longlong *)(lVar4 + 0x100);
  puVar6 = (undefined8 *)*in_RDX;
  auVar21._8_4_ = in_XMM1._8_4_;
  cVar2 = *(char *)(lVar4 + 0x108);
  local_240 = 8;
  local_170 = 0;
  local_120 = 0;
  local_118 = 0;
  uVar14 = 0;
  uVar12 = 0;
  auVar23 = auVar20;
  do {
    if (cVar2 == '\0') {
      if (uVar12 == uVar14) {
        if (puVar16 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar16 * 0xa0);
          if (uVar14 != 0) break;
          puVar16 = puVar16 + 1;
          if (puVar16 == puVar1) {
            return;
          }
        }
        lVar13 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar16 * 0xa0;
        uVar12 = (ulonglong)*(uint *)(lVar13 + 0x94);
        lVar10 = *(longlong *)(lVar5 + 0x1a8);
        lVar11 = lVar10 + uVar12 * 0x48;
        lVar8 = *(longlong *)(lVar10 + 0x38 + uVar12 * 0x48);
        uVar12 = *(ulonglong *)(lVar10 + 0x40 + uVar12 * 0x48);
        if (*(ulonglong *)(lVar4 + 0x110) < uVar12) {
          uVar9 = *(ulonglong *)(lVar8 + *(ulonglong *)(lVar4 + 0x110) * 8);
          lVar11 = *(longlong *)(lVar11 + 0x18);
          if (uVar9 == 0) goto LAB_141c509bf;
          local_120 = *(longlong *)(lVar11 + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(lVar4 + 0x120);
          if (uVar9 < uVar12) goto LAB_141c509dd;
LAB_141c509ff:
          local_118 = 0;
        }
        else {
          lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_141c509bf:
          local_120 = 0;
          uVar9 = *(ulonglong *)(lVar4 + 0x120);
          if (uVar12 <= uVar9) goto LAB_141c509ff;
LAB_141c509dd:
          uVar12 = *(ulonglong *)(lVar8 + uVar9 * 8);
          if (uVar12 == 0) goto LAB_141c509ff;
          local_118 = *(longlong *)(lVar11 + 0x10 + ~uVar12 * 0x30);
        }
        puVar16 = puVar16 + 1;
        lVar10 = ~*(ulonglong *)(lVar8 + *(longlong *)(lVar4 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar11 + 0x10 + lVar10);
        local_188 = *(longlong *)(lVar11 + 0x28 + lVar10);
        local_240 = *(longlong *)(lVar13 + 0x50);
        uVar12 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(local_240 + 8 + uVar12 * 0x10);
      puVar15 = (undefined8 *)(local_120 + uVar9 * 0x18);
      local_160 = (undefined8 *)(local_170 + uVar9 * 0x14);
      local_178 = (undefined4 *)(local_188 + uVar9 * 4);
      uVar12 = uVar12 + 1;
      uVar7 = *(uint *)(uVar9 * 0x18 + local_118);
      switch(uVar7) {
      case 6:
code_r0x000141c50acb:
        fVar32 = (float)*puVar6 + fVar24 + fVar24;
        fVar36 = (float)((ulonglong)*puVar6 >> 0x20) + 0.0;
        fVar37 = 0.0;
        fVar17 = 0.0;
        break;
      case 7:
        goto code_r0x000141c50dc6;
      case 8:
        goto code_r0x000141c50af9;
      default:
code_r0x000141c50cee:
                    // WARNING: Could not recover jumptable at 0x000141c50d17. Too many branches
                    // WARNING: Treating indirect jump as call
        (*(code *)(&UNK_142f14844 + *(int *)(&UNK_142f14844 + (ulonglong)uVar7 * 4)))();
        return;
      case 10:
        goto code_r0x000141c50ae0;
      }
    }
    else {
      if (uVar12 == uVar14) {
        if (puVar16 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar16 * 0x48);
          if (uVar14 != 0) break;
          puVar16 = puVar16 + 1;
          if (puVar16 == puVar1) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar16 * 0x48;
        lVar10 = *(longlong *)(lVar11 + 0x38);
        if (*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar11 + 0x40)) {
          uVar12 = *(ulonglong *)(lVar10 + *(ulonglong *)(lVar4 + 0x110) * 8);
          lVar8 = *(longlong *)(lVar11 + 0x18);
          if (uVar12 == 0) goto LAB_141c508cb;
          local_120 = *(longlong *)(lVar8 + 0x10 + ~uVar12 * 0x30);
        }
        else {
          lVar8 = *(longlong *)(lVar11 + 0x18);
LAB_141c508cb:
          local_120 = 0;
        }
        puVar16 = puVar16 + 1;
        lVar13 = ~*(ulonglong *)(lVar10 + *(longlong *)(lVar4 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar8 + 0x10 + lVar13);
        local_188 = *(longlong *)(lVar8 + 0x28 + lVar13);
        if ((*(ulonglong *)(lVar4 + 0x120) < *(ulonglong *)(lVar11 + 0x40)) &&
           (uVar12 = *(ulonglong *)(lVar10 + *(ulonglong *)(lVar4 + 0x120) * 8), uVar12 != 0)) {
          local_118 = *(longlong *)(lVar8 + 0x10 + ~uVar12 * 0x30);
        }
        else {
          local_118 = 0;
        }
        uVar12 = 0;
      }
      uVar9 = uVar12 & 0xffffffff;
      puVar15 = (undefined8 *)(local_120 + uVar9 * 0x18);
      local_160 = (undefined8 *)(local_170 + uVar9 * 0x14);
      local_178 = (undefined4 *)(local_188 + uVar9 * 4);
      uVar12 = uVar12 + 1;
      uVar7 = *(uint *)(uVar9 * 0x18 + local_118);
      switch(uVar7) {
      case 6:
        goto code_r0x000141c50acb;
      case 7:
code_r0x000141c50dc6:
        ::utils::swizzlers::impl$1::xvy();
        ::utils::camera::CameraMatrices::eye_right();
        country_core::resources::window::WindowSize::as_vec2();
        auVar23._4_4_ = extraout_XMM0_Db_03;
        auVar23._0_4_ = extraout_XMM0_Da_08;
        auVar23._8_4_ = extraout_XMM0_Dc_00;
        auVar23._12_4_ = extraout_XMM0_Dd_00;
        country_core::utils::world_space_to_screen_space();
        fVar32 = extraout_XMM0_Da_09 + fVar24;
        fVar36 = auVar23._0_4_ + fVar24;
        fVar37 = extraout_XMM0_Db_04 + 0.0;
        fVar17 = auVar23._4_4_ + 0.0;
        break;
      case 8:
code_r0x000141c50af9:
        ::utils::geometry::rectangle::Rectangle2d::as_points3();
        country_core::resources::window::WindowSize::as_vec2();
        uVar7 = extraout_XMM0_Da_00;
        fVar17 = extraout_XMM0_Db_00;
        country_core::utils::world_space_to_screen_space();
        country_core::resources::window::WindowSize::as_vec2();
        uVar18 = extraout_XMM0_Da_02;
        country_core::utils::world_space_to_screen_space();
        country_core::resources::window::WindowSize::as_vec2();
        uVar19 = extraout_XMM0_Da_04;
        country_core::utils::world_space_to_screen_space();
        country_core::resources::window::WindowSize::as_vec2();
        auVar23._4_4_ = extraout_XMM0_Db_02;
        auVar23._0_4_ = extraout_XMM0_Da_06;
        auVar23._8_4_ = extraout_XMM0_Dc;
        auVar23._12_4_ = extraout_XMM0_Dd;
        country_core::utils::world_space_to_screen_space();
        uVar25 = (int)((uint)(extraout_XMM0_Da_01 < extraout_XMM0_Da_03) << 0x1f) >> 0x1f;
        uVar27 = (int)((uint)(extraout_XMM0_Da_01 < extraout_XMM0_Da_03) << 0x1f) >> 0x1f;
        fVar32 = (float)(~uVar25 & (uint)extraout_XMM0_Da_01 | (uint)extraout_XMM0_Da_03 & uVar25);
        uVar25 = (uint)(fVar32 < extraout_XMM0_Da_05);
        uVar29 = (int)(uVar25 << 0x1f) >> 0x1f;
        uVar31 = (int)(uVar25 << 0x1f) >> 0x1f;
        fVar32 = (float)(~uVar29 & (uint)fVar32 | (uint)extraout_XMM0_Da_05 & uVar29);
        uVar25 = (uint)(fVar32 < extraout_XMM0_Da_07);
        uVar29 = (int)(uVar25 << 0x1f) >> 0x1f;
        uVar25 = (int)(uVar25 << 0x1f) >> 0x1f;
        fVar32 = (float)(~uVar29 & (uint)fVar32 | (uint)extraout_XMM0_Da_07 & uVar29) + fVar24;
        fVar36 = (float)(~uVar25 & (~uVar31 & (~uVar27 & uVar7 | uVar18 & uVar27) | uVar19 & uVar31)
                        | auVar23._0_4_ & uVar25) + fVar24;
        fVar37 = extraout_XMM0_Db_01 + 0.0;
        fVar17 = fVar17 + 0.0;
        break;
      default:
        goto code_r0x000141c50cee;
      case 10:
code_r0x000141c50ae0:
        country_core::resources::window::WindowSize::as_vec2();
        fVar32 = extraout_XMM0_Da * 0.5;
        fVar36 = auVar23._0_4_ * 0.5;
        fVar37 = extraout_XMM0_Db * 0.0;
        fVar17 = auVar23._4_4_ * 0.0;
      }
    }
    country_core::resources::window::WindowSize::as_vec2();
    fVar26 = (float)puVar15[1] - (float)*puVar15;
    fVar28 = (float)((ulonglong)puVar15[1] >> 0x20) - (float)((ulonglong)*puVar15 >> 0x20);
    auVar34._0_4_ = fVar32 + fVar26;
    auVar34._4_4_ = fVar36 + fVar28;
    auVar34._8_4_ = fVar37 + 0.0;
    auVar34._12_4_ = fVar17 + 0.0;
    fVar17 = (extraout_XMM0_Da_10 - (float)puVar15[2]) - auVar20._0_4_;
    fVar32 = (auVar23._0_4_ - (float)((ulonglong)puVar15[2] >> 0x20)) - auVar20._0_4_;
    auVar21._4_4_ = auVar20._0_4_;
    auVar21._0_4_ = auVar20._0_4_;
    auVar21._12_4_ = auVar21._8_4_;
    auVar22 = maxps(auVar21,auVar34);
    auVar30._0_4_ = (uint)auVar20._0_4_ & -(uint)NAN(auVar34._0_4_);
    auVar30._4_4_ = (uint)auVar20._0_4_ & -(uint)NAN(auVar34._4_4_);
    auVar30._8_4_ = (uint)auVar21._8_4_ & -(uint)NAN(auVar34._8_4_);
    auVar30._12_4_ = (uint)auVar21._8_4_ & -(uint)NAN(auVar34._12_4_);
    auVar35._0_4_ = ~-(uint)NAN(auVar34._0_4_) & auVar22._0_4_;
    auVar35._4_4_ = ~-(uint)NAN(auVar34._4_4_) & auVar22._4_4_;
    auVar35._8_4_ = ~-(uint)NAN(auVar34._8_4_) & auVar22._8_4_;
    auVar35._12_4_ = ~-(uint)NAN(auVar34._12_4_) & auVar22._12_4_;
    auVar35 = auVar35 | auVar30;
    auVar22._4_4_ = fVar32;
    auVar22._0_4_ = fVar17;
    auVar22._8_4_ = (extraout_XMM0_Db_05 - 0.0) - auVar21._8_4_;
    auVar22._12_4_ = (auVar23._4_4_ - 0.0) - auVar21._8_4_;
    auVar23 = minps(auVar22,auVar35);
    uVar33 = CONCAT44((float)(~-(uint)NAN(auVar35._4_4_) & auVar23._4_4_ |
                             (uint)fVar32 & -(uint)NAN(auVar35._4_4_)) - fVar28,
                      (float)(~-(uint)NAN(auVar35._0_4_) & auVar23._0_4_ |
                             (uint)fVar17 & -(uint)NAN(auVar35._0_4_)) - fVar26);
    *local_178 = uVar3;
    *local_160 = uVar33;
    if (*(char *)(local_160 + 2) == '\0') {
      local_160[1] = uVar33;
      *local_178 = uVar3;
      *(undefined1 *)(local_160 + 2) = 1;
    }
    else {
      ::utils::time_continuous_lerp();
      *local_178 = uVar3;
      *(undefined4 *)(local_160 + 1) = extraout_XMM0_Da_11;
      auVar23 = ZEXT416(*(uint *)((longlong)local_160 + 4));
      ::utils::time_continuous_lerp();
      *local_178 = uVar3;
      *(undefined4 *)((longlong)local_160 + 0xc) = extraout_XMM0_Da_12;
    }
  } while( true );
}

