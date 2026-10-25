
undefined8 * FUN_140c03fa0(undefined8 *param_1,longlong param_2,undefined8 param_3)

{
  char *pcVar1;
  longlong *plVar2;
  char cVar3;
  longlong lVar4;
  code *pcVar5;
  bool bVar6;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  char *pcStack_48;
  longlong *plStack_40;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  pcVar1 = (char *)(param_2 + 0x9f0);
  LOCK();
  bVar6 = *(char *)(param_2 + 0x9f0) == '\0';
  if (bVar6) {
    *(char *)(param_2 + 0x9f0) = '\x01';
  }
  UNLOCK();
  if (!bVar6) {
    FUN_1428cd210(pcVar1,param_2,1000000000);
  }
  pcStack_48 = pcVar1;
  FUN_140c81c70(pcVar1);
  plVar2 = (longlong *)(param_2 + 0x9f8);
  if (*(int *)(param_2 + 0x9f8) == 4) {
    FUN_140bef4a0(&uStack_100,param_2,&UNK_142b18788,8,*(undefined4 *)(param_2 + 0xad4));
    uStack_31 = 1;
    FUN_140bed230(&uStack_100,param_2,0);
    uStack_31 = 1;
    FUN_140c08560(param_3,&uStack_100);
    lStack_58 = lStack_b0;
    lStack_68 = lStack_c0;
    lStack_60 = lStack_b8;
    uStack_78 = uStack_d0;
    uStack_74 = uStack_cc;
    uStack_70 = uStack_c8;
    uStack_6c = uStack_c4;
    uStack_88 = uStack_e0;
    uStack_84 = uStack_dc;
    uStack_80 = uStack_d8;
    uStack_7c = uStack_d4;
    uStack_98 = uStack_f0;
    uStack_94 = uStack_ec;
    uStack_90 = uStack_e8;
    uStack_8c = uStack_e4;
    uStack_a8 = uStack_100;
    uStack_a4 = uStack_fc;
    uStack_a0 = uStack_f8;
    uStack_9c = uStack_f4;
    lVar4 = *plVar2;
    plStack_40 = plVar2;
    if (lVar4 != 4) {
      if ((lVar4 != 0) && ((int)lVar4 != 3)) {
        lStack_50 = param_2;
        FUN_1428d9430(&UNK_142b12b58,0x6e,&UNK_142b12bf0);
                    /* WARNING: Does not return */
        pcVar5 = (code *)invalidInstructionException();
        (*pcVar5)();
      }
      if (*(longlong *)(param_2 + 0xa10) != 0) {
        func_0x000140613c20(*(undefined8 *)(param_2 + 0xa18),*(longlong *)(param_2 + 0xa10) << 3,8);
      }
      FUN_1405df600(param_2 + 0xa28);
    }
    plStack_40[10] = lStack_58;
    plStack_40[8] = lStack_68;
    plStack_40[9] = lStack_60;
    plStack_40[6] = CONCAT44(uStack_74,uStack_78);
    plStack_40[7] = CONCAT44(uStack_6c,uStack_70);
    plStack_40[4] = CONCAT44(uStack_84,uStack_88);
    plStack_40[5] = CONCAT44(uStack_7c,uStack_80);
    plStack_40[2] = CONCAT44(uStack_94,uStack_98);
    plStack_40[3] = CONCAT44(uStack_8c,uStack_90);
    *(undefined4 *)plStack_40 = uStack_a8;
    *(undefined4 *)((longlong)plStack_40 + 4) = uStack_a4;
    *(undefined4 *)(plStack_40 + 1) = uStack_a0;
    *(undefined4 *)((longlong)plStack_40 + 0xc) = uStack_9c;
  }
  else {
    FUN_140c08560(param_3,plVar2);
  }
  pcVar1 = pcStack_48;
  *param_1 = 0xb;
  FUN_140c81e10();
  LOCK();
  cVar3 = *pcVar1;
  if (cVar3 == '\x01') {
    *pcVar1 = '\0';
  }
  UNLOCK();
  if (cVar3 != '\x01') {
    FUN_1428cdb90(pcVar1,0);
  }
  return param_1;
}

