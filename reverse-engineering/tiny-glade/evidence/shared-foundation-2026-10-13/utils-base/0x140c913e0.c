/* WARNING: Removing unreachable block (ram,0x000140c9147f) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_140c913e0(float *param_1,float param_2,float param_3)
{
  uint uVar1;
  float fVar2;
  float fVar4;
  float fVar5;
  float fVar6;
  undefined1 auVar3 [16];
  uint uVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined1 auVar11 [16];
  float fVar12;
  fVar9 = _DAT_142925904 / SQRT(param_3 * param_3 + param_2 * param_2);
  uVar1 = (uint)((int)fVar9 - 1U < 0x7fffff ||
                (int)ABS(fVar9) - 0x800000U < 0x7f000000 && -1 < (int)fVar9);
  uVar7 = (int)(uVar1 << 0x1f) >> 0x1f;
  fVar4 = (float)((uint)(param_3 * fVar9) & (int)(uVar1 << 0x1f) >> 0x1f);
  fVar8 = (float)(~uVar7 & (uint)_DAT_142925904 | (uint)(param_2 * fVar9) & uVar7);
  fVar9 = _DAT_1429cd0c0 * fVar8 - fVar4 * 0.0;
  fVar5 = _UNK_1429cd0c4 * 0.0 - _DAT_142925904 * fVar8;
  fVar6 = _UNK_1429cd0c8 * fVar4 - 0.0;
  auVar11._4_4_ = fVar9;
  auVar11._0_4_ = fVar6;
  auVar11._8_4_ = fVar5;
  auVar11._12_4_ = fVar5;
  auVar3._0_4_ = fVar5 * fVar5 + fVar6 * fVar6 + fVar9 * fVar9;
  auVar3._4_4_ = auVar3._0_4_;
  auVar3._8_4_ = auVar3._0_4_;
  auVar3._12_4_ = auVar3._0_4_;
  auVar3 = sqrtps(auVar3,auVar3);
  auVar3 = divps(auVar11,auVar3);
  fVar9 = (float)(auVar3._0_4_ ^ _DAT_1429258f0);
  fVar5 = (float)(auVar3._4_4_ ^ (uint)_UNK_1429258f4);
  fVar6 = (float)(auVar3._8_4_ ^ _UNK_1429258f8);
  fVar8 = (float)((uint)fVar8 ^ _DAT_1429258f0);
  fVar4 = (float)((uint)fVar4 ^ _UNK_1429258f8);
  if (fVar4 <= 0.0) {
    fVar9 = _DAT_142925904 - fVar9;
    if (fVar9 <= 0.0) {
      fVar9 = (_DAT_142925904 - fVar4) - fVar9;
      fVar10 = _DAT_142925870 / SQRT(fVar9);
      fVar12 = fVar9 * fVar10;
      fVar4 = (fVar5 + 0.0) * fVar10;
      fVar2 = (fVar6 + fVar8) * fVar10;
      fVar10 = (0.0 - _UNK_1429258f4) * fVar10;
    }
    else {
      fVar9 = (_DAT_142925904 - fVar4) + fVar9;
      fVar10 = _DAT_142925870 / SQRT(fVar9);
      fVar12 = (fVar5 + 0.0) * fVar10;
      fVar4 = fVar9 * fVar10;
      fVar2 = (_UNK_1429258f4 + 0.0) * fVar10;
      fVar10 = (fVar8 - fVar6) * fVar10;
    }
  }
  else {
    fVar9 = fVar9 + _DAT_142925904;
    if (fVar9 <= 0.0) {
      fVar9 = (_DAT_142925904 + fVar4) - fVar9;
      fVar10 = _DAT_142925870 / SQRT(fVar9);
      fVar12 = (fVar6 + fVar8) * fVar10;
      fVar4 = (_UNK_1429258f4 + 0.0) * fVar10;
      fVar2 = fVar9 * fVar10;
      fVar10 = fVar5 * fVar10;
    }
    else {
      fVar9 = _DAT_142925904 + fVar4 + fVar9;
      fVar10 = _DAT_142925870 / SQRT(fVar9);
      fVar12 = (0.0 - _UNK_1429258f4) * fVar10;
      fVar4 = (fVar8 - fVar6) * fVar10;
      fVar2 = fVar5 * fVar10;
      fVar10 = fVar10 * fVar9;
    }
  }
  *param_1 = fVar12;
  param_1[1] = fVar4;
  param_1[2] = fVar2;
  param_1[3] = fVar10;
  return;
}
