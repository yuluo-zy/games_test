
longlong *
FUN_140beea80(longlong *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             longlong param_5)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_40;
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  if ((*(char *)(param_5 + 0xe) != '\x02') && (*(char *)(param_5 + 0xe) != '\x04')) {
    FUN_1428d9430(&UNK_142b158f8,0x69,&UNK_142b15968);
code_r0x000140beebe1:
    FUN_1428d9330(&UNK_142b158d0,0xd,&UNK_142b158e0);
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  lVar4 = param_2 + 0x870;
  if (*(longlong *)(param_2 + 0x870) == 0) {
    lVar4 = 0;
  }
  FUN_140bee200(&uStack_98,param_2 + 0x78,lVar4,*(undefined8 *)(param_2 + 0x648),
                *(longlong *)(param_2 + 0x678) + 0x10,param_5,param_3,param_4);
  lVar3 = lStack_78;
  lVar2 = lStack_80;
  lVar4 = lStack_88;
  lStack_40 = lStack_88;
  lStack_38 = lStack_80;
  lStack_30 = lStack_78;
  lStack_28 = lStack_70;
  lStack_20 = lStack_68;
  if (CONCAT44(uStack_94,uStack_98) == 0xb) {
    lStack_78 = lStack_68;
    lStack_88 = lVar3;
    lStack_80 = lStack_70;
    uStack_c0 = (undefined4)lVar4;
    uStack_bc = (undefined4)((ulonglong)lVar4 >> 0x20);
    uStack_b8 = (undefined4)lVar2;
    uStack_b4 = (undefined4)((ulonglong)lVar2 >> 0x20);
    uStack_98 = uStack_c0;
    uStack_94 = uStack_bc;
    uStack_90 = uStack_b8;
    uStack_8c = uStack_b4;
    if (lStack_60 == 0) goto code_r0x000140beebe1;
    param_1[5] = lStack_68;
    param_1[3] = lVar3;
    param_1[4] = lStack_70;
    param_1[1] = lVar4;
    param_1[2] = lVar2;
    param_1[6] = lStack_60;
    param_1[7] = lStack_58;
    *param_1 = 0xb;
  }
  else {
    param_1[5] = lStack_70;
    param_1[6] = lStack_68;
    param_1[3] = lStack_80;
    param_1[4] = lStack_78;
    param_1[1] = CONCAT44(uStack_8c,uStack_90);
    param_1[2] = lStack_88;
    *param_1 = CONCAT44(uStack_94,uStack_98);
    param_1[7] = lStack_60;
    param_1[8] = lStack_58;
    param_1[9] = lStack_50;
  }
  return param_1;
}

