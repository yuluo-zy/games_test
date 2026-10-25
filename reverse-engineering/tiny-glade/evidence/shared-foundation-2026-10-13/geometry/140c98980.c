utils::geometry::rectangle::Rectangle2d::new

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98980(float *param_1,float param_2,float param_3,float param_4,float param_5,
                     float param_6,float param_7,undefined8 param_8)

{
  code *pcVar1;
  float *pfVar2;
  
  if ((uint)ABS(param_2) < 0x7f800000) {
    if ((uint)ABS(param_3) < 0x7f800000) {
      if ((float)((uint)(param_5 * param_5 + param_4 * param_4 + _DAT_14295a0e4) & _DAT_14292e980)
          <= _DAT_142b2e538) {
        *param_1 = param_4;
        param_1[1] = param_5;
        param_1[4] = param_2;
        param_1[5] = param_3;
        param_1[2] = param_6;
        param_1[3] = param_7;
        return param_1;
      }
      goto LAB_140c98a30;
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e5f8,0x23,param_8);
  }
  FUN_1428d9430(&UNK_142b2e61b,0x24);
LAB_140c98a30:
  FUN_1428d9430(&UNK_142b2e63f,0x2d);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

