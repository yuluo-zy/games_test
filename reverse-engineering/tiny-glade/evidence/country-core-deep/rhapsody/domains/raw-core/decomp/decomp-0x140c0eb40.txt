
undefined4 * FUN_140c0eb40(undefined4 *param_1,undefined8 *param_2,undefined *param_3)

{
  byte *pbVar1;
  int *piVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined1 uVar6;
  ushort uVar7;
  undefined4 uVar8;
  code *pcVar9;
  char cVar10;
  uint uVar11;
  longlong *plVar12;
  undefined *puVar13;
  undefined8 unaff_R14;
  undefined *puVar14;
  longlong lVar15;
  ulonglong uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  undefined8 uStack_68;
  longlong *plStack_50;
  longlong *plStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  cVar10 = func_0x000140c16a00();
  if (cVar10 == '\0') {
    uStack_70 = 0;
code_r0x000140c0ecdb:
    puVar13 = *(undefined **)(param_3 + 0x648);
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar10);
    if (*(short *)((longlong)param_2 + 0x14) != *(short *)(puVar13 + 0xa50)) {
code_r0x000140c0ee43:
      uStack_c8 = 0;
      FUN_1428ca816(0,(longlong)param_2 + 0x14,puVar13 + 0xa50,&uStack_c8,&UNK_142d022a0);
      goto code_r0x000140c0eeb9;
    }
    if (*(char *)((longlong)param_2 + 0x16) == '\x02') {
      if (*(int *)(param_2[1] + 0xc) != *(int *)(param_2 + 2)) {
        FUN_1428d9310(&UNK_142b19c70);
        goto code_r0x000140c0eeb9;
      }
      FUN_1418cb520(&uStack_c8,param_2[1] + 0x10,param_3);
      if ((uStack_c8 & 1) == 0) {
        param_1[0x16] = uStack_90;
        param_1[0x17] = uStack_8c;
        param_1[0x18] = uStack_88;
        param_1[0x19] = uStack_84;
        param_1[0x12] = uStack_a0;
        param_1[0x13] = uStack_9c;
        param_1[0x14] = uStack_98;
        param_1[0x15] = uStack_94;
        param_1[0xe] = uStack_b0;
        param_1[0xf] = uStack_ac;
        param_1[0x10] = uStack_a8;
        param_1[0x11] = uStack_a4;
        *(ulonglong *)(param_1 + 10) = CONCAT44(uStack_bc,uStack_c0);
        *(ulonglong *)(param_1 + 0xc) = CONCAT44(uStack_b4,uStack_b8);
        lVar15 = param_2[1];
        LOCK();
        piVar2 = (int *)(lVar15 + 8);
        *piVar2 = *piVar2 + 1;
        UNLOCK();
        uVar8 = *(undefined4 *)(param_2 + 2);
        uVar6 = *(undefined1 *)((longlong)param_2 + 0x16);
        uVar7 = *(ushort *)((longlong)param_2 + 0x14);
        *(undefined8 *)(param_1 + 4) = *param_2;
        *(longlong *)(param_1 + 6) = lVar15;
        param_1[8] = uVar8;
        *(ushort *)(param_1 + 9) = uVar7;
        *(undefined1 *)((longlong)param_1 + 0x26) = uVar6;
        *param_1 = uStack_80;
        param_1[1] = uStack_7c;
        param_1[2] = uStack_78;
        param_1[3] = uStack_74;
        puVar13 = (undefined *)(ulonglong)uVar7;
      }
      else {
        param_1[0xc] = uStack_90;
        param_1[0xd] = uStack_8c;
        param_1[0xe] = uStack_88;
        param_1[0xf] = uStack_84;
        param_1[8] = uStack_a0;
        param_1[9] = uStack_9c;
        param_1[10] = uStack_98;
        param_1[0xb] = uStack_94;
        param_1[4] = uStack_b0;
        param_1[5] = uStack_ac;
        param_1[6] = uStack_a8;
        param_1[7] = uStack_a4;
        *param_1 = uStack_c0;
        param_1[1] = uStack_bc;
        param_1[2] = uStack_b8;
        param_1[3] = uStack_b4;
        *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
        puVar13 = param_3;
      }
      if ((char)plStack_48 == '\0') {
        return param_1;
      }
      plVar12 = (longlong *)func_0x000140c18130();
      if (*plVar12 == 1) {
        plVar12 = plVar12 + 1;
      }
      else if ((*plVar12 != 0) ||
              (plVar12 = (longlong *)FUN_1429090d0(plVar12,0), plVar12 == (longlong *)0x0))
      goto code_r0x000140c0ee37;
      if (*plVar12 == 0) {
        *plVar12 = -1;
        plStack_48 = plVar12;
        FUN_140c170f0(plVar12 + 1,unaff_R14);
        *plStack_48 = *plStack_48 + 1;
        return param_1;
      }
      goto code_r0x000140c0ee72;
    }
  }
  else {
    puVar13 = &UNK_142b19bdc;
    puVar14 = &UNK_142b19bb8;
    do {
      bVar3 = puVar13[-1];
      uVar11 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar13[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar13[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar11 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar11 = bVar5 & 0xf;
          }
          uVar11 = bVar4 & 0x3f | uVar11 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar11 = bVar4 & 0x1f;
        }
        uVar11 = (uint)(bVar3 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto code_r0x000140c0ec04;
code_r0x000140c0ec1d:
        puVar13 = puVar13 + -0x142b19bb7;
        if (puVar13 != (undefined *)0x0) {
          if (puVar13 < (undefined *)0x24) goto code_r0x000140c0ec57;
          if (puVar13 != (undefined *)0x24) {
            do {
              FUN_1428d9940(&UNK_142b19bb8,0x24);
code_r0x000140c0ec57:
            } while ((char)(&UNK_142b19bb8)[(longlong)puVar13] < -0x40);
          }
        }
        lVar15 = 0x24 - (longlong)puVar13;
        puVar14 = &UNK_142b19bb8 + (longlong)puVar13;
        goto code_r0x000140c0ec6a;
      }
      puVar13 = puVar13 + -1;
      if (uVar11 == 0x5c) goto code_r0x000140c0ec1d;
code_r0x000140c0ec04:
      if (uVar11 == 0x2f) goto code_r0x000140c0ec1d;
    } while (puVar13 != &UNK_142b19bb8);
    lVar15 = 0x24;
    puVar13 = &UNK_142b19bb8;
code_r0x000140c0ec6a:
    plVar12 = (longlong *)func_0x000140c18130();
    if (*plVar12 != 1) {
      if ((*plVar12 == 0) &&
         (plVar12 = (longlong *)FUN_1429090d0(plVar12,0), plVar12 != (longlong *)0x0))
      goto code_r0x000140c0ec83;
code_r0x000140c0ee37:
      FUN_1428d80d0(&UNK_142d06498);
      goto code_r0x000140c0ee43;
    }
    plVar12 = plVar12 + 1;
code_r0x000140c0ec83:
    if (*plVar12 == 0) {
      *plVar12 = -1;
      plStack_50 = plVar12;
      unaff_R14 = FUN_140c16d40(plVar12 + 1,&UNK_142b19c58,0x17,puVar14,lVar15,1,0);
      *plStack_50 = *plStack_50 + 1;
      uStack_70 = 1;
      uStack_68 = unaff_R14;
      goto code_r0x000140c0ecdb;
    }
code_r0x000140c0ee72:
    FUN_1428d9250(&UNK_142d06508);
  }
  uStack_c8 = 0;
  FUN_1428ca902(0,(longlong)param_2 + 0x16,&UNK_142d022d8,&uStack_c8,&UNK_142d022c0);
code_r0x000140c0eeb9:
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}

