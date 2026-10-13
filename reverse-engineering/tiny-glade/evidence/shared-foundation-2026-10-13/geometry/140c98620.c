utils::geometry::rectangle::Rectangle3d::expand

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tg_math_140c98620(longlong param_1,float *param_2)

{
  float fVar1;
  float fVar2;
  
  fVar2 = param_2[1] * _DAT_142925870;
  fVar1 = param_2[2];
  *(float *)(param_1 + 0x10) = *param_2 + *(float *)(param_1 + 0x10);
  *(float *)(param_1 + 0x14) = fVar1 + *(float *)(param_1 + 0x14);
  *(float *)(param_1 + 0x18) = *(float *)(param_1 + 0x18) - fVar2;
  *(float *)(param_1 + 0x1c) = fVar2 + *(float *)(param_1 + 0x1c);
  return;
}

