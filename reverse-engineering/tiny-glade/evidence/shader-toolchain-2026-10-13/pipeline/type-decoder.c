
void FUN_14147a7e0(ulonglong *param_1,ulonglong *param_2,undefined8 param_3,ulonglong *param_4)

{
  char cVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  undefined4 *puVar5;
  code *pcVar6;
  ulonglong *puVar7;
  char *pcVar8;
  ulonglong *puVar9;
  ulonglong *puVar10;
  ulonglong uVar11;
  uint *puVar12;
  ulonglong uVar13;
  ulonglong *puVar14;
  longlong lVar15;
  undefined4 uVar16;
  ulonglong *puVar17;
  ulonglong uVar18;
  char *pcVar19;
  undefined4 *puVar20;
  undefined4 in_R10D;
  ulonglong *puVar21;
  char *pcVar22;
  ulonglong *puVar23;
  ulonglong *puVar24;
  ulonglong *unaff_R14;
  ulonglong *unaff_R15;
  undefined4 extraout_XMM0_Da;
  undefined4 uVar25;
  ulonglong *puStack_e8;
  ulonglong *puStack_e0;
  ulonglong *puStack_d8;
  ulonglong *puStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  ulonglong *puStack_b8;
  ulonglong *puStack_b0;
  ulonglong *puStack_a8;
  ulonglong *puStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  ulonglong *puStack_88;
  ulonglong *puStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  ulonglong *puStack_68;
  ulonglong *puStack_60;
  ulonglong *puStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puVar5 = (undefined4 *)*param_2;
  pcVar8 = (char *)param_2[1];
  if ((ulonglong)((longlong)pcVar8 - (longlong)puVar5) < 4) {
    param_1[1] = 7;
    goto LAB_14147b17f;
  }
  uVar25 = *puVar5;
  pcVar22 = (char *)(puVar5 + 1);
  *param_2 = (ulonglong)pcVar22;
  uVar16 = SUB84(&UNK_142c65d58,0);
  switch(uVar25) {
  case 0:
    if (3 < (ulonglong)((longlong)pcVar8 - (longlong)pcVar22)) {
      uVar25 = puVar5[1];
      *param_2 = (ulonglong)(puVar5 + 2);
      switch(uVar25) {
      case 0:
      case 1:
code_r0x00014147abe4:
        *param_1 = 0x8000000000000000;
        *(undefined4 *)(param_1 + 1) = uVar25;
        *(int *)((longlong)param_1 + 0xc) = (int)pcVar8;
        return;
      case 2:
      case 3:
      case 4:
        if (3 < (ulonglong)((longlong)pcVar8 - (longlong)(puVar5 + 2))) {
          pcVar8 = (char *)(ulonglong)(uint)puVar5[2];
          *param_2 = (ulonglong)(puVar5 + 3);
          goto code_r0x00014147abe4;
        }
        break;
      default:
        param_1[1] = 1;
        goto LAB_14147b17f;
      }
    }
    param_1[1] = 7;
    goto LAB_14147b17f;
  case 1:
    uVar13 = (longlong)pcVar8 - (longlong)pcVar22;
    uVar11 = 7;
    if (3 < uVar13) {
      uVar2 = puVar5[1];
      uVar13 = (ulonglong)uVar2;
      puVar20 = puVar5 + 2;
      *param_2 = (ulonglong)puVar20;
      switch(uVar13) {
      case 0:
      case 1:
        uVar16 = (int)&UNK_142c65da4;
        goto code_r0x00014147b043;
      case 2:
      case 3:
        break;
      case 4:
        break;
      default:
        uVar11 = 1;
        goto code_r0x00014147b09c;
      }
      uVar16 = SUB84(pcVar8 + -(longlong)puVar20,0);
      if ((char *)0x3 < pcVar8 + -(longlong)puVar20) {
        uVar16 = puVar5[2];
        puVar20 = puVar5 + 3;
        *param_2 = (ulonglong)puVar20;
code_r0x00014147b043:
        if (3 < (ulonglong)((longlong)pcVar8 - (longlong)puVar20)) {
          uVar25 = *puVar20;
          *param_2 = (ulonglong)(puVar20 + 1);
          *param_1 = 0x8000000000000001;
          *(uint *)(param_1 + 1) = uVar2;
          *(undefined4 *)((longlong)param_1 + 0xc) = uVar16;
          *(undefined4 *)(param_1 + 2) = uVar25;
          return;
        }
      }
    }
code_r0x00014147b09c:
    param_1[1] = uVar11;
    *(int *)(param_1 + 2) = (int)uVar13;
    *(undefined4 *)((longlong)param_1 + 0x14) = uVar16;
    *(undefined4 *)((longlong)param_1 + 0x1c) = 0;
    goto LAB_14147b17f;
  case 2:
    uVar11 = 7;
    if ((ulonglong)((longlong)pcVar8 - (longlong)pcVar22) < 4) {
code_r0x00014147b1c6:
      uVar13 = 0;
      uVar18 = 0;
    }
    else {
      uVar2 = puVar5[1];
      uVar13 = (ulonglong)uVar2;
      puVar20 = puVar5 + 2;
      uVar18 = 0;
      *param_2 = (ulonglong)puVar20;
      switch(uVar13) {
      case 0:
      case 1:
        goto code_r0x00014147af39;
      case 2:
      case 3:
        break;
      case 4:
        break;
      default:
        goto code_r0x00014147b074;
      }
      if ((ulonglong)((longlong)pcVar8 - (longlong)puVar20) < 4) goto code_r0x00014147b1c6;
      uVar3 = puVar5[2];
      puVar20 = puVar5 + 3;
      *param_2 = (ulonglong)puVar20;
      uVar18 = (ulonglong)uVar3 << 0x20;
code_r0x00014147af39:
      if (3 < (ulonglong)((longlong)pcVar8 - (longlong)puVar20)) {
        uVar25 = *puVar20;
        *param_2 = (ulonglong)(puVar20 + 1);
        if (3 < (ulonglong)((longlong)pcVar8 - (longlong)(puVar20 + 1))) {
          uVar16 = puVar20[1];
          *param_2 = (ulonglong)(puVar20 + 2);
          if (pcVar8 != (char *)(puVar20 + 2)) {
            pcVar22 = (char *)((longlong)puVar20 + 9);
            cVar1 = *(char *)(puVar20 + 2);
            *param_2 = (ulonglong)pcVar22;
            if (cVar1 == '\0') {
              uVar13 = 2;
            }
            else {
              if ((ulonglong)((longlong)pcVar8 - (longlong)pcVar22) < 4) goto code_r0x00014147b1c6;
              iVar4 = *(int *)((longlong)puVar20 + 9);
              pcVar22 = (char *)((longlong)puVar20 + 0xd);
              *param_2 = (ulonglong)pcVar22;
              uVar13 = 0;
              if (iVar4 != 0) {
                if (iVar4 != 1) {
code_r0x00014147b074:
                  uVar18 = 0;
                  uVar13 = 0;
                  uVar11 = 1;
                  goto code_r0x00014147b079;
                }
                uVar13 = 1;
              }
            }
            if (pcVar8 != pcVar22) {
              cVar1 = *pcVar22;
              *param_2 = (ulonglong)(pcVar22 + 1);
              if (cVar1 == '\0') {
                uVar11 = 0;
code_r0x00014147aff0:
                *param_1 = 0x8000000000000002;
                param_1[1] = uVar11;
                param_1[2] = (ulonglong)pcVar8;
                param_1[3] = uVar18 | uVar2;
                param_1[4] = CONCAT44(uVar16,uVar25);
                param_1[5] = uVar13;
                return;
              }
              if (7 < (ulonglong)((longlong)pcVar8 - (longlong)(pcVar22 + 1))) {
                pcVar8 = *(char **)(pcVar22 + 1);
                *param_2 = (ulonglong)(pcVar22 + 9);
                uVar11 = 1;
                goto code_r0x00014147aff0;
              }
            }
          }
        }
        goto code_r0x00014147b1c6;
      }
    }
code_r0x00014147b079:
    param_1[1] = uVar11;
    param_1[2] = uVar13 | uVar18;
    goto LAB_14147b17f;
  case 3:
    uVar11 = 0x8000000000000003;
    break;
  case 4:
    uVar11 = 0x8000000000000004;
    break;
  case 5:
    uVar11 = 0x8000000000000005;
    break;
  case 6:
    uVar11 = 0x8000000000000006;
    break;
  case 7:
    uVar11 = 0x8000000000000007;
    break;
  case 8:
    uVar11 = 0x8000000000000008;
    break;
  case 9:
    FUN_14147a7e0(&puStack_e8,param_2);
    if (puStack_e8 == (ulonglong *)0x800000000000000e) {
      if (puStack_e0 != (ulonglong *)0x10) {
        uVar25 = (undefined4)((ulonglong)puStack_d8 >> 0x20);
        goto code_r0x00014147a909;
      }
    }
    else {
      uStack_78 = uStack_c8;
      uStack_74 = uStack_c4;
      uStack_70 = uStack_c0;
      uStack_6c = uStack_bc;
      uStack_98 = puStack_e8;
      uStack_90 = puStack_e0;
      puStack_88 = puStack_d8;
      puStack_80 = puStack_d0;
      puStack_d8 = (ulonglong *)func_0x000140613c10(0x30,8);
      if (puStack_d8 == (ulonglong *)0x0) {
        FUN_1428d9000(8,0x30);
        goto code_r0x00014147b2e1;
      }
      puStack_d8[4] = CONCAT44(uStack_74,uStack_78);
      puStack_d8[5] = CONCAT44(uStack_6c,uStack_70);
      puStack_d8[2] = (ulonglong)puStack_88;
      puStack_d8[3] = (ulonglong)puStack_80;
      *(int *)puStack_d8 = (int)uStack_98;
      *(undefined4 *)((longlong)puStack_d8 + 4) = uStack_98._4_4_;
      *(undefined4 *)(puStack_d8 + 1) = (undefined4)uStack_90;
      *(undefined4 *)((longlong)puStack_d8 + 0xc) = uStack_90._4_4_;
    }
    pcVar8 = (char *)*param_2;
    pcVar22 = (char *)param_2[1];
    if (pcVar22 != pcVar8) {
      pcVar19 = pcVar8 + 1;
      cVar1 = *pcVar8;
      *param_2 = (ulonglong)pcVar19;
      if (cVar1 == '\0') {
        uVar25 = 0;
      }
      else {
        if ((ulonglong)((longlong)pcVar22 - (longlong)pcVar19) < 4) goto code_r0x00014147b1d0;
        in_R10D = *(undefined4 *)(pcVar8 + 1);
        pcVar19 = pcVar8 + 5;
        *param_2 = (ulonglong)pcVar19;
        uVar25 = 1;
      }
      if (pcVar22 != pcVar19) {
        cVar1 = *pcVar19;
        *param_2 = (ulonglong)(pcVar19 + 1);
        if (cVar1 == '\0') {
          uVar11 = 0;
code_r0x00014147ac3c:
          *param_1 = 0x8000000000000009;
          param_1[1] = uVar11;
          param_1[2] = (ulonglong)pcVar22;
          *(undefined4 *)(param_1 + 3) = uVar25;
          *(undefined4 *)((longlong)param_1 + 0x1c) = in_R10D;
          param_1[4] = (ulonglong)puStack_d8;
          return;
        }
        if (7 < (ulonglong)((longlong)pcVar22 - (longlong)(pcVar19 + 1))) {
          pcVar22 = *(char **)(pcVar19 + 1);
          *param_2 = (ulonglong)(pcVar19 + 9);
          uVar11 = 1;
          goto code_r0x00014147ac3c;
        }
      }
    }
code_r0x00014147b1d0:
    uVar11 = 10;
    if ((*puStack_d8 ^ 0x8000000000000000) < 0xe) {
      uVar11 = *puStack_d8 ^ 0x8000000000000000;
    }
    puStack_58 = puStack_d8;
    if (uVar11 < 0xd) {
      if ((0x19ffUL >> (uVar11 & 0x3f) & 1) == 0) {
        if (uVar11 == 9) {
          FUN_140892f20(puStack_d8 + 4);
        }
        else {
          FUN_14088b180(puStack_d8);
        }
      }
    }
    else {
      FUN_140892f20(puStack_d8 + 1);
    }
    puStack_d8 = (ulonglong *)func_0x000140613c20(puStack_58,0x30,8);
    puStack_e0 = (ulonglong *)0x7;
    uVar25 = 0;
code_r0x00014147a909:
    param_1[1] = (ulonglong)puStack_e0;
    *(int *)(param_1 + 2) = (int)puStack_d8;
    *(undefined4 *)((longlong)param_1 + 0x14) = uVar25;
    param_1[3] = (ulonglong)puStack_d0;
    *param_1 = 0x800000000000000e;
    return;
  case 10:
    puVar21 = (ulonglong *)0x7;
    if (pcVar8 != pcVar22) {
      puVar12 = (uint *)((longlong)puVar5 + 5);
      cVar1 = *(char *)(puVar5 + 1);
      *param_2 = (ulonglong)puVar12;
      if (cVar1 == '\0') {
        puVar9 = (ulonglong *)0x8000000000000000;
code_r0x00014147acda:
        if ((ulonglong)((longlong)pcVar8 - (longlong)puVar12) < 4) {
          puVar21 = (ulonglong *)0x7;
        }
        else {
          uVar2 = *puVar12;
          unaff_R14 = (ulonglong *)(ulonglong)uVar2;
          *param_2 = (ulonglong)(puVar12 + 1);
          puStack_60 = puVar9;
          if ((ulonglong)((longlong)unaff_R14 * 0xd) <=
              (ulonglong)((longlong)pcVar8 - (longlong)(puVar12 + 1))) {
            if (uVar2 == 0) {
              puVar9 = (ulonglong *)0x8;
code_r0x00014147aef7:
              param_1[4] = (ulonglong)puStack_a0;
              param_1[5] = (ulonglong)puStack_60;
              *param_1 = (ulonglong)unaff_R14;
              param_1[1] = (ulonglong)puVar9;
              param_1[2] = (ulonglong)unaff_R14;
              param_1[3] = (ulonglong)puStack_60;
              return;
            }
            puVar21 = (ulonglong *)((longlong)unaff_R14 * 0x50);
            param_4 = puVar21;
            puStack_b0 = param_1;
            puVar9 = (ulonglong *)func_0x000140613c10(puVar21,8);
            if (puVar9 != (ulonglong *)0x0) {
              lVar15 = 0;
              puVar21 = param_1;
              puVar24 = puVar9;
              uVar25 = extraout_XMM0_Da;
code_r0x00014147ad70:
              puVar23 = (ulonglong *)*param_2;
              puVar10 = (ulonglong *)param_2[1];
              puStack_e8 = (ulonglong *)0x7;
              puVar7 = puStack_a8;
              if (puVar10 != puVar23) {
                puVar17 = (ulonglong *)((longlong)puVar23 + 1);
                uVar11 = *puVar23;
                *param_2 = (ulonglong)puVar17;
                if ((char)uVar11 == '\0') {
                  puVar14 = (ulonglong *)0x8000000000000000;
code_r0x00014147ae45:
                  if ((ulonglong)((longlong)puVar10 - (longlong)puVar17) < 8) {
                    puStack_e8 = (ulonglong *)0x7;
                    param_4 = unaff_R15;
joined_r0x00014147b1bb:
                    puVar7 = puVar23;
                    if (((ulonglong)puVar14 & 0x7fffffffffffffff) != 0) {
                      func_0x000140613c20(uVar25,puVar14,1);
                    }
                    goto code_r0x00014147b109;
                  }
                  uVar11 = *puVar17;
                  *param_2 = (ulonglong)(puVar17 + 1);
                  puStack_68 = puVar14;
                  puStack_58 = param_4;
                  uVar25 = FUN_14147a7e0(&uStack_98,param_2);
                  puStack_e8 = uStack_90;
                  puVar14 = puStack_68;
                  puVar23 = puStack_80;
                  param_4 = puStack_88;
                  if (uStack_98 == (ulonglong *)0x800000000000000e) goto joined_r0x00014147b1bb;
                  *puVar24 = (ulonglong)puStack_68;
                  puVar24[1] = (ulonglong)puStack_58;
                  puVar24[2] = (ulonglong)puVar21;
                  puVar24[3] = (ulonglong)uStack_98;
                  puVar24[4] = (ulonglong)uStack_90;
                  puVar24[5] = (ulonglong)puStack_88;
                  puVar24[6] = (ulonglong)puStack_80;
                  *(undefined4 *)(puVar24 + 7) = uStack_78;
                  *(undefined4 *)((longlong)puVar24 + 0x3c) = uStack_74;
                  *(undefined4 *)(puVar24 + 8) = uStack_70;
                  *(undefined4 *)((longlong)puVar24 + 0x44) = uStack_6c;
                  puVar24[9] = uVar11;
                  puVar24 = puVar24 + 10;
                  lVar15 = lVar15 + 1;
                  param_4 = puStack_58;
                  param_1 = puStack_b0;
                  unaff_R15 = puStack_88;
                  uVar25 = uStack_78;
                  if (puVar9 + (longlong)unaff_R14 * 10 <= puVar24) goto code_r0x00014147aef7;
                  goto code_r0x00014147ad70;
                }
                if (3 < (ulonglong)((longlong)puVar10 - (longlong)puVar17)) {
                  uVar3 = *(uint *)((longlong)puVar23 + 1);
                  unaff_R15 = (ulonglong *)(ulonglong)uVar3;
                  uVar11 = (longlong)puVar23 + 5;
                  *param_2 = uVar11;
                  if ((ulonglong *)((longlong)puVar10 - uVar11) < unaff_R15)
                  goto code_r0x00014147b109;
                  if (uVar3 == 0) {
                    puVar21 = (ulonglong *)0x1;
                  }
                  else {
                    puVar21 = (ulonglong *)func_0x000140613c10(unaff_R15,1);
                    if (puVar21 == (ulonglong *)0x0) {
                      uStack_49 = 1;
                      FUN_1428d8fe3(1,unaff_R15,&UNK_142c65160);
                      goto code_r0x00014147b2e1;
                    }
                  }
                  func_0x000142923c00(puVar21,uVar11,unaff_R15);
                  puVar23 = (ulonglong *)(uVar11 + (longlong)unaff_R15);
                  *param_2 = (ulonglong)puVar23;
                  puStack_68 = unaff_R15;
                  puStack_58 = puVar21;
                  uVar25 = FUN_140d88f60(&uStack_98,puVar21,unaff_R15);
                  if ((int)uStack_98 != 1) {
                    puVar17 = (ulonglong *)*param_2;
                    puVar10 = (ulonglong *)param_2[1];
                    puStack_a8 = puStack_68;
                    puStack_b8 = puStack_58;
                    param_4 = puStack_58;
                    puVar14 = puStack_68;
                    puVar21 = puStack_68;
                    goto code_r0x00014147ae45;
                  }
                  if ((int)puStack_68 != 0) {
                    func_0x000140613c20(puStack_58,puStack_68,1);
                  }
                  puStack_e8 = (ulonglong *)0x2;
                  param_4 = puStack_b8;
                  puVar7 = puStack_a8;
                }
              }
code_r0x00014147b109:
              puStack_a8 = puVar7;
              unaff_R14 = puStack_a8;
              puStack_d8 = puStack_a8;
              puStack_88 = (ulonglong *)0x0;
              uStack_98 = (ulonglong *)(ulonglong)uVar2;
              puStack_e0 = param_4;
              uStack_90 = puVar9;
              puStack_58 = puStack_e8;
              FUN_1428fc010(&uStack_98,lVar15);
              puVar21 = puStack_58;
              puVar9 = puStack_60;
              param_1 = puStack_b0;
              goto joined_r0x00014147b242;
            }
            FUN_1428d8fe3(8,puVar21,&UNK_142c65160);
code_r0x00014147b2e1:
                    /* WARNING: Does not return */
            pcVar6 = (code *)invalidInstructionException();
            (*pcVar6)();
          }
          puStack_58 = (ulonglong *)0x7;
          puVar21 = puStack_58;
          param_4 = param_2;
        }
joined_r0x00014147b242:
        if (((ulonglong)puVar9 & 0x7fffffffffffffff) != 0) {
          func_0x000140613c20(puStack_a0,puVar9,1);
        }
      }
      else if (3 < (ulonglong)((longlong)pcVar8 - (longlong)puVar12)) {
        uVar2 = *(uint *)((longlong)puVar5 + 5);
        unaff_R14 = (ulonglong *)(ulonglong)uVar2;
        uVar11 = (longlong)puVar5 + 9;
        *param_2 = uVar11;
        if (unaff_R14 <= pcVar8 + -uVar11) {
          if (uVar2 == 0) {
            unaff_R15 = (ulonglong *)0x1;
          }
          else {
            unaff_R15 = (ulonglong *)func_0x000140613c10(unaff_R14,1);
            if (unaff_R15 == (ulonglong *)0x0) {
              FUN_1428d8fe3(1,unaff_R14,&UNK_142c65160);
              pcVar6 = (code *)swi(3);
              (*pcVar6)();
              return;
            }
          }
          func_0x000142923c00(unaff_R15,uVar11,unaff_R14);
          *param_2 = uVar11 + (longlong)unaff_R14;
          puStack_60 = unaff_R14;
          puStack_58 = unaff_R15;
          FUN_140d88f60(&uStack_98,unaff_R15,unaff_R14);
          if ((int)uStack_98 != 1) {
            puVar12 = (uint *)*param_2;
            pcVar8 = (char *)param_2[1];
            puStack_a0 = puStack_58;
            puVar9 = puStack_60;
            goto code_r0x00014147acda;
          }
          if ((int)puStack_60 != 0) {
            func_0x000140613c20(puStack_58,puStack_60,1);
          }
          puVar21 = (ulonglong *)0x2;
          param_4 = puStack_58;
        }
      }
    }
    param_1[1] = (ulonglong)puVar21;
    param_1[2] = (ulonglong)param_4;
    param_1[3] = (ulonglong)unaff_R14;
    goto LAB_14147b17f;
  case 0xb:
    uVar11 = 0x800000000000000b;
    break;
  case 0xc:
    uVar11 = 0x800000000000000c;
    break;
  case 0xd:
    FUN_14147a7e0(&puStack_e8,param_2);
    if (puStack_e8 == (ulonglong *)0x800000000000000e) {
      if (puStack_e0 != (ulonglong *)0x10) {
        param_1[1] = (ulonglong)puStack_e0;
        param_1[2] = (ulonglong)puStack_d8;
        param_1[3] = (ulonglong)puStack_d0;
        *param_1 = 0x800000000000000e;
        return;
      }
    }
    else {
      uStack_78 = uStack_c8;
      uStack_74 = uStack_c4;
      uStack_70 = uStack_c0;
      uStack_6c = uStack_bc;
      uStack_98 = puStack_e8;
      uStack_90 = puStack_e0;
      puStack_88 = puStack_d8;
      puStack_80 = puStack_d0;
      puStack_d8 = (ulonglong *)func_0x000140613c10(0x30,8);
      if (puStack_d8 == (ulonglong *)0x0) {
        FUN_1428d9000(8,0x30);
        goto code_r0x00014147b2e1;
      }
      puStack_d8[4] = CONCAT44(uStack_74,uStack_78);
      puStack_d8[5] = CONCAT44(uStack_6c,uStack_70);
      puStack_d8[2] = (ulonglong)puStack_88;
      puStack_d8[3] = (ulonglong)puStack_80;
      *puStack_d8 = (ulonglong)uStack_98;
      puStack_d8[1] = (ulonglong)uStack_90;
    }
    *param_1 = 0x800000000000000d;
    param_1[1] = (ulonglong)puStack_d8;
    return;
  default:
    param_1[1] = 1;
LAB_14147b17f:
    uVar11 = 0x800000000000000e;
  }
  *param_1 = uVar11;
  return;
}

