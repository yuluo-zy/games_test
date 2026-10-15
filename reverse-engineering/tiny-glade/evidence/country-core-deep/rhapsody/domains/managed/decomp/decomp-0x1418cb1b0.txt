
longlong FUN_1418cb1b0(longlong *param_1,longlong param_2)

{
  undefined4 *puVar1;
  longlong lVar2;
  bool bVar3;
  undefined8 uStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 **ppuStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_50;
  code *pcStack_48;
  longlong lStack_40;
  longlong *plStack_38;
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
  lStack_40 = param_2;
  if (4 < *puRam000000014362c3a8) {
    plStack_50 = &lStack_40;
    pcStack_48 = FUN_1418bde90;
    plStack_38 = param_1;
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142d086c8);
    uStack_b0 = *puVar1;
    uStack_ac = puVar1[1];
    uStack_a8 = puVar1[2];
    uStack_a4 = puVar1[3];
    uStack_84 = puVar1[4];
    uStack_a0 = 5;
    puStack_98 = &UNK_142d08470;
    uStack_90 = 0x21;
    puStack_80 = &UNK_142d086a8;
    uStack_78 = 2;
    ppuStack_70 = &plStack_50;
    uStack_68 = 1;
    uStack_60 = 0;
    uStack_d0 = 0;
    puStack_c8 = &UNK_142d08470;
    uStack_c0 = 0x21;
    uStack_b8 = 0;
    uStack_88 = 1;
    func_0x000140d4a330(&uStack_29,&uStack_d0);
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
  param_1[9] = lStack_40;
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

