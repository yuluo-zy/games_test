
undefined8 FUN_1418c8710(longlong param_1)

{
  code *pcVar1;
  int iVar2;
  undefined8 uVar3;
  undefined4 *puVar4;
  int iStack_40;
  undefined4 uStack_3c;
  undefined4 auStack_38 [2];
  undefined8 uStack_30;
  undefined4 uStack_28;
  undefined8 uStack_20;
  undefined4 auStack_18 [2];
  undefined4 *puStack_10;
  undefined4 uStack_8;
  
  auStack_38[0] = 0x3b9df29a;
  uStack_30 = 0;
  uStack_28 = 1;
  uStack_20 = 0;
  puVar4 = auStack_38;
  do {
    puVar4 = *(undefined4 **)(puVar4 + 2);
  } while (puVar4 != (undefined4 *)0x0);
  auStack_18[0] = 9;
  puStack_10 = auStack_38;
  uStack_8 = 0;
  iVar2 = (**(code **)(param_1 + 0xb8))(*(undefined8 *)(param_1 + 0x5c8),auStack_18,0,&iStack_40);
  if (iVar2 == 0) {
    return CONCAT44(uStack_3c,iStack_40);
  }
  iStack_40 = iVar2;
  FUN_1428d9760(&UNK_142d07218,0x2b,&iStack_40,&UNK_142d071f8,&UNK_142d07268);
  pcVar1 = (code *)swi(3);
  uVar3 = (*pcVar1)();
  return uVar3;
}

