
void FUN_1402c3240(undefined8 *param_1,undefined8 *param_2,longlong param_3,ulonglong param_4)

{
  byte *pbVar1;
  undefined4 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  code *pcVar6;
  char cVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  uint uVar10;
  longlong lVar11;
  longlong *plVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  undefined *puVar15;
  undefined8 *puVar16;
  undefined8 uVar17;
  undefined *puVar18;
  undefined4 *puVar19;
  ulonglong uVar20;
  undefined8 in_stack_fffffffffffffd78;
  int iStack_268;
  undefined4 uStack_264;
  undefined4 uStack_260;
  undefined4 uStack_25c;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 uStack_228;
  undefined8 uStack_220;
  int iStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  longlong lStack_170;
  undefined8 uStack_168;
  longlong lStack_158;
  undefined8 uStack_150;
  longlong lStack_140;
  undefined8 uStack_138;
  undefined8 *puStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined4 uStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  longlong lStack_d0;
  ulonglong uStack_c8;
  longlong lStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  undefined8 uStack_a8;
  longlong lStack_a0;
  undefined *puStack_98;
  undefined8 *puStack_90;
  ulonglong uStack_88;
  longlong lStack_80;
  ulonglong uStack_78;
  longlong lStack_70;
  longlong *plStack_68;
  longlong *plStack_60;
  undefined *puStack_58;
  undefined1 uStack_4a;
  char cStack_49;
  undefined8 uStack_48;
  
  uVar9 = (undefined4)((ulonglong)in_stack_fffffffffffffd78 >> 0x20);
  uStack_48 = 0xfffffffffffffffe;
  puVar15 = (undefined *)*param_2;
  puStack_90 = param_1;
  uVar8 = FUN_140bfc750(puVar15,0);
  FUN_140bef4a0(&uStack_1c8,puVar15,&UNK_1429aeca8,0xe,CONCAT44(uVar9,uVar8));
  puStack_58 = puVar15;
  FUN_140bed230(&uStack_1c8,puVar15,0);
  uVar20 = param_4 << 4;
  uVar17 = 0;
  if ((param_4 >> 0x3c == 0) && (uVar20 < 0x7ffffffffffffff9)) {
    if (uVar20 == 0) {
      lVar11 = 8;
      uVar14 = 0;
    }
    else {
      uVar17 = 8;
      puVar15 = (undefined *)0x8;
      lVar11 = func_0x000140613c10(uVar20);
      uVar14 = param_4;
      if (lVar11 == 0) goto code_r0x0001402c3a86;
    }
    uStack_78 = 0;
    lStack_c0 = param_4 * 0x40 + param_3;
    plStack_b8 = (longlong *)param_2[1];
    uVar17 = param_2[2];
    plStack_b0 = (longlong *)param_2[3];
    lStack_a0 = param_2[4];
    puStack_98 = puStack_58 + 0x78;
    uStack_a8 = param_2[5];
    uVar20 = 0;
    uStack_88 = uVar14;
    lStack_80 = lVar11;
    do {
      uVar8 = SUB84(puVar15,0);
      if (param_3 == lStack_c0) {
        puStack_90[10] = uStack_178;
        puStack_90[8] = uStack_188;
        puStack_90[9] = uStack_180;
        puStack_90[6] = uStack_198;
        puStack_90[7] = uStack_190;
        puStack_90[4] = uStack_1a8;
        puStack_90[5] = uStack_1a0;
        puStack_90[2] = uStack_1b8;
        puStack_90[3] = uStack_1b0;
        *puStack_90 = uStack_1c8;
        puStack_90[1] = uStack_1c0;
        puStack_90[0xb] = uStack_88;
        puStack_90[0xc] = lStack_80;
        puStack_90[0xd] = uStack_78;
        return;
      }
      uStack_100 = FUN_140d635e0();
      plStack_68 = (longlong *)(*plStack_b8 + uVar20);
      uVar9 = 0;
      uStack_f8 = uVar8;
      if (*pcRam000000014362c220 == '\x01') {
        uVar9 = FUN_140befb00(puStack_58,&uStack_1c8,*(undefined8 *)(param_3 + 0x18),
                              *(undefined8 *)(param_3 + 0x20));
      }
      FUN_140be2880(&iStack_218,*(undefined8 *)(param_3 + 0x18),*(undefined8 *)(param_3 + 0x20));
      FUN_140be2ad0(&iStack_218,uVar9);
      lStack_d0 = param_3 + 0x40;
      uStack_c8 = uVar20 + 1;
      puVar19 = *(undefined4 **)(param_3 + 8);
      puVar2 = puVar19 + *(longlong *)(param_3 + 0x10) * 3;
      while (puVar19 != puVar2) {
        FUN_140be78b0(&iStack_218,uVar17,*puVar19);
        puVar15 = &UNK_1429aed60;
        lVar11 = FUN_140bd6c80(CONCAT44(uStack_214,iStack_218),&UNK_1429aed60);
        puVar19 = puVar19 + 3;
        if (lVar11 == 0) {
          FUN_140bd6a70(puVar15,&uStack_1c8);
        }
        else {
          if (lVar11 != 1) {
            FUN_1428d9430(&UNK_1429aed78,0x13,&UNK_1429aed90);
            goto code_r0x0001402c3a98;
          }
          FUN_140bd6a70(puVar15,&uStack_1c8);
        }
      }
      func_0x000140bd6650(&lStack_170);
      FUN_1402c3d00(uVar17,plStack_68,&lStack_170,puStack_58);
      cVar7 = func_0x000140bd6980(&lStack_170);
      if (cVar7 == '\0') {
        FUN_140bee030(puStack_58,&uStack_1c8,&lStack_170);
      }
      func_0x000140bd6950(&lStack_170);
      if (puStack_58[0xab8] != '\0') {
        FUN_140bee090(puStack_58,uStack_180);
      }
      FUN_140bf09b0(puStack_58,&uStack_1c8,*(undefined8 *)(param_3 + 0x18),
                    *(undefined8 *)(param_3 + 0x20));
      if (*plStack_b0 != 0) {
        if (*(ulonglong *)(lStack_a0 + 0x10) <= uVar20) {
          FUN_1428d9518(uVar20,*(ulonglong *)(lStack_a0 + 0x10),&UNK_1429aecd0);
          goto code_r0x0001402c3a98;
        }
        FUN_140c11150(*plStack_b0,puStack_98,uStack_180,
                      *(undefined4 *)(*(longlong *)(lStack_a0 + 8) + uVar20 * 4));
      }
      cStack_49 = func_0x000140c16a00();
      if (cStack_49 == '\0') {
        uStack_e0 = 0;
      }
      else {
        puVar15 = &UNK_1429aed0a;
        do {
          bVar3 = puVar15[-1];
          uVar10 = (uint)(char)bVar3;
          if ((char)bVar3 < '\0') {
            bVar4 = puVar15[-2];
            if ((char)bVar4 < -0x40) {
              bVar5 = puVar15[-3];
              if ((char)bVar5 < -0x40) {
                pbVar1 = puVar15 + -4;
                puVar15 = puVar15 + -4;
                uVar10 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
              }
              else {
                puVar15 = puVar15 + -3;
                uVar10 = bVar5 & 0xf;
              }
              uVar10 = bVar4 & 0x3f | uVar10 << 6;
            }
            else {
              puVar15 = puVar15 + -2;
              uVar10 = bVar4 & 0x1f;
            }
            uVar10 = (uint)(bVar3 & 0x3f) | uVar10 << 6;
            if (uVar10 != 0x5c) goto code_r0x0001402c35f2;
code_r0x0001402c3630:
            puVar18 = puVar15 + -0x1429aece7;
            if (puVar18 != (undefined *)0x0) {
              if (puVar18 < (undefined *)0x22) {
                if (-0x41 < (char)puVar15[1]) goto code_r0x0001402c365d;
              }
              else if (puVar18 == (undefined *)0x22) goto code_r0x0001402c365d;
              FUN_1428d9940(&UNK_1429aece8,0x22,puVar18,0x22,&UNK_1429ae988);
              goto code_r0x0001402c3a98;
            }
code_r0x0001402c365d:
            lVar11 = 0x22 - (longlong)puVar18;
            puVar15 = puVar15 + 1;
            goto code_r0x0001402c3669;
          }
          puVar15 = puVar15 + -1;
          if (uVar10 == 0x5c) goto code_r0x0001402c3630;
code_r0x0001402c35f2:
          if (uVar10 == 0x2f) goto code_r0x0001402c3630;
        } while (puVar15 != &UNK_1429aece8);
        lVar11 = 0x22;
        puVar15 = &UNK_1429aece8;
code_r0x0001402c3669:
        uVar13 = *(undefined8 *)(param_3 + 0x18);
        lStack_70 = *(longlong *)(param_3 + 0x20);
        plVar12 = (longlong *)func_0x000140c18130();
        if (*plVar12 == 0) {
          plVar12 = (longlong *)FUN_1428a1640(plVar12,0);
          if (plVar12 == (longlong *)0x0) goto code_r0x0001402c3a42;
        }
        else {
          if ((int)*plVar12 != 1) {
code_r0x0001402c3a42:
            FUN_1428d80d0(&UNK_142932a10);
            goto code_r0x0001402c3a98;
          }
          plVar12 = plVar12 + 1;
        }
        if (*plVar12 != 0) {
          FUN_1428d9250(&UNK_142932ab8);
          goto code_r0x0001402c3a98;
        }
        *plVar12 = -1;
        plStack_60 = plVar12;
        uStack_d8 = FUN_140c16d40(plVar12 + 1,&UNK_1429aed0a,0xb,puVar15,lVar11,uVar13,lStack_70);
        *plStack_60 = *plStack_60 + 1;
        uStack_e0 = 1;
      }
      lStack_70 = *(longlong *)(param_3 + 0x28);
      plStack_60 = *(longlong **)(param_3 + 0x30);
      *(undefined8 *)(param_3 + 0x28) = 0;
      if (lStack_70 == 0) {
        FUN_1428d9310(&UNK_1429aed18);
        goto code_r0x0001402c3a98;
      }
      uStack_4a = 1;
      uStack_108 = FUN_140bd4b40(plStack_68);
      puStack_118 = puStack_58;
      uStack_4a = 0;
      puStack_120 = &uStack_1c8;
      uStack_110 = uVar17;
      (**(code **)((longlong)plStack_60 + 0x18))(&iStack_268,lStack_70,&puStack_120,uStack_a8);
      if (iStack_268 != 0xb) {
        uStack_1d8 = uStack_228;
        uStack_1d0 = uStack_220;
        uStack_1e8 = uStack_238;
        uStack_1e4 = uStack_234;
        uStack_1e0 = uStack_230;
        uStack_1dc = uStack_22c;
        uStack_1f8 = uStack_248;
        uStack_1f4 = uStack_244;
        uStack_1f0 = uStack_240;
        uStack_1ec = uStack_23c;
        uStack_208 = uStack_258;
        uStack_204 = uStack_254;
        uStack_200 = uStack_250;
        uStack_1fc = uStack_24c;
        iStack_218 = iStack_268;
        uStack_214 = uStack_264;
        uStack_210 = uStack_260;
        uStack_20c = uStack_25c;
        FUN_1428d9760(&UNK_1429ae8f8,0x2b,&iStack_218,&UNK_1429ae8d8,&UNK_1429aed30);
        goto code_r0x0001402c3a98;
      }
      if (cStack_49 != '\0') {
        plVar12 = (longlong *)func_0x000140c18130();
        if (*plVar12 == 0) {
          plVar12 = (longlong *)FUN_1428a1640(plVar12,0);
          if (plVar12 == (longlong *)0x0) goto code_r0x0001402c3a50;
        }
        else {
          if ((int)*plVar12 != 1) {
code_r0x0001402c3a50:
            FUN_1428d80d0(&UNK_142932a10);
            goto code_r0x0001402c3a98;
          }
          plVar12 = plVar12 + 1;
        }
        if (*plVar12 != 0) {
          FUN_1428d9250(&UNK_142932ab8);
          goto code_r0x0001402c3a98;
        }
        *plVar12 = -1;
        plStack_68 = plVar12;
        FUN_140c170f0(plVar12 + 1,uStack_d8);
        *plStack_68 = *plStack_68 + 1;
      }
      puVar16 = &uStack_1c8;
      FUN_140bf0b40(puStack_58,&uStack_1c8);
      uVar10 = (uint)puVar16;
      FUN_140be2b10();
      uVar13 = *(undefined8 *)(param_3 + 0x18);
      puVar18 = *(undefined **)(param_3 + 0x20);
      lVar11 = FUN_140d63910(&uStack_100);
      uVar13 = func_0x000140be9c70(uVar13,puVar18,(ulonglong)uVar10 + lVar11 * 1000000000);
      uVar20 = uStack_78;
      puVar15 = puVar18;
      if (uStack_78 == uStack_88) {
        puVar15 = &UNK_1429aed48;
        FUN_140562630(&uStack_88);
      }
      lVar11 = uVar20 * 0x10;
      *(undefined8 *)(lStack_80 + lVar11) = uVar13;
      *(undefined **)(lStack_80 + 8 + lVar11) = puVar18;
      uStack_78 = uVar20 + 1;
      if (lStack_170 != 0) {
        puVar15 = (undefined *)(lStack_170 * 0x18);
        func_0x000140613c20(uStack_168,puVar15,8);
      }
      if (lStack_158 != 0) {
        puVar15 = (undefined *)(lStack_158 * 0x38);
        func_0x000140613c20(uStack_150,puVar15,8);
      }
      param_3 = lStack_d0;
      uVar20 = uStack_c8;
      if (lStack_140 != 0) {
        puVar15 = (undefined *)(lStack_140 * 0x48);
        func_0x000140613c20(uStack_138,puVar15,8);
        param_3 = lStack_d0;
        uVar20 = uStack_c8;
      }
    } while( true );
  }
code_r0x0001402c3a86:
  FUN_1428d8fe3(uVar17,uVar20,&UNK_1429aecb8);
code_r0x0001402c3a98:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

