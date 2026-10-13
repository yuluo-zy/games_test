
undefined8 * FUN_142456250(undefined8 *param_1,int *param_2,undefined8 param_3,uint param_4)

{
  byte bVar1;
  char cVar2;
  undefined8 uVar3;
  code *pcVar4;
  uint uVar5;
  undefined8 *puVar6;
  uint uVar7;
  uint in_R10D;
  undefined *puStack_50;
  undefined8 uStack_48;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined **ppuStack_20;
  undefined *puStack_18;
  undefined *puStack_10;
  undefined8 uStack_8;
  
  uVar5 = *param_2 - 2;
  uVar7 = 2;
  if (1 < uVar5) {
    uVar5 = 2;
  }
  if (uVar5 == 2) {
    *(int *)(param_1 + 1) = param_2[2];
    *param_1 = *(undefined8 *)param_2;
    bVar1 = *(byte *)(param_2 + 5) - 2;
    param_4 = (uint)bVar1;
    uVar7 = 2;
    if (bVar1 < 2) {
      uVar7 = (uint)bVar1;
    }
    if ((char)uVar7 != '\x02') {
LAB_142456330:
      if ((uVar7 & 0xff) == 1) {
        puStack_10 = &UNK_14315119f;
        uStack_8 = 0x1b;
        ppuStack_20 = &puStack_10;
        puStack_18 = &UNK_140109e90;
        puStack_50 = &UNK_143150340;
        uStack_48 = 2;
        uStack_30 = 0;
        ppuStack_40 = &ppuStack_20;
        uStack_38 = 1;
        FUN_1428d9390(&puStack_50,&UNK_143079930);
        goto LAB_1424563aa;
      }
      goto LAB_142456508;
    }
    uVar3 = *(undefined8 *)(param_2 + 3);
    in_R10D = (int)uVar3 - 3;
    param_4 = 2;
    if (1 < in_R10D) {
      in_R10D = 2;
    }
    if (in_R10D == 2) {
      cVar2 = *(char *)((longlong)param_2 + 0x15);
      bVar1 = cVar2 - 3;
      param_4 = 2;
      if (bVar1 < 2) {
        param_4 = (uint)bVar1;
      }
      if ((char)param_4 == '\x02') {
        *(byte *)((longlong)param_1 + 0x14) = *(byte *)(param_2 + 5) & 1;
        *(undefined8 *)((longlong)param_1 + 0xc) = uVar3;
        *(char *)((longlong)param_1 + 0x15) = cVar2;
        return param_1;
      }
LAB_142456421:
      if ((param_4 & 0xff) != 1) goto LAB_142456576;
      puStack_10 = &UNK_1431511ba;
      uStack_8 = 0x27;
      ppuStack_20 = &puStack_10;
      puStack_18 = &UNK_140109e90;
      puStack_50 = &UNK_143150340;
      uStack_48 = 2;
      uStack_30 = 0;
      ppuStack_40 = &ppuStack_20;
      uStack_38 = 1;
      FUN_1428d9390(&puStack_50,&UNK_143079960);
      goto LAB_14245649b;
    }
LAB_1424563aa:
    if (in_R10D == 1) {
      puStack_10 = &UNK_1431510fc;
      uStack_8 = 0x1e;
      ppuStack_20 = &puStack_10;
      puStack_18 = &UNK_140109e90;
      puStack_50 = &UNK_143150340;
      uStack_48 = 2;
      uStack_30 = 0;
      ppuStack_40 = &ppuStack_20;
      uStack_38 = 1;
      FUN_1428d9390(&puStack_50,&UNK_143079948);
      goto LAB_142456421;
    }
  }
  else {
    if (uVar5 == 0) {
      puStack_50 = &UNK_1431503b0;
      uStack_48 = 1;
      ppuStack_40 = (undefined8 **)0x8;
      uStack_38 = 0;
      uStack_30 = 0;
      FUN_1428d9390(&puStack_50,&UNK_143079918);
      goto LAB_142456330;
    }
LAB_14245649b:
    puStack_10 = &UNK_14315111a;
    uStack_8 = 0x1e;
    ppuStack_20 = &puStack_10;
    puStack_18 = &UNK_140109e90;
    puStack_50 = &UNK_143150340;
    uStack_48 = 2;
    uStack_30 = 0;
    ppuStack_40 = &ppuStack_20;
    uStack_38 = 1;
    FUN_1428d9390(&puStack_50,&UNK_143079918);
LAB_142456508:
    puStack_50 = &UNK_1431503b0;
    uStack_48 = 1;
    ppuStack_40 = (undefined8 **)0x8;
    uStack_38 = 0;
    uStack_30 = 0;
    FUN_1428d9390(&puStack_50,&UNK_143079930);
  }
  puStack_50 = &UNK_1431503b0;
  uStack_48 = 1;
  ppuStack_40 = (undefined8 **)0x8;
  uStack_38 = 0;
  uStack_30 = 0;
  FUN_1428d9390(&puStack_50,&UNK_143079948);
LAB_142456576:
  puStack_50 = &UNK_1431503b0;
  uStack_48 = 1;
  ppuStack_40 = (undefined8 **)0x8;
  uStack_38 = 0;
  uStack_30 = 0;
  FUN_1428d9390(&puStack_50,&UNK_143079960);
  pcVar4 = (code *)swi(3);
  puVar6 = (undefined8 *)(*pcVar4)();
  return puVar6;
}

