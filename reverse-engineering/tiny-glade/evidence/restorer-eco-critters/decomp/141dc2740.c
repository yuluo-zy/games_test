// 0x141dc2740 frame_update<system_critters::fish::fish::closure_env$0>
// SIG: void __cdecl frame_update<system_critters::fish::fish::closure_env$0>(void)

void __cdecl
system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env_0>(void)

{
  float *pfVar1;
  float *pfVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  bool bVar6;
  undefined1 auVar7 [16];
  code *pcVar8;
  int iVar9;
  uint uVar10;
  float *in_RCX;
  int *in_RDX;
  longlong lVar11;
  longlong in_R8;
  float *in_R9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  ulonglong extraout_XMM0_Qa;
  float fVar13;
  undefined1 auVar12 [16];
  float fVar14;
  float fVar15;
  float fVar17;
  undefined1 auVar16 [16];
  float fVar18;
  float fVar19;
  float fVar21;
  undefined1 auVar20 [16];
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  ulonglong uVar27;
  float fVar30;
  undefined1 auVar29 [16];
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  undefined1 auVar35 [16];
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar40;
  float fVar41;
  float fVar42;
  undefined1 auVar39 [16];
  longlong in_stack_00000028;
  float in_stack_00000038;
  float in_stack_00000040;
  float in_stack_00000048;
  float in_stack_00000050;
  float in_stack_00000060;
  undefined4 local_164;
  undefined4 uStack_160;
  float local_15c;
  code *local_150;
  undefined8 local_148;
  float fStack_140;
  float fStack_13c;
  float local_138;
  float fStack_134;
  float fStack_130;
  float fStack_12c;
  undefined1 local_128 [16];
  undefined1 local_118 [8];
  float fStack_110;
  float fStack_10c;
  undefined1 local_108 [16];
  undefined8 local_f8;
  undefined8 uStack_f0;
  code **local_e0;
  undefined1 auVar28 [16];
  
  local_f8 = *(undefined8 *)(in_RCX + 3);
  uStack_f0 = 0;
  fVar32 = in_RCX[5];
  random_vel_change();
  uVar27 = (ulonglong)(uint)in_RCX[4];
  fVar37 = 0.0;
  if (0.0 < in_RCX[4]) {
    ::utils::time_continuous_lerp();
    uVar27 = extraout_XMM0_Qa;
  }
  fVar36 = *in_RCX;
  fVar24 = in_RCX[3];
  fVar17 = in_RCX[5];
  fVar13 = (float)(uVar27 >> 0x20);
  fVar15 = (float)uVar27;
  fVar14 = 0.0;
  if (in_stack_00000028 != 0) {
    pfVar1 = in_R9 + in_stack_00000028 * 6;
    fVar26 = (float)*(undefined8 *)(in_RCX + 1);
    fVar34 = (float)((ulonglong)*(undefined8 *)(in_RCX + 1) >> 0x20);
    fVar38 = 0.0;
    fVar40 = 0.0;
    fVar41 = 0.0;
    fVar42 = 0.0;
    iVar9 = 0;
    fVar22 = 0.0;
    fVar37 = 0.0;
    lVar11 = 0;
    local_138 = 0.0;
    fStack_134 = 0.0;
    fStack_130 = 0.0;
    fStack_12c = 0.0;
    local_128 = ZEXT416((uint)fVar32);
    auVar29 = ZEXT816(0);
    auVar35 = ZEXT816(0);
LAB_141dc288a:
    _local_118 = auVar35;
    local_108 = auVar29;
    lVar11 = lVar11 + 1;
    auVar29 = local_108;
    auVar35 = _local_118;
    if (0 < iVar9) {
      do {
        if (-in_R8 + lVar11 != 1) {
          fVar18 = *in_R9;
          fVar23 = fVar18 - fVar36;
          auVar39._0_8_ = *(undefined8 *)(in_R9 + 1);
          fVar25 = (float)auVar39._0_8_ - fVar26;
          fVar30 = (float)((ulonglong)auVar39._0_8_ >> 0x20) - fVar34;
          uVar27 = CONCAT44(fVar30,fVar25);
          fVar33 = fVar30 * fVar30 + fVar23 * fVar23 + fVar25 * fVar25;
          if (fVar33 < 16.0) {
            fVar31 = in_R9[3];
            fVar19 = in_R9[4];
            fVar21 = in_R9[5];
            if ((0.0 <= fVar17 * fVar21 + fVar24 * fVar31 + fVar15 * fVar19) &&
               (-0.5 <= (fVar30 * fVar21 + fVar23 * fVar31 + fVar25 * fVar19) /
                        SQRT((fVar21 * fVar21 + fVar31 * fVar31 + fVar19 * fVar19) * fVar33)))
            goto LAB_141dc2a66;
          }
        }
        in_R9 = in_R9 + 6;
        lVar11 = lVar11 + 1;
        if (in_R9 == pfVar1) goto LAB_141dc2b5e;
      } while( true );
    }
    do {
      if (-in_R8 + lVar11 != 1) {
        fVar18 = *in_R9;
        fVar23 = fVar18 - fVar36;
        auVar39._0_8_ = *(undefined8 *)(in_R9 + 1);
        fVar25 = (float)auVar39._0_8_ - fVar26;
        fVar30 = (float)((ulonglong)auVar39._0_8_ >> 0x20) - fVar34;
        uVar27 = CONCAT44(fVar30,fVar25);
        fVar33 = fVar30 * fVar30 + fVar23 * fVar23 + fVar25 * fVar25;
        if (fVar33 < 16.0) {
          fVar31 = in_R9[3];
          fVar19 = in_R9[4];
          fVar21 = in_R9[5];
          if (-0.5 <= (fVar30 * fVar21 + fVar23 * fVar31 + fVar25 * fVar19) /
                      SQRT((fVar21 * fVar21 + fVar31 * fVar31 + fVar19 * fVar19) * fVar33))
          goto LAB_141dc2a66;
        }
      }
      in_R9 = in_R9 + 6;
      lVar11 = lVar11 + 1;
      if (in_R9 == pfVar1) goto LAB_141dc2b5e;
    } while( true );
  }
  fVar38 = 0.0;
  fVar40 = 0.0;
  fVar41 = 0.0;
  fVar42 = 0.0;
LAB_141dc2cb9:
  pfVar1 = in_RCX + 3;
  fVar24 = fVar37 * 1.7 + fVar24;
  fVar15 = fVar38 * 1.7 + fVar15;
  fVar17 = fVar40 * 1.7 + fVar17;
  fVar13 = fVar41 * 0.0 + fVar13;
  fVar14 = fVar42 * 0.0 + fVar14;
  in_RCX[3] = fVar24;
  in_RCX[4] = fVar15;
  in_RCX[5] = fVar17;
  iVar9 = *in_RDX;
  if (iVar9 == 0) {
    fVar37 = in_RCX[2];
  }
  else {
    fVar37 = in_RCX[2];
    fStack_140 = (float)in_RDX[3] - fVar37;
    local_148 = CONCAT44((float)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20) - in_RCX[1],
                         (float)*(undefined8 *)(in_RDX + 1) - fVar36);
    ::utils::time_continuous_lerp_vec3();
    in_RCX[5] = local_15c;
    *(ulonglong *)pfVar1 = CONCAT44(uStack_160,local_164);
    fVar24 = in_RCX[3];
    fVar15 = in_RCX[4];
    fVar17 = in_RCX[5];
    fVar13 = 0.0;
    fVar14 = 0.0;
  }
  fVar22 = 0.0;
  fVar26 = 1.0;
  fVar34 = 1.0 / SQRT(fVar17 * fVar17 + fVar24 * fVar24 + fVar15 * fVar15);
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  if (0.5 <= extraout_XMM0_Da) {
    if (iVar9 != 0) {
      fVar26 = 1.5;
    }
LAB_141dc3147:
    fVar24 = fVar24 - (float)local_f8;
    fVar15 = fVar15 - local_f8._4_4_;
    fVar17 = fVar17 - fVar32;
    auVar39._0_8_ = CONCAT44(fVar17,fVar15);
    auVar39._8_4_ = fVar13 - uStack_f0._4_4_;
    auVar39._12_4_ = fVar14 - uStack_f0._4_4_;
    auVar12._12_4_ = auVar39._12_4_ * auVar39._12_4_;
    fVar17 = fVar17 * fVar17 + fVar24 * fVar24 + fVar15 * fVar15;
    if (fVar26 * fVar26 < fVar17) {
      auVar12._0_4_ = SQRT(fVar17);
      auVar12._4_4_ = auVar12._0_4_;
      auVar12._8_4_ = auVar12._12_4_;
      auVar29 = divps(auVar39,auVar12);
      fVar24 = (fVar24 / auVar12._0_4_) * fVar26;
      auVar39._0_8_ = CONCAT44(auVar29._4_4_ * fVar26,auVar29._0_4_ * fVar26);
    }
    fVar17 = fVar26 * -0.5;
    fVar26 = fVar26 * 0.5;
    if (fVar26 < fVar17) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    if (fVar17 <= (float)auVar39._0_8_) {
      fVar17 = (float)auVar39._0_8_;
    }
    if (fVar17 <= fVar26) {
      fVar26 = fVar17;
    }
    fVar24 = (float)local_f8 + fVar24;
    fVar26 = local_f8._4_4_ + fVar26;
    auVar20._0_8_ = CONCAT44(fVar26,fVar24);
    auVar20._8_4_ = (float)uStack_f0 + fVar22;
    auVar20._12_4_ = uStack_f0._4_4_ + 0.0;
    fVar32 = fVar32 + (float)((ulonglong)auVar39._0_8_ >> 0x20);
    *(undefined8 *)(in_RCX + 3) = auVar20._0_8_;
    in_RCX[5] = fVar32;
    fVar17 = in_stack_00000038;
    if (*in_RDX != 0) {
      ::utils::lerp();
      fVar17 = extraout_XMM0_Da_01 * in_stack_00000038;
    }
    fVar24 = fVar32 * fVar32 + fVar26 * fVar26 + fVar24 * fVar24;
    if ((fVar24 < fVar17 * fVar17) ||
       (fVar17 = in_stack_00000040, in_stack_00000040 * in_stack_00000040 < fVar24)) {
      fVar24 = SQRT(fVar24);
      auVar16._4_4_ = fVar24;
      auVar16._0_4_ = fVar24;
      auVar16._8_8_ = 0;
      auVar29 = divps(auVar20,auVar16);
      auVar20._0_8_ = CONCAT44(auVar29._4_4_ * fVar17,auVar29._0_4_ * fVar17);
      fVar32 = (fVar32 / fVar24) * fVar17;
    }
    *(undefined8 *)(in_RCX + 3) = auVar20._0_8_;
    in_RCX[5] = fVar32;
    if (((0x7f7fffff < ((uint)auVar20._0_8_ & 0x7fffffff)) ||
        (0x7f7fffff < ((uint)((ulonglong)auVar20._0_8_ >> 0x20) & 0x7fffffff))) ||
       (0x7f7fffff < (uint)ABS(fVar32))) {
      in_RCX[5] = 0.0;
      pfVar1[0] = 0.0;
      pfVar1[1] = 0.0;
      auVar20._0_8_ = 0;
      fVar32 = 0.0;
    }
    fVar24 = in_stack_00000060 * (float)auVar20._0_8_;
    fVar15 = in_stack_00000060 * (float)((ulonglong)auVar20._0_8_ >> 0x20);
    fVar32 = fVar32 * in_stack_00000060;
    fVar17 = 1.0 / SQRT(fVar32 * fVar32 + fVar15 * fVar15 + fVar24 * fVar24);
    if ((int)fVar17 - 1U < 0x7fffff || (int)ABS(fVar17) - 0x800000U < 0x7f000000 && -1 < (int)fVar17
       ) {
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if (0.5 <= extraout_XMM0_Da_02) {
        *(ulonglong *)in_RCX = CONCAT44(fVar15 + in_RCX[1],fVar24 + fVar36);
        in_RCX[2] = fVar32 + fVar37;
      }
      else {
        *(ulonglong *)in_RCX = CONCAT44(in_RCX[1] - fVar15,fVar36 - fVar24);
        in_RCX[2] = fVar37 - fVar32;
        *(ulonglong *)(in_RCX + 3) =
             CONCAT44(in_stack_00000038 * -fVar15,in_stack_00000038 * -fVar24);
        in_RCX[5] = -fVar32 * in_stack_00000038;
      }
    }
  }
  else {
    if ((int)fVar34 - 1U < 0x7fffff || (int)ABS(fVar34) - 0x800000U < 0x7f000000 && -1 < (int)fVar34
       ) {
      glam::f32::sse2::quat::Quat::from_rotation_arc();
      fVar17 = (float)local_148;
      fVar24 = local_148._4_4_;
      fVar15 = fStack_140;
    }
    else {
      fVar17 = 0.0;
      fVar24 = 0.0;
      fVar15 = 0.0;
      fStack_13c = 1.0;
    }
    local_150 = <system_critters::boids::RAY_DIRS_as_core::ops::deref::Deref>::deref::__stability::
                LAZY;
    if (DAT_14363aad0 != 3) {
      local_e0 = &local_150;
      std::sys::sync::once::futex::Once::call();
    }
    pcVar8 = local_150;
    fVar14 = fStack_13c * fStack_13c;
    fVar13 = fVar15 * fVar15 + fVar24 * fVar24 + fVar17 * fVar17;
    fVar22 = fStack_13c + fStack_13c;
    fVar36 = *in_RCX;
    fVar37 = in_RCX[2];
    lVar11 = 0;
    do {
      pfVar2 = (float *)(pcVar8 + lVar11 + 0x10);
      fVar26 = *pfVar2;
      fVar34 = pfVar2[1];
      fVar38 = pfVar2[2];
      fVar40 = fVar15 * fVar38 + fVar24 * fVar34 + fVar17 * fVar26;
      fVar40 = fVar40 + fVar40;
      fVar41 = (fVar24 * fVar38 - fVar34 * fVar15) * fVar22 +
               fVar40 * fVar17 + (fVar14 - fVar13) * fVar26;
      fVar42 = (fVar15 * fVar26 - fVar38 * fVar17) * fVar22 +
               fVar40 * fVar24 + (fVar14 - fVar13) * fVar34;
      fVar26 = (fVar17 * fVar34 - fVar26 * fVar24) * fVar22 +
               fVar40 * fVar15 + (fVar14 - fVar13) * fVar38;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if (0.5 <= extraout_XMM0_Da_00) {
        fVar17 = in_RCX[3];
        fVar13 = 1.0 / SQRT(fVar26 * fVar26 + fVar42 * fVar42 + fVar41 * fVar41);
        bVar6 = (int)fVar13 - 1U < 0x7fffff ||
                (int)ABS(fVar13) - 0x800000U < 0x7f000000 && -1 < (int)fVar13;
        if (bVar6) {
          fVar41 = fVar41 * fVar13;
          fVar22 = fVar42;
        }
        else {
          fVar41 = 0.0;
          fVar22 = 0.0;
        }
        uVar10 = (uint)bVar6;
        fVar15 = (float)*(undefined8 *)(in_RCX + 4);
        fVar34 = (float)((ulonglong)*(undefined8 *)(in_RCX + 4) >> 0x20);
        fVar14 = SQRT(fVar34 * fVar34 + fVar17 * fVar17 + fVar15 * fVar15);
        fVar24 = fVar41 * fVar14 * 6.0 + fVar17 * -5.0;
        fVar15 = fVar14 * (float)((int)(uVar10 << 0x1f) >> 0x1f & (uint)(fVar42 * fVar13)) * 6.0 +
                 fVar15 * -5.0;
        fVar17 = fVar14 * (float)((int)(uVar10 << 0x1f) >> 0x1f & (uint)(fVar26 * fVar13)) * 6.0 +
                 fVar34 * -5.0;
        fVar13 = 0.0;
        fVar14 = 0.0;
        in_RCX[3] = fVar24;
        in_RCX[4] = fVar15;
        in_RCX[5] = fVar17;
        fVar26 = 3.0;
        goto LAB_141dc3147;
      }
      lVar11 = lVar11 + 0x10;
    } while (lVar11 != 0x200);
  }
  return;
