
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint * FUN_1408e4920(uint *param_1,longlong param_2)

{
  int iVar1;
  undefined8 uVar2;
  float fVar3;
  bool bVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  uint uVar8;
  float fVar9;
  uint uVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  
  fVar3 = _DAT_142925904;
  fVar15 = *(float *)(param_2 + 0x2c);
  fVar5 = (float)func_0x000142923f70(param_1,_DAT_142925904 /
                                             (_DAT_1429258d0 * fVar15 +
                                             (_DAT_142925904 - fVar15) * _DAT_142925870));
  bVar4 = *(int *)(param_2 + 8) != 1;
  if (bVar4) {
    fVar7 = (((fVar3 - SQRT(fVar15)) * _DAT_142925870 + SQRT(fVar15)) *
             (*(float *)(param_2 + 0x38) * _DAT_1429258d0 +
             (fVar3 - *(float *)(param_2 + 0x38)) * 0.0) * _DAT_1429cd128 + _DAT_142a80a4c) *
            _DAT_142925870;
    fVar15 = *(float *)(param_2 + 0x14);
    uVar8 = *(uint *)(param_2 + 0x18);
    fVar6 = fVar5 * _DAT_1429cd068;
    *(undefined8 *)(param_1 + 1) = *(undefined8 *)(param_2 + 0xc);
    param_1[3] = (uint)(fVar6 + (fVar3 - fVar5) * (fVar7 + fVar15));
    param_1[4] = uVar8;
  }
  else {
    fVar7 = _DAT_1429258d0 * *(float *)(param_2 + 0x38);
    fVar9 = _UNK_14292e6b4 - *(float *)(param_2 + 0x38);
    fVar12 = _DAT_142925870 * (_DAT_14292e6b0 - SQRT(fVar15));
    fVar6 = *(float *)(param_2 + 0x34);
    fVar11 = (fVar3 - fVar6) * _DAT_142925984;
    uVar2 = *(undefined8 *)(param_2 + 0x14);
    iVar1 = *(int *)(param_2 + 0x3c);
    *(undefined8 *)(param_1 + 1) = *(undefined8 *)(param_2 + 0xc);
    *(undefined8 *)(param_1 + 3) = uVar2;
    fVar13 = (float)*(undefined8 *)(param_2 + 0x1c);
    fVar14 = (float)((ulonglong)*(undefined8 *)(param_2 + 0x1c) >> 0x20);
    fVar15 = (fVar7 + fVar9 * 0.0) * (SQRT(fVar15) + fVar12);
    uVar8 = (iVar1 << 0x1f) >> 0x1f;
    uVar10 = (iVar1 << 0x1f) >> 0x1f;
    *(ulonglong *)(param_1 + 5) =
         CONCAT44(fVar5 * (float)(~uVar10 & _UNK_142aa5114 |
                                 (uint)(fVar14 * fVar6 + fVar11) & uVar10) +
                  (fVar3 - fVar5) * (fVar15 + _UNK_142aa50f4 + fVar14),
                  fVar5 * (float)(~uVar8 & (uint)(fVar13 * fVar6 + fVar11) | _DAT_142aa5100 & uVar8)
                  + (fVar3 - fVar5) * (fVar15 + _DAT_142aa50f0 + fVar13));
  }
  *param_1 = (uint)!bVar4;
  return param_1;
}

