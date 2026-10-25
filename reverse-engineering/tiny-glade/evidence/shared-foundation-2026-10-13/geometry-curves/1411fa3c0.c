
longlong * FUN_1411fa3c0(longlong *param_1,undefined8 param_2,undefined8 param_3)

{
  code *pcVar1;
  longlong *plVar2;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong lStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  
  FUN_1411f9920(&lStack_50,param_2,0);
  if (!SBORROW8(0,lStack_50)) {
    param_1[6] = lStack_20;
    param_1[4] = lStack_30;
    param_1[5] = lStack_28;
    param_1[2] = CONCAT44(uStack_3c,uStack_40);
    param_1[3] = lStack_38;
    *param_1 = lStack_50;
    param_1[1] = CONCAT44(uStack_44,uStack_48);
    return param_1;
  }
  uStack_68 = uStack_48;
  uStack_64 = uStack_44;
  uStack_60 = uStack_40;
  uStack_5c = uStack_3c;
  FUN_1428d9760(&UNK_142c18bc8,0x2b,&uStack_68,&UNK_142c18ba8,param_3);
  pcVar1 = (code *)swi(3);
  plVar2 = (longlong *)(*pcVar1)();
  return plVar2;
}

