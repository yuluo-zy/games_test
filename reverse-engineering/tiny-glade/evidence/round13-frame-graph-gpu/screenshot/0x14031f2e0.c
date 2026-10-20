
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14031f2e0(undefined8 *param_1,char param_2)

{
  byte *pbVar1;
  undefined4 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined4 **ppuVar6;
  undefined8 uVar7;
  code *pcVar8;
  undefined8 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  char cVar13;
  uint uVar14;
  longlong lVar15;
  undefined4 *puVar16;
  undefined8 *puVar17;
  longlong *plVar18;
  ulonglong uVar19;
  undefined8 uVar20;
  longlong lVar21;
  ulonglong uVar22;
  undefined *puVar23;
  undefined *puVar24;
  undefined *puVar25;
  undefined1 uVar26;
  bool bVar27;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined2 uStack_1dc;
  undefined2 uStack_1da;
  undefined2 uStack_1d8;
  undefined4 uStack_1d6;
  undefined4 uStack_1d2;
  undefined4 uStack_1ce;
  char cStack_1ca;
  undefined1 uStack_1c9;
  undefined4 *puStack_1c0;
  undefined4 *puStack_1b8;
  undefined4 *puStack_1b0;
  longlong *plStack_1a8;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined1 uStack_188;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  longlong *plStack_168;
  longlong lStack_158;
  undefined8 uStack_150;
  longlong lStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  longlong *plStack_128;
  longlong *plStack_120;
  longlong *plStack_118;
  longlong *plStack_110;
  undefined4 **ppuStack_108;
  longlong *plStack_100;
  undefined4 **ppuStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined4 *puStack_e0;
  undefined4 ***pppuStack_d8;
  undefined8 uStack_d0;
  undefined4 **ppuStack_c8;
  undefined8 uStack_c0;
  undefined4 **ppuStack_a8;
  longlong *plStack_a0;
  undefined4 **ppuStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined1 uStack_88;
  undefined7 uStack_87;
  undefined4 *puStack_80;
  undefined4 *puStack_78;
  longlong *plStack_70;
  longlong *plStack_68;
  undefined4 *puStack_60;
  longlong *plStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cVar13 = func_0x000140c16a00();
  if (cVar13 == '\0') {
LAB_14031f373:
    lStack_148 = 0;
LAB_14031f543:
    cVar13 = *(char *)((longlong)param_1 + 0x96);
    uVar26 = *(undefined1 *)((longlong)param_1 + 0x97);
    *(undefined1 *)((longlong)param_1 + 0x96) = 6;
    if (cVar13 == '\x06') {
      FUN_1428d9310(&UNK_1429bc9f8);
      goto LAB_14032006f;
    }
    uVar9 = param_1[0xc];
    uStack_1f0 = *(undefined4 *)(param_1 + 0xe);
    uStack_1ec = *(undefined4 *)((longlong)param_1 + 0x74);
    uStack_1e8 = *(undefined4 *)(param_1 + 0xf);
    uStack_1e4 = *(undefined4 *)((longlong)param_1 + 0x7c);
    uStack_1e0 = *(undefined4 *)(param_1 + 0x10);
    uStack_1d6 = *(undefined4 *)((longlong)param_1 + 0x8a);
    uStack_1d2 = *(undefined4 *)((longlong)param_1 + 0x8e);
    uStack_1ce = *(undefined4 *)((longlong)param_1 + 0x92);
    uStack_1d8 = (undefined2)((uint)*(undefined4 *)((longlong)param_1 + 0x86) >> 0x10);
    uStack_1dc = (undefined2)*(undefined4 *)((longlong)param_1 + 0x84);
    uStack_1da = (undefined2)((uint)*(undefined4 *)((longlong)param_1 + 0x84) >> 0x10);
    cStack_1ca = cVar13;
    uStack_1c9 = uVar26;
    if (cVar13 == '\x05') {
      plStack_128 = (longlong *)param_1[2];
      uStack_138 = (undefined4 *)*param_1;
      uStack_130 = (undefined4 *)param_1[1];
      *param_1 = 0;
      if (uStack_138 == (undefined4 *)0x1) {
        ppuStack_a8 = (undefined4 **)0x0;
        plStack_a0 = plStack_128;
        if (uStack_130 == (undefined4 *)0x0) {
          FUN_1428d9310(&UNK_1429bc780);
          goto LAB_14032006f;
        }
        puStack_1b8 = (undefined4 *)0x0;
        puStack_1b0 = (undefined4 *)param_1[0xb];
        puStack_60 = uStack_130;
        plStack_58 = plStack_128;
        (*(code *)plStack_128[3])(uStack_130,&puStack_1b8);
        if (plStack_58[1] != 0) {
          func_0x000140613c20(puStack_60,plStack_58[1],plStack_58[2]);
        }
        FUN_1405fdd60(&ppuStack_a8);
        puVar2 = puStack_80;
      }
      else {
        puVar2 = puStack_80;
        if (uStack_138 != (undefined4 *)0x0) {
          FUN_1405fdd60(&uStack_130);
          puVar2 = puStack_80;
        }
      }
LAB_14031fead:
      puStack_80 = puVar2;
      if (cStack_1ca != '\x05') {
        FUN_140be24b0(&uStack_1f0);
      }
      if (lStack_148 == 0) {
        return;
      }
      plVar18 = (longlong *)func_0x000140c18130();
      if (*plVar18 == 1) {
        plVar18 = plVar18 + 1;
      }
      else if ((*plVar18 != 0) ||
              (plVar18 = (longlong *)FUN_1428a1640(plVar18,0), plVar18 == (longlong *)0x0))
      goto LAB_14031ff31;
      if (*plVar18 == 0) {
        *plVar18 = -1;
        plStack_58 = plVar18;
        FUN_140c170f0(plVar18 + 1,uStack_140);
        *plStack_58 = *plStack_58 + 1;
        return;
      }
      goto LAB_14031ff3d;
    }
    puStack_60 = (undefined4 *)param_1[0xb];
    plStack_58 = (longlong *)param_1[0xd];
    cVar13 = func_0x000140c16a00();
    if (cVar13 == '\0') {
      bVar27 = false;
      lStack_158 = 0;
      if (param_2 != '\0') goto LAB_14031f705;
LAB_14031f978:
      lVar15 = 0;
      if ((longlong)param_1[3] < -0x7ffffffffffffffe) {
        lVar15 = param_1[3] + -0x7fffffffffffffff;
      }
      if (lVar15 == 0) {
        ppuVar6 = (undefined4 **)param_1[5];
        if ((longlong)ppuVar6 < 0) {
          uVar20 = 0;
LAB_14032005d:
          FUN_1428d8fe3(uVar20,ppuVar6,&UNK_1429ae438);
          goto LAB_14032006f;
        }
        uVar7 = param_1[4];
        if (ppuVar6 == (undefined4 **)0x0) {
          plVar18 = (longlong *)0x1;
        }
        else {
          uVar20 = 1;
          plVar18 = (longlong *)func_0x000140613c10(ppuVar6,1);
          if (plVar18 == (longlong *)0x0) goto LAB_14032005d;
        }
        func_0x000142923c00(plVar18,uVar7,ppuVar6);
        uStack_88 = *(undefined1 *)(param_1 + 7);
        uStack_90 = CONCAT31(uStack_90._1_3_,*(undefined1 *)(param_1 + 6));
        ppuStack_a8 = ppuVar6;
        plStack_a0 = plVar18;
        ppuStack_98 = ppuVar6;
      }
      else if (lVar15 == 1) {
        plVar18 = (longlong *)param_1[4];
        LOCK();
        lVar15 = *plVar18;
        *plVar18 = *plVar18 + 1;
        UNLOCK();
        if (*plVar18 == 0 || SCARRY8(lVar15,1) != *plVar18 < 0) goto LAB_14032006f;
        ppuStack_98 = (undefined4 **)CONCAT71(ppuStack_98._1_7_,*(undefined1 *)(param_1 + 5));
        ppuStack_a8 = (undefined4 **)0x8000000000000000;
        plStack_a0 = plVar18;
      }
      else {
        plVar18 = (longlong *)param_1[4];
        LOCK();
        lVar15 = *plVar18;
        *plVar18 = *plVar18 + 1;
        UNLOCK();
        if (*plVar18 == 0 || SCARRY8(lVar15,1) != *plVar18 < 0) goto LAB_14032006f;
        ppuStack_a8 = (undefined4 **)0x8000000000000001;
        plStack_a0 = plVar18;
      }
      if ((longlong)plStack_58 < 0) {
        uVar20 = 0;
LAB_14031ffe9:
        uStack_4b = 1;
        FUN_1428d8fe3(uVar20,plStack_58,&UNK_1429bc558);
        goto LAB_14032006f;
      }
      if (plStack_58 == (longlong *)0x0) {
        plVar18 = (longlong *)0x1;
      }
      else {
        uVar20 = 1;
        plVar18 = (longlong *)func_0x000140613c10(plStack_58,1);
        if (plVar18 == (longlong *)0x0) goto LAB_14031ffe9;
      }
      plStack_68 = plVar18;
      func_0x000142923c00(plVar18,uVar9,plStack_58);
      plStack_168 = (longlong *)param_1[2];
      uStack_178 = *(undefined4 *)param_1;
      uStack_174 = *(undefined4 *)((longlong)param_1 + 4);
      uStack_170 = *(undefined4 *)(param_1 + 1);
      uStack_16c = *(undefined4 *)((longlong)param_1 + 0xc);
      *param_1 = 0;
      plStack_128 = (longlong *)0x8000000000000000;
      uStack_138 = (undefined4 *)0x0;
      plStack_110 = (longlong *)((ulonglong)plStack_110 & 0xffffffffffffff00);
      puStack_1c0 = (undefined4 *)func_0x000140613c10(0x1b,1);
      uVar9 = _UNK_1429bca81;
      if (puStack_1c0 == (undefined4 *)0x0) {
        FUN_1428d8fe3(1,0x1b,&UNK_1429bc558);
        goto LAB_14032006f;
      }
      *(ulonglong *)((longlong)puStack_1c0 + 0xb) =
           CONCAT35(_UNK_1429bca7e,CONCAT41(_UNK_1429bca7a,DAT_1429bca79));
      *(undefined8 *)((longlong)puStack_1c0 + 0x13) = uVar9;
      uVar12 = _UNK_1429bca7a;
      uVar11 = _UNK_1429bca76;
      uVar10 = _UNK_1429bca72;
      *puStack_1c0 = _DAT_1429bca6e;
      puStack_1c0[1] = uVar10;
      puStack_1c0[2] = uVar11;
      puStack_1c0[3] = uVar12;
      puStack_80 = (undefined4 *)0x1b;
      plStack_70 = (longlong *)0x1b;
      puStack_78 = puStack_1c0;
      FUN_140d558f0(&puStack_1b8,&uStack_138,&puStack_80);
      puStack_e8 = (undefined *)CONCAT71(uStack_87,uStack_88);
      ppuStack_f8 = ppuStack_98;
      uStack_f0 = uStack_90;
      uStack_ec = uStack_8c;
      ppuStack_108 = ppuStack_a8;
      plStack_100 = plStack_a0;
      puStack_e0 = puStack_60;
      plStack_120 = plStack_58;
      plStack_118 = plStack_68;
      plStack_110 = plStack_58;
      uStack_138 = (undefined4 *)CONCAT44(uStack_174,uStack_178);
      uStack_130 = (undefined4 *)CONCAT44(uStack_16c,uStack_170);
      plStack_128 = plStack_168;
      uStack_49 = 0;
      FUN_140289e80(&puStack_80,&puStack_1b8,&uStack_138);
      if (puStack_80 == (undefined4 *)0x0) {
        uStack_138 = puStack_78;
        FUN_1428d9760(&UNK_1429bca89,0x19,&uStack_138,&UNK_1429bc4e8,&UNK_1429bcaa8);
        goto LAB_14032006f;
      }
      plStack_1a8 = plStack_70;
      puStack_1b8 = puStack_80;
      puStack_1b0 = puStack_78;
      if (_DAT_14362b070 != 2) {
        uStack_4a = 1;
        FUN_1428a45f0(&DAT_14362b070,&DAT_14362b070);
      }
      LOCK();
      bVar27 = DAT_14362b080 == '\0';
      if (bVar27) {
        DAT_14362b080 = '\x01';
      }
      UNLOCK();
      if (!bVar27) {
        uStack_4a = 1;
        FUN_1428cd210(&DAT_14362b080);
      }
      uStack_4a = 1;
      FUN_140c81c70(&DAT_14362b080);
      uStack_4c = 1;
      FUN_1402b5ea0(&DAT_14362b088);
      lVar15 = _DAT_14362b098;
      plStack_128 = plStack_1a8;
      uStack_138 = puStack_1b8;
      uStack_130 = puStack_1b0;
      if (_DAT_14362b098 == _DAT_14362b088) {
        FUN_140562db0(&DAT_14362b088,&UNK_1429bcac0);
      }
      lVar21 = _DAT_14362b090;
      *(longlong **)(_DAT_14362b090 + 0x10 + lVar15 * 0x18) = plStack_128;
      puVar2 = (undefined4 *)(lVar21 + lVar15 * 0x18);
      *puVar2 = (undefined4)uStack_138;
      puVar2[1] = uStack_138._4_4_;
      puVar2[2] = (undefined4)uStack_130;
      puVar2[3] = uStack_130._4_4_;
      _DAT_14362b098 = lVar15 + 1;
      uStack_4a = 0;
      FUN_140c81e10();
      LOCK();
      bVar27 = DAT_14362b080 == '\x01';
      if (bVar27) {
        DAT_14362b080 = '\0';
      }
      UNLOCK();
      if (!bVar27) {
        uStack_4a = 0;
        FUN_1428cdb90(&DAT_14362b080,0);
      }
      puVar16 = (undefined4 *)0x0;
      puVar2 = puStack_80;
      if (lStack_158 != 0) goto LAB_14031fd0a;
joined_r0x00014031f759:
      puStack_80 = puVar16;
      if (puStack_80 != (undefined4 *)0x0) {
        if (*_DAT_14362c3a8 != 0) {
          ppuStack_a8 = &puStack_80;
          plStack_a0 = (longlong *)&UNK_140d4cc10;
          puStack_1b8 = (undefined4 *)0x2;
          plStack_1a8 = (longlong *)0x2;
          uStack_198 = 0;
          uStack_190 = 0x400000020;
          uStack_188 = 3;
          puVar17 = (undefined8 *)func_0x00014067dd80(&UNK_1429bca40);
          plStack_118 = (longlong *)*puVar17;
          plStack_110 = (longlong *)puVar17[1];
          uStack_ec = *(undefined4 *)(puVar17 + 2);
          ppuStack_108 = (undefined4 **)0x1;
          plStack_100 = (longlong *)&UNK_1429bca58;
          ppuStack_f8 = (undefined4 **)0x16;
          puStack_e8 = &UNK_1429bca30;
          puStack_e0 = (undefined4 *)0x1;
          pppuStack_d8 = &ppuStack_a8;
          uStack_d0 = 1;
          ppuStack_c8 = &puStack_1b8;
          uStack_c0 = 1;
          uStack_138 = (undefined4 *)0x0;
          uStack_130 = (undefined4 *)&UNK_1429bca58;
          plStack_128 = (longlong *)0x16;
          plStack_120 = (longlong *)0x0;
          uStack_f0 = 1;
          func_0x000140d4a330(&uStack_178,&uStack_138);
        }
        func_0x000140d4cc30(&puStack_80);
        puVar2 = puStack_80;
      }
      goto LAB_14031fead;
    }
    puVar25 = &UNK_1429bc5e8;
    lVar21 = 0x23;
    uVar19 = 0x23;
    lVar15 = FUN_14031aa90(&UNK_1429bc5e8);
    if (lVar15 == 1) {
      if (uVar19 != 0) {
        if (uVar19 < 0x23) {
          if ((char)(&UNK_1429bc5e8)[uVar19] < -0x40) {
LAB_14031f6b8:
            FUN_1428d9940(&UNK_1429bc5e8,0x23,0,uVar19,&UNK_1429bc6f8);
            goto LAB_14032006f;
          }
        }
        else if (uVar19 != 0x23) goto LAB_14031f6b8;
      }
      puVar25 = &UNK_1429bc5e8;
      lVar15 = FUN_14031aa90(&UNK_1429bc5e8);
      if (lVar15 == 1) {
        uVar22 = uVar19 + 2;
        if (uVar22 != 0) {
          if (uVar22 < 0x23) {
            if ((char)(&UNK_1429bc5ea)[uVar19] < -0x40) {
LAB_14031f79f:
              FUN_1428d9940(&UNK_1429bc5e8,0x23,uVar22,0x23,&UNK_1429bc710);
              goto LAB_14032006f;
            }
          }
          else if (uVar22 != 0x23) goto LAB_14031f79f;
        }
        lVar21 = 0x21 - uVar19;
        puVar25 = &UNK_1429bc5ea + uVar19;
      }
    }
    puVar23 = &UNK_1429bc763;
    puVar24 = &UNK_1429bc740;
    do {
      bVar3 = puVar23[-1];
      uVar14 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar23[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar23[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar23 + -4;
            puVar23 = puVar23 + -4;
            uVar14 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar23 = puVar23 + -3;
            uVar14 = bVar5 & 0xf;
          }
          uVar14 = bVar4 & 0x3f | uVar14 << 6;
        }
        else {
          puVar23 = puVar23 + -2;
          uVar14 = bVar4 & 0x1f;
        }
        uVar14 = (uint)(bVar3 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_14031f884;
LAB_14031f895:
        puVar24 = puVar23 + -0x1429bc73f;
        if (puVar24 != (undefined *)0x0) {
          if (puVar24 < (undefined *)0x23) {
            if ((char)puVar23[1] < -0x40) {
LAB_14031f8b1:
              FUN_1428d9940(&UNK_1429bc740,0x23,puVar24,0x23,&UNK_1429bc6d8);
              goto LAB_14032006f;
            }
          }
          else if (puVar24 != (undefined *)0x23) goto LAB_14031f8b1;
        }
        lVar15 = 0x23 - (longlong)puVar24;
        puVar24 = puVar23 + 1;
        goto LAB_14031f8e6;
      }
      puVar23 = puVar23 + -1;
      if (uVar14 == 0x5c) goto LAB_14031f895;
LAB_14031f884:
      if (uVar14 == 0x2f) goto LAB_14031f895;
    } while (puVar23 != &UNK_1429bc740);
    lVar15 = 0x23;
LAB_14031f8e6:
    plVar18 = (longlong *)func_0x000140c18130();
    if (*plVar18 == 0) {
      plVar18 = (longlong *)FUN_1428a1640(plVar18,0);
      if (plVar18 == (longlong *)0x0) goto LAB_14031ffd6;
    }
    else {
      if ((int)*plVar18 != 1) {
LAB_14031ffd6:
        FUN_1428d80d0(&UNK_142932a10);
        goto LAB_14032006f;
      }
      plVar18 = plVar18 + 1;
    }
    if (*plVar18 != 0) {
      FUN_1428d9250(&UNK_142932ab8);
      goto LAB_14032006f;
    }
    *plVar18 = -1;
    plStack_68 = plVar18;
    uStack_150 = FUN_140c16d40(plVar18 + 1,puVar25,lVar21,puVar24,lVar15,1,0);
    *plStack_68 = *plStack_68 + 1;
    bVar27 = true;
    lStack_158 = 1;
    if (param_2 == '\0') goto LAB_14031f978;
LAB_14031f705:
    plStack_128 = (longlong *)param_1[2];
    uStack_138 = (undefined4 *)*param_1;
    uStack_130 = (undefined4 *)param_1[1];
    *param_1 = 0;
    puVar16 = (undefined4 *)FUN_1403206a0(param_1 + 3,puStack_60,uVar9,plStack_58,&uStack_138);
    puVar2 = puStack_80;
    if (!bVar27) goto joined_r0x00014031f759;
LAB_14031fd0a:
    plVar18 = (longlong *)func_0x000140c18130();
    if (*plVar18 == 0) {
      plVar18 = (longlong *)FUN_1428a1640(plVar18,0);
      if (plVar18 == (longlong *)0x0) goto LAB_14031ffb4;
    }
    else {
      if ((int)*plVar18 != 1) {
LAB_14031ffb4:
        FUN_1428d80d0(&UNK_142932a10);
        goto LAB_14032006f;
      }
      plVar18 = plVar18 + 1;
    }
    if (*plVar18 == 0) {
      *plVar18 = -1;
      plStack_58 = plVar18;
      FUN_140c170f0(plVar18 + 1,uStack_150);
      *plStack_58 = *plStack_58 + 1;
      puVar2 = puStack_80;
      goto joined_r0x00014031f759;
    }
  }
  else {
    puVar25 = &UNK_1429bc642;
    lVar21 = 0x2c;
    uVar19 = 0x2c;
    lVar15 = FUN_14031aa90(&UNK_1429bc642);
    if (lVar15 == 1) {
      if (uVar19 != 0) {
        if (uVar19 < 0x2c) {
          if ((char)(&UNK_1429bc642)[uVar19] < -0x40) {
LAB_14031f353:
            FUN_1428d9940(&UNK_1429bc642,0x2c,0,uVar19,&UNK_1429bc6f8);
            goto LAB_14031f373;
          }
        }
        else if (uVar19 != 0x2c) goto LAB_14031f353;
      }
      puVar25 = &UNK_1429bc642;
      lVar15 = FUN_14031aa90(&UNK_1429bc642);
      if (lVar15 == 1) {
        uVar22 = uVar19 + 2;
        if (uVar22 != 0) {
          uVar26 = uVar22 == 0x2c;
          if (0x2b < uVar22) goto LAB_14031f3d4;
          uVar26 = (&UNK_1429bc644)[uVar19] == -0x41;
          if ((char)(&UNK_1429bc644)[uVar19] < -0x40) {
            do {
              uVar19 = 0x2c;
              FUN_1428d9940(&UNK_1429bc642);
LAB_14031f3d4:
            } while (!(bool)uVar26);
          }
        }
        lVar21 = 0x2a - uVar19;
        puVar25 = &UNK_1429bc642 + uVar22;
      }
    }
    puVar23 = &UNK_1429bc763;
    puVar24 = &UNK_1429bc740;
    do {
      bVar3 = puVar23[-1];
      uVar14 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar23[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar23[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar23 + -4;
            puVar23 = puVar23 + -4;
            uVar14 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar23 = puVar23 + -3;
            uVar14 = bVar5 & 0xf;
          }
          uVar14 = bVar4 & 0x3f | uVar14 << 6;
        }
        else {
          puVar23 = puVar23 + -2;
          uVar14 = bVar4 & 0x1f;
        }
        uVar14 = (uint)(bVar3 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_14031f474;
LAB_14031f486:
        puVar24 = puVar23 + -0x1429bc73f;
        if (puVar24 != (undefined *)0x0) {
          uVar26 = puVar24 == (undefined *)0x23;
          if ((undefined *)0x22 < puVar24) goto LAB_14031f4c5;
          uVar26 = puVar23[1] == -0x41;
          if ((char)puVar23[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_1429bc740,0x23);
LAB_14031f4c5:
            } while (!(bool)uVar26);
          }
        }
        lVar15 = 0x23 - (longlong)puVar24;
        puVar24 = &UNK_1429bc740 + (longlong)puVar24;
        goto LAB_14031f4d3;
      }
      puVar23 = puVar23 + -1;
      if (uVar14 == 0x5c) goto LAB_14031f486;
LAB_14031f474:
      if (uVar14 == 0x2f) goto LAB_14031f486;
    } while (puVar23 != &UNK_1429bc740);
    lVar15 = 0x23;
LAB_14031f4d3:
    plVar18 = (longlong *)func_0x000140c18130();
    if (*plVar18 == 1) {
      plVar18 = plVar18 + 1;
LAB_14031f4ec:
      if (*plVar18 == 0) {
        *plVar18 = -1;
        plStack_58 = plVar18;
        uStack_140 = FUN_140c16d40(plVar18 + 1,puVar25,lVar21,puVar24,lVar15,1,0);
        *plStack_58 = *plStack_58 + 1;
        lStack_148 = 1;
        goto LAB_14031f543;
      }
    }
    else {
      if ((*plVar18 == 0) &&
         (plVar18 = (longlong *)FUN_1428a1640(plVar18,0), plVar18 != (longlong *)0x0))
      goto LAB_14031f4ec;
LAB_14031ff31:
      FUN_1428d80d0(&UNK_142932a10);
    }
LAB_14031ff3d:
    FUN_1428d9250(&UNK_142932ab8);
  }
  FUN_1428d9250(&UNK_142932ab8);
LAB_14032006f:
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

