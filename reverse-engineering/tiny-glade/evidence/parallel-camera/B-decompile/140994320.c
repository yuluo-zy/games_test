
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain(void)

{
  float fVar1;
  float fVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  float *pfVar6;
  longlong lVar7;
  longlong in_RCX;
  undefined8 *puVar8;
  longlong in_RDX;
  longlong in_R8;
  undefined8 *puVar9;
  char in_R9B;
  longlong lVar10;
  float fVar11;
  float fVar12;
  float extraout_XMM0_Da;
  float fVar13;
  float fVar14;
  undefined8 local_110;
  float local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 local_fc;
  undefined8 *local_f8;
  code *local_f0;
  undefined *local_e8;
  undefined8 uStack_e0;
  undefined8 **local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  float local_b8;
  int local_b4;
  
  lVar10 = *(longlong *)(in_RCX + 0x10) << 4;
  puVar9 = *(undefined8 **)(in_RCX + 8);
  do {
    if (lVar10 == 0) {
      local_110 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
      local_108 = 4.76441e-44;
      uStack_104 = 0;
      local_f8 = &local_110;
      local_f0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_e8 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
      uStack_e0 = 2;
      local_c8 = 0;
      local_d8 = &local_f8;
      local_d0 = 1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    puVar8 = puVar9 + 2;
    pfVar6 = (float *)(**(code **)(puVar9[1] + 0x40))(*puVar9);
    lVar7 = (**(code **)(in_RDX + 0x18))(pfVar6);
    lVar10 = lVar10 + -0x10;
    puVar9 = puVar8;
  } while (((pfVar6 == (float *)0x0) || (lVar7 != 0x6b8463577afc1a66)) ||
          (in_RDX != 0xe4b9f294168d7e3));
  fVar1 = *pfVar6;
  fVar14 = __real_bf000000 + pfVar6[1];
  fVar13 = pfVar6[1] + ___real_bff33333 + ___real_bfb33333;
  fVar2 = pfVar6[2];
  *(undefined1 *)(in_RCX + 0x38) = 0x11;
  uVar3 = __real_7f7fffff;
  fVar12 = __real_c281cccd;
  if (__real_c281cccd <= fVar1) {
    fVar12 = fVar1;
  }
  fVar11 = __real_4281cccd;
  if (fVar12 <= __real_4281cccd) {
    fVar11 = fVar12;
  }
  fVar12 = __real_c281cccd;
  if (__real_c281cccd <= fVar2) {
    fVar12 = fVar2;
  }
  local_108 = __real_4281cccd;
  if (fVar12 <= __real_4281cccd) {
    local_108 = fVar12;
  }
  local_110 = (char *)CONCAT44(fVar14,fVar11);
  uStack_104 = (undefined4)__xmm_0000000000000000bf80000080000000;
  uVar4 = uStack_104;
  uStack_100 = (undefined4)((ulonglong)__xmm_0000000000000000bf80000080000000 >> 0x20);
  uVar5 = uStack_100;
  local_fc = 0x80000000;
  local_f8 = (undefined8 *)(ulonglong)__real_7f7fffff;
  local_f0 = (code *)((ulonglong)local_f0 & 0xffffffffffff0000);
  collision::world::Raycasting::spherecast();
  if (local_b4 != 2) {
    fVar11 = (fVar14 - local_b8) + __real_bee66666;
    fVar12 = fVar11;
    if (fVar11 <= fVar13) {
      fVar12 = fVar13;
    }
    fVar13 = (float)(~-(uint)NAN(fVar13) & (uint)fVar12 | -(uint)NAN(fVar13) & (uint)fVar11);
    *(undefined1 *)(in_RCX + 0x38) = 0xb;
  }
  local_110 = (char *)CONCAT44(fVar14,fVar1);
  local_fc = 0x80000000;
  local_f8 = (undefined8 *)(ulonglong)uVar3;
  local_f0 = (code *)((ulonglong)local_f0 & 0xffffffffffff0000);
  local_108 = fVar2;
  uStack_104 = uVar4;
  uStack_100 = uVar5;
  collision::world::Raycasting::spherecast();
  if ((local_b4 != 2) && (fVar13 < (fVar14 - local_b8) + __real_bee66666)) {
    *(undefined8 ***)(in_RCX + 0x48) = local_d8;
    *(undefined **)(in_RCX + 0x38) = local_e8;
    *(undefined8 *)(in_RCX + 0x40) = uStack_e0;
  }
  puVar9 = (undefined8 *)(in_R8 + 0x20);
  puVar8 = (undefined8 *)(in_R8 + 0x28);
  if (*(char *)(in_R8 + 0x298) == '\0') {
    puVar8 = (undefined8 *)(in_R8 + 0x10);
    puVar9 = (undefined8 *)(in_R8 + 8);
  }
  uStack_e0 = *puVar8;
  local_e8 = (undefined *)*puVar9;
  if (in_R9B == '\0') {
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  }
  ::utils::time_continuous_lerp();
  pfVar6[1] = extraout_XMM0_Da;
  return;
}

