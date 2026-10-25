
longlong * FUN_140bda7b0(longlong *param_1)

{
  code *pcVar1;
  longlong lVar2;
  longlong *plVar3;
  undefined4 uStack_24;
  longlong lStack_20;
  undefined4 uStack_18;
  undefined4 uStack_14;
  longlong lStack_10;
  
  FUN_1418b09f0(&lStack_20);
  if (!SBORROW8(0,lStack_20)) {
    lVar2 = CONCAT44(uStack_14,uStack_18);
    *param_1 = lVar2;
    param_1[1] = lVar2;
    param_1[2] = lStack_20;
    param_1[3] = lStack_10 * 0x208 + lVar2;
    return param_1;
  }
  uStack_24 = uStack_18;
  FUN_1428d9760(&UNK_142b1178e,0x2d,&uStack_24,&UNK_142b10450,&UNK_142b117c0);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

