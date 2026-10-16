
undefined8 FUN_141e4c650(undefined8 param_1,undefined4 param_2,undefined4 param_3)

{
  code *pcVar1;
  undefined8 uVar2;
  int iStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  int iStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  
  FUN_140fe7cb0(&iStack_48,param_1,param_2,param_3);
  if (iStack_48 == 4) {
    return CONCAT44(uStack_3c,uStack_40);
  }
  uStack_18 = uStack_38;
  uStack_14 = uStack_34;
  uStack_10 = uStack_30;
  uStack_c = uStack_2c;
  iStack_28 = iStack_48;
  uStack_24 = uStack_44;
  uStack_20 = uStack_40;
  uStack_1c = uStack_3c;
  FUN_1428d9760(&UNK_142f8f358,0x2b,&iStack_28,&UNK_142f8f338,&UNK_142f8f500);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

