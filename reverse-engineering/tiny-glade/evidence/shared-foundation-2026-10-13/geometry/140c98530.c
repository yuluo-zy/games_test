utils::geometry::rectangle::Rectangle3d::set_size

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tg_math_140c98530(longlong param_1,undefined4 *param_2)

{
  float fVar1;
  float fVar2;
  
  *(undefined4 *)(param_1 + 0x10) = *param_2;
  *(undefined4 *)(param_1 + 0x14) = param_2[2];
  fVar1 = (*(float *)(param_1 + 0x18) + *(float *)(param_1 + 0x1c)) * _DAT_142925870;
  fVar2 = _DAT_142925870 * (float)param_2[1];
  *(float *)(param_1 + 0x18) = fVar1 - fVar2;
  *(float *)(param_1 + 0x1c) = fVar2 + fVar1;
  return;
}

