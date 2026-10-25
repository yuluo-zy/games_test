
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 *
FUN_1416f34a0(undefined4 *param_1,longlong param_2,ulonglong param_3,undefined8 *param_4,
             longlong param_5,float param_6,float param_7)

{
  undefined8 *puVar1;
  code *pcVar2;
  undefined4 uVar3;
  ulonglong uVar4;
  undefined4 *puVar5;
  float *pfVar6;
  float *pfVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  
  param_5 = param_5 * 0x18;
  uVar4 = 0xffffffffffffffff;
  pfVar7 = (float *)(param_2 + -4);
  do {
    pfVar6 = pfVar7;
    if (param_5 == 0) {
      uVar3 = 0;
      goto LAB_1416f35f3;
    }
    puVar1 = param_4 + 3;
    fVar12 = (float)*param_4;
    fVar9 = (float)param_4[1] - fVar12;
    fVar8 = (float)((ulonglong)*param_4 >> 0x20);
    fVar10 = (float)((ulonglong)param_4[1] >> 0x20) - fVar8;
    fVar11 = (float)param_4[2] - fVar12;
    fVar13 = (float)((ulonglong)param_4[2] >> 0x20) - fVar8;
    fVar14 = _DAT_142925904 / (fVar9 * fVar13 - fVar10 * fVar11);
    fVar11 = fVar14 * (fVar13 * (param_6 - fVar12) - fVar11 * (param_7 - fVar8));
    fVar14 = fVar14 * ((param_7 - fVar8) * fVar9 - (param_6 - fVar12) * fVar10);
    param_5 = param_5 + -0x18;
    uVar4 = uVar4 + 1;
    pfVar7 = pfVar6 + 9;
    param_4 = puVar1;
  } while (((fVar14 < 0.0) || (fVar11 < 0.0)) ||
          (fVar12 = (_DAT_142925904 - fVar11) - fVar14, fVar12 < 0.0));
  if (param_3 <= uVar4) {
    FUN_1428d9518(uVar4,param_3,&UNK_142c906c8);
    pcVar2 = (code *)swi(3);
    puVar5 = (undefined4 *)(*pcVar2)();
    return puVar5;
  }
  fVar8 = pfVar6[3];
  fVar9 = pfVar6[6];
  fVar10 = *pfVar7;
  *(ulonglong *)(param_1 + 1) =
       CONCAT44((float)((ulonglong)*(undefined8 *)(pfVar6 + 7) >> 0x20) * fVar14 +
                fVar11 * (float)((ulonglong)*(undefined8 *)(pfVar6 + 4) >> 0x20) +
                (float)((ulonglong)*(undefined8 *)(pfVar6 + 1) >> 0x20) * fVar12,
                (float)*(undefined8 *)(pfVar6 + 7) * fVar14 +
                fVar11 * (float)*(undefined8 *)(pfVar6 + 4) +
                (float)*(undefined8 *)(pfVar6 + 1) * fVar12);
  param_1[3] = fVar10 * fVar14 + fVar9 * fVar11 + fVar12 * fVar8;
  uVar3 = 1;
LAB_1416f35f3:
  *param_1 = uVar3;
  return param_1;
}

