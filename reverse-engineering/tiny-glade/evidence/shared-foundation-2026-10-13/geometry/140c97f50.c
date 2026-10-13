utils::geometry::circle::Circle2d::as_curve2_range

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong *
FUN_140c97f50(longlong *param_1,longlong param_2,float param_3,float param_4,float param_5)

{
  code *pcVar1;
  ulonglong uVar2;
  longlong *plVar3;
  ulonglong uVar4;
  float fVar5;
  longlong lStack_90;
  undefined8 uStack_88;
  float fStack_80;
  float fStack_7c;
  longlong lStack_78;
  ulonglong uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  float fStack_50;
  float fStack_4c;
  
  fVar5 = (float)func_0x000142923e60((param_5 - param_4) *
                                     ((*(float *)(param_2 + 8) * _DAT_142934674) / param_3));
  uVar4 = (longlong)(fVar5 - _DAT_14295a0dc) & (longlong)fVar5 >> 0x3f | (longlong)fVar5;
  if (fVar5 < 0.0) {
    uVar4 = 0;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar5 <= _DAT_14295a0e0) {
    uVar2 = uVar4;
  }
  uStack_88 = 3;
  if (3 < uVar2) {
    uStack_88 = uVar2;
  }
  fStack_7c = param_5;
  lStack_78 = 0;
  lStack_90 = param_2;
  fStack_80 = param_4;
  uStack_70 = uStack_88;
  FUN_141942120(&uStack_58,&lStack_90,&UNK_142b2e478);
  FUN_1419401b0(&lStack_90,&uStack_58,0);
  if (!SBORROW8(0,lStack_90)) {
    param_1[6] = lStack_60;
    param_1[4] = uStack_70;
    param_1[5] = lStack_68;
    param_1[2] = CONCAT44(fStack_7c,fStack_80);
    param_1[3] = lStack_78;
    *param_1 = lStack_90;
    param_1[1] = uStack_88;
    return param_1;
  }
  uStack_58 = (undefined4)uStack_88;
  uStack_54 = uStack_88._4_4_;
  fStack_50 = fStack_80;
  fStack_4c = fStack_7c;
  FUN_1428d9760(&UNK_142e714b0,0x2b,&uStack_58,&UNK_142e71490,&UNK_142b2e500);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

