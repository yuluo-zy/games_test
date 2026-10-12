
/* WARNING: Possible PIC construction at 0x0001416a5bcc: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001416a5cb5: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001416a5dbf: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001416a5dc4) */

void FUN_1416a5910(longlong param_1,longlong param_2,longlong param_3,uint param_4,
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
  undefined *apuStack_220 [5];
  undefined8 uStack_1f8;
  longlong *plStack_1f0;
  undefined4 uStack_1e8;
  longlong lStack_1e0;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined1 auStack_188 [16];
  longlong ***ppplStack_178;
  longlong **pplStack_170;
  longlong *plStack_168;
  longlong lStack_160;
  undefined8 uStack_158;
  longlong lStack_150;
  longlong lStack_148;
  longlong lStack_140;
  longlong **pplStack_138;
  uint *puStack_130;
  longlong lStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  longlong lStack_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  longlong lStack_f0;
  longlong *plStack_e8;
  undefined1 *puStack_e0;
  longlong *plStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  undefined8 uStack_b8;
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
  
  puVar3 = &uStack_198;
  uStack_48 = 0xfffffffffffffffe;
  lStack_80 = param_1;
  if ((ulonglong)param_4 < *(ulonglong *)(param_3 + 0x10)) {
    plStack_78 = param_6;
    uStack_158 = *(undefined8 *)(param_2 + 0x48);
    lVar15 = (ulonglong)param_4 * 0x78 + *(longlong *)(param_3 + 8);
    uStack_49 = 1;
    apuStack_220[0] = &UNK_1416a598c;
    uStack_a4 = param_4;
    func_0x000140d4a3b0(auStack_188);
    uStack_4a = 1;
    apuStack_220[0] = &UNK_1416a59b1;
    lStack_150 = param_3;
    lStack_160 = FUN_140be5a40(lVar15,lStack_80 + 0x78,param_2 + 0x18);
    pplStack_70 = (longlong **)*plStack_78;
    puVar6 = (uint *)plStack_78[1];
    lStack_68 = plStack_78[2];
    uStack_4b = 1;
    apuStack_220[0] = &UNK_1416a59fc;
    lStack_148 = lStack_160;
    FUN_1417c69c0(&lStack_d0,param_5,0,auStack_188);
    uStack_b8 = 0;
    uStack_4c = 1;
    apuStack_220[0] = &UNK_1416a5a27;
    FUN_1417c6af0(&lStack_a0,param_5,0,auStack_188);
    uStack_88 = 0;
    pplStack_138 = pplStack_70;
    lStack_128 = lStack_68;
    uStack_120 = (undefined4)plStack_78[3];
    uStack_11c = *(undefined4 *)((longlong)plStack_78 + 0x1c);
    uStack_118 = (undefined4)plStack_78[4];
    uStack_114 = *(undefined4 *)((longlong)plStack_78 + 0x24);
    lStack_110 = plStack_78[5];
    lStack_108 = plStack_78[6];
    lStack_100 = plStack_78[7];
    lStack_f8 = plStack_78[8];
    lStack_f0 = plStack_78[9];
    plStack_e8 = &lStack_160;
    unaff_RSI = &pplStack_170;
    lStack_140 = lVar15;
    puStack_130 = puVar6;
    puStack_e0 = auStack_188;
    plStack_d8 = &lStack_d0;
    do {
      plStack_b0 = &lStack_140;
      unaff_RBP = puVar3;
      if (pplStack_138 != (longlong **)0x0) {
        pplStack_170 = &plStack_b0;
        plStack_168 = (longlong *)&uStack_118;
        puVar6 = (uint *)CONCAT44(uStack_11c,uStack_120);
        puVar13 = puStack_130;
        if (puStack_130 == puVar6) {
code_r0x0001416a5bb9:
          if (lStack_128 != 0) {
            pplVar12 = (longlong **)(lStack_128 << 3);
            pplVar16 = (longlong **)0x4;
            register0x00000020 = (BADSPACEBASE *)apuStack_220;
            apuStack_220[0] = &UNK_1416a5bd1;
            pplVar11 = pplStack_138;
            goto code_r0x000140613c20;
          }
        }
        else {
          do {
            uVar5 = *puVar13;
            bVar4 = (byte)puVar13[1];
            puVar13 = puVar13 + 2;
            uVar7 = *(ulonglong *)(*plStack_168 + 0x10);
            puStack_130 = puVar13;
            if (uVar7 <= bVar4) {
              apuStack_220[0] = &UNK_1416a5dec;
              FUN_1428d9518((ulonglong)bVar4,uVar7,&UNK_142c779e8);
                    /* WARNING: Does not return */
              pcVar8 = (code *)invalidInstructionException();
              (*pcVar8)();
            }
            lStack_58 = (ulonglong)bVar4 * 0x20 + *(longlong *)(*plStack_168 + 8);
            uStack_60 = (ulonglong)uVar5;
            apuStack_220[0] = &UNK_1416a5b90;
            cVar9 = FUN_140b05f50(unaff_RSI,&uStack_60);
            if ((cVar9 != '\0') && (lStack_58 != 0)) goto code_r0x0001416a5c0d;
          } while (puVar13 != puVar6);
          if (pplStack_138 != (longlong **)0x0) goto code_r0x0001416a5bb9;
        }
        pplStack_138 = (longlong **)0x0;
      }
      if (lStack_110 == 0) goto code_r0x0001416a5cba;
      apuStack_220[0] = &UNK_1416a5bfd;
      FUN_14147a0b0(&uStack_60,&lStack_110,plStack_b0);
      if (lStack_58 == 0) goto code_r0x0001416a5c99;
code_r0x0001416a5c0d:
      apuStack_220[0] = &UNK_1416a5c30;
      FUN_140b06020(&uStack_1c8,&plStack_e8,&uStack_60);
      if (uStack_88 == uStack_90) {
        apuStack_220[0] = &UNK_1416a5c59;
        FUN_1417c6c10(&lStack_a0,uStack_88,1);
      }
      lVar15 = uStack_88 * 0x40;
      puVar2 = (undefined8 *)(lStack_a0 + 0x30 + lVar15);
      *puVar2 = uStack_198;
      puVar2[1] = uStack_190;
      puVar2 = (undefined8 *)(lStack_a0 + 0x20 + lVar15);
      *puVar2 = uStack_1a8;
      puVar2[1] = uStack_1a0;
      puVar2 = (undefined8 *)(lStack_a0 + 0x10 + lVar15);
      *puVar2 = uStack_1b8;
      puVar2[1] = uStack_1b0;
      puVar1 = (undefined4 *)(lStack_a0 + lVar15);
      *puVar1 = uStack_1c8;
      puVar1[1] = uStack_1c4;
      puVar1[2] = uStack_1c0;
      puVar1[3] = uStack_1bc;
      uStack_88 = uStack_88 + 1;
    } while( true );
  }
  pplVar11 = (longlong **)*param_6;
  if ((pplVar11 == (longlong **)0x0) || (param_6[2] == 0)) {
    return;
  }
  pplVar12 = (longlong **)(param_6[2] << 3);
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
code_r0x0001416a5c99:
  if ((pplStack_138 == (longlong **)0x0) || (lStack_128 == 0)) {
code_r0x0001416a5cba:
    lVar15 = lStack_80;
    uStack_1f8 = 8;
    apuStack_220[0] = &UNK_1416a5ce7;
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
    lStack_140 = lStack_148;
    lStack_1e0 = lStack_d0;
    uStack_1e8 = (undefined4)uStack_b8;
    plStack_1f0 = &lStack_140;
    uStack_1f8 = CONCAT44(uStack_1f8._4_4_,1);
    apuStack_220[0] = &UNK_1416a5d7f;
    (**(code **)(lStack_80 + 0x328))
              (uStack_158,*(undefined4 *)(lStack_150 + 0x30),*(undefined8 *)(lStack_150 + 0x20),
               uStack_a4);
    if (lStack_c0 != 0) {
      lVar15 = *(longlong *)(*(longlong *)(lStack_c8 + 0x10) + 0x20);
      if (lVar15 == lStack_d0) {
        *(longlong *)(*(longlong *)(lStack_c8 + 0x10) + 0x20) = lVar15 + lStack_c0 * 4;
      }
    }
    if (ppplStack_178 == ppplRam000000014362c3e0) {
      return;
    }
    pplVar11 = *ppplStack_178;
    pplVar16 = ppplStack_178[1];
    pplVar12 = ppplStack_178[2];
    register0x00000020 = (BADSPACEBASE *)apuStack_220;
    apuStack_220[0] = &UNK_1416a5dc4;
    unaff_RSI = ppplRam000000014362c3e0;
  }
  else {
    pplVar12 = (longlong **)(lStack_128 << 3);
    pplVar16 = (longlong **)0x4;
    register0x00000020 = (BADSPACEBASE *)apuStack_220;
    apuStack_220[0] = &UNK_1416a5cba;
    pplVar11 = pplStack_138;
  }
  goto code_r0x000140613c20;
}

