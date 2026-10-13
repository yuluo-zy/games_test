utils::geometry::barycentric::barycentric

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1416c2f20(float *param_1,undefined8 *param_2,undefined8 *param_3,float *param_4,
                  float *param_5)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar8;
  float fVar9;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  
  fVar12 = *(float *)(param_3 + 1);
  fVar13 = (float)*param_3;
  fVar14 = (float)((ulonglong)*param_3 >> 0x20);
  fVar1 = *param_4 - fVar13;
  fVar2 = *param_5 - fVar13;
  fVar10 = param_4[1] - fVar14;
  fVar11 = param_5[1] - fVar14;
  fVar4 = param_4[2] - fVar12;
  fVar8 = param_5[2] - fVar12;
  fVar13 = (float)*param_2 - fVar13;
  fVar14 = (float)((ulonglong)*param_2 >> 0x20) - fVar14;
  fVar3 = fVar8 * fVar4 + fVar11 * fVar10 + fVar2 * fVar1;
  fVar5 = fVar4 * fVar4 + fVar10 * fVar10 + fVar1 * fVar1;
  fVar9 = fVar8 * fVar8 + fVar11 * fVar11 + fVar2 * fVar2;
  fVar2 = (*(float *)(param_2 + 1) - fVar12) * fVar8 + fVar14 * fVar11 + fVar2 * fVar13;
  fVar12 = fVar4 * (*(float *)(param_2 + 1) - fVar12) + fVar1 * fVar13 + fVar10 * fVar14;
  auVar6._0_4_ = fVar5 * fVar2 - fVar3 * fVar12;
  auVar6._4_4_ = fVar9 * fVar12 - fVar3 * fVar2;
  auVar6._8_8_ = 0;
  fVar12 = fVar9 * fVar5 - fVar3 * fVar3;
  auVar7._4_4_ = fVar12;
  auVar7._0_4_ = fVar12;
  auVar7._8_8_ = 0;
  auVar7 = divps(auVar6,auVar7);
  *param_1 = (_DAT_142925904 - auVar7._4_4_) - auVar7._0_4_;
  *(ulonglong *)(param_1 + 1) = CONCAT44(auVar7._0_4_,auVar7._4_4_);
  return;
}

