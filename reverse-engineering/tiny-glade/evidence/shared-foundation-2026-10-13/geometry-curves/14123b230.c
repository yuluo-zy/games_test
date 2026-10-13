
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14123b230(undefined4 *param_1,float *param_2,float *param_3,float param_4)

{
  undefined4 uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  float fVar8;
  float fVar9;
  float fVar10;
  
  fVar5 = param_2[3];
  fVar3 = (float)*(undefined8 *)(param_2 + 4);
  fVar4 = (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20);
  fVar2 = fVar4 * fVar4 + fVar5 * fVar5 + fVar3 * fVar3;
  uVar1 = 0;
  if ((fVar2 != 0.0) || (NAN(fVar2))) {
    fVar8 = *param_2 - *param_3;
    fVar9 = (float)*(undefined8 *)(param_2 + 1) - (float)*(undefined8 *)(param_3 + 1);
    fVar10 = (float)((ulonglong)*(undefined8 *)(param_2 + 1) >> 0x20) -
             (float)((ulonglong)*(undefined8 *)(param_3 + 1) >> 0x20);
    fVar4 = fVar4 * fVar10;
    fVar3 = fVar4 + fVar5 * fVar8 + fVar3 * fVar9;
    fVar3 = fVar3 + fVar3;
    fVar5 = _DAT_142a8cc68 * fVar2 *
            ((fVar10 * fVar10 + fVar8 * fVar8 + fVar9 * fVar9) - param_4 * param_4) + fVar3 * fVar3;
    if (0.0 <= fVar5) {
      fVar5 = SQRT(fVar5);
      auVar6._0_4_ = (float)(_DAT_1429258f0 ^ (uint)fVar3) - fVar5;
      auVar6._4_4_ = fVar5 - fVar3;
      auVar6._8_4_ = (float)(_UNK_1429258f4 ^ (uint)fVar4) - 0.0;
      auVar6._12_4_ = 0.0 - fVar4;
      auVar7._4_4_ = fVar2 + fVar2;
      auVar7._0_4_ = fVar2 + fVar2;
      auVar7._8_8_ = 0;
      auVar7 = divps(auVar6,auVar7);
      *(longlong *)(param_1 + 1) = auVar7._0_8_;
      uVar1 = 1;
    }
  }
  *param_1 = uVar1;
  return;
}

