utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_140c9a750(undefined8 *param_1,float param_2,float param_3)

{
  undefined1 auVar1 [16];
  undefined4 in_EAX;
  int iVar2;
  longlong lVar3;
  float fVar6;
  undefined1 auVar4 [12];
  undefined1 auVar5 [16];
  float fVar7;
  float fVar8;
  undefined1 auVar9 [12];
  float fVar11;
  undefined1 auVar10 [16];
  float fVar12;
  float fVar13;
  undefined8 extraout_XMM0_Qb;
  
  fVar11 = (float)((ulonglong)*param_1 >> 0x20);
  fVar6 = (float)*param_1;
  fVar13 = fVar6 * fVar6 - (float)((uint)fVar11 ^ _DAT_1429258f0) * fVar11;
  auVar1._4_4_ = fVar13;
  auVar1._0_4_ = fVar13;
  auVar1._8_4_ = fVar13;
  auVar1._12_4_ = fVar13;
  auVar10 = divps(_DAT_142a901b0,auVar1);
  fVar13 = (float)((ulonglong)param_1[1] >> 0x20);
  fVar12 = (float)param_1[1];
  fVar7 = (param_3 - fVar13) * (float)((uint)fVar11 ^ _DAT_1429258f0) * auVar10._8_4_;
  fVar8 = (param_3 - fVar13) * fVar6 * auVar10._12_4_;
  fVar13 = (float)param_1[2] * _DAT_142a171f0 + fVar7 + (param_2 - fVar12) * fVar6 * auVar10._0_4_;
  fVar6 = (float)((ulonglong)param_1[2] >> 0x20) * _UNK_142a171f4 +
          fVar8 + (param_2 - fVar12) * fVar11 * auVar10._4_4_;
  fVar7 = _UNK_142a171f8 * 0.0 + fVar7 + fVar7;
  fVar8 = _UNK_142a171fc * 0.0 + fVar8 + fVar8;
  auVar10._4_4_ = -(uint)(fVar13 < 0.0);
  auVar10._0_4_ = -(uint)(fVar13 < 0.0);
  auVar10._8_4_ = -(uint)(fVar6 < 0.0);
  auVar10._12_4_ = -(uint)(fVar6 < 0.0);
  iVar2 = movmskpd(in_EAX,auVar10);
  if (iVar2 == 0) {
    auVar5._0_8_ = func_0x000142923cf0();
    auVar5._8_8_ = extraout_XMM0_Qb;
    auVar4 = auVar5._0_12_;
  }
  else {
    auVar9._0_4_ = -(uint)NAN(fVar13) & (uint)fVar6;
    auVar9._4_4_ = fVar6;
    auVar9._8_4_ = (uint)fVar7 & (uint)fVar8;
    if (fVar6 <= fVar13) {
      fVar6 = fVar13;
    }
    auVar4._0_4_ = ~-(uint)NAN(fVar13) & (uint)fVar6;
    auVar4._4_4_ = 0;
    auVar4._8_4_ = ~(uint)fVar7 & (uint)fVar8;
    auVar4 = auVar4 | auVar9;
  }
  lVar3 = auVar4._0_8_;
  if (auVar4._0_4_ <= 0.0) {
    lVar3 = auVar4._4_8_ << 0x20;
  }
  return lVar3;
}

