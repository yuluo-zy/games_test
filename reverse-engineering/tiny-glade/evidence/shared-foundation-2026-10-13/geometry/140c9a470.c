utils::geometry::rectangle::Rectangle2d::get_edge_pts

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c9a470(float *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 uVar1;
  code *pcVar3;
  float *pfVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  undefined8 uVar2;
  
  fVar5 = *(float *)(param_2 + 2);
  fVar12 = *(float *)((longlong)param_2 + 0x14);
  if (fVar5 * fVar12 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6d0);
  }
  else if (param_3 < 4) {
    fVar17 = (float)((ulonglong)*param_2 >> 0x20);
    fVar6 = (float)((uint)fVar17 ^ _DAT_1429258f0);
    fVar16 = (float)*param_2;
    fVar7 = fVar5 * _DAT_142925870;
    fVar18 = _DAT_142925870 * fVar12;
    uVar1 = param_2[1];
    uVar2 = param_2[1];
    fVar5 = fVar5 * _DAT_14295a0e8;
    fVar8 = (float)uVar1;
    fVar13 = fVar5 * fVar16 + fVar18 * fVar6 + fVar8;
    fVar9 = (float)((ulonglong)uVar1 >> 0x20);
    fVar14 = fVar5 * fVar17 + fVar18 * fVar16 + fVar9;
    fVar10 = (float)uVar2;
    fVar15 = fVar7 * fVar16 + fVar18 * fVar6 + fVar10;
    fVar11 = (float)((ulonglong)uVar2 >> 0x20);
    fVar18 = fVar7 * fVar17 + fVar18 * fVar16 + fVar11;
    fVar12 = fVar12 * _DAT_14295a0e8;
    fVar8 = fVar7 * fVar16 + fVar12 * fVar6 + fVar8;
    fVar9 = fVar7 * fVar17 + fVar12 * fVar16 + fVar9;
    fVar10 = fVar5 * fVar16 + fVar12 * fVar6 + fVar10;
    fVar11 = fVar5 * fVar17 + fVar12 * fVar16 + fVar11;
    switch(param_3) {
    case 0:
      fVar8 = fVar10;
      fVar9 = fVar11;
      fVar10 = fVar13;
      fVar11 = fVar14;
    case 3:
      fVar13 = fVar8;
      fVar14 = fVar9;
      fVar15 = fVar10;
      fVar18 = fVar11;
      break;
    case 2:
      fVar13 = fVar15;
      fVar14 = fVar18;
      fVar15 = fVar8;
      fVar18 = fVar9;
    }
    *param_1 = fVar13;
    param_1[1] = fVar14;
    param_1[2] = fVar15;
    param_1[3] = fVar18;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e820,0x28,&UNK_142b2e898);
  pcVar3 = (code *)swi(3);
  pfVar4 = (float *)(*pcVar3)();
  return pfVar4;
}

