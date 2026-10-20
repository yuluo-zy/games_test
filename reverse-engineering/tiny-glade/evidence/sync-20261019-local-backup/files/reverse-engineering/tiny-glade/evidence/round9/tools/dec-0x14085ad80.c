
/* WARNING: Possible PIC construction at 0x00014085aeee: Changing call to branch */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14085ad80(undefined8 *param_1,longlong *param_2)

{
  undefined8 *puVar1;
  ulonglong *puVar2;
  code *pcVar3;
  ulonglong *puVar4;
  byte *pbVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined8 *unaff_RBP;
  longlong unaff_RSI;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  ulonglong *puVar14;
  ulonglong uVar15;
  longlong lVar16;
  undefined *apuStack_e0 [5];
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  ulonglong *puStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 *puStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  longlong *plStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  puVar1 = &uStack_58;
  uStack_38 = 0xfffffffffffffffe;
  if (param_1[3] == 0) {
    if (param_1[0xf] == 0) {
      if (param_1[0x15] == 0) {
        lVar12 = param_2[2];
        if (lVar12 == 0) {
          if (*param_2 != 0) {
            lVar16 = param_2[1];
            lVar9 = *param_2 * 0x78;
            uVar6 = 8;
code_r0x000140613c20:
            *(undefined8 **)((longlong)register0x00000020 + -8) = unaff_RBP;
            *(longlong *)((longlong)register0x00000020 + -0x10) = unaff_RSI;
            lVar12 = lVar16;
            if (0x10 < uVar6) {
              lVar12 = *(longlong *)(lVar16 + -8);
            }
            *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
            uVar7 = GetProcessHeap(lVar16,lVar9);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
            HeapFree(uVar7,0,lVar12);
            return;
          }
        }
        else {
          puVar2 = (ulonglong *)param_2[1];
          apuStack_e0[0] = &UNK_14085addc;
          plStack_48 = param_2;
          pbVar5 = (byte *)FUN_140d4d740();
          if ((*pbVar5 & 1) == 0) {
            uStack_39 = 1;
            apuStack_e0[0] = &UNK_14085ae2c;
            uStack_58 = FUN_140d64e00();
            pbVar5[0] = 1;
            pbVar5[1] = 0;
            pbVar5[2] = 0;
            pbVar5[3] = 0;
            pbVar5[4] = 0;
            pbVar5[5] = 0;
            pbVar5[6] = 0;
            pbVar5[7] = 0;
            *(longlong *)(pbVar5 + 8) = uStack_58;
            *(longlong **)(pbVar5 + 0x10) = param_2;
          }
          else {
            uStack_58 = *(longlong *)(pbVar5 + 8);
            param_2 = *(longlong **)(pbVar5 + 0x10);
          }
          *(longlong *)(pbVar5 + 8) = uStack_58 + 1;
          uStack_68 = _UNK_142a94950;
          uStack_60 = _UNK_142a94958;
          uStack_78 = _UNK_142a94940;
          uStack_74 = _UNK_142a94944;
          uStack_70 = _UNK_142a94948;
          uStack_6c = _UNK_142a9494c;
          apuStack_e0[0] = &UNK_14085ae7a;
          uStack_50 = param_2;
          FUN_1428aa7a0(&uStack_78,lVar12,puVar1,1);
          lVar16 = 0;
          do {
            apuStack_e0[0] = &UNK_14085aea0;
            FUN_1402ca2f0(&uStack_78,puVar2[lVar16 * 0xf]);
            lVar16 = lVar16 + 1;
          } while (lVar16 != lVar12);
          uStack_98 = (undefined4)uStack_58;
          uStack_94 = uStack_58._4_4_;
          uStack_90 = (undefined4)uStack_50;
          uStack_8c = uStack_50._4_4_;
          lStack_a8 = uStack_68;
          puStack_a0 = (ulonglong *)uStack_60;
          uStack_b8 = (ulonglong *)CONCAT44(uStack_74,uStack_78);
          uStack_b0 = (ulonglong *)CONCAT44(uStack_6c,uStack_70);
          lVar16 = param_1[7];
          if (lVar16 != 0) {
            uVar6 = lVar16 * 8 + 0x17U & 0xfffffffffffffff0;
            lVar9 = lVar16 + uVar6 + 0x11;
            if (lVar9 != 0) {
              lVar16 = param_1[6] - uVar6;
              uVar6 = 0x10;
              register0x00000020 = (BADSPACEBASE *)apuStack_e0;
              apuStack_e0[0] = &UNK_14085aef3;
              unaff_RBP = puVar1;
              unaff_RSI = lVar12;
              goto code_r0x000140613c20;
            }
          }
          *(undefined4 *)(param_1 + 10) = (undefined4)uStack_58;
          *(undefined4 *)((longlong)param_1 + 0x54) = uStack_58._4_4_;
          *(undefined4 *)(param_1 + 0xb) = (undefined4)uStack_50;
          *(undefined4 *)((longlong)param_1 + 0x5c) = uStack_50._4_4_;
          *(int *)(param_1 + 8) = (int)uStack_68;
          *(undefined4 *)((longlong)param_1 + 0x44) = uStack_68._4_4_;
          *(int *)(param_1 + 9) = (int)uStack_60;
          *(undefined4 *)((longlong)param_1 + 0x4c) = uStack_60._4_4_;
          *(undefined4 *)(param_1 + 6) = uStack_78;
          *(undefined4 *)((longlong)param_1 + 0x34) = uStack_74;
          *(undefined4 *)(param_1 + 7) = uStack_70;
          *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_6c;
          puStack_a0 = puVar2 + lVar12 * 0xf;
          uVar6 = *puVar2;
          if (lVar12 != 1) {
            uVar11 = (lVar12 * 0x78 - 0x78U >> 3) * -0x1111111111111111;
            uVar10 = (ulonglong)((uint)uVar11 & 3);
            if (uVar11 - 1 < 3) {
              uVar13 = 0;
            }
            else {
              uVar13 = 0;
              puVar4 = puVar2;
              do {
                puVar14 = puVar4 + 0x3c;
                uVar15 = puVar4[0xf];
                if (uVar6 < uVar15) {
                  uVar8 = puVar4[0x1e];
                  uVar6 = uVar15;
                  if (uVar8 <= uVar15) goto code_r0x00014085b05f;
code_r0x00014085b03c:
                  uVar15 = puVar4[0x2d];
                  if (uVar15 <= uVar8) goto code_r0x00014085b06b;
code_r0x00014085b045:
                  uVar6 = *puVar14;
                  uVar8 = uVar15;
                  if (uVar6 <= uVar15) {
code_r0x00014085b076:
                    uVar6 = uVar8;
                  }
                }
                else {
                  uVar8 = puVar4[0x1e];
                  if (uVar6 < uVar8) goto code_r0x00014085b03c;
code_r0x00014085b05f:
                  uVar8 = uVar6;
                  uVar15 = puVar4[0x2d];
                  if (uVar8 < uVar15) goto code_r0x00014085b045;
code_r0x00014085b06b:
                  uVar6 = *puVar14;
                  if (uVar6 <= uVar8) goto code_r0x00014085b076;
                }
                uVar13 = uVar13 + 4;
                puVar4 = puVar14;
              } while ((uVar11 & 0x3fffffffffffffc) != uVar13);
            }
            if (uVar10 != 0) {
              lVar12 = 0;
              do {
                uVar11 = *(ulonglong *)((longlong)puVar2 + lVar12 + uVar13 * 0x78 + 0x78);
                if (uVar6 < uVar11) {
                  uVar6 = uVar11;
                }
                lVar12 = lVar12 + 0x78;
              } while (uVar10 * 0x78 - lVar12 != 0);
            }
          }
          param_1[0x18] = uVar6;
          lStack_a8 = *plStack_48;
          uStack_39 = 0;
          apuStack_e0[0] = &UNK_14085afaf;
          uStack_b8 = puVar2;
          uStack_b0 = puVar2;
          FUN_141449a70(&uStack_78,&uStack_b8);
          apuStack_e0[0] = &UNK_14085afcb;
          puStack_80 = param_1;
          FUN_140051870(param_1,param_1 + 4,0x78,0x10);
          puStack_80[4] = uStack_58;
          puStack_80[5] = uStack_50;
          puStack_80[2] = uStack_68;
          puStack_80[3] = uStack_60;
          *puStack_80 = CONCAT44(uStack_74,uStack_78);
          puStack_80[1] = CONCAT44(uStack_6c,uStack_70);
        }
        return;
      }
      uStack_39 = 1;
      apuStack_e0[0] = &UNK_14085b0df;
      plStack_48 = param_2;
      FUN_1428d9430(&UNK_142a94fd0,0x28,&UNK_142a94ff8);
    }
    else {
      uStack_39 = 1;
      apuStack_e0[0] = &UNK_14085b0bd;
      plStack_48 = param_2;
      FUN_1428d9430(&UNK_142a94f90,0x28,&UNK_142a94fb8);
    }
  }
  else {
    uStack_39 = 1;
    apuStack_e0[0] = &UNK_14085b09b;
    plStack_48 = param_2;
    FUN_1428d9430(&UNK_142a94f50,0x27,&UNK_142a94f78);
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

