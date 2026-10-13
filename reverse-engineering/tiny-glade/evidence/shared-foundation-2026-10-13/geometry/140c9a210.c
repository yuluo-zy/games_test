utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c9a210(float *param_1,undefined8 *param_2,ulonglong param_3)

{
  code *pcVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
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
  
  fVar10 = *(float *)(param_2 + 2);
  fVar9 = *(float *)((longlong)param_2 + 0x14);
  if (fVar10 * fVar9 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6d0);
  }
  else if (param_3 < 4) {
    fVar13 = (float)*param_2;
    fVar14 = (float)((ulonglong)*param_2 >> 0x20);
    fVar3 = (float)((uint)fVar14 ^ _DAT_1429258f0);
    fVar15 = fVar10 * _DAT_142925870;
    fVar4 = _DAT_142925870 * fVar9;
    fVar11 = (float)param_2[1];
    fVar7 = fVar4 * fVar3 + fVar15 * fVar13 + fVar11;
    fVar12 = (float)((ulonglong)param_2[1] >> 0x20);
    fVar8 = fVar4 * fVar13 + fVar15 * fVar14 + fVar12;
    fVar10 = fVar10 * _DAT_14295a0e8;
    fVar5 = fVar4 * fVar3 + fVar10 * fVar13 + fVar11;
    fVar6 = fVar4 * fVar13 + fVar10 * fVar14 + fVar12;
    fVar9 = fVar9 * _DAT_14295a0e8;
    fVar4 = fVar9 * fVar3 + fVar10 * fVar13 + fVar11;
    fVar10 = fVar9 * fVar13 + fVar10 * fVar14 + fVar12;
    fVar11 = fVar9 * fVar3 + fVar15 * fVar13 + fVar11;
    fVar12 = fVar9 * fVar13 + fVar15 * fVar14 + fVar12;
    switch(param_3) {
    case 0:
      fVar11 = fVar11 - fVar4;
      fVar12 = fVar12 - fVar10;
      fVar9 = _DAT_142925904 / SQRT(fVar12 * fVar12 + fVar11 * fVar11);
      fVar3 = fVar9 * fVar11;
      fVar9 = fVar9 * fVar12;
      fVar4 = fVar5 - fVar4;
      fVar10 = fVar6 - fVar10;
      fVar5 = fVar10 * fVar10 + fVar4 * fVar4;
      break;
    case 1:
      fVar4 = fVar4 - fVar5;
      fVar10 = fVar10 - fVar6;
      fVar9 = _DAT_142925904 / SQRT(fVar10 * fVar10 + fVar4 * fVar4);
      fVar3 = fVar9 * fVar4;
      fVar9 = fVar9 * fVar10;
      fVar4 = fVar7 - fVar5;
      fVar10 = fVar8 - fVar6;
      fVar5 = fVar10 * fVar10 + fVar4 * fVar4;
      break;
    case 2:
      fVar5 = fVar5 - fVar7;
      fVar6 = fVar6 - fVar8;
      fVar9 = _DAT_142925904 / SQRT(fVar6 * fVar6 + fVar5 * fVar5);
      fVar3 = fVar9 * fVar5;
      fVar9 = fVar9 * fVar6;
      fVar4 = fVar11 - fVar7;
      fVar10 = fVar12 - fVar8;
      fVar5 = fVar10 * fVar10 + fVar4 * fVar4;
      break;
    case 3:
      fVar7 = fVar7 - fVar11;
      fVar8 = fVar8 - fVar12;
      fVar9 = _DAT_142925904 / SQRT(fVar8 * fVar8 + fVar7 * fVar7);
      fVar3 = fVar9 * fVar7;
      fVar9 = fVar9 * fVar8;
      fVar4 = fVar4 - fVar11;
      fVar10 = fVar10 - fVar12;
      fVar5 = fVar10 * fVar10 + fVar4 * fVar4;
    }
    fVar5 = _DAT_142925904 / SQRT(fVar5);
    *param_1 = fVar3;
    param_1[1] = fVar9;
    param_1[2] = fVar5 * fVar4;
    param_1[3] = fVar5 * fVar10;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e820,0x28,&UNK_142b2e880);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

