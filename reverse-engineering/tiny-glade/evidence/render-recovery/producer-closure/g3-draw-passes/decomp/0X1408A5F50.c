// FUN_1408a5f50  0x1408a5f50

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408a5f50(undefined8 param_1,longlong param_2,longlong param_3)

{
  code *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong *plVar5;
  ulonglong uVar6;
  undefined *puVar7;
  undefined1 auStack_910 [1000];
  ulonglong uStack_528;
  longlong alStack_520 [20];
  byte bStack_480;
  undefined2 uStack_47f;
  byte bStack_47d;
  undefined8 uStack_47c;
  undefined8 uStack_474;
  undefined8 uStack_46c;
  undefined8 uStack_464;
  undefined8 uStack_45c;
  undefined8 uStack_454;
  undefined8 uStack_44c;
  undefined4 uStack_43c;
  undefined4 uStack_438;
  undefined4 uStack_434;
  ulonglong uStack_140;
  undefined1 auStack_138 [8];
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined *puStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined2 uStack_90;
  undefined2 uStack_8e;
  undefined8 uStack_8c;
  undefined8 uStack_84;
  undefined8 uStack_7c;
  undefined8 uStack_74;
  undefined8 uStack_6c;
  undefined8 uStack_64;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uVar6 = 0x8597ee0727b461b3;
  lVar3 = FUN_1400dd6f0(param_2 + 0x38,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
  if (((lVar3 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar6)) ||
     (uVar6 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar6 * 8), uVar6 == 0)) {
code_r0x0001408a62c0:
    FUN_1428d9310(&UNK_142a9b600);
  }
  else {
    lVar3 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) == 0)
    goto code_r0x0001408a62c0;
    puVar7 = &UNK_142a9b5ab;
    uVar6 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10),
                          &UNK_142a9b5ab,0xc);
    if ((uVar6 & 1) != 0) {
      uVar4 = func_0x000140bd5790(param_3 + 4);
      uStack_528 = uStack_528 & 0xffffffff00000000;
      uStack_140 = CONCAT44(uStack_140._4_4_,2);
      alStack_520[0] = param_3;
      FUN_140867970(auStack_910,&UNK_142a9b630,0xc,&uStack_528,1,&uStack_140,uVar4,param_2,param_1);
      uStack_39 = 1;
      uVar2 = func_0x000140be6750();
      uStack_39 = 1;
      plVar5 = (longlong *)FUN_1400d9b20(0);
      if (plVar5 != (longlong *)0x0) {
        uStack_90 = 0;
        uStack_94 = 0x3f800000;
        uStack_140 = 0;
        uStack_130 = 0;
        uStack_128 = 4;
        uStack_120 = 0;
        uStack_b0 = (undefined4)*plVar5;
        uStack_ac = *(undefined4 *)((longlong)plVar5 + 4);
        uStack_a8 = (undefined4)plVar5[1];
        uStack_a4 = *(undefined4 *)((longlong)plVar5 + 0xc);
        *plVar5 = *plVar5 + 1;
        uStack_c0 = _UNK_142a96730;
        uStack_b8 = _UNK_142a96738;
        uStack_d0 = _UNK_142a96720;
        uStack_cc = _UNK_142a96724;
        uStack_c8 = _UNK_142a96728;
        uStack_c4 = _UNK_142a9672c;
        uStack_100 = 0x100000003;
        uStack_118 = 0;
        uStack_110 = 1;
        uStack_108 = 0;
        uStack_39 = 1;
        puStack_a0 = puVar7;
        uStack_98 = uVar2;
        FUN_140905be0(&uStack_528,&uStack_140,auStack_910,0);
        uStack_3a = 1;
        uVar4 = func_0x000140bd5790(param_3 + 4);
        uStack_3a = 0;
        FUN_141629d20(&uStack_140,&uStack_528,0x531bb9318eea4971,uVar4);
        uStack_48 = 0x75b4be0d27cfedd3;
        uStack_528 = CONCAT44(uStack_528._4_4_,2);
        FUN_140908610(&uStack_48,&uStack_140,0x11ba5697cc40b3dd,0x1d,&uStack_528);
        uVar6 = uStack_140;
        func_0x000142923c00(alStack_520,auStack_138,0xa0);
        uStack_47c = CONCAT26(uStack_8e,CONCAT24(uStack_90,uStack_94));
        uStack_474 = uStack_8c;
        uStack_46c = uStack_84;
        uStack_464 = uStack_7c;
        uStack_45c = uStack_74;
        uStack_454 = uStack_6c;
        uStack_44c = uStack_64;
        uStack_43c = uStack_54;
        uStack_438 = uStack_50;
        uStack_434 = uStack_4c;
        bStack_480 = (byte)uStack_98 & 0x7f;
        bStack_47d = uStack_98._3_1_ & 0x9f | 0x40;
        uStack_528 = uVar6;
        uStack_47f = uStack_98._1_2_;
        uStack_39 = 1;
        FUN_140906a90(&uStack_528,6);
        func_0x000142923c00(&uStack_528,auStack_910,1000);
        uStack_39 = 0;
        FUN_140868370(&uStack_528);
        return;
      }
      goto code_r0x0001408a62d8;
    }
  }
  FUN_1428d9310(&UNK_142a9b618);
code_r0x0001408a62d8:
  uStack_39 = 1;
  FUN_1428d80d0(&UNK_142c3aaa0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

