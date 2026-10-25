utils::geometry::circle::Circle2d::lerp

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140c98170(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,float param_4)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  float fVar5;
  float fVar6;
  undefined8 uVar7;
  undefined8 extraout_XMM0_Qb;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  float fVar10;
  float fVar11;
  
  fVar5 = _DAT_142934674;
  uVar3 = *param_2;
  uVar4 = *param_3;
  fVar1 = *(float *)(param_2 + 1);
  fVar2 = *(float *)(param_3 + 1);
  fVar6 = (float)func_0x000142923f10();
  fVar11 = (float)(~-(uint)(fVar6 < 0.0) & (uint)fVar6 |
                  (uint)(fVar6 + fVar5) & -(uint)(fVar6 < 0.0));
  uVar7 = func_0x000142923f10();
  fVar6 = (float)uVar7;
  auVar8._4_4_ = (int)((ulonglong)uVar7 >> 0x20);
  auVar8._0_4_ = (uint)(fVar6 + fVar5) & -(uint)(fVar6 < 0.0);
  auVar8._8_4_ = (int)extraout_XMM0_Qb;
  auVar8._12_4_ = (int)((ulonglong)extraout_XMM0_Qb >> 0x20);
  auVar8 = ZEXT416(~-(uint)(fVar6 < 0.0) & (uint)fVar6) | auVar8;
  auVar9._4_12_ = auVar8._4_12_;
  fVar6 = auVar8._0_4_ - fVar11;
  if (_DAT_1429edbb4 <= fVar6) {
    if (_DAT_1429edba4 <= fVar6) {
      fVar6 = fVar6 + _DAT_142ad58ec;
    }
  }
  else {
    fVar6 = fVar6 + _DAT_142934674;
  }
  fVar10 = _DAT_142925904 - param_4;
  auVar9._0_4_ = fVar6 * param_4 + fVar11;
  func_0x000142923f10(auVar9._0_8_);
  fVar6 = (float)func_0x000142923f10();
  *param_1 = CONCAT44(fVar10 * (float)((ulonglong)uVar3 >> 0x20) +
                      (float)((ulonglong)uVar4 >> 0x20) * param_4,
                      fVar10 * (float)uVar3 + (float)uVar4 * param_4);
  *(float *)(param_1 + 1) = fVar1 * fVar10 + fVar2 * param_4;
  *(uint *)((longlong)param_1 + 0xc) =
       ~-(uint)(fVar6 < 0.0) & (uint)fVar6 | (uint)(fVar5 + fVar6) & -(uint)(fVar6 < 0.0);
  return param_1;
}

