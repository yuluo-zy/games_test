
void __cdecl
utils::curve::Curve::
crashy_resample_remap_per_segment<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::shapes::calc_shape_height::calc_shape_height::closure_env_0>
          (void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  undefined1 *puVar5;
  longlong lVar6;
  float *pfVar7;
  bool bVar8;
  float *extraout_RAX;
  longlong *in_RCX;
  float *pfVar9;
  longlong in_RDX;
  float *pfVar10;
  float *pfVar11;
  ulonglong uVar12;
  undefined8 *in_R9;
  ulonglong uVar13;
  float extraout_XMM0_Da;
  float fVar14;
  float extraout_XMM0_Da_00;
  float fVar15;
  float fVar16;
  float in_XMM2_Da;
  uint uVar17;
  float fVar18;
  float fVar19;
  longlong *in_stack_00000028;
  longlong local_1b0;
  longlong lStack_1a8;
  longlong local_1a0;
  longlong lStack_198;
  longlong local_190;
  longlong lStack_188;
  longlong local_180;
  undefined1 local_178 [16];
  undefined **local_168;
  float *pfStack_160;
  undefined1 **local_158;
  undefined8 local_150;
  undefined8 local_148;
  longlong *local_138;
  longlong local_130;
  float *local_128;
  float *local_120;
  undefined1 *local_118;
  float *pfStack_110;
  undefined1 *local_108;
  code *local_100;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar5 = *(undefined1 **)(in_RDX + 0x10);
  if (puVar5 < (undefined1 *)0x2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((in_XMM2_Da == 0.0) && (!NAN(in_XMM2_Da))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((*(float *)(in_RDX + 0x30) == 0.0) && (!NAN(*(float *)(in_RDX + 0x30)))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (puVar5 < (undefined1 *)0xaaaaaaaaaaaaaab) {
    pfVar9 = (float *)*in_R9;
    lVar6 = *in_stack_00000028;
    local_138 = in_RCX;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX != (float *)0x0) {
      local_108 = (undefined1 *)0x0;
      pfVar10 = *(float **)(in_RDX + 8);
      fVar1 = *pfVar10;
      fVar2 = pfVar10[1];
      local_f1 = 1;
      local_130 = in_RDX;
      local_118 = puVar5;
      pfStack_110 = extraout_RAX;
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      fVar15 = *(float *)(lVar6 + 0x3c);
      uVar17 = -(uint)NAN(extraout_XMM0_Da) & (uint)fVar15;
      if (fVar15 <= extraout_XMM0_Da) {
        fVar15 = extraout_XMM0_Da;
      }
      *extraout_RAX = fVar1;
      extraout_RAX[1] = (float)(~-(uint)NAN(extraout_XMM0_Da) & (uint)fVar15 | uVar17);
      extraout_RAX[2] = fVar2;
      fVar15 = __real_bf800000;
      fVar2 = __real_5f7fffff;
      fVar1 = __real_3f800000;
      local_108 = (undefined1 *)0x1;
      local_120 = pfVar10 + (longlong)puVar5 * 2;
      bVar8 = true;
      while (pfVar10 != local_120) {
        pfVar7 = pfVar10 + 2;
        pfVar11 = pfVar9;
        pfVar9 = pfVar10;
        if (bVar8) {
          if (pfVar7 == local_120) break;
          pfVar11 = pfVar10;
          pfVar9 = pfVar7;
          pfVar7 = pfVar10 + 4;
        }
        local_128 = pfVar7;
        fVar14 = SQRT((pfVar9[1] - pfVar11[1]) * (pfVar9[1] - pfVar11[1]) +
                      (*pfVar9 - *pfVar11) * (*pfVar9 - *pfVar11));
        if (fVar14 <= __real_322bcc77) {
          fVar14 = __real_322bcc77;
        }
        fVar16 = in_XMM2_Da / fVar14;
        if (fVar1 <= in_XMM2_Da / fVar14) {
          fVar16 = fVar1;
        }
        fVar14 = (float)roundf(fVar1 / fVar16);
        uVar12 = (longlong)(fVar14 - __real_5f000000) & (longlong)fVar14 >> 0x3f | (longlong)fVar14;
        if (fVar14 < 0.0) {
          uVar12 = 0;
        }
        if (fVar2 < fVar14) {
          uVar12 = 0xffffffffffffffff;
        }
        uVar12 = uVar12 + 1;
        if (uVar12 < 3) {
          uVar12 = 2;
        }
        uVar13 = 1;
        while (uVar13 < uVar12) {
          fVar18 = (float)uVar13 / ((float)uVar12 + fVar15);
          fVar14 = *pfVar11;
          fVar16 = pfVar11[1];
          fVar3 = *pfVar9;
          fVar4 = pfVar9[1];
          local_f1 = 1;
          country_core::resources::terrain_height::TerrainHeightsData::
          sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
          puVar5 = local_108;
          fVar19 = *(float *)(lVar6 + 0x3c);
          if (local_108 == local_118) {
            local_f1 = 1;
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          uVar13 = uVar13 + 1;
          uVar17 = -(uint)NAN(extraout_XMM0_Da_00) & (uint)fVar19;
          if (fVar19 <= extraout_XMM0_Da_00) {
            fVar19 = extraout_XMM0_Da_00;
          }
          pfStack_110[(longlong)puVar5 * 3] = fVar3 * fVar18 + fVar14 * (fVar1 - fVar18);
          pfStack_110[(longlong)puVar5 * 3 + 1] =
               (float)(~-(uint)NAN(extraout_XMM0_Da_00) & (uint)fVar19 | uVar17);
          pfStack_110[(longlong)puVar5 * 3 + 2] = fVar18 * fVar4 + (fVar1 - fVar18) * fVar16;
          local_108 = puVar5 + 1;
        }
        bVar8 = false;
        pfVar10 = local_128;
      }
      local_158 = (undefined1 **)local_108;
      local_168 = (undefined **)local_118;
      pfStack_160 = pfStack_110;
      local_f1 = 0;
      try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (!SBORROW8(0,local_1b0)) {
        local_138[6] = local_180;
        local_138[4] = local_190;
        local_138[5] = lStack_188;
        local_138[2] = local_1a0;
        local_138[3] = lStack_198;
        *local_138 = local_1b0;
        local_138[1] = lStack_1a8;
        return;
      }
      local_118 = local_178;
      pfStack_110 = (float *)impl$46::fmt;
      local_108 = (undefined1 *)local_130;
      local_100 = alloc::vec::impl$27::fmt<glam::f32::vec2::Vec2,alloc::alloc::Global>;
      local_168 = &PTR_s_Curve_per_segment_resampling_fai_142c63ab8;
      pfStack_160 = (float *)0x2;
      local_148 = 0;
      local_158 = &local_118;
      local_150 = 2;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

