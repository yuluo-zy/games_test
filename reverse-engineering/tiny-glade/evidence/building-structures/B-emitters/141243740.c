
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_wall_constructor::construct_lintels::place_support_beams::
add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
          (void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char extraout_AL;
  ulonglong uVar4;
  uint in_EDX;
  longlong in_R9;
  uint *puVar5;
  float *pfVar6;
  float fVar7;
  float fVar9;
  float fVar10;
  undefined1 auVar8 [16];
  float in_XMM2_Da;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar16;
  float fVar17;
  undefined1 auVar15 [16];
  float fVar18;
  float fVar19;
  ulonglong in_stack_00000028;
  float in_stack_00000030;
  char in_stack_00000038;
  longlong in_stack_00000040;
  ulonglong in_stack_00000048;
  undefined1 *in_stack_00000058;
  uint local_118;
  float local_114;
  uint local_110;
  float local_10c [6];
  undefined4 local_f4;
  float local_e8;
  float fStack_e4;
  float fStack_e0;
  float fStack_dc;
  float local_d8;
  float fStack_d4;
  float fStack_d0;
  float fStack_cc;
  float local_c8;
  float fStack_c4;
  float fStack_c0;
  float fStack_bc;
  float local_b8;
  float fStack_b4;
  float fStack_b0;
  float fStack_ac;
  
  uVar4 = (ulonglong)in_EDX;
  if (in_stack_00000028 <= uVar4) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  uVar1 = uVar4 + 1;
  if (in_stack_00000028 <= uVar1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (in_stack_00000048 <= uVar4) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (uVar1 < in_stack_00000048) {
    fVar9 = *(float *)(in_R9 + uVar4 * 4);
    fVar18 = (*(float *)(in_R9 + 4 + uVar4 * 4) - fVar9) * in_XMM2_Da + fVar9 + __real_be666667;
    fVar7 = in_stack_00000030 + ___real_be866666;
    fVar19 = __real_3ecccccd * fVar18;
    fVar9 = fVar7;
    if (fVar19 <= fVar7) {
      fVar9 = fVar19;
    }
    fVar12 = *(float *)(in_stack_00000040 + uVar4 * 0xc);
    fVar10 = *(float *)(in_stack_00000040 + uVar1 * 0xc);
    uVar2 = *(undefined8 *)(in_stack_00000040 + 4 + uVar4 * 0xc);
    fVar11 = (float)uVar2;
    fVar13 = (float)((ulonglong)uVar2 >> 0x20);
    uVar2 = *(undefined8 *)(in_stack_00000040 + 4 + uVar1 * 0xc);
    if (in_stack_00000038 == '\0') {
      puVar5 = &local_118;
      pfVar6 = &local_114;
      ::utils::curve::curve_slice::CurveSlice::
      extend_left<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>
                ();
    }
    else {
      puVar5 = &local_110;
      pfVar6 = local_10c;
      ::utils::curve::curve_slice::CurveSlice::
      extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                ();
    }
    uVar4 = (ulonglong)*puVar5;
    if (in_stack_00000028 <= uVar4) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    uVar1 = uVar4 + 1;
    if (in_stack_00000028 <= uVar1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (uVar4 < in_stack_00000048) {
      if (uVar1 < in_stack_00000048) {
        fStack_b4 = (fVar18 - (float)(-(uint)NAN(fVar19) & (uint)fVar7 |
                                     ~-(uint)NAN(fVar19) & (uint)fVar9)) + ___real_be400000;
        local_b8 = fStack_b4 * 0.0;
        fStack_b4 = fStack_b4 + in_XMM2_Da * ((float)uVar2 - fVar11) + fVar11;
        fStack_ac = local_b8 + in_XMM2_Da * ((float)((ulonglong)uVar2 >> 0x20) - fVar13) + fVar13;
        local_b8 = (fVar10 - fVar12) * in_XMM2_Da + fVar12 + local_b8;
        fVar9 = *pfVar6;
        fVar7 = *(float *)(in_R9 + uVar4 * 4);
        fVar18 = *(float *)(in_stack_00000040 + uVar4 * 0xc);
        fVar19 = (*(float *)(in_R9 + 4 + uVar4 * 4) - fVar7) * fVar9 + fVar7 + __real_bde66667;
        fVar12 = fVar19 * 0.0;
        fVar7 = ((*(float *)(in_stack_00000040 + uVar1 * 0xc) - fVar18) * fVar9 + fVar18 + fVar12) -
                local_b8;
        uVar2 = *(undefined8 *)(in_stack_00000040 + 4 + uVar4 * 0xc);
        uVar3 = *(undefined8 *)(in_stack_00000040 + 4 + uVar1 * 0xc);
        fVar18 = (float)uVar2;
        fVar10 = (float)((ulonglong)uVar2 >> 0x20);
        fVar18 = (fVar19 + fVar9 * ((float)uVar3 - fVar18) + fVar18) - fStack_b4;
        fVar9 = (fVar12 + fVar9 * ((float)((ulonglong)uVar3 >> 0x20) - fVar10) + fVar10) - fStack_ac
        ;
        fVar13 = SQRT(fVar9 * fVar9 + fVar7 * fVar7 + fVar18 * fVar18);
        fVar11 = __real_be4ccccd + fVar13;
        fVar13 = __real_3f800000 / fVar13;
        fVar7 = fVar7 * fVar13;
        fVar18 = fVar13 * fVar18;
        fVar13 = fVar13 * fVar9;
        fVar9 = ___xmm_00000000bf8000008000000080000000 * fVar7 +
                (float)__xmm_00000000000000003f80000000000000 * fVar13;
        fVar19 = _UNK_1429a9084 * fVar18 +
                 (float)((ulonglong)__xmm_00000000000000003f80000000000000 >> 0x20) * fVar7;
        fVar12 = _UNK_1429a9088 * fVar13 + fVar18 * 0.0;
        auVar8._4_4_ = fVar9;
        auVar8._0_4_ = fVar12;
        auVar8._8_4_ = fVar19;
        auVar8._12_4_ = fVar19;
        auVar15._0_4_ = fVar19 * fVar19 + fVar12 * fVar12 + fVar9 * fVar9;
        auVar15._4_4_ = auVar15._0_4_;
        auVar15._8_4_ = auVar15._0_4_;
        auVar15._12_4_ = auVar15._0_4_;
        auVar15 = sqrtps(auVar15,auVar15);
        auVar8 = divps(auVar8,auVar15);
        fVar9 = auVar8._0_4_;
        fVar12 = auVar8._4_4_;
        fVar10 = auVar8._8_4_;
        fVar14 = fVar10 * fVar7 - fVar13 * fVar9;
        fVar16 = fVar9 * fVar18 - fVar7 * fVar12;
        fVar17 = fVar12 * fVar13 - fVar18 * fVar10;
        local_e8 = ___xmm_3e866666000000000000000000000000 * fVar17 +
                   fVar7 * ___xmm_3e866666000000000000000000000000 +
                   ___xmm_3e8666663e8666663e8666663e866666 * fVar9;
        fStack_e4 = _UNK_142c1f804 * fVar14 + fVar18 * _UNK_142c1f804 + _UNK_142c1f7f4 * fVar12;
        fStack_e0 = _UNK_142c1f808 * fVar16 + fVar13 * _UNK_142c1f808 + _UNK_142c1f7f8 * fVar10;
        fStack_dc = _UNK_142c1f80c * fVar16 +
                    fVar13 * _UNK_142c1f80c + _UNK_142c1f7fc * auVar8._12_4_;
        fVar19 = auVar8._12_4_ * 0.0;
        local_d8 = fVar11 * fVar7 + fVar9 * 0.0 + fVar17 * 0.0;
        fStack_d4 = fVar11 * fVar18 + fVar12 * 0.0 + fVar14 * 0.0;
        fStack_d0 = fVar11 * fVar13 + fVar10 * 0.0 + fVar16 * 0.0;
        fStack_cc = fVar13 * 0.0 + fVar19 + fVar16 * 0.0;
        fVar9 = fVar7 * 0.0 + fVar9 * 0.0;
        fVar7 = fVar18 * 0.0 + fVar12 * 0.0;
        fVar18 = fVar13 * 0.0 + fVar10 * 0.0;
        fVar19 = fVar13 * 0.0 + fVar19;
        local_c8 = fVar17 * ___xmm_000000003e8666663e8666663e866666 + fVar9;
        fStack_c4 = fVar14 * _UNK_142c1f814 + fVar7;
        fStack_c0 = fVar16 * _UNK_142c1f818 + fVar18;
        fStack_bc = fVar16 * _UNK_142c1f81c + fVar19;
        local_b8 = fVar9 + fVar17 * 0.0 + local_b8;
        fStack_b4 = fVar7 + fVar14 * 0.0 + fStack_b4;
        fStack_b0 = fVar18 + fVar16 * 0.0 + fStack_ac;
        fStack_ac = fVar19 + fVar16 * 0.0 + fStack_ac;
        country_core::resources::render::impl$1::from();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        local_f4 = 0;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (extraout_AL != '\0') {
          *in_stack_00000058 = 1;
        }
        return;
      }
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

