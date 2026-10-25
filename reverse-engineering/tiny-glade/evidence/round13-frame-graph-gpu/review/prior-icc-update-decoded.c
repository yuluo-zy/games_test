void FUN_14093d830(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3)
{
  code *pcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  undefined4 uVar9;
  int iVar10;
  longlong ******pppppplVar11;
  longlong ******pppppplVar12;
  longlong ******pppppplVar13;
  longlong *******ppppppplVar14;
  longlong lVar15;
  longlong *******ppppppplVar16;
  longlong *******ppppppplVar17;
  undefined4 *puVar18;
  undefined8 *puVar19;
  longlong lVar20;
  longlong *******ppppppplVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  undefined8 uVar25;
  longlong *******ppppppplVar26;
  ulonglong uVar27;
  longlong lVar28;
  ulonglong uVar29;
  longlong *******ppppppplVar30;
  bool bVar31;
  bool bVar32;
  bool bVar33;
  bool bVar34;
  undefined8 in_stack_fffffffffffffdc8;
  undefined4 uVar35;
  ulonglong in_stack_fffffffffffffdd0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  int iStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  int iStack_1ac;
  longlong lStack_1a8;
  undefined *puStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined *puStack_188;
  undefined8 uStack_180;
  longlong ******pppppplStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  longlong ******apppppplStack_f8 [4];
  longlong ******pppppplStack_d8;
  longlong ******pppppplStack_d0;
  longlong ******pppppplStack_c8;
  longlong ******pppppplStack_c0;
  undefined4 uStack_b8;
  int iStack_b4;
  undefined4 uStack_b0;
  int iStack_ac;
  undefined4 uStack_a8;
  int iStack_a4;
  undefined4 uStack_a0;
  int iStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined8 uStack_90;
  undefined8 uStack_88;
  longlong ******pppppplStack_80;
  longlong *plStack_78;
  longlong ******pppppplStack_70;
  undefined1 uStack_61;
  longlong *plStack_60;
  longlong ******pppppplStack_58;
  longlong ******pppppplStack_50;
  undefined8 uStack_48;
  uVar35 = (undefined4)((ulonglong)in_stack_fffffffffffffdc8 >> 0x20);
  uStack_48 = 0xfffffffffffffffe;
  lVar15 = param_3[1];
  uVar24 = *(ulonglong *)*param_3;
  uVar27 = 0;
  uVar29 = uVar24 - *(ulonglong *)(lVar15 + 0x18);
  if (uVar24 < *(ulonglong *)(lVar15 + 0x18)) {
    uVar29 = uVar27;
  }
  uVar23 = uVar24 - *(ulonglong *)(lVar15 + 0x38);
  if (uVar24 < *(ulonglong *)(lVar15 + 0x38)) {
    uVar23 = uVar27;
  }
  lVar20 = uVar29 * 0x28 + *(longlong *)(lVar15 + 8);
  lVar28 = uVar23 * 0x28 + *(longlong *)(lVar15 + 0x28);
  bVar31 = *(ulonglong *)(lVar15 + 0x10) < uVar29;
  uVar29 = *(ulonglong *)(lVar15 + 0x10) - uVar29;
  bVar33 = uVar29 == 0;
  if (bVar31) {
    uVar29 = uVar27;
  }
  if (bVar31) {
    lVar20 = 8;
  }
  uVar24 = lVar20 + -0x28 + uVar29 * 0x28;
  if (bVar31 || bVar33) {
    uVar24 = uVar27;
  }
  bVar32 = *(ulonglong *)(lVar15 + 0x30) < uVar23;
  uVar23 = *(ulonglong *)(lVar15 + 0x30) - uVar23;
  bVar34 = uVar23 == 0;
  if (bVar32) {
    lVar28 = 8;
    uVar23 = uVar27;
  }
  uVar22 = lVar28 + -0x28 + uVar23 * 0x28;
  if (bVar32 || bVar34) {
    uVar22 = uVar24;
  }
  uVar24 = uVar23 + uVar29;
  if (!bVar31 && !bVar33 || !bVar32 && !bVar34) {
    uVar24 = uVar27;
  }
  *(ulonglong *)*param_3 = *(longlong *)(lVar15 + 0x40) - uVar24;
  if ((bVar31 || bVar33) && (bVar32 || bVar34)) {
    return;
  }
  ppppppplVar26 = (longlong *******)param_2[2];
  plStack_78 = (longlong *)CONCAT44(plStack_78._4_4_,*(undefined4 *)((longlong)param_2 + 0x1c));
  *(undefined4 *)ppppppplVar26 = *(undefined4 *)((longlong)param_2 + 0x1c);
  plStack_60 = (longlong *)*param_2;
  if (!SBORROW8(0,*plStack_60)) {
    pppppplStack_50 = (longlong ******)(plStack_60 + 3);
    func_0x00014003bb80();
    FUN_140be24b0(pppppplStack_50);
    param_2 = (undefined8 *)*plStack_60;
    if (param_2 != (undefined8 *)0x0) {
      func_0x000140613c20(plStack_60[1],param_2,1);
    }
  }
  *plStack_60 = -0x8000000000000000;
  ppppppplVar14 = (longlong *******)FUN_140d76ed0(uVar22);
  pppppplStack_d0 = (longlong ******)0x0;
  pppppplStack_c8 = (longlong ******)0x1;
  pppppplStack_c0 = (longlong ******)0x0;
  uStack_1d8 = (longlong *******)((ulonglong)uStack_1d8._4_4_ << 0x20);
  uStack_1d0 = (longlong *******)0x0;
  uStack_1c8 = (longlong *******)0x0;
  uStack_1c0 = 7;
  iStack_1bc = 0;
  uStack_1b4 = (uint)uStack_1b4._2_2_ << 0x10;
  uStack_1b8 = 1;
  ppppppplVar17 = ppppppplVar14;
  lVar15 = FUN_140d59dc0(&uStack_1d8,ppppppplVar14,param_2);
  ppppppplVar16 = ppppppplVar17;
  if (lVar15 == 0) {
    ppppppplVar16 = &pppppplStack_d0;
    apppppplStack_f8[0] = (longlong ******)ppppppplVar17;
    lVar15 = FUN_140d59c30(apppppplStack_f8,ppppppplVar16);
    if (lVar15 != 0) {
      CloseHandle(apppppplStack_f8[0]);
      goto code_r0x00014093dd74;
    }
    CloseHandle(apppppplStack_f8[0]);
    if ((longlong *******)pppppplStack_c0 == (longlong *******)0x0) {
      if ((longlong *******)pppppplStack_d0 != (longlong *******)0x0) {
        func_0x000140613c20(pppppplStack_c8,pppppplStack_d0,1);
      }
    }
    else {
      ppppppplVar16 = (longlong *******)FUN_14168e0b0(0,pppppplStack_c8);
      if ((longlong *******)pppppplStack_d0 != (longlong *******)0x0) {
        func_0x000140613c20(pppppplStack_c8,pppppplStack_d0,1);
      }
      if (ppppppplVar16 != (longlong *******)0x0) {
        ppppppplVar17 = (longlong *******)FUN_141739890(0);
        pppppplStack_50 = (longlong ******)ppppppplVar16;
        if (ppppppplVar17 == (longlong *******)0x0) {
          uStack_1d8 = (longlong *******)((ulonglong)uStack_1d8 & 0xffffffffffffff00);
          FUN_1428d9760(&UNK_142ab1588,0x2b,&uStack_1d8,&UNK_142ab1568,&UNK_142ab1708);
          goto code_r0x00014093e0b6;
        }
        pppppplStack_58 = (longlong ******)ppppppplVar17;
        ppppppplVar16 =
             (longlong *******)
             FUN_14167e9d0(0,ppppppplVar17,0x4009a,ppppppplVar16,CONCAT44(uVar35,0x4009a),
                           in_stack_fffffffffffffdd0 & 0xffffffff00000000,0);
        if (ppppppplVar16 == (longlong *******)0x0) {
          ppppppplVar16 = (longlong *******)FUN_1428a9650(0);
          FUN_14168df40(pppppplStack_58);
          FUN_14168df40(pppppplStack_50);
          goto code_r0x00014093ddd0;
        }
        uStack_1d8 = (longlong *******)0x1;
        uStack_1d0 = (longlong *******)0x0;
        uStack_1c8 = (longlong *******)0x20;
        uStack_1c0 = 2;
        iStack_1bc = 0;
        uStack_160 = 2;
        pppppplStack_70 = (longlong ******)ppppppplVar16;
        FUN_1413036c0(&pppppplStack_d0,&uStack_1d8,&UNK_142ab1530);
        pppppplVar13 = pppppplStack_70;
        pppppplVar12 = pppppplStack_c0;
        ppppppplVar16 = (longlong *******)pppppplStack_c8;
        if ((longlong *******)0xfffffffe < pppppplStack_c0) {
          FUN_1428d9430(&UNK_142ab1720,0x2f,&UNK_142ab17b0);
          goto code_r0x00014093e0b6;
        }
        pppppplStack_80 = (longlong ******)ppppppplVar26;
        FUN_14167dd60(pppppplStack_70,pppppplStack_c8,pppppplStack_c8,
                      (ulonglong)pppppplStack_c0 & 0xffffffff);
        pppppplVar11 = pppppplStack_d0;
        FUN_14167dcb0(pppppplVar13);
        FUN_14168df40(pppppplStack_58);
        FUN_14168df40(pppppplStack_50);
        if (SBORROW8(0,(longlong)pppppplVar11)) goto code_r0x00014093ddd0;
        pppppplStack_58 = pppppplVar11;
        pppppplStack_50 = (longlong ******)ppppppplVar16;
        lVar15 = *(longlong *)*param_1;
        pppppplStack_d0 = (longlong ******)0x2000000020;
        pppppplStack_c8 = (longlong ******)CONCAT44(pppppplStack_c8._4_4_,0x20);
        func_0x000140bd5310(&uStack_1d8,&pppppplStack_d0,0x5b);
        func_0x000140bd5370(apppppplStack_f8,&uStack_1d8);
        uStack_1c8 = (longlong *******)((longlong)pppppplVar12 << 3);
        uStack_1d0 = (longlong *******)pppppplStack_50;
        uStack_1d8 = (longlong *******)0x3;
        FUN_1416a71f0(&pppppplStack_d0,lVar15 + 0x10,&UNK_142ab20e6,3,apppppplStack_f8,&uStack_1d8);
        if ((int)pppppplStack_d0 != 0xb) {
          uStack_198 = uStack_90;
          uStack_190 = uStack_88;
          uStack_1b8 = uStack_b0;
          uStack_1b4 = iStack_ac;
          uStack_1b0 = uStack_a8;
          iStack_1ac = iStack_a4;
          uStack_1c8 = (longlong *******)pppppplStack_c0;
          uStack_1c0 = uStack_b8;
          iStack_1bc = iStack_b4;
          uStack_1d8 = (longlong *******)pppppplStack_d0;
          uStack_1d0 = (longlong *******)pppppplStack_c8;
          FUN_1428d9760(&UNK_142ab20e9,0xf,&uStack_1d8,&UNK_142ab1548,&UNK_142ab20f8);
          goto code_r0x00014093e0b6;
        }
        lStack_1a8 = CONCAT44(uStack_94,uStack_98);
        uStack_1b8 = uStack_a8;
        uStack_1b4 = iStack_a4;
        uStack_1b0 = uStack_a0;
        iStack_1ac = iStack_9c;
        uStack_1c8 = (longlong *******)CONCAT44(iStack_b4,uStack_b8);
        uStack_1c0 = uStack_b0;
        iStack_1bc = iStack_ac;
        uStack_1d8 = (longlong *******)pppppplStack_c8;
        uStack_1d0 = (longlong *******)pppppplStack_c0;
        puVar19 = param_2;
        lVar15 = FUN_140d61e00(ppppppplVar14,param_2);
        if (lVar15 == 0) {
          FUN_140d77340(&pppppplStack_d0,ppppppplVar14,param_2);
          ppppppplVar21 = (longlong *******)pppppplStack_50;
          ppppppplVar30 = (longlong *******)pppppplStack_58;
          ppppppplVar17 = (longlong *******)pppppplStack_c0;
          ppppppplVar16 = (longlong *******)pppppplStack_c8;
          ppppppplVar26 = (longlong *******)pppppplStack_d0;
          ppppppplVar14 = (longlong *******)pppppplStack_c8;
          if (!SBORROW8(0,(longlong)pppppplStack_d0)) goto code_r0x00014093ded5;
          if ((longlong)pppppplStack_c0 < 0) {
            uVar25 = 0;
          }
          else {
            if ((longlong *******)pppppplStack_c0 == (longlong *******)0x0)
            goto code_r0x00014093debc;
            uVar25 = 1;
            ppppppplVar14 = (longlong *******)func_0x000140613c10(pppppplStack_c0,1);
            if (ppppppplVar14 != (longlong *******)0x0) goto code_r0x00014093dec1;
          }
          FUN_1428d8fe3(uVar25,ppppppplVar17,&UNK_142c41138);
          goto code_r0x00014093e0b6;
        }
        FUN_140d77340(&pppppplStack_d0,lVar15,puVar19);
        ppppppplVar21 = (longlong *******)pppppplStack_50;
        ppppppplVar30 = (longlong *******)pppppplStack_58;
        ppppppplVar17 = (longlong *******)pppppplStack_c0;
        ppppppplVar16 = (longlong *******)pppppplStack_c8;
        ppppppplVar26 = (longlong *******)pppppplStack_d0;
        ppppppplVar14 = (longlong *******)pppppplStack_c8;
        if (!SBORROW8(0,(longlong)pppppplStack_d0)) goto code_r0x00014093ded5;
        if (-1 < (longlong)pppppplStack_c0) {
          if ((longlong *******)pppppplStack_c0 == (longlong *******)0x0) {
code_r0x00014093debc:
            ppppppplVar14 = (longlong *******)0x1;
            ppppppplVar21 = (longlong *******)pppppplStack_50;
            ppppppplVar16 = (longlong *******)pppppplStack_c8;
            ppppppplVar30 = (longlong *******)pppppplStack_58;
            ppppppplVar17 = (longlong *******)pppppplStack_c0;
          }
          else {
            uVar25 = 1;
            ppppppplVar14 = (longlong *******)func_0x000140613c10(pppppplStack_c0,1);
            if (ppppppplVar14 == (longlong *******)0x0) goto code_r0x00014093e08e;
          }
code_r0x00014093dec1:
          func_0x000142923c00(ppppppplVar14,ppppppplVar16,ppppppplVar17);
          ppppppplVar26 = ppppppplVar17;
code_r0x00014093ded5:
          iVar10 = iStack_1ac;
          uVar9 = uStack_1b0;
          iVar8 = uStack_1b4;
          uVar7 = uStack_1b8;
          iVar6 = iStack_1bc;
          uVar5 = uStack_1c0;
          uVar35 = (undefined4)uStack_1d8;
          uVar2 = uStack_1d8._4_4_;
          uVar3 = (undefined4)uStack_1d0;
          uVar4 = uStack_1d0._4_4_;
          *(undefined4 *)pppppplStack_80 = plStack_78._0_4_;
          if (!SBORROW8(0,*plStack_60)) {
            plStack_78 = plStack_60 + 3;
            pppppplStack_d8 = (longlong ******)ppppppplVar26;
            pppppplStack_80 = (longlong ******)ppppppplVar14;
            pppppplStack_70 = (longlong ******)ppppppplVar17;
            func_0x00014003bb80();
            FUN_140be24b0(plStack_78);
            ppppppplVar21 = (longlong *******)pppppplStack_50;
            ppppppplVar30 = (longlong *******)pppppplStack_58;
            ppppppplVar17 = (longlong *******)pppppplStack_70;
            ppppppplVar14 = (longlong *******)pppppplStack_80;
            ppppppplVar26 = (longlong *******)pppppplStack_d8;
            if (*plStack_60 != 0) {
              func_0x000140613c20(plStack_60[1],*plStack_60,1);
            }
          }
          *plStack_60 = (longlong)ppppppplVar26;
          plStack_60[1] = (longlong)ppppppplVar14;
          plStack_60[2] = (longlong)ppppppplVar17;
          *(undefined4 *)(plStack_60 + 3) = uVar35;
          *(undefined4 *)((longlong)plStack_60 + 0x1c) = uVar2;
          *(undefined4 *)(plStack_60 + 4) = uVar3;
          *(undefined4 *)((longlong)plStack_60 + 0x24) = uVar4;
          *(undefined4 *)(plStack_60 + 5) = (undefined4)uStack_1c8;
          *(undefined4 *)((longlong)plStack_60 + 0x2c) = uStack_1c8._4_4_;
          *(undefined4 *)(plStack_60 + 6) = uVar5;
          *(int *)((longlong)plStack_60 + 0x34) = iVar6;
          *(undefined4 *)(plStack_60 + 7) = uVar7;
          *(int *)((longlong)plStack_60 + 0x3c) = iVar8;
          *(undefined4 *)(plStack_60 + 8) = uVar9;
          *(int *)((longlong)plStack_60 + 0x44) = iVar10;
          plStack_60[9] = lStack_1a8;
          if (ppppppplVar30 == (longlong *******)0x0) {
            return;
          }
          func_0x000140613c20(ppppppplVar21,(longlong)ppppppplVar30 << 3,2);
          return;
        }
        uVar25 = 0;
code_r0x00014093e08e:
        FUN_1428d8fe3(uVar25,ppppppplVar17,&UNK_142c41138);
code_r0x00014093e0b6:
                    /* WARNING: Does not return */
        pcVar1 = (code *)invalidInstructionException();
        (*pcVar1)();
      }
    }
    ppppppplVar16 = (longlong *******)0x2800000003;
  }
  else {
code_r0x00014093dd74:
    if ((longlong *******)pppppplStack_d0 != (longlong *******)0x0) {
      func_0x000140613c20(pppppplStack_c8,pppppplStack_d0,1);
    }
  }
  ppppppplVar16 = (longlong *******)FUN_1428a9590(ppppppplVar16);
code_r0x00014093ddd0:
  apppppplStack_f8[0] = (longlong ******)ppppppplVar16;
  if (1 < *puRam000000014362c3a8) {
    pppppplStack_d0 = (longlong ******)apppppplStack_f8;
    pppppplStack_c8 = (longlong ******)&UNK_140d4cc20;
    puVar18 = (undefined4 *)func_0x00014067dd80(&UNK_142ab2140);
    uStack_1b8 = *puVar18;
    uStack_1b4 = puVar18[1];
    uStack_1b0 = puVar18[2];
    iStack_1ac = puVar18[3];
    lStack_1a8 = 2;
    puStack_1a0 = &UNK_142ab2158;
    uStack_198 = 0x2a;
    puStack_188 = &UNK_142ab2130;
    uStack_180 = 1;
    pppppplStack_178 = (longlong ******)&pppppplStack_d0;
    uStack_170 = 1;
    uStack_168 = 0;
    uStack_1d8 = (longlong *******)0x0;
    uStack_1d0 = (longlong *******)&UNK_142ab2158;
    uStack_1c8 = (longlong *******)0x2a;
    uStack_1c0 = 0;
    iStack_1bc = 0;
    uStack_190 = CONCAT44(puVar18[4],1);
    func_0x000140d4a330(&uStack_61,&uStack_1d8);
  }
  func_0x000140d4cc30(apppppplStack_f8);
  return;
}
