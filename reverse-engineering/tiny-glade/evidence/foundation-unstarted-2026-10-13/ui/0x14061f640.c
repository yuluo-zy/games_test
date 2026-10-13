
undefined8 *
FUN_14061f640(undefined8 *param_1,undefined8 *param_2,ulonglong *param_3,ulonglong *param_4,
             undefined8 param_5,undefined8 param_6,ulonglong *param_7)

{
  longlong lVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 *puVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  char cVar21;
  byte bVar22;
  undefined8 uVar23;
  ulonglong *puVar24;
  longlong lVar25;
  undefined8 uVar26;
  longlong lVar27;
  char cVar28;
  longlong lVar29;
  longlong lVar30;
  char cVar31;
  longlong lStack_168;
  longlong lStack_160;
  undefined8 uStack_158;
  longlong lStack_150;
  ulonglong uStack_148;
  ulonglong uStack_140;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  ulonglong uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  ulonglong uStack_f8;
  ulonglong uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined1 uStack_a8;
  ulonglong uStack_a0;
  ulonglong *puStack_98;
  ulonglong *puStack_90;
  undefined8 *puStack_88;
  undefined8 *puStack_80;
  longlong lStack_78;
  undefined8 *puStack_70;
  longlong lStack_68;
  undefined4 uStack_58;
  undefined1 uStack_51;
  undefined1 uStack_50;
  char cStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  char cStack_4c;
  byte bStack_4b;
  undefined1 uStack_4a;
  char cStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_a0 = *param_4;
  puStack_98 = param_4;
  puStack_90 = param_3;
  puStack_88 = param_2;
  if (uStack_a0 == 0x8000000000000004) {
    uStack_148 = 0x8000000000000005;
    uVar26 = CONCAT71((int7)((ulonglong)param_2 >> 8),1);
  }
  else {
    uStack_108 = param_4[8];
    uStack_148 = *param_4;
    uStack_140 = param_4[1];
    uStack_138 = (undefined4)param_4[2];
    uStack_134 = *(undefined4 *)((longlong)param_4 + 0x14);
    uStack_130 = (undefined4)param_4[3];
    uStack_12c = *(undefined4 *)((longlong)param_4 + 0x1c);
    uStack_128 = (undefined4)param_4[4];
    uStack_124 = *(undefined4 *)((longlong)param_4 + 0x24);
    uStack_120 = (undefined4)param_4[5];
    uStack_11c = *(undefined4 *)((longlong)param_4 + 0x2c);
    uStack_118 = (undefined4)param_4[6];
    uStack_114 = *(undefined4 *)((longlong)param_4 + 0x34);
    uStack_110 = (undefined4)param_4[7];
    uStack_10c = *(undefined4 *)((longlong)param_4 + 0x3c);
    bStack_4b = 1;
    cStack_4c = '\0';
    uStack_4d = 1;
    uStack_4e = 1;
    func_0x000140dbaea0(&uStack_f8);
    uStack_148 = uStack_f8;
    uStack_140 = uStack_f0;
    uStack_138 = (undefined4)uStack_e8;
    uStack_134 = uStack_e8._4_4_;
    uStack_130 = (undefined4)uStack_e0;
    uStack_12c = uStack_e0._4_4_;
    uStack_128 = (undefined4)uStack_d8;
    uStack_124 = uStack_d8._4_4_;
    uStack_120 = (undefined4)uStack_d0;
    uStack_11c = uStack_d0._4_4_;
    uStack_118 = (undefined4)uStack_c8;
    uStack_114 = uStack_c8._4_4_;
    uStack_110 = (undefined4)uStack_c0;
    uStack_10c = uStack_c0._4_4_;
    uStack_108 = uStack_b8;
    uStack_100 = (undefined4)uStack_b0;
    uStack_fc = uStack_b0._4_4_;
    uVar26 = 0;
  }
  uStack_58 = (undefined4)uVar26;
  puStack_70 = param_1;
  if (SBORROW8(0,*puStack_90)) {
LAB_14061f742:
    lStack_68 = 0;
    lStack_160 = 1;
    uVar26 = 0;
  }
  else {
    uStack_f0 = puStack_90[1];
    uStack_e8 = puStack_90[2];
    uStack_e0 = puStack_90[3];
    uStack_f8 = *puStack_90;
    uVar23 = FUN_140d76ed0(&uStack_f8);
    FUN_140d59140(&lStack_168,uVar23,uVar26);
    if (uStack_f8 != 0) {
      func_0x000140613c20(uStack_f0,uStack_f8,1);
    }
    if (SBORROW8(0,lStack_168)) {
      if (((uint)lStack_160 & 3) == 1) {
        lStack_68 = lStack_160 + -1;
        lStack_78 = *(undefined8 *)(lStack_160 + -1);
        puStack_80 = *(undefined8 **)(lStack_160 + 7);
        if ((code *)*puStack_80 != (code *)0x0) {
          (*(code *)*puStack_80)(lStack_78);
        }
        if (puStack_80[1] != 0) {
          func_0x000140613c20(lStack_78,puStack_80[1],puStack_80[2]);
        }
        func_0x000140613c20(lStack_68,0x18,8);
      }
      goto LAB_14061f742;
    }
    lStack_68 = lStack_168;
    uVar26 = uStack_158;
  }
  cStack_49 = '\x01';
  uStack_4a = 1;
  lStack_78 = lStack_160;
  cVar21 = FUN_14061edf0(lStack_160,uVar26,&UNK_142a0b2f8,0xb);
  puStack_80._0_1_ = 1;
  if (cVar21 == '\0') {
    cStack_49 = '\x01';
    uStack_4a = 1;
    puStack_80._0_1_ = FUN_14061edf0(lStack_78,uVar26,&UNK_142a0b303,0x1c);
  }
  bVar22 = *(byte *)((longlong)puStack_88 + 0x3b);
  lVar30 = puStack_88[1];
  lStack_150 = puStack_88[2];
  lVar1 = lVar30 + lStack_150 * 0x18;
  lVar27 = lVar30;
  do {
    lVar25 = lVar27;
    lVar29 = lVar30;
    if (lVar25 == lVar1) break;
    uVar26 = *(undefined8 *)(lVar25 + 8);
    uVar23 = *(undefined8 *)(lVar25 + 0x10);
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b31f,0xd);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b32c,0xd);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b339,0xf);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b348,0xe);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b356,0x10);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b366,0xb);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b371,0xc);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b37d,0xc);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b389,0xc);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b395,0x12);
    if (cVar21 != '\0') break;
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(uVar26,uVar23,&UNK_142a0b3a7,0x13);
    lVar27 = lVar25 + 0x18;
  } while (cVar21 == '\0');
  do {
    if (lVar29 == lVar1) {
      if ((byte)((bVar22 ^ 1) & bVar22 != 2 & (lVar25 != lVar1 | (byte)puStack_80)) != 0) {
        uStack_f8 = 0x8000000000000005;
        goto LAB_14061fb3c;
      }
      lVar29 = 8;
      lVar27 = lVar30;
      goto LAB_14061fd00;
    }
    cStack_49 = '\x01';
    uStack_4a = 1;
    cVar21 = FUN_14061edf0(*(undefined8 *)(lVar29 + 8),*(undefined8 *)(lVar29 + 0x10),&UNK_142a0b3ba
                           ,0xe);
    lVar29 = lVar29 + 0x18;
  } while (cVar21 == '\0');
  uStack_f8 = 0x8000000000000008;
