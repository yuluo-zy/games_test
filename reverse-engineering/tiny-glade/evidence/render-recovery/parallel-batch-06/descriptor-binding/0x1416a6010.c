
/* WARNING: Possible PIC construction at 0x0001416a62d2: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001416a63bf: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001416a64bf: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001416a64c4) */

void FUN_1416a6010(longlong param_1,longlong param_2,longlong param_3,uint param_4,
                  undefined8 param_5,longlong *param_6)

{
  undefined4 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  byte bVar4;
  uint uVar5;
  uint *puVar6;
  ulonglong uVar7;
  code *pcVar8;
  char cVar9;
  undefined8 uVar10;
  longlong **pplVar11;
  longlong **pplVar12;
  uint *puVar13;
  undefined8 *unaff_RBP;
  longlong **pplVar14;
  longlong ***unaff_RSI;
  longlong lVar15;
  longlong **pplVar16;
  undefined *apuStack_200 [5];
  undefined8 uStack_1d8;
  longlong *plStack_1d0;
  undefined4 uStack_1c8;
  longlong lStack_1c0;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined1 auStack_170 [16];
  longlong ***ppplStack_160;
  longlong **pplStack_158;
  longlong *plStack_150;
  longlong lStack_148;
  undefined8 uStack_140;
  longlong lStack_138;
  longlong lStack_130;
  longlong lStack_128;
  longlong lStack_120;
  longlong lStack_118;
  undefined8 uStack_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  longlong **pplStack_f0;
  uint *puStack_e8;
  longlong lStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong *plStack_c8;
  undefined1 *puStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  uint uStack_a4;
  longlong lStack_a0;
  longlong lStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  longlong lStack_80;
  longlong *plStack_78;
  longlong **pplStack_70;
  longlong lStack_68;
  ulonglong uStack_60;
  longlong lStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  puVar3 = &uStack_178;
  uStack_48 = 0xfffffffffffffffe;
  lStack_80 = param_1;
  if ((ulonglong)param_4 < *(ulonglong *)(param_3 + 0x10)) {
    plStack_78 = param_6;
    uStack_140 = *(undefined8 *)(param_2 + 0x48);
    lVar15 = (ulonglong)param_4 * 0x78 + *(longlong *)(param_3 + 8);
    uStack_49 = 1;
    apuStack_200[0] = &UNK_1416a608c;
    uStack_a4 = param_4;
    func_0x000140d4a3b0(auStack_170);
    uStack_4a = 1;
    apuStack_200[0] = &UNK_1416a60b1;
    lStack_138 = param_3;
    lStack_148 = FUN_140be5a40(lVar15,lStack_80 + 0x78,param_2 + 0x18);
    pplStack_70 = (longlong **)plStack_78[2];
    puVar6 = (uint *)plStack_78[3];
    lStack_68 = plStack_78[4];
    uStack_4b = 1;
    apuStack_200[0] = &UNK_1416a60fa;
    lStack_130 = lStack_148;
    FUN_1417c69c0(&lStack_128,param_5,0,auStack_170);
    uStack_110 = 0;
    uStack_4c = 1;
    apuStack_200[0] = &UNK_1416a6122;
    FUN_1417c6af0(&lStack_a0,param_5,0,auStack_170);
    uStack_88 = 0;
    lStack_100 = *plStack_78;
    lStack_f8 = plStack_78[1];
    pplStack_f0 = pplStack_70;
    lStack_e0 = lStack_68;
    uStack_d8 = (undefined4)plStack_78[5];
    uStack_d4 = *(undefined4 *)((longlong)plStack_78 + 0x2c);
    uStack_d0 = (undefined4)plStack_78[6];
    uStack_cc = *(undefined4 *)((longlong)plStack_78 + 0x34);
    plStack_c8 = &lStack_148;
    unaff_RSI = &pplStack_158;
    lStack_108 = lVar15;
    puStack_e8 = puVar6;
    puStack_c0 = auStack_170;
    plStack_b8 = &lStack_128;
    do {
      plStack_b0 = &lStack_108;
      unaff_RBP = puVar3;
      if (pplStack_f0 != (longlong **)0x0) {
        pplStack_158 = &plStack_b0;
        plStack_150 = (longlong *)&uStack_d0;
        puVar6 = (uint *)CONCAT44(uStack_d4,uStack_d8);
        puVar13 = puStack_e8;
        if (puStack_e8 == puVar6) {
code_r0x0001416a62bc:
          if (lStack_e0 != 0) {
            pplVar12 = (longlong **)(lStack_e0 << 3);
            pplVar16 = (longlong **)0x4;
            register0x00000020 = (BADSPACEBASE *)apuStack_200;
            apuStack_200[0] = &UNK_1416a62d7;
            pplVar11 = pplStack_f0;
            goto code_r0x000140613c20;
          }
        }
        else {
          do {
            uVar5 = *puVar13;
            bVar4 = (byte)puVar13[1];
            puVar13 = puVar13 + 2;
            uVar7 = *(ulonglong *)(*plStack_150 + 0x10);
            puStack_e8 = puVar13;
            if (uVar7 <= bVar4) {
              apuStack_200[0] = &UNK_1416a64ec;
              FUN_1428d9518((ulonglong)bVar4,uVar7,&UNK_142c77990);
                    /* WARNING: Does not return */
              pcVar8 = (code *)invalidInstructionException();
              (*pcVar8)();
            }
            lStack_58 = (ulonglong)bVar4 * 0x20 + *(longlong *)(*plStack_150 + 8);
            uStack_60 = (ulonglong)uVar5;
            apuStack_200[0] = &UNK_1416a6293;
            cVar9 = FUN_140b05f50(unaff_RSI,&uStack_60);
            if ((cVar9 != '\0') && (lStack_58 != 0)) goto code_r0x0001416a6310;
          } while (puVar13 != puVar6);
          if (pplStack_f0 != (longlong **)0x0) goto code_r0x0001416a62bc;
        }
        pplStack_f0 = (longlong **)0x0;
      }
      if (lStack_100 == 0) goto code_r0x0001416a63c4;
      apuStack_200[0] = &UNK_1416a6300;
      FUN_141479b60(&uStack_60,&lStack_100,plStack_b0);
      if (lStack_58 == 0) goto code_r0x0001416a639d;
code_r0x0001416a6310:
      apuStack_200[0] = &UNK_1416a6333;
      FUN_140b06020(&uStack_1b0,&plStack_c8,&uStack_60);
      if (uStack_88 == uStack_90) {
        apuStack_200[0] = &UNK_1416a635c;
        FUN_1417c6c10(&lStack_a0,uStack_88,1);
      }
      lVar15 = uStack_88 * 0x40;
      puVar2 = (undefined8 *)(lStack_a0 + 0x30 + lVar15);
      *puVar2 = uStack_180;
      puVar2[1] = uStack_178;
      puVar2 = (undefined8 *)(lStack_a0 + 0x20 + lVar15);
      *puVar2 = uStack_190;
      puVar2[1] = uStack_188;
      puVar2 = (undefined8 *)(lStack_a0 + 0x10 + lVar15);
      *puVar2 = uStack_1a0;
      puVar2[1] = uStack_198;
      puVar1 = (undefined4 *)(lStack_a0 + lVar15);
      *puVar1 = uStack_1b0;
      puVar1[1] = uStack_1ac;
      puVar1[2] = uStack_1a8;
      puVar1[3] = uStack_1a4;
      uStack_88 = uStack_88 + 1;
    } while( true );
  }
  pplVar11 = (longlong **)param_6[2];
  if ((pplVar11 == (longlong **)0x0) || (param_6[4] == 0)) {
    return;
  }
  pplVar12 = (longlong **)(param_6[4] << 3);
  pplVar16 = (longlong **)0x4;
code_r0x000140613c20:
  *(undefined8 **)((longlong)register0x00000020 + -8) = unaff_RBP;
  *(longlong ****)((longlong)register0x00000020 + -0x10) = unaff_RSI;
  pplVar14 = pplVar11;
  if ((longlong **)0x10 < pplVar16) {
    pplVar14 = (longlong **)pplVar11[-1];
  }
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar10 = GetProcessHeap(pplVar11,pplVar12);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar10,0,pplVar14);
  return;
