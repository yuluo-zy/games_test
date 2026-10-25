/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_140996080(undefined8 *param_1,longlong *param_2,undefined8 *param_3,longlong *param_4,
                  undefined8 *param_5,longlong *param_6,longlong *param_7)
{
  byte *pbVar1;
  longlong *plVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  undefined8 *puVar6;
  code *pcVar7;
  uint uVar8;
  float fVar9;
  longlong lVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined8 *puVar13;
  ulonglong uVar14;
  longlong *plVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  ushort uVar23;
  longlong lVar24;
  longlong lVar25;
  undefined *puVar26;
  longlong *plVar27;
  undefined *puVar28;
  longlong *plVar29;
  undefined1 uVar30;
  byte bVar31;
  char cVar32;
  char cVar41;
  char cVar42;
  float fVar33;
  char cVar43;
  float fVar34;
  uint uVar35;
  uint uVar36;
  float fVar37;
  undefined8 uVar38;
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  float fVar44;
  undefined1 auVar45 [16];
  float fVar47;
  undefined1 auVar46 [16];
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  float fVar55;
  float fVar56;
  float fVar57;
  float fVar58;
  float fVar59;
  float fVar60;
  float fVar61;
  longlong lStack_248;
  undefined *puStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined1 (*pauStack_228) [16];
  undefined1 (*pauStack_220) [16];
  undefined1 *puStack_218;
  undefined *puStack_210;
  undefined8 uStack_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined *puStack_1f8;
  undefined8 uStack_1f0;
  undefined8 *puStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1b8;
  float fStack_1b0;
  undefined8 uStack_1a8;
  float fStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 *puStack_188;
  longlong *plStack_180;
  longlong lStack_178;
  undefined8 uStack_170;
  undefined4 *puStack_168;
  longlong lStack_160;
  longlong *plStack_158;
  undefined8 uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  longlong *plStack_140;
  undefined8 uStack_138;
  code *pcStack_130;
  ulonglong uStack_128;
  longlong *plStack_120;
  undefined1 uStack_111;
  longlong lStack_110;
  undefined4 *puStack_108;
  longlong *plStack_100;
  undefined4 uStack_f8;
  char cStack_f1;
  undefined8 uStack_f0;
  uStack_f0 = 0xfffffffffffffffe;
  cStack_f1 = func_0x000140c16a00();
  if (cStack_f1 == '\0') {
code_r0x000140996176:
    uStack_198 = 0;
    uVar38 = uStack_150;
    if (*(char *)(*param_7 + 0x1c) == '\0') {
      return;
    }
    goto code_r0x0001409963a4;
  }
  puVar26 = &UNK_142abac02;
  plVar27 = (longlong *)0x3e;
  uVar17 = 0x3e;
  lVar10 = FUN_140992af0(&UNK_142abac02);
  if (lVar10 == 1) {
    if (uVar17 != 0) {
      if (uVar17 < 0x3e) {
        if ((char)(&UNK_142abac02)[uVar17] < -0x40) {
code_r0x000140996156:
          FUN_1428d9940(&UNK_142abac02,0x3e,0,uVar17,&UNK_142abacc8);
          goto code_r0x000140996176;
        }
      }
      else if (uVar17 != 0x3e) goto code_r0x000140996156;
    }
    puVar26 = &UNK_142abac02;
    lVar10 = FUN_140992af0(&UNK_142abac02);
    if (lVar10 == 1) {
      uVar16 = uVar17 + 2;
      if (uVar16 != 0) {
        uVar30 = uVar16 == 0x3e;
        if (0x3d < uVar16) goto code_r0x0001409961ed;
        uVar30 = (&UNK_142abac04)[uVar17] == -0x41;
        if ((char)(&UNK_142abac04)[uVar17] < -0x40) {
          do {
            uVar17 = 0x3e;
            FUN_1428d9940(&UNK_142abac02);
code_r0x0001409961ed:
          } while (!(bool)uVar30);
        }
      }
      plVar27 = (longlong *)(0x3c - uVar17);
      puVar26 = &UNK_142abac02 + uVar16;
    }
  }
  puVar11 = &UNK_142abae2c;
  puVar28 = &UNK_142abadf5;
  do {
    bVar31 = puVar11[-1];
    uVar35 = (uint)(char)bVar31;
    plStack_140 = param_4;
    plStack_100 = plVar27;
    if ((char)bVar31 < '\0') {
      bVar3 = puVar11[-2];
      if ((char)bVar3 < -0x40) {
        bVar4 = puVar11[-3];
        if ((char)bVar4 < -0x40) {
          pbVar1 = puVar11 + -4;
          puVar11 = puVar11 + -4;
          uVar35 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar11 = puVar11 + -3;
          uVar35 = bVar4 & 0xf;
        }
        uVar35 = bVar3 & 0x3f | uVar35 << 6;
      }
      else {
        puVar11 = puVar11 + -2;
        uVar35 = bVar3 & 0x1f;
      }
      uVar35 = (uint)(bVar31 & 0x3f) | uVar35 << 6;
      if (uVar35 != 0x5c) goto code_r0x000140996299;
code_r0x0001409962b4:
      puVar12 = puVar11 + -0x142abadf5;
      puVar28 = puVar11 + -0x142abadf4;
      if (puVar28 != (undefined *)0x0) {
        uVar30 = puVar28 == (undefined *)0x37;
        if ((undefined *)0x36 < puVar28) goto code_r0x0001409962f6;
        uVar30 = puVar11[1] == -0x41;
        if ((char)puVar11[1] < -0x40) {
          do {
            puVar12 = (undefined *)FUN_1428d9940(&UNK_142abadf5,0x37);
code_r0x0001409962f6:
          } while (!(bool)uVar30);
        }
      }
      lVar10 = 0x36 - (longlong)puVar12;
      puVar28 = &UNK_142abadf5 + (longlong)puVar28;
      goto code_r0x000140996309;
    }
    puVar11 = puVar11 + -1;
    if (uVar35 == 0x5c) goto code_r0x0001409962b4;
code_r0x000140996299:
    if (uVar35 == 0x2f) goto code_r0x0001409962b4;
  } while (puVar11 != &UNK_142abadf5);
  lVar10 = 0x37;
code_r0x000140996309:
  plVar27 = (longlong *)func_0x000140c18130();
  if (*plVar27 == 1) {
    plVar27 = plVar27 + 1;
code_r0x000140996322:
    if (*plVar27 != 0) goto code_r0x000140996ffd;
    *plVar27 = -1;
    plStack_120 = plVar27;
    uVar38 = FUN_140c16d40(plVar27 + 1,puVar26,plStack_100,puVar28,lVar10,1,0);
    *plStack_120 = *plStack_120 + 1;
    uStack_198 = 1;
    param_4 = plStack_140;
    uStack_190 = uVar38;
    if (*(char *)(*param_7 + 0x1c) != '\0') {
code_r0x0001409963a4:
      uStack_150 = uVar38;
      puVar5 = (ulonglong *)*param_1;
      lVar10 = param_1[1];
      uVar17 = *puVar5;
      uVar14 = 0;
      uVar16 = uVar17 - *(ulonglong *)(lVar10 + 0x18);
      if (uVar17 < *(ulonglong *)(lVar10 + 0x18)) {
        uVar16 = uVar14;
      }
      uVar20 = uVar17 - *(ulonglong *)(lVar10 + 0x38);
      if (uVar17 < *(ulonglong *)(lVar10 + 0x38)) {
        uVar20 = uVar14;
      }
      plVar27 = (longlong *)(uVar16 * 0x20 + *(longlong *)(lVar10 + 8));
      uVar17 = *(ulonglong *)(lVar10 + 0x10) - uVar16;
      if (*(ulonglong *)(lVar10 + 0x10) < uVar16) {
        plVar27 = (longlong *)0x8;
        uVar17 = uVar14;
      }
      plVar15 = (longlong *)(uVar20 * 0x20 + *(longlong *)(lVar10 + 0x28));
      uVar16 = *(ulonglong *)(lVar10 + 0x30) - uVar20;
      if (*(ulonglong *)(lVar10 + 0x30) < uVar20) {
        plVar15 = (longlong *)0x8;
        uVar16 = uVar14;
      }
      *puVar5 = (*(longlong *)(lVar10 + 0x40) - uVar16) - uVar17;
      fVar9 = _UNK_142925970;
      plStack_100 = plVar27 + uVar17 * 4;
      plStack_120 = plVar15 + uVar16 * 4;
      uStack_148 = *(undefined4 *)((longlong)param_2 + 0x1c);
      plStack_158 = (longlong *)*param_2;
      puStack_188 = (undefined4 *)param_2[2];
      uStack_170 = *param_5;
      uStack_f8 = *(undefined4 *)((longlong)param_4 + 0x1c);
      lStack_110 = *param_4;
      puStack_108 = (undefined4 *)param_4[2];
      puVar6 = (undefined8 *)*param_3;
      puStack_168 = (undefined4 *)param_3[2];
      uStack_144 = *(undefined4 *)((longlong)param_3 + 0x1c);
      lStack_160 = *param_6;
      plStack_140 = plStack_158 + 6;
      plStack_180 = plStack_158 + 4;
      fVar48 = _UNK_142abaaa8;
      fVar51 = _UNK_142abaaa0;
      fVar47 = _UNK_142abaaa4;
      fVar34 = _UNK_1429ddce8;
      fVar58 = _UNK_1429258e0;
      fVar37 = _UNK_1429258e4;
code_r0x0001409964f6:
      do {
        if ((plVar27 == (longlong *)0x0) || (plVar27 == plStack_100)) {
          if ((plVar15 == (longlong *)0x0) || (plVar15 == plStack_120)) {
            if (cStack_f1 == '\0') {
              return;
            }
            plVar27 = (longlong *)func_0x000140c18130();
            uVar38 = uStack_150;
            if (*plVar27 == 1) {
              plVar27 = plVar27 + 1;
            }
            else if ((*plVar27 != 0) ||
                    (plVar27 = (longlong *)FUN_1428f87d0(plVar27,0), plVar27 == (longlong *)0x0))
            goto code_r0x000140996ff1;
            if (*plVar27 != 0) goto code_r0x000140996ffd;
            *plVar27 = -1;
            plStack_100 = plVar27;
            FUN_140c170f0(plVar27 + 1,uVar38);
            goto code_r0x000140996f2c;
          }
          plVar27 = (longlong *)0x0;
          plVar2 = plVar15 + 4;
          plVar29 = plVar15;
        }
        else {
          plVar2 = plVar15;
          plVar29 = plVar27;
          plVar27 = plVar27 + 4;
        }
        plVar15 = plVar2;
        *puVar5 = *puVar5 + 1;
        pauStack_228 = (undefined1 (*) [16])*puVar6;
        puStack_218 = *pauStack_228 + puVar6[1] + 1;
        auVar39 = *pauStack_228;
        uStack_208 = puVar6[3];
        lStack_248 = 0;
        uStack_238 = 0;
        pauStack_220 = pauStack_228 + 1;
        puStack_210 = (undefined *)
                      CONCAT62(puStack_210._2_6_,
                               ~((ushort)(SUB161(auVar39 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar39[0xf] >> 7) << 0xf));
        FUN_1412fa4d0(&uStack_138,&lStack_248,&UNK_142aa91b0);
        if (uStack_128 < 10000) {
          if (uStack_138 != (longlong *)0x0) {
            func_0x000140613c20(pcStack_130,(longlong)uStack_138 << 3,8);
          }
          *puStack_188 = uStack_148;
          fVar33 = (float)func_0x000141625ae0(plStack_140);
          if (fVar9 <= fVar33) {
            lVar10 = plVar29[3];
            lStack_248 = lVar10;
            if (plStack_158[3] != 0) {
              uVar17 = FUN_1405b0d70(plStack_180,&lStack_248);
              lVar25 = *plStack_158;
              bVar31 = (byte)(uVar17 >> 0x39);
              auVar39 = ZEXT216(CONCAT11(bVar31,bVar31));
              auVar39 = pshuflw(auVar39,auVar39,0);
              lVar21 = 0;
              do {
                uVar17 = plStack_158[1] & uVar17;
                auVar40 = *(undefined1 (*) [16])(lVar25 + uVar17);
                cVar32 = auVar39[0];
                auVar46[0] = -(auVar40[0] == cVar32);
                cVar41 = auVar39[1];
                auVar46[1] = -(auVar40[1] == cVar41);
                cVar42 = auVar39[2];
                auVar46[2] = -(auVar40[2] == cVar42);
                cVar43 = auVar39[3];
                auVar46[3] = -(auVar40[3] == cVar43);
                auVar46[4] = -(auVar40[4] == cVar32);
                auVar46[5] = -(auVar40[5] == cVar41);
                auVar46[6] = -(auVar40[6] == cVar42);
                auVar46[7] = -(auVar40[7] == cVar43);
                auVar46[8] = -(auVar40[8] == cVar32);
                auVar46[9] = -(auVar40[9] == cVar41);
                auVar46[10] = -(auVar40[10] == cVar42);
                auVar46[0xb] = -(auVar40[0xb] == cVar43);
                auVar46[0xc] = -(auVar40[0xc] == cVar32);
                auVar46[0xd] = -(auVar40[0xd] == cVar41);
                auVar46[0xe] = -(auVar40[0xe] == cVar42);
                auVar46[0xf] = -(auVar40[0xf] == cVar43);
                uVar23 = (ushort)(SUB161(auVar46 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar46[0xf] >> 7) << 0xf;
                uVar35 = (uint)uVar23;
                while (uVar23 != 0) {
                  uVar36 = 0;
                  for (uVar8 = uVar35; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
                    uVar36 = uVar36 + 1;
                  }
                  lVar24 = (uVar36 + uVar17 & plStack_158[1]) * -0x98;
                  if (*(longlong *)(lVar25 + -0x98 + lVar24) == lVar10) {
                    lVar10 = lVar25 + lVar24;
                    uVar17 = *(ulonglong *)(lVar25 + -0x78 + lVar24);
                    if (uVar17 == 0) {
                      lVar25 = 0;
                      uVar16 = 0;
code_r0x0001409967f1:
                      lVar21 = 0;
                    }
                    else {
                      uVar16 = *(ulonglong *)(lVar10 + -0x90);
                      uVar14 = 0;
                      if (uVar16 <= *(ulonglong *)(lVar10 + -0x80)) {
                        uVar14 = uVar16;
                      }
                      lVar25 = *(ulonglong *)(lVar10 + -0x80) - uVar14;
                      lVar21 = uVar17 - (uVar16 - lVar25);
                      if (uVar17 < uVar16 - lVar25 || lVar21 == 0) {
                        uVar16 = lVar25 + uVar17;
                        goto code_r0x0001409967f1;
                      }
                    }
                    lVar24 = *(longlong *)(lVar10 + -0x88);
                    lVar22 = uVar16 * 0x28 + lVar25 * -0x28;
                    lVar25 = lVar24 + lVar25 * 0x28 + -0x28;
                    goto code_r0x000140996820;
                  }
                  uVar23 = (ushort)(uVar35 - 1) & (ushort)uVar35;
                  uVar35 = CONCAT22((short)(uVar35 - 1 >> 0x10),uVar23);
                }
                auVar45[0] = -(auVar40[0] == UNK_142925990);
                auVar45[1] = -(auVar40[1] == UNK_142925991);
                auVar45[2] = -(auVar40[2] == UNK_142925992);
                auVar45[3] = -(auVar40[3] == UNK_142925993);
                auVar45[4] = -(auVar40[4] == UNK_142925994);
                auVar45[5] = -(auVar40[5] == UNK_142925995);
                auVar45[6] = -(auVar40[6] == UNK_142925996);
                auVar45[7] = -(auVar40[7] == UNK_142925997);
                auVar45[8] = -(auVar40[8] == UNK_142925998);
                auVar45[9] = -(auVar40[9] == UNK_142925999);
                auVar45[10] = -(auVar40[10] == UNK_14292599a);
                auVar45[0xb] = -(auVar40[0xb] == UNK_14292599b);
                auVar45[0xc] = -(auVar40[0xc] == UNK_14292599c);
                auVar45[0xd] = -(auVar40[0xd] == UNK_14292599d);
                auVar45[0xe] = -(auVar40[0xe] == UNK_14292599e);
                auVar45[0xf] = -(auVar40[0xf] == UNK_14292599f);
                if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar45 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar45 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar45 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar45 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar45 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x5f,0) & 1) != 0) || (SUB161(auVar45 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') break;
                uVar17 = uVar17 + lVar21 + 0x10;
                lVar21 = lVar21 + 0x10;
              } while( true );
            }
            if (1 < *puRam000000014362c3a8) {
              uStack_138 = plVar29 + 3;
              pcStack_130 = FUN_140d8d6c0;
              puVar13 = (undefined8 *)func_0x00014067dd80(&UNK_142abaed0);
              pauStack_228 = (undefined1 (*) [16])*puVar13;
              pauStack_220 = (undefined1 (*) [16])puVar13[1];
              uStack_1fc = *(undefined4 *)(puVar13 + 2);
              puStack_218 = (undefined1 *)0x2;
              puStack_210 = &UNK_142abaee8;
              uStack_208 = 0x2c;
              puStack_1f8 = &UNK_142abaeb0;
              uStack_1f0 = 2;
              puStack_1e8 = &uStack_138;
              uStack_1e0 = 1;
              uStack_1d8 = 0;
              lStack_248 = 0;
              puStack_240 = &UNK_142abaee8;
              uStack_238 = 0x2c;
              uStack_230 = 0;
              uStack_200 = 1;
              func_0x000140d4a330(&uStack_111);
            }
          }
        }
        else if (uStack_138 != (longlong *)0x0) {
          func_0x000140613c20(pcStack_130,(longlong)uStack_138 << 3,8);
        }
      } while( true );
    }
    plVar27 = (longlong *)func_0x000140c18130();
    if (*plVar27 == 1) {
      plVar27 = plVar27 + 1;
    }
    else if ((*plVar27 != 0) ||
            (plVar27 = (longlong *)FUN_1428f87d0(plVar27,0), plVar27 == (longlong *)0x0))
    goto code_r0x000140996ff1;
    if (*plVar27 == 0) {
      *plVar27 = -1;
      plStack_100 = plVar27;
      FUN_140c170f0(plVar27 + 1,uVar38);
code_r0x000140996f2c:
      *plStack_100 = *plStack_100 + 1;
      return;
    }
  }
  else {
    if ((*plVar27 == 0) &&
       (plVar27 = (longlong *)FUN_1428f87d0(plVar27,0), plVar27 != (longlong *)0x0))
    goto code_r0x000140996322;
code_r0x000140996ff1:
    FUN_1428d80d0(&UNK_142c3aaa0);
  }
code_r0x000140996ffd:
  FUN_1428d9250(&UNK_142c3ab70);
code_r0x000140997009:
  FUN_1428d9310(&UNK_142abae30);
code_r0x000140997023:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
  while( true ) {
    lVar19 = lVar25 + 0x28;
    lVar22 = lVar22 + -0x28;
    plVar2 = (longlong *)(lVar25 + 0x28);
    lVar25 = lVar19;
    if (*plVar2 == plVar29[1]) break;
code_r0x000140996820:
    if (lVar22 == 0) {
      lVar21 = lVar21 * 0x28;
      lVar25 = lVar24 + -0x28;
      goto code_r0x000140996850;
    }
  }
  goto code_r0x00014099686a;
  while( true ) {
    lVar19 = lVar25 + 0x28;
    lVar21 = lVar21 + -0x28;
    plVar2 = (longlong *)(lVar25 + 0x28);
    lVar25 = lVar19;
    if (*plVar2 == plVar29[1]) break;
code_r0x000140996850:
    if (lVar21 == 0) goto code_r0x000140997009;
  }
code_r0x00014099686a:
  if (uVar17 == 0) {
    lVar25 = 0;
    uVar16 = 0;
code_r0x0001409968ad:
    lVar21 = 0;
  }
  else {
    uVar16 = *(ulonglong *)(lVar10 + -0x90);
    uVar14 = 0;
    if (uVar16 <= *(ulonglong *)(lVar10 + -0x80)) {
      uVar14 = uVar16;
    }
    lVar25 = *(ulonglong *)(lVar10 + -0x80) - uVar14;
    lVar21 = uVar17 - (uVar16 - lVar25);
    if (uVar17 < uVar16 - lVar25 || lVar21 == 0) {
      uVar16 = uVar17 + lVar25;
      goto code_r0x0001409968ad;
    }
  }
  lVar22 = uVar16 * 0x28 + lVar25 * -0x28;
  lVar25 = lVar24 + lVar25 * 0x28 + -0x28;
  do {
    lStack_178 = lVar10;
    if (lVar22 == 0) {
      lVar21 = lVar21 * 0x28;
      lVar10 = lVar24 + -0x28;
      goto code_r0x0001409968f0;
    }
    lVar18 = lVar25 + 0x28;
    lVar22 = lVar22 + -0x28;
    plVar2 = (longlong *)(lVar25 + 0x28);
    lVar25 = lVar18;
  } while (*plVar2 != plVar29[2]);
code_r0x00014099690a:
  fVar44 = *(float *)(lVar19 + 8) + *(float *)(lVar19 + 8) + fVar51;
  fVar33 = (float)FUN_141741c60(uStack_170,fVar44,
                                *(float *)(lVar19 + 0xc) + *(float *)(lVar19 + 0xc) + fVar47,
                                *(float *)(lVar19 + 0x10) + *(float *)(lVar19 + 0x10) + fVar48);
  if (fVar34 < fVar33) {
    fVar54 = (*(float *)(lVar18 + 0x14) + *(float *)(lVar19 + 0x14)) * _UNK_142925870;
    fVar49 = ((float)*(undefined8 *)(lVar19 + 0x18) + (float)*(undefined8 *)(lVar18 + 0x18)) *
             fVar58;
    fVar51 = ((float)((ulonglong)*(undefined8 *)(lVar19 + 0x18) >> 0x20) +
             (float)((ulonglong)*(undefined8 *)(lVar18 + 0x18) >> 0x20)) * fVar37;
    *puStack_108 = uStack_f8;
    fVar34 = (float)FUN_141625cb0(lStack_110);
    fVar48 = _UNK_142925904 / SQRT(fVar51 * fVar51 + fVar54 * fVar54 + fVar49 * fVar49);
    fVar54 = fVar54 * fVar48;
    fVar49 = fVar48 * fVar49;
    fVar48 = fVar48 * fVar51;
    fVar34 = fVar34 * _UNK_142925920;
    fVar44 = fVar44 * _UNK_142925920;
    fVar50 = *(float *)(lVar18 + 0x10) - *(float *)(lVar19 + 0x10);
    fVar51 = (float)*(undefined8 *)(lVar19 + 8);
    fVar33 = (float)*(undefined8 *)(lVar18 + 8) - fVar51;
    fVar47 = (float)((ulonglong)*(undefined8 *)(lVar19 + 8) >> 0x20);
    fVar52 = (float)((ulonglong)*(undefined8 *)(lVar18 + 8) >> 0x20) - fVar47;
    fStack_1a0 = _UNK_142925904 / SQRT(fVar52 * fVar52 + fVar33 * fVar33 + fVar50 * fVar50);
    fVar33 = fStack_1a0 * fVar33;
    fVar52 = fStack_1a0 * fVar52;
    fStack_1a0 = fStack_1a0 * fVar50;
    uStack_1a8 = CONCAT44(fVar52,fVar33);
    fStack_1b0 = fVar44 * fStack_1a0 +
                 (fVar49 * fVar33 - fVar52 * fVar54) * fVar34 + *(float *)(lVar19 + 0x10);
    uStack_1b8 = CONCAT44(fVar44 * fVar52 +
                          fVar34 * (fStack_1a0 * fVar54 - fVar33 * fVar48) + fVar47,
                          fVar44 * fVar33 +
                          fVar34 * (fVar52 * fVar48 - fStack_1a0 * fVar49) + fVar51);
    *puStack_108 = uStack_f8;
    fVar51 = (float)FUN_141625b40(lStack_110);
    fVar34 = 0.0;
    if (0.0 <= fVar51) {
      fVar34 = fVar51;
    }
    fVar51 = _UNK_142925904;
    if (fVar34 <= _UNK_142925904) {
      fVar51 = fVar34;
    }
    fVar53 = (fVar51 - (_UNK_142925904 - fVar51)) * _UNK_142925870;
    *puStack_108 = uStack_f8;
    uVar38 = FUN_141625b40(lStack_110);
    *puStack_108 = uStack_f8;
    auVar40._0_8_ = FUN_141625b40(lStack_110);
    auVar40._8_4_ = (int)((ulonglong)auVar40._0_8_ >> 0x20);
    auVar40._12_4_ = (int)((ulonglong)uVar38 >> 0x20);
    auVar39._8_8_ = auVar40._8_8_;
    auVar39._0_4_ = (undefined4)auVar40._0_8_;
    auVar39._4_4_ = (int)uVar38;
    auVar39 = maxps(ZEXT816(0),auVar39);
    fVar34 = (float)UNK_14292e6b0;
    fVar51 = UNK_14292e6b0._4_4_;
    auVar39 = minps(_UNK_14292e6b0,auVar39);
    *puStack_108 = uStack_f8;
    FUN_141625b40(lStack_110);
    *puStack_108 = uStack_f8;
    uVar35 = FUN_141625b40(lStack_110);
    *puStack_108 = uStack_f8;
    uVar36 = FUN_141625b40(lStack_110);
    fVar53 = fVar53 + fVar54;
    fVar59 = (auVar39._0_4_ - (fVar34 - auVar39._0_4_)) * fVar58 + fVar49;
    fVar61 = (auVar39._4_4_ - (fVar51 - auVar39._4_4_)) * fVar37 + fVar48;
    fVar44 = _UNK_142925904 / SQRT(fVar61 * fVar61 + fVar53 * fVar53 + fVar59 * fVar59);
    fVar37 = (float)func_0x000142923fd0();
    fVar58 = _UNK_1429259a8;
    fVar34 = (float)func_0x000142923e80();
    fVar47 = _UNK_142abaaa4;
    fVar51 = _UNK_142abaaa0;
    fVar52 = fVar34 + fVar34;
    fVar34 = fVar34 * fVar34;
    fVar60 = fVar37 * fVar37 + 0.0 + 0.0;
    fVar55 = fVar48 * fVar58 + fVar49 * fVar37 + fVar54 * 0.0;
    fVar55 = fVar55 + fVar55;
    fVar56 = fVar55 * 0.0 + (fVar34 - fVar60) * fVar54 +
             (fVar37 * fVar48 - fVar49 * fVar58) * fVar52;
    fVar57 = fVar55 * fVar58 + (fVar34 - fVar60) * fVar48 +
             (fVar49 * 0.0 - fVar54 * fVar37) * fVar52;
    fVar50 = (float)(uVar35 & _UNK_14292e980) / _UNK_1429258d0;
    fVar33 = _UNK_142925904;
    if (fVar50 <= _UNK_142925904) {
      fVar33 = fVar50;
    }
    fVar33 = (fVar55 * fVar37 + (fVar34 - fVar60) * fVar49 +
             (fVar58 * fVar54 - fVar48 * 0.0) * fVar52) - fVar33;
    fVar52 = _UNK_142925904 / SQRT(fVar33 * fVar33 + fVar56 * fVar56 + fVar57 * fVar57);
    *puStack_168 = uStack_144;
    fVar48 = _UNK_142abaaa8;
    fVar34 = _UNK_1429ddce8;
    fVar37 = _UNK_1429258e4;
    fVar58 = _UNK_1429258e0;
    uStack_138 = (longlong *)CONCAT44(fVar33 * fVar52,fVar52 * fVar56);
    pcStack_130 = (code *)CONCAT44(pcStack_130._4_4_,fVar57 * fVar52);
    lStack_248 = CONCAT44(fVar44 * fVar59,fVar44 * fVar53);
    puStack_240 = (undefined *)CONCAT44(puStack_240._4_4_,fVar61 * fVar44);
    lVar10 = plVar29[3];
    lVar25 = plVar29[1];
    lVar21 = plVar29[2];
    *puStack_108 = uStack_f8;
    fVar44 = (float)func_0x000141625ae0(lStack_110);
    fVar52 = (float)(uVar36 & _UNK_14292e980) / _UNK_1429cd078;
    fVar33 = _UNK_142925904;
    if (fVar52 <= _UNK_142925904) {
      fVar33 = fVar52;
    }
    FUN_1408fe280(puVar6,&uStack_1b8,&uStack_138,&lStack_248,&uStack_1a8,lVar10,lVar25,lVar21,
                  (_UNK_142925904 - fVar33) * _UNK_142a16cb8 + fVar33 * _UNK_1429cd128,
                  fVar44 * _UNK_1429258d8 + _UNK_142960690,*(undefined4 *)(lStack_160 + 0x2c),
                  *(undefined8 *)(lStack_178 + -0x30));
  }
  goto code_r0x0001409964f6;
  while( true ) {
    lVar18 = lVar10 + 0x28;
    lVar21 = lVar21 + -0x28;
    plVar2 = (longlong *)(lVar10 + 0x28);
    lVar10 = lVar18;
    if (*plVar2 == plVar29[2]) break;
code_r0x0001409968f0:
    if (lVar21 == 0) {
      FUN_1428d9310(&UNK_142abae48);
      goto code_r0x000140997023;
    }
  }
  goto code_r0x00014099690a;
}
