
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000142b0ff1b) overlaps instruction at (ram,0x000142b0ff1a)
    */
/* WARNING: Removing unreachable block (ram,0x000142b0feb7) */
/* WARNING: Removing unreachable block (ram,0x000142b0feb9) */

char * FUN_140bd5ed0(char *param_1,char *param_2,char *param_3)

{
  byte bVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  code *pcVar4;
  char cVar5;
  undefined4 uVar6;
  ulonglong uVar7;
  char *pcVar8;
  byte bVar9;
  char *unaff_RBX;
  undefined1 *puVar10;
  char *unaff_RSI;
  char *unaff_RDI;
  int iVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  bool bVar14;
  undefined1 auStack_78 [32];
  undefined *puStack_58;
  undefined *puStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  puVar10 = auStack_78;
  uVar7 = (ulonglong)*(int *)(&UNK_142b0ff1c + (ulonglong)(byte)param_2[0x1e] * 4);
  bVar14 = 0xfffffffebd4f00e3 < uVar7;
  pcVar8 = &UNK_142b0ff1c + uVar7;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(param_2[0x1e]) {
  case '\0':
    if (*(int *)(param_2 + 0x18) != 1) {
      puStack_50 = &UNK_142b0fe08;
      puStack_58 = &UNK_142b0fe18;
code_r0x000140bd6080:
      uStack_30 = 0;
      uStack_38 = 0;
      uStack_40 = 8;
      uStack_48 = 1;
      FUN_1428ca7db(0,&UNK_142b0fde8,param_2 + 0x18,&puStack_50);
      pcVar4 = (code *)swi(3);
      pcVar8 = (char *)(*pcVar4)();
      return pcVar8;
    }
    uVar6 = 1;
    uVar12 = 0;
    goto code_r0x000140bd5f28;
  case '\x01':
    iVar11 = *(int *)(param_2 + 0x18);
    uVar6 = 1;
    uVar12 = 0;
    goto code_r0x000140bd5f76;
  case '\x02':
    if (*(int *)(param_2 + 0x18) != 1) {
      puStack_50 = &UNK_142b0fe48;
      puStack_58 = &UNK_142b0fe58;
      goto code_r0x000140bd6080;
    }
    uVar6 = *(undefined4 *)(param_2 + 0x10);
    uVar12 = 1;
code_r0x000140bd5f28:
    uVar13 = 1;
    iVar11 = 1;
    goto code_r0x000140bd5f7c;
  case '\x03':
    uVar6 = *(undefined4 *)(param_2 + 0x10);
    iVar11 = *(int *)(param_2 + 0x18);
    break;
  case '\x04':
    uVar6 = *(undefined4 *)(param_2 + 0x10);
    uVar13 = *(undefined4 *)(param_2 + 0x14);
    iVar11 = 1;
    uVar12 = 2;
    goto code_r0x000140bd5f7c;
  case '\x05':
    if (*(int *)(param_2 + 0x18) != 1) {
      puStack_50 = &UNK_142b0fe88;
      puStack_58 = &UNK_142b0fe98;
      goto code_r0x000140bd6080;
    }
    uVar6 = *(undefined4 *)(param_2 + 0x10);
    iVar11 = 6;
    break;
  case '\x06':
    uVar6 = *(undefined4 *)(param_2 + 0x10);
    iVar11 = *(int *)(param_2 + 0x18) * 6;
    break;
  default:
    goto code_r0x000142b0ff1c;
  case '\n':
  case '\f':
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case '\r':
  case '\x0e':
  case '\x0f':
  case '\x10':
    bVar9 = (byte)param_1;
    bVar14 = CARRY1(bRam00000001a5b0feb1,bVar9);
    bRam00000001a5b0feb1 = bRam00000001a5b0feb1 + bVar9;
    if (bVar14) {
code_r0x000142b0ff14:
      *(int *)(unaff_RBX + 0xc) = *(int *)(unaff_RBX + 0xc) + (int)param_2;
      *param_2 = *param_2 + -1;
      *(char **)(puVar10 + -8) = unaff_RBX;
    }
    else if (bRam00000001a5b0feb1 != 0) {
      cVar5 = (char)pcVar8;
      *pcVar8 = *pcVar8 + cVar5;
      *param_3 = *param_3 + (char)param_3;
      *unaff_RSI = *unaff_RSI + bVar9;
      *pcVar8 = *pcVar8 + cVar5;
      *(char *)(uVar7 + 0x143f3b01a) =
           *(char *)(uVar7 + 0x143f3b01a) + (char)((ulonglong)param_2 >> 8);
      *pcVar8 = *pcVar8 + cVar5;
      *unaff_RDI = *unaff_RDI + (char)((ulonglong)pcVar8 >> 8);
      *pcVar8 = *pcVar8 + cVar5;
      *pcVar8 = *pcVar8 + cVar5;
      *pcVar8 = *pcVar8 + cVar5;
      *(char *)((longlong)pcVar8 * 2) = *(char *)((longlong)pcVar8 * 2) + (char)param_2;
      cRam0000000143b0ff09 = cRam0000000143b0ff09 + bVar9;
      *(int *)(unaff_RBX + 0xc) = *(int *)(unaff_RBX + 0xc) + (int)param_2;
      *param_1 = *param_1 + '\x01';
      puVar10 = &stack0xffffffffffffff78;
      goto code_r0x000142b0ff14;
    }
    bVar14 = false;
code_r0x000142b0ff1c:
    unaff_RDI[0xc] = (char)(CONCAT11(bVar14,unaff_RDI[0xc]) >> 1);
    *unaff_RBX = *unaff_RBX + '\x01';
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  uVar12 = 1;
code_r0x000140bd5f76:
  uVar13 = 1;
code_r0x000140bd5f7c:
  uVar2 = *(undefined4 *)param_2;
  uVar3 = *(undefined4 *)(param_2 + 4);
  bVar9 = param_2[0x1c];
  bVar1 = param_2[0x1d];
  param_1[0] = '\x0e';
  param_1[1] = '\0';
  param_1[2] = '\0';
  param_1[3] = '\0';
  param_1[8] = '\0';
  param_1[9] = '\0';
  param_1[10] = '\0';
  param_1[0xb] = '\0';
  param_1[0xc] = '\0';
  param_1[0xd] = '\0';
  param_1[0xe] = '\0';
  param_1[0xf] = '\0';
  *(undefined4 *)(param_1 + 0x10) = uVar3;
  *(undefined4 *)(param_1 + 0x14) = uVar12;
  *(undefined8 *)(param_1 + 0x18) = *(undefined8 *)(param_2 + 8);
  *(undefined4 *)(param_1 + 0x20) = uVar6;
  *(undefined4 *)(param_1 + 0x24) = uVar13;
  *(uint *)(param_1 + 0x28) = (uint)bVar9;
  *(int *)(param_1 + 0x2c) = iVar11;
  *(uint *)(param_1 + 0x30) = (uint)bVar1;
  param_1[0x34] = '\0';
  param_1[0x35] = '\0';
  param_1[0x36] = '\0';
  param_1[0x37] = '\0';
  *(undefined4 *)(param_1 + 0x38) = uVar2;
  param_1[0x3c] = '\0';
  param_1[0x3d] = '\0';
  param_1[0x3e] = '\0';
  param_1[0x3f] = '\0';
  param_1[0x40] = '\0';
  param_1[0x41] = '\0';
  param_1[0x42] = '\0';
  param_1[0x43] = '\0';
  param_1[0x48] = '\0';
  param_1[0x49] = '\0';
  param_1[0x4a] = '\0';
  param_1[0x4b] = '\0';
  param_1[0x4c] = '\0';
  param_1[0x4d] = '\0';
  param_1[0x4e] = '\0';
  param_1[0x4f] = '\0';
  *(uint *)(param_1 + 0x50) = ((uint)param_3 & 0xff) << 3;
  return param_1;
}

