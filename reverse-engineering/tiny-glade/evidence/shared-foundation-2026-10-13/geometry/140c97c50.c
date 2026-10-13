utils::geometry::circle::Circle2d::as_points3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c97c50(longlong *param_1,longlong param_2,float param_3,undefined4 param_4)

{
  ulonglong uVar1;
  ulonglong uVar2;
  float fVar3;
  
  fVar3 = (float)func_0x000142923e60((*(float *)(param_2 + 8) * _DAT_142934674) / param_3);
  uVar1 = 0;
  if (0.0 <= fVar3) {
    uVar1 = (longlong)(fVar3 - _DAT_14295a0dc) & (longlong)fVar3 >> 0x3f | (longlong)fVar3;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar3 <= _DAT_14295a0e0) {
    uVar2 = uVar1;
  }
  uVar1 = 3;
  if (3 < uVar2) {
    uVar1 = uVar2;
  }
  *param_1 = param_2;
  param_1[1] = uVar1;
  param_1[2] = 0;
  param_1[3] = uVar1;
  *(undefined4 *)(param_1 + 4) = param_4;
  return param_1;
}

