utils::geometry::aabb::Aabb2::area

undefined8 tg_math_141240120(undefined8 *param_1)

{
  float fVar1;
  
  fVar1 = (float)((ulonglong)param_1[1] >> 0x20) - (float)((ulonglong)*param_1 >> 0x20);
  return CONCAT44(fVar1,fVar1 * ((float)param_1[1] - (float)*param_1));
}

