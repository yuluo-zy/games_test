
longlong FUN_140937560(longlong param_1,undefined8 param_2,ulonglong param_3)

{
  longlong *plVar1;
  undefined1 auVar2 [16];
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  longlong lVar6;
  ushort uVar7;
  longlong lVar8;
  uint uVar9;
  ulonglong uVar10;
  bool bVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined *puStack_110;
  undefined8 uStack_108;
  undefined8 **ppuStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  ulonglong *puStack_70;
  code *pcStack_68;
  ulonglong uStack_60;
  longlong *plStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  plVar1 = (longlong *)(param_1 + 0xb8);
  LOCK();
  bVar11 = *(longlong *)(param_1 + 0xb8) == 0;
  if (bVar11) {
    *(longlong *)(param_1 + 0xb8) = 8;
  }
  UNLOCK();
  uStack_60 = param_3;
  uStack_48 = param_2;
  if (!bVar11) {
    uStack_39 = 1;
    FUN_1428cb5f0(plVar1,param_2,1000000000);
  }
  uStack_39 = 1;
  FUN_140c81c70(plVar1);
  uVar10 = (ulonglong)plVar1 | 1;
  uStack_39 = 1;
  FUN_140c81c70(uVar10);
  lVar6 = lRam0000000143634cd0;
  plStack_58 = plVar1;
  uStack_50 = uVar10;
  if (*(longlong *)(param_1 + 0xd8) != 0) {
    bVar12 = (byte)(uStack_60 >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar8 = 0;
    uVar10 = uStack_60;
    while( true ) {
      uVar10 = *(ulonglong *)(param_1 + 200) & uVar10;
      auVar2 = *(undefined1 (*) [16])(*(longlong *)(param_1 + 0xc0) + uVar10);
      cVar13 = auVar14[0];
      auVar19[0] = -(auVar2[0] == cVar13);
      cVar15 = auVar14[1];
      auVar19[1] = -(auVar2[1] == cVar15);
      cVar16 = auVar14[2];
      auVar19[2] = -(auVar2[2] == cVar16);
      cVar17 = auVar14[3];
      auVar19[3] = -(auVar2[3] == cVar17);
      auVar19[4] = -(auVar2[4] == cVar13);
      auVar19[5] = -(auVar2[5] == cVar15);
      auVar19[6] = -(auVar2[6] == cVar16);
      auVar19[7] = -(auVar2[7] == cVar17);
      auVar19[8] = -(auVar2[8] == cVar13);
      auVar19[9] = -(auVar2[9] == cVar15);
      auVar19[10] = -(auVar2[10] == cVar16);
      auVar19[0xb] = -(auVar2[0xb] == cVar17);
      auVar19[0xc] = -(auVar2[0xc] == cVar13);
      auVar19[0xd] = -(auVar2[0xd] == cVar15);
      auVar19[0xe] = -(auVar2[0xe] == cVar16);
      auVar19[0xf] = -(auVar2[0xf] == cVar17);
      uVar7 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar9 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        if (uStack_60 ==
            *(ulonglong *)
             (*(longlong *)(param_1 + 0xc0) + -0x10 +
             (uVar4 + uVar10 & *(ulonglong *)(param_1 + 200)) * -0x10)) {
          puStack_70 = &uStack_60;
          pcStack_68 = FUN_140939420;
          puStack_110 = &UNK_142ab10d0;
          uStack_108 = 2;
          uStack_f0 = 0;
          ppuStack_100 = &puStack_70;
          uStack_f8 = 1;
          uStack_3a = 1;
          FUN_1428d9390(&puStack_110,&UNK_142ab11d8);
          goto code_r0x0001409377f2;
        }
        uVar7 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar7);
      }
      auVar18[0] = -(auVar2[0] == -1);
      auVar18[1] = -(auVar2[1] == -1);
      auVar18[2] = -(auVar2[2] == -1);
      auVar18[3] = -(auVar2[3] == -1);
      auVar18[4] = -(auVar2[4] == -1);
      auVar18[5] = -(auVar2[5] == -1);
      auVar18[6] = -(auVar2[6] == -1);
      auVar18[7] = -(auVar2[7] == -1);
      auVar18[8] = -(auVar2[8] == -1);
      auVar18[9] = -(auVar2[9] == -1);
      auVar18[10] = -(auVar2[10] == -1);
      auVar18[0xb] = -(auVar2[0xb] == -1);
      auVar18[0xc] = -(auVar2[0xc] == -1);
      auVar18[0xd] = -(auVar2[0xd] == -1);
      auVar18[0xe] = -(auVar2[0xe] == -1);
      auVar18[0xf] = -(auVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      uVar10 = uVar10 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
  LOCK();
  lRam0000000143634cd0 = lRam0000000143634cd0 + 1;
  UNLOCK();
  uStack_3a = 1;
  FUN_141529ea0(param_1 + 0xc0,uStack_60,lVar6);
  func_0x000142923c00(&puStack_110,uStack_48,0xa0);
  lVar8 = func_0x000140613c10(0xa0,8);
  if (lVar8 == 0) {
    FUN_1428d9000(8,0xa0);
  }
  else {
    func_0x000142923c00(lVar8,&puStack_110,0xa0);
    uStack_3a = 0;
    lVar8 = FUN_14096a1c0(param_1 + 0x40,lVar6,lVar8);
    plVar1 = plStack_58;
    if (lVar8 == 0) {
      uStack_39 = 0;
      FUN_140c81e10(plStack_58);
      uStack_39 = 0;
      FUN_140c81e10();
      LOCK();
      lVar8 = *plVar1;
      if (lVar8 == 8) {
        *plVar1 = 0;
      }
      UNLOCK();
      if (lVar8 != 8) {
        uStack_39 = 0;
        FUN_1428cbf60(plVar1,0);
      }
      return lVar6;
    }
    uStack_3a = 0;
    FUN_1428d9430(&UNK_142ab1170,0x4f,&UNK_142ab11c0);
  }
code_r0x0001409377f2:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