LAB_14061fb3c:
  cVar21 = '\x01';
  if (*param_7 == 0x800000000000000d) goto LAB_14061fb67;
  bVar22 = 0;
  puVar24 = param_7;
  goto LAB_14061fb91;
  while( true ) {
    cStack_49 = '\x01';
    uStack_4a = 1;
    bVar22 = FUN_14061edf0(*(undefined8 *)(lVar27 + 8),*(undefined8 *)(lVar27 + 0x10),&UNK_142a0b3c8
                           ,0xe);
    lVar27 = lVar27 + 0x18;
    lVar29 = lVar29 + -1;
    if (lVar29 != 0) {
      bVar22 = 2 - bVar22;
    }
    if (bVar22 != 2) break;
LAB_14061fd00:
    if (lVar27 == lVar1) goto LAB_14061fd54;
  }
  if ((bVar22 & 1) != 0) {
    uStack_f8 = 0x800000000000000c;
    goto LAB_14061fb3c;
  }
LAB_14061fd54:
  lVar29 = 8;
  lVar27 = lVar30;
  do {
    if (lVar27 == lVar1) goto LAB_14061fdc4;
    cStack_49 = '\x01';
    uStack_4a = 1;
    bVar22 = FUN_14061edf0(*(undefined8 *)(lVar27 + 8),*(undefined8 *)(lVar27 + 0x10),&UNK_142a0b3d6
                           ,0x13);
    lVar27 = lVar27 + 0x18;
    lVar29 = lVar29 + -1;
    if (lVar29 != 0) {
      bVar22 = 2 - bVar22;
    }
  } while (bVar22 == 2);
  if ((bVar22 & 1) == 0) {
LAB_14061fdc4:
    if (lStack_150 != 0) {
      cStack_49 = '\x01';
      uStack_4a = 1;
      cVar21 = FUN_14061edf0(*(undefined8 *)(lVar30 + 8),*(undefined8 *)(lVar30 + 0x10),
                             &UNK_142a0b3e9,0xc);
      if (cVar21 != '\0') {
        uStack_f8 = 0x8000000000000006;
        goto LAB_14061fea2;
      }
    }
    if (uStack_148 == 0x8000000000000005) {
      do {
        if (lVar30 == lVar1) {
          if (lVar25 != lVar1) {
            uStack_f8 = 0x8000000000000007;
            goto LAB_14061fea2;
          }
          if ((byte)puStack_80 == 0) {
            uStack_f8 = 0x800000000000000d;
          }
          else {
            uStack_f8 = 0x800000000000000b;
          }
          cVar28 = '\x01';
          cVar21 = '\x01';
          if (*param_7 == 0x800000000000000d) goto LAB_14061fb67;
          goto LAB_14061fed1;
        }
        cStack_49 = '\x01';
        uStack_4a = 1;
        cVar21 = FUN_14061edf0(*(undefined8 *)(lVar30 + 8),*(undefined8 *)(lVar30 + 0x10),
                               &UNK_142a0b3f5,0x12);
        lVar30 = lVar30 + 0x18;
      } while (cVar21 == '\0');
      uStack_f8 = 0x800000000000000a;
      goto LAB_14061fea2;
    }
    uStack_b8 = uStack_108;
    uStack_b0 = CONCAT44(uStack_fc,uStack_100);
    uStack_c8 = CONCAT44(uStack_114,uStack_118);
    uStack_c0 = CONCAT44(uStack_10c,uStack_110);
    uStack_d8 = CONCAT44(uStack_124,uStack_128);
    uStack_d0 = CONCAT44(uStack_11c,uStack_120);
    uStack_e8 = CONCAT44(uStack_134,uStack_138);
    uStack_e0 = CONCAT44(uStack_12c,uStack_130);
    uStack_f8 = uStack_148;
    uStack_f0 = uStack_140;
    uStack_a8 = 0;
    cVar28 = '\0';
  }
  else {
    uStack_f8 = 0x8000000000000009;
LAB_14061fea2:
    cVar28 = '\x01';
  }
  cVar21 = cVar28;
  if (*param_7 != 0x800000000000000d) {
LAB_14061fed1:
    puVar13 = puStack_70;
    uVar26 = *puStack_88;
    uVar23 = puStack_88[1];
    uVar8 = puStack_88[2];
    uVar9 = puStack_88[3];
    uVar10 = puStack_88[4];
    uVar11 = puStack_88[5];
    uVar12 = puStack_88[7];
    puStack_70[6] = puStack_88[6];
    puStack_70[7] = uVar12;
    puStack_70[4] = uVar10;
    puStack_70[5] = uVar11;
    puStack_70[2] = uVar8;
    puStack_70[3] = uVar9;
    *puStack_70 = uVar26;
    puStack_70[1] = uVar23;
    uVar14 = param_7[1];
    uVar15 = param_7[2];
    uVar2 = *(undefined4 *)((longlong)param_7 + 0x14);
    uVar16 = param_7[3];
    uVar3 = *(undefined4 *)((longlong)param_7 + 0x1c);
    uVar17 = param_7[4];
    uVar4 = *(undefined4 *)((longlong)param_7 + 0x24);
    uVar18 = param_7[5];
    uVar5 = *(undefined4 *)((longlong)param_7 + 0x2c);
    uVar19 = param_7[6];
    uVar6 = *(undefined4 *)((longlong)param_7 + 0x34);
    uVar20 = param_7[7];
    uVar7 = *(undefined4 *)((longlong)param_7 + 0x3c);
    puStack_70[8] = *param_7;
    puStack_70[9] = uVar14;
    *(int *)(puStack_70 + 10) = (int)uVar15;
    *(undefined4 *)((longlong)puStack_70 + 0x54) = uVar2;
    *(int *)(puStack_70 + 0xb) = (int)uVar16;
    *(undefined4 *)((longlong)puStack_70 + 0x5c) = uVar3;
    *(int *)(puStack_70 + 0xc) = (int)uVar17;
    *(undefined4 *)((longlong)puStack_70 + 100) = uVar4;
    *(int *)(puStack_70 + 0xd) = (int)uVar18;
    *(undefined4 *)((longlong)puStack_70 + 0x6c) = uVar5;
    *(int *)(puStack_70 + 0xe) = (int)uVar19;
    *(undefined4 *)((longlong)puStack_70 + 0x74) = uVar6;
    *(int *)(puStack_70 + 0xf) = (int)uVar20;
    *(undefined4 *)((longlong)puStack_70 + 0x7c) = uVar7;
    uVar2 = *(undefined4 *)((longlong)param_7 + 0x44);
    uVar14 = param_7[9];
    uVar3 = *(undefined4 *)((longlong)param_7 + 0x4c);
    *(int *)(puStack_70 + 0x10) = (int)param_7[8];
    *(undefined4 *)((longlong)puStack_70 + 0x84) = uVar2;
    *(int *)(puStack_70 + 0x11) = (int)uVar14;
    *(undefined4 *)((longlong)puStack_70 + 0x8c) = uVar3;
    puStack_70[0x12] = param_7[10];
    puStack_70[0x13] = param_5;
    puStack_70[0x14] = param_6;
    *(undefined1 *)(puStack_70 + 0x17) = 2;
    *(undefined4 *)(puStack_70 + 0x16) = 0x3b9aca01;
    if ((uStack_f8 < 0x8000000000000005) &&
       (2 < uStack_f8 + 0x7ffffffffffffff3 || uStack_f8 + 0x7ffffffffffffff3 == 1)) {
      uStack_4a = 0;
      cStack_49 = cVar28;
      FUN_140dba290(&uStack_f8);
    }
    cVar31 = (char)uStack_58;
    bVar22 = 0;
    cVar21 = cVar28;
    goto joined_r0x00014061ffc1;
  }
