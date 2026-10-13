
void asset__inheritable__impl$5__inherit_from(undefined8 *param_1,undefined8 *param_2)
{
  longlong lVar1;
  undefined8 *puVar2;
  ulonglong *puVar3;
  undefined1 auVar4 [16];
  undefined1 (*pauVar5) [16];
  ushort uVar6;
  uint uVar7;
  ulonglong uVar8;
  undefined8 *puVar9;
  ulonglong uVar10;
  byte bVar11;
  uint uVar12;
  longlong lVar13;
  undefined8 *puVar14;
  byte bVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  uint uVar18;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  undefined1 (*pauVar22) [16];
  ulonglong *puVar23;
  undefined1 (*pauVar24) [16];
  bool bVar25;
  ulonglong uStack_268;
  ulonglong uStack_260;
  ulonglong uStack_240;
  undefined8 *puStack_238;
  ulonglong uStack_230;
  ulonglong uStack_228;
  undefined8 *puStack_220;
  ulonglong uStack_218;
  ulonglong uStack_210;
  undefined8 *puStack_208;
  ulonglong uStack_200;
  ulonglong uStack_1f8;
  longlong lStack_1f0;
  undefined8 uStack_1e8;
  undefined4 auStack_1c0 [17];
  undefined1 uStack_17c;
  undefined8 *puStack_178;
  undefined4 uStack_170;
  undefined8 *puStack_168;
  undefined4 uStack_160;
  undefined8 uStack_158;
  undefined4 uStack_150;
  undefined8 *puStack_148;
  undefined4 uStack_140;
  undefined8 *puStack_138;
  undefined4 uStack_130;
  undefined8 *puStack_128;
  undefined4 uStack_120;
  undefined8 *puStack_118;
  ulonglong uStack_110;
  ulonglong uStack_108;
  undefined8 *puStack_100;
  ulonglong uStack_f8;
  ulonglong uStack_f0;
  undefined8 *puStack_e8;
  ulonglong uStack_e0;
  ulonglong uStack_d8;
  undefined8 *puStack_d0;
  ulonglong uStack_c8;
  uint uStack_c0;
  uint uStack_bc;
  uint uStack_b8;
  uint uStack_b4;
  undefined8 *puStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined8 *puStack_88;
  undefined4 uStack_80;
  uint uStack_7c;
  undefined8 *puStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  ulonglong uStack_68;
  uint uStack_5c;
  undefined8 *puStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  uStack_48 = 0xfffffffffffffffe;
  pauVar24 = (undefined1 (*) [16])*param_2;
  lVar19 = param_2[3];
  auVar4 = *pauVar24;
  pauVar22 = pauVar24 + 1;
  uVar12 = ~(uint)(ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar4[0xf] >> 7) << 0xf);
  puStack_d0 = param_1;
  do {
    while( true ) {
      bVar25 = lVar19 == 0;
      lVar19 = lVar19 + -1;
      if (bVar25) {
        lVar19 = param_1[3];
        if (lVar19 != 0) {
          pauVar24 = (undefined1 (*) [16])*param_1;
          pauVar22 = pauVar24 + 1;
          auVar4 = *pauVar24;
          uVar12 = ~(uint)(ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar4[0xf] >> 7) << 0xf);
          do {
            if ((short)uVar12 == 0) {
              do {
                auVar4 = *pauVar22;
                uVar6 = (ushort)(SUB161(auVar4 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
                pauVar24 = pauVar24 + -0xd8;
                pauVar22 = pauVar22 + 1;
              } while (uVar6 == 0xffff);
              uVar18 = ~(uint)uVar6;
              uVar12 = uVar18 & -(uint)uVar6 - 2;
            }
            else {
              uVar18 = uVar12;
              uVar12 = uVar12 - 1 & uVar12;
            }
            uVar7 = 0;
            for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
              uVar7 = uVar7 + 1;
            }
            lVar13 = (ulonglong)uVar7 * -0xd8;
            uStack_17c = 0;
            puStack_238 = (undefined8 *)0x8000000000000001;
            puStack_220 = (undefined8 *)0x8000000000000001;
            puStack_208 = (undefined8 *)0x8000000000000000;
            auStack_1c0[0] = 3;
            lStack_1f0 = -0x7ffffffffffffffc;
            if (*(char *)((longlong)pauVar24 + lVar13 + -4) == '\x02') {
              *(undefined1 *)((longlong)pauVar24 + lVar13 + -4) = 0;
            }
            asset__inheritable__impl$1__inherit_from((longlong)pauVar24 + lVar13 + -0xc0,&puStack_238);
            asset__inheritable__impl$1__inherit_from((longlong)pauVar24 + lVar13 + -0x48,auStack_1c0);
            asset__inheritable__impl$1__inherit_from((longlong)pauVar24 + lVar13 + -0x78,&lStack_1f0);
            if ((ulonglong)puStack_238 >> 1 != 0x4000000000000001) {
              if ((-0x7fffffffffffffff < (longlong)puStack_238) &&
                 (puStack_238 != (undefined8 *)0x0)) {
                func_0x000140613c20(uStack_230,puStack_238,1);
              }
              if ((-0x7ffffffffffffffe < (longlong)puStack_208) &&
                 (puStack_208 != (undefined8 *)0x0)) {
                func_0x000140613c20(uStack_200,puStack_208,1);
              }
              if ((-0x7fffffffffffffff < (longlong)puStack_220) &&
                 (puStack_220 != (undefined8 *)0x0)) {
                func_0x000140613c20(uStack_218,puStack_220,1);
              }
            }
            if ((-0x7ffffffffffffffa < lStack_1f0) && (lStack_1f0 != 0)) {
              func_0x000140613c20(uStack_1e8,lStack_1f0,1);
            }
            lVar19 = lVar19 + -1;
          } while (lVar19 != 0);
        }
        return;
      }
      if ((short)uVar12 == 0) {
        do {
          auVar4 = *pauVar22;
          uVar6 = (ushort)(SUB161(auVar4 >> 7,0) & 1) | (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
          pauVar24 = pauVar24 + -0xd8;
          pauVar22 = pauVar22 + 1;
        } while (uVar6 == 0xffff);
        uVar12 = ~(uint)uVar6;
        uStack_5c = uVar12 & -(uint)uVar6 - 2;
      }
      else {
        uStack_5c = uVar12 - 1 & uVar12;
      }
      uVar18 = 0;
      for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
        uVar18 = uVar18 + 1;
      }
      lVar13 = (ulonglong)uVar18 * -0xd8;
      puVar23 = (ulonglong *)((longlong)pauVar24 + lVar13 + -0xc0);
      FUN_140d7bd90(&puStack_118,(longlong)pauVar24 + lVar13 + -0xd8);
      hashbrown__map__HashMap__rustc_entry(&puStack_238,param_1,&puStack_118);
      uVar10 = uStack_218;
      puVar9 = puStack_220;
      uVar8 = uStack_228;
      if (!SBORROW8(0,(longlong)puStack_238)) break;
      if (*(char *)(uStack_230 - 4) == '\x02') {
        bVar11 = *(byte *)((longlong)pauVar24 + lVar13 + -4);
        bVar15 = bVar11 - 2;
        if (1 < bVar15) {
          bVar15 = 2;
        }
        if (bVar15 == 0) {
          bVar11 = 2;
        }
        else if (bVar15 == 1) {
          bVar11 = 3;
        }
        else {
          bVar11 = bVar11 & 1;
        }
        *(byte *)(uStack_230 - 4) = bVar11;
      }
      lVar1 = uStack_230 - 0x78;
      lVar20 = uStack_230 - 0x48;
      asset__inheritable__impl$1__inherit_from(uStack_230 - 0xc0,puVar23);
      asset__inheritable__impl$1__inherit_from(lVar20,(longlong)pauVar24 + lVar13 + -0x48);
      param_1 = puStack_d0;
      asset__inheritable__impl$1__inherit_from(lVar1,(longlong)pauVar24 + lVar13 + -0x78);
      uVar12 = uStack_5c;
    }
    bVar15 = *(byte *)((longlong)pauVar24 + lVar13 + -4);
    bVar11 = bVar15 - 2;
    if (1 < bVar11) {
      bVar11 = 2;
    }
    puStack_b0 = puStack_238;
    if (bVar11 == 0) {
      uStack_a8 = 2;
    }
    else if (bVar11 == 2) {
      uStack_a8 = (ulonglong)(bVar15 & 1);
    }
    else {
      uStack_a8 = 3;
    }
    uVar16 = *puVar23 + 0x7ffffffffffffffe;
    if (1 < uVar16) {
      uVar16 = 2;
    }
    if (uVar16 == 2) {
      uVar16 = *puVar23 ^ 0x8000000000000000;
      if (1 < uVar16) {
        uVar16 = 2;
      }
      uStack_a0 = uStack_230;
      if (uVar16 == 2) {
        FUN_140d7bd90(&puStack_78,puVar23);
      }
      else if (uVar16 == 1) {
        puStack_78 = (undefined8 *)0x8000000000000001;
      }
      else {
        puStack_78 = (undefined8 *)0x8000000000000000;
      }
      lVar1 = *(longlong *)((longlong)pauVar24 + lVar13 + -0x90);
      uVar16 = lVar1 + 0x7fffffffffffffff;
      if (1 < uVar16) {
        uVar16 = 2;
      }
      if (uVar16 == 0) {
        puStack_58 = (undefined8 *)0x8000000000000001;
      }
      else if (uVar16 == 2) {
        puVar14 = (undefined8 *)0x8000000000000000;
        if (!SBORROW8(0,lVar1)) {
          FUN_140d7bd90(&puStack_238,(longlong)pauVar24 + lVar13 + -0x90);
          puVar14 = puStack_238;
        }
        uStack_c8 = uStack_230;
        uStack_240 = uStack_228;
        puStack_58 = puVar14;
      }
      else {
        puStack_58 = (undefined8 *)0x8000000000000002;
      }
      uVar16 = *(ulonglong *)((longlong)pauVar24 + lVar13 + -0xa8) ^ 0x8000000000000000;
      if (1 < uVar16) {
        uVar16 = 2;
      }
      if (uVar16 == 2) {
        FUN_140d7bd90(&puStack_238,(longlong)pauVar24 + lVar13 + -0xa8);
      }
      else if (uVar16 == 1) {
        puStack_238 = (undefined8 *)0x8000000000000001;
      }
      else {
        puStack_238 = (undefined8 *)0x8000000000000000;
      }
      uStack_210 = uStack_228;
      puStack_220 = puStack_238;
      uStack_108 = uStack_68;
      uStack_110 = CONCAT44(uStack_6c,uStack_70);
      puStack_118 = puStack_78;
      uStack_f0 = uStack_228;
      puStack_100 = puStack_238;
      uStack_f8 = uStack_230;
      puStack_238 = puStack_78;
      uStack_228 = uStack_68;
      uStack_218 = uStack_230;
      puStack_208 = puStack_58;
      uStack_200 = uStack_c8;
      uStack_1f8 = uStack_240;
      uVar21 = uStack_a0;
      uStack_230 = uStack_110;
    }
    else {
      uVar21 = uStack_230;
      if (uVar16 == 1) {
        puStack_238 = (undefined8 *)0x8000000000000003;
      }
      else {
        puStack_238 = (undefined8 *)0x8000000000000002;
      }
    }
    uVar12 = *(uint *)((longlong)pauVar24 + lVar13 + -0x48);
    puStack_58 = (undefined8 *)(ulonglong)uVar12;
    uVar16 = (ulonglong)(uVar12 - 4);
    if (1 < uVar12 - 4) {
      uVar16 = 2;
    }
    if ((int)uVar16 == 0) {
      uStack_c0 = 4;
    }
    else if ((int)uVar16 == 2) {
      uStack_c0 = 3;
      puStack_58 = (undefined8 *)uVar16;
      if (uVar12 != 3) {
        uStack_49 = *(undefined1 *)((longlong)pauVar24 + lVar13 + -8);
        if (uVar12 < 2) {
          uStack_bc = *(uint *)((longlong)pauVar24 + lVar13 + -0x38);
          if (uStack_bc < 2) goto LAB_1424251d5;
LAB_14242560e:
          uStack_90 = *(undefined4 *)((longlong)pauVar24 + lVar13 + -0x2c);
          uStack_98 = *(undefined8 *)((longlong)pauVar24 + lVar13 + -0x34);
          uStack_b8 = *(uint *)((longlong)pauVar24 + lVar13 + -0x28);
          if (1 < uStack_b8) goto LAB_142425631;
LAB_1424251e3:
          uStack_b4 = *(uint *)((longlong)pauVar24 + lVar13 + -0x18);
          uStack_140 = uStack_70;
          puStack_148 = puStack_78;
          uStack_158 = uStack_98;
          uStack_150 = uStack_90;
          puStack_168 = puStack_88;
          uStack_160 = uStack_80;
        }
        else {
          uStack_80 = *(undefined4 *)((longlong)pauVar24 + lVar13 + -0x3c);
          puStack_88 = *(undefined8 **)((longlong)pauVar24 + lVar13 + -0x44);
          uStack_bc = *(uint *)((longlong)pauVar24 + lVar13 + -0x38);
          if (1 < uStack_bc) goto LAB_14242560e;
LAB_1424251d5:
          uStack_b8 = *(uint *)((longlong)pauVar24 + lVar13 + -0x28);
          if (uStack_b8 < 2) goto LAB_1424251e3;
LAB_142425631:
          uStack_140 = *(undefined4 *)((longlong)pauVar24 + lVar13 + -0x1c);
          puStack_148 = *(undefined8 **)((longlong)pauVar24 + lVar13 + -0x24);
          uStack_b4 = *(uint *)((longlong)pauVar24 + lVar13 + -0x18);
          uStack_158 = uStack_98;
          uStack_150 = uStack_90;
          puStack_168 = puStack_88;
          uStack_160 = uStack_80;
        }
        if (1 < uStack_b4) {
          uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)((longlong)pauVar24 + lVar13 + -0xc)
                               );
          puStack_118 = *(undefined8 **)((longlong)pauVar24 + lVar13 + -0x14);
        }
        bVar15 = *(byte *)((longlong)pauVar24 + lVar13 + -7);
        bVar11 = bVar15 - 2;
        if (1 < bVar11) {
          bVar11 = 2;
        }
        if (bVar11 == 0) {
          puStack_58 = (undefined8 *)0x2;
        }
        else if (bVar11 == 2) {
          puStack_58 = (undefined8 *)(ulonglong)(bVar15 & 1);
        }
        else {
          puStack_58 = (undefined8 *)0x3;
        }
        puStack_138 = puStack_118;
        uStack_130 = (undefined4)uStack_110;
        uStack_c0 = uVar12;
        uStack_98 = uStack_158;
        uStack_90 = uStack_150;
        puStack_88 = puStack_168;
        uStack_80 = uStack_160;
        puStack_78 = puStack_148;
        uStack_70 = uStack_140;
      }
    }
    else {
      uStack_c0 = 5;
    }
    uVar17 = *(ulonglong *)((longlong)pauVar24 + lVar13 + -0x78);
    uVar16 = uVar17 + 0x7ffffffffffffffb;
    if (1 < uVar16) {
      uVar16 = 2;
    }
    puVar14 = (undefined8 *)0x8000000000000005;
    if (uVar16 != 0) {
      if (uVar16 == 2) {
        puVar14 = (undefined8 *)0x8000000000000004;
        if (uVar17 != 0x8000000000000004) {
          uVar16 = uVar17 + 0x7ffffffffffffffe;
          if (1 < uVar16) {
            uVar16 = 2;
          }
          puVar14 = (undefined8 *)0x8000000000000002;
          if (uVar16 != 0) {
            if (uVar16 == 2) {
              puVar23 = (ulonglong *)(ulonglong)*(byte *)((longlong)pauVar24 + lVar13 + -0x50);
              uStack_7c = *(uint *)((longlong)pauVar24 + lVar13 + -0x60);
              if (1 < uStack_7c) {
                uStack_70 = *(undefined4 *)((longlong)pauVar24 + lVar13 + -0x54);
                puStack_78 = *(undefined8 **)((longlong)pauVar24 + lVar13 + -0x5c);
              }
              uVar17 = uVar17 ^ 0x8000000000000000;
              if (1 < uVar17) {
                uVar17 = 2;
              }
              puVar14 = (undefined8 *)0x8000000000000000;
              if (uVar17 != 0) {
                if (uVar17 == 2) {
                  uStack_a0 = uVar21;
                  FUN_140d7bd90(&puStack_118,(longlong)pauVar24 + lVar13 + -0x78);
                  puVar14 = puStack_118;
                  uVar21 = uStack_a0;
                }
                else {
                  puVar14 = (undefined8 *)0x8000000000000001;
                }
              }
              uStack_170 = uStack_70;
              puStack_178 = puStack_78;
              uStack_268 = uStack_110;
              uStack_260 = uStack_108;
            }
            else {
              puVar14 = (undefined8 *)0x8000000000000003;
            }
          }
        }
      }
      else {
        puVar14 = (undefined8 *)0x8000000000000006;
      }
    }
    puStack_118 = puStack_238;
    uStack_110 = uStack_230;
    uStack_108 = uStack_228;
    puStack_100 = puStack_220;
    uStack_f8 = uStack_218;
    uStack_f0 = uStack_210;
    puStack_e8 = puStack_208;
    uStack_e0 = uStack_200;
    uStack_d8 = uStack_1f8;
    puStack_78 = puStack_168;
    uStack_70 = uStack_160;
    uStack_98 = uStack_158;
    uStack_90 = uStack_150;
    uStack_80 = uStack_140;
    puStack_88 = puStack_148;
    uStack_120 = uStack_130;
    puStack_128 = puStack_138;
    pauVar5 = (undefined1 (*) [16])*puVar9;
    uVar16 = puVar9[1];
    uVar17 = uVar16 & uVar10;
    auVar4 = *(undefined1 (*) [16])(*pauVar5 + uVar17);
    uVar6 = (ushort)(SUB161(auVar4 >> 7,0) & 1) | (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
    uVar12 = (uint)uVar6;
    if (uVar6 == 0) {
      lVar13 = 0x10;
      do {
        uVar17 = uVar17 + lVar13 & uVar16;
        auVar4 = *(undefined1 (*) [16])(*pauVar5 + uVar17);
        lVar13 = lVar13 + 0x10;
        uVar12 = (uint)(ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar4[0xf] >> 7) << 0xf);
      } while (uVar12 == 0);
    }
    uVar18 = 0;
    for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar18 = uVar18 + 1;
    }
    uVar17 = uVar18 + uVar17 & uVar16;
    bVar15 = (*pauVar5)[uVar17];
    if (-1 < (char)bVar15) {
      auVar4 = *pauVar5;
      uVar12 = 0;
      for (uVar18 = (uint)(ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar4[0xf] >> 7) << 0xf); (uVar18 & 1) == 0;
          uVar18 = uVar18 >> 1 | 0x80000000) {
        uVar12 = uVar12 + 1;
      }
      uVar17 = (ulonglong)uVar12;
      bVar15 = (*pauVar5)[uVar17];
    }
    bVar11 = (byte)(uVar10 >> 0x39);
    (*pauVar5)[uVar17] = bVar11;
    pauVar5[1][uVar17 - 0x10 & uVar16] = bVar11;
    puVar9[2] = puVar9[2] - (ulonglong)(bVar15 & 1);
    lVar13 = uVar17 * -0xd8;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0xd8) = puStack_b0;
    *(ulonglong *)((longlong)pauVar5 + lVar13 + -0xd0) = uVar21;
    *(ulonglong *)((longlong)pauVar5 + lVar13 + -200) = uVar8;
    puVar2 = (undefined8 *)((longlong)pauVar5 + lVar13 + -0xc0);
    *puVar2 = puStack_238;
    puVar2[1] = uStack_230;
    puVar3 = (ulonglong *)((longlong)pauVar5 + lVar13 + -0xb0);
    *puVar3 = uStack_228;
    puVar3[1] = (ulonglong)puStack_220;
    puVar3 = (ulonglong *)((longlong)pauVar5 + lVar13 + -0xa0);
    *puVar3 = uStack_218;
    puVar3[1] = uStack_210;
    puVar2 = (undefined8 *)((longlong)pauVar5 + lVar13 + -0x90);
    *puVar2 = puStack_208;
    puVar2[1] = uStack_200;
    *(ulonglong *)((longlong)pauVar5 + lVar13 + -0x80) = uStack_1f8;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0x78) = puVar14;
    auVar4._8_8_ = uStack_260;
    auVar4._0_8_ = uStack_268;
    *(undefined1 (*) [16])((longlong)pauVar5 + lVar13 + -0x70) = auVar4;
    *(uint *)((longlong)pauVar5 + lVar13 + -0x60) = uStack_7c;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0x5c) = puStack_178;
    *(undefined4 *)((longlong)pauVar5 + lVar13 + -0x54) = uStack_170;
    *(char *)((longlong)pauVar5 + lVar13 + -0x50) = (char)puVar23;
    *(uint *)((longlong)pauVar5 + lVar13 + -0x48) = uStack_c0;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0x44) = puStack_168;
    *(undefined4 *)((longlong)pauVar5 + lVar13 + -0x3c) = uStack_160;
    *(uint *)((longlong)pauVar5 + lVar13 + -0x38) = uStack_bc;
    *(undefined4 *)((longlong)pauVar5 + lVar13 + -0x2c) = uStack_150;
    *(undefined8 *)((longlong)pauVar5 + lVar13 + -0x34) = uStack_158;
    *(uint *)((longlong)pauVar5 + lVar13 + -0x28) = uStack_b8;
    *(undefined4 *)((longlong)pauVar5 + lVar13 + -0x1c) = uStack_140;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0x24) = puStack_148;
    *(uint *)((longlong)pauVar5 + lVar13 + -0x18) = uStack_b4;
    *(undefined4 *)((longlong)pauVar5 + lVar13 + -0xc) = uStack_130;
    *(undefined8 **)((longlong)pauVar5 + lVar13 + -0x14) = puStack_138;
    *(undefined1 *)((longlong)pauVar5 + lVar13 + -8) = uStack_49;
    *(char *)((longlong)pauVar5 + lVar13 + -7) = (char)puStack_58;
    *(char *)((longlong)pauVar5 + lVar13 + -4) = (char)uStack_a8;
    puVar9[3] = puVar9[3] + 1;
    param_1 = puStack_d0;
    uVar12 = uStack_5c;
  } while( true );
}
