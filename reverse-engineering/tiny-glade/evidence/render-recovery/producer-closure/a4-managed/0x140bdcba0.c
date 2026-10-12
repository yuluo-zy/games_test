
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000140bdcddc) overlaps instruction at (ram,0x000140bdcdda)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140bdcba0(undefined8 *param_1,code *param_2,longlong param_3,byte *param_4,uint *param_5,
                  code *param_6)

{
  byte bVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  undefined4 uVar5;
  uint *puVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int *piVar10;
  uint *puVar11;
  undefined1 *puVar12;
  code *pcVar13;
  undefined1 **ppuVar14;
  undefined1 *puVar15;
  undefined *puVar16;
  uint uVar17;
  uint uVar18;
  ulonglong uVar19;
  code *pcVar20;
  code *pcVar21;
  uint uVar22;
  byte in_AF;
  byte in_TF;
  byte in_IF;
  byte in_NT;
  byte in_AC;
  byte in_VIF;
  byte in_VIP;
  byte in_ID;
  float10 in_ST0;
  undefined1 *puStack_128;
  undefined *puStack_120;
  undefined1 auStack_118 [44];
  uint uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  byte **ppbStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  byte *pbStack_98;
  code *pcStack_90;
  undefined8 *puStack_88;
  code *pcStack_80;
  
  puVar6 = param_5;
  ppuVar14 = (undefined1 **)auStack_118;
  puVar12 = auStack_118;
  if (*param_5 < 4) {
    param_1[1] = 0;
    goto code_r0x000140bdcf2f;
  }
  puVar11 = param_5 + 2;
  if (param_4[0x10] != 0) {
    puVar11 = (uint *)(param_4 + 0x14);
  }
  uVar2 = *puVar11;
  if ((param_4[0x18] & 1) == 0) {
code_r0x000140bdcc8d:
    puStack_120 = &UNK_140bdcc94;
    uVar7 = FUN_141738fe0(uVar2);
    uVar22 = 2;
    if (uVar7 != 6) {
      uVar22 = uVar7;
    }
  }
  else {
    uVar22 = *(uint *)(param_4 + 0x1c);
    puStack_120 = &UNK_140bdcc05;
    uVar7 = FUN_141738fe0(uVar2);
    if ((uVar7 & uVar22) == 0) {
      param_4 = param_4 + 0x18;
      pcStack_90 = FUN_140bdbcf0;
      puStack_88 = &param_5;
      pcStack_80 = FUN_1418bd4e0;
      puStack_e8 = &UNK_142b12688;
      uStack_e0 = 2;
      uStack_c8 = 0;
      ppbStack_d8 = &pbStack_98;
      uStack_d0 = 2;
      puStack_120 = &UNK_140bdcc8d;
      pbStack_98 = param_4;
      FUN_1428d9390(&puStack_e8,&UNK_142b126a8);
      goto code_r0x000140bdcc8d;
    }
  }
  if (param_4[0x38] != 0) {
    puStack_e8 = &UNK_142b12700;
    uStack_e0 = 1;
    ppbStack_d8 = (byte **)0x8;
    uStack_d0 = 0;
    uStack_c8 = 0;
    puStack_120 = &UNK_140bdcf81;
    FUN_1428d9390(&puStack_e8,&UNK_142b12710);
    pcVar13 = (code *)swi(3);
    (*pcVar13)();
    return;
  }
  uVar7 = *(uint *)(param_4 + 0x20);
  uVar3 = *(uint *)(param_4 + 0x24);
  pcVar21 = (code *)(ulonglong)uVar3;
  uVar4 = *(uint *)(param_4 + 0x28);
  uStack_ec = *(uint *)(param_4 + 0x2c);
  uVar5 = *(undefined4 *)(param_4 + 0x30);
  uVar8 = *(uint *)(param_4 + 0x34);
  bVar1 = *(byte *)((longlong)puVar6 + 0x1e);
  pcVar13 = param_6;
  if ((*param_4 & 1) != 0) {
    uVar17 = *(uint *)(param_4 + 4);
    pcVar20 = (code *)(ulonglong)(byte)puVar6[7];
    if ((uVar7 & 1) != 0) {
      pcVar20 = pcVar21;
    }
    uVar7 = 1;
    puVar16 = (undefined *)(ulonglong)bVar1;
code_r0x000140bdccf4:
    uVar18 = (uint)pcVar20;
    ppuVar14 = (undefined1 **)auStack_118;
    puVar15 = auStack_118;
    pcVar21 = param_2;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
    switch(&UNK_142b12744 + *(int *)(&UNK_142b12744 + (longlong)puVar16 * 4)) {
    case (undefined *)0x140bdcd10:
      puVar15 = auStack_118;
      uVar7 = uStack_ec;
      if ((uVar4 & 1) != 0) break;
      goto code_r0x000140bdcd97;
    case (undefined *)0x140bdcdbc:
      goto code_r0x000140bdcdbc;
    case (undefined *)0x140bdcde5:
      goto code_r0x000140bdcde5;
    case (undefined *)0x142b12744:
      pcVar13 = (code *)swi(3);
      (*pcVar13)();
      return;
    case (undefined *)0x142b1274c:
      pcVar13 = (code *)swi(3);
      (*pcVar13)();
      return;
    }
    goto code_r0x000140bdcdf8;
  }
  uVar17 = (uint)bVar1;
  puVar16 = &UNK_142b12728;
  uVar19 = (ulonglong)*(int *)(&UNK_142b12728 + (ulonglong)bVar1 * 4);
  pcVar20 = (code *)(&UNK_142b12728 + uVar19);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(bVar1) {
  case 0:
    break;
  case 1:
    uVar17 = 4;
    goto code_r0x000140bdcd75;
  case 2:
    uVar17 = 1;
    break;
  case 3:
    uVar17 = 5;
code_r0x000140bdcd75:
    uVar18 = (uint)(byte)puVar6[7];
    if ((uVar7 & 1) != 0) {
      uVar18 = uVar3;
    }
    puVar15 = auStack_118;
    uVar7 = uStack_ec;
    if ((uVar4 & 1) == 0) {
code_r0x000140bdcd97:
      puVar15 = auStack_118;
      uVar7 = puVar6[6];
    }
    goto code_r0x000140bdcdf8;
  case 4:
    uVar17 = 2;
    break;
  case 5:
    pcVar20 = (code *)(ulonglong)(byte)puVar6[7];
    goto code_r0x000140bdcda4;
  case 6:
    pcVar20 = (code *)(ulonglong)(byte)puVar6[7];
    goto code_r0x000140bdcdcd;
  case 7:
  case 9:
  case 0xb:
    *(longlong *)((ulonglong)uVar8 - 0x75) = (longlong)in_ST0;
    puStack_120 = (undefined *)
                  ((ulonglong)(in_NT & 1) * 0x4000 | (ulonglong)SCARRY8(uVar19,0x142b12728) * 0x800
                   | (ulonglong)(in_IF & 1) * 0x200 | (ulonglong)(in_TF & 1) * 0x100 |
                   (ulonglong)((longlong)pcVar20 < 0) * 0x80 |
                   (ulonglong)(pcVar20 == (code *)0x0) * 0x40 | (ulonglong)(in_AF & 1) * 0x10 |
                   (ulonglong)((POPCOUNT((ulonglong)pcVar20 & 0xff) & 1U) == 0) * 4 |
                   (ulonglong)(0xfffffffebd4ed8d7 < uVar19) | (ulonglong)(in_ID & 1) * 0x200000 |
                   (ulonglong)(in_VIP & 1) * 0x100000 | (ulonglong)(in_VIF & 1) * 0x80000 |
                  (ulonglong)(in_AC & 1) * 0x40000);
    puStack_128 = (undefined1 *)&puStack_120;
    piVar10 = (int *)((ulonglong)uVar8 & 0xffffffffffffff48);
    *piVar10 = *piVar10 + (int)piVar10;
    *(char *)(param_3 + -0x75) = *(char *)(param_3 + -0x75) + (char)piVar10;
    ppuVar14 = &puStack_128;
    uVar8 = uVar8 & 0xffffff08;
    pcVar13 = param_2;
    goto code_r0x000140bdcde5;
  case 8:
  case 10:
    goto code_r0x000140bdccf4;
  case 0xc:
    uVar7 = uVar7 & 0xff;
    uVar8 = uVar8 & 0xffffff1c;
code_r0x000140bdcda4:
    uVar18 = (uint)pcVar20;
    if ((uVar7 & 1) != 0) {
      uVar18 = uVar3;
    }
    uVar17 = 3;
code_r0x000140bdcdbc:
    uVar7 = 6;
code_r0x000140bdcdf3:
    puVar15 = puVar12;
    if ((uVar4 & 1) != 0) {
      uVar7 = *(uint *)(puVar12 + 0x2c);
    }
    goto code_r0x000140bdcdf8;
  case 0xd:
    uVar7 = uVar7 & 0xff;
    uVar8 = uVar8 & 0xffffff1c;
code_r0x000140bdcdcd:
    pcVar20 = (code *)((ulonglong)pcVar20 & 0xffffffff);
    if ((uVar7 & 1) != 0) {
      pcVar20 = (code *)(ulonglong)uVar3;
    }
    uVar17 = 6;
    pcVar21 = param_2;
code_r0x000140bdcde5:
    uVar18 = (uint)pcVar20;
    uVar7 = puVar6[6] * 6;
    puVar12 = (undefined1 *)ppuVar14;
    param_2 = pcVar21;
    goto code_r0x000140bdcdf3;
  default:
  case 0x10:
  case 0x12:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  uVar18 = (uint)(byte)puVar6[7];
  if ((uVar7 & 1) != 0) {
    uVar18 = uVar3;
  }
  puVar15 = auStack_118;
  uVar7 = 1;
code_r0x000140bdcdf8:
  *(undefined4 *)(puVar15 + 0x30) = 0xf;
  *(undefined8 *)(puVar15 + 0x38) = 0;
  *(undefined4 *)(puVar15 + 0x40) = 0;
  *(code **)(puVar15 + 0x48) = pcVar13;
  *(uint *)(puVar15 + 0x50) = uVar17;
  *(uint *)(puVar15 + 0x54) = uVar2;
  *(undefined4 *)(puVar15 + 0x58) = _UNK_142b12560;
  *(undefined4 *)(puVar15 + 0x5c) = _UNK_142b12564;
  *(undefined4 *)(puVar15 + 0x60) = _UNK_142b12568;
  *(undefined4 *)(puVar15 + 100) = _UNK_142b1256c;
  *(uint *)(puVar15 + 0x68) = uVar22;
  *(undefined4 *)(puVar15 + 0x6c) = uVar5;
  *(uint *)(puVar15 + 0x70) = uVar18;
  *(uint *)(puVar15 + 0x74) = uVar8;
  *(uint *)(puVar15 + 0x78) = uVar7;
  if ((param_4[8] & 1) != 0) {
    uVar5 = *(undefined4 *)(param_4 + 0xc);
    *(undefined4 *)(puVar15 + 0xc0) = 0x3b9c930a;
    *(undefined8 *)(puVar15 + 200) = 0;
    *(undefined4 *)(puVar15 + 0xd0) = uVar5;
    *(undefined8 *)(puVar15 + 0xb0) = *(undefined8 *)(puVar15 + 0x70);
    *(undefined8 *)(puVar15 + 0xb8) = *(undefined8 *)(puVar15 + 0x78);
    *(undefined8 *)(puVar15 + 0xa0) = *(undefined8 *)(puVar15 + 0x60);
    *(undefined8 *)(puVar15 + 0xa8) = *(undefined8 *)(puVar15 + 0x68);
    *(undefined8 *)(puVar15 + 0x90) = *(undefined8 *)(puVar15 + 0x50);
    *(undefined8 *)(puVar15 + 0x98) = *(undefined8 *)(puVar15 + 0x58);
    *(undefined8 *)(puVar15 + 0x80) = *(undefined8 *)(puVar15 + 0x40);
    *(undefined8 *)(puVar15 + 0x88) = *(undefined8 *)(puVar15 + 0x48);
    puVar12 = puVar15 + 0xc0;
    do {
      puVar12 = *(undefined1 **)(puVar12 + 8);
    } while (puVar12 != (undefined1 *)0x0);
    *(undefined1 **)(puVar15 + 0x38) = puVar15 + 0xc0;
    *(undefined8 *)(puVar15 + 0x70) = *(undefined8 *)(puVar15 + 0xb0);
    *(undefined8 *)(puVar15 + 0x78) = *(undefined8 *)(puVar15 + 0xb8);
    *(undefined8 *)(puVar15 + 0x60) = *(undefined8 *)(puVar15 + 0xa0);
    *(undefined8 *)(puVar15 + 0x68) = *(undefined8 *)(puVar15 + 0xa8);
    *(undefined8 *)(puVar15 + 0x50) = *(undefined8 *)(puVar15 + 0x90);
    *(undefined8 *)(puVar15 + 0x58) = *(undefined8 *)(puVar15 + 0x98);
    *(undefined4 *)(puVar15 + 0x40) = *(undefined4 *)(puVar15 + 0x80);
    *(undefined4 *)(puVar15 + 0x44) = *(undefined4 *)(puVar15 + 0x84);
    *(undefined4 *)(puVar15 + 0x48) = *(undefined4 *)(puVar15 + 0x88);
    *(undefined4 *)(puVar15 + 0x4c) = *(undefined4 *)(puVar15 + 0x8c);
  }
  *(undefined **)(puVar15 + -8) = &UNK_140bdcf08;
  iVar9 = (*param_2)(param_3,puVar15 + 0x30,0,puVar15 + 0x80);
  if (iVar9 == 0) {
    param_1[1] = *(undefined8 *)(puVar15 + 0x80);
code_r0x000140bdcf2f:
    *param_1 = 0xb;
  }
  else {
    *(undefined **)(puVar15 + -8) = &UNK_140bdcf17;
    FUN_140c84010(param_1 + 1);
    *(int *)(param_1 + 4) = iVar9;
    *param_1 = 9;
  }
  return;
}

