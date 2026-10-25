utils::geometry::circle::Circle2d::intersect_ray

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c97980(undefined4 *param_1,undefined8 *param_2,float param_3,float param_4,float param_5
                  ,float param_6)

{
  undefined4 uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  
  fVar2 = param_6 * param_6 + param_5 * param_5;
  uVar1 = 0;
  if ((fVar2 != 0.0) || (NAN(fVar2))) {
    param_3 = param_3 - (float)*param_2;
    param_4 = param_4 - (float)((ulonglong)*param_2 >> 0x20);
    param_6 = param_6 * param_4;
    fVar3 = param_6 + param_5 * param_3;
    fVar3 = fVar3 + fVar3;
    fVar4 = _DAT_142a8cc68 * fVar2 *
            ((param_4 * param_4 + param_3 * param_3) -
            *(float *)(param_2 + 1) * *(float *)(param_2 + 1)) + fVar3 * fVar3;
    if (0.0 <= fVar4) {
      fVar4 = SQRT(fVar4);
      auVar5._0_4_ = (float)(_DAT_1429258f0 ^ (uint)fVar3) - fVar4;
      auVar5._4_4_ = fVar4 - fVar3;
      auVar5._8_4_ = (float)(_UNK_1429258f4 ^ (uint)param_6) - 0.0;
      auVar5._12_4_ = 0.0 - param_6;
      auVar6._4_4_ = fVar2 + fVar2;
      auVar6._0_4_ = fVar2 + fVar2;
      auVar6._8_8_ = 0;
      auVar6 = divps(auVar5,auVar6);
      *(longlong *)(param_1 + 1) = auVar6._0_8_;
      uVar1 = 1;
    }
  }
  *param_1 = uVar1;
  return;
}

