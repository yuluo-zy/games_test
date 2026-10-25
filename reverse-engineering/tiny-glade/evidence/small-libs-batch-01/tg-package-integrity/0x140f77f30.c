
undefined8 * FUN_140f77f30(undefined8 *param_1,longlong *param_2)

{
  undefined8 uVar1;
  char *pcVar2;
  longlong lVar3;
  char *pcVar4;
  longlong lVar5;
  code *pcVar6;
  longlong lVar7;
  char cVar8;
  int iVar9;
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auStack_f8 [32];
  longlong lStack_d8;
  undefined8 *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_b8;
  undefined1 uStack_b0;
  longlong *plStack_a8;
  undefined *puStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  longlong lStack_88;
  longlong lStack_80;
  char *pcStack_78;
  char *pcStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined1 uStack_51;
  undefined8 *puStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  lStack_60 = *param_2;
  pcVar2 = (char *)param_2[1];
  lVar3 = param_2[2];
  lStack_68 = param_2[3];
  pcVar4 = (char *)param_2[4];
  lVar5 = param_2[5];
  if (lVar3 == 0x18) {
    uVar1 = *(undefined8 *)(pcVar2 + 0x10);
    auVar11[0] = -((char)uVar1 == DAT_142bab510);
    auVar11[1] = -((char)((ulonglong)uVar1 >> 8) == UNK_142bab511);
    auVar11[2] = -((char)((ulonglong)uVar1 >> 0x10) == UNK_142bab512);
    auVar11[3] = -((char)((ulonglong)uVar1 >> 0x18) == UNK_142bab513);
    auVar11[4] = -((char)((ulonglong)uVar1 >> 0x20) == UNK_142bab514);
    auVar11[5] = -((char)((ulonglong)uVar1 >> 0x28) == UNK_142bab515);
    auVar11[6] = -((char)((ulonglong)uVar1 >> 0x30) == UNK_142bab516);
    auVar11[7] = -((char)((ulonglong)uVar1 >> 0x38) == UNK_142bab517);
    auVar11[8] = -(UNK_142bab518 == '\0');
    auVar11[9] = -(UNK_142bab519 == '\0');
    auVar11[10] = -(UNK_142bab51a == '\0');
    auVar11[0xb] = -(UNK_142bab51b == '\0');
    auVar11[0xc] = -(UNK_142bab51c == '\0');
    auVar11[0xd] = -(UNK_142bab51d == '\0');
    auVar11[0xe] = -(UNK_142bab51e == '\0');
    auVar11[0xf] = -(UNK_142bab51f == '\0');
    auVar10[0] = -(*pcVar2 == DAT_142bab520);
    auVar10[1] = -(pcVar2[1] == UNK_142bab521);
    auVar10[2] = -(pcVar2[2] == UNK_142bab522);
    auVar10[3] = -(pcVar2[3] == UNK_142bab523);
    auVar10[4] = -(pcVar2[4] == UNK_142bab524);
    auVar10[5] = -(pcVar2[5] == UNK_142bab525);
    auVar10[6] = -(pcVar2[6] == UNK_142bab526);
    auVar10[7] = -(pcVar2[7] == UNK_142bab527);
    auVar10[8] = -(pcVar2[8] == UNK_142bab528);
    auVar10[9] = -(pcVar2[9] == UNK_142bab529);
    auVar10[10] = -(pcVar2[10] == UNK_142bab52a);
    auVar10[0xb] = -(pcVar2[0xb] == UNK_142bab52b);
    auVar10[0xc] = -(pcVar2[0xc] == UNK_142bab52c);
    auVar10[0xd] = -(pcVar2[0xd] == UNK_142bab52d);
    auVar10[0xe] = -(pcVar2[0xe] == UNK_142bab52e);
    auVar10[0xf] = -(pcVar2[0xf] == UNK_142bab52f);
    auVar10 = auVar10 & auVar11;
    if ((ushort)((ushort)(SUB161(auVar10 >> 7,0) & 1) | (ushort)(SUB161(auVar10 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar10 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar10 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar10 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar10 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar10 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar10 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar10 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar10 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar10 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar10 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar10 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar10 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar10 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar10[0xf] >> 7) << 0xf) == 0xffff) {
      *param_1 = 0;
      lVar3 = lStack_60;
      if (lStack_68 != 0) {
        func_0x000140613c20(pcVar4,lStack_68,1);
        lVar3 = lStack_60;
      }
      goto joined_r0x000140f78056;
    }
  }
  pcStack_78 = pcVar4;
  pcStack_70 = pcVar2;
  FUN_140d58eb0(&lStack_d8,pcVar2,lVar3);
  if (SBORROW8(0,lStack_d8)) {
    if (((uint)puStack_d0 & 3) == 1) {
      lStack_80 = (longlong)puStack_d0 + -1;
      lStack_48 = *(longlong *)((longlong)puStack_d0 + -1);
      puStack_50 = *(undefined8 **)((longlong)puStack_d0 + 7);
      if ((code *)*puStack_50 != (code *)0x0) {
        (*(code *)*puStack_50)(lStack_48);
      }
      if (puStack_50[1] != 0) {
        func_0x000140613c20(lStack_48,puStack_50[1],puStack_50[2]);
      }
      func_0x000140613c20(lStack_80,0x18,8);
    }
    param_1[1] = lStack_60;
    param_1[2] = pcStack_70;
    param_1[3] = lVar3;
    *param_1 = 1;
    pcVar2 = pcStack_78;
    lVar3 = lStack_68;
  }
  else {
    lStack_48 = lStack_d8;
    puStack_50 = puStack_d0;
    FUN_140db6090(auStack_f8,puStack_d0,uStack_c8);
    lStack_98 = 0;
    uStack_90 = 1;
    lStack_88 = 0;
    lStack_d8 = 0;
    uStack_c8 = 0;
    uStack_b8 = 0x20;
    uStack_b0 = 3;
    plStack_a8 = &lStack_98;
    puStack_a0 = &UNK_142bab6f8;
    cVar8 = FUN_140db5320(auStack_f8,&lStack_d8);
    pcVar4 = pcStack_78;
    uVar1 = uStack_90;
    lVar7 = lStack_98;
    if (cVar8 != '\0') {
      FUN_1428d9760(&UNK_142bab728,0x37,&uStack_51,&UNK_142bab7e8,&UNK_142bab7b0);
                    /* WARNING: Does not return */
      pcVar6 = (code *)invalidInstructionException();
      (*pcVar6)();
    }
    if ((lVar5 == lStack_88) &&
       (iVar9 = func_0x000142923bf0(pcStack_78,uStack_90,lVar5), lVar5 = lStack_68, iVar9 == 0)) {
      *param_1 = 0;
      if (lVar7 != 0) {
        func_0x000140613c20(uVar1,lVar7,1);
      }
      if (lStack_48 != 0) {
        func_0x000140613c20(puStack_50,lStack_48,1);
      }
      pcVar2 = pcStack_70;
      lVar3 = lStack_60;
      if (lVar5 != 0) {
        func_0x000140613c20(pcVar4,lVar5,1);
        lVar3 = lStack_60;
      }
    }
    else {
      param_1[1] = lStack_60;
      param_1[2] = pcStack_70;
      param_1[3] = lVar3;
      *param_1 = 2;
      if (lVar7 != 0) {
        func_0x000140613c20(uVar1,lVar7,1);
      }
      lVar3 = lStack_68;
      pcVar2 = pcVar4;
      if (lStack_48 != 0) {
        func_0x000140613c20(puStack_50,lStack_48,1);
      }
    }
  }
joined_r0x000140f78056:
  if (lVar3 != 0) {
    func_0x000140613c20(pcVar2,lVar3,1);
  }
  return param_1;
}

