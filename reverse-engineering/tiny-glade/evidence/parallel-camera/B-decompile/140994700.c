
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::walk::impl$2::translate_screen_space(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  float fVar4;
  uint uVar5;
  float *pfVar6;
  longlong lVar7;
  undefined8 *puVar8;
  longlong lVar9;
  longlong in_RCX;
  bool bVar10;
  float *in_RDX;
  undefined **ppuVar12;
  undefined8 *puVar13;
  int iVar14;
  longlong lVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  float fVar21;
  float fVar24;
  float fVar25;
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  float in_XMM2_Da;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float unaff_XMM9_Dc;
  float fVar33;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  char **local_168;
  code *local_160;
  char *local_158;
  float local_150;
  float fStack_14c;
  float fStack_148;
  float local_144;
  float local_13c;
  undefined *local_138;
  undefined8 local_130;
  char ***local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  float local_108;
  int local_104;
  undefined8 local_f8;
  float local_f0;
  ulonglong uVar11;
  
  *(float *)(in_RCX + 0x50) = in_XMM2_Da;
  fVar30 = __real_3f800000;
  fVar18 = *in_RDX;
  fVar37 = (float)*(undefined8 *)(in_RDX + 1);
  fVar27 = (float)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20);
  fVar16 = fVar27 * fVar27 + fVar18 * fVar18 + fVar37 * fVar37;
  if (__real_2edbe6ff <= fVar16) {
    fVar40 = *(float *)(in_RCX + 0x18);
    fVar39 = *(float *)(in_RCX + 0x1c);
    fVar17 = *(float *)(in_RCX + 0x20);
    fVar38 = *(float *)(in_RCX + 0x24);
    fVar25 = fVar38 * fVar38;
    fVar29 = fVar17 * fVar17 + fVar39 * fVar39 + fVar40 * fVar40;
    fVar26 = fVar27 * fVar17 + fVar37 * fVar39 + fVar18 * fVar40;
    fVar26 = fVar26 + fVar26;
    fVar38 = fVar38 + fVar38;
    fVar35 = fVar38 * (fVar39 * fVar27 - fVar37 * fVar17) +
             fVar26 * fVar40 + (fVar25 - fVar29) * fVar18;
    fVar36 = fVar38 * (fVar17 * fVar18 - fVar27 * fVar40) +
             fVar26 * fVar39 + (fVar25 - fVar29) * fVar37;
    fVar37 = fVar38 * (fVar40 * fVar37 - fVar18 * fVar39) +
             fVar26 * fVar17 + (fVar25 - fVar29) * fVar27;
    fVar18 = fVar36;
    if (*(char *)(in_RCX + 0x54) == '\0') {
      fVar18 = 0.0;
    }
    fVar27 = __real_3f800000 / SQRT(fVar37 * fVar37 + fVar18 * fVar18 + fVar35 * fVar35);
    bVar10 = (int)ABS(fVar27) - 0x800000U < 0x7f000000 && -1 < (int)fVar27;
    uVar11 = (ulonglong)CONCAT31((int3)((int)ABS(fVar27) - 0x800000U >> 8),bVar10);
    bVar10 = (int)fVar27 - 1U < 0x7fffff || bVar10;
    if (bVar10) {
      fVar40 = fVar35 * fVar27;
    }
    else {
      fVar40 = 0.0;
    }
    fVar16 = SQRT(fVar16);
    uVar5 = (uint)bVar10;
    fVar18 = (float)((int)(uVar5 << 0x1f) >> 0x1f & (uint)(fVar18 * fVar27));
    auVar31._0_4_ = fVar16 * fVar18 * ___xmm_00000000410000004100000041000000;
    auVar31._4_4_ = fVar16 * fVar18 * _UNK_142abaa14;
    auVar31._8_4_ =
         fVar16 * (float)((int)(uVar5 << 0x1f) >> 0x1f & (uint)(fVar37 * fVar27)) * _UNK_142abaa18;
    auVar31._12_4_ = unaff_XMM9_Dc * 0.0 * _UNK_142abaa1c;
    auVar31 = divps(auVar31,___xmm_00000000427000004270000042700000);
    auVar32._4_12_ = auVar31._4_12_;
    auVar32._0_4_ = (fVar40 * fVar16 * __real_41000000) / __real_42700000;
    lVar15 = *(longlong *)(in_RCX + 0x10) << 4;
    puVar13 = *(undefined8 **)(in_RCX + 8);
    do {
      if (lVar15 == 0) goto LAB_140994e67;
      puVar1 = puVar13 + 2;
      pfVar6 = (float *)(**(code **)(puVar13[1] + 0x38))(*puVar13);
      lVar7 = (**(code **)(uVar11 + 0x18))(pfVar6);
      fVar27 = _UNK_142abaa5c;
      fVar16 = _UNK_142abaa54;
      fVar18 = ___xmm_00000000000000003dccccd03dccccd0;
      lVar15 = lVar15 + -0x10;
      puVar13 = puVar1;
    } while (((pfVar6 == (float *)0x0) || (lVar7 != 0x6b8463577afc1a66)) ||
            (uVar11 != 0xe4b9f294168d7e3));
    fVar17 = pfVar6[1] + __real_bf000000;
    fVar40 = pfVar6[2];
    fVar39 = *pfVar6;
    fVar25 = 0.0;
    fVar38 = 0.0;
    iVar14 = 8;
    local_13c = in_XMM2_Da;
    while( true ) {
      local_158 = (char *)CONCAT44(fVar17,fVar39);
      fStack_14c = auVar32._0_4_;
      fVar26 = fStack_14c;
      fStack_148 = auVar32._4_4_;
      fVar29 = fStack_148;
      fVar33 = auVar32._8_4_;
      local_168 = (char **)(ulonglong)(uint)__real_3f800000;
      local_160 = (code *)((ulonglong)local_160 & 0xffffffffffff0000);
      local_150 = fVar40;
      local_144 = fVar33;
      collision::world::Raycasting::spherecast();
      fVar34 = auVar32._12_4_;
      if (local_104 == 2) break;
      fVar21 = 0.0;
      if (0.0 <= local_108) {
        fVar21 = local_108;
      }
      fVar4 = fVar30;
      if (fVar21 <= fVar30) {
        fVar4 = fVar21;
      }
      fVar39 = fVar39 + fVar4 * fVar26 * (float)__xmm_00000000000000003f6666663f666666._0_4_;
      fVar17 = fVar17 + fVar4 * fVar29 * (float)__xmm_00000000000000003f6666663f666666._4_4_;
      fVar25 = fVar25 + fVar33 * 0.0 * ram0x000142abaa38;
      fVar38 = fVar38 + fVar34 * 0.0 * _UNK_142abaa3c;
      fVar40 = fVar40 + fVar33 * fVar4 * __real_3f666666;
      fVar28 = fVar30 - fVar4;
      fVar41 = (float)local_110;
      fVar21 = (float)((ulonglong)local_110 >> 0x20);
      fVar42 = fVar30 / SQRT(fVar21 * fVar21 + local_118._4_4_ * local_118._4_4_ + fVar41 * fVar41);
      fVar43 = fVar42 * local_118._4_4_ * fVar28 * fVar26 + fVar41 * fVar42 * fVar29 * fVar28 +
               fVar42 * fVar21 * fVar28 * fVar33;
      fVar21 = fVar33 * fVar4 * fVar18 + (fVar28 * fVar33 - fVar43 * fVar42 * fVar21);
      fVar24 = fVar4 * fVar26 * fVar16 + (fVar28 * fVar26 - fVar43 * fVar42 * local_118._4_4_);
      fVar26 = (fVar29 * fVar28 - fVar41 * fVar42 * fVar43) + fVar4 * fVar29 * ___real_3dccccd0;
      if ((fVar35 * fVar24 + fVar36 * fVar26 + fVar37 * fVar21 <= 0.0) ||
         (fVar24 * fVar24 + fVar26 * fVar26 + fVar21 * fVar21 < __real_2edbe6ff))
      goto LAB_140994bb6;
      auVar32._4_4_ = fVar26;
      auVar32._0_4_ = fVar24;
      auVar32._8_4_ = fVar21;
      auVar32._12_4_ = fVar33 * 0.0 * fVar27 + (fVar34 * 0.0 - fVar34 * 0.0 * 0.0 * 0.0);
      iVar14 = iVar14 + -1;
      if (iVar14 == 0) goto LAB_140994bb6;
    }
    fVar39 = fVar39 + fVar26;
    fVar17 = fVar17 + fVar29;
    fVar25 = fVar25 + fVar33;
    fVar38 = fVar38 + fVar34;
    fVar40 = fVar40 + fVar33;
LAB_140994bb6:
    fVar39 = fVar39 + (float)__xmm_00000000000000003f00000000000000;
    fVar17 = fVar17 + __xmm_00000000000000003f00000000000000._4_4_;
    fVar25 = fVar25 + _UNK_142a80ce8;
    fVar38 = fVar38 + _UNK_142a80cec;
    fVar40 = fVar40 + 0.0;
    fVar18 = expf(local_13c * __real_c1000000);
    fVar30 = fVar30 - fVar18;
    if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
      auVar20._0_8_ = (undefined *)CONCAT44(fVar40,fVar39);
      auVar20._8_4_ = fVar25;
      auVar20._12_4_ = fVar38;
      auVar31 = maxps(___xmm_0000000000000000c2080000c2080000,auVar20);
      auVar23._0_4_ = ~-(uint)NAN(fVar39) & auVar31._0_4_;
      auVar23._4_4_ = ~-(uint)NAN(fVar40) & auVar31._4_4_;
      auVar23._8_4_ = ~-(uint)NAN(fVar25) & auVar31._8_4_;
      auVar23._12_4_ = ~-(uint)NAN(fVar38) & auVar31._12_4_;
      auVar3._4_4_ = __xmm_0000000000000000c2080000c2080000._4_4_ & -(uint)NAN(fVar40);
      auVar3._0_4_ = (uint)__xmm_0000000000000000c2080000c2080000 & -(uint)NAN(fVar39);
      auVar3._8_4_ = __xmm_0000000000000000c2080000c2080000._8_4_ & -(uint)NAN(fVar25);
      auVar3._12_4_ = __xmm_0000000000000000c2080000c2080000._12_4_ & -(uint)NAN(fVar38);
      auVar23 = auVar23 | auVar3;
      auVar31 = minps(___xmm_00000000000000004208000042080000,auVar23);
      fVar18 = (float)(~-(uint)NAN(auVar23._0_4_) & auVar31._0_4_ |
                      (uint)__xmm_00000000000000004208000042080000 & -(uint)NAN(auVar23._0_4_));
      fVar16 = (float)(~-(uint)NAN(auVar23._4_4_) & auVar31._4_4_ |
                      __xmm_00000000000000004208000042080000._4_4_ & -(uint)NAN(auVar23._4_4_));
      fVar37 = fVar39 - fVar18;
      fVar27 = fVar40 - fVar16;
      fVar38 = fVar27 * fVar27 + fVar37 * fVar37;
      auVar19._0_8_ = auVar20._0_8_;
      if (___real_44908000 < fVar38) {
        fVar38 = __real_42080000 / SQRT(fVar38);
        auVar19._0_8_ = (undefined *)CONCAT44(fVar16 + fVar27 * fVar38,fVar18 + fVar37 * fVar38);
      }
    }
    else {
      fVar18 = fVar40 * fVar40 + fVar39 * fVar39;
      auVar19._0_8_ = (undefined *)CONCAT44(fVar40,fVar39);
      auVar19._8_4_ = fVar25;
      auVar19._12_4_ = fVar38;
      if (___real_45908000 < fVar18) {
        auVar22._0_4_ = SQRT(fVar18);
        auVar22._4_4_ = auVar22._0_4_;
        auVar22._8_8_ = 0;
        auVar31 = divps(auVar19,auVar22);
        auVar19._0_8_ =
             (undefined *)
             CONCAT44(auVar31._4_4_ * _UNK_142abaa94,
                      auVar31._0_4_ * ___xmm_00000000000000004288000042880000);
      }
    }
    ppuVar12 = &local_138;
    local_138 = auVar19._0_8_;
    ::utils::swizzlers::impl$1::xvy();
    lVar15 = *(longlong *)(in_RCX + 0x10) << 4;
    puVar13 = *(undefined8 **)(in_RCX + 8);
    do {
      if (lVar15 == 0) {
LAB_140994e67:
        local_168 = &local_158;
        fStack_14c = 0.0;
        local_150 = 4.76441e-44;
        local_158 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
        local_160 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_138 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
        local_130 = 2;
        local_118 = 0;
        local_128 = &local_168;
        local_120 = 1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      puVar1 = puVar13 + 2;
      puVar8 = (undefined8 *)(**(code **)(puVar13[1] + 0x38))(*puVar13);
      lVar7 = (*(code *)ppuVar12[3])(puVar8);
      lVar15 = lVar15 + -0x10;
      puVar13 = puVar1;
    } while (((puVar8 == (undefined8 *)0x0) || (lVar7 != 0x6b8463577afc1a66)) ||
            (ppuVar12 != (undefined **)0xe4b9f294168d7e3));
    uVar2 = *puVar8;
    fVar18 = *(float *)(puVar8 + 1);
    lVar7 = *(longlong *)(in_RCX + 0x10) << 4;
    lVar15 = 0xe4b9f294168d7e3;
    puVar13 = *(undefined8 **)(in_RCX + 8);
    do {
      if (lVar7 == 0) {
        local_158 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
        local_150 = 4.76441e-44;
        fStack_14c = 0.0;
        local_160 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_138 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
        local_130 = 2;
        local_118 = 0;
        local_128 = &local_168;
        local_120 = 1;
        local_168 = &local_158;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      puVar1 = puVar13 + 2;
      puVar8 = (undefined8 *)(**(code **)(puVar13[1] + 0x40))(*puVar13);
      lVar9 = (**(code **)(lVar15 + 0x18))(puVar8);
      lVar7 = lVar7 + -0x10;
      puVar13 = puVar1;
    } while (((puVar8 == (undefined8 *)0x0) || (lVar9 != 0x6b8463577afc1a66)) ||
            (lVar15 != 0xe4b9f294168d7e3));
    *puVar8 = CONCAT44(((fVar30 * ((float)((ulonglong)local_f8 >> 0x20) - fVar17) + fVar17) -
                       (float)((ulonglong)uVar2 >> 0x20)) + (float)((ulonglong)*puVar8 >> 0x20),
                       ((fVar30 * ((float)local_f8 - fVar39) + fVar39) - (float)uVar2) +
                       (float)*puVar8);
    *(float *)(puVar8 + 1) =
         (((local_f0 - fVar40) * fVar30 + fVar40) - fVar18) + *(float *)(puVar8 + 1);
  }
  return;
}