LAB_141dc2a66:
  auVar28._8_8_ = 0;
  auVar28._0_8_ = uVar27;
  in_R9 = in_R9 + 6;
  if (fVar33 < in_stack_00000048 * in_stack_00000048) {
    fVar33 = SQRT(fVar33);
    fVar30 = (fVar33 - in_stack_00000048) / (0.0 - in_stack_00000048);
    fVar25 = 0.0;
    if (0.0 <= fVar30) {
      fVar25 = fVar30;
    }
    fVar30 = 1.0;
    if (fVar25 <= 1.0) {
      fVar30 = fVar25;
    }
    fVar25 = (3.0 - (fVar30 + fVar30)) * fVar30 * fVar30;
    fVar37 = fVar37 - (fVar23 / fVar33) * fVar25;
    auVar3._4_4_ = fVar33;
    auVar3._0_4_ = fVar33;
    auVar3._8_8_ = 0;
    auVar29 = divps(auVar28,auVar3);
    fVar38 = fVar38 - fVar25 * auVar29._0_4_;
    fVar40 = fVar40 - fVar25 * auVar29._4_4_;
    fVar41 = fVar41 - auVar29._8_4_ * 0.0;
    fVar42 = fVar42 - auVar29._12_4_ * 0.0;
  }
  iVar9 = iVar9 + 1;
  fVar22 = fVar22 + fVar31;
  local_138 = local_138 + fVar19;
  fStack_134 = fStack_134 + fVar21;
  fStack_130 = fStack_130 + 0.0;
  fStack_12c = fStack_12c + 0.0;
  auVar29._4_12_ = local_108._4_12_;
  auVar29._0_4_ = local_108._0_4_ + (float)auVar39._0_8_;
  auVar35._0_4_ = (float)local_118._0_4_ + (float)((ulonglong)auVar39._0_8_ >> 0x20);
  auVar35._4_4_ = (float)local_118._4_4_ + fVar18;
  auVar35._8_4_ = fStack_110 + 0.0;
  auVar35._12_4_ = fStack_10c + 0.0;
  if (in_R9 == pfVar1) goto LAB_141dc2b5e;
  goto LAB_141dc288a;
