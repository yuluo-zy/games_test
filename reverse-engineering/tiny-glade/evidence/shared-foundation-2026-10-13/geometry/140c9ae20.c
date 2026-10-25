utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9ae20(undefined8 *param_1,longlong param_2,float param_3)

{
  code *pcVar1;
  longlong lVar2;
  float fVar3;
  float fVar4;
  float fVar6;
  float fVar7;
  undefined8 uVar5;
  float in_XMM1_Da;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float afStack_58 [4];
  float fStack_48;
  float fStack_44;
  float fStack_40;
  float fStack_3c;
  
  lVar2 = FUN_140c9a9d0();
  fStack_48 = (float)param_1[2];
  afStack_58[3] = (float)((ulonglong)param_1[2] >> 0x20);
  if (0.0 < afStack_58[3] * fStack_48) {
    if (lVar2 == 4) {
      lVar2 = 0;
    }
    afStack_58[1] = afStack_58[3] * _DAT_142aa5d60;
    afStack_58[0] = fStack_48 * _UNK_142aa5d64;
    afStack_58[3] = afStack_58[3] * _UNK_142aa5d68;
    fStack_48 = fStack_48 * _UNK_142aa5d6c;
    afStack_58[2] = afStack_58[0];
    fStack_44 = afStack_58[3];
    fStack_40 = fStack_48;
    fStack_3c = afStack_58[1];
    fVar11 = (float)((ulonglong)*(undefined8 *)(afStack_58 + lVar2 * 2) >> 0x20);
    fVar12 = (float)*(undefined8 *)(afStack_58 + lVar2 * 2);
    fVar14 = (float)((ulonglong)*param_1 >> 0x20);
    fVar9 = (float)((uint)fVar14 ^ _DAT_1429258f0);
    fVar8 = (float)*param_1;
    if (param_2 == 4) {
      param_2 = 0;
    }
    fVar6 = (float)((ulonglong)*(undefined8 *)(afStack_58 + param_2 * 2) >> 0x20);
    fVar3 = (float)*(undefined8 *)(afStack_58 + param_2 * 2);
    fVar4 = (float)param_1[1];
    fVar10 = fVar11 * fVar9 + fVar12 * fVar8 + fVar4;
    fVar7 = (float)((ulonglong)param_1[1] >> 0x20);
    fVar11 = fVar11 * fVar8 + fVar12 * fVar14 + fVar7;
    fVar13 = (fVar6 * fVar9 + fVar3 * fVar8 + fVar4) - fVar10;
    fVar14 = (fVar6 * fVar8 + fVar3 * fVar14 + fVar7) - fVar11;
    fVar12 = fVar14 * fVar14 + fVar13 * fVar13;
    if (fVar12 <= _DAT_142a7baa0) {
      fVar12 = _DAT_142a7baa0;
    }
    fVar12 = ((param_3 - fVar11) * fVar14 + (in_XMM1_Da - fVar10) * fVar13) / fVar12;
    fVar11 = 0.0;
    if (0.0 <= fVar12) {
      fVar11 = fVar12;
    }
    fVar12 = _DAT_142925904;
    if (fVar11 <= _DAT_142925904) {
      fVar12 = fVar11;
    }
    return CONCAT44(fVar7,(fVar4 + (float)((ulonglong)*(undefined8 *)(afStack_58 + param_2 * 2) >>
                                          0x20) * fVar9 +
                                   (float)*(undefined8 *)(afStack_58 + param_2 * 2) * fVar8) *
                          fVar12 + (fVar4 + (float)((ulonglong)
                                                    *(undefined8 *)(afStack_58 + lVar2 * 2) >> 0x20)
                                            * fVar9 + (float)*(undefined8 *)(afStack_58 + lVar2 * 2)
                                                      * fVar8) * (_DAT_142925904 - fVar12));
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  pcVar1 = (code *)swi(3);
  uVar5 = (*pcVar1)();
  return uVar5;
}

