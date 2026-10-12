
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408373b0(longlong param_1,longlong param_2,longlong param_3,longlong param_4,
                  longlong param_5)

{
  ulonglong *puVar1;
  code *pcVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong *plVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  undefined1 auStack_b30 [1000];
  undefined1 auStack_748 [169];
  byte bStack_69f;
  undefined4 auStack_650 [2];
  longlong lStack_648;
  undefined1 uStack_59f;
  undefined1 auStack_268 [169];
  byte bStack_1bf;
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
  longlong lStack_68;
  float fStack_60;
  undefined1 uStack_5c;
  longlong lStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar8 = *(longlong *)(param_5 + 0x130);
  lStack_58 = param_2;
  if (lVar8 == 0) {
code_r0x000140837753:
    lVar9 = lStack_58;
    lVar8 = *(longlong *)(param_5 + 0x148);
    if (lVar8 == 0) {
      return;
    }
    lVar5 = *(longlong *)(param_5 + 0x140);
    uVar4 = func_0x000140bd5790(param_3 + 0x28);
    auStack_650[0] = 0;
    uStack_170 = 0;
    uStack_160 = 0;
    lStack_648 = param_4;
    lStack_168 = param_3 + 0x24;
    FUN_140867970(auStack_b30,&UNK_142a92bb8,0xc,auStack_650,1,&uStack_170,uVar4,lVar9,param_1);
    uStack_4a = 1;
    uVar7 = 0x4bc362195b80f729;
    lVar9 = FUN_1400dd6f0(lVar9 + 0x38,0x4bc362195b80f729,0x982798ce80cbcefb);
    if (((lVar9 != 0) && (uVar7 < *(ulonglong *)(lStack_58 + 0x218))) &&
       (uVar7 = *(ulonglong *)(*(longlong *)(lStack_58 + 0x210) + uVar7 * 8), uVar7 != 0)) {
      lVar9 = ~uVar7 * 0x60;
      if (*(longlong *)(*(longlong *)(lStack_58 + 0x1e0) + 0x20 + lVar9) != 0) {
        puVar1 = *(ulonglong **)(*(longlong *)(lStack_58 + 0x1e0) + lVar9 + 0x10);
        fStack_60 = (float)(int)puVar1[1] / _UNK_1429a7640 + (float)*puVar1;
        uStack_5c = 0;
        lStack_68 = lVar8 * 0x80 + lVar5;
        if (lVar5 != lStack_68) {
          do {
            lVar8 = *(longlong *)(lVar5 + 0x50);
            uVar4 = *(undefined8 *)(lVar5 + 0x58);
            uStack_4a = 1;
            uVar3 = func_0x000140be6750();
            uStack_4a = 1;
            plVar6 = (longlong *)FUN_1400d9b20(0);
            if (plVar6 == (longlong *)0x0) {
              uStack_4a = 1;
              FUN_1428d80d0(&UNK_142c3aaa0);
              goto code_r0x000140837b11;
            }
            uStack_c0 = 0;
            uStack_c4 = 0x3f800000;
            uStack_170 = 1;
            uStack_160 = 0;
            uStack_158 = 4;
            uStack_150 = 0;
            uStack_e0 = (undefined4)*plVar6;
            uStack_dc = *(undefined4 *)((longlong)plVar6 + 4);
            uStack_d8 = (undefined4)plVar6[1];
            uStack_d4 = *(undefined4 *)((longlong)plVar6 + 0xc);
            *plVar6 = *plVar6 + 1;
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
            uStack_4a = 1;
            lStack_168 = lVar8;
            uStack_d0 = uVar4;
            uStack_c8 = uVar3;
            FUN_140905be0(auStack_650,&uStack_170,auStack_b30,0);
            if (*(char *)(lVar5 + 100) == '\0') {
              FUN_1428d9310(&UNK_142a92bc8);
              goto code_r0x000140837b11;
            }
            uStack_59f = *(undefined1 *)(lVar5 + 0x65);
            func_0x000142923c00(&uStack_170,auStack_650,0xf8);
            uStack_4a = 1;
            FUN_14090b460(auStack_748,&uStack_170,&fStack_60);
            pcVar2 = *(code **)(lVar5 + 0x70);
            bStack_69f = bStack_69f & 0x8e | 0x21;
            func_0x000142923c00(auStack_650,auStack_748,0xf8);
            uStack_4a = 1;
            (*pcVar2)(auStack_650,lVar5,lStack_58);
            lVar5 = lVar5 + 0x80;
          } while (lVar5 != lStack_68);
        }
        func_0x000142923c00(auStack_650,auStack_b30,1000);
        uStack_4a = 0;
        FUN_140868370(auStack_650);
        return;
      }
    }
    uStack_4a = 1;
    FUN_1428d9310(&UNK_142aa5bd8);
  }
  else {
    lVar9 = *(longlong *)(param_5 + 0x128);
    lStack_78 = param_4;
    uVar4 = func_0x000140bd5790(param_3 + 0x28);
    auStack_650[0] = 0;
    uStack_170 = 0;
    uStack_160 = 0;
    lStack_648 = param_3;
    lStack_168 = param_3 + 0x24;
    lStack_70 = param_3;
    lStack_68 = param_1;
    FUN_140867970(auStack_b30,&UNK_142a92b90,0xd,auStack_650,1,&uStack_170,uVar4,param_2,param_1);
    uStack_49 = 1;
    uVar7 = 0x4bc362195b80f729;
    lVar5 = FUN_1400dd6f0(param_2 + 0x38,0x4bc362195b80f729,0x982798ce80cbcefb);
    if (((lVar5 != 0) && (uVar7 < *(ulonglong *)(lStack_58 + 0x218))) &&
       (uVar7 = *(ulonglong *)(*(longlong *)(lStack_58 + 0x210) + uVar7 * 8), uVar7 != 0)) {
      lVar5 = ~uVar7 * 0x60;
      if (*(longlong *)(*(longlong *)(lStack_58 + 0x1e0) + 0x20 + lVar5) != 0) {
        puVar1 = *(ulonglong **)(*(longlong *)(lStack_58 + 0x1e0) + lVar5 + 0x10);
        fStack_60 = (float)(int)puVar1[1] / _UNK_1429a7640 + (float)*puVar1;
        uStack_5c = 0;
        lVar8 = lVar8 * 0x80 + lVar9;
        for (; lVar9 != lVar8; lVar9 = lVar9 + 0x80) {
          lVar5 = *(longlong *)(lVar9 + 0x50);
          uVar4 = *(undefined8 *)(lVar9 + 0x58);
          uStack_49 = 1;
          uVar3 = func_0x000140be6750();
          uStack_49 = 1;
          plVar6 = (longlong *)FUN_1400d9b20(0);
          if (plVar6 == (longlong *)0x0) {
            uStack_49 = 1;
            FUN_1428d80d0(&UNK_142c3aaa0);
            goto code_r0x000140837b11;
          }
          uStack_c0 = 0;
          uStack_c4 = 0x3f800000;
          uStack_170 = 1;
          uStack_160 = 0;
          uStack_158 = 4;
          uStack_150 = 0;
          uStack_e0 = (undefined4)*plVar6;
          uStack_dc = *(undefined4 *)((longlong)plVar6 + 4);
          uStack_d8 = (undefined4)plVar6[1];
          uStack_d4 = *(undefined4 *)((longlong)plVar6 + 0xc);
          *plVar6 = *plVar6 + 1;
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
          lStack_168 = lVar5;
          uStack_d0 = uVar4;
          uStack_c8 = uVar3;
          FUN_140905be0(auStack_650,&uStack_170,auStack_b30,0);
          if (*(char *)(lVar9 + 100) == '\0') {
            FUN_1428d9310(&UNK_142a92ba0);
            goto code_r0x000140837b11;
          }
          uStack_59f = *(undefined1 *)(lVar9 + 0x65);
          func_0x000142923c00(&uStack_170,auStack_650,0xf8);
          uStack_49 = 1;
          FUN_14090b460(auStack_268,&uStack_170,&fStack_60);
          pcVar2 = *(code **)(lVar9 + 0x70);
          bStack_1bf = bStack_1bf & 0x8e | 0x21;
          func_0x000142923c00(auStack_650,auStack_268,0xf8);
          uStack_49 = 1;
          (*pcVar2)(auStack_650,lVar9,lStack_58);
        }
        func_0x000142923c00(auStack_650,auStack_b30,1000);
        uStack_49 = 0;
        FUN_140868370(auStack_650);
        param_1 = lStack_68;
        param_3 = lStack_70;
        param_4 = lStack_78;
        goto code_r0x000140837753;
      }
    }
    uStack_49 = 1;
    FUN_1428d9310(&UNK_142aa5bd8);
  }
code_r0x000140837b11:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

