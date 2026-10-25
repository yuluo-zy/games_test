
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char FUN_14031af10(undefined **param_1,undefined8 *param_2,undefined8 *param_3,undefined8 param_4,
                  undefined8 *param_5,undefined8 *param_6,longlong *param_7,char *param_8,
                  longlong *param_9,undefined8 *param_10,longlong *param_11,undefined8 *param_12,
                  longlong *param_13,undefined8 *param_14,undefined8 *param_15)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  longlong lVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  undefined *puVar8;
  undefined4 *puVar9;
  code *pcVar10;
  ulonglong uVar11;
  undefined **ppuVar12;
  bool bVar13;
  char cVar14;
  byte bVar15;
  byte bVar16;
  undefined **ppuVar17;
  undefined *puVar18;
  undefined8 uVar19;
  longlong *plVar20;
  undefined **ppuVar21;
  undefined8 *puVar22;
  int iVar23;
  longlong lVar24;
  undefined1 uVar25;
  longlong lVar26;
  ulonglong uVar27;
  int *piVar28;
  undefined8 uVar29;
  int iVar30;
  ulonglong uVar31;
  longlong lVar32;
  longlong lVar33;
  longlong lVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  longlong lVar37;
  ulonglong uVar38;
  bool bVar39;
  bool bVar40;
  bool bVar41;
  bool bVar42;
  bool bVar43;
  float fVar44;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined **ppuStack_1c0;
  longlong *plStack_1b8;
  undefined8 uStack_1b0;
  undefined *puStack_1a8;
  undefined8 uStack_1a0;
  undefined *puStack_198;
  undefined8 uStack_190;
  undefined8 *puStack_180;
  longlong lStack_178;
  undefined **ppuStack_170;
  undefined8 uStack_168;
  undefined **ppuStack_150;
  longlong *plStack_148;
  undefined8 uStack_140;
  undefined *puStack_138;
  undefined **ppuStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined4 uStack_108;
  undefined2 uStack_104;
  undefined2 uStack_102;
  undefined2 uStack_100;
  undefined2 uStack_fe;
  undefined2 uStack_fc;
  undefined2 uStack_fa;
  undefined1 uStack_f8;
  undefined1 uStack_f7;
  undefined2 uStack_f6;
  undefined2 uStack_f4;
  undefined2 uStack_f2;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  int iStack_dc;
  int iStack_d8;
  char cStack_d4;
  undefined1 uStack_d3;
  undefined1 uStack_d2;
  undefined1 uStack_d1;
  undefined4 uStack_d0;
  undefined1 uStack_8a;
  undefined **ppuStack_88;
  undefined8 *puStack_80;
  undefined **ppuStack_78;
  undefined **ppuStack_70;
  undefined **ppuStack_68;
  undefined1 uStack_5a;
  undefined **ppuStack_58;
  undefined1 uStack_4f;
  undefined1 uStack_4c;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  ppuStack_150 = (undefined **)param_14[5];
  lVar5 = param_14[6];
  puVar6 = (ulonglong *)param_14[10];
  lVar37 = param_14[0xb];
  ppuStack_78 = (undefined **)0x0;
  ppuStack_70 = (undefined **)0x8;
  ppuStack_68 = (undefined **)0x0;
  ppuVar21 = (undefined **)*param_5;
  puStack_180 = param_3;
  ppuStack_88 = param_1;
  puStack_80 = param_2;
  if (ppuVar21 != (undefined **)0x0) {
    puVar18 = *ppuVar21;
    ppuStack_58 = (undefined **)(puVar18 + 0x10);
    LOCK();
    bVar40 = *(longlong *)(puVar18 + 0x10) == 0;
    if (bVar40) {
      *(longlong *)(puVar18 + 0x10) = 8;
    }
    UNLOCK();
    if (!bVar40) {
      uStack_49 = 1;
      FUN_1428cb5f0(ppuStack_58,8,1000000000);
    }
    uStack_49 = 1;
    FUN_140c81c70(ppuStack_58);
    uVar38 = (ulonglong)ppuStack_58 | 1;
    uStack_49 = 1;
    FUN_140c81c70(uVar38);
    lVar33 = *(longlong *)(puVar18 + 0x28);
    if (lVar33 != 0) {
      lVar34 = *(longlong *)(puVar18 + 0x20);
      lVar32 = lVar33 * 8;
      lVar26 = 0;
      do {
        if (*(longlong *)(lVar34 + lVar26 * 8) == 0) {
          uStack_120 = *(undefined ***)(lVar34 + lVar26 * 8);
          *(undefined8 *)(lVar34 + lVar26 * 8) = *(undefined8 *)(lVar34 + -8 + lVar33 * 8);
          *(longlong *)(puVar18 + 0x28) = lVar33 + -1;
          if (uStack_120 != (undefined **)0x0) {
            LOCK();
            *uStack_120 = *uStack_120 + -1;
            UNLOCK();
            if (*uStack_120 == (undefined *)0x0) {
              FUN_140b5c0a0(&uStack_120);
            }
          }
          uStack_49 = 1;
          FUN_140c81e10(ppuStack_58);
          uStack_49 = 1;
          FUN_140c81e10(uVar38);
          LOCK();
          puVar18 = *ppuStack_58;
          if (puVar18 == (undefined *)0x8) {
            *ppuStack_58 = (undefined *)0x0;
          }
          UNLOCK();
          bVar15 = 1;
          if (puVar18 != (undefined *)0x8) {
            uStack_49 = 1;
            FUN_1428cbf60(ppuStack_58,0);
          }
          goto LAB_14031b1cf;
        }
        lVar26 = lVar26 + 1;
        lVar32 = lVar32 + -8;
      } while (lVar32 != 0);
    }
    uStack_49 = 1;
    FUN_140c81e10(ppuStack_58);
    uStack_49 = 1;
    FUN_140c81e10(uVar38);
    LOCK();
    puVar18 = *ppuStack_58;
    if (puVar18 == (undefined *)0x8) {
      *ppuStack_58 = (undefined *)0x0;
    }
    UNLOCK();
    if (puVar18 != (undefined *)0x8) {
      uStack_49 = 1;
      FUN_1428cbf60(ppuStack_58,0);
    }
  }
  uVar38 = *puVar6;
  uVar35 = 0;
  uVar31 = uVar38 - *(ulonglong *)(lVar37 + 0x18);
  if (uVar38 < *(ulonglong *)(lVar37 + 0x18)) {
    uVar31 = uVar35;
  }
  uVar36 = uVar38 - *(ulonglong *)(lVar37 + 0x38);
  if (uVar38 < *(ulonglong *)(lVar37 + 0x38)) {
    uVar36 = uVar35;
  }
  lVar33 = uVar31 * 0x10 + *(longlong *)(lVar37 + 8);
  uVar38 = *(ulonglong *)(lVar37 + 0x10);
  lVar32 = uVar36 * 0x10 + *(longlong *)(lVar37 + 0x28);
  uVar27 = uVar38 - uVar31;
  if (uVar38 < uVar31) {
    uVar27 = uVar35;
  }
  uVar7 = *(ulonglong *)(lVar37 + 0x30);
  if (uVar38 < uVar31) {
    lVar33 = 8;
    uVar31 = uVar38;
  }
  uVar11 = uVar7 - uVar36;
  if (uVar7 < uVar36) {
    lVar32 = 8;
    uVar11 = uVar35;
  }
  lVar37 = *(longlong *)(lVar37 + 0x40);
  if (uVar7 < uVar36) {
    uVar36 = uVar7;
  }
  *puVar6 = (lVar37 - uVar11) - uVar27;
  uVar38 = ((uVar36 + lVar37 + uVar31) - uVar7) - uVar38;
  lVar37 = lVar32;
  lVar34 = lVar33;
  do {
    uVar38 = uVar38 + 1;
    ppuStack_58 = ppuVar21;
    if ((lVar34 == 0) || (lVar34 == uVar27 * 0x10 + lVar33)) {
      if ((lVar37 == 0) || (lVar37 == uVar11 * 0x10 + lVar32)) {
        uStack_49 = 1;
        bVar15 = func_0x00014093fa90(*(undefined4 *)(*ppuStack_88 + 0xe8),
                                     *(undefined4 *)(*ppuStack_88 + 0xec));
        goto LAB_14031b1cf;
      }
      lVar26 = 0;
      lVar24 = lVar37 + 0x10;
      lVar34 = lVar37;
    }
    else {
      lVar26 = lVar34 + 0x10;
      lVar24 = lVar37;
    }
    *puVar6 = uVar38;
    pcVar1 = (char *)(lVar34 + 8);
    lVar37 = lVar24;
    lVar34 = lVar26;
  } while (*pcVar1 != '\x04');
  bVar15 = 1;