LAB_141dc2b5e:
  if (iVar9 != 0) {
    fVar18 = (float)iVar9;
    fVar26 = auVar29._0_4_ / fVar18 - fVar26;
    fVar14 = 1.0 - in_stack_00000050;
    fVar24 = (fVar22 / fVar18) * in_stack_00000050 + fVar24 * fVar14;
    auVar4._4_4_ = fVar18;
    auVar4._0_4_ = fVar18;
    auVar4._8_8_ = 0;
    auVar29 = divps(auVar35,auVar4);
    fVar34 = auVar29._0_4_ - fVar34;
    fVar22 = auVar29._4_4_ - fVar36;
    fVar33 = 1.0 / SQRT(fVar22 * fVar22 + fVar26 * fVar26 + fVar34 * fVar34);
    bVar6 = (int)fVar33 - 1U < 0x7fffff ||
            (int)ABS(fVar33) - 0x800000U < 0x7f000000 && -1 < (int)fVar33;
    if (bVar6) {
      fVar22 = fVar22 * fVar33;
    }
    else {
      fVar22 = 0.0;
    }
    auVar7._4_4_ = fStack_134;
    auVar7._0_4_ = local_138;
    auVar7._8_4_ = fStack_130;
    auVar7._12_4_ = fStack_12c;
    auVar5._4_4_ = fVar18;
    auVar5._0_4_ = fVar18;
    auVar5._8_8_ = 0;
    auVar29 = divps(auVar7,auVar5);
    fVar15 = in_stack_00000050 * auVar29._0_4_ + fVar14 * fVar15;
    fVar17 = in_stack_00000050 * auVar29._4_4_ + fVar14 * fVar17;
    fVar14 = auVar29._12_4_ * 0.0 + 0.0;
    uVar10 = (uint)bVar6;
    fVar18 = SQRT(fVar17 * fVar17 + fVar24 * fVar24 + fVar15 * fVar15);
    fVar24 = fVar22 * fVar18 * 0.005 + fVar24;
    fVar15 = fVar18 * (float)((uint)(fVar26 * fVar33) & (int)(uVar10 << 0x1f) >> 0x1f) * 0.005 +
             fVar15;
    fVar17 = fVar18 * (float)((uint)(fVar34 * fVar33) & (int)(uVar10 << 0x1f) >> 0x1f) * 0.005 +
             fVar17;
    fVar13 = fVar14 * fVar14 * 0.0 * 0.0 + auVar29._8_4_ * 0.0 + fVar13 * 0.0;
    fVar14 = fVar14 * fVar14 * 0.0 * 0.0 + fVar14;
  }
  goto LAB_141dc2cb9;
}

