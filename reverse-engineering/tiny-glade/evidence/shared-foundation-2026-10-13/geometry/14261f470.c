utils::geometry::line_relation::LineInterval::relate

void FUN_14261f470(undefined4 *param_1,undefined8 *param_2,undefined8 *param_3)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  undefined4 uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined1 auVar10 [16];
  float fVar11;
  undefined1 auVar12 [16];
  float fVar13;
  
  fVar8 = (float)*param_2;
  fVar5 = (float)param_2[1] - fVar8;
  fVar4 = (float)((ulonglong)*param_2 >> 0x20);
  fVar6 = (float)((ulonglong)param_2[1] >> 0x20) - fVar4;
  auVar10._4_4_ = fVar5;
  auVar10._0_4_ = fVar6;
  auVar10._8_8_ = 0;
  fVar7 = (float)*param_3;
  auVar12._0_4_ = (float)param_3[1] - fVar7;
  fVar9 = (float)((ulonglong)*param_3 >> 0x20);
  auVar12._4_4_ = (float)((ulonglong)param_3[1] >> 0x20) - fVar9;
  auVar12._8_8_ = 0;
  fVar13 = fVar5 * auVar12._4_4_ - fVar6 * auVar12._0_4_;
  fVar7 = fVar7 - fVar8;
  fVar9 = fVar9 - fVar4;
  if ((fVar13 != 0.0) || (NAN(fVar13))) {
    auVar1._4_4_ = fVar13;
    auVar1._0_4_ = fVar13;
    auVar1._8_8_ = 0;
    auVar12 = divps(auVar12,auVar1);
    fVar11 = fVar7 * auVar12._4_4_ - fVar9 * auVar12._0_4_;
    uVar3 = 1;
    if (*(float *)(param_2 + 2) <= fVar11) {
      auVar2._4_4_ = fVar13;
      auVar2._0_4_ = fVar13;
      auVar2._8_8_ = 0;
      auVar10 = divps(auVar10,auVar2);
      fVar7 = fVar7 * auVar10._0_4_ - fVar9 * auVar10._4_4_;
      if (((*(float *)(param_3 + 2) <= fVar7) && (fVar11 <= *(float *)((longlong)param_2 + 0x14)))
         && (fVar7 <= *(float *)((longlong)param_3 + 0x14))) {
        *(ulonglong *)(param_1 + 1) = CONCAT44(fVar4 + fVar6 * fVar11,fVar8 + fVar5 * fVar11);
        uVar3 = 0;
      }
    }
  }
  else {
    fVar8 = fVar7 * fVar6 - fVar9 * fVar5;
    if ((fVar8 != 0.0) || (NAN(fVar8))) {
      uVar3 = 3;
    }
    else {
      uVar3 = 2;
    }
  }
  *param_1 = uVar3;
  return;
}

