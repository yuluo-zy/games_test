
longlong FUN_1418cb370(longlong *param_1,undefined8 param_2)

{
  undefined4 *puVar1;
  longlong lVar2;
  bool bVar3;
  undefined8 uStack_c8;
  undefined *puStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined *puStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined *puStack_78;
  undefined8 uStack_70;
  undefined1 **ppuStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined1 *puStack_48;
  code *pcStack_40;
  longlong *plStack_38;
  undefined1 uStack_2a;
  undefined1 uStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  LOCK();
  lVar2 = *param_1;
  if (lVar2 == 0) {
    *param_1 = 8;
  }
  UNLOCK();
  if (lVar2 != 0) {
    FUN_1428cb5f0(param_1,param_2,1000000000);
  }
  FUN_140c81c70(param_1);
  FUN_140c81c70((ulonglong)param_1 | 1);
  uStack_29 = (char)param_2;
  if (4 < *puRam000000014362c3a8) {
    puStack_48 = &uStack_29;
    pcStack_40 = FUN_140d88320;
    plStack_38 = param_1;
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142d08718);
    uStack_a8 = *puVar1;
    uStack_a4 = puVar1[1];
    uStack_a0 = puVar1[2];
    uStack_9c = puVar1[3];
    uStack_7c = puVar1[4];
    uStack_98 = 5;
    puStack_90 = &UNK_142d08470;
    uStack_88 = 0x21;
    puStack_78 = &UNK_142d086f8;
    uStack_70 = 2;
    ppuStack_68 = &puStack_48;
    uStack_60 = 1;
    uStack_58 = 0;
    uStack_c8 = 0;
    puStack_c0 = &UNK_142d08470;
    uStack_b8 = 0x21;
    uStack_b0 = 0;
    uStack_80 = 1;
    func_0x000140d4a330(&uStack_2a,&uStack_c8);
    param_1 = plStack_38;
  }
  if ((int)param_1[6] == 2) {
    param_1[10] = param_1[5];
    param_1[8] = param_1[3];
    param_1[9] = param_1[4];
    *(int *)(param_1 + 6) = (int)param_1[1];
    *(undefined4 *)((longlong)param_1 + 0x34) = *(undefined4 *)((longlong)param_1 + 0xc);
    *(int *)(param_1 + 7) = (int)param_1[2];
    *(undefined4 *)((longlong)param_1 + 0x3c) = *(undefined4 *)((longlong)param_1 + 0x14);
  }
  *(undefined1 *)(param_1 + 10) = uStack_29;
  FUN_140c81e10(param_1);
  FUN_140c81e10();
  LOCK();
  lVar2 = *param_1;
  bVar3 = lVar2 == 8;
  if (bVar3) {
    *param_1 = 0;
    lVar2 = 8;
  }
  UNLOCK();
  if (!bVar3) {
    lVar2 = FUN_1428cbf60(param_1,0);
  }
  return lVar2;
}

