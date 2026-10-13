utils::geometry::rectangle::Rectangle2d::closest_edge_normal

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9a8f0(undefined8 *param_1,float param_2,float param_3)

{
  undefined1 auVar1 [16];
  float fVar3;
  undefined1 auVar2 [16];
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar9;
  undefined1 auVar8 [16];
  float fVar10;
  
  fVar9 = (float)((ulonglong)*param_1 >> 0x20);
  fVar6 = (float)((uint)fVar9 ^ _DAT_1429258f0);
  fVar7 = (float)*param_1;
  fVar10 = fVar7 * fVar7 - fVar6 * fVar9;
  auVar1._4_4_ = fVar10;
  auVar1._0_4_ = fVar10;
  auVar1._8_4_ = fVar10;
  auVar1._12_4_ = fVar10;
  auVar8 = divps(_DAT_142a901b0,auVar1);
  fVar3 = (float)((ulonglong)param_1[1] >> 0x20);
  fVar5 = (float)param_1[1];
  fVar10 = (param_3 - fVar3) * fVar6 * auVar8._8_4_;
  fVar3 = (param_3 - fVar3) * fVar7 * auVar8._12_4_;
  fVar4 = fVar10 + (param_2 - fVar5) * fVar7 * auVar8._0_4_;
  fVar5 = fVar3 + (param_2 - fVar5) * fVar9 * auVar8._4_4_;
  auVar2._4_4_ = fVar5;
  auVar2._0_4_ = fVar4;
  auVar2._8_4_ = fVar10 + fVar10;
  auVar2._12_4_ = fVar3 + fVar3;
  auVar8._8_8_ = 0;
  auVar8._0_8_ = param_1[2];
  auVar8 = divps(auVar2,auVar8);
  if ((float)(auVar8._0_4_ & _DAT_14292e980) <= (float)(auVar8._4_4_ & _UNK_14292e984)) {
    if (fVar5 <= 0.0) {
      fVar10 = (float)_DAT_1429ddd00;
      fVar3 = (float)((ulonglong)_DAT_1429ddd00 >> 0x20);
    }
    else {
      fVar10 = (float)_DAT_1429345b0;
      fVar3 = (float)((ulonglong)_DAT_1429345b0 >> 0x20);
    }
  }
  else if (fVar4 <= 0.0) {
    fVar10 = (float)_DAT_142a16e30;
    fVar3 = (float)((ulonglong)_DAT_142a16e30 >> 0x20);
  }
  else {
    fVar3 = 0.0;
    fVar10 = _DAT_142925904;
  }
  return CONCAT44(fVar7 * fVar3 + fVar9 * fVar10,fVar6 * fVar3 + fVar7 * fVar10);
}

