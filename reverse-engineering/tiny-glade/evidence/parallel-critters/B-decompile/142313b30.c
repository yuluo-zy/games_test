
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_critters::duck::update_duck_goals(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  float fVar4;
  undefined8 uVar5;
  char cVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  float fVar10;
  float fVar11;
  char cVar12;
  undefined1 uVar13;
  char extraout_AL;
  uint uVar14;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  uint *puVar19;
  longlong lVar20;
  longlong *in_R8;
  longlong lVar21;
  longlong lVar22;
  uint *puVar23;
  ulonglong uVar24;
  bool bVar25;
  bool bVar26;
  float fVar27;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float fVar28;
  float extraout_XMM0_Db;
  float fVar29;
  float fVar30;
  float fVar31;
  longlong in_stack_00000028;
  longlong local_1b8;
  longlong local_1a0;
  longlong local_170;
  undefined8 local_168;
  float local_160;
  undefined8 local_158;
  float local_150;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  fVar11 = __real_3e23d70b;
  fVar10 = __real_3f4ccccd;
  fVar4 = *(float *)(*in_R8 + 0x28);
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar23 = *(uint **)(lVar8 + 0xf8);
  puVar1 = puVar23 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  local_170 = 8;
  lVar22 = 0;
  local_1b8 = 0;
  lVar21 = 0;
  uVar24 = 0;
  uVar17 = 0;
LAB_142313c95:
  if (cVar6 == '\0') {
    if (uVar17 == uVar24) {
      if (puVar23 == puVar1) {
        return;
      }
      while( true ) {
        uVar24 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)*puVar23 * 0xa0);
        if (uVar24 != 0) break;
        puVar23 = puVar23 + 1;
        if (puVar23 == puVar1) {
          return;
        }
      }
      lVar15 = *(longlong *)(lVar9 + 0x100) + (ulonglong)*puVar23 * 0xa0;
      uVar17 = (ulonglong)*(uint *)(lVar15 + 0x94);
      lVar21 = *(longlong *)(lVar9 + 0x1a8);
      lVar22 = lVar21 + uVar17 * 0x48;
      lVar20 = *(longlong *)(lVar21 + 0x38 + uVar17 * 0x48);
      if (*(ulonglong *)(lVar8 + 0x110) < *(ulonglong *)(lVar21 + 0x40 + uVar17 * 0x48)) {
        uVar17 = *(ulonglong *)(lVar20 + *(ulonglong *)(lVar8 + 0x110) * 8);
        lVar16 = *(longlong *)(lVar22 + 0x18);
        if (uVar17 == 0) goto LAB_142313e15;
        lVar22 = *(longlong *)(lVar16 + 0x10 + ~uVar17 * 0x30);
      }
      else {
        lVar16 = *(longlong *)(lVar22 + 0x18);
LAB_142313e15:
        lVar22 = 0;
      }
      puVar23 = puVar23 + 1;
      lVar21 = ~*(ulonglong *)(lVar20 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
      local_1b8 = *(longlong *)(lVar16 + 0x10 + lVar21);
      local_1a0 = *(longlong *)(lVar16 + 0x28 + lVar21);
      lVar20 = ~*(ulonglong *)(lVar20 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
      lVar21 = *(longlong *)(lVar16 + 0x10 + lVar20);
      in_stack_00000028 = *(longlong *)(lVar16 + 0x28 + lVar20);
      local_170 = *(longlong *)(lVar15 + 0x50);
      uVar17 = 0;
    }
    uVar18 = (ulonglong)*(uint *)(local_170 + 8 + uVar17 * 0x10);
  }
  else {
    if (uVar17 == uVar24) {
      if (puVar23 == puVar1) {
        return;
      }
      while( true ) {
        uVar24 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)*puVar23 * 0x48);
        if (uVar24 != 0) break;
        puVar23 = puVar23 + 1;
        if (puVar23 == puVar1) {
          return;
        }
      }
      lVar22 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*puVar23 * 0x48;
      lVar21 = *(longlong *)(lVar22 + 0x38);
      if (*(ulonglong *)(lVar8 + 0x110) < *(ulonglong *)(lVar22 + 0x40)) {
        uVar17 = *(ulonglong *)(lVar21 + *(ulonglong *)(lVar8 + 0x110) * 8);
        lVar20 = *(longlong *)(lVar22 + 0x18);
        if (uVar17 == 0) goto LAB_142313db5;
        lVar22 = *(longlong *)(lVar20 + 0x10 + ~uVar17 * 0x30);
      }
      else {
        lVar20 = *(longlong *)(lVar22 + 0x18);
LAB_142313db5:
        lVar22 = 0;
      }
      puVar23 = puVar23 + 1;
      lVar15 = ~*(ulonglong *)(lVar21 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
      local_1b8 = *(longlong *)(lVar20 + 0x10 + lVar15);
      local_1a0 = *(longlong *)(lVar20 + 0x28 + lVar15);
      lVar15 = ~*(ulonglong *)(lVar21 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
      lVar21 = *(longlong *)(lVar20 + 0x10 + lVar15);
      in_stack_00000028 = *(longlong *)(lVar20 + 0x28 + lVar15);
      uVar17 = 0;
    }
    uVar18 = uVar17 & 0xffffffff;
  }
  lVar15 = uVar18 * 0x30 + lVar22;
  lVar20 = uVar18 * 0x100 + local_1b8;
  puVar2 = (undefined4 *)(local_1a0 + uVar18 * 4);
  puVar3 = (undefined4 *)(in_stack_00000028 + uVar18 * 4);
  puVar19 = (uint *)(uVar18 * 0x10 + lVar21);
  uVar17 = uVar17 + 1;
  *puVar2 = uVar7;
  if (*(char *)(lVar20 + 0xe0) == '\0') goto LAB_142313f00;
  fVar27 = (float)*(undefined8 *)(lVar20 + 0xe4) - fVar4;
  fVar28 = (float)((ulonglong)*(undefined8 *)(lVar20 + 0xe4) >> 0x20) - fVar4;
  *(ulonglong *)(lVar20 + 0xe4) = CONCAT44(fVar28,fVar27);
  if (fVar27 <= 0.0) {
    cVar12 = *(char *)(lVar20 + 0xe1);
    if (fVar28 <= 0.0) {
      if (cVar12 != '\0') goto LAB_142313f76;
    }
    else if (cVar12 != '\0') goto LAB_142313f7e;
    perchance::PerchanceContext::uniform_f32();
    fVar28 = extraout_XMM0_Da_01 * __real_3f000000 + __real_3f000000;
    uVar13 = 0;
    fVar27 = extraout_XMM0_Db;
    goto LAB_142314000;
  }
  if (0.0 < fVar28) {
LAB_142313f7e:
    uVar14 = *puVar19;
    if (((uVar14 & 1) != 0) &&
       (fVar27 = (float)puVar19[2] - *(float *)(lVar15 + 0x14),
       fVar28 = (float)puVar19[1] - *(float *)(lVar15 + 0x10),
       fVar30 = (float)puVar19[3] - *(float *)(lVar15 + 0x18),
       fVar11 <= fVar30 * fVar30 + fVar27 * fVar27 + fVar28 * fVar28)) goto LAB_142313c95;
  }
  else {
    cVar12 = *(char *)(lVar20 + 0xe1);
LAB_142313f76:
    if (cVar12 != '\0') goto LAB_142313f7e;
    uVar14 = *puVar19;
  }
  fVar28 = 0.0;
  fVar27 = 0.0;
  if (uVar14 != 0) {
    fVar27 = fVar10;
  }
  *(undefined4 *)(lVar20 + 0xe8) = 0x3dcccccd;
  perchance::PerchanceContext::uniform_f32();
  perchance::PerchanceContext::uniform_f32();
  ::utils::uniform_sample_disk();
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$3::x0z();
  fVar31 = fVar27 * local_150 + local_160;
  fVar30 = (float)local_168 + (float)local_158 * fVar27;
  fVar27 = (float)((ulonglong)local_168 >> 0x20) + (float)((ulonglong)local_158 >> 0x20) * fVar27;
  fVar29 = __real_3f800000 / SQRT(fVar31 * fVar31 + fVar27 * fVar27 + fVar30 * fVar30);
  bVar25 = (int)ABS(fVar29) - 0x800000U < 0x7f000000;
  bVar26 = (int)fVar29 - 1U < 0x7fffff;
  if (bVar26 || bVar25 && -1 < (int)fVar29) {
    fVar28 = fVar31 * fVar29;
  }
  uVar14 = (uint)(bVar26 || bVar25 && -1 < (int)fVar29);
  uVar18 = CONCAT44((int)(uVar14 << 0x1f) >> 0x1f,(int)(uVar14 << 0x1f) >> 0x1f) &
           CONCAT44(fVar27 * fVar29,fVar30 * fVar29);
  uVar5 = *(undefined8 *)(lVar15 + 0x10);
  fVar27 = (float)uVar5 + (float)uVar18;
  fVar28 = fVar28 + *(float *)(lVar15 + 0x18);
  country_core::game_config::is_within_glade_shape();
  if (((extraout_AL == '\0') &&
      (SQRT(*(float *)(lVar15 + 0x18) * *(float *)(lVar15 + 0x18) +
            *(float *)(lVar15 + 0x10) * *(float *)(lVar15 + 0x10)) <=
       SQRT(fVar28 * fVar28 + fVar27 * fVar27))) ||
     (::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>(),
     extraout_XMM0_Da_02 <= __real_3f000000)) {
LAB_1423143cc:
    *puVar3 = uVar7;
    *puVar19 = 0;
  }
  else {
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    fVar29 = extraout_XMM0_Da_06 + extraout_XMM0_Da_05 + extraout_XMM0_Da_04 + extraout_XMM0_Da_03;
    fVar30 = 0.0;
    if (0.0 <= fVar29) {
      fVar30 = fVar29;
    }
    fVar29 = __real_3f800000;
    if (fVar30 <= __real_3f800000) {
      fVar29 = fVar30;
    }
    if (__real_3dcccccd <= fVar29) goto LAB_1423143cc;
    *puVar3 = uVar7;
    *puVar19 = 1;
    *(ulonglong *)(puVar19 + 1) =
         CONCAT44((float)((ulonglong)uVar5 >> 0x20) + (float)(uVar18 >> 0x20),fVar27);
    puVar19[3] = (uint)fVar28;
  }
  *(undefined1 *)(lVar20 + 0xe1) = 0;
  goto LAB_142313c95;
LAB_142313f00:
  fVar27 = *(float *)(lVar20 + 0xe4) - fVar4;
  *(float *)(lVar20 + 0xe4) = fVar27;
  if (fVar27 <= 0.0) {
    perchance::PerchanceContext::uniform_f32();
    perchance::PerchanceContext::uniform_f32();
    fVar28 = extraout_XMM0_Da * (float)__xmm_00000000000000004040000040400000 +
             (float)__xmm_00000000000000003f8000003f800000;
    fVar27 = extraout_XMM0_Da_00 * __xmm_00000000000000004040000040400000._4_4_ +
             __xmm_00000000000000003f8000003f800000._4_4_;
    uVar13 = 1;
LAB_142314000:
    *puVar2 = uVar7;
    *(undefined1 *)(lVar20 + 0xe0) = uVar13;
    *(undefined1 *)(lVar20 + 0xe1) = 0;
    *(ulonglong *)(lVar20 + 0xe4) = CONCAT44(fVar27,fVar28);
  }
  goto LAB_142313c95;
}

