
void FUN_140bf09b0(longlong param_1,int *param_2,undefined8 param_3,undefined8 param_4)

{
  code *pcVar1;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  if (*(longlong *)(param_1 + 0x870) == 0) {
    return;
  }
  FUN_140d797d0(&uStack_88,param_3,param_4);
  if (SBORROW8(0,CONCAT44(uStack_84,uStack_88))) {
    uStack_40 = 0;
    uStack_38 = 0;
    uStack_58 = 0x3b9cbe02;
    uStack_50 = 0;
    if (*param_2 == 1) {
      (**(code **)(param_1 + 0x898))(*(undefined8 *)(param_2 + 0x12),&uStack_58);
      *(undefined1 *)CONCAT44(uStack_7c,uStack_80) = 0;
      if (CONCAT44(uStack_74,uStack_78) == 0) {
        return;
      }
      func_0x000140613c20((undefined1 *)CONCAT44(uStack_7c,uStack_80),CONCAT44(uStack_74,uStack_78),
                          1);
      return;
    }
    FUN_1428d9430(&UNK_142b15b78,0x4c,&UNK_142b15bf0);
  }
  else {
    uStack_40 = CONCAT44(uStack_6c,uStack_70);
    uStack_58 = uStack_88;
    uStack_54 = uStack_84;
    uStack_50 = CONCAT44(uStack_7c,uStack_80);
    FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_58,&UNK_142b151d8,&UNK_142b15f00);
  }
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

