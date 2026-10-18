// 0x142313b30 update_duck_goals
// SIG: void __cdecl update_duck_goals(void)

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
  char cVar10;
  undefined1 uVar11;
  char extraout_AL;
  uint uVar12;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar13;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  uint *puVar17;
  longlong lVar18;
  longlong *in_R8;
  longlong lVar19;
  longlong lVar20;
  uint *puVar21;
  ulonglong uVar22;
  bool bVar23;
  bool bVar24;
  float fVar25;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float fVar26;
  float extraout_XMM0_Db;
  float fVar27;
  float fVar28;
  float fVar29;
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
  fVar4 = *(float *)(*in_R8 + 0x28);
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar21 = *(uint **)(lVar8 + 0xf8);
  puVar1 = puVar21 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  local_170 = 8;
  lVar20 = 0;
  local_1b8 = 0;
  lVar19 = 0;
  uVar22 = 0;
  uVar15 = 0;
LAB_142313c95:
  if (cVar6 == '\0') {
    if (uVar15 == uVar22) {
      if (puVar21 == puVar1) {
        return;
      }
      while( true ) {
        uVar22 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)*puVar21 * 0xa0);
        if (uVar22 != 0) break;
        puVar21 = puVar21 + 1;
        if (puVar21 == puVar1) {
          return;
        }
      }
      lVar13 = *(longlong *)(lVar9 + 0x100) + (ulonglong)*puVar21 * 0xa0;
      uVar15 = (ulonglong)*(uint *)(lVar13 + 0x94);
      lVar19 = *(longlong *)(lVar9 + 0x1a8);
      lVar20 = lVar19 + uVar15 * 0x48;
      lVar18 = *(longlong *)(lVar19 + 0x38 + uVar15 * 0x48);
      if (*(ulonglong *)(lVar8 + 0x110) < *(ulonglong *)(lVar19 + 0x40 + uVar15 * 0x48)) {
        uVar15 = *(ulonglong *)(lVar18 + *(ulonglong *)(lVar8 + 0x110) * 8);
        lVar14 = *(longlong *)(lVar20 + 0x18);
        if (uVar15 == 0) goto LAB_142313e15;
        lVar20 = *(longlong *)(lVar14 + 0x10 + ~uVar15 * 0x30);
      }
      else {
        lVar14 = *(longlong *)(lVar20 + 0x18);
LAB_142313e15:
        lVar20 = 0;
      }
      puVar21 = puVar21 + 1;
      lVar19 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
      local_1b8 = *(longlong *)(lVar14 + 0x10 + lVar19);
      local_1a0 = *(longlong *)(lVar14 + 0x28 + lVar19);
      lVar18 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
      lVar19 = *(longlong *)(lVar14 + 0x10 + lVar18);
      in_stack_00000028 = *(longlong *)(lVar14 + 0x28 + lVar18);
      local_170 = *(longlong *)(lVar13 + 0x50);
      uVar15 = 0;
    }
    uVar16 = (ulonglong)*(uint *)(local_170 + 8 + uVar15 * 0x10);
  }
  else {
    if (uVar15 == uVar22) {
      if (puVar21 == puVar1) {
        return;
      }
      while( true ) {
        uVar22 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)*puVar21 * 0x48);
        if (uVar22 != 0) break;
        puVar21 = puVar21 + 1;
        if (puVar21 == puVar1) {
          return;
        }
      }
      lVar20 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*puVar21 * 0x48;
      lVar19 = *(longlong *)(lVar20 + 0x38);
      if (*(ulonglong *)(lVar8 + 0x110) < *(ulonglong *)(lVar20 + 0x40)) {
        uVar15 = *(ulonglong *)(lVar19 + *(ulonglong *)(lVar8 + 0x110) * 8);
        lVar18 = *(longlong *)(lVar20 + 0x18);
        if (uVar15 == 0) goto LAB_142313db5;
        lVar20 = *(longlong *)(lVar18 + 0x10 + ~uVar15 * 0x30);
      }
      else {
        lVar18 = *(longlong *)(lVar20 + 0x18);
LAB_142313db5:
        lVar20 = 0;
      }
      puVar21 = puVar21 + 1;
      lVar13 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
      local_1b8 = *(longlong *)(lVar18 + 0x10 + lVar13);
      local_1a0 = *(longlong *)(lVar18 + 0x28 + lVar13);
      lVar13 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar8 + 0x120) * 8) * 0x30;
      lVar19 = *(longlong *)(lVar18 + 0x10 + lVar13);
      in_stack_00000028 = *(longlong *)(lVar18 + 0x28 + lVar13);
      uVar15 = 0;
    }
    uVar16 = uVar15 & 0xffffffff;
  }
  lVar13 = uVar16 * 0x30 + lVar20;
  lVar18 = uVar16 * 0x100 + local_1b8;
  puVar2 = (undefined4 *)(local_1a0 + uVar16 * 4);
  puVar3 = (undefined4 *)(in_stack_00000028 + uVar16 * 4);
  puVar17 = (uint *)(uVar16 * 0x10 + lVar19);
  uVar15 = uVar15 + 1;
  *puVar2 = uVar7;
  if (*(char *)(lVar18 + 0xe0) == '\0') goto LAB_142313f00;
  fVar25 = (float)*(undefined8 *)(lVar18 + 0xe4) - fVar4;
  fVar26 = (float)((ulonglong)*(undefined8 *)(lVar18 + 0xe4) >> 0x20) - fVar4;
  *(ulonglong *)(lVar18 + 0xe4) = CONCAT44(fVar26,fVar25);
  if (fVar25 <= 0.0) {
    cVar10 = *(char *)(lVar18 + 0xe1);
    if (fVar26 <= 0.0) {
      if (cVar10 != '\0') goto LAB_142313f76;
    }
    else if (cVar10 != '\0') goto LAB_142313f7e;
    perchance::PerchanceContext::uniform_f32();
    fVar26 = extraout_XMM0_Da_01 * 0.5 + 0.5;
    uVar11 = 0;
    fVar25 = extraout_XMM0_Db;
    goto LAB_142314000;
  }
  if (0.0 < fVar26) {
LAB_142313f7e:
    uVar12 = *puVar17;
    if (((uVar12 & 1) != 0) &&
       (fVar25 = (float)puVar17[2] - *(float *)(lVar13 + 0x14),
       fVar26 = (float)puVar17[1] - *(float *)(lVar13 + 0x10),
       fVar28 = (float)puVar17[3] - *(float *)(lVar13 + 0x18),
       0.16000001 <= fVar28 * fVar28 + fVar25 * fVar25 + fVar26 * fVar26)) goto LAB_142313c95;
  }
  else {
    cVar10 = *(char *)(lVar18 + 0xe1);
LAB_142313f76:
    if (cVar10 != '\0') goto LAB_142313f7e;
    uVar12 = *puVar17;
  }
  fVar26 = 0.0;
  fVar25 = 0.0;
  if (uVar12 != 0) {
    fVar25 = 0.8;
  }
  *(undefined4 *)(lVar18 + 0xe8) = 0x3dcccccd;
  perchance::PerchanceContext::uniform_f32();
  perchance::PerchanceContext::uniform_f32();
  ::utils::uniform_sample_disk();
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$3::x0z();
  fVar29 = fVar25 * local_150 + local_160;
  fVar28 = (float)local_168 + (float)local_158 * fVar25;
  fVar25 = (float)((ulonglong)local_168 >> 0x20) + (float)((ulonglong)local_158 >> 0x20) * fVar25;
  fVar27 = 1.0 / SQRT(fVar29 * fVar29 + fVar25 * fVar25 + fVar28 * fVar28);
  bVar23 = (int)ABS(fVar27) - 0x800000U < 0x7f000000;
  bVar24 = (int)fVar27 - 1U < 0x7fffff;
  if (bVar24 || bVar23 && -1 < (int)fVar27) {
    fVar26 = fVar29 * fVar27;
  }
  uVar12 = (uint)(bVar24 || bVar23 && -1 < (int)fVar27);
  uVar16 = CONCAT44((int)(uVar12 << 0x1f) >> 0x1f,(int)(uVar12 << 0x1f) >> 0x1f) &
           CONCAT44(fVar25 * fVar27,fVar28 * fVar27);
  uVar5 = *(undefined8 *)(lVar13 + 0x10);
  fVar25 = (float)uVar5 + (float)uVar16;
  fVar26 = fVar26 + *(float *)(lVar13 + 0x18);
  country_core::game_config::is_within_glade_shape();
  if (((extraout_AL == '\0') &&
      (SQRT(*(float *)(lVar13 + 0x18) * *(float *)(lVar13 + 0x18) +
            *(float *)(lVar13 + 0x10) * *(float *)(lVar13 + 0x10)) <=
       SQRT(fVar26 * fVar26 + fVar25 * fVar25))) ||
     (::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>(),
     extraout_XMM0_Da_02 <= 0.5)) {
LAB_1423143cc:
    *puVar3 = uVar7;
    *puVar17 = 0;
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
    fVar27 = extraout_XMM0_Da_06 + extraout_XMM0_Da_05 + extraout_XMM0_Da_04 + extraout_XMM0_Da_03;
    fVar28 = 0.0;
    if (0.0 <= fVar27) {
      fVar28 = fVar27;
    }
    fVar27 = 1.0;
    if (fVar28 <= 1.0) {
      fVar27 = fVar28;
    }
    if (0.1 <= fVar27) goto LAB_1423143cc;
    *puVar3 = uVar7;
    *puVar17 = 1;
    *(ulonglong *)(puVar17 + 1) =
         CONCAT44((float)((ulonglong)uVar5 >> 0x20) + (float)(uVar16 >> 0x20),fVar25);
    puVar17[3] = (uint)fVar26;
  }
  *(undefined1 *)(lVar18 + 0xe1) = 0;
  goto LAB_142313c95;
LAB_142313f00:
  fVar25 = *(float *)(lVar18 + 0xe4) - fVar4;
  *(float *)(lVar18 + 0xe4) = fVar25;
  if (fVar25 <= 0.0) {
    perchance::PerchanceContext::uniform_f32();
    perchance::PerchanceContext::uniform_f32();
    fVar26 = extraout_XMM0_Da * 3.0 + 1.0;
    fVar25 = extraout_XMM0_Da_00 * 3.0 + 1.0;
    uVar11 = 1;
LAB_142314000:
    *puVar2 = uVar7;
    *(undefined1 *)(lVar18 + 0xe0) = uVar11;
    *(undefined1 *)(lVar18 + 0xe1) = 0;
    *(ulonglong *)(lVar18 + 0xe4) = CONCAT44(fVar25,fVar26);
  }
  goto LAB_142313c95;
}

