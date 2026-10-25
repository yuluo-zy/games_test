utils::geometry::aabb::Aabb2::from_center_dims

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_14123fff0(float *param_1,float param_2,float param_3,float param_4,float param_5)

{
  code *pcVar1;
  float *pfVar2;
  
  if (0.0 <= SQRT(param_5 * param_5 + param_4 * param_4)) {
    param_4 = param_4 * _DAT_142925870;
    param_5 = param_5 * _DAT_142925870;
    *param_1 = param_2 - param_4;
    param_1[1] = param_3 - param_5;
    param_1[2] = param_2 + param_4;
    param_1[3] = param_3 + param_5;
    return param_1;
  }
  FUN_1428d9430(&UNK_142c1f4f8,0x26,&UNK_142c1f540);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

