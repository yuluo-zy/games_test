
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409ccb20(undefined8 param_1,longlong param_2,longlong param_3,longlong param_4)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  code *pcVar3;
  uint uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined1 auStack_928 [1000];
  undefined4 auStack_540 [2];
  longlong lStack_538;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  uint uStack_b0;
  undefined4 uStack_ac;
  undefined2 uStack_a8;
  float fStack_60;
  undefined1 uStack_5c;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar5 = func_0x000140bd5790(param_3 + 4);
  auStack_540[0] = 0;
  uStack_158 = CONCAT44(uStack_158._4_4_,2);
  lStack_538 = param_3;
  FUN_140867970(auStack_928,&UNK_142ac0ce7,9,auStack_540,1,&uStack_158,uVar5,param_2,param_1);
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
      uStack_5c = 1;
      lVar6 = *(longlong *)(param_4 + 0x1e8);
      lVar9 = *(longlong *)(param_4 + 0x1f0) * 0x80 + lVar6;
      while( true ) {
        if (lVar6 == lVar9) {
          func_0x000142923c00(auStack_540,auStack_928,1000);
          uStack_49 = 0;
          FUN_140868370(auStack_540);
          return;
        }
        uVar5 = *(undefined8 *)(lVar6 + 0x50);
        uVar2 = *(undefined8 *)(lVar6 + 0x58);
        uStack_49 = 1;
        uVar4 = func_0x000140be6750();
        uStack_49 = 1;
        plVar7 = (longlong *)FUN_1400d9b20(0);
        if (plVar7 == (longlong *)0x0) break;
        uStack_a8 = 0;
        uStack_ac = 0x3f800000;
        uStack_158 = 1;
        uStack_148 = 0;
        uStack_140 = 4;
        uStack_138 = 0;
        uStack_c8 = (undefined4)*plVar7;
        uStack_c4 = *(undefined4 *)((longlong)plVar7 + 4);
        uStack_c0 = (undefined4)plVar7[1];
        uStack_bc = *(undefined4 *)((longlong)plVar7 + 0xc);
        *plVar7 = *plVar7 + 1;
        uStack_d8 = _UNK_142a96730;
        uStack_d0 = _UNK_142a96738;
        uStack_e8 = _UNK_142a96720;
        uStack_e4 = _UNK_142a96724;
        uStack_e0 = _UNK_142a96728;
        uStack_dc = _UNK_142a9672c;
        uStack_118 = 0x100000003;
        uStack_130 = 0;
        uStack_128 = 1;
        uStack_120 = 0;
        uStack_49 = 1;
        uStack_150 = uVar5;
        uStack_b8 = uVar2;
        uStack_b0 = uVar4;
        FUN_140905be0(auStack_540,&uStack_158,auStack_928,0);
        uStack_49 = 1;
        FUN_14090b460(&uStack_158,auStack_540,&fStack_60);
        uStack_b0 = uStack_b0 & 0xffffff7f | 0x80000000;
        uStack_ac = 0x3f800000;
        pcVar3 = *(code **)(lVar6 + 0x70);
        func_0x000142923c00(auStack_540,&uStack_158,0xf8);
        uStack_49 = 1;
        (*pcVar3)(auStack_540,lVar6,lStack_58);
        lVar6 = lVar6 + 0x80;
      }
      uStack_49 = 1;
      FUN_1428d80d0(&UNK_142c3aaa0);
      goto code_r0x0001409cce77;
    }
  }
  uStack_49 = 1;
  FUN_1428d9310(&UNK_142aa5bd8);
code_r0x0001409cce77:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

