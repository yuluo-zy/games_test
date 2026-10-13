utils::geometry::rectangle::Rectangle2d::get_closest_edge_id

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong thunk_FUN_140c9a9d0(undefined8 *param_1,float param_2,float param_3)

{
  code *pcVar1;
  bool bVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  bool bVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float afStack_d8 [4];
  float fStack_c8;
  float fStack_c4;
  float fStack_c0;
  float fStack_bc;
  
  fStack_c8 = (float)param_1[2];
  afStack_d8[3] = (float)((ulonglong)param_1[2] >> 0x20);
  if (fStack_c8 * afStack_d8[3] <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
    pcVar1 = (code *)swi(3);
    uVar4 = (*pcVar1)();
    return uVar4;
  }
  afStack_d8[1] = afStack_d8[3] * _DAT_142aa5d60;
  afStack_d8[0] = fStack_c8 * _UNK_142aa5d64;
  afStack_d8[3] = afStack_d8[3] * _UNK_142aa5d68;
  fStack_c8 = fStack_c8 * _UNK_142aa5d6c;
  fVar10 = (float)((ulonglong)*param_1 >> 0x20);
  fVar11 = (float)*param_1;
  bVar2 = false;
  uVar8 = 0;
  uVar4 = 0;
  uVar7 = 0;
  fVar12 = _DAT_14295b630;
  do {
    bVar9 = 3 < uVar7;
    if (bVar9) {
      uVar5 = 4;
      if (!bVar2) {
        return uVar4;
      }
LAB_140c9aad0:
      uVar6 = 0;
      uVar3 = uVar8;
      if (uVar8 != 4) {
        uVar6 = uVar8;
      }
    }
    else {
      uVar5 = uVar7 + 1;
      if (bVar2) goto LAB_140c9aad0;
      uVar5 = uVar7 + 2;
      bVar9 = uVar7 == 3;
      if (bVar9) {
        uVar5 = 4;
      }
      bVar2 = true;
      uVar6 = uVar7;
      uVar3 = uVar7;
      uVar7 = uVar7 + 1;
    }
    uVar8 = uVar7;
    afStack_d8[2] = afStack_d8[0];
    fStack_c4 = afStack_d8[3];
    fStack_c0 = fStack_c8;
    fStack_bc = afStack_d8[1];
    fVar17 = (float)*(undefined8 *)(afStack_d8 + uVar6 * 2);
    fVar14 = (float)((ulonglong)*(undefined8 *)(afStack_d8 + uVar6 * 2) >> 0x20);
    uVar7 = 0;
    if (uVar8 != 4) {
      uVar7 = uVar8;
    }
    fVar18 = (float)*(undefined8 *)(afStack_d8 + uVar7 * 2);
    fVar19 = (float)((ulonglong)*(undefined8 *)(afStack_d8 + uVar7 * 2) >> 0x20);
    fVar15 = (float)param_1[1];
    fVar13 = fVar14 * (float)((uint)fVar10 ^ _UNK_1429258f8) + fVar17 * fVar11 + fVar15;
    fVar16 = (float)((ulonglong)param_1[1] >> 0x20);
    fVar14 = fVar14 * fVar11 + fVar17 * fVar10 + fVar16;
    fVar15 = fVar19 * (float)((uint)fVar10 ^ _UNK_1429258f8) + fVar18 * fVar11 + fVar15;
    fVar16 = fVar19 * fVar11 + fVar18 * fVar10 + fVar16;
    fVar18 = fVar15 - fVar13;
    fVar19 = fVar16 - fVar14;
    fVar17 = fVar19 * fVar19 + fVar18 * fVar18;
    if (fVar17 <= _DAT_142a7baa0) {
      fVar17 = _DAT_142a7baa0;
    }
    fVar17 = ((param_3 - fVar14) * fVar19 + (param_2 - fVar13) * fVar18) / fVar17;
    fVar18 = 0.0;
    if (0.0 <= fVar17) {
      fVar18 = fVar17;
    }
    fVar17 = _DAT_142925904;
    if (fVar18 <= _DAT_142925904) {
      fVar17 = fVar18;
    }
    fVar15 = param_2 - (fVar17 * fVar15 + (_DAT_142925904 - fVar17) * fVar13);
    fVar17 = param_3 - (fVar17 * fVar16 + (_DAT_142925904 - fVar17) * fVar14);
    fVar17 = SQRT(fVar17 * fVar17 + fVar15 * fVar15);
    if (fVar12 <= fVar17) {
      uVar3 = uVar4;
      fVar17 = fVar12;
    }
    fVar12 = fVar17;
    uVar4 = uVar3;
    uVar7 = uVar5;
    if (bVar9) {
      return uVar3;
    }
  } while( true );
}

