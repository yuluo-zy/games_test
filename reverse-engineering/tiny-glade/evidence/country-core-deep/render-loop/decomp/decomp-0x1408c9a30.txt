
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408c9a30(undefined8 param_1,longlong param_2,longlong param_3)

{
  char *pcVar1;
  char cVar2;
  longlong *plVar3;
  longlong lVar4;
  code *pcVar5;
  undefined8 uVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong *plVar11;
  ulonglong uVar12;
  undefined *puVar13;
  bool bVar14;
  undefined4 uStack_938;
  undefined4 uStack_934;
  undefined4 uStack_930;
  undefined4 uStack_92c;
  undefined4 uStack_928;
  undefined4 uStack_924;
  undefined4 uStack_920;
  undefined4 uStack_91c;
  undefined4 uStack_918;
  undefined4 uStack_914;
  undefined4 uStack_910;
  undefined4 uStack_90c;
  undefined8 uStack_908;
  undefined8 uStack_548;
  undefined8 uStack_540;
  undefined8 uStack_538;
  undefined8 uStack_530;
  undefined8 uStack_528;
  undefined8 uStack_520;
  longlong lStack_518;
  undefined8 uStack_510;
  undefined8 uStack_508;
  undefined4 uStack_4d8;
  undefined4 uStack_4d4;
  undefined4 uStack_4d0;
  undefined4 uStack_4cc;
  undefined8 uStack_4c8;
  undefined8 uStack_4c0;
  undefined4 uStack_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  undefined4 uStack_4ac;
  undefined *puStack_4a8;
  uint uStack_4a0;
  undefined4 uStack_49c;
  undefined1 uStack_498;
  undefined1 uStack_497;
  undefined7 uStack_496;
  undefined8 uStack_48f;
  undefined8 uStack_487;
  undefined8 uStack_47f;
  undefined8 uStack_477;
  undefined8 uStack_46f;
  undefined4 uStack_460;
  undefined4 uStack_45c;
  undefined4 uStack_458;
  undefined4 uStack_454;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined1 auStack_158 [160];
  undefined1 uStack_b8;
  undefined8 uStack_b7;
  undefined8 uStack_af;
  undefined8 uStack_a7;
  undefined8 uStack_9f;
  undefined8 uStack_97;
  undefined8 uStack_8f;
  undefined8 uStack_87;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong *plStack_68;
  undefined8 uStack_60;
  longlong *plStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar9 = param_2 + 0x38;
  uVar12 = 0xd38dae617fb79c7a;
  lVar8 = FUN_1400dd6f0(lVar9,0xd38dae617fb79c7a,0x22aadf5c5dad1eae);
  if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
     (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0)) {
code_r0x0001408c9fd4:
    FUN_1428d9310(&UNK_142aa3198);
code_r0x0001408c9fe0:
    FUN_1428d9310(&UNK_142aa31b0);
code_r0x0001408c9fec:
    FUN_1428d9310(&UNK_142aa31c8);
  }
  else {
    lVar8 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x0001408c9fd4;
    plVar3 = *(longlong **)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    lVar8 = *plVar3;
    if ((lVar8 == 2) || (((int)lVar8 == 10 && ((char)plVar3[1] != '\x01')))) {
      return;
    }
    uVar12 = 4;
    if (lVar8 - 2U < 9) {
      uVar12 = lVar8 - 2U;
    }
    if ((0xafUL >> (uVar12 & 0x3f) & 1) == 0) {
      if ((0x50UL >> (uVar12 & 0x3f) & 1) != 0) {
        return;
      }
      if ((char)plVar3[1] != '\0') {
        return;
      }
    }
    if (((int)lVar8 == 7) && ((char)plVar3[1] == '\x01')) {
      return;
    }
    uVar12 = 0xfb32e4dcc8287bca;
    uStack_60 = param_1;
    lVar8 = FUN_1400dd6f0(lVar9,0xfb32e4dcc8287bca,0x80383a8198597dc2);
    if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
       (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0))
    goto code_r0x0001408c9fe0;
    lVar8 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x0001408c9fe0;
    lVar8 = *(longlong *)(param_2 + 0x1e0) + lVar8;
    plVar3 = *(longlong **)(lVar8 + 0x10);
    plStack_58 = (longlong *)CONCAT44(plStack_58._4_4_,*(undefined4 *)(param_2 + 0x5b0));
    lVar4 = *plVar3;
    pcVar1 = (char *)(lVar4 + 0x10);
    LOCK();
    bVar14 = *(char *)(lVar4 + 0x10) == '\0';
    if (bVar14) {
      *(char *)(lVar4 + 0x10) = '\x01';
    }
    UNLOCK();
    if (!bVar14) {
      FUN_1428cd210(pcVar1);
    }
    FUN_140c81c70(pcVar1);
    uStack_908 = *(longlong *)(lVar4 + 0x48);
    uStack_938 = *(undefined4 *)(lVar4 + 0x18);
    uStack_934 = *(undefined4 *)(lVar4 + 0x1c);
    uStack_930 = *(undefined4 *)(lVar4 + 0x20);
    uStack_92c = *(undefined4 *)(lVar4 + 0x24);
    uStack_928 = *(undefined4 *)(lVar4 + 0x28);
    uStack_924 = *(undefined4 *)(lVar4 + 0x2c);
    uStack_920 = *(undefined4 *)(lVar4 + 0x30);
    uStack_91c = *(undefined4 *)(lVar4 + 0x34);
    uStack_918 = *(undefined4 *)(lVar4 + 0x38);
    uStack_914 = *(undefined4 *)(lVar4 + 0x3c);
    uStack_910 = *(undefined4 *)(lVar4 + 0x40);
    uStack_90c = *(undefined4 *)(lVar4 + 0x44);
    *(undefined1 *)(lVar4 + 0x4e) = 7;
    uStack_4a = 1;
    FUN_140c81e10();
    LOCK();
    cVar2 = *pcVar1;
    if (cVar2 == '\x01') {
      *pcVar1 = '\0';
    }
    UNLOCK();
    if (cVar2 != '\x01') {
      uStack_4a = 1;
      FUN_1428cdb90(pcVar1,0);
    }
    if (uStack_908._6_1_ != '\a') {
      lStack_518 = uStack_908;
      uStack_528 = CONCAT44(uStack_914,uStack_918);
      uStack_520 = CONCAT44(uStack_90c,uStack_910);
      uStack_538 = CONCAT44(uStack_924,uStack_928);
      uStack_530 = CONCAT44(uStack_91c,uStack_920);
      uStack_548 = CONCAT44(uStack_934,uStack_938);
      uStack_540 = CONCAT44(uStack_92c,uStack_930);
      *(undefined4 *)(lVar8 + 0x5c) = plStack_58._0_4_;
      plStack_58 = plVar3 + 1;
      if (*(char *)((longlong)plVar3 + 0x3e) != '\a') {
        func_0x00014003bb80(plStack_58);
        FUN_140be24b0(plStack_58);
      }
      plStack_58[6] = lStack_518;
      plStack_58[4] = uStack_528;
      plStack_58[5] = uStack_520;
      plStack_58[2] = uStack_538;
      plStack_58[3] = uStack_530;
      *(undefined4 *)plStack_58 = (undefined4)uStack_548;
      *(undefined4 *)((longlong)plStack_58 + 4) = uStack_548._4_4_;
      *(undefined4 *)(plStack_58 + 1) = (undefined4)uStack_540;
      *(undefined4 *)((longlong)plStack_58 + 0xc) = uStack_540._4_4_;
    }
    uVar6 = uStack_60;
    if (*(char *)((longlong)plVar3 + 0x3e) == '\a') {
      return;
    }
    uVar12 = 0x8597ee0727b461b3;
    lVar9 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar9 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
       (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0))
    goto code_r0x0001408c9fec;
    lVar9 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) == 0)
    goto code_r0x0001408c9fec;
    puVar13 = &UNK_142aa31e0;
    uVar12 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10),
                           &UNK_142aa31e0,0x18);
    if ((uVar12 & 1) != 0) {
      uVar10 = func_0x000140bd5790(param_3 + 4);
      uStack_548 = CONCAT44(uStack_548._4_4_,2);
      uStack_160 = 2;
      uStack_540 = param_3;
      FUN_140867970(&uStack_938,&UNK_142aa3210,7,&uStack_548,1,&uStack_160,uVar10,param_2,uVar6);
      uStack_49 = 1;
      uVar7 = func_0x000140be6750();
      uStack_49 = 1;
      plVar11 = (longlong *)FUN_1400d9b20(0);
      if (plVar11 != (longlong *)0x0) {
        uStack_498 = 0;
        uStack_497 = 0;
        uStack_49c = 0x3f800000;
        uStack_548 = 0;
        uStack_538 = 0;
        uStack_530 = 4;
        uStack_528 = 0;
        uStack_4b8 = (undefined4)*plVar11;
        uStack_4b4 = *(undefined4 *)((longlong)plVar11 + 4);
        uStack_4b0 = (undefined4)plVar11[1];
        uStack_4ac = *(undefined4 *)((longlong)plVar11 + 0xc);
        *plVar11 = *plVar11 + 1;
        uStack_4c8 = _UNK_142a96730;
        uStack_4c0 = _UNK_142a96738;
        uStack_4d8 = _UNK_142a96720;
        uStack_4d4 = _UNK_142a96724;
        uStack_4d0 = _UNK_142a96728;
        uStack_4cc = _UNK_142a9672c;
        uStack_508 = 0x100000003;
        uStack_520 = 0;
        lStack_518 = 1;
        uStack_510 = 0;
        uStack_49 = 1;
        puStack_4a8 = puVar13;
        uStack_4a0 = uVar7;
        FUN_140905be0(&uStack_160,&uStack_548,&uStack_938,0);
        plStack_68 = plVar3 + 1;
        uStack_548 = CONCAT44(uStack_548._4_4_,2);
        FUN_1408fd6c0(&plStack_68,&uStack_160,0x11ba5697cc40b3dd,0x1d,&uStack_548);
        uVar6 = CONCAT44(uStack_15c,uStack_160);
        func_0x000142923c00(&uStack_540,auStack_158,0xa0);
        uStack_4a0._1_3_ = (undefined3)uStack_b7;
        uStack_49c = (undefined4)((ulonglong)uStack_b7 >> 0x18);
        uStack_498 = (undefined1)((ulonglong)uStack_b7 >> 0x38);
        uStack_497 = (undefined1)uStack_af;
        uStack_496 = (undefined7)((ulonglong)uStack_af >> 8);
        uStack_48f = uStack_a7;
        uStack_487 = uStack_9f;
        uStack_47f = uStack_97;
        uStack_477 = uStack_8f;
        uStack_46f = uStack_87;
        uStack_460 = uStack_78;
        uStack_45c = uStack_74;
        uStack_458 = uStack_70;
        uStack_454 = uStack_6c;
        uStack_4a0 = CONCAT31(uStack_4a0._1_3_,uStack_b8) & 0xffffff7f;
        uStack_49 = 1;
        uStack_548 = uVar6;
        FUN_140906a90(&uStack_548,3);
        func_0x000142923c00(&uStack_548,&uStack_938,1000);
        uStack_49 = 0;
        FUN_140868370(&uStack_548);
        return;
      }
      goto code_r0x0001408ca004;
    }
  }
  FUN_1428d9310(&UNK_142aa31f8);
code_r0x0001408ca004:
  uStack_49 = 1;
  FUN_1428d80d0(&UNK_142c3aaa0);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

