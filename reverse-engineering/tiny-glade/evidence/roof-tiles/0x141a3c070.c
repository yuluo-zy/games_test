
ulonglong FUN_141a3c070(longlong *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  ulonglong *puVar2;
  undefined8 *puVar3;
  byte bVar4;
  uint uVar5;
  longlong lVar6;
  longlong *plVar7;
  code *pcVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  undefined4 uVar24;
  undefined8 uVar25;
  undefined8 uVar26;
  undefined8 uVar27;
  undefined8 uVar28;
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  longlong *plVar32;
  ulonglong uVar33;
  ulonglong unaff_R14;
  undefined8 uVar34;
  
  uVar34 = 0xfffffffffffffffe;
  uVar5 = *(uint *)(param_1 + 10);
  uVar31 = (ulonglong)uVar5;
  if (uVar31 < (ulonglong)param_1[1]) {
    lVar6 = *param_1;
    bVar4 = *(byte *)(lVar6 + uVar31);
    if ((ulonglong)bVar4 < 0x20) {
      uVar29 = uVar31 << 5 | (ulonglong)bVar4;
      *(byte *)(lVar6 + uVar31) = bVar4 + 1;
      if ((ulonglong)param_1[6] <= uVar29) goto LAB_141a3c2b7;
      lVar6 = param_1[5];
      lVar30 = uVar29 * 0x40;
      uVar9 = *param_2;
      uVar10 = param_2[1];
      uVar11 = param_2[2];
      uVar12 = param_2[3];
      uVar34 = *(undefined8 *)(param_2 + 4);
      uVar25 = *(undefined8 *)(param_2 + 6);
      uVar26 = *(undefined8 *)(param_2 + 8);
      uVar27 = *(undefined8 *)(param_2 + 10);
      uVar28 = *(undefined8 *)(param_2 + 0xe);
      puVar3 = (undefined8 *)(lVar6 + 0x30 + lVar30);
      *puVar3 = *(undefined8 *)(param_2 + 0xc);
      puVar3[1] = uVar28;
      puVar3 = (undefined8 *)(lVar6 + 0x20 + lVar30);
      *puVar3 = uVar26;
      puVar3[1] = uVar27;
      puVar3 = (undefined8 *)(lVar6 + 0x10 + lVar30);
      *puVar3 = uVar34;
      puVar3[1] = uVar25;
      puVar1 = (undefined4 *)(lVar6 + lVar30);
      *puVar1 = uVar9;
      puVar1[1] = uVar10;
      puVar1[2] = uVar11;
      puVar1[3] = uVar12;
      uVar31 = (ulonglong)(uVar5 >> 6);
      if (uVar31 < (ulonglong)param_1[9]) {
        LOCK();
        puVar2 = (ulonglong *)(param_1[8] + uVar31 * 8);
        *puVar2 = *puVar2 | 1L << ((byte)uVar5 & 0x3f);
        UNLOCK();
        uVar31 = 0;
        goto LAB_141a3c27b;
      }
      goto LAB_141a3c2c6;
    }
    plVar32 = (longlong *)param_1[7];
    LOCK();
    lVar30 = *plVar32;
    if (lVar30 == 0) {
      *plVar32 = 8;
    }
    UNLOCK();
    if (lVar30 != 0) {
      FUN_1428cb5f0(plVar32,lVar6,1000000000);
    }
    FUN_140c81c70(plVar32);
    uVar31 = (ulonglong)plVar32 | 1;
    FUN_140c81c70(uVar31);
    lVar6 = plVar32[3];
    if (lVar6 == 0) {
      FUN_140c81e10(plVar32);
      FUN_140c81e10();
      LOCK();
      lVar6 = *plVar32;
      if (lVar6 == 8) {
        *plVar32 = 0;
      }
      UNLOCK();
      uVar31 = CONCAT71((int7)((ulonglong)param_1 >> 8),1);
      if (lVar6 == 8) goto LAB_141a3c27b;
LAB_141a3c2a2:
      FUN_1428cbf60(plVar32,0);
LAB_141a3c27b:
      return uVar31 & 0xffffffff;
    }
    plVar32[3] = lVar6 + -1;
    uVar5 = *(uint *)(plVar32[2] + -4 + lVar6 * 4);
    unaff_R14 = (ulonglong)uVar5;
    plVar7 = (longlong *)param_1[4];
    lVar6 = plVar7[2];
    if (lVar6 == *plVar7) {
      FUN_140b5f870(plVar7,&UNK_142ebb508);
    }
    *(uint *)(plVar7[1] + lVar6 * 4) = uVar5;
    plVar7[2] = lVar6 + 1;
    if (unaff_R14 < (ulonglong)param_1[1]) {
      *(undefined1 *)(*param_1 + unaff_R14) = 1;
      if ((ulonglong)param_1[6] <= unaff_R14 << 5) {
        FUN_1428d9518(unaff_R14 << 5,param_1[6],&UNK_142ebb538);
        goto LAB_141a3c314;
      }
      lVar6 = param_1[5];
      lVar30 = unaff_R14 * 0x800;
      uVar9 = *param_2;
      uVar10 = param_2[1];
      uVar11 = param_2[2];
      uVar12 = param_2[3];
      uVar13 = param_2[4];
      uVar14 = param_2[5];
      uVar15 = param_2[6];
      uVar16 = param_2[7];
      uVar17 = param_2[8];
      uVar18 = param_2[9];
      uVar19 = param_2[10];
      uVar20 = param_2[0xb];
      uVar21 = param_2[0xc];
      uVar22 = param_2[0xd];
      uVar23 = param_2[0xe];
      uVar24 = param_2[0xf];
      puVar1 = (undefined4 *)(lVar6 + 0x30 + lVar30);
      *puVar1 = uVar21;
      puVar1[1] = uVar22;
      puVar1[2] = uVar23;
      puVar1[3] = uVar24;
      puVar1 = (undefined4 *)(lVar6 + 0x20 + lVar30);
      *puVar1 = uVar17;
      puVar1[1] = uVar18;
      puVar1[2] = uVar19;
      puVar1[3] = uVar20;
      puVar1 = (undefined4 *)(lVar6 + 0x10 + lVar30);
      *puVar1 = uVar13;
      puVar1[1] = uVar14;
      puVar1[2] = uVar15;
      puVar1[3] = uVar16;
      puVar1 = (undefined4 *)(lVar6 + lVar30);
      *puVar1 = uVar9;
      puVar1[1] = uVar10;
      puVar1[2] = uVar11;
      puVar1[3] = uVar12;
      uVar29 = (ulonglong)(uVar5 >> 6);
      if ((ulonglong)param_1[3] <= uVar29) {
        FUN_1428d9518(uVar29,param_1[3],&UNK_142ebb3d0,uVar21,plVar32,uVar34);
        goto LAB_141a3c314;
      }
      uVar33 = 1L << ((byte)uVar5 & 0x3f);
      LOCK();
      puVar2 = (ulonglong *)(param_1[2] + uVar29 * 8);
      *puVar2 = *puVar2 | uVar33;
      UNLOCK();
      if ((ulonglong)param_1[9] <= uVar29) {
        FUN_1428d9518(uVar29,param_1[9],&UNK_142ebb3d0,uVar21,plVar32,uVar34);
        goto LAB_141a3c314;
      }
      LOCK();
      puVar2 = (ulonglong *)(param_1[8] + uVar29 * 8);
      *puVar2 = *puVar2 | uVar33;
      UNLOCK();
      *(uint *)(param_1 + 10) = uVar5;
      FUN_140c81e10(plVar32);
      FUN_140c81e10(uVar31);
      uVar31 = 0;
      LOCK();
      lVar6 = *plVar32;
      if (lVar6 == 8) {
        *plVar32 = 0;
      }
      UNLOCK();
      if (lVar6 == 8) goto LAB_141a3c27b;
      uVar31 = 0;
      goto LAB_141a3c2a2;
    }
  }
  else {
    uVar29 = FUN_1428d9518(uVar31,param_1[1],&UNK_142ebb4f0);
LAB_141a3c2b7:
    uVar31 = FUN_1428d9518(uVar29);
LAB_141a3c2c6:
    FUN_1428d9518(uVar31);
  }
  FUN_1428d9518(unaff_R14);
LAB_141a3c314:
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

