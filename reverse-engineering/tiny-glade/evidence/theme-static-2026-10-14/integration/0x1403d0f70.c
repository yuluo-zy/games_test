
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1403d0f70(undefined8 param_1,undefined8 param_2)

{
  code *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  undefined8 **ppuVar6;
  longlong *plStack_478;
  undefined8 uStack_470;
  undefined8 uStack_468;
  longlong *plStack_460;
  undefined4 uStack_458;
  undefined4 uStack_454;
  undefined8 uStack_450;
  undefined8 *puStack_428;
  undefined8 uStack_420;
  longlong lStack_418;
  undefined8 uStack_410;
  longlong lStack_408;
  undefined8 *puStack_400;
  undefined *puStack_3f8;
  longlong lStack_3f0;
  undefined8 uStack_3e8;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  uint uStack_3d8;
  undefined4 uStack_3d4;
  undefined8 uStack_3d0;
  undefined *puStack_3c8;
  undefined8 uStack_3c0;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
  undefined *puStack_3b0;
  undefined8 uStack_3a8;
  undefined8 **ppuStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined2 uStack_300;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined1 uStack_2c8;
  undefined8 uStack_280;
  undefined8 uStack_278;
  undefined1 uStack_270;
  longlong **pplStack_1c0;
  undefined *puStack_1b8;
  int aiStack_100 [2];
  longlong *plStack_f8;
  longlong **pplStack_40;
  undefined8 uStack_30;
  
  pplStack_40 = &plStack_478;
  uStack_30 = 0xfffffffffffffffe;
  uStack_280 = 1;
  uStack_278 = 1;
  uStack_270 = 0;
  puStack_400 = (undefined8 *)0x0;
  puStack_3f8 = (undefined *)0x0;
  uStack_380 = 0;
  uStack_378 = 0;
  uStack_300 = 0;
  uStack_2f8 = 0;
  uStack_2f0 = 8;
  uStack_2e8 = 0;
  uStack_2e0 = 0;
  uStack_2d8 = 8;
  uStack_2d0 = 0;
  uStack_2c8 = 1;
  lVar3 = func_0x000140613c10(0x200,0x80);
  if (lVar3 == 0) {
    FUN_1428d9000(0x80,0x200);
  }
  else {
    ppuVar6 = &puStack_400;
    func_0x000142923c00(lVar3,ppuVar6,0x200);
    uStack_420 = 1;
    uStack_410 = 1;
    lStack_418 = lVar3;
    lStack_408 = lVar3;
    uVar4 = FUN_140b9dc50();
    FUN_140d62810(&puStack_400,uVar4,ppuVar6,&UNK_1429d1c9e,9);
    FUN_14054aa00(aiStack_100,&puStack_400,1,lVar3);
    if (aiStack_100[0] == 2) {
      plStack_478 = plStack_f8;
      if (*_DAT_14362c3a8 != 0) {
        pplStack_1c0 = &plStack_478;
        puStack_1b8 = &UNK_140d4cc20;
        puVar5 = (undefined4 *)func_0x00014067dd80(&UNK_1429d1d40);
        uStack_3e0 = *puVar5;
        uStack_3dc = puVar5[1];
        uStack_3d8 = puVar5[2];
        uStack_3d4 = puVar5[3];
        uStack_3b4 = puVar5[4];
        uStack_3d0 = 1;
        puStack_3c8 = &UNK_1429d1d58;
        uStack_3c0 = 0x20;
        puStack_3b0 = &UNK_1429d1d30;
        uStack_3a8 = 1;
        ppuStack_3a0 = &pplStack_1c0;
        uStack_398 = 1;
        uStack_390 = 0;
        puStack_400 = (undefined8 *)0x0;
        puStack_3f8 = &UNK_1429d1d58;
        lStack_3f0 = 0x20;
        uStack_3e8 = 0;
        uStack_3b8 = 1;
        func_0x000140d4a330(&plStack_460,&puStack_400);
      }
      plStack_460 = (longlong *)0x0;
      func_0x000140d4cc30(&plStack_478);
LAB_1403d132a:
      func_0x00014083c480(&puStack_400);
      uVar4 = FUN_140136210(param_2,&puStack_400);
      puStack_400 = (undefined8 *)((ulonglong)puStack_400 & 0xffffffffffffff00);
      puStack_3f8 = (undefined *)0x1;
      lStack_3f0 = lVar3;
      uVar4 = FUN_14013c410(uVar4,&puStack_400);
      FUN_140327210(uVar4);
      plVar2 = plStack_460;
      if (plStack_460 != (longlong *)0x0) {
        CloseHandle(uStack_450);
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        if (*plVar2 == 0) {
          FUN_140d53b60(&plStack_460);
        }
        plVar2 = (longlong *)CONCAT44(uStack_454,uStack_458);
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        if (*plVar2 == 0) {
          FUN_140d53db0(&uStack_458);
        }
      }
      return;
    }
    func_0x000142923c00(&pplStack_1c0,aiStack_100,0xc0);
    lStack_3f0 = 0x8000000000000000;
    puStack_400 = (undefined8 *)0x0;
    uStack_3d8 = uStack_3d8 & 0xffffff00;
    puStack_428 = (undefined8 *)func_0x000140613c10(0xd,1);
    if (puStack_428 == (undefined8 *)0x0) {
      FUN_1428d8fe3(1,0xd,&UNK_1429d1a38);
    }
    else {
      *(undefined8 *)((longlong)puStack_428 + 5) = 0x7265686374656620;
      *puStack_428 = 0x656620656d656874;
      plStack_478 = (longlong *)0xd;
      uStack_468 = 0xd;
      uStack_470 = puStack_428;
      FUN_140d558f0(&plStack_460,&puStack_400,&plStack_478);
      func_0x000142923c00(&puStack_400,&pplStack_1c0,0xc0);
      FUN_14028b5a0(&plStack_478,&plStack_460,&puStack_400);
      if (plStack_478 != (longlong *)0x0) {
        uStack_450 = uStack_468;
        plStack_460 = plStack_478;
        uStack_458 = (undefined4)uStack_470;
        uStack_454 = uStack_470._4_4_;
        goto LAB_1403d132a;
      }
      puStack_400 = uStack_470;
      FUN_1428d9760(&UNK_1429d1cd4,0x1c,&puStack_400,&UNK_1429d19a8,&UNK_1429d1cf0);
    }
  }
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

