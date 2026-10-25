utils::geometry::rectangle::Rectangle2d::edge_length

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c9ca50(undefined8 *param_1,ulonglong param_2)

{
  undefined8 uVar1;
  code *pcVar3;
  float fVar4;
  float fVar5;
  float fVar7;
  ulonglong uVar6;
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
  
  fVar9 = *(float *)(param_1 + 2);
  fVar15 = *(float *)((longlong)param_1 + 0x14);
  if (fVar9 * fVar15 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6d0);
  }
  else if (param_2 < 4) {
    fVar17 = (float)((ulonglong)*param_1 >> 0x20);
    fVar4 = (float)((uint)fVar17 ^ _DAT_1429258f0);
    fVar16 = (float)*param_1;
    fVar10 = fVar9 * _DAT_142925870;
    fVar18 = _DAT_142925870 * fVar15;
    uVar1 = param_1[1];
    uVar2 = param_1[1];
    fVar9 = fVar9 * _DAT_14295a0e8;
    fVar11 = (float)uVar1;
    fVar5 = fVar9 * fVar16 + fVar18 * fVar4 + fVar11;
    fVar12 = (float)((ulonglong)uVar1 >> 0x20);
    fVar7 = fVar9 * fVar17 + fVar18 * fVar16 + fVar12;
    fVar13 = (float)uVar2;
    fVar8 = fVar10 * fVar16 + fVar18 * fVar4 + fVar13;
    fVar14 = (float)((ulonglong)uVar2 >> 0x20);
    fVar18 = fVar10 * fVar17 + fVar18 * fVar16 + fVar14;
    fVar15 = fVar15 * _DAT_14295a0e8;
    fVar11 = fVar10 * fVar16 + fVar15 * fVar4 + fVar11;
    fVar12 = fVar10 * fVar17 + fVar15 * fVar16 + fVar12;
    fVar13 = fVar9 * fVar16 + fVar15 * fVar4 + fVar13;
    fVar14 = fVar9 * fVar17 + fVar15 * fVar16 + fVar14;
    switch(param_2) {
    case 0:
      fVar11 = fVar13;
      fVar12 = fVar14;
      fVar13 = fVar5;
      fVar14 = fVar7;
    case 3:
      fVar5 = fVar11;
      fVar7 = fVar12;
      fVar8 = fVar13;
      fVar18 = fVar14;
      break;
    case 2:
      fVar5 = fVar8;
      fVar7 = fVar18;
      fVar8 = fVar11;
      fVar18 = fVar12;
    }
    return (ulonglong)
           (uint)SQRT((fVar7 - fVar18) * (fVar7 - fVar18) + (fVar5 - fVar8) * (fVar5 - fVar8));
  }
  FUN_1428d9430(&UNK_142b2e820,0x28,&UNK_142b2e898);
  pcVar3 = (code *)swi(3);
  uVar6 = (*pcVar3)();
  return uVar6;
}

