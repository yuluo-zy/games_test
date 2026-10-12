
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409c3a80(undefined8 param_1,longlong param_2,undefined8 param_3,longlong param_4,
                  longlong param_5,undefined8 param_6,undefined8 param_7,longlong param_8)

{
  ulonglong *puVar1;
  longlong lVar2;
  code *pcVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong uVar8;
  undefined1 auStack_a38 [1000];
  undefined8 uStack_650;
  undefined8 auStack_648 [20];
  byte bStack_5a8;
  undefined2 uStack_5a7;
  byte bStack_5a5;
  undefined8 uStack_5a4;
  undefined8 uStack_59c;
  undefined4 uStack_594;
  undefined4 uStack_590;
  undefined4 uStack_58c;
  undefined4 uStack_588;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined4 uStack_578;
  undefined4 uStack_574;
  undefined4 uStack_570;
  undefined4 uStack_56c;
  undefined4 uStack_568;
  undefined4 uStack_564;
  undefined4 uStack_560;
  undefined4 uStack_55c;
  longlong lStack_268;
  undefined1 auStack_260 [160];
  byte bStack_1c0;
  undefined2 uStack_1bf;
  byte bStack_1bd;
  undefined8 uStack_1bc;
  undefined8 uStack_1b4;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined8 uStack_170;
  longlong lStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined2 uStack_c0;
  longlong lStack_78;
  longlong lStack_70;
  float fStack_68;
  undefined1 uStack_64;
  longlong lStack_60;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar5 = func_0x000140bd5790(param_4 + 4);
  uStack_650 = (ulonglong)uStack_650._4_4_ << 0x20;
  uStack_170 = 1;
  uStack_160 = 0;
  auStack_648[0] = param_3;
  lStack_168 = param_4;
  lStack_70 = param_4;
  FUN_140867970(auStack_a38,&UNK_142abfec6,0x14,&uStack_650,1,&uStack_170,uVar5,param_2,param_1);
  uStack_49 = 1;
  uVar8 = 0x4bc362195b80f729;
  lStack_58 = param_2;
  lVar6 = FUN_1400dd6f0(param_2 + 0x38,0x4bc362195b80f729,0x982798ce80cbcefb);
  if (((lVar6 != 0) && (uVar8 < *(ulonglong *)(lStack_58 + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(lStack_58 + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar6 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(lStack_58 + 0x1e0) + 0x20 + lVar6) != 0) {
      puVar1 = *(ulonglong **)(*(longlong *)(lStack_58 + 0x1e0) + lVar6 + 0x10);
      fStack_68 = (float)(int)puVar1[1] / _UNK_1429a7640 + (float)*puVar1;
      uStack_64 = 0;
      lVar6 = *(longlong *)(param_8 + 0x188);
      lStack_78 = *(longlong *)(param_8 + 400) * 0x80 + lVar6;
      if (lVar6 != lStack_78) {
        do {
          lVar2 = *(longlong *)(lVar6 + 0x50);
          uVar5 = *(undefined8 *)(lVar6 + 0x58);
          uStack_49 = 1;
          uVar4 = func_0x000140be6750();
          uStack_49 = 1;
          plVar7 = (longlong *)FUN_1400d9b20(0);
          if (plVar7 == (longlong *)0x0) {
            uStack_49 = 1;
            FUN_1428d80d0(&UNK_142c3aaa0);
            goto code_r0x0001409c3f6e;
          }
          uStack_c0 = 0;
          uStack_c4 = 0x3f800000;
          uStack_170 = 1;
          uStack_160 = 0;
          uStack_158 = 4;
          uStack_150 = 0;
          uStack_e0 = (undefined4)*plVar7;
          uStack_dc = *(undefined4 *)((longlong)plVar7 + 4);
          uStack_d8 = (undefined4)plVar7[1];
          uStack_d4 = *(undefined4 *)((longlong)plVar7 + 0xc);
          *plVar7 = *plVar7 + 1;
          uStack_f0 = _UNK_142a96730;
          uStack_e8 = _UNK_142a96738;
          uStack_100 = _UNK_142a96720;
          uStack_fc = _UNK_142a96724;
          uStack_f8 = _UNK_142a96728;
          uStack_f4 = _UNK_142a9672c;
          uStack_130 = 0x100000003;
          uStack_148 = 0;
          uStack_140 = 1;
          uStack_138 = 0;
          uStack_49 = 1;
          lStack_168 = lVar2;
          uStack_d0 = uVar5;
          uStack_c8 = uVar4;
          FUN_140905be0(&uStack_650,&uStack_170,auStack_a38,0);
          uStack_49 = 1;
          FUN_14090b460(&uStack_170,&uStack_650,&fStack_68,lVar6);
          lStack_60 = lStack_70;
          uStack_650._0_4_ = 2;
          FUN_1408ee800(&lStack_60,&uStack_170,0x4861579da049fbfc,0x1d,&uStack_650);
          func_0x000142923c00(&uStack_650,&uStack_170,0xf8);
          lStack_60 = param_5;
          uStack_170 = CONCAT44(uStack_170._4_4_,2);
          FUN_1408ee800(&lStack_60,&uStack_650,0x2a333c3a28028230,0x1d,&uStack_170);
          func_0x000142923c00(&lStack_268,&uStack_650,0xf8);
          uStack_170 = param_6;
          uStack_650 = CONCAT44(uStack_650._4_4_,2);
          FUN_1408ee800(&uStack_170,&lStack_268,0x4175185826c09d0,0x1d,&uStack_650);
          lVar2 = lStack_268;
          func_0x000142923c00(auStack_648,auStack_260,0xa0);
          uStack_5a4 = uStack_1bc;
          uStack_59c = uStack_1b4;
          uStack_594 = uStack_1ac;
          uStack_590 = uStack_1a8;
          uStack_58c = uStack_1a4;
          uStack_588 = uStack_1a0;
          uStack_584 = uStack_19c;
          uStack_580 = uStack_198;
          uStack_57c = uStack_194;
          uStack_578 = uStack_190;
          uStack_574 = uStack_18c;
          uStack_570 = uStack_188;
          uStack_56c = uStack_184;
          uStack_568 = uStack_180;
          uStack_564 = uStack_17c;
          uStack_560 = uStack_178;
          uStack_55c = uStack_174;
          bStack_5a8 = bStack_1c0 & 0xbf;
          bStack_5a5 = bStack_1bd & 0x9f | 0x40;
          uStack_650 = lVar2;
          uStack_5a7 = uStack_1bf;
          uStack_49 = 1;
          (**(code **)(lVar6 + 0x70))(&uStack_650,lVar6,lStack_58);
          lVar6 = lVar6 + 0x80;
        } while (lVar6 != lStack_78);
      }
      func_0x000142923c00(&uStack_650,auStack_a38,1000);
      uStack_49 = 0;
      FUN_140868370(&uStack_650);
      return;
    }
  }
  uStack_49 = 1;
  FUN_1428d9310(&UNK_142aa5bd8);
code_r0x0001409c3f6e:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

