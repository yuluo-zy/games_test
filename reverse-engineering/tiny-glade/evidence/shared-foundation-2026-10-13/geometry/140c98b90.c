utils::geometry::rectangle::Rectangle2d::as_points3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98b90(float *param_1,undefined8 *param_2,float param_3)

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
  
  fVar4 = *(float *)(param_2 + 2);
  fVar7 = *(float *)((longlong)param_2 + 0x14);
  if (0.0 < fVar4 * fVar7) {
    fVar6 = (float)((ulonglong)*param_2 >> 0x20);
    fVar3 = (float)((uint)fVar6 ^ _DAT_1429258f0);
    fVar5 = (float)*param_2;
    fVar11 = fVar4 * _DAT_14295a0e8;
    fVar8 = _DAT_14295a0e8 * fVar7;
    fVar7 = fVar7 * _DAT_142925870;
    fVar4 = fVar4 * _DAT_142925870;
    param_1[1] = param_3;
    fVar9 = (float)param_2[1];
    fVar10 = (float)((ulonglong)param_2[1] >> 0x20);
    *param_1 = fVar9 + fVar8 * fVar3 + fVar11 * fVar5;
    *(ulonglong *)(param_1 + 2) =
         CONCAT44(fVar7 * fVar3 + fVar11 * fVar5 + fVar9,fVar8 * fVar5 + fVar11 * fVar6 + fVar10);
    param_1[4] = param_3;
    *(ulonglong *)(param_1 + 5) =
         CONCAT44(fVar7 * fVar3 + fVar4 * fVar5 + fVar9,fVar7 * fVar5 + fVar11 * fVar6 + fVar10);
    param_1[7] = param_3;
    *(ulonglong *)(param_1 + 8) =
         CONCAT44(fVar8 * fVar3 + fVar4 * fVar5 + fVar9,fVar7 * fVar5 + fVar4 * fVar6 + fVar10);
    param_1[10] = param_3;
    param_1[0xb] = fVar10 + fVar8 * fVar5 + fVar4 * fVar6;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6e8);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

