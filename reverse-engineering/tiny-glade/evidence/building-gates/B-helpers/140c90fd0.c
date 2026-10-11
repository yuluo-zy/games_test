
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::xz_rotation_bad(void)

{
  float *in_RCX;
  float fVar1;
  float in_XMM1_Da;
  float fVar2;
  undefined1 auVar3 [16];
  float in_XMM2_Da;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar10;
  float fVar11;
  undefined1 auVar9 [16];
  
  fVar7 = __real_3f800000 / SQRT(in_XMM2_Da * in_XMM2_Da + in_XMM1_Da * in_XMM1_Da);
  if ((int)fVar7 - 1U < 0x7fffff || (int)ABS(fVar7) - 0x800000U < 0x7f000000 && -1 < (int)fVar7) {
    fVar2 = in_XMM1_Da * fVar7;
    fVar7 = in_XMM2_Da * fVar7;
  }
  else {
    fVar7 = 0.0;
    fVar2 = __real_3f800000;
  }
  fVar8 = fVar2 * ___xmm_000000003f8000000000000000000000 - fVar7 * 0.0;
  fVar10 = _UNK_1429cd0c4 * 0.0 - __real_3f800000 * fVar2;
  fVar11 = fVar7 * _UNK_1429cd0c8 - 0.0;
  auVar3._4_4_ = fVar8;
  auVar3._0_4_ = fVar11;
  auVar3._8_4_ = fVar10;
  auVar3._12_4_ = fVar7 * _UNK_1429cd0cc - 0.0;
  auVar9._0_4_ = fVar10 * fVar10 + fVar11 * fVar11 + fVar8 * fVar8;
  auVar9._4_4_ = auVar9._0_4_;
  auVar9._8_4_ = auVar9._0_4_;
  auVar9._12_4_ = auVar9._0_4_;
  auVar9 = sqrtps(auVar9,auVar9);
  auVar3 = divps(auVar3,auVar9);
  fVar8 = auVar3._4_4_;
  fVar10 = auVar3._8_4_;
  if (fVar7 <= 0.0) {
    fVar11 = __real_3f800000 - auVar3._0_4_;
    if (0.0 < fVar11) {
      fVar11 = (__real_3f800000 - fVar7) + fVar11;
      fVar5 = __real_3f000000 / SQRT(fVar11);
      fVar6 = (fVar8 + 0.0) * fVar5;
      fVar11 = fVar11 * fVar5;
      fVar7 = fVar5 * 0.0;
      fVar5 = (fVar2 - fVar10) * fVar5;
      goto LAB_140c911bc;
    }
    fVar6 = (__real_3f800000 - fVar7) - fVar11;
    fVar1 = __real_3f000000 / SQRT(fVar6);
    fVar11 = fVar8 + 0.0;
    fVar7 = fVar10 + fVar2;
    fVar5 = 0.0;
    fVar2 = fVar1;
    fVar4 = fVar1;
    fVar8 = fVar1;
  }
  else {
    fVar4 = auVar3._0_4_ + __real_3f800000;
    if (0.0 < fVar4) {
      fVar4 = fVar7 + __real_3f800000 + fVar4;
      fVar5 = __real_3f000000 / SQRT(fVar4);
      fVar6 = fVar5 * 0.0;
      fVar11 = fVar5 * (fVar2 - fVar10);
      fVar7 = fVar5 * fVar8;
      fVar5 = fVar5 * fVar4;
      goto LAB_140c911bc;
    }
    fVar4 = (fVar7 + __real_3f800000) - fVar4;
    fVar6 = __real_3f000000 / SQRT(fVar4);
    fVar1 = fVar2 + fVar10;
    fVar11 = fVar6;
    fVar7 = fVar6;
    fVar5 = fVar6;
    fVar2 = 0.0;
  }
  fVar6 = fVar6 * fVar1;
  fVar11 = fVar11 * fVar2;
  fVar7 = fVar7 * fVar4;
  fVar5 = fVar5 * fVar8;
LAB_140c911bc:
  *in_RCX = fVar6;
  in_RCX[1] = fVar11;
  in_RCX[2] = fVar7;
  in_RCX[3] = fVar5;
  return;
}

