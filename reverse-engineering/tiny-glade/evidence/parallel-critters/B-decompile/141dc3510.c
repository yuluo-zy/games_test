
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_critters::boids::Boid::
frame_update<system_critters::wren_boids::sim_wren_boids::closure_env_3>(void)

{
  float *pfVar1;
  float *pfVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  bool bVar6;
  code *pcVar7;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  int iVar8;
  uint uVar9;
  float *in_RCX;
  int *in_RDX;
  longlong lVar10;
  longlong in_R8;
  float *in_R9;
  float extraout_XMM0_Da;
  ulonglong extraout_XMM0_Qa;
  float fVar12;
  undefined1 auVar11 [16];
  float fVar13;
  float fVar14;
  float fVar16;
  undefined1 auVar15 [16];
  float fVar17;
  float fVar18;
  float fVar20;
  undefined1 auVar19 [16];
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  ulonglong uVar26;
  float fVar29;
  undefined1 auVar28 [16];
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar34;
  undefined1 auVar33 [16];
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar39;
  float fVar40;
  float fVar41;
  undefined1 auVar38 [16];
  longlong in_stack_00000028;
  float in_stack_00000038;
  float in_stack_00000040;
  float in_stack_00000048;
  float in_stack_00000050;
  float in_stack_00000060;
  undefined4 local_16c;
  undefined4 uStack_168;
  float local_164;
  code *local_160;
  undefined8 local_158;
  float fStack_150;
  float fStack_14c;
  undefined1 local_148 [8];
  float fStack_140;
  float fStack_13c;
  float local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined1 local_128 [8];
  float fStack_120;
  float fStack_11c;
  undefined1 local_118 [16];
  undefined8 local_108;
  undefined8 uStack_100;
  code **local_f0;
  undefined1 auVar27 [16];
  
  local_108 = *(undefined8 *)(in_RCX + 3);
  uStack_100 = 0;
  fVar32 = in_RCX[5];
  random_vel_change();
  uVar26 = (ulonglong)(uint)in_RCX[4];
  fVar36 = 0.0;
  if (0.0 < in_RCX[4]) {
    ::utils::time_continuous_lerp();
    uVar26 = extraout_XMM0_Qa;
  }
  fVar23 = in_RCX[3];
  fVar16 = in_RCX[5];
  fVar12 = (float)(uVar26 >> 0x20);
  fVar14 = (float)uVar26;
  fVar13 = 0.0;
  if (in_stack_00000028 != 0) {
    uStack_134 = 0;
    uStack_130 = 0;
    uStack_12c = 0;
    pfVar1 = in_R9 + in_stack_00000028 * 6;
    fVar34 = *in_RCX;
    fVar25 = (float)*(undefined8 *)(in_RCX + 1);
    fVar31 = (float)((ulonglong)*(undefined8 *)(in_RCX + 1) >> 0x20);
    fVar37 = 0.0;
    fVar39 = 0.0;
    fVar40 = 0.0;
    fVar41 = 0.0;
    iVar8 = 0;
    fVar21 = 0.0;
    fVar36 = 0.0;
    lVar10 = 0;
    _local_148 = ZEXT816(0);
    auVar28 = ZEXT816(0);
    auVar33 = ZEXT816(0);
LAB_141dc3672:
    _local_128 = auVar33;
    local_118 = auVar28;
    lVar10 = lVar10 + 1;
    auVar28 = local_118;
    auVar33 = _local_128;
    if (0 < iVar8) {
      do {
        if (-in_R8 + lVar10 != 1) {
          fVar17 = *in_R9;
          fVar22 = fVar17 - fVar34;
          auVar38._0_8_ = *(undefined8 *)(in_R9 + 1);
          fVar24 = (float)auVar38._0_8_ - fVar25;
          fVar29 = (float)((ulonglong)auVar38._0_8_ >> 0x20) - fVar31;
          uVar26 = CONCAT44(fVar29,fVar24);
          fVar35 = fVar29 * fVar29 + fVar22 * fVar22 + fVar24 * fVar24;
          if (fVar35 < __real_41800000) {
            fVar30 = in_R9[3];
            fVar18 = in_R9[4];
            fVar20 = in_R9[5];
            if ((0.0 <= fVar16 * fVar20 + fVar23 * fVar30 + fVar14 * fVar18) &&
               (__real_bf000000 <=
                (fVar29 * fVar20 + fVar22 * fVar30 + fVar24 * fVar18) /
                SQRT((fVar20 * fVar20 + fVar30 * fVar30 + fVar18 * fVar18) * fVar35)))
            goto LAB_141dc3868;
          }
        }
        in_R9 = in_R9 + 6;
        lVar10 = lVar10 + 1;
        if (in_R9 == pfVar1) goto LAB_141dc3957;
      } while( true );
    }
    do {
      if (-in_R8 + lVar10 != 1) {
        fVar17 = *in_R9;
        fVar22 = fVar17 - fVar34;
        auVar38._0_8_ = *(undefined8 *)(in_R9 + 1);
        fVar24 = (float)auVar38._0_8_ - fVar25;
        fVar29 = (float)((ulonglong)auVar38._0_8_ >> 0x20) - fVar31;
        uVar26 = CONCAT44(fVar29,fVar24);
        fVar35 = fVar29 * fVar29 + fVar22 * fVar22 + fVar24 * fVar24;
        if (fVar35 < __real_41800000) {
          fVar30 = in_R9[3];
          fVar18 = in_R9[4];
          fVar20 = in_R9[5];
          if (__real_bf000000 <=
              (fVar29 * fVar20 + fVar22 * fVar30 + fVar24 * fVar18) /
              SQRT((fVar20 * fVar20 + fVar30 * fVar30 + fVar18 * fVar18) * fVar35))
          goto LAB_141dc3868;
        }
      }
      in_R9 = in_R9 + 6;
      lVar10 = lVar10 + 1;
      if (in_R9 == pfVar1) goto LAB_141dc3957;
    } while( true );
  }
  fVar37 = 0.0;
  fVar39 = 0.0;
  fVar40 = 0.0;
  fVar41 = 0.0;
LAB_141dc3adc:
  pfVar1 = in_RCX + 3;
  fVar23 = fVar36 * ___real_3fd9999a + fVar23;
  fVar14 = fVar37 * ___xmm_00000000000000003fd9999a3fd9999a + fVar14;
  fVar16 = fVar39 * _UNK_142f7d7a4 + fVar16;
  fVar12 = fVar40 * _UNK_142f7d7a8 + fVar12;
  fVar13 = fVar41 * _UNK_142f7d7ac + fVar13;
  in_RCX[3] = fVar23;
  in_RCX[4] = fVar14;
  in_RCX[5] = fVar16;
  iVar8 = *in_RDX;
  if (iVar8 == 1) {
    fStack_150 = in_RCX[5];
    local_158 = *(undefined8 *)pfVar1;
    ::utils::time_continuous_lerp_vec3();
    in_RCX[5] = local_164;
    *(ulonglong *)pfVar1 = CONCAT44(uStack_168,local_16c);
    fVar23 = in_RCX[3];
    fVar14 = in_RCX[4];
    fVar16 = in_RCX[5];
    fVar12 = 0.0;
    fVar13 = 0.0;
  }
  fVar36 = __real_3f800000;
  fVar34 = 0.0;
  fVar21 = __real_3f800000 / SQRT(fVar16 * fVar16 + fVar23 * fVar23 + fVar14 * fVar14);
  wren_boids::sim_wren_boids::closure_3();
  if (extraout_AL == '\0') {
    if (iVar8 != 0) {
      fVar36 = __real_3fc00000;
    }
LAB_141dc3f1e:
    fVar23 = fVar23 - (float)local_108;
    fVar14 = fVar14 - local_108._4_4_;
    fVar16 = fVar16 - fVar32;
    auVar38._0_8_ = CONCAT44(fVar16,fVar14);
    auVar38._8_4_ = fVar12 - uStack_100._4_4_;
    auVar38._12_4_ = fVar13 - uStack_100._4_4_;
    auVar11._12_4_ = auVar38._12_4_ * auVar38._12_4_;
    fVar16 = fVar16 * fVar16 + fVar23 * fVar23 + fVar14 * fVar14;
    if (fVar36 * fVar36 < fVar16) {
      auVar11._0_4_ = SQRT(fVar16);
      auVar11._4_4_ = auVar11._0_4_;
      auVar11._8_4_ = auVar11._12_4_;
      auVar28 = divps(auVar38,auVar11);
      fVar23 = (fVar23 / auVar11._0_4_) * fVar36;
      auVar38._0_8_ = CONCAT44(auVar28._4_4_ * fVar36,auVar28._0_4_ * fVar36);
    }
    fVar16 = __real_bf000000 * fVar36;
    fVar36 = fVar36 * __real_3f000000;
    if (fVar36 < fVar16) {
                    /* WARNING: Subroutine does not return */
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    if (fVar16 <= (float)auVar38._0_8_) {
      fVar16 = (float)auVar38._0_8_;
    }
    if (fVar16 <= fVar36) {
      fVar36 = fVar16;
    }
    fVar23 = (float)local_108 + fVar23;
    fVar36 = local_108._4_4_ + fVar36;
    auVar19._0_8_ = CONCAT44(fVar36,fVar23);
    auVar19._8_4_ = (float)uStack_100 + fVar34;
    auVar19._12_4_ = uStack_100._4_4_ + 0.0;
    fVar32 = fVar32 + (float)((ulonglong)auVar38._0_8_ >> 0x20);
    *(undefined8 *)(in_RCX + 3) = auVar19._0_8_;
    in_RCX[5] = fVar32;
    fVar16 = in_stack_00000038;
    if (*in_RDX != 0) {
      ::utils::lerp();
      fVar16 = extraout_XMM0_Da * in_stack_00000038;
    }
    fVar36 = fVar32 * fVar32 + fVar36 * fVar36 + fVar23 * fVar23;
    if ((fVar36 < fVar16 * fVar16) ||
       (fVar16 = in_stack_00000040, in_stack_00000040 * in_stack_00000040 < fVar36)) {
      fVar36 = SQRT(fVar36);
      auVar15._4_4_ = fVar36;
      auVar15._0_4_ = fVar36;
      auVar15._8_8_ = 0;
      auVar28 = divps(auVar19,auVar15);
      auVar19._0_8_ = CONCAT44(auVar28._4_4_ * fVar16,auVar28._0_4_ * fVar16);
      fVar32 = (fVar32 / fVar36) * fVar16;
    }
    *(undefined8 *)(in_RCX + 3) = auVar19._0_8_;
    in_RCX[5] = fVar32;
    if (((0x7f7fffff < ((uint)auVar19._0_8_ & 0x7fffffff)) ||
        (0x7f7fffff < ((uint)((ulonglong)auVar19._0_8_ >> 0x20) & 0x7fffffff))) ||
       (0x7f7fffff < (uint)ABS(fVar32))) {
      in_RCX[5] = 0.0;
      pfVar1[0] = 0.0;
      pfVar1[1] = 0.0;
      auVar19._0_8_ = 0;
      fVar32 = 0.0;
    }
    fVar16 = in_stack_00000060 * (float)auVar19._0_8_;
    fVar23 = in_stack_00000060 * (float)((ulonglong)auVar19._0_8_ >> 0x20);
    fVar32 = fVar32 * in_stack_00000060;
    fVar36 = __real_3f800000 / SQRT(fVar32 * fVar32 + fVar23 * fVar23 + fVar16 * fVar16);
    if ((int)fVar36 - 1U < 0x7fffff || (int)ABS(fVar36) - 0x800000U < 0x7f000000 && -1 < (int)fVar36
       ) {
      wren_boids::sim_wren_boids::closure_3();
      if (extraout_AL_01 == '\0') {
        *(ulonglong *)in_RCX = CONCAT44(fVar23 + in_RCX[1],fVar16 + *in_RCX);
        in_RCX[2] = fVar32 + in_RCX[2];
      }
      else {
        *(ulonglong *)in_RCX = CONCAT44(in_RCX[1] - fVar23,*in_RCX - fVar16);
        in_RCX[2] = in_RCX[2] - fVar32;
        fVar32 = (float)((uint)fVar32 ^ ___xmm_80000000800000008000000080000000);
        *(ulonglong *)(in_RCX + 3) =
             CONCAT44(in_stack_00000038 * (float)((uint)fVar23 ^ _UNK_1429258f4),
                      in_stack_00000038 *
                      (float)((uint)fVar16 ^ ___xmm_80000000800000008000000080000000));
        in_RCX[5] = fVar32 * in_stack_00000038;
      }
    }
  }
  else {
    fVar36 = (float)__xmm_3f800000000000000000000000000000;
    fVar16 = __xmm_3f800000000000000000000000000000._4_4_;
    fVar23 = (float)DAT_14293e0c8;
    fVar14 = DAT_14293e0c8._4_4_;
    if ((int)fVar21 - 1U < 0x7fffff || (int)ABS(fVar21) - 0x800000U < 0x7f000000 && -1 < (int)fVar21
       ) {
      glam::f32::sse2::quat::Quat::from_rotation_arc();
      fVar36 = (float)local_158;
      fVar16 = local_158._4_4_;
      fVar23 = fStack_150;
      fVar14 = fStack_14c;
    }
    local_160 = <system_critters::boids::RAY_DIRS_as_core::ops::deref::Deref>::deref::__stability::
                LAZY;
    if (DAT_14363aad0 != 3) {
      local_f0 = &local_160;
      std::sys::sync::once::futex::Once::call();
    }
    pcVar7 = local_160;
    fVar13 = fVar14 * fVar14;
    fVar12 = fVar23 * fVar23 + fVar16 * fVar16 + fVar36 * fVar36;
    fVar14 = fVar14 + fVar14;
    lVar10 = 0;
    do {
      pfVar2 = (float *)(pcVar7 + lVar10 + 0x10);
      fVar34 = *pfVar2;
      fVar21 = pfVar2[1];
      fVar25 = pfVar2[2];
      fVar31 = fVar23 * fVar25 + fVar16 * fVar21 + fVar36 * fVar34;
      fVar31 = fVar31 + fVar31;
      fVar37 = (fVar16 * fVar25 - fVar21 * fVar23) * fVar14 +
               fVar31 * fVar36 + (fVar13 - fVar12) * fVar34;
      fVar39 = (fVar23 * fVar34 - fVar25 * fVar36) * fVar14 +
               fVar31 * fVar16 + (fVar13 - fVar12) * fVar21;
      fVar21 = (fVar36 * fVar21 - fVar34 * fVar16) * fVar14 +
               fVar31 * fVar23 + (fVar13 - fVar12) * fVar25;
      core::ops::function::impls::impl$0::
      call<tuple_<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env_3>
                ();
      if (extraout_AL_00 == '\0') {
        fVar36 = in_RCX[3];
        fVar16 = __real_3f800000 / SQRT(fVar21 * fVar21 + fVar39 * fVar39 + fVar37 * fVar37);
        bVar6 = (int)fVar16 - 1U < 0x7fffff ||
                (int)ABS(fVar16) - 0x800000U < 0x7f000000 && -1 < (int)fVar16;
        if (bVar6) {
          fVar37 = fVar37 * fVar16;
          fVar34 = fVar39;
        }
        else {
          fVar37 = 0.0;
          fVar34 = 0.0;
        }
        uVar9 = (uint)bVar6;
        fVar14 = (float)*(undefined8 *)(in_RCX + 4);
        fVar13 = (float)((ulonglong)*(undefined8 *)(in_RCX + 4) >> 0x20);
        fVar12 = SQRT(fVar13 * fVar13 + fVar36 * fVar36 + fVar14 * fVar14);
        fVar23 = fVar37 * fVar12 * __real_40c00000 + fVar36 * __real_c0a00000;
        fVar14 = fVar12 * (float)((int)(uVar9 << 0x1f) >> 0x1f & (uint)(fVar39 * fVar16)) *
                 (float)__xmm_000000000000000040c0000040c00000 +
                 fVar14 * ___xmm_0000000000000000c0a00000c0a00000;
        fVar16 = fVar12 * (float)((int)(uVar9 << 0x1f) >> 0x1f & (uint)(fVar21 * fVar16)) *
                 __xmm_000000000000000040c0000040c00000._4_4_ + fVar13 * _UNK_142f7d7b4;
        fVar12 = _UNK_1429cd0d8 * 0.0 + _UNK_142f7d7b8 * 0.0;
        fVar13 = _UNK_1429cd0dc * 0.0 + _UNK_142f7d7bc * 0.0;
        in_RCX[3] = fVar23;
        in_RCX[4] = fVar14;
        in_RCX[5] = fVar16;
        fVar36 = __real_40400000;
        goto LAB_141dc3f1e;
      }
      lVar10 = lVar10 + 0x10;
    } while (lVar10 != 0x200);
  }
  return;
LAB_141dc3868:
  auVar27._8_8_ = 0;
  auVar27._0_8_ = uVar26;
  in_R9 = in_R9 + 6;
  if (fVar35 < in_stack_00000048 * in_stack_00000048) {
    fVar35 = SQRT(fVar35);
    fVar29 = (fVar35 - in_stack_00000048) / (0.0 - in_stack_00000048);
    fVar24 = 0.0;
    if (0.0 <= fVar29) {
      fVar24 = fVar29;
    }
    fVar29 = __real_3f800000;
    if (fVar24 <= __real_3f800000) {
      fVar29 = fVar24;
    }
    fVar24 = (__real_40400000 - (fVar29 + fVar29)) * fVar29 * fVar29;
    fVar36 = fVar36 - (fVar22 / fVar35) * fVar24;
    auVar3._4_4_ = fVar35;
    auVar3._0_4_ = fVar35;
    auVar3._8_8_ = 0;
    auVar28 = divps(auVar27,auVar3);
    fVar37 = fVar37 - fVar24 * auVar28._0_4_;
    fVar39 = fVar39 - fVar24 * auVar28._4_4_;
    fVar40 = fVar40 - auVar28._8_4_ * 0.0;
    fVar41 = fVar41 - auVar28._12_4_ * 0.0;
  }
  iVar8 = iVar8 + 1;
  fVar21 = fVar21 + fVar30;
  local_148._4_4_ = (float)local_148._4_4_ + fVar20;
  local_148._0_4_ = (float)local_148._0_4_ + fVar18;
  fStack_140 = fStack_140 + 0.0;
  fStack_13c = fStack_13c + 0.0;
  auVar28._4_12_ = local_118._4_12_;
  auVar28._0_4_ = local_118._0_4_ + (float)auVar38._0_8_;
  auVar33._4_4_ = (float)local_128._4_4_ + fVar17;
  auVar33._0_4_ = (float)local_128._0_4_ + (float)((ulonglong)auVar38._0_8_ >> 0x20);
  auVar33._8_4_ = fStack_120 + 0.0;
  auVar33._12_4_ = fStack_11c + 0.0;
  if (in_R9 == pfVar1) goto LAB_141dc3957;
  goto LAB_141dc3672;
LAB_141dc3957:
  local_138 = fVar32;
  if (iVar8 != 0) {
    fVar17 = (float)iVar8;
    fVar25 = auVar28._0_4_ / fVar17 - fVar25;
    fVar13 = __real_3f800000 - in_stack_00000050;
    fVar23 = (fVar21 / fVar17) * in_stack_00000050 + fVar23 * fVar13;
    auVar4._4_4_ = fVar17;
    auVar4._0_4_ = fVar17;
    auVar4._8_8_ = 0;
    auVar28 = divps(auVar33,auVar4);
    fVar31 = auVar28._0_4_ - fVar31;
    fVar34 = auVar28._4_4_ - fVar34;
    fVar21 = __real_3f800000 / SQRT(fVar34 * fVar34 + fVar25 * fVar25 + fVar31 * fVar31);
    bVar6 = (int)fVar21 - 1U < 0x7fffff ||
            (int)ABS(fVar21) - 0x800000U < 0x7f000000 && -1 < (int)fVar21;
    if (bVar6) {
      fVar34 = fVar34 * fVar21;
    }
    else {
      fVar34 = 0.0;
    }
    auVar5._4_4_ = fVar17;
    auVar5._0_4_ = fVar17;
    auVar5._8_8_ = 0;
    auVar28 = divps(_local_148,auVar5);
    fVar14 = in_stack_00000050 * auVar28._0_4_ + fVar13 * fVar14;
    fVar16 = in_stack_00000050 * auVar28._4_4_ + fVar13 * fVar16;
    fVar13 = auVar28._12_4_ * 0.0 + 0.0;
    uVar9 = (uint)bVar6;
    fVar17 = SQRT(fVar16 * fVar16 + fVar23 * fVar23 + fVar14 * fVar14);
    fVar23 = fVar34 * fVar17 * ___real_3ba3d70a + fVar23;
    fVar14 = fVar17 * (float)((uint)(fVar25 * fVar21) & (int)(uVar9 << 0x1f) >> 0x1f) *
             ___xmm_00000000000000003ba3d70a3ba3d70a + fVar14;
    fVar16 = fVar17 * (float)((uint)(fVar31 * fVar21) & (int)(uVar9 << 0x1f) >> 0x1f) *
             _UNK_142f7d784 + fVar16;
    fVar12 = fVar13 * fVar13 * 0.0 * _UNK_142f7d788 + auVar28._8_4_ * 0.0 + fVar12 * 0.0;
    fVar13 = fVar13 * fVar13 * 0.0 * _UNK_142f7d78c + fVar13;
  }
  goto LAB_141dc3adc;
}

