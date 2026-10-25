
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::free::impl$1::translate_screen_space(void)

{
  float fVar1;
  undefined8 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float *pfVar6;
  longlong lVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  longlong in_RCX;
  float *in_RDX;
  longlong lVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  longlong lVar13;
  float fVar14;
  float fVar15;
  undefined8 extraout_XMM0_Qa;
  float fVar16;
  float fVar17;
  float fVar18;
  float in_XMM2_Da;
  float fVar19;
  float fVar20;
  float fVar21;
  undefined1 auVar22 [16];
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  char **local_128;
  code *local_120;
  char *local_118;
  undefined8 local_110;
  undefined *local_108;
  undefined8 local_100;
  undefined8 **local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  
  fVar24 = *(float *)(in_RCX + 0x18);
  fVar4 = *(float *)(in_RCX + 0x1c);
  fVar5 = *(float *)(in_RCX + 0x20);
  fVar27 = *(float *)(in_RCX + 0x24);
  fVar18 = (float)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20);
  fVar17 = *in_RDX;
  fVar20 = (float)*(undefined8 *)(in_RDX + 1);
  fVar19 = fVar27 * fVar27;
  fVar21 = fVar5 * fVar5 + fVar4 * fVar4 + fVar24 * fVar24;
  fVar16 = fVar5 * fVar18 + fVar4 * fVar20 + fVar24 * fVar17;
  fVar16 = fVar16 + fVar16;
  fVar27 = fVar27 + fVar27;
  puVar12 = *(undefined8 **)(in_RCX + 8);
  lVar10 = *(longlong *)(in_RCX + 0x10);
  lVar13 = lVar10 << 4;
  puVar11 = puVar12;
  do {
    if (lVar13 == 0) goto LAB_140993c4f;
    puVar9 = puVar11 + 2;
    pfVar6 = (float *)(**(code **)(puVar11[1] + 0x38))(*puVar11);
    lVar7 = (**(code **)(in_RDX + 6))(pfVar6);
    lVar13 = lVar13 + -0x10;
    puVar11 = puVar9;
  } while (((pfVar6 == (float *)0x0) || (lVar7 != 0x6b8463577afc1a66)) ||
          (in_RDX != (float *)0xe4b9f294168d7e3));
  fVar15 = pfVar6[1];
  fVar25 = *pfVar6 + fVar27 * (fVar4 * fVar18 - fVar20 * fVar5) +
                     fVar16 * fVar24 + (fVar19 - fVar21) * fVar17;
  fVar29 = fVar27 * (fVar24 * fVar20 - fVar17 * fVar4) + fVar16 * fVar5 + (fVar19 - fVar21) * fVar18
           + pfVar6[2];
  fVar14 = expf(in_XMM2_Da * __real_c1000000);
  fVar28 = __real_42b60000;
  fVar14 = __real_3f800000 - fVar14;
  fVar26 = __real_c2b60000;
  if (__real_c2b60000 <= fVar25) {
    fVar26 = fVar25;
  }
  fVar3 = __real_42b60000;
  if (fVar26 <= __real_42b60000) {
    fVar3 = fVar26;
  }
  fVar26 = __real_c2b60000;
  if (__real_c2b60000 <= fVar29) {
    fVar26 = fVar29;
  }
  fVar23 = __real_42b60000;
  if (fVar26 <= __real_42b60000) {
    fVar23 = fVar26;
  }
  resources::terrain_height::TerrainHeightsData::
  sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
  fVar26 = (float)extraout_XMM0_Qa;
  lVar13 = lVar10 << 4;
  puVar11 = puVar12;
  while (lVar13 != 0) {
    puVar9 = puVar11 + 2;
    puVar8 = (undefined8 *)(**(code **)(puVar11[1] + 0x38))(*puVar11);
    lVar7 = (**(code **)(in_RDX + 6))(puVar8);
    lVar13 = lVar13 + -0x10;
    puVar11 = puVar9;
    if (((puVar8 != (undefined8 *)0x0) && (lVar7 == 0x6b8463577afc1a66)) &&
       (in_RDX == (float *)0xe4b9f294168d7e3)) {
      uVar2 = *puVar8;
      fVar1 = *(float *)(puVar8 + 1);
      lVar10 = lVar10 << 4;
      lVar13 = 0xe4b9f294168d7e3;
      do {
        if (lVar10 == 0) {
          local_118 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
          local_110 = 0x22;
          local_128 = &local_118;
          local_120 = anyhow::context::impl$3::
                      fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
          local_108 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
          local_100 = 2;
          local_e8 = 0;
          local_f8 = &local_128;
          local_f0 = 1;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        puVar11 = puVar12 + 2;
        puVar9 = (undefined8 *)(**(code **)(puVar12[1] + 0x40))(*puVar12);
        lVar7 = (**(code **)(lVar13 + 0x18))(puVar9);
        lVar10 = lVar10 + -0x10;
        puVar12 = puVar11;
      } while (((puVar9 == (undefined8 *)0x0) || (lVar7 != 0x6b8463577afc1a66)) ||
              (lVar13 != 0xe4b9f294168d7e3));
      fVar15 = fVar27 * (fVar5 * fVar17 - fVar18 * fVar24) +
               fVar16 * fVar4 + (fVar19 - fVar21) * fVar20 + fVar15;
      if (fVar26 <= 0.0) {
        fVar26 = 0.0;
      }
      fVar17 = 0.0;
      if (0.0 <= fVar15) {
        fVar17 = fVar15;
      }
      if (fVar17 <= fVar28) {
        fVar28 = fVar17;
      }
      fVar26 = fVar26 + __real_3f400000;
      fVar15 = fVar14 * (fVar28 - fVar15) + fVar15;
      fVar17 = fVar26;
      if (fVar26 <= fVar15) {
        fVar17 = fVar15;
      }
      fVar24 = ((((fVar23 - fVar29) * fVar14 + fVar29) - fVar1) * __real_41a00000) / __real_42700000
      ;
      auVar22._0_4_ =
           (((fVar3 - fVar25) * fVar14 + fVar25) - (float)uVar2) *
           ___xmm_000000000000000041a0000041a00000;
      auVar22._4_4_ =
           ((float)(-(uint)NAN(fVar15) & (uint)fVar26 | ~-(uint)NAN(fVar15) & (uint)fVar17) -
           (float)((ulonglong)uVar2 >> 0x20)) * _UNK_142aba9c4;
      auVar22._8_4_ = _UNK_142aba9c8 * 0.0;
      auVar22._12_4_ = ((float)((ulonglong)extraout_XMM0_Qa >> 0x20) - 0.0) * _UNK_142aba9cc;
      auVar22 = divps(auVar22,___xmm_00000000000000004270000042700000);
      *puVar9 = CONCAT44((float)((ulonglong)*puVar9 >> 0x20) + auVar22._4_4_,
                         (float)*puVar9 + auVar22._0_4_);
      *(float *)(puVar9 + 1) = fVar24 + *(float *)(puVar9 + 1);
      return;
    }
  }
LAB_140993c4f:
  local_118 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
  local_110 = 0x22;
  local_128 = &local_118;
  local_120 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_108 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
  local_100 = 2;
  local_e8 = 0;
  local_f8 = &local_128;
  local_f0 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

