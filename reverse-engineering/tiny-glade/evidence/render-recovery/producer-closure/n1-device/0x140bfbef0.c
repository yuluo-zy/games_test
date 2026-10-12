
undefined8 *
FUN_140bfbef0(undefined8 *param_1,longlong param_2,longlong param_3,longlong param_4,
             undefined8 param_5,undefined4 param_6)

{
  undefined2 uVar1;
  undefined4 uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  code *pcVar5;
  int iVar6;
  longlong lVar7;
  undefined8 auStack_a0 [6];
  undefined4 auStack_70 [2];
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined8 uStack_58;
  undefined4 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined4 uStack_38;
  longlong lStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  lStack_30 = param_4;
  if (*(short *)(param_3 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_3 + 0x16) == '\x04') {
      if (*(int *)(*(longlong *)(param_3 + 8) + 0xc) == *(int *)(param_3 + 0x10)) {
        uStack_40 = *(undefined8 *)(param_4 + 0x70);
        uStack_58 = *(undefined8 *)(*(longlong *)(param_3 + 8) + 0x10);
        auStack_70[0] = 0x25;
        uStack_68 = 0;
        uStack_60 = 0;
        uStack_50 = param_6;
        uStack_48 = param_5;
        uStack_38 = 1;
        iVar6 = (**(code **)(param_2 + 0x270))
                          (*(undefined8 *)(param_2 + 0x640),auStack_70,0,auStack_a0);
        if (iVar6 == 0) {
          puVar3 = *(undefined8 **)(param_2 + 0x648);
          lVar7 = FUN_1418ae7e0(puVar3 + 0x11b,auStack_a0[0]);
          uVar2 = *(undefined4 *)(lVar7 + 0xc);
          uVar1 = *(undefined2 *)(puVar3 + 0x14a);
          uVar4 = *puVar3;
          param_1[1] = lVar7 + 0x10;
          param_1[2] = uVar4;
          param_1[3] = lVar7;
          *(undefined4 *)(param_1 + 4) = uVar2;
          *(undefined2 *)((longlong)param_1 + 0x24) = uVar1;
          *(undefined1 *)((longlong)param_1 + 0x26) = 7;
          *param_1 = 0xb;
          iVar6 = *(int *)(lStack_30 + 0x6c);
        }
        else {
          FUN_140c84010(param_1 + 1);
          *(int *)(param_1 + 4) = iVar6;
          *param_1 = 9;
          iVar6 = *(int *)(lStack_30 + 0x6c);
        }
        if (iVar6 != 0) {
          *(undefined4 *)(lStack_30 + 0x6c) = 0;
        }
        return param_1;
      }
      FUN_1428d9310(&UNK_142b16ac8);
    }
    else {
      auStack_a0[0] = 0;
      FUN_1428ca902(0,param_3 + 0x16,&UNK_142d022db,auStack_a0,&UNK_142d022c0);
    }
  }
  else {
    auStack_a0[0] = 0;
    FUN_1428ca816(0,param_3 + 0x14,*(longlong *)(param_2 + 0x648) + 0xa50,auStack_a0,&UNK_142d022a0)
    ;
  }
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

