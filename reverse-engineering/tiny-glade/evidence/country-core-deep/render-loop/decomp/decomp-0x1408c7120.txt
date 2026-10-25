
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408c7120(longlong param_1,longlong param_2)

{
  byte *pbVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined1 auVar5 [16];
  byte bVar6;
  byte bVar7;
  byte bVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  code *pcVar11;
  ushort uVar12;
  uint uVar13;
  float fVar14;
  undefined8 uVar15;
  longlong ******pppppplVar16;
  char cVar17;
  uint uVar18;
  undefined4 uVar19;
  longlong lVar20;
  longlong lVar21;
  longlong *plVar22;
  undefined8 uVar23;
  longlong *******ppppppplVar24;
  undefined8 *puVar25;
  longlong *plVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  char *pcVar29;
  undefined *puVar30;
  undefined1 (*pauVar31) [16];
  undefined *puVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  undefined1 (*pauVar35) [16];
  uint uVar36;
  bool bVar37;
  longlong *******ppppppplStack_5c8;
  undefined8 uStack_5c0;
  undefined8 uStack_5b8;
  undefined8 uStack_5b0;
  undefined8 uStack_5a8;
  undefined8 uStack_5a0;
  undefined8 uStack_598;
  undefined8 uStack_590;
  undefined8 uStack_588;
  undefined8 uStack_580;
  longlong lStack_430;
  undefined4 uStack_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  undefined4 uStack_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined4 uStack_408;
  longlong *plStack_400;
  longlong *******ppppppplStack_3f8;
  longlong *******ppppppplStack_3f0;
  longlong ******pppppplStack_3e8;
  undefined8 uStack_3e0;
  undefined *puStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined *puStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined *puStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  longlong lStack_320;
  undefined8 uStack_318;
  undefined1 auStack_2e8 [38];
  char cStack_2c2;
  longlong lStack_2c0;
  longlong *******ppppppplStack_268;
  longlong *******ppppppplStack_260;
  longlong ******pppppplStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined *puStack_230;
  undefined8 uStack_228;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined *puStack_218;
  undefined8 uStack_210;
  longlong *******ppppppplStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
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
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined1 auStack_158 [16];
  undefined1 auStack_148 [22];
  char cStack_132;
  longlong lStack_130;
  undefined8 uStack_e8;
  longlong lStack_d0;
  undefined8 uStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  ulonglong uStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  longlong ******pppppplStack_90;
  longlong lStack_88;
  longlong ******pppppplStack_80;
  longlong *******ppppppplStack_78;
  longlong *plStack_70;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  lVar21 = param_2 + 0x38;
  uVar28 = 0x47c94cb78283a72e;
  lVar20 = FUN_1400dd6f0(lVar21,0x47c94cb78283a72e,0x8c173ded97323e9d);
  if (((lVar20 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar28)) ||
     (uVar28 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar28 * 8), uVar28 == 0)) {
code_r0x0001408c8aeb:
    FUN_1428d9310(&UNK_142aa2f08);
code_r0x0001408c8af7:
    FUN_1428d9310(&UNK_142aa2f20);
code_r0x0001408c8b03:
    FUN_1428d9310(&UNK_142aa2f38);
  }
  else {
    lVar20 = ~uVar28 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar20) == 0)
    goto code_r0x0001408c8aeb;
    plVar26 = *(longlong **)(*(longlong *)(param_2 + 0x1e0) + lVar20 + 0x10);
    uVar28 = 0x1e38c16726d0fb8d;
    lVar20 = FUN_1400dd6f0(lVar21,0x1e38c16726d0fb8d,0xec8924b33809588b);
    if (((lVar20 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar28)) ||
       (uVar28 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar28 * 8), uVar28 == 0))
    goto code_r0x0001408c8af7;
    lVar20 = ~uVar28 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar20) == 0)
    goto code_r0x0001408c8af7;
    lVar20 = *(longlong *)(param_2 + 0x1e0) + lVar20;
    lStack_c0 = *(longlong *)(lVar20 + 0x10);
    *(undefined4 *)(lVar20 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
    uVar28 = 0xcda6af2f6afb130e;
    lVar21 = FUN_1400dd6f0(lVar21,0xcda6af2f6afb130e,0x36e88a927bd70aa);
    if (((lVar21 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar28)) ||
       (uVar28 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar28 * 8), uVar28 == 0))
    goto code_r0x0001408c8b03;
    lVar21 = ~uVar28 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar21) == 0)
    goto code_r0x0001408c8b03;
    lVar21 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar21 + 0x10);
    if ((*(longlong *)(lVar21 + 0x30) == 0 && *(longlong *)(lVar21 + 0x10) == 0) &&
       (*(char *)(lStack_c0 + 0x40) == '\x01')) {
      return;
    }
    pppppplStack_90 = (longlong ******)0x0;
    lStack_88 = 8;
    pppppplStack_80 = (longlong ******)0x0;
    uStack_a8 = 0;
    lStack_a0 = 8;
    uStack_98 = 0;
    uStack_61 = 1;
    lStack_b0 = param_2;
    cVar17 = func_0x000140c16a00();
    if (cVar17 == '\0') {
      lStack_d0 = 0;
    }
    else {
      puVar32 = &UNK_142aa2f07;
      puVar30 = &UNK_142aa2eb1;
      do {
        bVar6 = puVar32[-1];
        uVar18 = (uint)(char)bVar6;
        if ((char)bVar6 < '\0') {
          bVar7 = puVar32[-2];
          if ((char)bVar7 < -0x40) {
            bVar8 = puVar32[-3];
            if ((char)bVar8 < -0x40) {
              pbVar1 = puVar32 + -4;
              puVar32 = puVar32 + -4;
              uVar18 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar32 = puVar32 + -3;
              uVar18 = bVar8 & 0xf;
            }
            uVar18 = bVar7 & 0x3f | uVar18 << 6;
          }
          else {
            puVar32 = puVar32 + -2;
            uVar18 = bVar7 & 0x1f;
          }
          uVar18 = (uint)(bVar6 & 0x3f) | uVar18 << 6;
          if (uVar18 != 0x5c) goto code_r0x0001408c73a4;
code_r0x0001408c73bc:
          puVar30 = puVar32 + -0x142aa2eb0;
          if (puVar30 != (undefined *)0x0) {
            if (puVar30 < (undefined *)0x56) {
              if ((char)puVar32[1] < -0x40) goto code_r0x0001408c73d3;
            }
            else if (puVar30 != (undefined *)0x56) {
code_r0x0001408c73d3:
              uStack_61 = 1;
              FUN_1428d9940(&UNK_142aa2eb1,0x56,puVar30,0x56,&UNK_142aa2db8);
              goto code_r0x0001408c8b97;
            }
          }
          lVar21 = 0x56 - (longlong)puVar30;
          puVar30 = puVar32 + 1;
          goto code_r0x0001408c7414;
        }
        puVar32 = puVar32 + -1;
        if (uVar18 == 0x5c) goto code_r0x0001408c73bc;
code_r0x0001408c73a4:
        if (uVar18 == 0x2f) goto code_r0x0001408c73bc;
      } while (puVar32 != &UNK_142aa2eb1);
      lVar21 = 0x56;
code_r0x0001408c7414:
      plVar22 = (longlong *)func_0x000140c18130();
      if (*plVar22 == 0) {
        uStack_61 = 1;
        ppppppplVar24 = (longlong *******)FUN_1428f87d0(plVar22,0);
        if (ppppppplVar24 == (longlong *******)0x0) goto code_r0x0001408c8b24;
      }
      else {
        if ((int)*plVar22 != 1) {
code_r0x0001408c8b24:
          uStack_61 = 1;
          FUN_1428d80d0(&UNK_142c3aaa0);
          goto code_r0x0001408c8b97;
        }
        ppppppplVar24 = (longlong *******)(plVar22 + 1);
      }
      if (*ppppppplVar24 != (longlong ******)0x0) {
        uStack_61 = 1;
        FUN_1428d9250(&UNK_142c3ab70);
        goto code_r0x0001408c8b97;
      }
      *ppppppplVar24 = (longlong ******)0xffffffffffffffff;
      ppppppplStack_78 = ppppppplVar24;
      uStack_c8 = FUN_140c16d40(ppppppplVar24 + 1,&UNK_142aa2f50,0x16,puVar30,lVar21,1,0);
      *ppppppplStack_78 = (longlong ******)((longlong)*ppppppplStack_78 + 1);
      lStack_d0 = 1;
    }
    fVar14 = _UNK_142960674;
    pauVar35 = *(undefined1 (**) [16])(lStack_c0 + 8);
    auVar5 = *pauVar35;
    pauVar31 = pauVar35 + 1;
    plStack_400 = plVar26 + 1;
    lVar21 = *(longlong *)(lStack_c0 + 0x20);
    uVar18 = ~(uint)(ushort)((ushort)(SUB161(auVar5 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar5[0xf] >> 7) << 0xf);
    lStack_430 = param_1;
    plStack_70 = plVar26;
code_r0x0001408c7549:
    if (lVar21 != 0) {
      if ((short)uVar18 == 0) {
        do {
          auVar5 = *pauVar31;
          uVar12 = (ushort)(SUB161(auVar5 >> 7,0) & 1) | (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar5[0xf] >> 7) << 0xf;
          pauVar35 = pauVar35 + -0x30;
          pauVar31 = pauVar31 + 1;
        } while (uVar12 == 0xffff);
        uVar18 = ~(uint)uVar12;
        uVar36 = uVar18 & -(uint)uVar12 - 2;
      }
      else {
        uVar36 = uVar18 - 1 & uVar18;
      }
      lVar21 = lVar21 + -1;
      uVar13 = 0;
      for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
        uVar13 = uVar13 + 1;
      }
      uVar28 = (ulonglong)uVar13;
      uVar18 = uVar36;
      if ((fVar14 < *(float *)(pauVar35[uVar28 * -3 + -1] + 8)) &&
         (*(longlong *)(pauVar35[uVar28 * -3 + -2] + 8) != 0)) {
        if (0x3fff < uStack_98) {
          if (iRam0000000143634cb8 != 3) {
            ppppppplStack_3f8 = (longlong *******)CONCAT71(ppppppplStack_3f8._1_7_,1);
            ppppppplStack_268 = (longlong *******)&ppppppplStack_3f8;
            FUN_1428d8820(0x143634cb8,0,&ppppppplStack_268,&UNK_142aa2c38,&UNK_142aa2fd8);
          }
          FUN_140542890(&ppppppplStack_268,uRam0000000143634cb0);
          if ((int)ppppppplStack_268 == 1) {
            ppppppplStack_3f8 = ppppppplStack_260;
            ppppppplStack_3f0 =
                 (longlong *******)CONCAT71(ppppppplStack_3f0._1_7_,pppppplStack_258._0_1_);
            FUN_1428d9760(&UNK_142aa2ff0,0x13,&ppppppplStack_3f8,&UNK_142aa2cd0,&UNK_142aa2fd8);
            goto code_r0x0001408c8b97;
          }
          ppppppplStack_78 = ppppppplStack_260;
          lStack_b8 = CONCAT71(lStack_b8._1_7_,pppppplStack_258._0_1_);
          ppppppplVar24 = (longlong *******)func_0x000140613c10(0xf,1);
          if (ppppppplVar24 == (longlong *******)0x0) {
            FUN_1428d8fe3(1,0xf,&UNK_142aa2d40);
            goto code_r0x0001408c8b97;
          }
          builtin_strncpy((char *)((longlong)ppppppplVar24 + 7),"path! ()",8);
          *ppppppplVar24 = (longlong ******)0x705f656c75646f6d;
          ppppppplStack_268 = (longlong *******)0xf;
          pppppplStack_258 = (longlong ******)0xf;
          ppppppplStack_260 = ppppppplVar24;
          FUN_1428a9a60(&ppppppplStack_268,0xf,0x18,1,1);
          uVar23 = _UNK_142aa300b;
          *(undefined8 *)((longlong)ppppppplStack_260 + (longlong)pppppplStack_258) = _UNK_142aa3003
          ;
          *(undefined8 *)((char *)((longlong)ppppppplStack_260 + (longlong)pppppplStack_258) + 8) =
               uVar23;
          builtin_strncpy((char *)((longlong)ppppppplStack_260 + (longlong)(pppppplStack_258 + 2)),
                          ":: Error",8);
          pppppplStack_3e8 = pppppplStack_258 + 3;
          ppppppplStack_3f8 = ppppppplStack_268;
          ppppppplStack_3f0 = ppppppplStack_260;
          if ((ulonglong)((longlong)ppppppplStack_268 - (longlong)pppppplStack_3e8) < 0x1f) {
            FUN_1428a9a60(&ppppppplStack_3f8,pppppplStack_3e8,0x1f,1,1);
          }
          uVar23 = _UNK_142aa3032;
          pcVar29 = (char *)((longlong)ppppppplStack_3f0 + (longlong)pppppplStack_3e8 + 0xf);
          *(ulonglong *)pcVar29 = CONCAT71(_UNK_142aa302b,UNK_142aa302a);
          *(undefined8 *)(pcVar29 + 8) = uVar23;
          uVar23 = CONCAT17(UNK_142aa302a,_UNK_142aa3023);
          *(undefined8 *)((longlong)ppppppplStack_3f0 + (longlong)pppppplStack_3e8) = _UNK_142aa301b
          ;
          *(undefined8 *)((char *)((longlong)ppppppplStack_3f0 + (longlong)pppppplStack_3e8) + 8) =
               uVar23;
          pppppplStack_3e8 = (longlong ******)((longlong)pppppplStack_3e8 + 0x1f);
          ppppppplStack_268 = ppppppplStack_3f8;
          ppppppplStack_260 = ppppppplStack_3f0;
          pppppplStack_258 = pppppplStack_3e8;
          cVar17 = FUN_14143b8b0(ppppppplStack_78 + 1,&ppppppplStack_268);
          if ((cVar17 == '\0') && (*plRam000000014362c3a8 != 0)) {
            puVar25 = (undefined8 *)func_0x00014067dd80(&UNK_142aa2fd8);
            uStack_248 = *puVar25;
            uStack_240 = puVar25[1];
            uStack_21c = *(undefined4 *)(puVar25 + 2);
            uStack_238 = 1;
            puStack_230 = &UNK_142aa3050;
            uStack_228 = 0x4c;
            puStack_218 = &UNK_142aa3040;
            uStack_210 = 1;
            ppppppplStack_208 = (longlong *******)&ppppppplStack_3f8;
            ppppppplStack_268 = (longlong *******)0x0;
            uStack_200 = 0;
            uStack_1f8 = 0;
            ppppppplStack_260 = (longlong *******)&UNK_142aa3050;
            pppppplStack_258 = (longlong ******)0x4c;
            uStack_250 = 0;
            uStack_220 = 1;
            func_0x000140d4a330(ppppppplStack_208,&ppppppplStack_268);
          }
          ppppppplVar24 = ppppppplStack_78;
          if ((((char)lStack_b8 == '\0') && ((*puRam000000014362c420 & 0x7fffffffffffffff) != 0)) &&
             (cVar17 = FUN_1428d86b0(), cVar17 == '\0')) {
            *(char *)((longlong)ppppppplVar24 + 1) = '\x01';
          }
          LOCK();
          cVar17 = *(char *)ppppppplVar24;
          *(char *)ppppppplVar24 = '\0';
          UNLOCK();
          if (cVar17 == '\x02') {
            FUN_1428d8810(ppppppplVar24);
          }
          goto code_r0x0001408c7baf;
        }
        ppppppplStack_78 =
             (longlong *******)
             CONCAT44(ppppppplStack_78._4_4_,*(undefined4 *)pauVar35[uVar28 * -3 + -3]);
        LOCK();
        bVar37 = (char)*plVar26 == '\0';
        if (bVar37) {
          *(char *)plVar26 = '\x01';
        }
        UNLOCK();
        if (!bVar37) {
          FUN_1428cd210(plVar26);
        }
        FUN_140c81c70(plVar26);
        uVar33 = *(longlong *)(pauVar35[uVar28 * -3 + -2] + 8) + 1U & 0x1ffffffffffffffe;
        if (0x3ff < uVar33) {
          uVar33 = 0x400;
        }
        lStack_b8 = lVar21;
        FUN_140c02740(&ppppppplStack_3f8,plStack_400,*(undefined8 *)(param_1 + 400),(int)uVar33 << 3
                      ,1,0x10);
        uStack_62 = 1;
        FUN_140c81e10(plStack_70);
        LOCK();
        bVar37 = (char)*plStack_70 == '\x01';
        if (bVar37) {
          *(char *)plStack_70 = '\0';
        }
        UNLOCK();
        if (!bVar37) {
          uStack_62 = 1;
          FUN_1428cdb90(plStack_70,0);
        }
        if (((ulonglong)ppppppplStack_3f8 & 3) == 0) {
          if (((ulonglong)ppppppplStack_3f0 & 7) == 0) {
            uVar33 = *(ulonglong *)(pauVar35[uVar28 * -3 + -2] + 8);
            uVar27 = (ulonglong)ppppppplStack_3f0 >> 3;
            if (uVar33 < (ulonglong)ppppppplStack_3f0 >> 3) {
              uVar27 = uVar33;
            }
            if (uVar27 != 0) {
              ppppppplVar24 = *(longlong ********)pauVar35[uVar28 * -3 + -2];
              if ((uVar27 < 6) ||
                 ((ppppppplStack_3f8 < (longlong *******)((longlong)ppppppplVar24 + uVar27 * 0xc) &&
                  (ppppppplVar24 < ppppppplStack_3f8 + uVar27)))) {
                uVar33 = 0;
code_r0x0001408c76cd:
                uVar34 = uVar33;
                if ((uVar27 & 1) != 0) {
                  uVar19 = *(undefined4 *)((longlong)ppppppplVar24 + uVar33 * 0xc + 8);
                  *(undefined4 *)(ppppppplStack_3f8 + uVar33) =
                       *(undefined4 *)((longlong)ppppppplVar24 + uVar33 * 0xc);
                  *(undefined4 *)((longlong)ppppppplStack_3f8 + uVar33 * 8 + 4) = uVar19;
                  uVar34 = uVar33 | 1;
                }
                if (uVar27 != (uVar33 | 1)) {
                  pcVar29 = (char *)((longlong)ppppppplVar24 + uVar34 * 0xc + 0x14);
                  do {
                    uVar19 = *(undefined4 *)(pcVar29 + -0xc);
                    *(undefined4 *)(ppppppplStack_3f8 + uVar34) = *(undefined4 *)(pcVar29 + -0x14);
                    *(undefined4 *)((longlong)ppppppplStack_3f8 + uVar34 * 8 + 4) = uVar19;
                    uVar19 = *(undefined4 *)pcVar29;
                    *(undefined4 *)(ppppppplStack_3f8 + uVar34 + 1) = *(undefined4 *)(pcVar29 + -8);
                    *(undefined4 *)((longlong)ppppppplStack_3f8 + uVar34 * 8 + 0xc) = uVar19;
                    uVar34 = uVar34 + 2;
                    pcVar29 = pcVar29 + 0x18;
                  } while (uVar27 != uVar34);
                }
              }
              else {
                uVar33 = uVar27 & 0x1ffffffffffffffc;
                pcVar29 = (char *)((longlong)ppppppplVar24 + 0x2c);
                uVar34 = 0;
                do {
                  uVar9 = *(undefined4 *)(pcVar29 + -0x20);
                  uVar10 = *(undefined4 *)(pcVar29 + -0x18);
                  uVar19 = *(undefined4 *)(pcVar29 + -0xc);
                  uVar2 = *(undefined4 *)(pcVar29 + -0x14);
                  uVar3 = *(undefined4 *)pcVar29;
                  uVar4 = *(undefined4 *)(pcVar29 + -8);
                  ppppppplStack_3f8[uVar34] =
                       (longlong ******)
                       CONCAT44(*(undefined4 *)(pcVar29 + -0x24),*(undefined4 *)(pcVar29 + -0x2c));
                  (ppppppplStack_3f8 + uVar34)[1] = (longlong ******)CONCAT44(uVar10,uVar9);
                  ppppppplStack_3f8[uVar34 + 2] = (longlong ******)CONCAT44(uVar19,uVar2);
                  (ppppppplStack_3f8 + uVar34 + 2)[1] = (longlong ******)CONCAT44(uVar3,uVar4);
                  uVar34 = uVar34 + 4;
                  pcVar29 = pcVar29 + 0x30;
                } while (uVar33 != uVar34);
                if (uVar27 != uVar33) goto code_r0x0001408c76cd;
              }
              uVar33 = *(ulonglong *)(pauVar35[uVar28 * -3 + -2] + 8);
            }
            uVar28 = 0;
            if (uVar33 < 0x400) {
              uVar28 = uVar33;
            }
            if (uVar33 == 0) {
              uStack_62 = 1;
              FUN_1428d9430(&UNK_142aa2f66,0x29,&UNK_142aa2f90);
              goto code_r0x0001408c8b97;
            }
            uStack_62 = 1;
            uVar23 = func_0x000140c02280(&ppppppplStack_3f8);
            uStack_62 = 1;
            uVar19 = FUN_140c90df0();
            uVar33 = uStack_98;
            if (uStack_98 == uStack_a8) {
              uStack_62 = 1;
              FUN_140562630(&uStack_a8,&UNK_142aa2fa8);
            }
            lVar21 = uVar33 * 0x10;
            *(undefined8 *)(lStack_a0 + lVar21) = uVar23;
            *(uint *)(lStack_a0 + 8 + lVar21) = (int)ppppppplStack_78 << 10 | (uint)uVar28 & 0x3ff;
            *(undefined4 *)(lStack_a0 + 0xc + lVar21) = uVar19;
            uStack_98 = uVar33 + 1;
            uStack_228 = uStack_3b8;
            uStack_238 = uStack_3c8;
            puStack_230 = puStack_3c0;
            uStack_248 = puStack_3d8;
            uStack_240 = uStack_3d0;
            pppppplStack_258 = pppppplStack_3e8;
            uStack_250 = uStack_3e0;
            ppppppplStack_268 = ppppppplStack_3f8;
            ppppppplStack_260 = ppppppplStack_3f0;
            uStack_62 = 0;
            func_0x000140c02290(&uStack_428);
            pppppplVar16 = pppppplStack_80;
            if (pppppplStack_80 == pppppplStack_90) {
              FUN_140562c30(&pppppplStack_90);
            }
            lVar21 = (longlong)pppppplVar16 * 0x20;
            auVar5._4_4_ = uStack_424;
            auVar5._0_4_ = uStack_428;
            auVar5._8_4_ = uStack_420;
            auVar5._12_4_ = uStack_41c;
            puVar25 = (undefined8 *)(lStack_88 + 0x10 + lVar21);
            *puVar25 = CONCAT44(uStack_414,uStack_418);
            puVar25[1] = CONCAT44(uStack_40c,uStack_410);
            *(undefined1 (*) [16])(lStack_88 + lVar21) = auVar5;
            pppppplStack_80 = (longlong ******)((longlong)pppppplVar16 + 1);
            lVar21 = lStack_b8;
            plVar26 = plStack_70;
            param_1 = lStack_430;
            goto code_r0x0001408c7549;
          }
          uVar23 = 1;
        }
        else {
          uVar23 = 0;
        }
        uStack_62 = 1;
        FUN_1428c9e00(&UNK_142a95651,0xe,uVar23);
        goto code_r0x0001408c8b97;
      }
      goto code_r0x0001408c7549;
    }
