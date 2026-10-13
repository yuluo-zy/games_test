utils::geometry::aabb::Aabb3::center

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * tg_math_141240420(undefined8 *param_1,undefined8 *param_2)

{
  float fVar1;
  
  fVar1 = (*(float *)(param_2 + 1) + *(float *)((longlong)param_2 + 0x14)) * _DAT_142925870;
  *param_1 = CONCAT44(((float)((ulonglong)*(undefined8 *)((longlong)param_2 + 0xc) >> 0x20) +
                      (float)((ulonglong)*param_2 >> 0x20)) * _UNK_1429258e4,
                      ((float)*(undefined8 *)((longlong)param_2 + 0xc) + (float)*param_2) *
                      _DAT_1429258e0);
  *(float *)(param_1 + 1) = fVar1;
  return param_1;
}

