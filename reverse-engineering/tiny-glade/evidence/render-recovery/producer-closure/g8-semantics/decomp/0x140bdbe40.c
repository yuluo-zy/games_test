
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140bdbe40(longlong param_1,longlong *param_2)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [16];
  char cVar4;
  undefined1 uVar5;
  undefined2 uVar6;
  undefined4 uVar7;
  undefined1 (*pauVar8) [16];
  ulonglong uVar9;
  undefined8 uVar10;
  longlong lVar11;
  ushort uVar12;
  uint uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  uint uVar22;
  ulonglong uVar23;
  undefined1 (*pauVar24) [16];
  undefined1 (*pauVar25) [16];
  uint uVar26;
  longlong lVar27;
  bool bVar28;
  
  uVar1 = param_1 + 0x38;
  LOCK();
  bVar28 = *(longlong *)(param_1 + 0x38) == 0;
  if (bVar28) {
    *(longlong *)(param_1 + 0x38) = 8;
  }
  UNLOCK();
  if (!bVar28) {
    FUN_1428cb5f0(uVar1,param_2,1000000000);
  }
  FUN_140c81c70(uVar1);
  FUN_140c81c70(uVar1 | 1);
  uVar10 = _UNK_142ce7ac8;
  pauVar8 = *(undefined1 (**) [16])(param_1 + 0x40);
  uVar9 = *(ulonglong *)(param_1 + 0x48);
  auVar3 = *pauVar8;
  lVar27 = *(longlong *)(param_1 + 0x58);
  *(undefined8 *)(param_1 + 0x40) = _UNK_142ce7ac0;
  *(undefined8 *)(param_1 + 0x48) = uVar10;
  uVar10 = _UNK_142ce7ad8;
  *(undefined8 *)(param_1 + 0x50) = _UNK_142ce7ad0;
  *(undefined8 *)(param_1 + 0x58) = uVar10;
  if (lVar27 != 0) {
    pauVar24 = pauVar8 + 1;
    pauVar25 = pauVar8;
    uVar22 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar22 == 0) {
        do {
          auVar3 = *pauVar24;
          uVar12 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
          pauVar25 = pauVar25 + -0x48;
          pauVar24 = pauVar24 + 1;
        } while (uVar12 == 0xffff);
        uVar22 = ~(uint)uVar12;
        uVar26 = uVar22 & -(uint)uVar12 - 2;
      }
      else {
        uVar26 = uVar22 - 1 & uVar22;
      }
      uVar13 = 0;
      for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
        uVar13 = uVar13 + 1;
      }
      if (*(int *)((longlong)pauVar25 + (ulonglong)uVar13 * -0x48 + -0x48) == 2) break;
      uVar10 = *(undefined8 *)((longlong)pauVar25 + (ulonglong)uVar13 * -0x48 + -8);
      lVar11 = param_2[5];
      if (lVar11 == param_2[3]) {
        FUN_140562870(param_2 + 3,&UNK_142d052f0);
      }
      lVar27 = lVar27 + -1;
      *(undefined8 *)(param_2[4] + lVar11 * 8) = uVar10;
      param_2[5] = lVar11 + 1;
      uVar22 = uVar26;
    } while (lVar27 != 0);
  }
  if (uVar9 == 0) {
    uVar23 = 0;
  }
  else {
    func_0x000142923c20(pauVar8,0xff,uVar9 + 0x11);
    uVar23 = (uVar9 + 1 & 0xfffffffffffffff8) - (uVar9 + 1 >> 3);
    if (uVar9 < 8) {
      uVar23 = uVar9;
    }
  }
  *(undefined1 (**) [16])(param_1 + 0x40) = pauVar8;
  *(ulonglong *)(param_1 + 0x48) = uVar9;
  *(ulonglong *)(param_1 + 0x50) = uVar23;
  *(undefined8 *)(param_1 + 0x58) = 0;
  FUN_140c81e10(uVar1);
  FUN_140c81e10();
  LOCK();
  bVar28 = *(longlong *)(param_1 + 0x38) == 8;
  if (bVar28) {
    *(longlong *)(param_1 + 0x38) = 0;
  }
  UNLOCK();
  if (bVar28) {
    cVar4 = *(char *)(param_1 + 0xbc);
    *(undefined1 *)(param_1 + 0xbc) = 2;
  }
  else {
    FUN_1428cbf60(uVar1,0);
    cVar4 = *(char *)(param_1 + 0xbc);
    *(undefined1 *)(param_1 + 0xbc) = 2;
  }
  if (cVar4 != '\x02') {
    uVar7 = *(undefined4 *)(param_1 + 0xb8);
    uVar14 = *(undefined8 *)(param_1 + 0x78);
    uVar15 = *(undefined8 *)(param_1 + 0x80);
    uVar16 = *(undefined8 *)(param_1 + 0x88);
    uVar17 = *(undefined8 *)(param_1 + 0x90);
    uVar18 = *(undefined8 *)(param_1 + 0x98);
    uVar19 = *(undefined8 *)(param_1 + 0xa0);
    uVar20 = *(undefined8 *)(param_1 + 0xa8);
    uVar21 = *(undefined8 *)(param_1 + 0xb0);
    uVar5 = *(undefined1 *)(param_1 + 0xbf);
    uVar6 = *(undefined2 *)(param_1 + 0xbd);
    uVar10 = *(undefined8 *)(param_1 + 0x30);
    lVar27 = param_2[2];
    if (lVar27 == *param_2) {
      FUN_140562870(param_2,&UNK_142d052d8);
    }
    *(undefined8 *)(param_2[1] + lVar27 * 8) = uVar10;
    param_2[2] = lVar27 + 1;
    lVar27 = param_2[0x1d];
    if (lVar27 == param_2[0x1b]) {
      FUN_140b5f9f0(param_2 + 0x1b,&UNK_142d053d8);
    }
    lVar11 = param_2[0x1c];
    *(ulonglong *)(lVar11 + 0x40 + lVar27 * 0x48) =
         CONCAT17(uVar5,CONCAT25(uVar6,CONCAT14(cVar4,uVar7)));
    puVar2 = (undefined8 *)(lVar11 + 0x30 + lVar27 * 0x48);
    *puVar2 = uVar20;
    puVar2[1] = uVar21;
    puVar2 = (undefined8 *)(lVar11 + 0x20 + lVar27 * 0x48);
    *puVar2 = uVar18;
    puVar2[1] = uVar19;
    puVar2 = (undefined8 *)(lVar11 + 0x10 + lVar27 * 0x48);
    *puVar2 = uVar16;
    puVar2[1] = uVar17;
    puVar2 = (undefined8 *)(lVar11 + lVar27 * 0x48);
    *puVar2 = uVar14;
    puVar2[1] = uVar15;
    param_2[0x1d] = lVar27 + 1;
  }
  lVar27 = *(longlong *)(param_1 + 0x70);
  if (lVar27 != 0) {
    lVar11 = param_2[5];
    if (lVar11 == param_2[3]) {
      FUN_140562870(param_2 + 3,&UNK_142d052f0);
    }
    *(longlong *)(param_2[4] + lVar11 * 8) = lVar27;
    param_2[5] = lVar11 + 1;
  }
  *(undefined8 *)(param_1 + 0x30) = 0;
  return;
}

