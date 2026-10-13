// 0x1409f1ab0 make_animations_instant

// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000142ac5c72) overlaps instruction at (ram,0x000142ac5c70)
// 

void __cdecl
country_core::resources::history::replay_cmd_buffer::ReplayBuffer::make_animations_instant(void)

{
  ulonglong uVar1;
  code *pcVar2;
  undefined3 uVar3;
  char cVar4;
  byte bVar5;
  byte bVar6;
  uint uVar7;
  int *piVar8;
  int *piVar9;
  undefined7 uVar12;
  uint *puVar11;
  ulonglong *in_RCX;
  int *piVar13;
  int *piVar14;
  int *piVar15;
  char cVar16;
  longlong lVar17;
  char *unaff_RBX;
  ulonglong uVar18;
  longlong unaff_RBP;
  char *pcVar19;
  char *pcVar20;
  char *unaff_RDI;
  char *pcVar21;
  char *pcVar22;
  ulonglong uVar23;
  int *piVar24;
  longlong lVar25;
  byte *pbVar26;
  int *piVar27;
  byte *pbVar28;
  char *pcVar29;
  longlong lVar30;
  bool bVar31;
  bool bVar32;
  char *pcVar10;
  
  uVar1 = in_RCX[3];
  if (uVar1 == 0) {
    lVar17 = 0;
    uVar23 = 0;
    lVar25 = 0;
  }
  else {
    uVar23 = *in_RCX;
    uVar18 = 0;
    if (uVar23 <= in_RCX[2]) {
      uVar18 = uVar23;
    }
    lVar17 = in_RCX[2] - uVar18;
    lVar25 = uVar1 - (uVar23 - lVar17);
    if (uVar1 < uVar23 - lVar17 || lVar25 == 0) {
      uVar23 = uVar1 + lVar17;
      lVar25 = 0;
    }
  }
  piVar13 = (int *)in_RCX[1];
  piVar15 = piVar13 + lVar17 * 0x10;
  piVar24 = piVar13 + lVar25 * 0x10;
  piVar8 = piVar13 + uVar23 * 0x10;
LAB_1409f1b23:
  do {
    piVar9 = piVar8;
    piVar14 = piVar13;
    piVar27 = piVar15;
    if ((piVar15 == piVar8) &&
       ((piVar13 == piVar24 ||
        (piVar9 = piVar24, piVar14 = piVar8, piVar24 = piVar8, piVar27 = piVar13,
        piVar13 == (int *)0x0)))) {
      return;
    }
    piVar15 = piVar27 + 0x10;
    piVar8 = piVar9;
    piVar13 = piVar14;
  } while (*(longlong *)(piVar27 + 4) == 0);
  pbVar26 = *(byte **)(piVar27 + 2);
  pbVar28 = pbVar26 + *(longlong *)(piVar27 + 4) * 0x30;
LAB_1409f1b81:
  lVar17 = (longlong)*(int *)(&DAT_142ac5c90 + (ulonglong)*pbVar26 * 4);
  pcVar19 = &DAT_142ac5c90 + lVar17;
  bVar32 = pcVar19 == (char *)0x0;
  bVar31 = (POPCOUNT((ulonglong)pcVar19 & 0xff) & 1U) == 0;
  cVar16 = (char)piVar15;
  cVar4 = (char)piVar9;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*pbVar26) {
  case 0:
  case 1:
  case 2:
    pbVar26[0x1c] = 0;
    pbVar26[0x1d] = 0;
    pbVar26[0x1e] = 0;
    pbVar26[0x1f] = 0;
    goto code_r0x0001409f1b78;
  case 3:
    pbVar26[0x14] = 0;
    pbVar26[0x15] = 0;
    pbVar26[0x16] = 0;
    pbVar26[0x17] = 0;
    goto code_r0x0001409f1b78;
  case 4:
    uVar1 = *(ulonglong *)(pbVar26 + 0x20);
    if (uVar1 == 0) {
      lVar17 = 0;
      uVar23 = 0;
code_r0x0001409f1bdd:
      lVar25 = 0;
    }
    else {
      uVar23 = *(ulonglong *)(pbVar26 + 8);
      uVar18 = 0;
      if (uVar23 <= *(ulonglong *)(pbVar26 + 0x18)) {
        uVar18 = uVar23;
      }
      lVar17 = *(ulonglong *)(pbVar26 + 0x18) - uVar18;
      lVar25 = uVar1 - (uVar23 - lVar17);
      if (uVar1 < uVar23 - lVar17 || lVar25 == 0) {
        uVar23 = uVar1 + lVar17;
        goto code_r0x0001409f1bdd;
      }
    }
    pcVar10 = *(char **)(pbVar26 + 0x10);
    unaff_RBX = pcVar10 + lVar17 * 0x18;
    pcVar19 = pcVar10 + lVar25 * 0x18;
    pcVar21 = pcVar10 + uVar23 * 0x18;
    while ((pcVar20 = pcVar10, pcVar22 = pcVar21, pcVar29 = unaff_RBX, unaff_RBX != pcVar21 ||
           ((unaff_RDI = pcVar21, pcVar10 != pcVar19 &&
            (pcVar20 = pcVar21, unaff_RDI = pcVar19, pcVar22 = pcVar19, pcVar29 = pcVar10,
            pcVar19 = pcVar21, pcVar10 != (char *)0x0))))) {
      unaff_RBX = pcVar29 + 0x18;
      lVar17 = *(longlong *)(pcVar29 + 0x10);
      pcVar10 = pcVar20;
      pcVar21 = pcVar22;
      if (lVar17 != 0) {
        lVar25 = *(longlong *)(pcVar29 + 8);
        lVar30 = 0;
        do {
          *(undefined4 *)(lVar25 + lVar30) = 0;
          lVar30 = lVar30 + 0x18;
        } while (lVar17 * 0x18 != lVar30);
      }
    }
    goto code_r0x0001409f1b78;
  default:
    pbVar26[8] = 0;
    pbVar26[9] = 0;
    pbVar26[10] = 0;
    pbVar26[0xb] = 0;
code_r0x0001409f1b78:
    pbVar26 = pbVar26 + 0x30;
    if (pbVar26 == pbVar28) goto LAB_1409f1b23;
    goto LAB_1409f1b81;
  case 0xb:
  case 0xc:
  case 0x11:
  case 0x30:
  case 0x33:
    *(char *)(lVar17 + 0x101ab5a82) = *(char *)(lVar17 + 0x101ab5a82) + '\x01';
  case 0x14:
  case 0x15:
code_r0x000142ac5c96:
code_r0x000142ac5c98:
    pcVar19[-0x40c1020e] = pcVar19[-0x40c1020e] + '\x01';
code_r0x000142ac5c9e:
code_r0x000142ac5ca0:
    pbVar26 = (byte *)(unaff_RDI + -0x411f020e);
    *pbVar26 = *pbVar26 | (byte)((ulonglong)unaff_RBX >> 8);
    bVar32 = *pbVar26 == 0;
    bVar31 = (POPCOUNT(*pbVar26) & 1U) == 0;
    break;
  case 0xd:
  case 0x32:
    pcVar19 = (char *)0xbefefdf2;
    goto code_r0x000142ac5c96;
  case 0xe:
  case 0xf:
  case 0x31:
  case 0x12:
  case 0x17:
code_r0x000142ac5c94:
    pcVar10 = (char *)(lVar17 + 0x101ab5a82);
    *pcVar10 = *pcVar10 + '\x01';
    bVar32 = *pcVar10 == '\0';
    bVar31 = (POPCOUNT(*pcVar10) & 1U) == 0;
code_r0x000142ac5c9a:
code_r0x000142ac5c9c:
    unaff_RDI = (char *)0xbf08fdf2;
code_r0x000142ac5ca2:
code_r0x000142ac5ca4:
    piVar14 = (int *)((longlong)piVar14 + -1);
    if (piVar14 != (int *)0x0 && !bVar32) {
      halt_baddata();
    }
    break;
  case 0x10:
    goto code_r0x000142ac5c94;
  case 0x13:
    pcVar19 = (char *)0xbefefdf2;
    goto code_r0x000142ac5c9a;
  case 0x16:
  case 0x18:
  case 0x1d:
    goto code_r0x000142ac5c98;
  case 0x19:
    pcVar19 = (char *)0xbf3efdf2;
    goto code_r0x000142ac5c9e;
  case 0x1a:
  case 0x1b:
    goto code_r0x000142ac5c9a;
  case 0x1c:
  case 0x1e:
  case 0x23:
    goto code_r0x000142ac5c9c;
  case 0x1f:
    unaff_RDI = (char *)0xbf08fdf2;
    goto code_r0x000142ac5ca2;
  case 0x20:
  case 0x21:
    goto code_r0x000142ac5c9e;
  case 0x22:
  case 0x24:
  case 0x29:
    goto code_r0x000142ac5ca0;
  case 0x25:
    unaff_RDI = (char *)0xbee0fdf2;
    break;
  case 0x26:
  case 0x27:
    goto code_r0x000142ac5ca2;
  case 0x28:
  case 0x2a:
  case 0x2f:
    goto code_r0x000142ac5ca4;
  case 0x2b:
    pcVar19 = (char *)0xbee0fdf2;
    goto code_r0x000142ac5caa;
  case 0x2c:
  case 0x2d:
    break;
  case 0x2e:
  }
  piVar15 = (int *)((longlong)piVar14 + -1);
  piVar14 = piVar15;
  if (piVar15 == (int *)0x0 || bVar32) {
code_r0x000142ac5caa:
    if (piVar14 != (int *)0x1 && !bVar32) {
      halt_baddata();
    }
    if (piVar14 != (int *)0x2 && !bVar32) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    piVar15 = (int *)((longlong)piVar14 + -3);
    if (piVar15 == (int *)0x0 || bVar32) {
      if (piVar14 + -1 != (int *)0x0 && !bVar32) {
        halt_baddata();
      }
      *(char *)piVar9 = (char)*piVar9 + cVar4;
      *(char *)piVar9 = (char)*piVar9 + cVar4;
      *(char *)piVar9 = (char)*piVar9 + cVar4;
      *(char *)piVar9 = (char)*piVar9 + cVar4;
      *piVar9 = *piVar9 + (int)piVar9;
      *(char *)piVar9 = (char)*piVar9 + cVar4;
      uVar12 = (undefined7)((ulonglong)piVar9 >> 8);
      cVar4 = cVar4 + (char)*piVar9;
      pcVar10 = (char *)CONCAT71(uVar12,cVar4);
      *pcVar10 = *pcVar10 + cVar4;
      piVar15 = (int *)CONCAT71(uVar12,cVar4 + *pcVar10);
      *(char *)piVar15 = (char)*piVar15 + cVar4 + *pcVar10;
      uVar7 = (int)piVar15 + *piVar15;
      pcVar10 = (char *)(ulonglong)uVar7;
      cVar4 = (char)uVar7;
      *pcVar10 = *pcVar10 + cVar4;
      *pcVar10 = *pcVar10 + cVar4;
      *pcVar10 = *pcVar10 + cVar4;
      *(char *)(ulonglong)uVar7 = *(char *)(ulonglong)uVar7 + cVar4;
      uVar7 = uVar7 + 0x6000000;
      puVar11 = (uint *)(ulonglong)uVar7;
      bVar5 = (byte)uVar7;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      *pcVar19 = *pcVar19 + bVar5;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      *unaff_RDI = *unaff_RDI + bVar5;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      *(char *)((longlong)puVar11 * 2) = *(char *)((longlong)puVar11 * 2) + bVar5;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      *(byte *)puVar11 = (byte)*puVar11 | bVar5;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      *puVar11 = *puVar11 | uVar7;
      *(byte *)puVar11 = (byte)*puVar11 + bVar5;
      uVar3 = (undefined3)(uVar7 >> 8);
      bVar5 = bVar5 | (byte)*puVar11;
      pbVar26 = (byte *)(ulonglong)CONCAT31(uVar3,bVar5);
      *pbVar26 = *pbVar26 + bVar5;
      uVar7 = CONCAT31(uVar3,bVar5 | *pbVar26);
      puVar11 = (uint *)(ulonglong)uVar7;
      *(byte *)puVar11 = (char)*puVar11 + (bVar5 | *pbVar26);
      uVar7 = uVar7 | *puVar11;
      pbVar26 = (byte *)(ulonglong)uVar7;
      bVar6 = (byte)uVar7;
      *pbVar26 = *pbVar26 + bVar6;
      *pbVar26 = *pbVar26 | bVar6;
      *pbVar26 = *pbVar26 + bVar6;
      *(char *)(ulonglong)uVar7 = *(char *)(ulonglong)uVar7 + bVar6;
      uVar7 = uVar7 | 0xe000000;
      pbVar26 = (byte *)(ulonglong)uVar7;
      *pbVar26 = *pbVar26 + bVar6;
      cVar4 = (char)(piVar14 + -1);
      *pcVar19 = *pcVar19 + cVar4;
      *pbVar26 = *pbVar26 + bVar6;
      *unaff_RDI = *unaff_RDI + cVar4;
      *pbVar26 = *pbVar26 + bVar6;
      *(char *)((longlong)pbVar26 * 2) = *(char *)((longlong)pbVar26 * 2) + cVar4;
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      *pbVar26 = *pbVar26 + bVar6 + CARRY1(bVar5,bVar6);
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      *(uint *)pbVar26 = *(int *)pbVar26 + uVar7 + (uint)CARRY1(bVar5,bVar6);
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      uVar3 = (undefined3)(uVar7 >> 8);
      bVar6 = bVar6 + *pbVar26 + CARRY1(bVar5,bVar6);
      pbVar26 = (byte *)(ulonglong)CONCAT31(uVar3,bVar6);
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      bVar6 = bVar6 + *pbVar26 + CARRY1(bVar5,bVar6);
      uVar7 = CONCAT31(uVar3,bVar6);
      pbVar26 = (byte *)(ulonglong)uVar7;
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      uVar7 = uVar7 + *(int *)pbVar26 + (uint)CARRY1(bVar5,bVar6);
      pbVar26 = (byte *)(ulonglong)uVar7;
      bVar5 = *pbVar26;
      bVar6 = (byte)uVar7;
      *pbVar26 = *pbVar26 + bVar6;
      *pbVar26 = *pbVar26 + bVar6 + CARRY1(bVar5,bVar6);
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      bVar6 = bVar6 + CARRY1(bVar5,bVar6);
      uVar7 = CONCAT31((int3)(uVar7 >> 8),bVar6);
      pbVar26 = (byte *)(ulonglong)uVar7;
      bVar5 = *pbVar26;
      *pbVar26 = *pbVar26 + bVar6;
      uVar7 = uVar7 + 0x16000000 + (uint)CARRY1(bVar5,bVar6);
      pcVar10 = (char *)(ulonglong)uVar7;
      cVar4 = (char)uVar7;
      *pcVar10 = *pcVar10 + cVar4;
      *pcVar19 = *pcVar19 + cVar16;
      *pcVar10 = *pcVar10 + cVar4;
      *unaff_RDI = *unaff_RDI + cVar16;
      *pcVar10 = *pcVar10 + cVar4;
      *(char *)((longlong)pcVar10 * 2) = *(char *)((longlong)pcVar10 * 2) + cVar16;
      *pcVar10 = *pcVar10 + cVar4;
      *pcVar10 = *pcVar10 + cVar4;
      *pcVar10 = *pcVar10 + cVar4;
      cVar4 = cVar4 + *pcVar10;
      uVar7 = CONCAT31((int3)(uVar7 >> 8),cVar4);
      piVar15 = (int *)(ulonglong)uVar7;
      *(char *)piVar15 = (char)*piVar15 + cVar4;
      *piVar15 = *piVar15 + uVar7;
      *(char *)piVar15 = (char)*piVar15 + cVar4;
      *(char *)piVar15 = (char)*piVar15 + cVar4;
      *(char *)piVar15 = (char)*piVar15 + cVar4;
      pcVar2 = (code *)swi(0xcc);
      (*pcVar2)();
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
  }
  if (bVar31) {
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  *(char *)piVar9 = (char)*piVar9 + cVar4;
  *pcVar19 = *pcVar19 + (char)piVar15;
  *(char *)piVar9 = (char)*piVar9 + cVar4;
  *(char *)(unaff_RBP + 0x70) = *(char *)(unaff_RBP + 0x70) + cVar16;
                    // WARNING: Bad instruction - Truncating control flow here
  halt_baddata();
}

