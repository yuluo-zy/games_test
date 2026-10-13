utils::geometry::aabb::Aabb2::contains

bool tg_math_141240140(float *param_1,float param_2,float param_3)

{
  bool bVar1;
  
  bVar1 = false;
  if (((*param_1 <= param_2) && (param_2 <= param_1[2])) && (param_1[1] <= param_3)) {
    bVar1 = param_3 <= param_1[3];
  }
  return bVar1;
}

