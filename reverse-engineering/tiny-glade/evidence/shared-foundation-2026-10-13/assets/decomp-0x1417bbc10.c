
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000142ce1231) overlaps instruction at (ram,0x000142ce1230)
    */
/* WARNING: Removing unreachable block (ram,0x000142ce124d) */
/* WARNING: Removing unreachable block (ram,0x000142ce1252) */
/* WARNING: Removing unreachable block (ram,0x000142ce122f) */
/* WARNING: Removing unreachable block (ram,0x000142ce120e) */
/* WARNING: Removing unreachable block (ram,0x000142ce11cb) */
/* WARNING: Removing unreachable block (ram,0x000142ce11cf) */
/* WARNING: Removing unreachable block (ram,0x000142ce1232) */
/* WARNING: Removing unreachable block (ram,0x000142ce11d1) */
/* WARNING: Removing unreachable block (ram,0x000142ce11d6) */
/* WARNING: Removing unreachable block (ram,0x000142ce11e2) */
/* WARNING: Removing unreachable block (ram,0x000142ce124f) */
/* WARNING: Removing unreachable block (ram,0x000142ce1259) */
/* WARNING: Removing unreachable block (ram,0x000142ce1235) */
/* WARNING: Removing unreachable block (ram,0x0001417bbf97) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1417bbc10(longlong *param_1,undefined4 *param_2)

{
  byte *pbVar1;
  char cVar2;
  undefined1 auVar3 [16];
  code *pcVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  byte bVar9;
  undefined8 uVar10;
  longlong lVar11;
  byte *pbVar12;
  char cVar13;
  byte bVar16;
  undefined4 *puVar14;
  uint *puVar15;
  undefined4 uVar17;
  uint uVar18;
  undefined4 *unaff_RBX;
  undefined4 *puVar19;
  char *pcVar20;
  int iVar21;
  ulonglong uVar22;
  char *pcVar23;
  uint uVar24;
  int iVar25;
  bool bVar26;
  float fVar27;
  undefined4 uVar28;
  char *pcStack_248;
  undefined4 *puStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  char *pcStack_200;
  undefined4 *puStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  uint uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1a8;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined1 uStack_198;
  undefined1 uStack_197;
  undefined1 uStack_196;
  byte bStack_195;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  uint uStack_188;
  uint uStack_184;
  longlong *plStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  uint uStack_118;
  uint uStack_114;
  uint uStack_110;
  uint uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  char acStack_a8 [3];
  undefined1 uStack_a5;
  char cStack_a4;
  undefined1 auStack_a3 [99];
  
  auStack_a3._3_8_ = 0xfffffffffffffffe;
  puVar14 = param_2;
  uVar10 = FUN_140d76ed0(param_2);
  FUN_140ba3a10(&uStack_108,uVar10,puVar14);
  if (uStack_108 == (char *)0x2) {
    uStack_108 = (char *)0x8000000000000009;
  }
  else {
    uStack_128 = uStack_b8;
    uStack_138 = uStack_c8;
    uStack_134 = uStack_c4;
    uStack_130 = uStack_c0;
    uStack_12c = uStack_bc;
    uStack_148 = uStack_d8;
    uStack_144 = uStack_d4;
    uStack_140 = uStack_d0;
    uStack_13c = uStack_cc;
    uStack_158 = uStack_e8;
    uStack_150 = uStack_e0;
    uStack_14c = uStack_dc;
    uStack_168 = uStack_f8;
    uStack_160 = uStack_f0;
    uStack_178 = uStack_108;
    uStack_170 = uStack_100;
    FUN_1420e2200(&uStack_108,&uStack_178);
  }
  FUN_1420d7480(&pcStack_248,&uStack_108,param_2);
  if (pcStack_248 == (char *)0xa) {
    param_1[1] = (longlong)puStack_240;
    *param_1 = 0;
    return;
  }
  uStack_1f0 = uStack_238;
  uStack_1ec = uStack_234;
  uStack_1e8 = uStack_230;
  uStack_1e4 = uStack_22c;
  pcStack_200 = pcStack_248;
  puStack_1f8 = puStack_240;
  cVar13 = *(char *)(param_2 + 0x13);
  if (cVar13 == '\0') {
    auStack_a3[1] = 1;
    uVar10 = FUN_140bad8d0(&uStack_178,&pcStack_200);
    uStack_190 = (undefined4)uStack_168;
    uStack_18c = uStack_168._4_4_;
    uStack_188 = (uint)uStack_160;
    uStack_184 = uStack_160._4_4_;
    uStack_1a0 = (undefined4)uStack_178;
    uStack_19c = uStack_178._4_4_;
    uStack_198 = SUB81(uStack_170,0);
    uStack_197 = (undefined1)((ulonglong)uStack_170 >> 8);
    uStack_196 = (undefined1)((ulonglong)uStack_170 >> 0x10);
    bStack_195 = (byte)((ulonglong)uStack_170 >> 0x18);
    uStack_194 = uStack_170._4_4_;
    uStack_1a8 = 3;
    uStack_ac = (undefined4)CONCAT71((int7)((ulonglong)uVar10 >> 8),1);
  }
  else {
    uStack_168 = (undefined4 *)CONCAT44(uStack_234,uStack_238);
    uStack_160 = (undefined1 *)CONCAT44(uStack_22c,uStack_230);
    uStack_178 = pcStack_248;
    uStack_170 = puStack_240;
    auStack_a3[1] = 0;
    uStack_158 = CONCAT44(uStack_224,uStack_228);
    FUN_140bada60(&uStack_108,&uStack_178);
    puVar14 = uStack_100;
    uVar17 = _DAT_142a10e44;
    fVar8 = _DAT_142a10e40;
    fVar7 = _DAT_142a10e3c;
    fVar6 = _DAT_142a10e38;
    fVar5 = _DAT_142a10e30;
    auVar3 = ZEXT416((uint)uStack_f0) * (undefined1  [16])0x4 * ZEXT416(uStack_f0._4_4_);
    unaff_RBX = auVar3._0_8_;
    if (auVar3._8_8_ != 0) {
      FUN_1428d9310(&UNK_142fded60);
      goto LAB_1417bc50c;
    }
    if (uStack_f8 < unaff_RBX) {
      FUN_1428d9840(unaff_RBX,uStack_f8,&UNK_142fded78);
      goto LAB_1417bc50c;
    }
    if (unaff_RBX != (undefined4 *)0x0) {
      puVar19 = (undefined4 *)0x0;
      do {
        while (fVar27 = (float)puVar14[(longlong)puVar19], fVar7 < fVar27) {
          uVar28 = func_0x000142923f70((fVar27 + fVar8) / fVar5,uVar17);
          puVar14[(longlong)puVar19] = uVar28;
          fVar27 = (float)puVar14[(longlong)((longlong)puVar19 + 1)];
          if (fVar27 <= fVar7) goto LAB_1417bbe71;
LAB_1417bbea5:
          uVar28 = func_0x000142923f70((fVar27 + fVar8) / fVar5,uVar17);
          puVar14[(longlong)((longlong)puVar19 + 1)] = uVar28;
          fVar27 = (float)puVar14[(longlong)((longlong)puVar19 + 2)];
          if (fVar7 < fVar27) goto LAB_1417bbe20;
LAB_1417bbecc:
          puVar14[(longlong)((longlong)puVar19 + 2)] = fVar27 / fVar6;
          puVar19 = puVar19 + 1;
          if (unaff_RBX == puVar19) goto LAB_1417bbee4;
        }
        puVar14[(longlong)puVar19] = fVar27 / fVar6;
        fVar27 = (float)puVar14[(longlong)((longlong)puVar19 + 1)];
        if (fVar7 < fVar27) goto LAB_1417bbea5;
LAB_1417bbe71:
        puVar14[(longlong)((longlong)puVar19 + 1)] = fVar27 / fVar6;
        fVar27 = (float)puVar14[(longlong)((longlong)puVar19 + 2)];
        if (fVar27 <= fVar7) goto LAB_1417bbecc;
LAB_1417bbe20:
        uVar28 = func_0x000142923f70((fVar27 + fVar8) / fVar5,uVar17);
        puVar14[(longlong)((longlong)puVar19 + 2)] = uVar28;
        puVar19 = puVar19 + 1;
      } while (unaff_RBX != puVar19);
    }
LAB_1417bbee4:
    uStack_190 = (undefined4)uStack_f8;
    uStack_18c = uStack_f8._4_4_;
    uStack_188 = (uint)uStack_f0;
    uStack_184 = uStack_f0._4_4_;
    uStack_1a0 = (undefined4)uStack_108;
    uStack_19c = uStack_108._4_4_;
    uStack_198 = SUB81(uStack_100,0);
    uStack_197 = (undefined1)((ulonglong)uStack_100 >> 8);
    uStack_196 = (undefined1)((ulonglong)uStack_100 >> 0x10);
    bStack_195 = (byte)((ulonglong)uStack_100 >> 0x18);
    uStack_194 = uStack_100._4_4_;
    uStack_1a8 = 9;
    uStack_ac = 0;
  }
  cVar2 = *(char *)((longlong)param_2 + 0x4d);
  cStack_a4 = cVar2 != '\x02' && (3 < uStack_184 && 3 < uStack_188);
  if (cVar2 == '\0') {
    auStack_a3[0] = 2;
    if (3 >= uStack_184 || 3 >= uStack_188) goto LAB_1417bc025;
LAB_1417bbfbd:
    acStack_a8[0] = '\x04';
    acStack_a8[1] = '\x04';
    iVar21 = 4;
    if ((*(byte *)(param_2 + 0xe) & 1) != 0) goto LAB_1417bc03b;
    uVar24 = uStack_188 & 3;
    iVar21 = 4 - uVar24;
    if (uVar24 == 0) {
      iVar21 = 0;
    }
    uVar18 = uStack_184 & 3;
    iVar25 = 4 - uVar18;
    if (uVar18 == 0) {
      iVar25 = 0;
    }
    if (uVar18 == 0 && uVar24 == 0) goto LAB_1417bc096;
    uVar22 = CONCAT44(iVar25 + uStack_184,iVar21 + uStack_188);
LAB_1417bc072:
    uStack_a5 = 1;
    uVar17 = (undefined4)CONCAT71((int7)((ulonglong)unaff_RBX >> 8),1);
    FUN_140badad0(&uStack_1d8,&uStack_1a8,uVar22,uVar22 >> 0x20);
  }
  else {
    if (cVar2 == '\x02') {
      auStack_a3[0] = 3;
    }
    else {
      auStack_a3[0] = 1;
      if ((bool)cStack_a4) goto LAB_1417bbfbd;
    }
LAB_1417bc025:
    acStack_a8[0] = '\x01';
    acStack_a8[1] = '\x01';
    iVar21 = 1;
    if ((*(byte *)(param_2 + 0xe) & 1) != 0) {
LAB_1417bc03b:
      uVar24 = param_2[0xf] & iVar21 - 1U;
      iVar25 = iVar21 - uVar24;
      if (uVar24 == 0) {
        iVar25 = 0;
      }
      uVar24 = iVar21 - 1U & param_2[0x10];
      iVar21 = iVar21 - uVar24;
      if (uVar24 == 0) {
        iVar21 = 0;
      }
      uVar22 = CONCAT44(iVar21 + param_2[0x10],iVar25 + param_2[0xf]);
      goto LAB_1417bc072;
    }
LAB_1417bc096:
    uStack_1b8 = CONCAT44(uStack_184,uStack_188);
    uStack_1c8 = CONCAT44(uStack_194,
                          CONCAT13(bStack_195,CONCAT12(uStack_196,CONCAT11(uStack_197,uStack_198))))
    ;
    uStack_1c0 = CONCAT44(uStack_18c,uStack_190);
    uStack_1d8 = (uint)uStack_1a8;
    uStack_1d4 = 0;
    uStack_1d0 = uStack_1a0;
    uStack_1cc = uStack_19c;
    uVar17 = 0;
  }
  uStack_118 = (uint)uStack_1b8;
  uStack_114 = uStack_1b8._4_4_;
  uStack_b0 = uVar17;
  if (((uint)uStack_1b8 == 0) || (uStack_1b8._4_4_ == 0)) {
    FUN_1428d9430(&UNK_142cdfad8,0x2c,&UNK_142cdfb08);
    goto LAB_1417bc50c;
  }
  uStack_110 = (uint)uStack_1b8;
  uStack_10c = uStack_1b8._4_4_;
  uVar24 = (uint)uStack_1b8;
  if ((uint)uStack_1b8 < uStack_1b8._4_4_) {
    uVar24 = uStack_1b8._4_4_;
  }
  if (param_2[0x11] == 0) {
    pcVar20 = (char *)0x1;
  }
  else {
    uVar18 = 0x1f;
    if (uVar24 != 0) {
      for (; uVar24 >> uVar18 == 0; uVar18 = uVar18 - 1) {
      }
    }
    uVar24 = (uVar18 ^ 0xffffffe0) + 0x21;
    pcVar20 = (char *)(ulonglong)uVar24;
    if (param_2[0x11] == 2) {
      if ((uint)param_2[0x12] < uVar24) {
        uVar24 = param_2[0x12];
      }
      pcVar20 = (char *)(ulonglong)uVar24;
    }
  }
  uStack_178 = &cStack_a4;
  uStack_168 = (undefined4 *)auStack_a3;
  plStack_180 = param_1;
  uStack_170 = param_2;
  acStack_a8[2] = cVar13;
  FUN_1420e3080(&uStack_218,param_2,&uStack_1d8,&uStack_178);
  auStack_a3[2] = 1;
  puVar15 = &uStack_118;
  pcVar23 = acStack_a8;
  FUN_1417bc930(&uStack_108,puVar15,pcVar23,&uStack_1d8);
  uVar22 = 1;
  uVar24 = (uint)pcVar20;
  if (1 < uVar24) {
    do {
      uVar18 = (int)uVar22 + 1;
      uVar22 = (ulonglong)uVar18;
      if (((uVar18 != uVar24) && ((uint)uStack_e8 < 2)) && (uStack_e8._4_4_ < 2)) {
        FUN_1428d9430(&UNK_142cdfbc0,0x4c,&UNK_142cdfc10);
        goto LAB_1417bc50c;
      }
      uStack_178 = &cStack_a4;
      uStack_170 = &uStack_218;
      uStack_160 = auStack_a3;
      uStack_168 = param_2;
      FUN_1420e2bd0(param_2,&uStack_108,&uStack_178);
      puVar15 = &uStack_118;
      pcVar23 = acStack_a8;
      FUN_1417bc930(&uStack_178,puVar15,acStack_a8,&uStack_1d8);
      func_0x0001417b5550(&uStack_108);
      uStack_e8 = uStack_158;
      uStack_f8 = uStack_168;
      uStack_f0 = uStack_160;
      uStack_108 = uStack_178;
      uStack_100 = uStack_170;
    } while (uVar18 < uVar24);
  }
  func_0x0001417b5550(&uStack_108);
  if (cStack_a4 == '\0') {
    cVar13 = '%';
    if (acStack_a8[2] != '\0') {
      cVar13 = '+';
    }
  }
  else {
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(auStack_a3[0]) {
    case 0:
      cVar13 = -0x73;
      if (acStack_a8[2] != '\0') {
        auStack_a3[2] = 1;
        FUN_1428d9430(&UNK_142cdfb38,0xf,&UNK_142cdfb48);
        goto LAB_1417bc50c;
      }
      break;
    case 1:
      cVar13 = -0x75;
      if (acStack_a8[2] != '\0') {
        uStack_178 = &UNK_142cdfb98;
        uStack_170 = (undefined4 *)0x1;
        uStack_168 = (undefined4 *)0x8;
        uStack_160 = (undefined1 *)0x0;
        uStack_158 = 0;
        auStack_a3[2] = 1;
        FUN_1428d9390(&uStack_178,&UNK_142cdfba8);
        goto LAB_1417bc50c;
      }
      break;
    case 2:
      cVar13 = acStack_a8[2] + -0x6f;
      break;
    case 3:
      auStack_a3[2] = 1;
      FUN_1428d9310(&UNK_142cdfb20);
LAB_1417bc50c:
                    /* WARNING: Does not return */
      pcVar4 = (code *)invalidInstructionException();
      (*pcVar4)();
    default:
      pbVar12 = (byte *)(ulonglong)uStack_1d8;
      uStack_1d4 = CONCAT31(uStack_1d4._1_3_,(char)uStack_1d4 + '\x01');
      *pcVar20 = *pcVar20 + '\x01';
      *pbVar12 = *pbVar12 + 1;
      bVar9 = (byte)uStack_1d8;
      *pbVar12 = *pbVar12 + bVar9;
      *pbVar12 = *pbVar12 + bVar9;
      *pbVar12 = *pbVar12 + bVar9;
      out(uStack_1d4,(short)puVar15);
      bVar16 = (byte)((ulonglong)puVar15 >> 8);
      if (CARRY1(bStack_195,bVar9)) {
        *pbVar12 = *pbVar12 + 0xad;
        *pbVar12 = *pbVar12 + bVar9;
        *pbVar12 = *pbVar12 + bVar9;
        *pbVar12 = *pbVar12 + bVar9;
        pbVar12[-0x57] = pbVar12[-0x57] + bVar16;
        *(uint *)pbVar12 = *(int *)pbVar12 + uStack_1d8;
        bVar26 = CARRY1(*pbVar12,bVar9);
        *pbVar12 = *pbVar12 + bVar9;
        if (!bVar26) {
          *pbVar12 = *pbVar12 + bVar9;
          *pcVar23 = *pcVar23 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *(uint *)pbVar12 = *(int *)pbVar12 + uStack_1d8;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *(char *)((longlong)pbVar12 * 2) = *(char *)((longlong)pbVar12 * 2) + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          *pbVar12 = *pbVar12 + bVar9;
          pcVar20 = (char *)(ulonglong)uStack_1d8;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          pcVar20 = (char *)(ulonglong)uStack_1d8;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + -0x53;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + -0x53;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + bVar9;
          *pcVar20 = *pcVar20 + (char)puVar15;
                    /* WARNING: Bad instruction - Truncating control flow here */
          halt_baddata();
        }
      }
      else {
        if ((byte)(bStack_195 + bVar9) != '\0') {
                    /* WARNING: Bad instruction - Truncating control flow here */
          halt_baddata();
        }
        pbVar1 = (byte *)(CONCAT71((int7)(uVar22 >> 8),0xad) + 0x74);
        bVar26 = CARRY1(*pbVar1,bVar16);
        *pbVar1 = *pbVar1 + bVar16;
      }
      if (!bVar26) {
        out((undefined1)uStack_1d0,(short)puVar15);
        *(undefined1 *)(longlong)(int)(uVar24 + 0x72) = 0;
        *pbVar12 = *pbVar12 + bVar9;
        *(char *)puVar15 = (char)*puVar15 - (char)puVar15;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
      *pbVar12 = *pbVar12 + bVar9;
      pbVar12[0x12] = pbVar12[0x12] + 0xad;
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  uStack_f8 = (undefined4 *)uStack_208;
  uStack_108 = (char *)CONCAT44(uStack_214,uStack_218);
  uStack_100 = (undefined4 *)CONCAT44(uStack_20c,uStack_210);
  uStack_f0 = (undefined1 *)CONCAT44(uStack_10c,uStack_110);
  uStack_e8 = CONCAT26(1,CONCAT24((short)pcVar20,1));
  uStack_e0 = CONCAT31(CONCAT21(uStack_e0._2_2_,cVar13),2);
  FUN_1417b55c0(&pcStack_248,&uStack_108);
  if (pcStack_248 == (char *)0x8000000000000004) {
    if (uStack_108 != (char *)0x0) {
      func_0x000140613c20(uStack_100,uStack_108,1);
    }
    *plStack_180 = (longlong)puStack_240;
    plStack_180[1] = (longlong)CONCAT44(uStack_234,uStack_238);
    plStack_180[2] = (longlong)CONCAT44(uStack_22c,uStack_230);
  }
  else {
    uStack_150 = uStack_220;
    uStack_14c = uStack_21c;
    uStack_178 = pcStack_248;
    uStack_170 = puStack_240;
    uStack_168 = (undefined4 *)CONCAT44(uStack_234,uStack_238);
    uStack_160 = (undefined1 *)CONCAT44(uStack_22c,uStack_230);
    lVar11 = FUN_142915940(&uStack_178);
    plStack_180[1] = lVar11;
    *plStack_180 = 0;
    if (uStack_108 != (char *)0x0) {
      func_0x000140613c20(uStack_100,uStack_108,1);
    }
  }
  func_0x0001417b5550(&uStack_1d8);
  if ((char)uStack_b0 != '\0') {
    func_0x0001417b5550(&uStack_1a8);
  }
  if ((char)uStack_ac != '\0') {
    func_0x0001417b5550(&pcStack_200);
  }
  return;
}

