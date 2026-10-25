utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98a50(float *param_1,longlong param_2)

{
  code *pcVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  
  fVar6 = (float)*(undefined8 *)(param_2 + 0x10);
  fVar5 = (float)((ulonglong)*(undefined8 *)(param_2 + 0x10) >> 0x20);
  if (0.0 < fVar5 * fVar6) {
    fVar3 = fVar5 * _DAT_142aa5d60;
    fVar4 = fVar6 * _UNK_142aa5d64;
    fVar5 = fVar5 * _UNK_142aa5d68;
    fVar6 = fVar6 * _UNK_142aa5d6c;
    *param_1 = fVar4;
    param_1[1] = fVar3;
    param_1[2] = fVar4;
    param_1[3] = fVar5;
    param_1[4] = fVar6;
    *(ulonglong *)(param_1 + 5) = CONCAT44(fVar6,fVar5);
    param_1[7] = fVar3;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

