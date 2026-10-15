
longlong FUN_140c0e8a0(longlong param_1,longlong param_2,undefined4 param_3)

{
  code *pcVar1;
  undefined4 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  bool bVar5;
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
  longlong *plStack_68;
  undefined *puStack_60;
  undefined8 uStack_58;
  longlong lStack_48;
  code *pcStack_40;
  longlong *plStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) == '\x02') {
      if (*(int *)(*(longlong *)(param_1 + 8) + 0xc) == *(int *)(param_1 + 0x10)) {
        plVar4 = (longlong *)(*(longlong *)(param_1 + 8) + 0x10);
        uStack_28 = 0xfffffffffffffffe;
        LOCK();
        lVar3 = *plVar4;
        if (lVar3 == 0) {
          *plVar4 = 8;
        }
        UNLOCK();
        if (lVar3 != 0) {
          FUN_1428cb5f0(plVar4,param_3,1000000000);
        }
        FUN_140c81c70(plVar4);
        FUN_140c81c70((ulonglong)plVar4 | 1);
        uStack_30._7_1_ = (char)param_3;
        if (4 < *puRam000000014362c3a8) {
          lStack_48 = (longlong)&uStack_30 + 7;
          pcStack_40 = FUN_140d88320;
          plStack_38 = plVar4;
          puVar2 = (undefined4 *)func_0x00014067dd80(&UNK_142d08718);
          uStack_a8 = *puVar2;
          uStack_a4 = puVar2[1];
          uStack_a0 = puVar2[2];
          uStack_9c = puVar2[3];
          uStack_7c = puVar2[4];
          uStack_98 = 5;
          puStack_90 = &UNK_142d08470;
          uStack_88 = 0x21;
          puStack_78 = &UNK_142d086f8;
          uStack_70 = 2;
          plStack_68 = &lStack_48;
          puStack_60 = (undefined *)0x1;
          uStack_58 = 0;
          uStack_c8 = 0;
          puStack_c0 = &UNK_142d08470;
          uStack_b8 = 0x21;
          uStack_b0 = 0;
          uStack_80 = 1;
          func_0x000140d4a330((longlong)&uStack_30 + 6,&uStack_c8);
          plVar4 = plStack_38;
        }
        if ((int)plVar4[6] == 2) {
          plVar4[10] = plVar4[5];
          plVar4[8] = plVar4[3];
          plVar4[9] = plVar4[4];
          *(int *)(plVar4 + 6) = (int)plVar4[1];
          *(undefined4 *)((longlong)plVar4 + 0x34) = *(undefined4 *)((longlong)plVar4 + 0xc);
          *(int *)(plVar4 + 7) = (int)plVar4[2];
          *(undefined4 *)((longlong)plVar4 + 0x3c) = *(undefined4 *)((longlong)plVar4 + 0x14);
        }
        *(undefined1 *)(plVar4 + 10) = uStack_30._7_1_;
        FUN_140c81e10(plVar4);
        FUN_140c81e10();
        LOCK();
        lVar3 = *plVar4;
        bVar5 = lVar3 == 8;
        if (bVar5) {
          *plVar4 = 0;
          lVar3 = 8;
        }
        UNLOCK();
        if (!bVar5) {
          lVar3 = FUN_1428cbf60(plVar4,0);
        }
        return lVar3;
      }
      goto code_r0x000140c0e93f;
    }
  }
  else {
    uStack_30 = 0;
    param_1 = param_1 + 0x14;
    plStack_38 = (longlong *)&UNK_142d022a0;
    puStack_60 = &UNK_140c0e910;
    FUN_1428ca816(0,param_1,*(longlong *)(param_2 + 0x648) + 0xa50,&uStack_30);
  }
  uStack_30 = 0;
  plStack_38 = (longlong *)&UNK_142d022c0;
  puStack_60 = &UNK_140c0e93f;
  FUN_1428ca902(0,param_1 + 0x16,&UNK_142d022d8,&uStack_30);
code_r0x000140c0e93f:
  puStack_60 = &UNK_140c0e94b;
  FUN_1428d9310(&UNK_142b19c10);
  pcVar1 = (code *)swi(3);
  lVar3 = (*pcVar1)();
  return lVar3;
}