code_r0x0001408c7baf:
    LOCK();
    bVar37 = (char)*plVar26 == '\0';
    if (bVar37) {
      *(char *)plVar26 = '\x01';
    }
    UNLOCK();
    if (!bVar37) {
      FUN_1428cd210(plVar26);
    }
    FUN_140c81c70(plVar26);
    lVar21 = lStack_a0;
    ppppppplVar24 = (longlong *******)(uStack_98 << 4);
    FUN_140c02740(&ppppppplStack_268,plStack_400,*(undefined8 *)(param_1 + 400),ppppppplVar24,0);
    if (ppppppplStack_260 != ppppppplVar24) {
      FUN_1428d98e0(ppppppplStack_260,ppppppplVar24,&UNK_142a956c0);
      goto code_r0x0001408c8b97;
    }
    func_0x000142923c00(ppppppplStack_268,lVar21,ppppppplVar24);
    plVar26 = plStack_70;
    ppppppplStack_3f8 = ppppppplStack_268;
    ppppppplStack_3f0 = ppppppplStack_260;
    pppppplStack_3e8 = pppppplStack_258;
    uStack_3e0 = uStack_250;
    puStack_3d8 = (undefined *)uStack_248;
    uStack_3d0 = uStack_240;
    uStack_3c8 = uStack_238;
    puStack_3c0 = puStack_230;
    uStack_3b8 = uStack_228;
    uStack_63 = 1;
    FUN_140c81e10();
    LOCK();
    bVar37 = (char)*plVar26 == '\x01';
    if (bVar37) {
      *(char *)plVar26 = '\0';
    }
    UNLOCK();
    if (!bVar37) {
      uStack_63 = 1;
      FUN_1428cdb90(plVar26,0);
    }
    uStack_63 = 1;
    uVar23 = func_0x000140c02280(&ppppppplStack_3f8);
    uStack_228 = uStack_3b8;
    uStack_238 = uStack_3c8;
    puStack_230 = puStack_3c0;
    uStack_248 = puStack_3d8;
    uStack_240 = uStack_3d0;
    pppppplStack_258 = pppppplStack_3e8;
    uStack_250 = uStack_3e0;
    ppppppplStack_268 = ppppppplStack_3f8;
    ppppppplStack_260 = ppppppplStack_3f0;
    uStack_63 = 0;
    func_0x000140c02290(&uStack_428,&ppppppplStack_268);
    pppppplVar16 = pppppplStack_80;
    if (pppppplStack_80 == pppppplStack_90) {
      FUN_140562c30(&pppppplStack_90,&UNK_142aa30a0);
    }
    lVar21 = (longlong)pppppplVar16 * 0x20;
    puVar25 = (undefined8 *)(lStack_88 + 0x10 + lVar21);
    *puVar25 = CONCAT44(uStack_414,uStack_418);
    puVar25[1] = CONCAT44(uStack_40c,uStack_410);
    puVar25 = (undefined8 *)(lStack_88 + lVar21);
    *puVar25 = CONCAT44(uStack_424,uStack_428);
    puVar25[1] = CONCAT44(uStack_41c,uStack_420);
    pppppplStack_80 = (longlong ******)((longlong)pppppplVar16 + 1);
    if (lStack_d0 == 0) goto code_r0x0001408c7dd1;
    plVar26 = (longlong *)func_0x000140c18130();
    if (*plVar26 == 0) {
      uStack_61 = 1;
      plVar26 = (longlong *)FUN_1428f87d0(plVar26,0);
      if (plVar26 != (longlong *)0x0) goto code_r0x0001408c7d9f;
    }
    else if ((int)*plVar26 == 1) {
      plVar26 = plVar26 + 1;
code_r0x0001408c7d9f:
      if (*plVar26 == 0) {
        *plVar26 = -1;
        plStack_70 = plVar26;
        FUN_140c170f0(plVar26 + 1,uStack_c8);
        *plStack_70 = *plStack_70 + 1;
code_r0x0001408c7dd1:
        uStack_61 = 1;
        FUN_1416032a0(&ppppppplStack_268,&UNK_142aa30b8,0x18,&UNK_142aa30d0,0x31,lStack_b0,param_1);
        if (cStack_132 != '\b') {
          FUN_14096b570(auStack_158,lStack_130 + 0xe8,0x5efaa1884a48985d,uStack_98);
        }
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        if (cStack_2c2 != '\b') {
          FUN_14096b760(auStack_2e8,lStack_2c0 + 0xe8,0x497d31a6f59bd1e5,uVar23);
        }
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        if (cStack_132 != '\b') {
          FUN_14096ba30(auStack_158,lStack_130 + 0xe8,0x8a5c96e47fe18f3a,_UNK_1429258d0);
        }
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        ppppppplStack_5c8._0_1_ = 9;
        ppppppplStack_268._0_4_ = 2;
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_3f8,0xb981790202418bc8,0x32,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        ppppppplStack_5c8._0_1_ = 0xb;
        ppppppplStack_3f8._0_4_ = 2;
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_268,0xe3659c0fbde7bf03,0x32,
                      &ppppppplStack_3f8);
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        ppppppplStack_5c8._0_1_ = 0x13;
        ppppppplStack_268._0_4_ = 2;
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_3f8,0x6ad63ccd18c8f041,0x19,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        ppppppplStack_5c8._0_1_ = 0xe;
        ppppppplStack_3f8 = (longlong *******)CONCAT44(ppppppplStack_3f8._4_4_,2);
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_268,0x4ee098b38612d2b9,0x32,
                      &ppppppplStack_3f8);
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        ppppppplStack_5c8 = (longlong *******)CONCAT71(ppppppplStack_5c8._1_7_,0xd);
        ppppppplStack_268 = (longlong *******)CONCAT44(ppppppplStack_268._4_4_,2);
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_3f8,0xab24287344aba8c1,0x32,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        uStack_61 = 1;
        FUN_141607210(&ppppppplStack_3f8,&ppppppplStack_268,0xd6623460fa508d3e);
        pppppplStack_258 = pppppplStack_80;
        ppppppplStack_268 = (longlong *******)pppppplStack_90;
        ppppppplStack_260 = (longlong *******)lStack_88;
        uStack_61 = 0;
        FUN_141609e20(&ppppppplStack_3f8,&ppppppplStack_268);
        uStack_61 = 0;
        func_0x000140bd52a0(&ppppppplStack_5c8,0x20000000200,0x6d);
        pppppplStack_3e8 = (longlong ******)uStack_5b8;
        uStack_3e0 = uStack_5b0;
        ppppppplStack_3f8 = ppppppplStack_5c8;
        ppppppplStack_3f0 = (longlong *******)uStack_5c0;
        uStack_61 = 0;
        func_0x000140bd6ce0(&pppppplStack_258,&ppppppplStack_3f8);
        ppppppplStack_268 = (longlong *******)&UNK_142aa3101;
        ppppppplStack_260 = (longlong *******)0x9;
        uStack_61 = 0;
        uStack_428 = FUN_140be94d0(param_1,&ppppppplStack_268);
        uStack_424 = SUB84(ppppppplStack_5c8,0);
        uStack_420 = (undefined4)((ulonglong)ppppppplStack_5c8 >> 0x20);
        uStack_41c = (undefined4)uStack_5c0;
        uStack_418 = (undefined4)((ulonglong)uStack_5c0 >> 0x20);
        uStack_414 = (undefined4)uStack_5b8;
        uStack_410 = (undefined4)((ulonglong)uStack_5b8 >> 0x20);
        uStack_40c = (undefined4)uStack_5b0;
        uStack_408 = (undefined4)((ulonglong)uStack_5b0 >> 0x20);
        ppppppplStack_3f8 = (longlong *******)&UNK_142aa310a;
        ppppppplStack_3f0 = (longlong *******)0x7;
        pppppplStack_3e8 = (longlong ******)CONCAT71(pppppplStack_3e8._1_7_,1);
        uStack_61 = 0;
        FUN_141602990(&ppppppplStack_268,&UNK_142aa3111,0x13,&ppppppplStack_3f8,lStack_b0,param_1);
        if (cStack_132 != '\b') {
          FUN_14096b850(auStack_158,lStack_130 + 0xe8,0xd91ff8372094bc22,1);
        }
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        if (cStack_2c2 != '\b') {
          FUN_14096ba30(auStack_2e8,lStack_2c0 + 0xe8,0xdc022c4f3092a794,_UNK_14292e6f0);
        }
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        ppppppplStack_5c8 = (longlong *******)CONCAT71(ppppppplStack_5c8._1_7_,9);
        ppppppplStack_3f8 = (longlong *******)CONCAT44(ppppppplStack_3f8._4_4_,2);
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_268,0xd82d7a0adb9164c2,0x19,
                      &ppppppplStack_3f8);
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        ppppppplStack_268 = (longlong *******)CONCAT44(ppppppplStack_268._4_4_,2);
        ppppppplStack_5c8 = (longlong *******)&uStack_428;
        FUN_1408edd00(&ppppppplStack_5c8,&ppppppplStack_3f8,0x1ca51a73373208db,0x32,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        uVar15 = _UNK_142aa2508;
        uVar23 = _UNK_142aa2500;
        uStack_588 = uStack_1c8;
        uStack_580 = uStack_1c0;
        uStack_598 = uStack_1d8;
        uStack_590 = uStack_1d0;
        uStack_5a8 = uStack_1e8;
        uStack_5a0 = uStack_1e0;
        uStack_5b8 = uStack_1f8;
        uStack_5b0 = uStack_1f0;
        ppppppplStack_5c8 = ppppppplStack_208;
        uStack_5c0 = uStack_200;
        pppppplStack_3e8 = pppppplStack_258;
        ppppppplStack_3f8 = ppppppplStack_268;
        ppppppplStack_3f0 = ppppppplStack_260;
        uStack_3c8 = uStack_250;
        puStack_3c0 = (undefined *)uStack_248;
        uStack_3b8 = uStack_240;
        uStack_3e0 = uStack_238;
        puStack_3d8 = puStack_230;
        uStack_3d0 = uStack_228;
        uStack_380 = uStack_1b8;
        uStack_378 = uStack_1b0;
        uStack_370 = uStack_1a8;
        uStack_368 = uStack_1a0;
        uStack_360 = uStack_198;
        uStack_358 = uStack_190;
        uStack_350 = uStack_188;
        uStack_34c = uStack_184;
        uStack_348 = uStack_180;
        uStack_344 = uStack_17c;
        uStack_340 = uStack_178;
        uStack_338 = uStack_170;
        uStack_330 = uStack_168;
        uStack_328 = uStack_160;
        uStack_3a0 = uStack_210;
        uStack_3b0 = CONCAT44(uStack_21c,uStack_220);
        puStack_3a8 = puStack_218;
        uStack_318 = uStack_e8;
        lStack_320 = lStack_130;
        uStack_398 = _UNK_142aa2500;
        uStack_390 = _UNK_142aa2508;
        FUN_1409dd4d0(&ppppppplStack_5c8,&ppppppplStack_3f8);
        if (cStack_132 != '\b') {
          uStack_61 = 0;
          FUN_140be24b0(auStack_148);
        }
        ppppppplStack_3f8 = (longlong *******)&UNK_142aa3124;
        ppppppplStack_3f0 = (longlong *******)0x3;
        pppppplStack_3e8 = (longlong ******)((ulonglong)pppppplStack_3e8 & 0xffffffffffffff00);
        uStack_61 = 0;
        FUN_141602990(&ppppppplStack_268,&UNK_142aa3111,0x13,&ppppppplStack_3f8,lStack_b0,param_1);
        if (cStack_132 != '\b') {
          FUN_14096b850(auStack_158,lStack_130 + 0xe8,0xd91ff8372094bc22,0);
        }
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        if (cStack_2c2 != '\b') {
          FUN_14096ba30(auStack_2e8,lStack_2c0 + 0xe8,0xdc022c4f3092a794,_UNK_14292e6f0);
        }
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        ppppppplStack_3f8 = (longlong *******)CONCAT44(ppppppplStack_3f8._4_4_,2);
        ppppppplStack_5c8 = (longlong *******)&uStack_428;
        FUN_1408ee240(&ppppppplStack_5c8,&ppppppplStack_268,0xd82d7a0adb9164c2,0x19,
                      &ppppppplStack_3f8);
        func_0x000142923c00(&ppppppplStack_3f8,&ppppppplStack_268,400);
        ppppppplStack_5c8 = (longlong *******)CONCAT71(ppppppplStack_5c8._1_7_,9);
        ppppppplStack_268 = (longlong *******)CONCAT44(ppppppplStack_268._4_4_,2);
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_3f8,0x1ca51a73373208db,0x32,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        uStack_588 = uStack_1c8;
        uStack_580 = uStack_1c0;
        uStack_598 = uStack_1d8;
        uStack_590 = uStack_1d0;
        uStack_5a8 = uStack_1e8;
        uStack_5a0 = uStack_1e0;
        uStack_5b8 = uStack_1f8;
        uStack_5b0 = uStack_1f0;
        ppppppplStack_5c8 = ppppppplStack_208;
        uStack_5c0 = uStack_200;
        pppppplStack_3e8 = pppppplStack_258;
        ppppppplStack_3f8 = ppppppplStack_268;
        ppppppplStack_3f0 = ppppppplStack_260;
        uStack_3c8 = uStack_250;
        puStack_3c0 = (undefined *)uStack_248;
        uStack_3b8 = uStack_240;
        uStack_3e0 = uStack_238;
        puStack_3d8 = puStack_230;
        uStack_3d0 = uStack_228;
        uStack_380 = uStack_1b8;
        uStack_378 = uStack_1b0;
        uStack_370 = uStack_1a8;
        uStack_368 = uStack_1a0;
        uStack_360 = uStack_198;
        uStack_358 = uStack_190;
        uStack_350 = uStack_188;
        uStack_34c = uStack_184;
        uStack_348 = uStack_180;
        uStack_344 = uStack_17c;
        uStack_340 = uStack_178;
        uStack_338 = uStack_170;
        uStack_330 = uStack_168;
        uStack_328 = uStack_160;
        uStack_3a0 = uStack_210;
        uStack_3b0 = CONCAT44(uStack_21c,uStack_220);
        puStack_3a8 = puStack_218;
        uStack_318 = uStack_e8;
        lStack_320 = lStack_130;
        uStack_398 = uVar23;
        uStack_390 = uVar15;
        FUN_1409dd4d0(&ppppppplStack_5c8,&ppppppplStack_3f8);
        if (cStack_132 != '\b') {
          uStack_61 = 0;
          FUN_140be24b0(auStack_148);
        }
        uStack_61 = 0;
        FUN_1416032a0(&ppppppplStack_3f8,&UNK_142aa3127,0x12,&UNK_142aa3139,0x1b,lStack_b0,param_1);
        ppppppplStack_5c8 = (longlong *******)CONCAT71(ppppppplStack_5c8._1_7_,9);
        ppppppplStack_268._0_4_ = 2;
        FUN_140905260(&ppppppplStack_5c8,&ppppppplStack_3f8,0x61b08b5e0470c5f2,0x19,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_3f8,400);
        lStack_d0 = CONCAT71(lStack_d0._1_7_,10);
        ppppppplStack_5c8 = (longlong *******)CONCAT44(ppppppplStack_5c8._4_4_,2);
        FUN_140905260(&lStack_d0,&ppppppplStack_268,0x297d1e4cdae8f657,0x19,&ppppppplStack_5c8);
        func_0x000142923c00(&ppppppplStack_5c8,&ppppppplStack_268,400);
        ppppppplStack_3f8 = (longlong *******)CONCAT71(ppppppplStack_3f8._1_7_,0xe);
        ppppppplStack_268 = (longlong *******)CONCAT44(ppppppplStack_268._4_4_,2);
        FUN_140905260(&ppppppplStack_3f8,&ppppppplStack_5c8,0x4ee098b38612d2b9,0x32,
                      &ppppppplStack_268);
        func_0x000142923c00(&ppppppplStack_268,&ppppppplStack_5c8,400);
        uStack_588 = uStack_1c8;
        uStack_580 = uStack_1c0;
        uStack_598 = uStack_1d8;
        uStack_590 = uStack_1d0;
        uStack_5a8 = uStack_1e8;
        uStack_5a0 = uStack_1e0;
        uStack_5b8 = uStack_1f8;
        uStack_5b0 = uStack_1f0;
        ppppppplStack_5c8 = ppppppplStack_208;
        uStack_5c0 = uStack_200;
        pppppplStack_3e8 = pppppplStack_258;
        ppppppplStack_3f8 = ppppppplStack_268;
        ppppppplStack_3f0 = ppppppplStack_260;
        uStack_3c8 = uStack_250;
        puStack_3c0 = (undefined *)uStack_248;
        uStack_3b8 = uStack_240;
        uStack_3e0 = uStack_238;
        puStack_3d8 = puStack_230;
        uStack_3d0 = uStack_228;
        uStack_380 = uStack_1b8;
        uStack_378 = uStack_1b0;
        uStack_370 = uStack_1a8;
        uStack_368 = uStack_1a0;
        uStack_360 = uStack_198;
        uStack_358 = uStack_190;
        uStack_350 = uStack_188;
        uStack_34c = uStack_184;
        uStack_348 = uStack_180;
        uStack_344 = uStack_17c;
        uStack_340 = uStack_178;
        uStack_338 = uStack_170;
        uStack_330 = uStack_168;
        uStack_328 = uStack_160;
        uStack_3a0 = uStack_210;
        uStack_3b0 = CONCAT44(uStack_21c,uStack_220);
        puStack_3a8 = puStack_218;
        uStack_318 = uStack_e8;
        lStack_320 = lStack_130;
        uStack_398 = uVar23;
        uStack_390 = uVar15;
        FUN_1409dd4d0(&ppppppplStack_5c8,&ppppppplStack_3f8);
        if (cStack_132 != '\b') {
          uStack_61 = 0;
          FUN_140be24b0(auStack_148);
        }
        *(undefined1 *)(lStack_c0 + 0x40) = 1;
        if (uStack_a8 != 0) {
          func_0x000140613c20(lStack_a0,uStack_a8 << 4,8);
        }
        return;
      }
      uStack_61 = 1;
      FUN_1428d9250(&UNK_142c3ab70);
      goto code_r0x0001408c8b97;
    }
  }
  uStack_61 = 1;
  FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001408c8b97:
                    /* WARNING: Does not return */
  pcVar11 = (code *)invalidInstructionException();
  (*pcVar11)();
}