code_r0x0001416a639d:
  if ((pplStack_f0 == (longlong **)0x0) || (lStack_e0 == 0)) {
code_r0x0001416a63c4:
    lVar15 = lStack_80;
    uStack_1d8 = 8;
    apuStack_200[0] = &UNK_1416a63f1;
    (**(code **)(lStack_80 + 0x268))
              (*(undefined8 *)(lStack_80 + 0x640),uStack_88 & 0xffffffff,lStack_a0,0);
    LOCK();
    *(longlong *)(lVar15 + 0xab0) = *(longlong *)(lVar15 + 0xab0) + 1;
    UNLOCK();
    if (uStack_90 != 0) {
      lVar15 = *(longlong *)(*(longlong *)(lStack_98 + 0x10) + 0x20);
      if (lVar15 == lStack_a0) {
        *(ulonglong *)(*(longlong *)(lStack_98 + 0x10) + 0x20) = lVar15 + uStack_90 * 0x40;
      }
    }
    lStack_108 = lStack_130;
    lStack_1c0 = lStack_128;
    uStack_1c8 = (undefined4)uStack_110;
    plStack_1d0 = &lStack_108;
    uStack_1d8 = CONCAT44(uStack_1d8._4_4_,1);
    apuStack_200[0] = &UNK_1416a647d;
    (**(code **)(lStack_80 + 0x328))
              (uStack_140,*(undefined4 *)(lStack_138 + 0x30),*(undefined8 *)(lStack_138 + 0x20),
               uStack_a4);
    if (lStack_118 != 0) {
      lVar15 = *(longlong *)(*(longlong *)(lStack_120 + 0x10) + 0x20);
      if (lVar15 == lStack_128) {
        *(longlong *)(*(longlong *)(lStack_120 + 0x10) + 0x20) = lVar15 + lStack_118 * 4;
      }
    }
    if (ppplStack_160 == ppplRam000000014362c3e0) {
      return;
    }
    pplVar11 = *ppplStack_160;
    pplVar16 = ppplStack_160[1];
    pplVar12 = ppplStack_160[2];
    register0x00000020 = (BADSPACEBASE *)apuStack_200;
    apuStack_200[0] = &UNK_1416a64c4;
    unaff_RSI = ppplRam000000014362c3e0;
  }
  else {
    pplVar12 = (longlong **)(lStack_e0 << 3);
    pplVar16 = (longlong **)0x4;
    register0x00000020 = (BADSPACEBASE *)apuStack_200;
    apuStack_200[0] = &UNK_1416a63c4;
    pplVar11 = pplStack_f0;
  }
  goto code_r0x000140613c20;
}

