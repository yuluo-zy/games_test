
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408a5a70(undefined8 param_1,longlong param_2,longlong param_3)

{
  code *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong *plVar6;
  ulonglong uVar7;
  undefined *puVar8;
  undefined1 auStack_910 [1000];
  undefined8 uStack_528;
  longlong alStack_520 [13];
  undefined4 uStack_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  undefined4 uStack_4ac;
  undefined8 uStack_4a8;
  undefined8 uStack_4a0;
  undefined4 uStack_498;
  undefined4 uStack_494;
  undefined4 uStack_490;
  undefined4 uStack_48c;
  undefined *puStack_488;
  uint uStack_480;
  undefined4 uStack_47c;
  undefined1 uStack_478;
  undefined1 uStack_477;
  undefined7 uStack_476;
  undefined8 uStack_46f;
  undefined8 uStack_467;
  undefined8 uStack_45f;
  undefined8 uStack_457;
  undefined8 uStack_44f;
  undefined4 uStack_440;
  undefined4 uStack_43c;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined1 auStack_138 [160];
  undefined1 uStack_98;
  undefined8 uStack_97;
  undefined8 uStack_8f;
  undefined8 uStack_87;
  undefined8 uStack_7f;
  undefined8 uStack_77;
  undefined8 uStack_6f;
  undefined8 uStack_67;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar4 = param_2 + 0x38;
  uVar7 = 0xd38dae617fb79c7a;
  lVar3 = FUN_1400dd6f0(lVar4,0xd38dae617fb79c7a,0x22aadf5c5dad1eae);
  if (((lVar3 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar7)) ||
     (uVar7 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar7 * 8), uVar7 == 0)) {
code_r0x0001408a5e78:
    FUN_1428d9310(&UNK_142a9b508);
code_r0x0001408a5e84:
    FUN_1428d9310(&UNK_142a9b520);
code_r0x0001408a5e90:
    FUN_1428d9310(&UNK_142a9b538);
  }
  else {
    lVar3 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) == 0)
    goto code_r0x0001408a5e78;
    if (**(int **)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10) != 8) {
      return;
    }
    uVar7 = 0xb7e994d5f6a937ce;
    lVar3 = FUN_1400dd6f0(lVar4,0xb7e994d5f6a937ce,0x629c04e22cbc0520);
    if (((lVar3 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar7)) ||
       (uVar7 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar7 * 8), uVar7 == 0))
    goto code_r0x0001408a5e84;
    lVar3 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) == 0)
    goto code_r0x0001408a5e84;
    lVar3 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10);
    if (*(char *)(lVar3 + 0x44) != '\x01') {
      return;
    }
    if (6 < *(byte *)(lVar3 + 0x3e)) {
      return;
    }
    uVar7 = 0x8597ee0727b461b3;
    lVar4 = FUN_1400dd6f0(lVar4,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar4 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar7)) ||
       (uVar7 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar7 * 8), uVar7 == 0))
    goto code_r0x0001408a5e90;
    lVar4 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar4) == 0)
    goto code_r0x0001408a5e90;
    puVar8 = &UNK_142a9b550;
    uVar7 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar4 + 0x10),
                          &UNK_142a9b550,0x18);
    if ((uVar7 & 1) != 0) {
      uVar5 = func_0x000140bd5790(param_3 + 4);
      uStack_528 = CONCAT44(uStack_528._4_4_,2);
      uStack_140 = 2;
      alStack_520[0] = param_3;
      FUN_140867970(auStack_910,&UNK_142a9b580,0x12,&uStack_528,1,&uStack_140,uVar5,param_2,param_1)
      ;
      uStack_39 = 1;
      uVar2 = func_0x000140be6750();
      uStack_39 = 1;
      plVar6 = (longlong *)FUN_1400d9b20(0);
      if (plVar6 != (longlong *)0x0) {
        uStack_478 = 0;
        uStack_477 = 0;
        uStack_47c = 0x3f800000;
        uStack_528 = 0;
        alStack_520[1] = 0;
        alStack_520[2] = 4;
        alStack_520[3] = 0;
        uStack_498 = (undefined4)*plVar6;
        uStack_494 = *(undefined4 *)((longlong)plVar6 + 4);
        uStack_490 = (undefined4)plVar6[1];
        uStack_48c = *(undefined4 *)((longlong)plVar6 + 0xc);
        *plVar6 = *plVar6 + 1;
        uStack_4a8 = _UNK_142a96730;
        uStack_4a0 = _UNK_142a96738;
        uStack_4b8 = _UNK_142a96720;
        uStack_4b4 = _UNK_142a96724;
        uStack_4b0 = _UNK_142a96728;
        uStack_4ac = _UNK_142a9672c;
        alStack_520[7] = 0x100000003;
        alStack_520[4] = 0;
        alStack_520[5] = 1;
        alStack_520[6] = 0;
        uStack_39 = 1;
        puStack_488 = puVar8;
        uStack_480 = uVar2;
        FUN_140905be0(&uStack_140,&uStack_528,auStack_910,0);
        lStack_48 = lVar3 + 8;
        uStack_528 = CONCAT44(uStack_528._4_4_,2);
        FUN_1408fd6c0(&lStack_48,&uStack_140,0x11ba5697cc40b3dd,0x1d,&uStack_528);
        uVar5 = CONCAT44(uStack_13c,uStack_140);
        func_0x000142923c00(alStack_520,auStack_138,0xa0);
        uStack_480._1_3_ = (undefined3)uStack_97;
        uStack_47c = (undefined4)((ulonglong)uStack_97 >> 0x18);
        uStack_478 = (undefined1)((ulonglong)uStack_97 >> 0x38);
        uStack_477 = (undefined1)uStack_8f;
        uStack_476 = (undefined7)((ulonglong)uStack_8f >> 8);
        uStack_46f = uStack_87;
        uStack_467 = uStack_7f;
        uStack_45f = uStack_77;
        uStack_457 = uStack_6f;
        uStack_44f = uStack_67;
        uStack_440 = uStack_58;
        uStack_43c = uStack_54;
        uStack_438 = uStack_50;
        uStack_434 = uStack_4c;
        uStack_480 = CONCAT31(uStack_480._1_3_,uStack_98) & 0xffffff7f;
        uStack_39 = 1;
        uStack_528 = uVar5;
        FUN_140906a90(&uStack_528,3);
        func_0x000142923c00(&uStack_528,auStack_910,1000);
        uStack_39 = 0;
        FUN_140868370(&uStack_528);
        return;
      }
      goto code_r0x0001408a5ea8;
    }
  }
  FUN_1428d9310(&UNK_142a9b568);
code_r0x0001408a5ea8:
  uStack_39 = 1;
  FUN_1428d80d0(&UNK_142c3aaa0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

