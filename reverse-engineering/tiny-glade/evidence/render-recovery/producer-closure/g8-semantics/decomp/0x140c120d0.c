
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong * FUN_140c120d0(ulonglong *param_1,ulonglong *param_2)

{
  uint *puVar1;
  undefined1 uVar2;
  uint uVar3;
  uint uVar4;
  code *pcVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  ulonglong *puVar8;
  undefined8 *puVar9;
  undefined4 *puVar10;
  ulonglong uVar11;
  undefined1 *puVar12;
  ulonglong *puVar13;
  ulonglong uVar14;
  uint *puVar15;
  ulonglong unaff_R13;
  ulonglong *unaff_R14;
  ulonglong uVar16;
  uint uStack_608;
  uint auStack_604 [64];
  uint uStack_504;
  undefined1 auStack_500 [256];
  undefined8 uStack_400;
  undefined *puStack_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined4 uStack_3d4;
  undefined8 uStack_3d0;
  undefined *puStack_3c8;
  undefined8 uStack_3c0;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
  undefined *puStack_3b0;
  undefined8 uStack_3a8;
  ulonglong **ppuStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  ulonglong uStack_2a8;
  ulonglong uStack_c8;
  ulonglong *puStack_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong *puStack_a8;
  ulonglong uStack_a0;
  undefined4 uStack_94;
  ulonglong uStack_90;
  ulonglong *puStack_88;
  code *pcStack_80;
  ulonglong *puStack_78;
  code *pcStack_70;
  ulonglong *puStack_68;
  code *pcStack_60;
  ulonglong uStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar16 = param_2[0xd6];
  if (uVar16 == 0) {
    puVar9 = (undefined8 *)func_0x000140613c10(0x34,1);
    uVar7 = _UNK_142b1b3c8;
    if (puVar9 != (undefined8 *)0x0) {
      puVar9[4] = _UNK_142b1b3c0;
      puVar9[5] = uVar7;
      uVar7 = _UNK_142b1b3b8;
      puVar9[2] = _UNK_142b1b3b0;
      puVar9[3] = uVar7;
      uVar7 = _UNK_142b1b3a8;
      *puVar9 = _UNK_142b1b3a0;
      puVar9[1] = uVar7;
      *(undefined4 *)(puVar9 + 6) = 0x2e6c6c75;
      param_1[1] = 4;
      param_1[2] = 0x34;
      param_1[3] = (ulonglong)puVar9;
      param_1[4] = 0x34;
      *param_1 = 0x8000000000000000;
      return param_1;
    }
    FUN_1428d8fe3(1,0x34,&UNK_142b1adc8);
    pcVar5 = (code *)swi(3);
    puVar8 = (ulonglong *)(*pcVar5)();
    return puVar8;
  }
  (*(code *)param_2[0xc3])(uVar16,&uStack_608);
  uVar11 = (ulonglong)uStack_608;
  if (uVar11 < 0x21) {
    unaff_R13 = (ulonglong)uStack_504;
    if (0x10 < unaff_R13) goto code_r0x000140c12d24;
    unaff_R14 = (ulonglong *)(unaff_R13 << 4);
    uStack_b0 = uVar16;
    uStack_a0 = uVar11;
    if (uStack_504 == 0) {
      puVar8 = (ulonglong *)0x8;
code_r0x000140c121f0:
      puStack_a8 = unaff_R14;
      func_0x000142923c00(puVar8,auStack_500,unaff_R14);
      uStack_c8 = unaff_R13;
      puStack_c0 = puVar8;
      uStack_b8 = unaff_R13;
      if ((char)param_2[0xd7] != '\0') {
        if (3 < *puRam000000014362c3a8) {
          pcStack_80 = FUN_140d8d560;
          puStack_88 = (ulonglong *)&uStack_608;
          puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b250);
          uStack_3e0 = *puVar10;
          uStack_3dc = puVar10[1];
          uStack_3d8 = puVar10[2];
          uStack_3d4 = puVar10[3];
          uStack_3b4 = puVar10[4];
          uStack_3d0 = 4;
          puStack_3c8 = &UNK_142b1b078;
          uStack_3c0 = 0x15;
          puStack_3b0 = &UNK_142b1b240;
          uStack_3a8 = 1;
          ppuStack_3a0 = &puStack_88;
          uStack_398 = 1;
          uStack_390 = 0;
          uStack_400 = 0;
          puStack_3f8 = &UNK_142b1b078;
          uStack_3f0 = 0x15;
          uStack_3e8 = 0;
          uStack_3b8 = 1;
          func_0x000140d4a330(&uStack_49,&uStack_400);
        }
        if (3 < *puRam000000014362c3a8) {
          puStack_88 = (ulonglong *)&uStack_504;
          pcStack_80 = FUN_140d8d560;
          puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b290);
          uStack_3e0 = *puVar10;
          uStack_3dc = puVar10[1];
          uStack_3d8 = puVar10[2];
          uStack_3d4 = puVar10[3];
          uStack_3b4 = puVar10[4];
          uStack_3d0 = 4;
          puStack_3c8 = &UNK_142b1b078;
          uStack_3c0 = 0x15;
          puStack_3b0 = &UNK_142b1b280;
          uStack_3a8 = 1;
          ppuStack_3a0 = &puStack_88;
          uStack_398 = 1;
          uStack_390 = 0;
          uStack_400 = 0;
          puStack_3f8 = &UNK_142b1b078;
          uStack_3f0 = 0x15;
          uStack_3e8 = 0;
          uStack_3b8 = 1;
          func_0x000140d4a330(&uStack_49,&uStack_400);
        }
        puVar15 = auStack_604;
        puVar1 = puVar15 + uStack_a0 * 2;
        uVar16 = 0;
        puVar13 = puRam000000014362c3a8;
        for (; puVar15 != puVar1; puVar15 = puVar15 + 2) {
          uStack_90 = uVar16;
          if (3 < *puVar13) {
            uStack_58 = CONCAT44(uStack_58._4_4_,*puVar15);
            puStack_68 = (ulonglong *)(puVar15 + 1);
            puStack_88 = &uStack_90;
            pcStack_80 = FUN_140d8d6c0;
            puStack_78 = &uStack_58;
            pcStack_70 = FUN_140d8cc50;
            pcStack_60 = FUN_140d8d560;
            puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b388);
            uStack_3e0 = *puVar10;
            uStack_3dc = puVar10[1];
            uStack_3d8 = puVar10[2];
            uStack_3d4 = puVar10[3];
            uStack_3b4 = puVar10[4];
            uStack_3d0 = 4;
            puStack_3c8 = &UNK_142b1b078;
            uStack_3c0 = 0x15;
            puStack_3b0 = &UNK_142b1b348;
            uStack_3a8 = 4;
            ppuStack_3a0 = &puStack_88;
            uStack_398 = 3;
            uStack_390 = 0;
            uStack_400 = 0;
            puStack_3f8 = &UNK_142b1b078;
            uStack_3f0 = 0x15;
            uStack_3e8 = 0;
            uStack_3b8 = 1;
            func_0x000140d4a330(&uStack_49,&uStack_400);
            puVar13 = puRam000000014362c3a8;
          }
          uVar16 = uVar16 + 1;
        }
        puStack_a8 = (ulonglong *)((longlong)puStack_a8 + (longlong)puVar8);
        uVar16 = 0;
        for (; puVar8 != puStack_a8; puVar8 = puVar8 + 2) {
          uStack_58 = uVar16;
          if (3 < *puVar13) {
            uStack_94 = (undefined4)puVar8[1];
            uStack_90 = *puVar8 >> 0x14;
            puStack_88 = &uStack_58;
            pcStack_80 = FUN_140d8d6c0;
            puStack_78 = (ulonglong *)&uStack_94;
            pcStack_70 = FUN_140d8cc50;
            puStack_68 = &uStack_90;
            pcStack_60 = FUN_140d8d6c0;
            puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b308);
            uStack_3e0 = *puVar10;
            uStack_3dc = puVar10[1];
            uStack_3d8 = puVar10[2];
            uStack_3d4 = puVar10[3];
            uStack_3b4 = puVar10[4];
            uStack_3d0 = 4;
            puStack_3c8 = &UNK_142b1b078;
            uStack_3c0 = 0x15;
            puStack_3b0 = &UNK_142b1b2c8;
            uStack_3a8 = 4;
            ppuStack_3a0 = &puStack_88;
            uStack_398 = 3;
            uStack_390 = 0;
            uStack_400 = 0;
            puStack_3f8 = &UNK_142b1b078;
            uStack_3f0 = 0x15;
            uStack_3e8 = 0;
            uStack_3b8 = 1;
            func_0x000140d4a330(&uStack_49,&uStack_400);
            puVar13 = puRam000000014362c3a8;
          }
          uVar16 = uVar16 + 1;
        }
      }
      uVar16 = uStack_a0;
      if ((int)uStack_a0 == 0) {
        uVar11 = 8;
code_r0x000140c12787:
        (*(code *)param_2[0xc1])(uStack_b0,&uStack_400);
        param_1[5] = uStack_b8;
        param_1[3] = uStack_c8;
        param_1[4] = (ulonglong)puStack_c0;
        uVar14 = param_2[2];
        uVar6 = param_2[3];
        *param_1 = uVar16;
        param_1[1] = uVar11;
        param_1[2] = uVar16;
        param_1[6] = uVar14;
        param_1[7] = uVar6;
        uVar16 = param_2[5];
        param_1[8] = param_2[4];
        param_1[9] = uVar16;
        uVar16 = param_2[7];
        param_1[10] = param_2[6];
        param_1[0xb] = uVar16;
        uVar16 = param_2[9];
        param_1[0xc] = param_2[8];
        param_1[0xd] = uVar16;
        uVar16 = param_2[0xb];
        param_1[0xe] = param_2[10];
        param_1[0xf] = uVar16;
        uVar16 = param_2[0xd];
        param_1[0x10] = param_2[0xc];
        param_1[0x11] = uVar16;
        uVar16 = param_2[0xf];
        param_1[0x12] = param_2[0xe];
        param_1[0x13] = uVar16;
        uVar16 = param_2[0x11];
        param_1[0x14] = param_2[0x10];
        param_1[0x15] = uVar16;
        uVar16 = param_2[0x13];
        param_1[0x16] = param_2[0x12];
        param_1[0x17] = uVar16;
        uVar16 = param_2[0x15];
        param_1[0x18] = param_2[0x14];
        param_1[0x19] = uVar16;
        uVar16 = param_2[0x17];
        param_1[0x1a] = param_2[0x16];
        param_1[0x1b] = uVar16;
        uVar16 = param_2[0x19];
        param_1[0x1c] = param_2[0x18];
        param_1[0x1d] = uVar16;
        uVar16 = param_2[0x1b];
        param_1[0x1e] = param_2[0x1a];
        param_1[0x1f] = uVar16;
        uVar16 = param_2[0x1d];
        param_1[0x20] = param_2[0x1c];
        param_1[0x21] = uVar16;
        uVar16 = param_2[0x1f];
        param_1[0x22] = param_2[0x1e];
        param_1[0x23] = uVar16;
        uVar16 = param_2[0x21];
        param_1[0x24] = param_2[0x20];
        param_1[0x25] = uVar16;
        uVar16 = param_2[0x23];
        param_1[0x26] = param_2[0x22];
        param_1[0x27] = uVar16;
        uVar16 = param_2[0x25];
        param_1[0x28] = param_2[0x24];
        param_1[0x29] = uVar16;
        uVar16 = param_2[0x27];
        param_1[0x2a] = param_2[0x26];
        param_1[0x2b] = uVar16;
        uVar16 = param_2[0x29];
        param_1[0x2c] = param_2[0x28];
        param_1[0x2d] = uVar16;
        uVar16 = param_2[0x2b];
        param_1[0x2e] = param_2[0x2a];
        param_1[0x2f] = uVar16;
        uVar16 = param_2[0x2d];
        param_1[0x30] = param_2[0x2c];
        param_1[0x31] = uVar16;
        uVar16 = param_2[0x2f];
        param_1[0x32] = param_2[0x2e];
        param_1[0x33] = uVar16;
        uVar16 = param_2[0x31];
        param_1[0x34] = param_2[0x30];
        param_1[0x35] = uVar16;
        uVar16 = param_2[0x33];
        param_1[0x36] = param_2[0x32];
        param_1[0x37] = uVar16;
        uVar16 = param_2[0x35];
        param_1[0x38] = param_2[0x34];
        param_1[0x39] = uVar16;
        uVar16 = param_2[0x37];
        param_1[0x3a] = param_2[0x36];
        param_1[0x3b] = uVar16;
        uVar16 = param_2[0x39];
        param_1[0x3c] = param_2[0x38];
        param_1[0x3d] = uVar16;
        uVar16 = param_2[0x3b];
        param_1[0x3e] = param_2[0x3a];
        param_1[0x3f] = uVar16;
        uVar16 = param_2[0x3d];
        param_1[0x40] = param_2[0x3c];
        param_1[0x41] = uVar16;
        uVar16 = param_2[0x3f];
        param_1[0x42] = param_2[0x3e];
        param_1[0x43] = uVar16;
        uVar16 = param_2[0x41];
        param_1[0x44] = param_2[0x40];
        param_1[0x45] = uVar16;
        uVar16 = param_2[0x43];
        param_1[0x46] = param_2[0x42];
        param_1[0x47] = uVar16;
        uVar16 = param_2[0x45];
        param_1[0x48] = param_2[0x44];
        param_1[0x49] = uVar16;
        uVar16 = param_2[0x47];
        param_1[0x4a] = param_2[0x46];
        param_1[0x4b] = uVar16;
        uVar16 = param_2[0x49];
        param_1[0x4c] = param_2[0x48];
        param_1[0x4d] = uVar16;
        uVar16 = param_2[0x4b];
        param_1[0x4e] = param_2[0x4a];
        param_1[0x4f] = uVar16;
        uVar16 = param_2[0x4d];
        param_1[0x50] = param_2[0x4c];
        param_1[0x51] = uVar16;
        uVar16 = param_2[0x4f];
        param_1[0x52] = param_2[0x4e];
        param_1[0x53] = uVar16;
        uVar16 = param_2[0x51];
        param_1[0x54] = param_2[0x50];
        param_1[0x55] = uVar16;
        uVar16 = param_2[0x53];
        param_1[0x56] = param_2[0x52];
        param_1[0x57] = uVar16;
        uVar16 = param_2[0x55];
        param_1[0x58] = param_2[0x54];
        param_1[0x59] = uVar16;
        uVar16 = param_2[0x57];
        param_1[0x5a] = param_2[0x56];
        param_1[0x5b] = uVar16;
        uVar16 = param_2[0x59];
        param_1[0x5c] = param_2[0x58];
        param_1[0x5d] = uVar16;
        uVar16 = param_2[0x5b];
        param_1[0x5e] = param_2[0x5a];
        param_1[0x5f] = uVar16;
        uVar16 = param_2[0x5d];
        param_1[0x60] = param_2[0x5c];
        param_1[0x61] = uVar16;
        uVar16 = param_2[0x5f];
        param_1[0x62] = param_2[0x5e];
        param_1[99] = uVar16;
        uVar16 = param_2[0x61];
        param_1[100] = param_2[0x60];
        param_1[0x65] = uVar16;
        uVar16 = param_2[99];
        param_1[0x66] = param_2[0x62];
        param_1[0x67] = uVar16;
        uVar16 = param_2[0x65];
        param_1[0x68] = param_2[100];
        param_1[0x69] = uVar16;
        uVar16 = param_2[0x67];
        param_1[0x6a] = param_2[0x66];
        param_1[0x6b] = uVar16;
        uVar16 = param_2[0x69];
        param_1[0x6c] = param_2[0x68];
        param_1[0x6d] = uVar16;
        uVar16 = param_2[0x6b];
        param_1[0x6e] = param_2[0x6a];
        param_1[0x6f] = uVar16;
        uVar16 = param_2[0x6d];
        param_1[0x70] = param_2[0x6c];
        param_1[0x71] = uVar16;
        uVar16 = param_2[0x6f];
        param_1[0x72] = param_2[0x6e];
        param_1[0x73] = uVar16;
        uVar16 = param_2[0x71];
        param_1[0x74] = param_2[0x70];
        param_1[0x75] = uVar16;
        uVar16 = param_2[0x73];
        param_1[0x76] = param_2[0x72];
        param_1[0x77] = uVar16;
        uVar16 = param_2[0x75];
        param_1[0x78] = param_2[0x74];
        param_1[0x79] = uVar16;
        uVar16 = param_2[0x77];
        param_1[0x7a] = param_2[0x76];
        param_1[0x7b] = uVar16;
        uVar16 = param_2[0x79];
        param_1[0x7c] = param_2[0x78];
        param_1[0x7d] = uVar16;
        uVar16 = param_2[0x7b];
        param_1[0x7e] = param_2[0x7a];
        param_1[0x7f] = uVar16;
        uVar16 = param_2[0x7d];
        param_1[0x80] = param_2[0x7c];
        param_1[0x81] = uVar16;
        uVar16 = param_2[0x7f];
        param_1[0x82] = param_2[0x7e];
        param_1[0x83] = uVar16;
        uVar16 = param_2[0x81];
        param_1[0x84] = param_2[0x80];
        param_1[0x85] = uVar16;
        uVar16 = param_2[0x83];
        param_1[0x86] = param_2[0x82];
        param_1[0x87] = uVar16;
        uVar16 = param_2[0x85];
        param_1[0x88] = param_2[0x84];
        param_1[0x89] = uVar16;
        uVar16 = param_2[0x87];
        param_1[0x8a] = param_2[0x86];
        param_1[0x8b] = uVar16;
        uVar16 = param_2[0x89];
        param_1[0x8c] = param_2[0x88];
        param_1[0x8d] = uVar16;
        uVar16 = param_2[0x8b];
        param_1[0x8e] = param_2[0x8a];
        param_1[0x8f] = uVar16;
        uVar16 = param_2[0x8d];
        param_1[0x90] = param_2[0x8c];
        param_1[0x91] = uVar16;
        uVar16 = param_2[0x8f];
        param_1[0x92] = param_2[0x8e];
        param_1[0x93] = uVar16;
        uVar16 = param_2[0x91];
        param_1[0x94] = param_2[0x90];
        param_1[0x95] = uVar16;
        uVar16 = param_2[0x93];
        param_1[0x96] = param_2[0x92];
        param_1[0x97] = uVar16;
        uVar16 = param_2[0x95];
        param_1[0x98] = param_2[0x94];
        param_1[0x99] = uVar16;
        uVar16 = param_2[0x97];
        param_1[0x9a] = param_2[0x96];
        param_1[0x9b] = uVar16;
        uVar16 = param_2[0x99];
        param_1[0x9c] = param_2[0x98];
        param_1[0x9d] = uVar16;
        uVar16 = param_2[0x9b];
        param_1[0x9e] = param_2[0x9a];
        param_1[0x9f] = uVar16;
        uVar16 = param_2[0x9d];
        param_1[0xa0] = param_2[0x9c];
        param_1[0xa1] = uVar16;
        uVar16 = param_2[0x9f];
        param_1[0xa2] = param_2[0x9e];
        param_1[0xa3] = uVar16;
        uVar16 = param_2[0xa1];
        param_1[0xa4] = param_2[0xa0];
        param_1[0xa5] = uVar16;
        uVar16 = param_2[0xa3];
        param_1[0xa6] = param_2[0xa2];
        param_1[0xa7] = uVar16;
        uVar16 = param_2[0xa5];
        param_1[0xa8] = param_2[0xa4];
        param_1[0xa9] = uVar16;
        uVar16 = param_2[0xa7];
        param_1[0xaa] = param_2[0xa6];
        param_1[0xab] = uVar16;
        uVar16 = param_2[0xa9];
        param_1[0xac] = param_2[0xa8];
        param_1[0xad] = uVar16;
        uVar16 = param_2[0xab];
        param_1[0xae] = param_2[0xaa];
        param_1[0xaf] = uVar16;
        uVar16 = param_2[0xad];
        param_1[0xb0] = param_2[0xac];
        param_1[0xb1] = uVar16;
        uVar16 = param_2[0xaf];
        param_1[0xb2] = param_2[0xae];
        param_1[0xb3] = uVar16;
        uVar16 = param_2[0xb1];
        param_1[0xb4] = param_2[0xb0];
        param_1[0xb5] = uVar16;
        uVar16 = param_2[0xb3];
        param_1[0xb6] = param_2[0xb2];
        param_1[0xb7] = uVar16;
        uVar16 = param_2[0xb5];
        param_1[0xb8] = param_2[0xb4];
        param_1[0xb9] = uVar16;
        uVar16 = param_2[0xb7];
        param_1[0xba] = param_2[0xb6];
        param_1[0xbb] = uVar16;
        uVar16 = param_2[0xb9];
        param_1[0xbc] = param_2[0xb8];
        param_1[0xbd] = uVar16;
        uVar16 = param_2[0xbb];
        param_1[0xbe] = param_2[0xba];
        param_1[0xbf] = uVar16;
        uVar16 = *param_2;
        uVar11 = param_2[1];
        param_1[0xc0] = uStack_2a8;
        param_1[0xc1] = uVar16;
        param_1[0xc2] = uVar11;
        uVar16 = param_2[0xd7];
        *(undefined2 *)((longlong)param_1 + 0x61c) = *(undefined2 *)((longlong)param_2 + 0x6bc);
        *(int *)(param_1 + 0xc3) = (int)uVar16;
        return param_1;
      }
      unaff_R14 = (ulonglong *)(uStack_a0 * 0x38);
      uVar11 = func_0x000140613c10(unaff_R14);
      if (uVar11 != 0) {
        uVar2 = *(undefined1 *)((longlong)param_2 + 0x6be);
        puVar12 = (undefined1 *)(uVar11 + 0x35);
        uVar14 = 0;
        do {
          uVar3 = auStack_604[uVar14 * 2];
          uVar4 = auStack_604[uVar14 * 2 + 1];
          *(undefined8 *)(puVar12 + -0x35) = 0;
          *(undefined8 *)(puVar12 + -0x2d) = 8;
          *(undefined8 *)(puVar12 + -0x25) = 0;
          *(ulonglong *)(puVar12 + -0x1d) = uVar14;
          *(ulonglong *)(puVar12 + -0x15) = (ulonglong)uVar4;
          *(undefined8 *)(puVar12 + -0xd) = 0;
          *(uint *)(puVar12 + -5) = uVar3;
          puVar12[-1] = (byte)uVar3 >> 1 & 1;
          *puVar12 = uVar2;
          uVar14 = uVar14 + 1;
          puVar12 = puVar12 + 0x38;
        } while (uVar16 != uVar14);
        goto code_r0x000140c12787;
      }
      goto code_r0x000140c12d4c;
    }
    puVar8 = (ulonglong *)func_0x000140613c10(unaff_R14,8);
    if (puVar8 != (ulonglong *)0x0) goto code_r0x000140c121f0;
  }
  else {
    FUN_1428d9840(uVar11,0x20,&UNK_142b1ac00);
code_r0x000140c12d24:
    FUN_1428d9840(unaff_R13,0x10,&UNK_142b1abe8);
  }
  FUN_1428d8fe3(8,unaff_R14,&UNK_142b1adc8);
code_r0x000140c12d4c:
  FUN_1428d8fe3(8,unaff_R14,&UNK_142b1ad40);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

