utils::geometry::rectangle::Rectangle3d::new

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98380(float *param_1,float *param_2,float *param_3,float param_4,float param_5,
                     undefined8 param_6)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  code *pcVar6;
  float *pfVar7;
  float fVar8;
  
  fVar4 = *param_3;
  if ((uint)ABS(fVar4) < 0x7f800000) {
    fVar5 = param_3[2];
    if ((uint)ABS(fVar5) < 0x7f800000) {
      if ((float)((uint)(param_5 * param_5 + param_4 * param_4 + _DAT_14295a0e4) & _DAT_14292e980)
          <= _DAT_142b2e538) {
        fVar1 = *param_2;
        fVar2 = param_2[2];
        fVar3 = param_2[1];
        fVar8 = param_3[1] * _DAT_142925870;
        *param_1 = param_4;
        param_1[1] = param_5;
        param_1[2] = fVar1;
        param_1[3] = fVar2;
        param_1[4] = fVar4;
        param_1[5] = fVar5;
        param_1[6] = fVar3 - fVar8;
        param_1[7] = fVar8 + fVar3;
        return param_1;
      }
      goto LAB_140c9849f;
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e5f8,0x23,param_6);
  }
  FUN_1428d9430(&UNK_142b2e61b,0x24);
LAB_140c9849f:
  FUN_1428d9430(&UNK_142b2e63f,0x2d);
  pcVar6 = (code *)swi(3);
  pfVar7 = (float *)(*pcVar6)();
  return pfVar7;
}

