// 0x142315050 update_sheep_goals
// SIG: void __cdecl update_sheep_goals(void)

void __cdecl system_critters::sheep::sheep_behavior::update_sheep_goals(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  float fVar4;
  char cVar5;
  undefined4 uVar6;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  bool bVar11;
  char extraout_AL;
  char cVar12;
  uint uVar13;
  longlong *in_RCX;
  longlong lVar14;
  undefined1 uVar15;
  longlong in_RDX;
  ulonglong uVar16;
  float *pfVar17;
  longlong *in_R8;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  uint *puVar23;
  uint *puVar24;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar25;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar26;
  undefined4 extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float unaff_XMM7_Da;
  float unaff_XMM8_Da;
  undefined8 local_1d8;
  float local_1d0;
  longlong local_1b8;
  longlong local_1b0;
  undefined8 local_1a0;
  float local_198;
  longlong local_170;
  longlong local_168;
  longlong local_160;
  longlong local_158;
  char local_14c;
  longlong local_130;
  undefined8 local_108;
  float local_100;
  
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  lVar8 = *in_R8;
  fVar4 = *(float *)(lVar8 + 0x28);
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar24 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar24 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  local_130 = 8;
  local_168 = 0;
  local_170 = 0;
  local_1b0 = 0;
  local_1b8 = 0;
  uVar16 = 0;
  uVar20 = 0;
  do {
    if (cVar5 == '\0') {
      if (uVar20 == uVar16) {
        do {
          if (puVar24 == puVar1) {
            return;
          }
          uVar13 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar16 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar13 * 0xa0);
        } while (uVar16 == 0);
        lVar18 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar13 * 0xa0;
        uVar20 = (ulonglong)*(uint *)(lVar18 + 0x94);
        lVar19 = *(longlong *)(lVar10 + 0x1a8);
        lVar22 = lVar19 + uVar20 * 0x48;
        lVar14 = *(longlong *)(lVar19 + 0x38 + uVar20 * 0x48);
        uVar20 = *(ulonglong *)(lVar19 + 0x40 + uVar20 * 0x48);
        if (*(ulonglong *)(lVar9 + 0x110) < uVar20) {
          uVar21 = *(ulonglong *)(lVar14 + *(ulonglong *)(lVar9 + 0x110) * 8);
          lVar22 = *(longlong *)(lVar22 + 0x18);
          if (uVar21 == 0) goto LAB_1423154b1;
          local_1b0 = *(longlong *)(lVar22 + 0x10 + ~uVar21 * 0x30);
          uVar21 = *(ulonglong *)(lVar9 + 0x128);
          if (uVar21 < uVar20) goto LAB_1423154ce;
LAB_1423154ee:
          local_1b8 = 0;
        }
        else {
          lVar22 = *(longlong *)(lVar22 + 0x18);
LAB_1423154b1:
          local_1b0 = 0;
          uVar21 = *(ulonglong *)(lVar9 + 0x128);
          if (uVar20 <= uVar21) goto LAB_1423154ee;
LAB_1423154ce:
          uVar20 = *(ulonglong *)(lVar14 + uVar21 * 8);
          if (uVar20 == 0) goto LAB_1423154ee;
          local_1b8 = *(longlong *)(lVar22 + 0x10 + ~uVar20 * 0x30);
        }
        lVar19 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_158 = *(longlong *)(lVar22 + 0x28 + lVar19);
        lVar19 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_168 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_160 = *(longlong *)(lVar22 + 0x28 + lVar19);
        local_130 = *(longlong *)(lVar18 + 0x50);
        uVar20 = 0;
      }
      uVar21 = (ulonglong)*(uint *)(local_130 + 8 + uVar20 * 0x10);
    }
    else {
      if (uVar20 == uVar16) {
        do {
          if (puVar24 == puVar1) {
            return;
          }
          uVar13 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar16 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar13 * 0x48);
        } while (uVar16 == 0);
        lVar22 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar13 * 0x48;
        lVar19 = *(longlong *)(lVar22 + 0x38);
        if (*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar22 + 0x40)) {
          uVar20 = *(ulonglong *)(lVar19 + *(ulonglong *)(lVar9 + 0x110) * 8);
          lVar14 = *(longlong *)(lVar22 + 0x18);
          if (uVar20 == 0) goto LAB_1423153fc;
          local_1b0 = *(longlong *)(lVar14 + 0x10 + ~uVar20 * 0x30);
        }
        else {
          lVar14 = *(longlong *)(lVar22 + 0x18);
LAB_1423153fc:
          local_1b0 = 0;
        }
        lVar18 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar14 + 0x10 + lVar18);
        local_158 = *(longlong *)(lVar14 + 0x28 + lVar18);
        lVar18 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_168 = *(longlong *)(lVar14 + 0x10 + lVar18);
        local_160 = *(longlong *)(lVar14 + 0x28 + lVar18);
        if ((*(ulonglong *)(lVar9 + 0x128) < *(ulonglong *)(lVar22 + 0x40)) &&
           (uVar20 = *(ulonglong *)(lVar19 + *(ulonglong *)(lVar9 + 0x128) * 8), uVar20 != 0)) {
          local_1b8 = *(longlong *)(lVar14 + 0x10 + ~uVar20 * 0x30);
        }
        else {
          local_1b8 = 0;
        }
        uVar20 = 0;
      }
      uVar21 = uVar20 & 0xffffffff;
    }
    pfVar17 = (float *)(uVar21 * 0x30 + local_1b0);
    lVar22 = uVar21 * 0x68 + local_170;
    puVar2 = (undefined4 *)(local_158 + uVar21 * 4);
    puVar3 = (undefined4 *)(local_160 + uVar21 * 4);
    puVar23 = (uint *)(uVar21 * 0x10 + local_168);
    *puVar2 = uVar6;
    iVar7 = *(int *)(lVar22 + 8);
    uVar15 = 2;
    if (iVar7 == 1) {
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL == '\x03') {
        country_core::systems::camera_rig::MainCamera::eye_position();
        country_core::systems::camera_rig::MainCamera::eye_forward();
        fVar25 = (float)((ulonglong)local_1d8 >> 0x20);
        local_1d8 = CONCAT44((float)((ulonglong)local_1a0 >> 0x20) + fVar25 + fVar25,
                             (float)local_1a0 + (float)local_1d8 + (float)local_1d8);
        local_1d0 = local_1d0 + local_1d0 + local_198;
      }
      else {
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      }
      local_198 = local_1d0;
      local_1a0 = local_1d8;
      *puVar3 = uVar6;
      *puVar23 = 1;
      *(undefined8 *)(puVar23 + 1) = local_1d8;
      puVar23[3] = (uint)local_1d0;
      *(float *)(lVar22 + 0xc) = *(float *)(lVar22 + 0xc) - *(float *)(lVar8 + 0x28);
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      if (0.2 <= SQRT((local_1d0 - pfVar17[6]) * (local_1d0 - pfVar17[6]) +
                      (local_1d8._4_4_ - pfVar17[5]) * (local_1d8._4_4_ - pfVar17[5]) +
                      ((float)local_1d8 - pfVar17[4]) * ((float)local_1d8 - pfVar17[4]))) {
        fVar25 = *(float *)(lVar22 + 0xc);
      }
      else {
        fVar25 = *(float *)(lVar8 + 0x28) * -3.0 + *(float *)(lVar22 + 0xc);
        *(float *)(lVar22 + 0xc) = fVar25;
      }
      uVar15 = 0.0 <= fVar25;
      if ((bool)uVar15) {
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM7_Da = extraout_XMM0_Da_02 * 12.0 + 4.0;
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM8_Da = extraout_XMM0_Da_03 * 3.0 + 1.0;
      }
      else {
        *puVar2 = uVar6;
        *(undefined4 *)(lVar22 + 8) = 0;
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM7_Da = extraout_XMM0_Da_01 * 7.0 + 1.0;
      }
    }
    uVar20 = uVar20 + 1;
    *puVar2 = uVar6;
    local_14c = (char)iVar7;
    if (*(char *)(lVar22 + 0x30) != '\0') {
      fVar25 = (float)*(undefined8 *)(lVar22 + 0x34) - fVar4;
      fVar27 = (float)((ulonglong)*(undefined8 *)(lVar22 + 0x34) >> 0x20) - fVar4;
      *(ulonglong *)(lVar22 + 0x34) = CONCAT44(fVar27,fVar25);
      if (0.0 < fVar25) {
        if (0.0 < fVar27) {
LAB_142315849:
          uVar13 = *puVar23;
          if (((uVar13 & 1) != 0) &&
             (0.16000001 <=
              ((float)puVar23[1] - pfVar17[4]) * ((float)puVar23[1] - pfVar17[4]) +
              ((float)puVar23[3] - pfVar17[6]) * ((float)puVar23[3] - pfVar17[6])))
          goto joined_r0x000142315dbf;
        }
        else {
          cVar12 = *(char *)(lVar22 + 0x31);
LAB_142315841:
          if (cVar12 != '\0') goto LAB_142315849;
          uVar13 = *puVar23;
        }
        fVar25 = *(float *)(&__real_3f0000003f4ccccd + (ulonglong)(uVar13 == 0) * 4);
        perchance::PerchanceContext::uniform_f32();
        *(float *)(lVar22 + 0x38) = extraout_XMM0_Da_05 * 3.0 + 1.0;
        perchance::PerchanceContext::uniform_f32();
        perchance::PerchanceContext::uniform_f32();
        ::utils::uniform_sample_disk();
        ::utils::swizzlers::impl$0::x0y();
        fVar27 = *pfVar17;
        fVar28 = pfVar17[1];
        fVar31 = pfVar17[2];
        fVar29 = pfVar17[3];
        fVar32 = fVar29 * fVar29;
        fVar33 = fVar31 * fVar31 + fVar28 * fVar28 + fVar27 * fVar27;
        fVar30 = fVar31 * -1.0 + fVar28 * -0.0 + fVar27 * -0.0;
        fVar30 = fVar30 + fVar30;
        fVar29 = fVar29 + fVar29;
        local_1d0 = (fVar29 * (fVar27 * -0.0 - fVar28 * -0.0) +
                    fVar30 * fVar31 + (fVar32 - fVar33) * -1.0) * 0.5;
        fVar26 = (fVar29 * (fVar28 * -1.0 - fVar31 * -0.0) +
                 fVar30 * fVar27 + (fVar32 - fVar33) * -0.0) * 0.5;
        fVar28 = (fVar29 * (fVar31 * -0.0 - fVar27 * -1.0) +
                 fVar30 * fVar28 + (fVar32 - fVar33) * -0.0) * 0.5;
        local_1d8 = CONCAT44(fVar28,fVar26);
        ::utils::swizzlers::impl$3::x0z();
        fVar29 = fVar25 * local_198 + local_100;
        fVar27 = (float)local_108 + (float)local_1a0 * fVar25;
        fVar25 = (float)((ulonglong)local_108 >> 0x20) +
                 (float)((ulonglong)local_1a0 >> 0x20) * fVar25;
        fVar31 = 1.0 / SQRT(fVar29 * fVar29 + fVar25 * fVar25 + fVar27 * fVar27);
        bVar11 = (int)fVar31 - 1U < 0x7fffff ||
                 (int)ABS(fVar31) - 0x800000U < 0x7f000000 && -1 < (int)fVar31;
        if (bVar11) {
          fVar29 = fVar29 * fVar31;
        }
        else {
          fVar29 = 0.0;
        }
        uVar13 = (uint)bVar11;
        fVar30 = (float)((int)(uVar13 << 0x1f) >> 0x1f & (uint)(fVar27 * fVar31));
        fVar31 = (float)((int)(uVar13 << 0x1f) >> 0x1f & (uint)(fVar25 * fVar31));
        fVar25 = *(float *)(lVar22 + 0x38);
        fVar32 = (float)*(undefined8 *)(pfVar17 + 4);
        fVar33 = (float)((ulonglong)*(undefined8 *)(pfVar17 + 4) >> 0x20);
        fVar27 = pfVar17[6];
        if ((*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') &&
           (35.0 < SQRT(fVar27 * fVar27 + fVar33 * fVar33 + fVar32 * fVar32))) {
          perchance::PerchanceContext::uniform_f32();
          perchance::PerchanceContext::uniform_f32();
          ::utils::uniform_sample_disk();
          local_1a0 = CONCAT44(fVar27,extraout_XMM0_Da_06);
          ::utils::swizzlers::impl$0::x0y();
          fVar32 = (float)*(undefined8 *)(pfVar17 + 4);
          fVar33 = (float)((ulonglong)*(undefined8 *)(pfVar17 + 4) >> 0x20);
          fVar27 = pfVar17[6];
          fVar30 = fVar26 * 25.0 - fVar32;
          fVar31 = fVar28 * 25.0 - fVar33;
          fVar28 = local_1d0 * 25.0 - fVar27;
          fVar29 = 1.0 / SQRT(fVar28 * fVar28 + fVar31 * fVar31 + fVar30 * fVar30);
          fVar30 = fVar29 * fVar30;
          fVar31 = fVar29 * fVar31;
          fVar29 = fVar29 * fVar28;
        }
        fVar26 = fVar25 * fVar30 + fVar32;
        fVar29 = fVar29 * fVar25 + fVar27;
        fVar28 = SQRT(fVar29 * fVar29 + fVar26 * fVar26);
        if ((fVar28 < 65.0) || (fVar28 < SQRT(fVar27 * fVar27 + fVar32 * fVar32))) {
          if ((*(int *)(local_1b8 + uVar21 * 8) != 2) ||
             ((::utils::float_grid::impl$0::
               bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                         (), extraout_XMM0_Da_07 <= 0.0 &&
              ((::utils::float_grid::impl$0::
                bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                          (), 0.0 < extraout_XMM0_Da_08 ||
               (::utils::float_grid::impl$0::
                bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                          (), extraout_XMM0_Da_09 <= 0.0)))))) {
            *puVar3 = uVar6;
            *puVar23 = 1;
            *(ulonglong *)(puVar23 + 1) = CONCAT44(fVar25 * fVar31 + fVar33,fVar26);
            puVar23[3] = (uint)fVar29;
          }
          else {
            *puVar3 = uVar6;
            *puVar23 = 0;
          }
          *(undefined1 *)(lVar22 + 0x31) = 0;
        }
        else {
          *puVar3 = uVar6;
          *puVar23 = 0;
          *(undefined1 *)(lVar22 + 0x31) = 0;
        }
        goto joined_r0x000142315dbf;
      }
      cVar12 = *(char *)(lVar22 + 0x31);
      if (fVar27 <= 0.0) {
        if (cVar12 != '\0') goto LAB_142315841;
      }
      else if (cVar12 != '\0') goto LAB_142315849;
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM7_Da = extraout_XMM0_Da_04 * 7.0 + 1.0;
      uVar15 = 0;
      goto LAB_14231528d;
    }
    fVar25 = *(float *)(lVar22 + 0x34) - fVar4;
    *(float *)(lVar22 + 0x34) = fVar25;
    if (fVar25 <= 0.0) {
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM7_Da = extraout_XMM0_Da * 12.0 + 4.0;
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM8_Da = extraout_XMM0_Da_00 * 3.0 + 1.0;
      uVar15 = 1;
      goto LAB_14231528d;
    }
joined_r0x000142315dbf:
    if (local_14c != '\0') {
LAB_14231528d:
      *puVar2 = uVar6;
      *(undefined1 *)(lVar22 + 0x30) = uVar15;
      *(undefined1 *)(lVar22 + 0x31) = 0;
      *(float *)(lVar22 + 0x34) = unaff_XMM7_Da;
      *(float *)(lVar22 + 0x38) = unaff_XMM8_Da;
    }
  } while( true );
}

