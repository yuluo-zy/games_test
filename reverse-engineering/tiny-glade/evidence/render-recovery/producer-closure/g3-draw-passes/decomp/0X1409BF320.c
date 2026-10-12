// FUN_1409bf320  0x1409bf320

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409bf320(undefined8 param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
                  longlong param_5,longlong param_6)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  code *pcVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong uVar8;
  undefined1 auStack_9f0 [1000];
  undefined4 uStack_608;
  undefined4 uStack_604;
  undefined4 uStack_600;
  undefined8 uStack_5fc;
  undefined8 uStack_5f0;
  byte bStack_560;
  undefined8 uStack_55f;
  undefined8 uStack_557;
  undefined4 uStack_54f;
  undefined4 uStack_54b;
  undefined4 uStack_547;
  undefined4 uStack_543;
  undefined4 uStack_53f;
  undefined4 uStack_53b;
  undefined4 uStack_537;
  undefined4 uStack_533;
  undefined4 uStack_52f;
  undefined4 uStack_52b;
  undefined4 uStack_527;
  undefined4 uStack_51c;
  undefined4 uStack_518;
  undefined4 uStack_514;
  undefined8 uStack_220;
  longlong alStack_218 [20];
  byte bStack_178;
  undefined8 uStack_177;
  undefined8 uStack_16f;
  undefined4 uStack_167;
  undefined4 uStack_163;
  undefined4 uStack_15f;
  undefined4 uStack_15b;
  undefined4 uStack_157;
  undefined4 uStack_153;
  undefined4 uStack_14f;
  undefined4 uStack_14b;
  undefined4 uStack_147;
  undefined4 uStack_143;
  undefined4 uStack_13f;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined2 uStack_78;
  longlong lStack_70;
  undefined8 uStack_68;
  float fStack_60;
  undefined1 uStack_5c;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_604 = 0;
  uStack_600 = 0;
  uStack_5fc = 0;
  uStack_68 = param_4;
  uVar5 = func_0x000140bd5790(param_5 + 4);
  uStack_608 = 1;
  uStack_220 = 1;
  alStack_218[0] = param_5;
  alStack_218[1] = 0;
  uStack_5f0 = param_3;
  FUN_140867970(auStack_9f0,&UNK_142abfbdb,0x16,&uStack_608,1,&uStack_220,uVar5,param_2,param_1);
  uStack_49 = 1;
  uVar8 = 0x4bc362195b80f729;
  lStack_58 = param_2;
  lVar6 = FUN_1400dd6f0(param_2 + 0x38,0x4bc362195b80f729,0x982798ce80cbcefb);
  if (((lVar6 != 0) && (uVar8 < *(ulonglong *)(lStack_58 + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(lStack_58 + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar6 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(lStack_58 + 0x1e0) + 0x20 + lVar6) != 0) {
      puVar1 = *(ulonglong **)(*(longlong *)(lStack_58 + 0x1e0) + lVar6 + 0x10);
      fStack_60 = (float)(int)puVar1[1] / _UNK_1429a7640 + (float)*puVar1;
      uStack_5c = 0;
      lVar6 = *(longlong *)(param_6 + 0x158);
      lStack_70 = *(longlong *)(param_6 + 0x160) * 0x80 + lVar6;
      if (lVar6 != lStack_70) {
        do {
          uVar5 = *(undefined8 *)(lVar6 + 0x50);
          uVar2 = *(undefined8 *)(lVar6 + 0x58);
          uStack_49 = 1;
          uVar4 = func_0x000140be6750();
          uStack_49 = 1;
          plVar7 = (longlong *)FUN_1400d9b20(0);
          if (plVar7 == (longlong *)0x0) {
            uStack_49 = 1;
            FUN_1428d80d0(&UNK_142c3aaa0);
            goto code_r0x0001409bf759;
          }
          uStack_78 = 0;
          uStack_7c = 0x3f800000;
          uStack_128 = 1;
          uStack_118 = 0;
          uStack_110 = 4;
          uStack_108 = 0;
          uStack_98 = (undefined4)*plVar7;
          uStack_94 = *(undefined4 *)((longlong)plVar7 + 4);
          uStack_90 = (undefined4)plVar7[1];
          uStack_8c = *(undefined4 *)((longlong)plVar7 + 0xc);
          *plVar7 = *plVar7 + 1;
          uStack_a8 = _UNK_142a96730;
          uStack_a0 = _UNK_142a96738;
          uStack_b8 = _UNK_142a96720;
          uStack_b4 = _UNK_142a96724;
          uStack_b0 = _UNK_142a96728;
          uStack_ac = _UNK_142a9672c;
          uStack_e8 = 0x100000003;
          uStack_100 = 0;
          uStack_f8 = 1;
          uStack_f0 = 0;
          uStack_49 = 1;
          uStack_120 = uVar5;
          uStack_88 = uVar2;
          uStack_80 = uVar4;
          FUN_140905be0(&uStack_608,&uStack_128,auStack_9f0,0);
          uStack_49 = 1;
          FUN_14090b460(&uStack_220,&uStack_608,&fStack_60,lVar6);
          uStack_128 = uStack_68;
          uStack_608 = 2;
          FUN_1408ee800(&uStack_128,&uStack_220,0xd48bea13951c1ddc,0x1d,&uStack_608);
          uVar5 = uStack_220;
          func_0x000142923c00(&uStack_600,alStack_218,0xa0);
          uStack_55f = uStack_177;
          uStack_557 = uStack_16f;
          uStack_54f = uStack_167;
          uStack_54b = uStack_163;
          uStack_547 = uStack_15f;
          uStack_543 = uStack_15b;
          uStack_53f = uStack_157;
          uStack_53b = uStack_153;
          uStack_537 = uStack_14f;
          uStack_533 = uStack_14b;
          uStack_52f = uStack_147;
          uStack_52b = uStack_143;
          uStack_527 = uStack_13f;
          uStack_51c = uStack_134;
          uStack_518 = uStack_130;
          uStack_514 = uStack_12c;
          bStack_560 = bStack_178 & 0xbf;
          uStack_608 = (undefined4)uVar5;
          uStack_604 = (undefined4)((ulonglong)uVar5 >> 0x20);
          uStack_49 = 1;
          (**(code **)(lVar6 + 0x70))(&uStack_608,lVar6,lStack_58);
          lVar6 = lVar6 + 0x80;
        } while (lVar6 != lStack_70);
      }
      func_0x000142923c00(&uStack_608,auStack_9f0,1000);
      uStack_49 = 0;
      FUN_140868370(&uStack_608);
      return;
    }
  }
  uStack_49 = 1;
  FUN_1428d9310(&UNK_142aa5bd8);
code_r0x0001409bf759:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

