
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float FUN_140c975a0(float *param_1,float param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  
  fVar3 = _DAT_142934674;
  fVar4 = (_DAT_142925904 - param_2) * _DAT_142934674;
  fVar1 = param_1[2];
  fVar2 = (float)func_0x000142923f10(param_1[3],_DAT_142934674);
  fVar4 = (float)(~-(uint)(fVar2 < 0.0) & (uint)fVar2 | (uint)(fVar3 + fVar2) & -(uint)(fVar2 < 0.0)
                 ) + fVar4;
  fVar3 = (float)func_0x000142923e80(fVar4);
  func_0x000142923fd0(fVar4);
  return fVar3 * fVar1 + *param_1;
}