LAB_14061fb67:
  bVar22 = 1;
  puVar24 = &uStack_f8;
LAB_14061fb91:
  cVar31 = (char)uStack_58;
  uVar26 = *puStack_88;
  uVar23 = puStack_88[1];
  uVar8 = puStack_88[2];
  uVar9 = puStack_88[3];
  uVar10 = puStack_88[4];
  uVar11 = puStack_88[5];
  uVar12 = puStack_88[7];
  puStack_70[6] = puStack_88[6];
  puStack_70[7] = uVar12;
  puStack_70[4] = uVar10;
  puStack_70[5] = uVar11;
  puStack_70[2] = uVar8;
  puStack_70[3] = uVar9;
  *puStack_70 = uVar26;
  puStack_70[1] = uVar23;
  uVar14 = puVar24[1];
  uVar15 = puVar24[2];
  uVar16 = puVar24[3];
  uVar17 = puVar24[4];
  uVar18 = puVar24[5];
  uVar19 = puVar24[6];
  uVar20 = puVar24[7];
  puStack_70[8] = *puVar24;
  puStack_70[9] = uVar14;
  puStack_70[10] = uVar15;
  puStack_70[0xb] = uVar16;
  puStack_70[0xc] = uVar17;
  puStack_70[0xd] = uVar18;
  puStack_70[0xe] = uVar19;
  puStack_70[0xf] = uVar20;
  uVar2 = *(undefined4 *)((longlong)puVar24 + 0x44);
  uVar14 = puVar24[9];
  uVar3 = *(undefined4 *)((longlong)puVar24 + 0x4c);
  *(int *)(puStack_70 + 0x10) = (int)puVar24[8];
  *(undefined4 *)((longlong)puStack_70 + 0x84) = uVar2;
  *(int *)(puStack_70 + 0x11) = (int)uVar14;
  *(undefined4 *)((longlong)puStack_70 + 0x8c) = uVar3;
  puStack_70[0x12] = puVar24[10];
  puStack_70[0x13] = param_5;
  puStack_70[0x14] = param_6;
  *(undefined1 *)(puStack_70 + 0x17) = 2;
  *(undefined4 *)(puStack_70 + 0x16) = 0x3b9aca01;
  puVar13 = puStack_70;
joined_r0x00014061ffc1:
  if (lStack_68 != 0) {
    func_0x000140613c20(lStack_78,lStack_68,1);
  }
  if ((uStack_148 != 0x8000000000000005) && (cVar21 != '\0')) {
    uStack_4d = 0;
    uStack_4e = 0;
    cStack_4c = cVar31;
    bStack_4b = bVar22;
    FUN_140dba290(&uStack_148);
  }
  if ((*param_7 != 0x800000000000000d & *param_7 < 0x8000000000000005 & bVar22) != 0) {
    uStack_50 = 0;
    uStack_51 = 0;
    cStack_4f = cVar31;
    FUN_140dba290();
  }
  if ((uStack_a0 != 0x8000000000000004) && (cVar31 != '\0')) {
    FUN_14061ec70(puStack_98);
  }
  return puVar13;
}

