utils::geometry::circle::Circle2d::get_tangent_at_u

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c97660(longlong param_1,float param_2)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  float fVar4;
  
  fVar1 = _DAT_142934674;
  fVar4 = (_DAT_142925904 - param_2) * _DAT_142934674;
  fVar2 = (float)func_0x000142923f10(*(undefined4 *)(param_1 + 0xc),_DAT_142934674);
  fVar4 = (float)(~-(uint)(fVar2 < 0.0) & (uint)fVar2 | (uint)(fVar1 + fVar2) & -(uint)(fVar2 < 0.0)
                 ) + fVar4;
  func_0x000142923e80(fVar4);
  uVar3 = func_0x000142923fd0(fVar4);
  return CONCAT44((uint)((ulonglong)uVar3 >> 0x20) ^ _UNK_1429258f4,(uint)uVar3 ^ _DAT_1429258f0);
}

