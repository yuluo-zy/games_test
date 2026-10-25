
longlong FUN_1418ccb50(longlong *param_1,longlong param_2)

{
  byte bVar1;
  longlong lVar2;
  code *pcVar3;
  byte bVar4;
  undefined8 *puVar5;
  longlong lVar6;
  longlong *plVar7;
  undefined *puStack_e0;
  undefined *puStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined *puStack_90;
  undefined8 uStack_88;
  undefined4 **ppuStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  uint *puStack_60;
  code *pcStack_58;
  uint uStack_50;
  int iStack_4c;
  longlong *plStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  LOCK();
  lVar6 = *param_1;
  if (lVar6 == 0) {
    *param_1 = 8;
  }
  UNLOCK();
  if (lVar6 != 0) {
    FUN_1428cb5f0(param_1,param_2,1000000000);
  }
  FUN_140c81c70(param_1);
  FUN_140c81c70((ulonglong)param_1 | 1);
  puStack_e0 = (undefined *)CONCAT71(puStack_e0._1_7_,0x27);
  plStack_48 = param_1;
  FUN_1418c1380(0x143634e40,&puStack_e0,1);
  plVar7 = plStack_48;
  bVar1 = (char)plStack_48[0x15] - 2;
  bVar4 = 1;
  if (bVar1 < 3) {
    bVar4 = bVar1;
  }
  if (bVar4 == 0) {
    lVar6 = plStack_48[0x14];
    if (((*(longlong *)(param_2 + 0x850) != 0) && ((char)plStack_48[0x15] == '\x02')) &&
       ((char)plStack_48[0x17] != '\0')) {
      iStack_4c = (**(code **)(param_2 + 0x858))(*(undefined8 *)(param_2 + 0x868),lVar6);
      uStack_50 = (uint)(iStack_4c != 0);
      if (3 < *puRam000000014362c3a8) {
        puStack_60 = &uStack_50;
        pcStack_58 = FUN_1418c9f40;
        puVar5 = (undefined8 *)func_0x00014067dd80(&UNK_142d08990);
        uStack_c0 = *puVar5;
        uStack_b8 = *(undefined4 *)(puVar5 + 1);
        uStack_b4 = *(undefined4 *)((longlong)puVar5 + 0xc);
        uStack_94 = *(undefined4 *)(puVar5 + 2);
        uStack_b0 = 4;
        puStack_a8 = &UNK_142d08470;
        uStack_a0 = 0x21;
        puStack_90 = &UNK_142d08980;
        uStack_88 = 1;
        ppuStack_80 = &puStack_60;
        uStack_78 = 1;
        uStack_70 = 0;
        puStack_e0 = (undefined *)0x0;
        puStack_d8 = &UNK_142d08470;
        uStack_d0 = 0x21;
        uStack_c8 = 0;
        uStack_98 = 1;
        func_0x000140d4a330(&uStack_39,&puStack_e0);
        plVar7 = plStack_48;
      }
    }
    plVar7[0x14] = lVar6;
    *(undefined1 *)(plVar7 + 0x15) = *(undefined1 *)((longlong)plVar7 + 0xb9);
  }
  else {
    if (bVar4 != 1) {
      puStack_e0 = &UNK_142d089d8;
      puStack_d8 = (undefined *)0x1;
      uStack_d0 = 8;
      uStack_c8 = 0;
      uStack_c0 = 0;
      FUN_1428d9390(&puStack_e0,&UNK_142d089e8);
                    /* WARNING: Does not return */
      pcVar3 = (code *)invalidInstructionException();
      (*pcVar3)();
    }
    lVar6 = plStack_48[0x14];
  }
  FUN_140c81e10(plVar7);
  FUN_140c81e10();
  LOCK();
  lVar2 = *plVar7;
  if (lVar2 == 8) {
    *plVar7 = 0;
  }
  UNLOCK();
  if (lVar2 != 8) {
    FUN_1428cbf60(plVar7,0);
  }
  return lVar6;
}

