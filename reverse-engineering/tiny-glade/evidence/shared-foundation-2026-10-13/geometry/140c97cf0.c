utils::geometry::circle::Circle2d::as_curve2

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140c97cf0(longlong *param_1,longlong param_2,float param_3)

{
  code *pcVar1;
  ulonglong uVar2;
  longlong *plVar3;
  ulonglong uVar4;
  float fVar5;
  longlong lStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  ulonglong uStack_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  
  fVar5 = (float)func_0x000142923e60((*(float *)(param_2 + 8) * _DAT_142934674) / param_3);
  uVar4 = (longlong)(fVar5 - _DAT_14295a0dc) & (longlong)fVar5 >> 0x3f | (longlong)fVar5;
  if (fVar5 < 0.0) {
    uVar4 = 0;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar5 <= _DAT_14295a0e0) {
    uVar2 = uVar4;
  }
  uStack_68 = 3;
  if (3 < uVar2) {
    uStack_68 = uVar2;
  }
  uStack_60 = 0;
  lStack_70 = param_2;
  uStack_58 = uStack_68;
  FUN_141941fc0(&uStack_38,&lStack_70,&UNK_142b2e478);
  FUN_1419401b0(&lStack_70,&uStack_38,0);
  if (!SBORROW8(0,lStack_70)) {
    param_1[6] = lStack_40;
    param_1[4] = lStack_50;
    param_1[5] = lStack_48;
    param_1[2] = uStack_60;
    param_1[3] = uStack_58;
    *param_1 = lStack_70;
    param_1[1] = uStack_68;
    return param_1;
  }
  uStack_38 = (undefined4)uStack_68;
  uStack_34 = uStack_68._4_4_;
  uStack_30 = (undefined4)uStack_60;
  uStack_2c = uStack_60._4_4_;
  FUN_1428d9760(&UNK_142e714b0,0x2b,&uStack_38,&UNK_142e71490,&UNK_142b2e4d0);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

