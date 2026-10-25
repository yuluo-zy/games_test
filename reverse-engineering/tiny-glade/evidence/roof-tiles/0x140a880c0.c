
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tiles(undefined4 *param_1,longlong param_2)

{
  float fVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  
  if ((*(byte *)(param_2 + 8) & 1) != 0) {
    fVar6 = (float)*(undefined8 *)(param_2 + 0x1c);
    fVar7 = (float)((ulonglong)*(undefined8 *)(param_2 + 0x1c) >> 0x20);
    fVar4 = _DAT_142aa50f0 + fVar6;
    fVar5 = _UNK_142aa50f4 + fVar7;
    fVar8 = (SQRT(*(float *)(param_2 + 0x2c)) +
            (_UNK_14292e6b4 - SQRT(*(float *)(param_2 + 0x2c))) * _UNK_142a80ce4) *
            (_DAT_1429258d0 * *(float *)(param_2 + 0x38) +
            (_DAT_14292e6b0 - *(float *)(param_2 + 0x38)) * _DAT_142a80ce0);
    fVar1 = *(float *)(param_2 + 0x34);
    fVar9 = (_DAT_142925904 - fVar1) * _DAT_142925984;
    if (*(char *)(param_2 + 0x3c) == '\0') {
      uVar3 = (ulonglong)(uint)(fVar6 * fVar1 + fVar9);
      uVar2 = 0x3dcccccd00000000;
    }
    else {
      uVar2 = (ulonglong)(uint)(fVar7 * fVar1 + fVar9) << 0x20;
      uVar3 = 0x3dcccccd;
    }
    *(ulonglong *)(param_1 + 3) = uVar2 | uVar3;
    *(ulonglong *)(param_1 + 1) = CONCAT44(fVar5 + fVar8,fVar4 + fVar8);
    *param_1 = 1;
    return;
  }
  *(ulonglong *)(param_1 + 1) =
       CONCAT44(_UNK_142a89ef4,
                ((SQRT(*(float *)(param_2 + 0x2c)) +
                 (_UNK_14292e6b4 - SQRT(*(float *)(param_2 + 0x2c))) * _UNK_142a80ce4) *
                 (_DAT_1429258d0 * *(float *)(param_2 + 0x38) +
                 (_DAT_14292e6b0 - *(float *)(param_2 + 0x38)) * _DAT_142a80ce0) * _DAT_1429cd128 +
                _DAT_142a80a4c) * _DAT_142925870 + *(float *)(param_2 + 0x14));
  *param_1 = 0;
  return;
}

