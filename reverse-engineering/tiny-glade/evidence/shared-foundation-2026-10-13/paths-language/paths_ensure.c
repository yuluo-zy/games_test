
undefined8 * FUN_14184a0a0(undefined8 *param_1,longlong *param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined8 uStack_60;
  longlong *plStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  longlong *plStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  plStack_30 = param_2;
  uVar2 = FUN_140d76ed0(param_2);
  uStack_50 = CONCAT31(uStack_50._1_3_,1);
  uStack_60 = uVar2;
  plStack_58 = param_2;
  uVar3 = FUN_140d5a150(&uStack_50,uVar2,param_2);
  lVar4 = FUN_142131410(uVar3,&uStack_60);
  if (lVar4 == 0) {
    FUN_140d61b00(&uStack_50,uVar2,param_2);
    uVar2 = 8;
    puVar5 = (undefined4 *)func_0x000140613c10(0x20);
    if (puVar5 == (undefined4 *)0x0) {
      FUN_1428d9000(8,0x20);
                    /* WARNING: Does not return */
      pcVar1 = (code *)invalidInstructionException();
      (*pcVar1)();
    }
    puVar5[4] = uStack_40;
    puVar5[5] = uStack_3c;
    puVar5[6] = uStack_38;
    puVar5[7] = uStack_34;
    *puVar5 = uStack_50;
    puVar5[1] = uStack_4c;
    puVar5[2] = uStack_48;
    puVar5[3] = uStack_44;
    uVar3 = FUN_140d76ed0(puVar5);
    *param_1 = uVar3;
    param_1[1] = uVar2;
    lVar4 = *plStack_30;
  }
  else {
    param_1[1] = lVar4;
    *param_1 = 0;
    lVar4 = *plStack_30;
  }
  if (lVar4 != 0) {
    func_0x000140613c20(plStack_30[1],lVar4,1);
  }
  return param_1;
}

