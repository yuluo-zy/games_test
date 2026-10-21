
void __cdecl system_decorator::decorator_visual::stairs_assemble::construct_stair_steps(void)

{
  ulonglong uVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  float *pfVar5;
  float fVar6;
  undefined1 auVar7 [16];
  float *pfVar8;
  bool bVar9;
  uint uVar10;
  bool bVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  float *pfVar14;
  float *pfVar15;
  float *pfVar16;
  float fVar17;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  undefined1 auVar18 [12];
  undefined1 auVar19 [16];
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar25;
  float fVar26;
  float fVar27;
  undefined1 auVar24 [16];
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar32;
  float fVar33;
  float fVar34;
  undefined1 auVar31 [16];
  float fVar35;
  float fVar36;
  float *in_stack_00000028;
  ulonglong in_stack_00000030;
  longlong *in_stack_00000038;
  ushort in_stack_00000058;
  char in_stack_00000060;
  uint in_stack_00000068;
  longlong in_stack_00000070;
  char in_stack_00000088;
  longlong local_498;
  float *pfStack_490;
  float *pfStack_480;
  float *local_478;
  float *pfStack_470;
  ulonglong local_3a0;
  float local_258;
  float local_254;
  float local_250;
  float local_24c;
  float local_248;
  float local_244;
  longlong local_218;
  float *local_210;
  longlong local_208;
  undefined8 local_178;
  float *pfStack_170;
  float *pfStack_160;
  float *local_158;
  float *pfStack_150;
  uint uStack_140;
  uint uStack_13c;
  
  if (in_stack_00000030 == 1) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (in_stack_00000030 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  fVar3 = in_stack_00000028[1];
  fVar4 = in_stack_00000028[4];
  fVar17 = floorf(0.56 / SQRT((in_stack_00000028[2] - in_stack_00000028[5]) *
                              (in_stack_00000028[2] - in_stack_00000028[5]) +
                              (*in_stack_00000028 - in_stack_00000028[3]) *
                              (*in_stack_00000028 - in_stack_00000028[3])));
  uVar12 = (longlong)(fVar17 - 9.223372e+18) & (longlong)fVar17 >> 0x3f | (longlong)fVar17;
  if (fVar17 < 0.0) {
    uVar12 = 0;
  }
  if (1.8446743e+19 < fVar17) {
    uVar12 = 0xffffffffffffffff;
  }
  auVar18 = ZEXT812(0);
  if (in_stack_00000060 != '\0') {
    auVar18 = ZEXT812(0x3e800000);
  }
  local_3a0 = 0;
  uVar13 = in_stack_00000030 - (uVar12 + 2);
  if (in_stack_00000030 < uVar12 + 2) {
    uVar13 = 0;
  }
  pfVar5 = (float *)(*(longlong *)(in_stack_00000070 + 0x10) * 2);
  auVar7._4_4_ = in_stack_00000068;
  auVar7._0_4_ = in_stack_00000068;
  pfVar16 = in_stack_00000028;
  do {
    itertools::tuple_impl::impl$7::
    next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32>_>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::construct_stair_steps::closure_env_0>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<f32>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<f32>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>
              ();
    if (local_178 == 0) {
      return;
    }
    uVar1 = local_3a0 + 1;
    fVar17 = local_158[1];
    fVar21 = *(float *)(local_178 + 4);
    fVar35 = ABS(fVar17 - fVar21);
    if (fVar35 <= 0.6) {
      fVar35 = 0.6;
    }
    fVar22 = ABS(fVar3 - fVar4);
    if ((in_stack_00000058 & 0x100) == 0) {
      if (in_stack_00000028 + in_stack_00000030 * 3 + -3 == (float *)0x0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      fVar23 = in_stack_00000028[in_stack_00000030 * 3 + -2];
      if (fVar3 < fVar23) {
        bVar11 = uVar13 < local_3a0;
        if ((in_stack_00000058 & 1) == 0) goto LAB_1407da461;
LAB_1407da4b0:
        if (!bVar11) goto LAB_1407da4bb;
      }
      else {
        if ((in_stack_00000058 & 1) != 0) goto LAB_1407da4bb;
        bVar11 = false;
        bVar9 = false;
        if (fVar3 <= fVar23) goto LAB_1407da4b0;
LAB_1407da46e:
        bVar11 = bVar9;
        if (uVar12 <= local_3a0) goto LAB_1407da4b0;
      }
    }
    else {
      if ((in_stack_00000058 & 1) == 0) {
        if (in_stack_00000028 + in_stack_00000030 * 3 + -3 == (float *)0x0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        fVar23 = in_stack_00000028[in_stack_00000030 * 3 + -2];
        bVar11 = false;
LAB_1407da461:
        bVar9 = bVar11;
        if (fVar23 < fVar3) goto LAB_1407da46e;
        goto LAB_1407da4b0;
      }
LAB_1407da4bb:
      fVar22 = 0.0;
    }
    fVar35 = fVar35 + fVar22;
    if (in_stack_00000088 != '\0') {
      fVar35 = fVar35 * 0.5;
    }
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>();
    ::utils::swizzlers::impl$1::xvy();
    fVar22 = *local_158;
    fVar23 = local_158[2];
    local_178 = CONCAT44(fVar23,fVar22);
    ::utils::swizzlers::impl$1::xvy();
    fVar35 = fVar35 * -0.5;
    fVar36 = fVar35 * 0.0;
    fVar20 = *pfStack_170;
    ::utils::swizzlers::impl$0::x0y();
    pfStack_170._0_4_ = SUB84(pfVar5,0);
    ::utils::swizzlers::impl$0::x0y();
    auVar24._0_4_ = fVar20 - auVar18._0_4_;
    auVar24._4_4_ = *pfStack_150 - auVar18._0_4_;
    auVar24._8_4_ = 0.0 - auVar18._8_4_;
    auVar24._12_4_ = 0.0 - auVar18._8_4_;
    ::utils::swizzlers::impl$0::x0y();
    ::utils::swizzlers::impl$0::x0y();
    auVar7._8_8_ = 0;
    auVar19 = maxps(auVar7,auVar24);
    fVar26 = (float)(~-(uint)NAN(auVar24._8_4_) & auVar19._8_4_);
    fVar28 = (float)(~-(uint)NAN(auVar24._12_4_) & auVar19._12_4_);
    fVar25 = (float)(~-(uint)NAN(auVar24._0_4_) & auVar19._0_4_ |
                    in_stack_00000068 & -(uint)NAN(auVar24._0_4_));
    fVar27 = (float)(~-(uint)NAN(auVar24._4_4_) & auVar19._4_4_ |
                    in_stack_00000068 & -(uint)NAN(auVar24._4_4_));
    fVar29 = fVar27 * 0.5 + fVar25 * 0.5;
    ::utils::inverse_lerp();
    fVar20 = 0.0;
    if (0.0 <= extraout_XMM0_Da) {
      fVar20 = extraout_XMM0_Da;
    }
    fVar6 = 1.0;
    if (fVar20 <= 1.0) {
      fVar6 = fVar20;
    }
    if ((fVar29 < 0.0) || (0.66 <= fVar29)) {
      if ((fVar29 < 0.66) || (1.2 <= fVar29)) {
        fastrand::Rng::f32();
        fVar20 = 0.0;
        if (0.0 <= extraout_XMM0_Da_00) {
          fVar20 = extraout_XMM0_Da_00;
        }
        fVar30 = 1.0;
        if (fVar20 <= 1.0) {
          fVar30 = fVar20;
        }
        ceilf(fVar29 / ((3.0 - (fVar30 + fVar30)) * fVar30 * fVar30 *
                        (fVar6 * 1.3 + (1.0 - fVar6) * 0.7) + 0.46));
      }
      else {
        *in_stack_00000038 = *in_stack_00000038 + -0x5f89e29b87429bd1;
      }
    }
    ::utils::random_splits();
    fastrand::Rng::f32();
    pfStack_170 = pfVar5;
    if ((0.1 <= *(float *)CONCAT44(uStack_13c,uStack_140) * 0.5 + *pfStack_160 * 0.5) ||
       (in_stack_00000088 == '\0')) {
      ::utils::xz_rotation_mat3_bad();
      fVar30 = ((local_24c + fVar36) - fVar22 * fVar25 * 0.5) -
               ((local_258 + fVar36) - fVar22 * fVar27 * 0.5);
      fVar32 = (fVar22 * fVar25 * 0.5 + local_24c + fVar36) -
               (fVar22 * fVar27 * 0.5 + local_258 + fVar36);
      fVar33 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      fVar34 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      fVar22 = ((local_248 + fVar35) - fVar23 * fVar25 * 0.5) -
               ((fVar35 + local_254) - fVar23 * fVar27 * 0.5);
      fVar35 = (fVar23 * fVar25 * 0.5 + local_248 + fVar35) -
               (fVar23 * fVar27 * 0.5 + fVar35 + local_254);
      fVar23 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      fVar20 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      fVar29 = ((local_244 + fVar36) - pfStack_170._0_4_ * fVar25 * 0.5) -
               ((fVar36 + local_250) - pfStack_170._0_4_ * fVar27 * 0.5);
      fVar25 = (pfStack_170._0_4_ * fVar25 * 0.5 + local_244 + fVar36) -
               (pfStack_170._0_4_ * fVar27 * 0.5 + fVar36 + local_250);
      fVar27 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      fVar26 = (0.0 - fVar26 * 0.0 * 0.0) - (0.0 - fVar28 * 0.0 * 0.0);
      auVar31._0_4_ = fVar30 * fVar30;
      auVar31._4_4_ = fVar32 * fVar32;
      auVar31._8_4_ = fVar33 * fVar33;
      auVar31._12_4_ = fVar34 * fVar34;
      auVar19._0_4_ = fVar29 * fVar29 + fVar22 * fVar22 + auVar31._0_4_;
      auVar19._4_4_ = fVar25 * fVar25 + fVar35 * fVar35 + auVar31._4_4_;
      auVar19._8_4_ = fVar27 * fVar27 + fVar23 * fVar23 + auVar31._8_4_;
      auVar19._12_4_ = fVar26 * fVar26 + fVar20 * fVar20 + auVar31._12_4_;
      sqrtps(auVar31,auVar19);
      fVar21 = ABS(fVar21 - fVar17) * 0.8;
      fVar35 = fVar6 * 0.3 + (1.0 - fVar6) * 0.15;
      fVar17 = fVar21;
      if (fVar35 <= fVar21) {
        fVar17 = fVar35;
      }
      fVar23 = fVar6 * 0.15 + (1.0 - fVar6) * 0.05;
      fVar22 = fVar21;
      if (fVar23 <= fVar21) {
        fVar22 = fVar23;
      }
      fVar17 = (float)(-(uint)NAN(fVar35) & (uint)fVar21 | ~-(uint)NAN(fVar35) & (uint)fVar17) *
               extraout_XMM0_Da_01 +
               (1.0 - extraout_XMM0_Da_01) *
               (float)(-(uint)NAN(fVar23) & (uint)fVar21 | ~-(uint)NAN(fVar23) & (uint)fVar22);
      uVar10 = (uint)(local_3a0 == in_stack_00000030 - 2 || local_3a0 == 0);
      bVar11 = true;
      pfVar8 = local_210;
      while (pfVar8 != local_210 + local_208) {
        pfVar2 = pfVar8 + 1;
        pfVar14 = pfVar2;
        pfVar15 = pfVar8;
        if (bVar11) {
          if (pfVar2 == local_210 + local_208) break;
          pfVar14 = pfVar8 + 2;
          pfVar15 = pfVar2;
          pfVar16 = pfVar8;
        }
        fastrand::Rng::f32();
        country_core::resources::render::Affine3Packed::from_cols();
        fVar21 = *pfVar16;
        fastrand::Rng::f32();
        fVar35 = *pfVar15;
        fastrand::Rng::f32();
        country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
        uStack_140 = ~((int)(uVar10 << 0x1f) >> 0x1f) &
                     (uint)(extraout_XMM0_Da_03 * 0.05 -
                           (1.0 - (ABS(fVar35 + -0.5) + ABS(fVar35 + -0.5))) * fVar17);
        uStack_13c = ~((int)(uVar10 << 0x1f) >> 0x1f) &
                     (uint)(extraout_XMM0_Da_02 * 0.05 -
                           (1.0 - (ABS(fVar21 + -0.5) + ABS(fVar21 + -0.5))) * fVar17);
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
        bVar11 = false;
        pfVar8 = pfVar14;
        local_178 = local_498;
        pfStack_170 = pfStack_490;
        pfStack_160 = pfStack_480;
        local_158 = local_478;
        pfStack_150 = pfStack_470;
        pfVar16 = pfVar15;
      }
      local_3a0 = uVar1;
      if (local_218 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      local_3a0 = uVar1;
      if (local_218 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  } while( true );
}

