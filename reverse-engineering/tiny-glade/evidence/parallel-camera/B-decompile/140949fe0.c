
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::normal::impl$1::update(void)

{
  char cVar1;
  float *pfVar2;
  longlong lVar3;
  undefined1 auVar4 [16];
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  uint uVar8;
  uint uVar9;
  undefined8 *in_RCX;
  float *pfVar10;
  longlong in_RDX;
  longlong lVar11;
  float *pfVar12;
  longlong lVar13;
  float *pfVar14;
  float fVar15;
  float fVar16;
  undefined1 auVar17 [16];
  float fVar18;
  ulonglong uVar19;
  float fVar24;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  float in_XMM2_Da;
  float fVar25;
  float fVar30;
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  float fVar31;
  float fVar32;
  undefined1 auVar33 [16];
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
  undefined1 auVar44 [16];
  float local_d8;
  float fStack_d4;
  float fStack_d0;
  float fStack_cc;
  
  if (0.0 <= in_XMM2_Da) {
    fVar15 = expf(in_XMM2_Da * __real_c1000000);
    fVar32 = __real_3f800000;
    fVar18 = (float)*(undefined8 *)(in_RDX + 0x38);
    fVar24 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x38) >> 0x20);
    fVar16 = *(float *)(in_RDX + 0x40);
    fVar40 = (float)*(undefined8 *)(in_RDX + 0x44);
    fVar25 = fVar18 + fVar40;
    fVar43 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x44) >> 0x20);
    fVar30 = fVar24 + fVar43;
    auVar44._4_4_ = fVar30;
    auVar44._0_4_ = fVar25;
    fVar31 = fVar16 + *(float *)(in_RDX + 0x4c);
    fVar42 = *(float *)(in_RDX + 100);
    fVar37 = fVar31 * fVar31 + fVar30 * fVar30 + fVar25 * fVar25;
    fVar36 = fVar31;
    fVar39 = fVar25;
    fVar41 = fVar30;
    if (fVar42 * fVar42 < fVar37) {
      fVar37 = SQRT(fVar37);
      auVar44._8_8_ = 0;
      auVar4._4_4_ = fVar37;
      auVar4._0_4_ = fVar37;
      auVar4._8_8_ = 0;
      auVar44 = divps(auVar44,auVar4);
      fVar39 = fVar42 * auVar44._0_4_;
      fVar41 = fVar42 * auVar44._4_4_;
      fVar36 = (fVar31 / fVar37) * fVar42;
    }
    fVar15 = __real_3f800000 - fVar15;
    *(ulonglong *)(in_RDX + 0x44) = CONCAT44(fVar43 + (fVar41 - fVar30),fVar40 + (fVar39 - fVar25));
    *(float *)(in_RDX + 0x4c) = *(float *)(in_RDX + 0x4c) + (fVar36 - fVar31);
    cVar1 = *(char *)(in_RDX + 0x6c);
    glam::euler::impl$46::from_euler_angles();
    fVar39 = fStack_cc * fStack_cc;
    fVar41 = fStack_d0 * fStack_d0 + fStack_d4 * fStack_d4 + local_d8 * local_d8;
    fVar36 = _UNK_1429a9088 * fStack_d0 +
             _UNK_1429a9084 * fStack_d4 + ___xmm_00000000bf8000008000000080000000 * local_d8;
    fVar36 = fVar36 + fVar36;
    fVar25 = fStack_cc + fStack_cc;
    fVar40 = fVar25 * (fStack_d4 * ___xmm_000000008000000080000000bf800000 -
                      fStack_d0 * ___xmm_0000000080000000bf80000080000000) +
             fVar36 * local_d8 + (fVar39 - fVar41) * ___xmm_00000000bf8000008000000080000000;
    fVar42 = fVar25 * (fStack_d0 * _UNK_1429a90a4 - local_d8 * _UNK_1429a9094) +
             fVar36 * fStack_d4 + (fVar39 - fVar41) * _UNK_1429a9084;
    fVar43 = fVar25 * (local_d8 * _UNK_1429a90a8 - fStack_d4 * _UNK_1429a9098) +
             fVar36 * fStack_d0 + (fVar39 - fVar41) * _UNK_1429a9088;
    fVar36 = fVar25 * (fStack_cc * _UNK_1429a90ac - fStack_cc * _UNK_1429a909c) +
             fVar36 * fStack_cc + (fVar39 - fVar41) * _UNK_1429a908c;
    if (cVar1 == '\0') {
      fVar39 = fVar40 * ___xmm_000000000000000042b3666642b36666 *
               (float)__xmm_00000000000000003e4ccccd3e4ccccd;
      fVar41 = fVar42 * _UNK_142ab3114 * __xmm_00000000000000003e4ccccd3e4ccccd._4_4_;
      fVar42 = fVar43 * ___real_42b36666 * __real_3e4ccccd;
      auVar17._0_4_ = fVar39 + fVar18;
      auVar17._4_4_ = fVar42 + fVar16;
      auVar17._8_4_ = fVar43 * _UNK_142ab3118 * _UNK_142a8c6f8 + 0.0;
      auVar17._12_4_ = fVar36 * _UNK_142ab311c * _UNK_142a8c6fc + 0.0;
      if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
        auVar44 = maxps(___xmm_0000000000000000c2336666c2336666,auVar17);
        auVar29._0_4_ = (uint)__xmm_0000000000000000c2336666c2336666 & -(uint)NAN(auVar17._0_4_);
        auVar29._4_4_ = __xmm_0000000000000000c2336666c2336666._4_4_ & -(uint)NAN(auVar17._4_4_);
        auVar29._8_4_ = __xmm_0000000000000000c2336666c2336666._8_4_ & -(uint)NAN(auVar17._8_4_);
        auVar29._12_4_ = __xmm_0000000000000000c2336666c2336666._12_4_ & -(uint)NAN(auVar17._12_4_);
        auVar23._0_4_ = ~-(uint)NAN(auVar17._0_4_) & auVar44._0_4_;
        auVar23._4_4_ = ~-(uint)NAN(auVar17._4_4_) & auVar44._4_4_;
        auVar23._8_4_ = ~-(uint)NAN(auVar17._8_4_) & auVar44._8_4_;
        auVar23._12_4_ = ~-(uint)NAN(auVar17._12_4_) & auVar44._12_4_;
        minps(___xmm_00000000000000004233666642336666,auVar23 | auVar29);
      }
      else {
        fVar36 = auVar17._4_4_ * auVar17._4_4_ + auVar17._0_4_ * auVar17._0_4_;
        if (___real_45fb70b7 < fVar36) {
          auVar21._0_4_ = SQRT(fVar36);
          auVar21._4_4_ = auVar21._0_4_;
          auVar21._8_4_ = auVar17._12_4_ * auVar17._12_4_;
          auVar21._12_4_ = auVar17._12_4_ * auVar17._12_4_;
          divps(auVar17,auVar21);
        }
      }
      ::utils::swizzlers::impl$1::xvy();
      uVar19 = CONCAT44(fStack_d4 - fVar41,local_d8 - fVar39);
      fVar42 = fStack_d0 - fVar42;
    }
    else {
      fVar39 = *(float *)(in_RDX + 0x60);
      fVar42 = (float)((uint)fVar42 ^ ___xmm_80000000800000008000000080000000);
      if (fVar42 <= __real_3e19999a) {
        fVar42 = __real_3e19999a;
      }
      fVar42 = (fVar24 + __real_c1200000) / fVar42;
      if (fVar42 <= __real_41406667) {
        fVar42 = __real_41406667;
      }
      auVar20._0_4_ =
           fVar18 - fVar42 * (float)((uint)fVar40 ^ ___xmm_80000000800000008000000080000000);
      auVar20._4_4_ = fVar16 - fVar42 * (float)((uint)fVar43 ^ _UNK_1429258f4);
      auVar20._8_4_ = 0.0 - (float)((uint)fVar43 ^ _UNK_1429258f8) * 0.0;
      auVar20._12_4_ = 0.0 - (float)((uint)fVar36 ^ _UNK_1429258fc) * 0.0;
      if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
        auVar44 = maxps(___xmm_0000000000000000c205e666c205e666,auVar20);
        auVar33._0_4_ = (uint)__xmm_0000000000000000c205e666c205e666 & -(uint)NAN(auVar20._0_4_);
        auVar33._4_4_ = __xmm_0000000000000000c205e666c205e666._4_4_ & -(uint)NAN(auVar20._4_4_);
        auVar33._8_4_ = __xmm_0000000000000000c205e666c205e666._8_4_ & -(uint)NAN(auVar20._8_4_);
        auVar33._12_4_ = __xmm_0000000000000000c205e666c205e666._12_4_ & -(uint)NAN(auVar20._12_4_);
        auVar27._0_4_ = ~-(uint)NAN(auVar20._0_4_) & auVar44._0_4_;
        auVar27._4_4_ = ~-(uint)NAN(auVar20._4_4_) & auVar44._4_4_;
        auVar27._8_4_ = ~-(uint)NAN(auVar20._8_4_) & auVar44._8_4_;
        auVar27._12_4_ = ~-(uint)NAN(auVar20._12_4_) & auVar44._12_4_;
        minps(___xmm_00000000000000004205e6664205e666,auVar27 | auVar33);
      }
      else {
        fVar42 = auVar20._4_4_ * auVar20._4_4_ + auVar20._0_4_ * auVar20._0_4_;
        if (___real_458c126b < fVar42) {
          auVar26._0_4_ = SQRT(fVar42);
          auVar26._4_4_ = auVar26._0_4_;
          auVar26._8_4_ = auVar20._12_4_ * auVar20._12_4_;
          auVar26._12_4_ = auVar20._12_4_ * auVar20._12_4_;
          divps(auVar20,auVar26);
        }
      }
      ::utils::swizzlers::impl$1::xvy();
      uVar19 = CONCAT44(fStack_d4,local_d8);
      auVar22._8_8_ = 0;
      auVar22._0_8_ = uVar19;
      fVar36 = fStack_d0 * fStack_d0 + fStack_d4 * fStack_d4 + local_d8 * local_d8;
      fVar42 = fStack_d0;
      if (fVar39 * fVar39 < fVar36) {
        fVar36 = SQRT(fVar36);
        auVar28._4_4_ = fVar36;
        auVar28._0_4_ = fVar36;
        auVar28._8_8_ = 0;
        auVar44 = divps(auVar22,auVar28);
        uVar19 = CONCAT44(auVar44._4_4_ * fVar39,auVar44._0_4_ * fVar39);
        fVar42 = (fStack_d0 / fVar36) * fVar39;
      }
    }
    fVar18 = ((float)uVar19 - fVar18) * fVar15 + fVar18;
    fVar24 = ((float)(uVar19 >> 0x20) - fVar24) * fVar15 + fVar24;
    *(float *)(in_RDX + 0x38) = fVar18;
    *(float *)(in_RDX + 0x3c) = fVar24;
    fVar16 = (fVar42 - fVar16) * fVar15 + fVar16;
    *(float *)(in_RDX + 0x40) = fVar16;
    uVar9 = _UNK_1429258f4;
    uVar8 = ___xmm_80000000800000008000000080000000;
    pfVar2 = *(float **)(in_RDX + 8);
    lVar3 = *(longlong *)(in_RDX + 0x10);
    pfVar10 = (float *)(in_RDX + 0x1c);
    if (*(char *)(in_RDX + 0x18) == '\0') {
      pfVar10 = (float *)0x0;
    }
    pfVar12 = pfVar2;
    do {
      if ((pfVar12 == (float *)0x0) || (pfVar12 == pfVar2 + lVar3 * 7)) {
        if (pfVar10 == (float *)0x0) goto LAB_14094a724;
        pfVar14 = (float *)0x0;
        pfVar12 = pfVar10;
        pfVar10 = (float *)0x0;
      }
      else {
        pfVar14 = pfVar12 + 7;
      }
      fVar16 = (float)*(undefined8 *)(pfVar12 + 2);
      fVar41 = fVar15 * fVar16;
      fVar42 = (float)((ulonglong)*(undefined8 *)(pfVar12 + 2) >> 0x20);
      fVar24 = fVar15 * fVar42;
      *(ulonglong *)(pfVar12 + 2) = CONCAT44(fVar42 - fVar24,fVar16 - fVar41);
      glam::euler::impl$46::from_euler_angles();
      fVar16 = *pfVar12;
      fVar36 = (float)((uint)pfVar12[4] ^ uVar8) * fVar16;
      fVar43 = fVar16 * (float)((uint)*(undefined8 *)(pfVar12 + 5) ^ uVar8);
      fVar25 = fVar16 * (float)((uint)((ulonglong)*(undefined8 *)(pfVar12 + 5) >> 0x20) ^ uVar9);
      fVar30 = fStack_cc * fStack_cc;
      fVar34 = fStack_d0 * fStack_d0 + fStack_d4 * fStack_d4 + local_d8 * local_d8;
      fVar39 = fStack_d0 * fVar25 + fStack_d4 * fVar43 + local_d8 * fVar36;
      fVar39 = fVar39 + fVar39;
      fVar38 = fStack_cc + fStack_cc;
      *pfVar12 = pfVar12[1] * fVar15 + fVar16 * (fVar32 - fVar15);
      *(ulonglong *)(in_RDX + 0x50) =
           CONCAT44((float)((ulonglong)*(undefined8 *)(in_RDX + 0x50) >> 0x20) + fVar24,
                    (float)*(undefined8 *)(in_RDX + 0x50) + fVar41);
      glam::euler::impl$46::from_euler_angles();
      fVar16 = *pfVar12;
      fVar41 = (float)((uint)pfVar12[4] ^ uVar8) * fVar16;
      fVar37 = fVar16 * (float)((uint)*(undefined8 *)(pfVar12 + 5) ^ uVar8);
      fVar16 = fVar16 * (float)((uint)((ulonglong)*(undefined8 *)(pfVar12 + 5) >> 0x20) ^ uVar9);
      fVar31 = fStack_cc * fStack_cc;
      fVar35 = fStack_d0 * fStack_d0 + fStack_d4 * fStack_d4 + local_d8 * local_d8;
      fVar40 = fStack_d0 * fVar16 + fStack_d4 * fVar37 + local_d8 * fVar41;
      fVar40 = fVar40 + fVar40;
      fVar42 = fStack_cc + fStack_cc;
      fVar18 = (float)*(undefined8 *)(in_RDX + 0x38) +
               ((fVar42 * (fStack_d4 * fVar16 - fVar37 * fStack_d0) +
                fVar40 * local_d8 + (fVar31 - fVar35) * fVar41) -
               (fVar38 * (fStack_d4 * fVar25 - fVar43 * fStack_d0) +
               fVar39 * local_d8 + (fVar30 - fVar34) * fVar36));
      fVar24 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x38) >> 0x20) +
               ((fVar42 * (fStack_d0 * fVar41 - fVar16 * local_d8) +
                fVar40 * fStack_d4 + (fVar31 - fVar35) * fVar37) -
               (fVar38 * (fStack_d0 * fVar36 - fVar25 * local_d8) +
               fVar39 * fStack_d4 + (fVar30 - fVar34) * fVar43));
      *(float *)(in_RDX + 0x38) = fVar18;
      *(float *)(in_RDX + 0x3c) = fVar24;
      fVar16 = ((fVar42 * (local_d8 * fVar37 - fVar41 * fStack_d4) +
                fVar40 * fStack_d0 + (fVar31 - fVar35) * fVar16) -
               (fVar38 * (local_d8 * fVar43 - fVar36 * fStack_d4) +
               fVar39 * fStack_d0 + (fVar30 - fVar34) * fVar25)) + *(float *)(in_RDX + 0x40);
      *(float *)(in_RDX + 0x40) = fVar16;
      pfVar12 = pfVar14;
    } while( true );
  }
  *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(in_RDX + 0x40);
  *in_RCX = *(undefined8 *)(in_RDX + 0x38);
  goto LAB_14094a8b3;
