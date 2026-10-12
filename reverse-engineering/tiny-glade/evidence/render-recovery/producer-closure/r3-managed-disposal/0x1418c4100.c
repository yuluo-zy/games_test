
undefined8 FUN_1418c4100(undefined8 param_1,longlong *param_2,undefined8 param_3)

{
  longlong lVar1;
  code *pcVar2;
  longlong *plVar3;
  undefined8 uVar4;
  undefined1 auStack_238 [8];
  undefined1 auStack_230 [240];
  byte abStack_140 [16];
  undefined1 auStack_130 [248];
  undefined8 uStack_38;
  longlong *plStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  LOCK();
  lVar1 = *param_2;
  if (lVar1 == 0) {
    *param_2 = 8;
  }
  UNLOCK();
  uStack_38 = param_3;
  if (lVar1 != 0) {
    FUN_1428cb5f0(param_2,param_2,1000000000);
  }
  FUN_140c81c70(param_2);
  FUN_140c81c70((ulonglong)param_2 | 1);
  plStack_30 = param_2;
  uVar4 = FUN_1405b0d70(param_2 + 5,&uStack_38);
  FUN_1417e7ad0(abStack_140,plStack_30 + 1,uVar4,&uStack_38);
  plVar3 = plStack_30;
  if ((abStack_140[0] & 1) == 0) {
    FUN_1428d9330(&UNK_142d069c1,0x42,&UNK_142d06a28);
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  func_0x000142923c00(auStack_238,auStack_130,0xf8);
  func_0x000142923c00(param_1,auStack_230,0xf0);
  FUN_140c81e10(plVar3);
  FUN_140c81e10();
  LOCK();
  lVar1 = *plVar3;
  if (lVar1 == 8) {
    *plVar3 = 0;
  }
  UNLOCK();
  if (lVar1 != 8) {
    FUN_1428cbf60(plVar3,0);
  }
  return param_1;
}

