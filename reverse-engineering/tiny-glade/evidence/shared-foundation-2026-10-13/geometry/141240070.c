utils::geometry::aabb::Aabb2::from_center_radius

float * FUN_141240070(float *param_1,float param_2,float param_3,float param_4)

{
  code *pcVar1;
  float *pfVar2;
  
  if (0.0 <= param_4) {
    *param_1 = param_2 - param_4;
    param_1[1] = param_3 - param_4;
    param_1[2] = param_4 + param_2;
    param_1[3] = param_4 + param_3;
    return param_1;
  }
  FUN_1428d9430(&UNK_142c1f558,0x1f,&UNK_142c1f578);
  pcVar1 = (code *)swi(3);
  pfVar2 = (float *)(*pcVar1)();
  return pfVar2;
}