LAB_14094a724:
  fVar36 = fVar15 * *(float *)(in_RDX + 0x4c);
  fVar42 = (float)*(undefined8 *)(in_RDX + 0x44);
  fVar39 = fVar15 * fVar42;
  fVar32 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x44) >> 0x20);
  fVar15 = fVar15 * fVar32;
  *(ulonglong *)(in_RDX + 0x44) = CONCAT44(fVar32 - fVar15,fVar42 - fVar39);
  *(float *)(in_RDX + 0x4c) = *(float *)(in_RDX + 0x4c) - fVar36;
  fVar24 = fVar24 + fVar15;
  *(float *)(in_RDX + 0x38) = fVar18 + fVar39;
  if (fVar24 <= __real_3f800000) {
    fVar24 = __real_3f800000;
  }
  *(float *)(in_RDX + 0x40) = fVar16 + fVar36;
  *(float *)(in_RDX + 0x3c) = fVar24;
  fVar42 = __real_1e3ce508;
  fVar16 = __real_3a83126f;
  uVar8 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  lVar3 = *(longlong *)(in_RDX + 0x10);
  if (lVar3 != 0) {
    lVar11 = 1 - lVar3;
    lVar13 = 0;
    pfVar2 = *(float **)(in_RDX + 8);
    do {
      pfVar10 = pfVar2 + 7;
      if (((float)((uint)pfVar2[2] & ___xmm_7fffffff7fffffff7fffffff7fffffff) < __real_3a83126f) &&
         ((float)((uint)pfVar2[3] & ___xmm_7fffffff7fffffff7fffffff7fffffff) < __real_3a83126f)) {
        fVar32 = (float)((uint)pfVar2[1] & ___xmm_7fffffff7fffffff7fffffff7fffffff);
        if (fVar32 <= __real_1e3ce508) {
          fVar32 = __real_1e3ce508;
        }
        if ((float)((uint)(*pfVar2 - pfVar2[1]) & ___xmm_7fffffff7fffffff7fffffff7fffffff) / fVar32
            < __real_3a83126f) {
          if (lVar11 != 0) {
            lVar11 = -lVar11;
            lVar13 = 1;
            goto LAB_14094a82f;
          }
          lVar13 = 1;
          break;
        }
      }
      lVar11 = lVar11 + 1;
      pfVar2 = pfVar10;
    } while (lVar11 != 1);
    goto LAB_14094a87d;
  }
  goto LAB_14094a884;
