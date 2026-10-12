
undefined8 * FUN_140bed050(undefined8 *param_1,longlong param_2,undefined8 *param_3)

{
  undefined2 uVar1;
  undefined4 uVar2;
  code *pcVar3;
  undefined8 uVar4;
  int iVar5;
  undefined8 *puVar6;
  int iStack_6c;
  int iStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined4 auStack_48 [2];
  undefined8 uStack_40;
  undefined4 uStack_38;
  undefined4 uStack_34;
  
  iStack_68 = CONCAT31(iStack_68._1_3_,0x15);
  FUN_1418c1380(0x143634e40,&iStack_68,1);
  uVar2 = *(undefined4 *)(param_3 + 3);
  auStack_48[0] = 0x27;
  uStack_40 = 0;
  uStack_38 = 2;
  uStack_34 = uVar2;
  iVar5 = (**(code **)(param_2 + 0x298))(*(undefined8 *)(param_2 + 0x640),auStack_48,0,&iStack_68);
  if (iVar5 == 0) {
    uStack_58 = CONCAT44(uStack_64,iStack_68);
    iStack_68 = 0x28;
    uStack_60 = 0;
    uStack_50 = 0x100000000;
    pcVar3 = *(code **)(param_2 + 0x2b0);
    uVar4 = *(undefined8 *)(param_2 + 0x640);
    puVar6 = (undefined8 *)func_0x000140613c10(8,8);
    if (puVar6 != (undefined8 *)0x0) {
      iVar5 = (*pcVar3)(uVar4,&iStack_68,puVar6);
      param_3 = puVar6;
      if (iVar5 == 0) {
        uVar4 = *puVar6;
        func_0x000140613c20(puVar6,8,8);
        uVar1 = *(undefined2 *)(*(longlong *)(param_2 + 0x648) + 0xa50);
        param_1[5] = 0;
        param_1[6] = 0;
        *param_1 = 0;
        param_1[3] = 0;
        param_1[4] = 8;
        param_1[7] = 8;
        param_1[8] = 0;
        param_1[9] = uVar4;
        *(undefined4 *)(param_1 + 10) = uVar2;
        *(undefined2 *)((longlong)param_1 + 0x54) = uVar1;
        *(undefined1 *)((longlong)param_1 + 0x56) = 0;
        return param_1;
      }
      goto code_r0x000140bed1e7;
    }
  }
  else {
    iStack_68 = iVar5;
    FUN_1428d9760(&UNK_142b14b50,0x2b,&iStack_68,&UNK_142b14b30,&UNK_142b14c08);
  }
  iVar5 = FUN_1428d8fe3(8,8,&UNK_142b14aa8);
code_r0x000140bed1e7:
  func_0x000140613c20(param_3,8,8);
  iStack_6c = iVar5;
  FUN_1428d9760(&UNK_142b14b50,0x2b,&iStack_6c,&UNK_142b14b30,&UNK_142b14c20);
  pcVar3 = (code *)swi(3);
  puVar6 = (undefined8 *)(*pcVar3)();
  return puVar6;
}

