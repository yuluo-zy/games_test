utils::geometry::aabb::Aabb2::intersects

uint tg_math_141240170(float *param_1,float *param_2)

{
  if (param_1[2] < *param_2) {
    return 0;
  }
  if (param_2[2] < *param_1) {
    return 0;
  }
  return -(uint)(param_1[1] <= param_2[3] && param_2[1] <= param_1[3]) & 0xffffff01;
}

