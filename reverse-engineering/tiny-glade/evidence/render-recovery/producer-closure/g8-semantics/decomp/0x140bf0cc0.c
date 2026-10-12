
void FUN_140bf0cc0(longlong param_1,longlong param_2)

{
  byte *pbVar1;
  byte *pbVar2;
  undefined8 *puVar3;
  ulonglong *puVar4;
  undefined4 *puVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  code *pcVar12;
  longlong lVar13;
  undefined8 *puVar14;
  bool bVar15;
  longlong *plVar16;
  char cVar17;
  longlong lVar18;
  undefined *puVar19;
  undefined7 uVar21;
  longlong *plVar20;
  uint uVar22;
  longlong lVar23;
  ulonglong uVar24;
  longlong lVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  char *pcVar28;
  undefined *puVar29;
  undefined *puVar30;
  char *pcVar31;
  undefined *puVar32;
  longlong lVar33;
  undefined1 uVar34;
  bool bVar35;
  longlong lStack_210;
  int iStack_1a0;
  int iStack_190;
  int iStack_18c;
  undefined8 *puStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  ulonglong uStack_160;
  ulonglong uStack_158;
  ulonglong uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  longlong lStack_138;
  longlong lStack_130;
  undefined8 uStack_128;
  int *piStack_120;
  ulonglong uStack_118;
  char *pcStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  ulonglong uStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  char *pcStack_a8;
  longlong *plStack_a0;
  undefined8 uStack_98;
  longlong lStack_88;
  longlong lStack_80;
  longlong *plStack_78;
  longlong *plStack_70;
  undefined1 uStack_66;
  undefined1 uStack_65;
  undefined1 uStack_64;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  cVar17 = func_0x000140c16a00();
  lStack_88 = param_2;
  if (cVar17 == '\0') {
code_r0x000140bf0d62:
    lStack_130 = 0;
code_r0x000140bf0f44:
    uVar24 = *(ulonglong *)(param_1 + 0x80);
    pcVar28 = (char *)(param_2 + 0x9d0);
    piStack_120 = (int *)(param_1 + 0x68);
    pcVar31 = (char *)(param_2 + 0x9b0);
    pcStack_110 = pcVar28;
    pcStack_a8 = pcVar31;
    while (*(longlong *)(param_1 + 0x38) != 0) {
      uVar26 = *(ulonglong *)(param_1 + 0x30);
      uVar9 = *(ulonglong *)(param_1 + 0x20);
      lVar18 = *(longlong *)(param_1 + 0x28);
      uVar27 = uVar9;
      if (uVar26 < uVar9) {
        uVar27 = 0;
      }
      if (uVar24 < *(ulonglong *)(lVar18 + 0x58 + (uVar26 - uVar27) * 0x60)) break;
      uVar27 = 0;
      if (uVar9 <= uVar26 + 1) {
        uVar27 = uVar9;
      }
      *(ulonglong *)(param_1 + 0x30) = (uVar26 - uVar27) + 1;
      *(longlong *)(param_1 + 0x38) = *(longlong *)(param_1 + 0x38) + -1;
      puVar10 = *(undefined8 **)(lVar18 + uVar26 * 0x60);
      if (puVar10 == (undefined8 *)0x4) goto code_r0x000140bf1c5b;
      lVar18 = lVar18 + uVar26 * 0x60;
      uStack_b8 = *(ulonglong *)(lVar18 + 0x58);
      uStack_158 = *(ulonglong *)(lVar18 + 0x48);
      uStack_150 = *(ulonglong *)(lVar18 + 0x50);
      plVar20 = (longlong *)(lVar18 + 8);
      puVar14 = (undefined8 *)*plVar20;
      lVar13 = *plVar20;
      lVar25 = *plVar20;
      uStack_108 = (undefined8 *)*plVar20;
      iStack_190 = *(int *)(lVar18 + 0x10);
      iStack_18c = *(int *)(lVar18 + 0x14);
      uStack_100 = *(undefined8 **)(lVar18 + 0x10);
      lStack_80 = *(longlong *)(lVar18 + 0x18);
      uStack_f8 = *(longlong *)(lVar18 + 0x18);
      uStack_98 = *(undefined8 *)(lVar18 + 0x20);
      uStack_f0 = *(undefined8 **)(lVar18 + 0x20);
      uStack_e8 = *(undefined4 *)(lVar18 + 0x28);
      uStack_e4 = *(undefined4 *)(lVar18 + 0x2c);
      uVar26 = *(ulonglong *)(lVar18 + 0x28);
      uStack_148 = *(undefined4 *)(lVar18 + 0x30);
      uStack_144 = *(undefined4 *)(lVar18 + 0x34);
      uStack_d8 = *(undefined8 *)(lVar18 + 0x38);
      lStack_138 = *(longlong *)(lVar18 + 0x40);
      uStack_168._0_4_ = (undefined4)uStack_d8;
      uStack_168._4_4_ = (undefined4)((ulonglong)uStack_d8 >> 0x20);
      uStack_140 = (undefined4)uStack_168;
      uStack_13c = uStack_168._4_4_;
      puStack_188 = (undefined8 *)0x0;
      uStack_180 = 8;
      uStack_178 = 0;
      uStack_170 = 0;
      uStack_168 = 8;
      uStack_160 = 0;
      uVar21 = (undefined7)((ulonglong)lStack_138 >> 8);
      plStack_78 = (longlong *)CONCAT44(plStack_78._4_4_,(int)CONCAT71(uVar21,1));
      plStack_70 = (longlong *)CONCAT44(plStack_70._4_4_,(int)CONCAT71(uVar21,1));
      uStack_e0 = uStack_148;
      uStack_dc = uStack_144;
      uStack_d0 = lStack_138;
      uStack_c8 = uStack_158;
      uStack_c0 = uStack_150;
      if (uVar26 != 0) {
        uStack_61 = 1;
        uStack_62 = 1;
        uStack_63 = 1;
        cVar17 = func_0x000140c16a00();
        if (cVar17 == '\0') {
          uStack_108 = (undefined8 *)0x0;
        }
        else {
          puVar29 = &UNK_142b15f39;
          puVar32 = &UNK_142b15f18;
          do {
            pbVar1 = puVar29 + -1;
            uVar22 = (uint)(char)*pbVar1;
            if ((int)uVar22 < 0) {
              bVar6 = puVar29[-2];
              if ((char)bVar6 < -0x40) {
                bVar7 = puVar29[-3];
                if ((char)bVar7 < -0x40) {
                  pbVar2 = puVar29 + -4;
                  puVar29 = puVar29 + -4;
                  uVar22 = bVar7 & 0x3f | (*pbVar2 & 7) << 6;
                }
                else {
                  puVar29 = puVar29 + -3;
                  uVar22 = bVar7 & 0xf;
                }
                uVar22 = bVar6 & 0x3f | uVar22 << 6;
              }
              else {
                puVar29 = puVar29 + -2;
                uVar22 = bVar6 & 0x1f;
              }
              uVar22 = (uint)(*pbVar1 & 0x3f) | uVar22 << 6;
              if (uVar22 != 0x5c) goto code_r0x000140bf1199;
code_r0x000140bf11b7:
              puVar32 = puVar29 + -0x142b15f17;
              if (puVar32 != (undefined *)0x0) {
                if (puVar32 < (undefined *)0x21) {
                  if ((char)puVar29[1] < -0x40) {
code_r0x000140bf1d23:
                    uStack_61 = 1;
                    uStack_62 = 1;
                    uStack_63 = 1;
                    FUN_1428d9940(&UNK_142b15f18,0x21,puVar32,0x21,&UNK_142b155d8);
                    goto code_r0x000140bf1da0;
                  }
                }
                else if (puVar32 != (undefined *)0x21) goto code_r0x000140bf1d23;
              }
              lVar18 = 0x20 - (longlong)(puVar29 + -0x142b15f18);
              puVar32 = puVar29 + 1;
              goto code_r0x000140bf11ef;
            }
            puVar29 = puVar29 + -1;
            if (uVar22 == 0x5c) goto code_r0x000140bf11b7;
code_r0x000140bf1199:
            if (uVar22 == 0x2f) goto code_r0x000140bf11b7;
          } while (puVar29 != &UNK_142b15f18);
          lVar18 = 0x21;
code_r0x000140bf11ef:
          plVar20 = (longlong *)func_0x000140c18130();
          if (*plVar20 == 0) {
            uStack_61 = 1;
            uStack_62 = 1;
            uStack_63 = 1;
            plVar20 = (longlong *)FUN_1429090d0(plVar20,0);
            if (plVar20 == (longlong *)0x0) goto code_r0x000140bf1cd5;
          }
          else {
            if ((int)*plVar20 != 1) {
code_r0x000140bf1cd5:
              uStack_61 = 1;
              uStack_62 = 1;
              uStack_63 = 1;
              FUN_1428d80d0(&UNK_142d06498);
              goto code_r0x000140bf1da0;
            }
            plVar20 = plVar20 + 1;
          }
          if (*plVar20 != 0) {
            uStack_61 = 1;
            uStack_62 = 1;
            uStack_63 = 1;
            FUN_1428d9250(&UNK_142d06508);
            goto code_r0x000140bf1da0;
          }
          *plVar20 = -1;
          plStack_70 = plVar20;
          uStack_100 = (undefined8 *)
                       FUN_140c16d40(plVar20 + 1,&UNK_142b15f70,0xe,puVar32,lVar18,1,0);
          *plStack_70 = *plStack_70 + 1;
          uStack_108 = (undefined8 *)0x1;
        }
        lVar18 = lStack_88;
        LOCK();
        cVar17 = *pcVar28;
        if (cVar17 == '\0') {
          *pcVar28 = '\x01';
        }
        UNLOCK();
        if (cVar17 != '\0') {
          uStack_64 = 1;
          FUN_1428cd210(pcVar28);
        }
        uStack_64 = 1;
        FUN_140c81c70(pcVar28);
        lVar33 = *(longlong *)(lVar18 + 0x9e8);
        if ((ulonglong)(*(longlong *)(lVar18 + 0x9d8) - lVar33) < uVar26) {
          FUN_1428ca940(param_2 + 0x9d8,lVar33,uVar26,8,8);
          lVar33 = *(longlong *)(lVar18 + 0x9e8);
        }
        uVar11 = uStack_98;
        func_0x000142923c00(lVar33 * 8 + *(longlong *)(lVar18 + 0x9e0),uStack_98,uVar26 * 8);
        *(ulonglong *)(lVar18 + 0x9e8) = lVar33 + uVar26;
        if (lStack_80 != 0) {
          func_0x000140613c20(uVar11,lStack_80 * 8,8);
        }
        pcVar28 = pcStack_110;
        uStack_64 = 0;
        FUN_140c81e10(pcStack_110);
        pcVar31 = pcStack_a8;
        LOCK();
        cVar17 = *pcVar28;
        if (cVar17 == '\x01') {
          *pcVar28 = '\0';
        }
        UNLOCK();
        if (cVar17 != '\x01') {
          uStack_64 = 0;
          FUN_1428cdb90(pcVar28,0);
        }
        if (uStack_108 != (undefined8 *)0x0) {
          plVar20 = (longlong *)func_0x000140c18130();
          if (*plVar20 == 0) {
            uStack_61 = 1;
            uStack_62 = 0;
            uStack_63 = 1;
            plVar20 = (longlong *)FUN_1429090d0(plVar20,0);
            if (plVar20 == (longlong *)0x0) goto code_r0x000140bf1c84;
          }
          else {
            if ((int)*plVar20 != 1) {
code_r0x000140bf1c84:
              uStack_61 = 1;
              uStack_62 = 0;
              uStack_63 = 1;
              FUN_1428d80d0(&UNK_142d06498);
              goto code_r0x000140bf1da0;
            }
            plVar20 = plVar20 + 1;
          }
          if (*plVar20 != 0) {
            uStack_61 = 1;
            uStack_62 = 0;
            uStack_63 = 1;
            FUN_1428d9250(&UNK_142d06508);
            goto code_r0x000140bf1da0;
          }
          *plVar20 = -1;
          plStack_70 = plVar20;
          FUN_140c170f0(plVar20 + 1,uStack_100);
          *plStack_70 = *plStack_70 + 1;
        }
        plStack_70 = (longlong *)((ulonglong)plStack_70 & 0xffffffff00000000);
      }
      if (lStack_138 != 0) {
        uStack_61 = 1;
        uStack_62 = SUB81(plStack_70,0);
        uStack_63 = 1;
        cVar17 = func_0x000140c16a00();
        if (cVar17 == '\0') {
          bVar15 = false;
        }
        else {
          puVar29 = &UNK_142b15f39;
          do {
            bVar6 = puVar29[-1];
            uVar22 = (uint)(char)bVar6;
            if ((char)bVar6 < '\0') {
              bVar7 = puVar29[-2];
              if ((char)bVar7 < -0x40) {
                bVar8 = puVar29[-3];
                if ((char)bVar8 < -0x40) {
                  pbVar1 = puVar29 + -4;
                  puVar29 = puVar29 + -4;
                  uVar22 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
                }
                else {
                  puVar29 = puVar29 + -3;
                  uVar22 = bVar8 & 0xf;
                }
                uVar22 = bVar7 & 0x3f | uVar22 << 6;
              }
              else {
                puVar29 = puVar29 + -2;
                uVar22 = bVar7 & 0x1f;
              }
              uVar22 = (uint)(bVar6 & 0x3f) | uVar22 << 6;
              if (uVar22 != 0x5c) goto code_r0x000140bf1499;
code_r0x000140bf14c7:
              puVar32 = puVar29 + -0x142b15f17;
              if (puVar32 != (undefined *)0x0) {
                if (puVar32 < (undefined *)0x21) {
                  if ((char)puVar29[1] < -0x40) {
code_r0x000140bf1d60:
                    uStack_61 = 1;
                    uStack_62 = SUB81(plStack_70,0);
                    uStack_63 = 1;
                    FUN_1428d9940(&UNK_142b15f18,0x21,puVar32,0x21,&UNK_142b155d8);
                    goto code_r0x000140bf1da0;
                  }
                }
                else if (puVar32 != (undefined *)0x21) goto code_r0x000140bf1d60;
              }
              lVar18 = 0x20 - (longlong)(puVar29 + -0x142b15f18);
              puVar29 = puVar29 + 1;
              goto code_r0x000140bf1502;
            }
            puVar29 = puVar29 + -1;
            if (uVar22 == 0x5c) goto code_r0x000140bf14c7;
code_r0x000140bf1499:
            if (uVar22 == 0x2f) goto code_r0x000140bf14c7;
          } while (puVar29 != &UNK_142b15f18);
          lVar18 = 0x21;
          puVar29 = &UNK_142b15f18;
code_r0x000140bf1502:
          plVar20 = (longlong *)func_0x000140c18130();
          if (*plVar20 == 0) {
            uStack_61 = 1;
            uStack_62 = SUB81(plStack_70,0);
            uStack_63 = 1;
            plVar20 = (longlong *)FUN_1429090d0(plVar20,0);
            if (plVar20 == (longlong *)0x0) goto code_r0x000140bf1cfb;
          }
          else {
            if ((int)*plVar20 != 1) {
code_r0x000140bf1cfb:
              uStack_61 = 1;
              uStack_62 = SUB81(plStack_70,0);
              uStack_63 = 1;
              FUN_1428d80d0(&UNK_142d06498);
              goto code_r0x000140bf1da0;
            }
            plVar20 = plVar20 + 1;
          }
          if (*plVar20 != 0) {
            uStack_61 = 1;
            uStack_62 = SUB81(plStack_70,0);
            uStack_63 = 1;
            FUN_1428d9250(&UNK_142d06508);
            goto code_r0x000140bf1da0;
          }
          *plVar20 = -1;
          plStack_78 = plVar20;
          lStack_210 = FUN_140c16d40(plVar20 + 1,&UNK_142b15f7e,0x18,puVar29,lVar18,1,0);
          *plStack_78 = *plStack_78 + 1;
          bVar15 = true;
        }
        uStack_f8 = CONCAT44(uStack_144,uStack_148);
        uStack_108 = (undefined8 *)CONCAT44(uStack_13c,uStack_140);
        uStack_f0 = uStack_108 + lStack_138 * 2;
        uStack_100 = uStack_108;
        while (uStack_100 != uStack_f0) {
          puVar3 = uStack_100 + 2;
          uVar11 = *uStack_100;
          plVar20 = (longlong *)uStack_100[1];
          plStack_78 = plVar20 + 2;
          LOCK();
          bVar35 = (char)plVar20[2] == '\0';
          if (bVar35) {
            *(char *)(plVar20 + 2) = '\x01';
          }
          UNLOCK();
          uStack_100 = puVar3;
          plStack_a0 = plVar20;
          if (!bVar35) {
            FUN_1428cd210();
          }
          FUN_140c81c70(plStack_78);
          lVar18 = plVar20[5];
          if (plVar20[3] == lVar18) {
            FUN_1428ca940(plVar20 + 3,lVar18,1,8,8);
            lVar18 = plVar20[5];
          }
          plVar16 = plStack_78;
          *(undefined8 *)(plVar20[4] + lVar18 * 8) = uVar11;
          plVar20[5] = lVar18 + 1;
          FUN_140c81e10(plStack_78);
          LOCK();
          bVar35 = (char)*plVar16 == '\x01';
          if (bVar35) {
            *(char *)plVar16 = '\0';
          }
          UNLOCK();
          if (!bVar35) {
            FUN_1428cdb90(plVar16);
          }
          LOCK();
          *plStack_a0 = *plStack_a0 + -1;
          UNLOCK();
          if (*plStack_a0 == 0) {
            FUN_140b5c6b0(&plStack_a0);
          }
        }
        FUN_140bdef60(&uStack_108);
        if (bVar15) {
          plVar20 = (longlong *)func_0x000140c18130();
          if (*plVar20 == 0) {
            uStack_61 = 0;
            uStack_62 = SUB81(plStack_70,0);
            uStack_63 = 1;
            plVar20 = (longlong *)FUN_1429090d0(plVar20,0);
            if (plVar20 == (longlong *)0x0) goto code_r0x000140bf1caa;
          }
          else {
            if ((int)*plVar20 != 1) {
code_r0x000140bf1caa:
              uStack_61 = 0;
              uStack_62 = SUB81(plStack_70,0);
              uStack_63 = 1;
              FUN_1428d80d0(&UNK_142d06498);
              goto code_r0x000140bf1da0;
            }
            plVar20 = plVar20 + 1;
          }
          if (*plVar20 != 0) {
            uStack_61 = 0;
            uStack_62 = SUB81(plStack_70,0);
            uStack_63 = 1;
            FUN_1428d9250(&UNK_142d06508);
            goto code_r0x000140bf1da0;
          }
          *plVar20 = -1;
          plStack_78 = plVar20;
          FUN_140c170f0(plVar20 + 1,lStack_210);
          *plStack_78 = *plStack_78 + 1;
        }
        plStack_78 = (longlong *)((ulonglong)plStack_78 & 0xffffffff00000000);
      }
      iStack_1a0 = (int)puVar10;
      if (iStack_1a0 != 3) goto code_r0x000140bf1b1d;
      uStack_61 = SUB81(plStack_78,0);
      uStack_62 = SUB81(plStack_70,0);
      if ((iStack_190 != *piStack_120) ||
         ((iStack_190 != 0 && (iStack_18c != *(int *)(param_1 + 0x6c))))) {
        uStack_108 = (undefined8 *)0x0;
        uStack_63 = 1;
        FUN_1428ca88c(0,&iStack_190,piStack_120,&uStack_108,&UNK_142b15f98);
        goto code_r0x000140bf1da0;
      }
      uStack_63 = 1;
      FUN_1418c7ae0(param_1 + 0x40,lVar25,lStack_88);
      LOCK();
      cVar17 = *pcVar31;
      if (cVar17 == '\0') {
        *pcVar31 = '\x01';
      }
      UNLOCK();
      if (cVar17 != '\0') {
        uStack_61 = SUB81(plStack_78,0);
        uStack_62 = SUB81(plStack_70,0);
        uStack_63 = 1;
        FUN_1428cd210(pcVar31);
      }
      uStack_61 = SUB81(plStack_78,0);
      uStack_62 = SUB81(plStack_70,0);
      uStack_63 = 1;
      FUN_140c81c70(pcVar31);
      uVar26 = uStack_150 & 0xffffffff;
      uStack_118 = *(ulonglong *)(lStack_88 + 0x9c8);
      if (uStack_118 <= uVar26) {
        FUN_1428d9518(uVar26,uStack_118,&UNK_142b14da0);
        goto code_r0x000140bf1da0;
      }
      lVar25 = *(longlong *)(lStack_88 + 0x9c0);
      lVar18 = lVar25 + uVar26 * 0x18;
      uStack_b8 = uStack_150;
      uStack_c8 = uStack_160;
      uStack_c0 = uStack_158;
      uStack_d8 = uStack_170;
      uStack_d0 = uStack_168;
      uStack_e8 = (undefined4)uStack_180;
      uStack_e4 = uStack_180._4_4_;
      uStack_e0 = (undefined4)uStack_178;
      uStack_dc = uStack_178._4_4_;
      uStack_f8 = CONCAT44(iStack_18c,iStack_190);
      uStack_f0 = puStack_188;
      lVar33 = *(longlong *)(lVar25 + 0x10 + uVar26 * 0x18);
      uStack_108 = puVar10;
      uStack_100 = puVar14;
      if (lVar33 == *(longlong *)(lVar25 + uVar26 * 0x18)) {
        FUN_140562f30(lVar18,&UNK_142b14db8);
      }
      pcVar31 = pcStack_a8;
      lVar25 = *(longlong *)(lVar18 + 8);
      lVar23 = lVar33 * 0x58;
      *(ulonglong *)(lVar25 + 0x50 + lVar23) = uStack_b8;
      puVar4 = (ulonglong *)(lVar25 + 0x40 + lVar23);
      *puVar4 = uStack_c8;
      puVar4[1] = uStack_c0;
      puVar5 = (undefined4 *)(lVar25 + 0x30 + lVar23);
      *puVar5 = (undefined4)uStack_d8;
      puVar5[1] = uStack_d8._4_4_;
      puVar5[2] = (undefined4)uStack_d0;
      puVar5[3] = uStack_d0._4_4_;
      puVar5 = (undefined4 *)(lVar25 + 0x20 + lVar23);
      *puVar5 = uStack_e8;
      puVar5[1] = uStack_e4;
      puVar5[2] = uStack_e0;
      puVar5[3] = uStack_dc;
      puVar5 = (undefined4 *)(lVar25 + 0x10 + lVar23);
      *puVar5 = (undefined4)uStack_f8;
      puVar5[1] = uStack_f8._4_4_;
      puVar5[2] = (undefined4)uStack_f0;
      puVar5[3] = uStack_f0._4_4_;
      puVar5 = (undefined4 *)(lVar25 + lVar23);
      *puVar5 = (undefined4)uStack_108;
      puVar5[1] = uStack_108._4_4_;
      puVar5[2] = (undefined4)uStack_100;
      puVar5[3] = uStack_100._4_4_;
      *(longlong *)(lVar18 + 0x10) = lVar33 + 1;
      uStack_61 = SUB81(plStack_78,0);
      uStack_62 = SUB81(plStack_70,0);
      uStack_63 = 0;
      FUN_140c81e10(pcStack_a8);
      LOCK();
      cVar17 = *pcVar31;
      if (cVar17 == '\x01') {
        *pcVar31 = '\0';
      }
      UNLOCK();
      if (cVar17 != '\x01') {
        uStack_61 = SUB81(plStack_78,0);
        uStack_62 = SUB81(plStack_70,0);
        uStack_63 = 0;
        FUN_1428cdb90(pcVar31,0);
      }
      if ((char)plStack_78 != '\0') {
        uStack_65 = SUB81(plStack_70,0);
        uStack_66 = 0;
        FUN_1405df600(&uStack_148);
      }
      lStack_210 = lVar13;
      if (((byte)plStack_70 & lStack_80 != 0) != 0) {
        func_0x000140613c20(uStack_98,lStack_80 << 3,8);
      }
    }
    if (lStack_130 == 0) {
      return;
    }
    plVar20 = (longlong *)func_0x000140c18130();
    if (*plVar20 == 1) {
      plVar20 = plVar20 + 1;
    }
    else if ((*plVar20 != 0) ||
            (plVar20 = (longlong *)FUN_1429090d0(plVar20,0), plVar20 == (longlong *)0x0))
    goto code_r0x000140bf1b11;
    if (*plVar20 == 0) {
      *plVar20 = -1;
      plStack_78 = plVar20;
      FUN_140c170f0(plVar20 + 1,uStack_128);
      *plStack_78 = *plStack_78 + 1;
      return;
    }
  }
  else {
    puVar29 = &UNK_142b15531;
    lVar25 = 0x3e;
    uVar24 = 0x3e;
    lVar18 = FUN_140bed840(&UNK_142b15531);
    if (lVar18 == 1) {
      if (uVar24 != 0) {
        if (uVar24 < 0x3e) {
          if ((char)(&UNK_142b15531)[uVar24] < -0x40) {
code_r0x000140bf0d42:
            FUN_1428d9940(&UNK_142b15531,0x3e,0,uVar24,&UNK_142b155f8);
            goto code_r0x000140bf0d62;
          }
        }
        else if (uVar24 != 0x3e) goto code_r0x000140bf0d42;
      }
      puVar29 = &UNK_142b15531;
      lVar18 = FUN_140bed840(&UNK_142b15531);
      if (lVar18 == 1) {
        uVar26 = uVar24 + 2;
        if (uVar26 != 0) {
          uVar34 = uVar26 == 0x3e;
          if (0x3d < uVar26) goto code_r0x000140bf0dc3;
          uVar34 = (&UNK_142b15533)[uVar24] == -0x41;
          if ((char)(&UNK_142b15533)[uVar24] < -0x40) {
            do {
              uVar24 = 0x3e;
              FUN_1428d9940(&UNK_142b15531,0x3e);
code_r0x000140bf0dc3:
            } while (!(bool)uVar34);
          }
        }
        lVar25 = 0x3c - uVar24;
        puVar29 = &UNK_142b15531 + uVar26;
      }
    }
    puVar32 = &UNK_142b15f39;
    puVar30 = &UNK_142b15f18;
    do {
      bVar6 = puVar32[-1];
      uVar22 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = puVar32[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar32[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar32 + -4;
            puVar32 = puVar32 + -4;
            uVar22 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar32 = puVar32 + -3;
            uVar22 = bVar8 & 0xf;
          }
          uVar22 = bVar7 & 0x3f | uVar22 << 6;
        }
        else {
          puVar32 = puVar32 + -2;
          uVar22 = bVar7 & 0x1f;
        }
        uVar22 = (uint)(bVar6 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto code_r0x000140bf0e69;
code_r0x000140bf0e7e:
        puVar19 = puVar32 + -0x142b15f18;
        puVar30 = puVar32 + -0x142b15f17;
        if (puVar30 != (undefined *)0x0) {
          uVar34 = puVar30 == (undefined *)0x21;
          if ((undefined *)0x20 < puVar30) goto code_r0x000140bf0ec0;
          uVar34 = puVar32[1] == -0x41;
          if ((char)puVar32[1] < -0x40) {
            do {
              puVar19 = (undefined *)FUN_1428d9940(&UNK_142b15f18,0x21);
code_r0x000140bf0ec0:
            } while (!(bool)uVar34);
          }
        }
        lVar18 = 0x20 - (longlong)puVar19;
        puVar30 = &UNK_142b15f18 + (longlong)puVar30;
        goto code_r0x000140bf0ecd;
      }
      puVar32 = puVar32 + -1;
      if (uVar22 == 0x5c) goto code_r0x000140bf0e7e;
code_r0x000140bf0e69:
      if (uVar22 == 0x2f) goto code_r0x000140bf0e7e;
    } while (puVar32 != &UNK_142b15f18);
    lVar18 = 0x21;
code_r0x000140bf0ecd:
    plVar20 = (longlong *)func_0x000140c18130();
    if (*plVar20 != 1) {
      if ((*plVar20 == 0) &&
         (plVar20 = (longlong *)FUN_1429090d0(plVar20,0), plVar20 != (longlong *)0x0))
      goto code_r0x000140bf0ee6;
code_r0x000140bf1b11:
      FUN_1428d80d0(&UNK_142d06498);
code_r0x000140bf1b1d:
      uStack_61 = SUB81(plStack_78,0);
      uStack_62 = SUB81(plStack_70,0);
      uStack_63 = 1;
      FUN_1428d9430(&UNK_142b15fb0,0x28,&UNK_142b15fd8);
      goto code_r0x000140bf1da0;
    }
    plVar20 = plVar20 + 1;
code_r0x000140bf0ee6:
    if (*plVar20 == 0) {
      *plVar20 = -1;
      plStack_78 = plVar20;
      uStack_128 = FUN_140c16d40(plVar20 + 1,puVar29,lVar25,puVar30,lVar18,1,0);
      *plStack_78 = *plStack_78 + 1;
      lStack_130 = 1;
      param_2 = lStack_88;
      goto code_r0x000140bf0f44;
    }
  }
  FUN_1428d9250(&UNK_142d06508);
code_r0x000140bf1c5b:
  FUN_1428d9310(&UNK_142b15f58);
code_r0x000140bf1da0:
                    /* WARNING: Does not return */
  pcVar12 = (code *)invalidInstructionException();
  (*pcVar12)();
}

