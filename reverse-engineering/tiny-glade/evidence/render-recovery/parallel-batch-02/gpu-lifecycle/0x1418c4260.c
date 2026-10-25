
longlong FUN_1418c4260(longlong *param_1,undefined8 param_2)

{
  code *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  bool bVar4;
  longlong alStack_228 [32];
  undefined1 auStack_128 [8];
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  longlong *plStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  LOCK();
  lVar3 = *param_1;
  if (lVar3 == 0) {
    *param_1 = 8;
  }
  UNLOCK();
  if (lVar3 != 0) {
    FUN_1428cb5f0(param_1,param_2,1000000000);
  }
  FUN_140c81c70(param_1);
  FUN_140c81c70((ulonglong)param_1 | 1);
  uStack_110 = 0;
  uStack_108 = 0;
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e0 = 0;
  uStack_d8 = 0;
  uStack_c8 = 0;
  uStack_c0 = 0;
  uStack_b0 = 0;
  uStack_a8 = 0;
  uStack_98 = 0;
  uStack_90 = 0;
  uStack_80 = 0;
  uStack_78 = 0;
  uStack_68 = 0;
  uStack_60 = 0;
  uStack_50 = 0;
  uStack_48 = 0;
  auStack_128[0] = 0;
  uStack_120 = 0;
  uStack_118 = 8;
  uStack_100 = 8;
  uStack_e8 = 8;
  uStack_d0 = 8;
  uStack_b8 = 8;
  uStack_a0 = 8;
  uStack_88 = 8;
  uStack_70 = 8;
  uStack_58 = 8;
  uStack_40 = 8;
  uStack_38 = 0;
  plStack_30 = param_1;
  FUN_1418adf90(alStack_228,param_1 + 1,param_2,auStack_128);
  plVar2 = plStack_30;
  FUN_140c81e10(plStack_30);
  FUN_140c81e10();
  LOCK();
  lVar3 = *plVar2;
  bVar4 = lVar3 == 8;
  if (bVar4) {
    *plVar2 = 0;
    lVar3 = 8;
  }
  UNLOCK();
  if (!bVar4) {
    lVar3 = FUN_1428cbf60(plVar2,0);
  }
  if (alStack_228[0] != 0) {
    FUN_1428d9430(&UNK_142d06a40,0x20,&UNK_142d06a60);
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  return lVar3;
}