LAB_14031b1cf:
  if (ppuVar21 != (undefined **)0x0) {
    puVar18 = *ppuVar21;
    ppuVar17 = (undefined **)(puVar18 + 0x10);
    LOCK();
    bVar40 = *(longlong *)(puVar18 + 0x10) == 0;
    if (bVar40) {
      *(longlong *)(puVar18 + 0x10) = 8;
    }
    UNLOCK();
    if (!bVar40) {
      uStack_49 = 1;
      FUN_1428cb5f0(ppuVar17);
    }
    uStack_49 = 1;
    FUN_140c81c70(ppuVar17);
    uStack_49 = 1;
    FUN_140c81c70((ulonglong)ppuVar17 | 1);
    ppuStack_58 = ppuVar17;
    FUN_1402b54f0(puVar18 + 0x18,param_4);
    ppuVar17 = ppuStack_58;
    uStack_49 = 1;
    FUN_140c81e10(ppuStack_58);
    uStack_49 = 1;
    FUN_140c81e10();
    LOCK();
    puVar18 = *ppuVar17;
    if (puVar18 == (undefined *)0x8) {
      *ppuVar17 = (undefined *)0x0;
    }
    UNLOCK();
    if (puVar18 != (undefined *)0x8) {
      uStack_49 = 1;
      FUN_1428cbf60(ppuVar17,0);
    }
  }
  piVar28 = (int *)*param_6;
  if (*param_8 == '\0') {
    if ((bVar15 & *piVar28 == 0x13) == 0) {
LAB_14031b30a:
      if (bVar15 != 0) goto LAB_14031b313;
    }
    else {
      *(undefined4 *)param_13[2] = *(undefined4 *)((longlong)param_13 + 0x1c);
      *(undefined1 *)(*param_13 + 0x31) = 1;
      param_8[0] = '\x01';
      param_8[1] = '\x01';
    }
LAB_14031b582:
    puVar18 = *ppuStack_88;
    uStack_49 = 1;
    cVar14 = func_0x00014093fa90(*(undefined4 *)(puVar18 + 0xf0),*(undefined4 *)(puVar18 + 0xf4));
    if (cVar14 != '\0') {
      uStack_49 = 1;
      FUN_140d61b00(&uStack_140,&UNK_1429bc800,0x15);
      ppuVar21 = ppuStack_68;
      uStack_f8 = SUB81(ppuStack_130,0);
      uStack_f7 = (undefined1)((ulonglong)ppuStack_130 >> 8);
      uStack_f6 = (undefined2)((ulonglong)ppuStack_130 >> 0x10);
      uStack_f4 = (undefined2)((ulonglong)ppuStack_130 >> 0x20);
      uStack_f2 = (undefined2)((ulonglong)ppuStack_130 >> 0x30);
      uStack_f0 = (undefined4)uStack_128;
      uStack_ec = uStack_128._4_4_;
      uStack_108 = (undefined4)uStack_140;
      uStack_104 = (undefined2)((ulonglong)uStack_140 >> 0x20);
      uStack_102 = (undefined2)((ulonglong)uStack_140 >> 0x30);
      uStack_100 = SUB82(puStack_138,0);
      uStack_fe = (undefined2)((ulonglong)puStack_138 >> 0x10);
      uStack_fc = (undefined2)((ulonglong)puStack_138 >> 0x20);
      uStack_fa = (undefined2)((ulonglong)puStack_138 >> 0x30);
      cStack_d4 = '\0';
      uStack_e0 = 0;
      uStack_e8 = CONCAT31(uStack_e8._1_3_,100);
      uStack_120 = (undefined **)0x0;
      if (ppuStack_68 == ppuStack_78) {
        FUN_140563540(&ppuStack_78,&UNK_1429bc818);
      }
      ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
      (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
           (undefined *)
           CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))))
      ;
      ppuStack_70[(longlong)ppuVar21 * 10 + 6] = (undefined *)CONCAT44(uStack_ec,uStack_f0);
      (ppuStack_70 + (longlong)ppuVar21 * 10 + 6)[1] = (undefined *)CONCAT44(uStack_e4,uStack_e8);
      ppuStack_70[(longlong)ppuVar21 * 10 + 4] =
           (undefined *)CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
      (ppuStack_70 + (longlong)ppuVar21 * 10 + 4)[1] =
           (undefined *)
           CONCAT26(uStack_f2,CONCAT24(uStack_f4,CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8))))
      ;
      ppuStack_70[(longlong)ppuVar21 * 10 + 2] = (undefined *)uStack_110;
      (ppuStack_70 + (longlong)ppuVar21 * 10 + 2)[1] =
           (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
      ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
      *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
      *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
      *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
      *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
      ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
    }
  }
  else {
    bVar16 = param_8[1];
    if (bVar16 < 0x10) {
      *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
      *(undefined1 *)(*param_9 + 0x20) = 1;
      param_8[1] = bVar16 + 1;
      goto LAB_14031b582;
    }
    if (bVar16 != 0x10) {
      *(undefined4 *)param_13[2] = *(undefined4 *)((longlong)param_13 + 0x1c);
      *(undefined1 *)(*param_13 + 0x31) = 0;
      *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
      *(undefined1 *)(*param_9 + 0x20) = 0;
      *param_8 = '\0';
      goto LAB_14031b30a;
    }
    param_8[1] = '\x11';
LAB_14031b313:
    lVar37 = *param_7;
    puStack_1a8 = &UNK_1429bc798;
    if (*(char *)(lVar37 + 0x120) != '\0') {
      puStack_1a8 = &UNK_1429bc79b;
    }
    uStack_1a0 = 3;
    uStack_49 = 1;
    lStack_178 = lVar37;
    ppuVar17 = (undefined **)FUN_140b9db30();
    uStack_49 = 1;
    FUN_140d62810(&ppuStack_170,ppuVar17,lVar37,&UNK_1429bc79e,0xb);
    uVar19 = FUN_140d76ed0(&ppuStack_170);
    FUN_140c96690(&puStack_198);
    puStack_138 = &UNK_14003bad0;
    ppuStack_130 = &puStack_1a8;
    uStack_128 = &UNK_140109e90;
    uStack_120 = (undefined **)&UNK_1429bc7b0;
    uStack_118 = (undefined **)0x2;
    uStack_110 = (undefined **)&uStack_140;
    uStack_108 = 2;
    uStack_104 = 0;
    uStack_102 = 0;
    uStack_100 = 0;
    uStack_fe = 0;
    uStack_fc = 0;
    uStack_fa = 0;
    uStack_140 = &puStack_198;
    FUN_140d79f90(&ppuStack_1c0,&uStack_120);
    if (puStack_198 != (undefined *)0x0) {
      func_0x000140613c20(uStack_190,puStack_198,1);
    }
    ppuStack_58 = ppuStack_1c0;
    plStack_148 = plStack_1b8;
    FUN_140d62810(&uStack_140,uVar19,ppuVar17,plStack_1b8,uStack_1b0);
    if (ppuStack_58 != (undefined **)0x0) {
      func_0x000140613c20(plStack_148,ppuStack_58,1);
    }
    if (ppuStack_170 != (undefined **)0x0) {
      func_0x000140613c20(uStack_168,ppuStack_170,1);
    }
    if (ppuVar21 == (undefined **)0x0) {
      ppuVar21 = (undefined **)0x0;
    }
    else {
      puVar18 = *ppuVar21;
      plVar20 = (longlong *)(puVar18 + 0x10);
      LOCK();
      bVar40 = *(longlong *)(puVar18 + 0x10) == 0;
      if (bVar40) {
        *(longlong *)(puVar18 + 0x10) = 8;
      }
      UNLOCK();
      if (!bVar40) {
        uStack_4a = 1;
        FUN_1428cb5f0(plVar20);
      }
      uStack_4a = 1;
      FUN_140c81c70(plVar20);
      uStack_4a = 1;
      FUN_140c81c70((ulonglong)plVar20 | 1);
      ppuVar21 = (undefined **)func_0x000140613c10(0x40,8);
      plStack_148 = plVar20;
      if (ppuVar21 == (undefined **)0x0) {
        FUN_1428d9000(8,0x40);
        goto LAB_14031c67d;
      }
      *ppuVar21 = (undefined *)0x1;
      ppuVar21[1] = (undefined *)0x1;
      *(undefined1 *)(ppuVar21 + 2) = 0;
      ppuVar21[3] = (undefined *)0x8000000000000001;
      LOCK();
      puVar8 = *ppuVar21;
      *ppuVar21 = *ppuVar21 + 1;
      UNLOCK();
      ppuStack_170 = ppuVar21;
      if (*ppuVar21 == (undefined *)0x0 || SCARRY8((longlong)puVar8,1) != (longlong)*ppuVar21 < 0)
      goto LAB_14031c67d;
      lVar37 = *(longlong *)(puVar18 + 0x28);
      uStack_120 = ppuVar21;
      ppuStack_58 = ppuVar21;
      if (lVar37 == *(longlong *)(puVar18 + 0x18)) {
        FUN_140562870(puVar18 + 0x18,&UNK_1429bc7d0);
      }
      ppuVar21 = ppuStack_130;
      puVar8 = puStack_138;
      *(undefined ***)(*(longlong *)(puVar18 + 0x20) + lVar37 * 8) = ppuStack_58;
      *(longlong *)(puVar18 + 0x28) = lVar37 + 1;
      if ((longlong)ppuStack_130 < 0) {
        uVar19 = 0;
LAB_14031c653:
        uStack_4c = 1;
        FUN_1428d8fe3(uVar19,ppuVar21,&UNK_1429ae438);
        goto LAB_14031c67d;
      }
      if (ppuStack_130 == (undefined **)0x0) {
        ppuVar17 = (undefined **)0x1;
      }
      else {
        uVar19 = 1;
        ppuVar17 = (undefined **)func_0x000140613c10(ppuStack_130,1);
        if (ppuVar17 == (undefined **)0x0) goto LAB_14031c653;
      }
      func_0x000142923c00(ppuVar17,puVar8,ppuVar21);
      uStack_100 = SUB82(ppuStack_58,0);
      uStack_fe = (undefined2)((ulonglong)ppuStack_58 >> 0x10);
      uStack_fc = (undefined2)((ulonglong)ppuStack_58 >> 0x20);
      uStack_fa = (undefined2)((ulonglong)ppuStack_58 >> 0x30);
      uStack_120 = ppuVar21;
      uStack_110 = ppuVar21;
      uStack_108 = CONCAT31(uStack_108._1_3_,(undefined1)uStack_128);
      uStack_118 = ppuVar17;
      ppuVar17 = (undefined **)func_0x000140613c10(0x28,8);
      plVar20 = plStack_148;
      if (ppuVar17 == (undefined **)0x0) {
        FUN_1428d9000(8,0x28);
        goto LAB_14031c67d;
      }
      ppuVar17[4] = (undefined *)
                    CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
      *(undefined4 *)(ppuVar17 + 2) = (undefined4)uStack_110;
      *(undefined4 *)((longlong)ppuVar17 + 0x14) = uStack_110._4_4_;
      *(undefined4 *)(ppuVar17 + 3) = uStack_108;
      *(uint *)((longlong)ppuVar17 + 0x1c) = CONCAT22(uStack_102,uStack_104);
      *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
      *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
      *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
      *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
      uStack_4a = 1;
      FUN_140c81e10(plStack_148);
      uStack_4a = 1;
      FUN_140c81e10();
      LOCK();
      lVar37 = *plVar20;
      if (lVar37 == 8) {
        *plVar20 = 0;
      }
      UNLOCK();
      ppuVar21 = (undefined **)0x1;
      if (lVar37 != 8) {
        uStack_4a = 1;
        FUN_1428cbf60(plVar20,0);
      }
    }
    ppuVar12 = ppuStack_68;
    cStack_d4 = '\x04' - (*piVar28 == 0x13);
    uStack_f8 = SUB81(ppuStack_130,0);
    uStack_f7 = (undefined1)((ulonglong)ppuStack_130 >> 8);
    uStack_f6 = (undefined2)((ulonglong)ppuStack_130 >> 0x10);
    uStack_f4 = (undefined2)((ulonglong)ppuStack_130 >> 0x20);
    uStack_f2 = (undefined2)((ulonglong)ppuStack_130 >> 0x30);
    uStack_f0 = (undefined4)uStack_128;
    uStack_ec = uStack_128._4_4_;
    uStack_108 = (undefined4)uStack_140;
    uStack_104 = (undefined2)((ulonglong)uStack_140 >> 0x20);
    uStack_102 = (undefined2)((ulonglong)uStack_140 >> 0x30);
    uStack_100 = SUB82(puStack_138,0);
    uStack_fe = (undefined2)((ulonglong)puStack_138 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)puStack_138 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)puStack_138 >> 0x30);
    uVar25 = 0x55;
    if (*(char *)(lStack_178 + 0x120) != '\0') {
      uVar25 = 100;
    }
    uStack_e0 = 0;
    uStack_e8 = CONCAT31(uStack_e8._1_3_,uVar25);
    uStack_110 = (undefined **)&UNK_1429b1280;
    uStack_120 = ppuVar21;
    uStack_118 = ppuVar17;
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78,&UNK_1429bc7e8);
    }
    ppuStack_70[(longlong)ppuVar12 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar12 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuStack_70[(longlong)ppuVar12 * 10 + 6] = (undefined *)CONCAT44(uStack_ec,uStack_f0);
    (ppuStack_70 + (longlong)ppuVar12 * 10 + 6)[1] = (undefined *)CONCAT44(uStack_e4,uStack_e8);
    ppuStack_70[(longlong)ppuVar12 * 10 + 4] =
         (undefined *)CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
    (ppuStack_70 + (longlong)ppuVar12 * 10 + 4)[1] =
         (undefined *)
         CONCAT26(uStack_f2,CONCAT24(uStack_f4,CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8))));
    ppuStack_70[(longlong)ppuVar12 * 10 + 2] = (undefined *)uStack_110;
    (ppuStack_70 + (longlong)ppuVar12 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
    ppuVar21 = ppuStack_70 + (longlong)ppuVar12 * 10;
    *(undefined4 *)ppuVar21 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar21 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar21 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar21 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar12 + 1);
    puVar18 = *ppuStack_88;
  }
  puVar8 = *ppuStack_150;
  uVar31 = 0;
  uVar38 = (longlong)puVar8 - *(ulonglong *)(lVar5 + 0x18);
  if (puVar8 < *(undefined **)(lVar5 + 0x18)) {
    uVar38 = uVar31;
  }
  uVar35 = (longlong)puVar8 - *(ulonglong *)(lVar5 + 0x38);
  if (puVar8 < *(undefined **)(lVar5 + 0x38)) {
    uVar35 = uVar31;
  }
  bVar40 = uVar35 <= *(ulonglong *)(lVar5 + 0x30);
  uVar35 = *(ulonglong *)(lVar5 + 0x30) - uVar35;
  bVar41 = uVar35 != 0;
  if (!bVar40) {
    uVar35 = uVar31;
  }
  bVar39 = uVar38 <= *(ulonglong *)(lVar5 + 0x10);
  uVar38 = *(ulonglong *)(lVar5 + 0x10) - uVar38;
  bVar42 = uVar38 != 0;
  if (!bVar39) {
    uVar38 = uVar31;
  }
  uVar38 = uVar38 + uVar35;
  if (bVar39 && bVar42 || bVar40 && bVar41) {
    uVar38 = uVar31;
  }
  *ppuStack_150 = (undefined *)(*(longlong *)(lVar5 + 0x40) - uVar38);
  uStack_49 = 1;
  cVar14 = func_0x00014093fa90(*(undefined4 *)(puVar18 + 0xf8));
  if (cVar14 == '\0') {
    bVar43 = false;
    piVar28 = (int *)*param_15;
    bVar13 = false;
    if (*piVar28 != 8) goto LAB_14031b8a3;
LAB_14031b8b2:
    bVar43 = bVar13;
    uStack_49 = 1;
    bVar15 = func_0x00014093fbe0(*puStack_80);
  }
  else {
    bVar43 = *(int *)*puStack_80 != 8;
    piVar28 = (int *)*param_15;
    bVar13 = bVar43;
    if (*piVar28 == 8) goto LAB_14031b8b2;
LAB_14031b8a3:
    bVar15 = 0;
  }
  if ((byte)(bVar43 | bVar15 | (bVar39 && bVar42 || bVar40 && bVar41)) == 1) {
    ppuVar17 = (undefined **)func_0x000140613c10(0x30,8);
    ppuVar21 = ppuStack_68;
    if (ppuVar17 == (undefined **)0x0) {
      uStack_49 = 1;
      FUN_1428d9000(8,0x30);
      goto LAB_14031c67d;
    }
    *ppuVar17 = (undefined *)0x1;
    ppuVar17[1] = (undefined *)0x1;
    *(undefined1 *)(ppuVar17 + 2) = 0;
    ppuVar17[3] = (undefined *)0x8000000000000000;
    LOCK();
    puVar18 = *ppuVar17;
    *ppuVar17 = *ppuVar17 + 1;
    UNLOCK();
    ppuStack_170 = ppuVar17;
    if (*ppuVar17 == (undefined *)0x0 || SCARRY8((longlong)puVar18,1) != (longlong)*ppuVar17 < 0)
    goto LAB_14031c67d;
    cStack_d4 = '\x06';
    uStack_e0 = 0;
    uStack_108 = 0;
    uStack_104 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB82(ppuVar17,0);
    uStack_fe = (undefined2)((ulonglong)ppuVar17 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)ppuVar17 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)ppuVar17 >> 0x30);
    uStack_f8 = 0x3c;
    uStack_120 = (undefined **)0x0;
    uStack_118._0_4_ = uStack_1d0;
    uStack_118._4_4_ = uStack_1cc;
    uStack_110 = (undefined **)CONCAT44(uStack_1c4,uStack_1c8);
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78,&UNK_1429bc830);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 6;
    *(undefined4 *)ppuVar17 = uStack_f0;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_ec;
    *(undefined4 *)(ppuVar17 + 1) = uStack_e8;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_e4;
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 4;
    *(uint *)ppuVar17 = CONCAT22(uStack_fe,uStack_100);
    *(uint *)((longlong)ppuVar17 + 4) = CONCAT22(uStack_fa,uStack_fc);
    *(uint *)(ppuVar17 + 1) = CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8));
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_f2,uStack_f4);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 2;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_110._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = uStack_108;
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_102,uStack_104);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
    uStack_120 = (undefined **)0x1;
    uStack_118 = (undefined **)0x1;
    uStack_110 = (undefined **)((ulonglong)uStack_110 & 0xffffffffffffff00);
    uStack_d2 = 7;
    ppuVar17 = (undefined **)func_0x000140613c10(0x50,8);
    ppuVar21 = ppuStack_68;
    if (ppuVar17 == (undefined **)0x0) {
      FUN_1428d9000(8,0x50);
      goto LAB_14031c67d;
    }
    ppuVar17[8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    ppuVar17[9] = (undefined *)
                  CONCAT17(uStack_d1,
                           CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuVar17[6] = (undefined *)CONCAT44(uStack_ec,uStack_f0);
    ppuVar17[7] = (undefined *)CONCAT44(uStack_e4,uStack_e8);
    ppuVar17[4] = (undefined *)
                  CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
    ppuVar17[5] = (undefined *)
                  CONCAT26(uStack_f2,
                           CONCAT24(uStack_f4,CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8))));
    ppuVar17[2] = (undefined *)uStack_110;
    ppuVar17[3] = (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
    *ppuVar17 = (undefined *)uStack_120;
    ppuVar17[1] = (undefined *)uStack_118;
    uVar2 = *(uint *)*param_9;
    uVar3 = ((uint *)*param_9)[1];
    LOCK();
    puVar18 = *ppuVar17;
    *ppuVar17 = *ppuVar17 + 1;
    UNLOCK();
    uStack_140 = ppuVar17;
    if (*ppuVar17 == (undefined *)0x0 || SCARRY8((longlong)puVar18,1) != (longlong)*ppuVar17 < 0)
    goto LAB_14031c67d;
    fVar44 = (float)uVar3 * _DAT_142925984;
    iStack_d8 = (int)(longlong)fVar44;
    if (fVar44 < 0.0) {
      iStack_d8 = 0;
    }
    if (_DAT_1429a07d0 < fVar44) {
      iStack_d8 = -1;
    }
    iStack_d8 = iStack_d8 + (uint)(iStack_d8 == 0);
    fVar44 = (float)uVar2 * _DAT_142925984;
    iStack_dc = (int)(longlong)fVar44;
    if (fVar44 < 0.0) {
      iStack_dc = 0;
    }
    if (_DAT_1429a07d0 < fVar44) {
      iStack_dc = -1;
    }
    iStack_dc = iStack_dc + (uint)(iStack_dc == 0);
    cStack_d4 = '\x06';
    uStack_e0 = 1;
    uStack_108 = 1;
    uStack_104 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB82(ppuVar17,0);
    uStack_fe = (undefined2)((ulonglong)ppuVar17 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)ppuVar17 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)ppuVar17 >> 0x30);
    uStack_120 = (undefined **)0x0;
    uStack_118 = (undefined **)CONCAT44(uStack_1cc,uStack_1d0);
    uStack_110 = (undefined **)CONCAT44(uStack_1c4,uStack_1c8);
    ppuStack_88 = ppuVar17;
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78,&UNK_1429bc848);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 6;
    *(undefined4 *)ppuVar17 = uStack_f0;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_ec;
    *(undefined4 *)(ppuVar17 + 1) = uStack_e8;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_e4;
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 4;
    *(uint *)ppuVar17 = CONCAT22(uStack_fe,uStack_100);
    *(uint *)((longlong)ppuVar17 + 4) = CONCAT22(uStack_fa,uStack_fc);
    *(uint *)(ppuVar17 + 1) = CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8));
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_f2,uStack_f4);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 2;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_110._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = uStack_108;
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_102,uStack_104);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
    uVar4 = *(undefined4 *)((longlong)param_10 + 0x1c);
    puVar9 = (undefined4 *)param_10[2];
    *puVar9 = uVar4;
    ppuStack_58 = (undefined **)*param_10;
    bVar16 = *(char *)((longlong)ppuStack_58 + 0x3e) - 7;
    bVar15 = 2;
    if (bVar16 < 2) {
      bVar15 = bVar16;
    }
    if (bVar15 != 0) {
      if (bVar15 == 1) {
        plVar20 = (longlong *)ppuStack_58[1];
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          FUN_140b5c8c0();
        }
      }
      else {
        ppuStack_150 = ppuStack_58 + 1;
        func_0x00014003bb80();
        FUN_140be24b0(ppuStack_150);
      }
    }
    ppuStack_58[1] = (undefined *)ppuStack_88;
    *(undefined4 *)(ppuStack_58 + 2) = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuStack_58 + 0x14) = uStack_120._4_4_;
    *(undefined4 *)(ppuStack_58 + 3) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuStack_58 + 0x1c) = uStack_118._4_4_;
    *(undefined4 *)(ppuStack_58 + 4) = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppuStack_58 + 0x24) = uStack_110._4_4_;
    *(undefined4 *)(ppuStack_58 + 5) = uStack_108;
    *(uint *)((longlong)ppuStack_58 + 0x2c) = CONCAT22(uStack_102,uStack_104);
    *(uint *)((longlong)ppuStack_58 + 0x2e) = CONCAT22(uStack_100,uStack_102);
    *(uint *)((longlong)ppuStack_58 + 0x32) = CONCAT22(uStack_fc,uStack_fe);
    *(uint *)((longlong)ppuStack_58 + 0x36) = CONCAT13(uStack_f7,CONCAT12(uStack_f8,uStack_fa));
    *(uint *)((longlong)ppuStack_58 + 0x3a) = CONCAT22(uStack_f4,uStack_f6);
    *(undefined1 *)((longlong)ppuStack_58 + 0x3e) = 8;
    ppuStack_88 = ppuStack_170;
    *puVar9 = uVar4;
    plVar20 = (longlong *)*ppuStack_58;
    if (plVar20 != (longlong *)0x0) {
      LOCK();
      *plVar20 = *plVar20 + -1;
      UNLOCK();
      if (*plVar20 == 0) {
        FUN_140b5c990(ppuStack_58);
      }
    }
    *ppuStack_58 = (undefined *)ppuStack_88;
  }
  if ((*(int *)*puStack_80 == 10) && ((*(byte *)((int *)*puStack_80 + 2) & 1) == 0)) {
    *(undefined4 *)param_15[2] = *(undefined4 *)((longlong)param_15 + 0x1c);
    uStack_118 = (undefined **)CONCAT71(uStack_118._1_7_,1);
    uStack_120 = (undefined **)0xa;
    uStack_49 = 1;
    FUN_14093fad0(piVar28,&uStack_120);
    fVar44 = (float)*(uint *)(*param_9 + 4) * _DAT_142925970;
    iVar23 = 0;
    if (0.0 <= fVar44) {
      iVar23 = (int)(longlong)fVar44;
    }
    iVar30 = -1;
    if (fVar44 <= _DAT_1429a07d0) {
      iVar30 = iVar23;
    }
    iVar30 = iVar30 + (uint)(iVar30 == 0);
    uStack_120 = (undefined **)0x1;
    uStack_118 = (undefined **)0x1;
    uStack_110 = (undefined **)((ulonglong)uStack_110 & 0xffffffffffffff00);
    uStack_d2 = 7;
    ppuVar17 = (undefined **)func_0x000140613c10(0x50,8);
    ppuVar21 = ppuStack_68;
    if (ppuVar17 == (undefined **)0x0) {
      FUN_1428d9000(8,0x50);
      goto LAB_14031c67d;
    }
    ppuVar17[8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    ppuVar17[9] = (undefined *)
                  CONCAT17(uStack_d1,
                           CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    *(undefined4 *)(ppuVar17 + 6) = uStack_f0;
    *(undefined4 *)((longlong)ppuVar17 + 0x34) = uStack_ec;
    *(undefined4 *)(ppuVar17 + 7) = uStack_e8;
    *(undefined4 *)((longlong)ppuVar17 + 0x3c) = uStack_e4;
    *(uint *)(ppuVar17 + 4) = CONCAT22(uStack_fe,uStack_100);
    *(uint *)((longlong)ppuVar17 + 0x24) = CONCAT22(uStack_fa,uStack_fc);
    *(uint *)(ppuVar17 + 5) = CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8));
    *(uint *)((longlong)ppuVar17 + 0x2c) = CONCAT22(uStack_f2,uStack_f4);
    *(undefined4 *)(ppuVar17 + 2) = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppuVar17 + 0x14) = uStack_110._4_4_;
    *(undefined4 *)(ppuVar17 + 3) = uStack_108;
    *(uint *)((longlong)ppuVar17 + 0x1c) = CONCAT22(uStack_102,uStack_104);
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    LOCK();
    puVar18 = *ppuVar17;
    *ppuVar17 = *ppuVar17 + 1;
    UNLOCK();
    uStack_140 = ppuVar17;
    if (*ppuVar17 == (undefined *)0x0 || SCARRY8((longlong)puVar18,1) != (longlong)*ppuVar17 < 0)
    goto LAB_14031c67d;
    cStack_d4 = '\x01';
    uStack_e0 = 1;
    uStack_108 = 1;
    uStack_104 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB82(ppuVar17,0);
    uStack_fe = (undefined2)((ulonglong)ppuVar17 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)ppuVar17 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)ppuVar17 >> 0x30);
    uStack_120 = (undefined **)0x0;
    iStack_dc = iVar30;
    iStack_d8 = iVar30;
    ppuStack_58 = ppuVar17;
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78,&UNK_1429bc860);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 6;
    *(undefined4 *)ppuVar17 = uStack_f0;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_ec;
    *(undefined4 *)(ppuVar17 + 1) = uStack_e8;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_e4;
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 4;
    *(uint *)ppuVar17 = CONCAT22(uStack_fe,uStack_100);
    *(uint *)((longlong)ppuVar17 + 4) = CONCAT22(uStack_fa,uStack_fc);
    *(uint *)(ppuVar17 + 1) = CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8));
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_f2,uStack_f4);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 2;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_110._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = uStack_108;
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_102,uStack_104);
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
    *(undefined4 *)param_11[2] = *(undefined4 *)((longlong)param_11 + 0x1c);
    plVar20 = *(longlong **)(*param_11 + 8);
    puStack_80 = (undefined8 *)(*param_11 + 8);
    if (plVar20 != (longlong *)0x0) {
      LOCK();
      *plVar20 = *plVar20 + -1;
      UNLOCK();
      if (*plVar20 == 0) {
        FUN_140b5c8c0(puStack_80);
      }
    }
    *puStack_80 = ppuStack_58;
  }
  ppuVar21 = ppuStack_68;
  if (bVar39 && bVar42 || bVar40 && bVar41) {
    plVar20 = *(longlong **)*puStack_180;
    LOCK();
    lVar5 = *plVar20;
    *plVar20 = *plVar20 + 1;
    UNLOCK();
    if (*plVar20 == 0 || SCARRY8(lVar5,1) != *plVar20 < 0) goto LAB_14031c67d;
    cStack_d4 = '\x02';
    uStack_e0 = 0;
    uStack_108 = 1;
    uStack_104 = 0;
    uStack_102 = 0x8000;
    uStack_100 = SUB82(plVar20,0);
    uStack_fe = (undefined2)((ulonglong)plVar20 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)plVar20 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)plVar20 >> 0x30);
    uStack_120 = (undefined **)0x0;
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuStack_70[(longlong)ppuVar21 * 10 + 6] = (undefined *)CONCAT44(uStack_ec,uStack_f0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 6)[1] = (undefined *)CONCAT44(uStack_e4,uStack_e8);
    ppuStack_70[(longlong)ppuVar21 * 10 + 4] =
         (undefined *)CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 4)[1] =
         (undefined *)
         CONCAT26(uStack_f2,CONCAT24(uStack_f4,CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8))));
    ppuStack_70[(longlong)ppuVar21 * 10 + 2] = (undefined *)uStack_110;
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
  }
  lVar5 = param_14[1];
  uVar38 = *(ulonglong *)*param_14;
  uVar35 = 0;
  uVar31 = uVar38 - *(ulonglong *)(lVar5 + 0x18);
  if (uVar38 < *(ulonglong *)(lVar5 + 0x18)) {
    uVar31 = uVar35;
  }
  uVar36 = uVar38 - *(ulonglong *)(lVar5 + 0x38);
  if (uVar38 < *(ulonglong *)(lVar5 + 0x38)) {
    uVar36 = uVar35;
  }
  lVar37 = uVar31 * 0x28 + *(longlong *)(lVar5 + 8);
  lVar33 = uVar36 * 0x28 + *(longlong *)(lVar5 + 0x28);
  bVar40 = *(ulonglong *)(lVar5 + 0x10) < uVar31;
  uVar31 = *(ulonglong *)(lVar5 + 0x10) - uVar31;
  bVar41 = uVar31 == 0;
  if (bVar40) {
    uVar31 = uVar35;
  }
  if (bVar40) {
    lVar37 = 8;
  }
  uVar38 = lVar37 + -0x28 + uVar31 * 0x28;
  if (bVar40 || bVar41) {
    uVar38 = uVar35;
  }
  bVar39 = *(ulonglong *)(lVar5 + 0x30) < uVar36;
  uVar36 = *(ulonglong *)(lVar5 + 0x30) - uVar36;
  bVar42 = uVar36 == 0;
  if (bVar39) {
    lVar33 = 8;
    uVar36 = uVar35;
  }
  uVar27 = lVar33 + -0x28 + uVar36 * 0x28;
  if (bVar39 || bVar42) {
    uVar27 = uVar38;
  }
  uVar38 = uVar36 + uVar31;
  if (!bVar40 && !bVar41 || !bVar39 && !bVar42) {
    uVar38 = uVar35;
  }
  *(ulonglong *)*param_14 = *(longlong *)(lVar5 + 0x40) - uVar38;
  if (!bVar40 && !bVar41 || !bVar39 && !bVar42) {
    uStack_49 = 1;
    puVar18 = (undefined *)FUN_140d76ed0(uVar27);
    uStack_49 = 1;
    FUN_140d62810(&uStack_140,puVar18,uVar35,&UNK_1429bc890,0xe);
    ppuVar21 = ppuStack_68;
    uStack_f8 = SUB81(ppuStack_130,0);
    uStack_f7 = (undefined1)((ulonglong)ppuStack_130 >> 8);
    uStack_f6 = (undefined2)((ulonglong)ppuStack_130 >> 0x10);
    uStack_f4 = (undefined2)((ulonglong)ppuStack_130 >> 0x20);
    uStack_f2 = (undefined2)((ulonglong)ppuStack_130 >> 0x30);
    uStack_f0 = (undefined4)uStack_128;
    uStack_ec = uStack_128._4_4_;
    uStack_108 = SUB84(uStack_140,0);
    uStack_104 = (undefined2)((ulonglong)uStack_140 >> 0x20);
    uStack_102 = (undefined2)((ulonglong)uStack_140 >> 0x30);
    uStack_100 = SUB82(puStack_138,0);
    uStack_fe = (undefined2)((ulonglong)puStack_138 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)puStack_138 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)puStack_138 >> 0x30);
    cStack_d4 = 2;
    uStack_e0 = 1;
    iStack_dc = 0x200;
    iStack_d8 = 0x200;
    uStack_e8 = CONCAT31(uStack_e8._1_3_,0x55);
    uStack_120 = (undefined **)0x0;
    uStack_118 = (undefined **)CONCAT44(uStack_1dc,uStack_1e0);
    uStack_110 = (undefined **)CONCAT44(uStack_1d4,uStack_1d8);
    if (ppuStack_68 == ppuStack_78) {
      puVar18 = &UNK_1429bc8a0;
      FUN_140563540(&ppuStack_78,&UNK_1429bc8a0);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 6;
    *(undefined4 *)ppuVar17 = uStack_f0;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_ec;
    *(undefined4 *)(ppuVar17 + 1) = uStack_e8;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_e4;
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10 + 4;
    *(uint *)ppuVar17 = CONCAT22(uStack_fe,uStack_100);
    *(uint *)((longlong)ppuVar17 + 4) = CONCAT22(uStack_fa,uStack_fc);
    *(uint *)(ppuVar17 + 1) = CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8));
    *(uint *)((longlong)ppuVar17 + 0xc) = CONCAT22(uStack_f2,uStack_f4);
    ppuStack_70[(longlong)ppuVar21 * 10 + 2] = (undefined *)uStack_110;
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
    ppuStack_70[(longlong)ppuVar21 * 10] = (undefined *)uStack_120;
    (ppuStack_70 + (longlong)ppuVar21 * 10)[1] = (undefined *)uStack_118;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
    uVar2 = ((uint *)*param_9)[1];
    fVar44 = ((float)*(uint *)*param_9 * _DAT_1429bc430) / (float)(uVar2 + (uVar2 == 0));
    if (fVar44 <= _DAT_142925904) {
      fVar44 = _DAT_142925904;
    }
    iVar23 = 0;
    if (0.0 <= fVar44) {
      iVar23 = (int)(longlong)fVar44;
    }
    iVar30 = -1;
    if (fVar44 <= _DAT_1429a07d0) {
      iVar30 = iVar23;
    }
    uStack_49 = 1;
    uVar19 = FUN_140d76ed0(uVar27);
    uStack_49 = 1;
    FUN_140d62810(&uStack_140,uVar19,puVar18,&UNK_1429bc8b8,0x16);
    ppuVar21 = ppuStack_68;
    uStack_f8 = SUB81(ppuStack_130,0);
    uStack_f7 = (undefined1)((ulonglong)ppuStack_130 >> 8);
    uStack_f6 = (undefined2)((ulonglong)ppuStack_130 >> 0x10);
    uStack_f4 = (undefined2)((ulonglong)ppuStack_130 >> 0x20);
    uStack_f2 = (undefined2)((ulonglong)ppuStack_130 >> 0x30);
    uStack_f0 = (undefined4)uStack_128;
    uStack_ec = uStack_128._4_4_;
    uStack_108 = SUB84(uStack_140,0);
    uStack_104 = (undefined2)((ulonglong)uStack_140 >> 0x20);
    uStack_102 = (undefined2)((ulonglong)uStack_140 >> 0x30);
    uStack_100 = SUB82(puStack_138,0);
    uStack_fe = (undefined2)((ulonglong)puStack_138 >> 0x10);
    uStack_fc = (undefined2)((ulonglong)puStack_138 >> 0x20);
    uStack_fa = (undefined2)((ulonglong)puStack_138 >> 0x30);
    cStack_d4 = '\x02';
    uStack_e0 = 1;
    iStack_d8 = 0x40;
    uStack_e8 = CONCAT31(uStack_e8._1_3_,0x5f);
    uStack_120 = (undefined **)0x0;
    uStack_118 = (undefined **)CONCAT44(uStack_1dc,uStack_1e0);
    uStack_110 = (undefined **)CONCAT44(uStack_1d4,uStack_1d8);
    iStack_dc = iVar30;
    if (ppuStack_68 == ppuStack_78) {
      FUN_140563540(&ppuStack_78,&UNK_1429bc8d0);
    }
    ppuStack_70[(longlong)ppuVar21 * 10 + 8] = (undefined *)CONCAT44(iStack_dc,uStack_e0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 8)[1] =
         (undefined *)
         CONCAT17(uStack_d1,CONCAT16(uStack_d2,CONCAT15(uStack_d3,CONCAT14(cStack_d4,iStack_d8))));
    ppuStack_70[(longlong)ppuVar21 * 10 + 6] = (undefined *)CONCAT44(uStack_ec,uStack_f0);
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 6)[1] = (undefined *)CONCAT44(uStack_e4,uStack_e8);
    ppuStack_70[(longlong)ppuVar21 * 10 + 4] =
         (undefined *)CONCAT26(uStack_fa,CONCAT24(uStack_fc,CONCAT22(uStack_fe,uStack_100)));
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 4)[1] =
         (undefined *)
         CONCAT26(uStack_f2,CONCAT24(uStack_f4,CONCAT22(uStack_f6,CONCAT11(uStack_f7,uStack_f8))));
    ppuStack_70[(longlong)ppuVar21 * 10 + 2] = (undefined *)uStack_110;
    (ppuStack_70 + (longlong)ppuVar21 * 10 + 2)[1] =
         (undefined *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
    ppuVar17 = ppuStack_70 + (longlong)ppuVar21 * 10;
    *(undefined4 *)ppuVar17 = (undefined4)uStack_120;
    *(undefined4 *)((longlong)ppuVar17 + 4) = uStack_120._4_4_;
    *(undefined4 *)(ppuVar17 + 1) = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppuVar17 + 0xc) = uStack_118._4_4_;
    ppuStack_68 = (undefined **)((longlong)ppuVar21 + 1);
  }
  LOCK();
  bVar40 = DAT_14362b050 == '\0';
  if (bVar40) {
    DAT_14362b050 = '\x01';
  }
  UNLOCK();
  if (!bVar40) {
    uStack_49 = 1;
    FUN_1428cd210(&DAT_14362b050);
  }
  uStack_49 = 1;
  FUN_140c81c70(&DAT_14362b050);
  ppuVar21 = ppuStack_70 + (longlong)ppuStack_68 * 10;
  uStack_120 = ppuStack_70;
  uStack_118 = ppuStack_70;
  uStack_110 = ppuStack_78;
  uStack_108 = SUB84(ppuVar21,0);
  uStack_104 = (undefined2)((ulonglong)ppuVar21 >> 0x20);
  uStack_102 = (undefined2)((ulonglong)ppuVar21 >> 0x30);
  if (_DAT_14362b058 + -(longlong)_DAT_14362b068 < ppuStack_68) {
    uStack_5a = 1;
    FUN_1428a29e0(&DAT_14362b058,_DAT_14362b068,ppuStack_68,8,0x98);
  }
  uStack_140 = (undefined **)&DAT_14362b068;
  puStack_138 = _DAT_14362b068;
  ppuStack_130 = _DAT_14362b060;
  uStack_5a = 0;
  FUN_14048db70(&uStack_120,&uStack_140);
  uVar19 = *param_12;
  plVar20 = (longlong *)func_0x00014091f0c0(uVar19);
  if ((plVar20 == (longlong *)0x0) || (*plVar20 != -0x7ffffffffffffff7)) goto LAB_14031c443;
  puVar22 = (undefined8 *)plVar20[3];
  if ((longlong)puVar22 < 0) {
    uVar29 = 0;
LAB_14031c62c:
    FUN_1428d8fe3(uVar29,puVar22,&UNK_1429ae438);
LAB_14031c67d:
                    /* WARNING: Does not return */
    pcVar10 = (code *)invalidInstructionException();
    (*pcVar10)();
  }
  lVar5 = plVar20[2];
  if (puVar22 == (undefined8 *)0x0) {
    ppuVar21 = (undefined **)0x1;
  }
  else {
    uVar29 = 1;
    ppuVar21 = (undefined **)func_0x000140613c10(puVar22,1);
    if (ppuVar21 == (undefined **)0x0) goto LAB_14031c62c;
  }
  puStack_80 = puVar22;
  ppuStack_58 = ppuVar21;
  func_0x000142923c00(ppuVar21,lVar5,puVar22);
  lVar5 = plVar20[4];
  *(undefined4 *)param_12[2] = *(undefined4 *)((longlong)param_12 + 0x1c);
  uStack_4f = 1;
  func_0x00014091f100(&uStack_120,uVar19);
  if (uStack_120 != (undefined **)0x8000000000000014) {
    uVar38 = 0xd;
    if (((ulonglong)uStack_120 ^ 0x8000000000000000) < 0x14) {
      uVar38 = (ulonglong)uStack_120 ^ 0x8000000000000000;
    }
    if (uVar38 < 0x10) {
      puVar22 = &uStack_110;
      switch(uVar38) {
      case 0:
        plVar20 = (longlong *)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
        if (plVar20 != (longlong *)0x0) {
          LOCK();
          *plVar20 = *plVar20 + -1;
          UNLOCK();
          if (*plVar20 == 0) {
            uStack_4f = 1;
            func_0x0001402e5d80(&uStack_108);
          }
        }
        break;
      case 8:
      case 9:
      case 0xc:
      case 0xe:
      case 0xf:
        ppuVar21 = uStack_118;
        if (uStack_118 != (undefined **)0x0) {
code_r0x00014031c37f:
          func_0x000140613c20(*puVar22,ppuVar21,1);
        }
        break;
      case 0xd:
        if (uStack_120 != (undefined **)0x0) {
          func_0x000140613c20(uStack_118,uStack_120,1);
        }
        ppuVar21 = (undefined **)CONCAT26(uStack_102,CONCAT24(uStack_104,uStack_108));
        if (((ulonglong)ppuVar21 & 0x7fffffffffffffff) != 0) {
          puVar22 = (undefined8 *)&uStack_100;
          goto code_r0x00014031c37f;
        }
      }
    }
  }
  puVar18 = _DAT_14362b068;
  uStack_108 = SUB84(puStack_80,0);
  uStack_104 = (undefined2)((ulonglong)puStack_80 >> 0x20);
  uStack_102 = (undefined2)((ulonglong)puStack_80 >> 0x30);
  uStack_100 = SUB82(ppuStack_58,0);
  uStack_fe = (undefined2)((ulonglong)ppuStack_58 >> 0x10);
  uStack_fc = (undefined2)((ulonglong)ppuStack_58 >> 0x20);
  uStack_fa = (undefined2)((ulonglong)ppuStack_58 >> 0x30);
  uStack_f8 = SUB81(puStack_80,0);
  uStack_f7 = (undefined1)((ulonglong)puStack_80 >> 8);
  uStack_f6 = (undefined2)((ulonglong)puStack_80 >> 0x10);
  uStack_f0 = CONCAT31((int3)uStack_140,(char)lVar5);
  uStack_ec = uStack_140._3_4_;
  uStack_120 = (undefined **)0x0;
  uStack_e8 = CONCAT31(uStack_e8._1_3_,0x55);
  uStack_e0 = 0;
  cStack_d4 = '\x04';
  uStack_d0 = 1;
  uStack_8a = 6;
  uStack_f4 = uStack_104;
  uStack_f2 = uStack_102;
  if (_DAT_14362b068 == _DAT_14362b058) {
    FUN_140562ff0(&DAT_14362b058,&UNK_1429bc8e8);
  }
  func_0x000142923c10(_DAT_14362b060 + (longlong)puVar18 * 0x13,&uStack_120,0x98);
  _DAT_14362b068 = puVar18 + 1;
LAB_14031c443:
  uStack_49 = 0;
  FUN_140c81e10();
  LOCK();
  bVar40 = DAT_14362b050 == '\x01';
  cVar14 = DAT_14362b050;
  if (bVar40) {
    DAT_14362b050 = '\0';
    cVar14 = '\x01';
  }
  UNLOCK();
  if (!bVar40) {
    uStack_49 = 0;
    cVar14 = FUN_1428cdb90(&DAT_14362b050,0);
  }
  return cVar14;
}

