/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
float * FUN_141941510(float *param_1,longlong *param_2,undefined *param_3)
{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  code *pcVar7;
  undefined *puVar8;
  float *pfVar9;
  undefined *puVar10;
  undefined *puVar11;
  float fVar12;
  float fVar13;
  puVar8 = param_3 + 1;
  puVar11 = (undefined *)param_2[1];
  if (puVar8 < puVar11) {
    if (puVar11 <= param_3) goto LAB_141941615;
    puVar11 = (undefined *)param_2[3];
    puVar10 = param_3;
    if (puVar11 <= param_3) goto LAB_14194162a;
    puVar11 = (undefined *)param_2[5];
    if (puVar11 <= param_3) goto LAB_14194163f;
    puVar11 = (undefined *)param_2[7];
    if (puVar11 <= puVar8) goto LAB_141941654;
    if (param_3 < puVar11) {
      puVar8 = (undefined *)param_2[9];
      if (param_3 < puVar8) {
        fVar1 = *(float *)(*param_2 + (longlong)param_3 * 4);
        fVar2 = *(float *)(*param_2 + 4 + (longlong)param_3 * 4);
        fVar3 = *(float *)(param_2[4] + (longlong)param_3 * 4);
        fVar4 = *(float *)(param_2[6] + 4 + (longlong)param_3 * 4);
        fVar5 = *(float *)(param_2[6] + (longlong)param_3 * 4);
        fVar12 = fVar3 * _DAT_1429cd078;
        fVar6 = *(float *)(param_2[2] + (longlong)param_3 * 4);
        fVar13 = ((fVar5 + fVar5 + fVar4) * fVar3) / _DAT_1429cd078;
        fVar3 = *(float *)(param_2[8] + (longlong)param_3 * 4);
        *param_1 = fVar1;
        param_1[1] = (fVar2 - fVar1) * fVar6 - fVar13;
        param_1[2] = fVar5;
        param_1[3] = (fVar4 - fVar5) / fVar12;
        param_1[4] = fVar3;
        return param_1;
      }
      goto LAB_14194167b;
    }
  }
  else {
    param_3 = &UNK_142e71998;
    FUN_1428d9518(puVar8,puVar11,&UNK_142e71998);
LAB_141941615:
    puVar10 = &UNK_142e719b0;
    FUN_1428d9518(param_3,puVar11,&UNK_142e719b0);
LAB_14194162a:
    param_3 = &UNK_142e719c8;
    FUN_1428d9518(puVar10,puVar11,&UNK_142e719c8);
LAB_14194163f:
    puVar8 = (undefined *)FUN_1428d9518(param_3,puVar11,&UNK_142e719e0);
LAB_141941654:
    puVar10 = &UNK_142e719f8;
    FUN_1428d9518(puVar8,puVar11,&UNK_142e719f8);
  }
  param_3 = &UNK_142e71a10;
  puVar8 = (undefined *)FUN_1428d9518(puVar10,puVar11,&UNK_142e71a10);
LAB_14194167b:
  FUN_1428d9518(param_3,puVar8,&UNK_142e71a28);
  pcVar7 = (code *)swi(3);
  pfVar9 = (float *)(*pcVar7)();
  return pfVar9;
}