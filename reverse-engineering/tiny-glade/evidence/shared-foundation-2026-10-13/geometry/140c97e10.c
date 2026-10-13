utils::geometry::circle::Circle2d::as_curve3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c97e10(longlong *param_1,longlong param_2,float param_3,undefined4 param_4)

{
  code *pcVar1;
  ulonglong uVar2;
  longlong *plVar3;
  ulonglong uVar4;
  float fVar5;
  longlong lStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  ulonglong uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong lStack_58;
  longlong lStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  
  fVar5 = (float)func_0x000142923e60((*(float *)(param_2 + 8) * _DAT_142934674) / param_3);
  uVar4 = (longlong)(fVar5 - _DAT_14295a0dc) & (longlong)fVar5 >> 0x3f | (longlong)fVar5;
  if (fVar5 < 0.0) {
    uVar4 = 0;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar5 <= _DAT_14295a0e0) {
    uVar2 = uVar4;
  }
  uStack_78 = 3;
  if (3 < uVar2) {
    uStack_78 = uVar2;
  }
  uStack_70 = 0;
  lStack_80 = param_2;
  uStack_68 = uStack_78;
  uStack_60 = param_4;
  FUN_141941e60(&uStack_48,&lStack_80,&UNK_142b2e478);
  FUN_141940470(&lStack_80,&uStack_48,0);
  if (!SBORROW8(0,lStack_80)) {
    param_1[6] = lStack_50;
    param_1[4] = CONCAT44(uStack_5c,uStack_60);
    param_1[5] = lStack_58;
    param_1[2] = uStack_70;
    param_1[3] = uStack_68;
    *param_1 = lStack_80;
    param_1[1] = uStack_78;
    return param_1;
  }
  uStack_48 = (undefined4)uStack_78;
  uStack_44 = uStack_78._4_4_;
  uStack_40 = (undefined4)uStack_70;
  uStack_3c = uStack_70._4_4_;
  FUN_1428d9760(&UNK_142e714b0,0x2b,&uStack_48,&UNK_142e71490,&UNK_142b2e4e8);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

