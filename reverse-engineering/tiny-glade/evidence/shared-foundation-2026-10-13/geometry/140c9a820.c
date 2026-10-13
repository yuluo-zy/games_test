utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c9a820(undefined8 *param_1,float param_2,float param_3)

{
  undefined1 auVar1 [16];
  undefined4 in_EAX;
  int iVar2;
  float fVar4;
  ulonglong uVar3;
  float fVar5;
  undefined1 auVar6 [16];
  float fVar7;
  float fVar8;
  float fVar9;
  
  fVar5 = (float)((ulonglong)*param_1 >> 0x20);
  fVar4 = (float)*param_1;
  fVar9 = fVar4 * fVar4 - (float)((uint)fVar5 ^ _DAT_1429258f0) * fVar5;
  auVar1._4_4_ = fVar9;
  auVar1._0_4_ = fVar9;
  auVar1._8_4_ = fVar9;
  auVar1._12_4_ = fVar9;
  auVar6 = divps(_DAT_142a901b0,auVar1);
  fVar8 = (float)((ulonglong)param_1[1] >> 0x20);
  fVar7 = (float)param_1[1];
  fVar9 = (float)param_1[2] * _DAT_142a171f0 +
          (param_3 - fVar8) * (float)((uint)fVar5 ^ _DAT_1429258f0) * auVar6._8_4_ +
          (param_2 - fVar7) * fVar4 * auVar6._0_4_;
  fVar4 = (float)((ulonglong)param_1[2] >> 0x20) * _UNK_142a171f4 +
          (param_3 - fVar8) * fVar4 * auVar6._12_4_ + (param_2 - fVar7) * fVar5 * auVar6._4_4_;
  auVar6._4_4_ = -(uint)(fVar9 < 0.0);
  auVar6._0_4_ = -(uint)(fVar9 < 0.0);
  auVar6._8_4_ = -(uint)(fVar4 < 0.0);
  auVar6._12_4_ = -(uint)(fVar4 < 0.0);
  iVar2 = movmskpd(in_EAX,auVar6);
  if (iVar2 != 0) {
    fVar5 = fVar4;
    if (fVar4 <= fVar9) {
      fVar5 = fVar9;
    }
    return (ulonglong)(~-(uint)NAN(fVar9) & (uint)fVar5) |
           CONCAT44(fVar4,-(uint)NAN(fVar9) & (uint)fVar4);
  }
                    /* WARNING: Could not recover jumptable at 0x000142923cf0. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  uVar3 = _hypotf();
  return uVar3;
}

