
longlong * FUN_140c02570(longlong *param_1,longlong param_2)

{
  ulonglong uVar1;
  code *pcVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  longlong lStack_60;
  longlong lStack_58;
  int iStack_50;
  short sStack_4c;
  char cStack_4a;
  undefined1 uStack_49;
  longlong lStack_48;
  longlong lStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  FUN_140c029f0(&lStack_60);
  uVar1 = *(ulonglong *)(*(longlong *)(param_2 + 0x658) + 0x340);
  if (uVar1 - 1 < (uVar1 ^ uVar1 - 1)) {
    lVar5 = param_2;
    lVar3 = FUN_140bd4cd0(&lStack_60);
    if (lVar3 == 0) {
      FUN_1428d9310(&UNK_142b17f48);
    }
    else if (sStack_4c == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
      if (cStack_4a == '\x01') {
        if (*(int *)(lStack_58 + 0xc) == iStack_50) {
          uStack_80 = *(undefined8 *)(lStack_58 + 0x30);
          uStack_90 = CONCAT44(uStack_90._4_4_,0x3b9e8321);
          uStack_88 = 0;
          lVar4 = (**(code **)(param_2 + 0x508))(*(undefined8 *)(param_2 + 0x640),&uStack_90);
          param_1[9] = lStack_40;
          param_1[7] = CONCAT17(uStack_49,CONCAT16(cStack_4a,CONCAT24(sStack_4c,iStack_50)));
          param_1[8] = lStack_48;
          param_1[5] = lStack_60;
          param_1[6] = lStack_58;
          *param_1 = lVar3;
          param_1[1] = lVar5;
          param_1[2] = 0;
          param_1[3] = lVar4;
          param_1[4] = uVar1;
          return param_1;
        }
        FUN_1428d9310(&UNK_142b0f7d8);
      }
      else {
        uStack_90 = 0;
        FUN_1428ca902(0,&cStack_4a,&UNK_142d022d9,&uStack_90,&UNK_142d022c0);
      }
    }
    else {
      uStack_90 = 0;
      FUN_1428ca816(0,&sStack_4c,*(longlong *)(param_2 + 0x648) + 0xa50,&uStack_90,&UNK_142d022a0);
    }
  }
  else {
    FUN_1428d9430(&UNK_142b17ed2,0x2d,&UNK_142b17f30);
  }
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