LAB_14094a82f:
  do {
    if ((fVar16 <= (float)((uint)pfVar10[2] & uVar8)) ||
       (fVar16 <= (float)((uint)pfVar10[3] & uVar8))) {
LAB_14094a810:
      uVar5 = *(undefined8 *)pfVar10;
      uVar6 = *(undefined8 *)(pfVar10 + 2);
      uVar7 = *(undefined8 *)(pfVar10 + 5);
      *(undefined8 *)(pfVar10 + lVar13 * -7 + 3) = *(undefined8 *)(pfVar10 + 3);
      *(undefined8 *)(pfVar10 + lVar13 * -7 + 3 + 2) = uVar7;
      *(undefined8 *)(pfVar10 + lVar13 * -7) = uVar5;
      *(undefined8 *)(pfVar10 + lVar13 * -7 + 2) = uVar6;
    }
    else {
      fVar32 = (float)((uint)pfVar10[1] & uVar8);
      if (fVar32 <= fVar42) {
        fVar32 = fVar42;
      }
      if (fVar16 <= (float)((uint)(*pfVar10 - pfVar10[1]) & uVar8) / fVar32) goto LAB_14094a810;
      lVar13 = lVar13 + 1;
    }
    lVar11 = lVar11 + -1;
    pfVar10 = pfVar10 + 7;
  } while (lVar11 != 0);
LAB_14094a87d:
  *(longlong *)(in_RDX + 0x10) = lVar3 - lVar13;
LAB_14094a884:
  *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(in_RDX + 0x40);
  *in_RCX = *(undefined8 *)(in_RDX + 0x38);
LAB_14094a8b3:
  glam::euler::impl$46::from_euler_angles();
  return;
}

