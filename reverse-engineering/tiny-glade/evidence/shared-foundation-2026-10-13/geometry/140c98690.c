utils::geometry::rectangle::Rectangle3d::points

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98690(float *param_1,undefined8 *param_2)

{
  float fVar1;
  code *pcVar2;
  float *pfVar3;
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
  float fVar16;
  
  fVar8 = *(float *)(param_2 + 2);
  fVar4 = *(float *)((longlong)param_2 + 0x14);
  if (0.0 < fVar8 * fVar4) {
    fVar1 = *(float *)(param_2 + 3);
    fVar14 = *(float *)(param_2 + 1);
    fVar6 = (float)((ulonglong)*param_2 >> 0x20);
    fVar7 = (float)((uint)fVar6 ^ _DAT_1429258f0);
    fVar5 = (float)*param_2;
    fVar9 = *(float *)((longlong)param_2 + 0xc);
    fVar11 = fVar8 * _DAT_14295a0e8;
    fVar15 = _DAT_14295a0e8 * fVar4;
    fVar4 = fVar4 * _DAT_142925870;
    fVar8 = fVar8 * _DAT_142925870;
    fVar16 = fVar15 * fVar7 + fVar8 * fVar5 + fVar14;
    fVar10 = fVar14 + fVar15 * fVar7 + fVar11 * fVar5;
    fVar13 = fVar15 * fVar5 + fVar11 * fVar6 + fVar9;
    fVar12 = fVar14 + fVar4 * fVar7 + fVar11 * fVar5;
    fVar11 = fVar4 * fVar5 + fVar11 * fVar6 + fVar9;
    fVar14 = fVar14 + fVar4 * fVar7 + fVar8 * fVar5;
    fVar4 = fVar4 * fVar5 + fVar8 * fVar6 + fVar9;
    fVar9 = fVar15 * fVar5 + fVar8 * fVar6 + fVar9;
    fVar8 = *(float *)((longlong)param_2 + 0x1c);
    *param_1 = fVar10;
    param_1[1] = fVar1;
    param_1[2] = fVar13;
    param_1[3] = fVar12;
    param_1[4] = fVar1;
    param_1[5] = fVar11;
    param_1[6] = fVar14;
    param_1[7] = fVar1;
    param_1[8] = fVar4;
    param_1[9] = fVar16;
    param_1[10] = fVar1;
    param_1[0xb] = fVar9;
    param_1[0xc] = fVar10;
    param_1[0xd] = fVar8;
    param_1[0xe] = fVar13;
    param_1[0xf] = fVar12;
    param_1[0x10] = fVar8;
    param_1[0x11] = fVar11;
    param_1[0x12] = fVar14;
    param_1[0x13] = fVar8;
    param_1[0x14] = fVar4;
    param_1[0x15] = fVar16;
    param_1[0x16] = fVar8;
    param_1[0x17] = fVar9;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6e8);
  pcVar2 = (code *)swi(3);
  pfVar3 = (float *)(*pcVar2)();
  return pfVar3;
}

