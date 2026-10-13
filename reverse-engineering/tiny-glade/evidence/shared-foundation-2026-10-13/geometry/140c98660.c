utils::geometry::rectangle::Rectangle3d::extent

undefined4 * tg_math_140c98660(undefined4 *param_1,longlong param_2)

{
  float fVar1;
  float fVar2;
  undefined4 uVar3;
  
  fVar1 = *(float *)(param_2 + 0x1c);
  fVar2 = *(float *)(param_2 + 0x18);
  uVar3 = *(undefined4 *)(param_2 + 0x14);
  *param_1 = *(undefined4 *)(param_2 + 0x10);
  param_1[1] = fVar1 - fVar2;
  param_1[2] = uVar3;
  return param_1;
}

