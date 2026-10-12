
longlong * FUN_140be9040(longlong *param_1,longlong *param_2)

{
  undefined1 auVar1 [16];
  int iVar2;
  longlong lVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  longlong *plVar12;
  longlong lVar13;
  int *piVar14;
  longlong lVar15;
  longlong lVar16;
  int *piVar17;
  longlong lVar18;
  longlong lStack_a8;
  longlong lStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined1 (*pauStack_78) [16];
  undefined1 (*pauStack_70) [16];
  undefined1 *puStack_68;
  ushort uStack_60;
  longlong lStack_58;
  longlong *plStack_50;
  longlong *plStack_48;
  longlong *plStack_40;
  longlong *plStack_38;
  longlong *plStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  plStack_38 = param_2 + 9;
  pauStack_78 = (undefined1 (*) [16])param_2[0x1b];
  puStack_68 = *pauStack_78 + param_2[0x1c] + 1;
  auVar1 = *pauStack_78;
  uStack_60 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
  lStack_58 = param_2[0x1e];
  plStack_48 = param_2 + 6;
  pauStack_70 = pauStack_78 + 1;
  plStack_50 = param_2;
  plStack_40 = plStack_48;
  plStack_30 = param_2;
  FUN_1418c2b60(&lStack_a8,&pauStack_78);
  plVar12 = plStack_30;
  lVar3 = plStack_30[6];
  piVar4 = (int *)plStack_30[7];
  lVar13 = plStack_30[8];
  if (lVar13 == 0) {
    lVar13 = 0;
  }
  else {
    lVar15 = 1;
    lVar16 = 1;
    iVar2 = *piVar4;
    piVar17 = piVar4;
    while (iVar2 != 5) {
      piVar17 = piVar17 + 0x10;
      if (lVar13 == lVar16) {
        lVar15 = 0;
        goto code_r0x000140be918a;
      }
      lVar16 = lVar16 + 1;
      iVar2 = *piVar17;
    }
    lVar18 = lVar13 - lVar16;
    if (lVar18 != 0) {
      piVar17 = piVar4 + lVar16 * 0x10;
      lVar15 = 1;
      do {
        while (*piVar17 != 5) {
          piVar14 = piVar17 + lVar15 * -0x10;
          uVar5 = *(undefined8 *)piVar17;
          uVar6 = *(undefined8 *)(piVar17 + 2);
          uVar7 = *(undefined8 *)(piVar17 + 4);
          uVar8 = *(undefined8 *)(piVar17 + 6);
          uVar9 = *(undefined8 *)(piVar17 + 8);
          uVar10 = *(undefined8 *)(piVar17 + 10);
          uVar11 = *(undefined8 *)(piVar17 + 0xe);
          *(undefined8 *)(piVar17 + lVar15 * -0x10 + 0xc) = *(undefined8 *)(piVar17 + 0xc);
          *(undefined8 *)(piVar17 + lVar15 * -0x10 + 0xc + 2) = uVar11;
          *(undefined8 *)(piVar14 + 8) = uVar9;
          *(undefined8 *)(piVar14 + 10) = uVar10;
          *(undefined8 *)(piVar14 + 4) = uVar7;
          *(undefined8 *)(piVar14 + 6) = uVar8;
          *(undefined8 *)piVar14 = uVar5;
          *(undefined8 *)(piVar14 + 2) = uVar6;
          piVar17 = piVar17 + 0x10;
          lVar18 = lVar18 + -1;
          if (lVar18 == 0) goto code_r0x000140be918a;
        }
        lVar15 = lVar15 + 1;
        piVar17 = piVar17 + 0x10;
        lVar18 = lVar18 + -1;
      } while (lVar18 != 0);
    }
code_r0x000140be918a:
    lVar13 = lVar13 - lVar15;
  }
  *param_1 = lVar3;
  param_1[1] = (longlong)piVar4;
  param_1[2] = lVar13;
  param_1[3] = lStack_a8;
  param_1[4] = lStack_a0;
  *(undefined4 *)(param_1 + 5) = uStack_98;
  *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_94;
  *(undefined4 *)(param_1 + 6) = uStack_90;
  *(undefined4 *)((longlong)param_1 + 0x34) = uStack_8c;
  *(undefined4 *)(param_1 + 7) = uStack_88;
  *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_84;
  *(undefined4 *)(param_1 + 8) = uStack_80;
  *(undefined4 *)((longlong)param_1 + 0x44) = uStack_7c;
  if (*plStack_30 != 0) {
    func_0x000140613c20(plStack_30[1],*plStack_30 << 2,2);
  }
  lVar3 = plVar12[3];
  if (lVar3 != 0) {
    func_0x000140613c20(plVar12[4],lVar3,1);
  }
  FUN_1405f6670(plStack_38);
  return param_1;
}

