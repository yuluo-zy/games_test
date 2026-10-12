
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar12;
  float fVar13;
  float fVar14;
  char extraout_AL;
  char cVar15;
  uint uVar16;
  longlong *in_RCX;
  longlong lVar17;
  undefined1 uVar18;
  longlong in_RDX;
  ulonglong uVar19;
  float *pfVar20;
  longlong *in_R8;
  longlong lVar21;
  longlong lVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong lVar25;
  uint *puVar26;
  uint *puVar27;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar28;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar29;
  float fVar30;
  undefined4 extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
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
  fVar14 = __real_40e00000;
  fVar13 = __real_3e4ccccd;
  fVar12 = __real_3f800000;
  lVar8 = *in_R8;
  fVar4 = *(float *)(lVar8 + 0x28);
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar27 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar27 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  local_130 = 8;
  local_168 = 0;
  local_170 = 0;
  local_1b0 = 0;
  local_1b8 = 0;
  uVar19 = 0;
  uVar23 = 0;
  fVar30 = __real_41400000;
  fVar32 = __real_40800000;
  do {
    if (cVar5 == '\0') {
      if (uVar23 == uVar19) {
        do {
          if (puVar27 == puVar1) {
            return;
          }
          uVar16 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar16 * 0xa0);
        } while (uVar19 == 0);
        lVar21 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar16 * 0xa0;
        uVar23 = (ulonglong)*(uint *)(lVar21 + 0x94);
        lVar22 = *(longlong *)(lVar10 + 0x1a8);
        lVar25 = lVar22 + uVar23 * 0x48;
        lVar17 = *(longlong *)(lVar22 + 0x38 + uVar23 * 0x48);
        uVar23 = *(ulonglong *)(lVar22 + 0x40 + uVar23 * 0x48);
        if (*(ulonglong *)(lVar9 + 0x110) < uVar23) {
          uVar24 = *(ulonglong *)(lVar17 + *(ulonglong *)(lVar9 + 0x110) * 8);
          lVar25 = *(longlong *)(lVar25 + 0x18);
          if (uVar24 == 0) goto LAB_1423154b1;
          local_1b0 = *(longlong *)(lVar25 + 0x10 + ~uVar24 * 0x30);
          uVar24 = *(ulonglong *)(lVar9 + 0x128);
          if (uVar24 < uVar23) goto LAB_1423154ce;
LAB_1423154ee:
          local_1b8 = 0;
        }
        else {
          lVar25 = *(longlong *)(lVar25 + 0x18);
LAB_1423154b1:
          local_1b0 = 0;
          uVar24 = *(ulonglong *)(lVar9 + 0x128);
          if (uVar23 <= uVar24) goto LAB_1423154ee;
LAB_1423154ce:
          uVar23 = *(ulonglong *)(lVar17 + uVar24 * 8);
          if (uVar23 == 0) goto LAB_1423154ee;
          local_1b8 = *(longlong *)(lVar25 + 0x10 + ~uVar23 * 0x30);
        }
        lVar22 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar25 + 0x10 + lVar22);
        local_158 = *(longlong *)(lVar25 + 0x28 + lVar22);
        lVar22 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_168 = *(longlong *)(lVar25 + 0x10 + lVar22);
        local_160 = *(longlong *)(lVar25 + 0x28 + lVar22);
        local_130 = *(longlong *)(lVar21 + 0x50);
        uVar23 = 0;
      }
      uVar24 = (ulonglong)*(uint *)(local_130 + 8 + uVar23 * 0x10);
    }
    else {
      if (uVar23 == uVar19) {
        do {
          if (puVar27 == puVar1) {
            return;
          }
          uVar16 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar16 * 0x48);
        } while (uVar19 == 0);
        lVar25 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar16 * 0x48;
        lVar22 = *(longlong *)(lVar25 + 0x38);
        if (*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar25 + 0x40)) {
          uVar23 = *(ulonglong *)(lVar22 + *(ulonglong *)(lVar9 + 0x110) * 8);
          lVar17 = *(longlong *)(lVar25 + 0x18);
          if (uVar23 == 0) goto LAB_1423153fc;
          local_1b0 = *(longlong *)(lVar17 + 0x10 + ~uVar23 * 0x30);
        }
        else {
          lVar17 = *(longlong *)(lVar25 + 0x18);
LAB_1423153fc:
          local_1b0 = 0;
        }
        lVar21 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_170 = *(longlong *)(lVar17 + 0x10 + lVar21);
        local_158 = *(longlong *)(lVar17 + 0x28 + lVar21);
        lVar21 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_168 = *(longlong *)(lVar17 + 0x10 + lVar21);
        local_160 = *(longlong *)(lVar17 + 0x28 + lVar21);
        if ((*(ulonglong *)(lVar9 + 0x128) < *(ulonglong *)(lVar25 + 0x40)) &&
           (uVar23 = *(ulonglong *)(lVar22 + *(ulonglong *)(lVar9 + 0x128) * 8), uVar23 != 0)) {
          local_1b8 = *(longlong *)(lVar17 + 0x10 + ~uVar23 * 0x30);
        }
        else {
          local_1b8 = 0;
        }
        uVar23 = 0;
      }
      uVar24 = uVar23 & 0xffffffff;
    }
    pfVar20 = (float *)(uVar24 * 0x30 + local_1b0);
    lVar25 = uVar24 * 0x68 + local_170;
    puVar2 = (undefined4 *)(local_158 + uVar24 * 4);
    puVar3 = (undefined4 *)(local_160 + uVar24 * 4);
    puVar26 = (uint *)(uVar24 * 0x10 + local_168);
    *puVar2 = uVar6;
    iVar7 = *(int *)(lVar25 + 8);
    uVar18 = 2;
    if (iVar7 == 1) {
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL == '\x03') {
        country_core::systems::camera_rig::MainCamera::eye_position();
        country_core::systems::camera_rig::MainCamera::eye_forward();
        fVar28 = (float)((ulonglong)local_1d8 >> 0x20);
        local_1d8 = CONCAT44((float)((ulonglong)local_1a0 >> 0x20) + fVar28 + fVar28,
                             (float)local_1a0 + (float)local_1d8 + (float)local_1d8);
        local_1d0 = local_1d0 + local_1d0 + local_198;
      }
      else {
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      }
      local_198 = local_1d0;
      local_1a0 = local_1d8;
      *puVar3 = uVar6;
      *puVar26 = 1;
      *(undefined8 *)(puVar26 + 1) = local_1d8;
      puVar26[3] = (uint)local_1d0;
      *(float *)(lVar25 + 0xc) = *(float *)(lVar25 + 0xc) - *(float *)(lVar8 + 0x28);
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      if (fVar13 <= SQRT((local_1d0 - pfVar20[6]) * (local_1d0 - pfVar20[6]) +
                         (local_1d8._4_4_ - pfVar20[5]) * (local_1d8._4_4_ - pfVar20[5]) +
                         ((float)local_1d8 - pfVar20[4]) * ((float)local_1d8 - pfVar20[4]))) {
        fVar28 = *(float *)(lVar25 + 0xc);
      }
      else {
        fVar28 = *(float *)(lVar8 + 0x28) * __real_c0400000 + *(float *)(lVar25 + 0xc);
        *(float *)(lVar25 + 0xc) = fVar28;
      }
      uVar18 = 0.0 <= fVar28;
      if ((bool)uVar18) {
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM7_Da = extraout_XMM0_Da_02 * fVar30 + fVar32;
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM8_Da = extraout_XMM0_Da_03 * __real_40400000 + fVar12;
      }
      else {
        *puVar2 = uVar6;
        *(undefined4 *)(lVar25 + 8) = 0;
        perchance::PerchanceContext::uniform_f32();
        unaff_XMM7_Da = extraout_XMM0_Da_01 * fVar14 + fVar12;
      }
    }
    uVar23 = uVar23 + 1;
    *puVar2 = uVar6;
    local_14c = (char)iVar7;
    if (*(char *)(lVar25 + 0x30) != '\0') {
      fVar28 = (float)*(undefined8 *)(lVar25 + 0x34) - fVar4;
      fVar31 = (float)((ulonglong)*(undefined8 *)(lVar25 + 0x34) >> 0x20) - fVar4;
      *(ulonglong *)(lVar25 + 0x34) = CONCAT44(fVar31,fVar28);
      if (0.0 < fVar28) {
        if (0.0 < fVar31) {
LAB_142315849:
          uVar16 = *puVar26;
          if (((uVar16 & 1) != 0) &&
             (__real_3e23d70b <=
              ((float)puVar26[1] - pfVar20[4]) * ((float)puVar26[1] - pfVar20[4]) +
              ((float)puVar26[3] - pfVar20[6]) * ((float)puVar26[3] - pfVar20[6])))
          goto joined_r0x000142315dbf;
        }
        else {
          cVar15 = *(char *)(lVar25 + 0x31);
LAB_142315841:
          if (cVar15 != '\0') goto LAB_142315849;
          uVar16 = *puVar26;
        }
        fVar32 = *(float *)(&__real_3f0000003f4ccccd + (ulonglong)(uVar16 == 0) * 4);
        perchance::PerchanceContext::uniform_f32();
        *(float *)(lVar25 + 0x38) = extraout_XMM0_Da_05 * __real_40400000 + fVar12;
        perchance::PerchanceContext::uniform_f32();
        perchance::PerchanceContext::uniform_f32();
        ::utils::uniform_sample_disk();
        ::utils::swizzlers::impl$0::x0y();
        fVar30 = *pfVar20;
        fVar28 = pfVar20[1];
        fVar31 = pfVar20[2];
        fVar33 = pfVar20[3];
        fVar35 = fVar33 * fVar33;
        fVar36 = fVar31 * fVar31 + fVar28 * fVar28 + fVar30 * fVar30;
        fVar34 = fVar31 * _UNK_1429a9088 +
                 fVar28 * _UNK_1429a9084 + fVar30 * ___xmm_00000000bf8000008000000080000000;
        fVar34 = fVar34 + fVar34;
        fVar33 = fVar33 + fVar33;
        local_1d0 = (fVar33 * (fVar30 * _UNK_1429a90a8 - fVar28 * _UNK_1429a9098) +
                    fVar34 * fVar31 + (fVar35 - fVar36) * _UNK_1429a9088) * __real_3f000000;
        fVar29 = (fVar33 * (fVar28 * ___xmm_000000008000000080000000bf800000 -
                           fVar31 * ___xmm_0000000080000000bf80000080000000) +
                 fVar34 * fVar30 + (fVar35 - fVar36) * ___xmm_00000000bf8000008000000080000000) *
                 (float)__xmm_00000000000000003f0000003f000000;
        fVar28 = (fVar33 * (fVar31 * _UNK_1429a90a4 - fVar30 * _UNK_1429a9094) +
                 fVar34 * fVar28 + (fVar35 - fVar36) * _UNK_1429a9084) *
                 __xmm_00000000000000003f0000003f000000._4_4_;
        local_1d8 = CONCAT44(fVar28,fVar29);
        ::utils::swizzlers::impl$3::x0z();
        fVar33 = fVar32 * local_198 + local_100;
        fVar30 = (float)local_108 + (float)local_1a0 * fVar32;
        fVar32 = (float)((ulonglong)local_108 >> 0x20) +
                 (float)((ulonglong)local_1a0 >> 0x20) * fVar32;
        fVar31 = fVar12 / SQRT(fVar33 * fVar33 + fVar32 * fVar32 + fVar30 * fVar30);
        bVar11 = (int)fVar31 - 1U < 0x7fffff ||
                 (int)ABS(fVar31) - 0x800000U < 0x7f000000 && -1 < (int)fVar31;
        if (bVar11) {
          fVar33 = fVar33 * fVar31;
        }
        else {
          fVar33 = 0.0;
        }
        uVar16 = (uint)bVar11;
        fVar34 = (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar30 * fVar31));
        fVar31 = (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar32 * fVar31));
        fVar32 = *(float *)(lVar25 + 0x38);
        fVar35 = (float)*(undefined8 *)(pfVar20 + 4);
        fVar36 = (float)((ulonglong)*(undefined8 *)(pfVar20 + 4) >> 0x20);
        fVar30 = pfVar20[6];
        if ((*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') &&
           (___real_420c0000 < SQRT(fVar30 * fVar30 + fVar36 * fVar36 + fVar35 * fVar35))) {
          perchance::PerchanceContext::uniform_f32();
          perchance::PerchanceContext::uniform_f32();
          ::utils::uniform_sample_disk();
          local_1a0 = CONCAT44(fVar30,extraout_XMM0_Da_06);
          ::utils::swizzlers::impl$0::x0y();
          fVar35 = (float)*(undefined8 *)(pfVar20 + 4);
          fVar36 = (float)((ulonglong)*(undefined8 *)(pfVar20 + 4) >> 0x20);
          fVar30 = pfVar20[6];
          fVar34 = fVar29 * ___xmm_000000000000000041c8000041c80000 - fVar35;
          fVar31 = fVar28 * _UNK_14304ad34 - fVar36;
          fVar28 = local_1d0 * __real_41c80000 - fVar30;
          fVar33 = fVar12 / SQRT(fVar28 * fVar28 + fVar31 * fVar31 + fVar34 * fVar34);
          fVar34 = fVar33 * fVar34;
          fVar31 = fVar33 * fVar31;
          fVar33 = fVar33 * fVar28;
        }
        fVar29 = fVar32 * fVar34 + fVar35;
        fVar33 = fVar33 * fVar32 + fVar30;
        fVar28 = SQRT(fVar33 * fVar33 + fVar29 * fVar29);
        if ((fVar28 < __real_42820000) || (fVar28 < SQRT(fVar30 * fVar30 + fVar35 * fVar35))) {
          if ((*(int *)(local_1b8 + uVar24 * 8) != 2) ||
             ((::utils::float_grid::impl$0::
               bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                         (), extraout_XMM0_Da_07 <= __real_00000000 &&
              ((::utils::float_grid::impl$0::
                bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                          (), __real_00000000 < extraout_XMM0_Da_08 ||
               (::utils::float_grid::impl$0::
                bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                          (), extraout_XMM0_Da_09 <= __real_00000000)))))) {
            *puVar3 = uVar6;
            *puVar26 = 1;
            *(ulonglong *)(puVar26 + 1) = CONCAT44(fVar32 * fVar31 + fVar36,fVar29);
            puVar26[3] = (uint)fVar33;
          }
          else {
            *puVar3 = uVar6;
            *puVar26 = 0;
          }
          *(undefined1 *)(lVar25 + 0x31) = 0;
          fVar32 = __real_40800000;
          fVar30 = __real_41400000;
        }
        else {
          *puVar3 = uVar6;
          *puVar26 = 0;
          *(undefined1 *)(lVar25 + 0x31) = 0;
          fVar32 = __real_40800000;
          fVar30 = __real_41400000;
        }
        goto joined_r0x000142315dbf;
      }
      cVar15 = *(char *)(lVar25 + 0x31);
      if (fVar31 <= 0.0) {
        if (cVar15 != '\0') goto LAB_142315841;
      }
      else if (cVar15 != '\0') goto LAB_142315849;
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM7_Da = extraout_XMM0_Da_04 * fVar14 + fVar12;
      uVar18 = 0;
      goto LAB_14231528d;
    }
    fVar28 = *(float *)(lVar25 + 0x34) - fVar4;
    *(float *)(lVar25 + 0x34) = fVar28;
    if (fVar28 <= 0.0) {
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM7_Da = extraout_XMM0_Da * fVar30 + fVar32;
      perchance::PerchanceContext::uniform_f32();
      unaff_XMM8_Da = extraout_XMM0_Da_00 * __real_40400000 + fVar12;
      uVar18 = 1;
      goto LAB_14231528d;
    }
joined_r0x000142315dbf:
    if (local_14c != '\0') {
LAB_14231528d:
      *puVar2 = uVar6;
      *(undefined1 *)(lVar25 + 0x30) = uVar18;
      *(undefined1 *)(lVar25 + 0x31) = 0;
      *(float *)(lVar25 + 0x34) = unaff_XMM7_Da;
      *(float *)(lVar25 + 0x38) = unaff_XMM8_Da;
    }
  } while( true );
}

