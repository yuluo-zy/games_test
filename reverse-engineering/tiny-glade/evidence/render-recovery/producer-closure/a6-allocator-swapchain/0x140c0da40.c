
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140c0da40(undefined8 *param_1,longlong param_2,longlong param_3)

{
  undefined8 uVar1;
  code *pcVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  int iVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined1 *puStack_170;
  longlong lStack_168;
  ulonglong uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  ulonglong uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  longlong lStack_120;
  code *pcStack_118;
  ulonglong *puStack_110;
  code *pcStack_108;
  undefined8 *puStack_100;
  code *pcStack_f8;
  longlong lStack_f0;
  undefined1 *puStack_e8;
  undefined *puStack_e0;
  ulonglong uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  ulonglong uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  longlong *plStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  longlong lStack_60;
  undefined8 *puStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_140c12dc0(&uStack_1b8);
  if (*(char *)(param_2 + 0x688) == '\x01') {
    FUN_140c15150(&puStack_170,param_2);
    uVar7 = uStack_140;
    lVar6 = lStack_168;
    auVar5._8_8_ = uStack_128;
    auVar5._0_8_ = uStack_130;
    auVar14 = auVar5 ^ _UNK_1429258f0;
    auVar12 = _UNK_1429258f0 ^ *(undefined1 (*) [16])(param_2 + 0x670);
    iVar15 = -(uint)(auVar12._0_4_ < auVar14._0_4_);
    iVar16 = -(uint)(auVar12._4_4_ < auVar14._4_4_);
    iVar17 = -(uint)(auVar12._8_4_ < auVar14._8_4_);
    iVar18 = -(uint)(auVar12._12_4_ < auVar14._12_4_);
    auVar13._4_4_ = -(uint)(auVar12._4_4_ == auVar14._4_4_);
    auVar13._12_4_ = -(uint)(auVar12._12_4_ == auVar14._12_4_);
    auVar13._0_4_ = auVar13._4_4_;
    auVar13._8_4_ = auVar13._12_4_;
    auVar12._4_4_ = iVar15;
    auVar12._0_4_ = iVar15;
    auVar12._8_4_ = iVar17;
    auVar12._12_4_ = iVar17;
    auVar14._4_4_ = iVar16;
    auVar14._0_4_ = iVar16;
    auVar14._8_4_ = iVar18;
    auVar14._12_4_ = iVar18;
    auVar14 = auVar14 | auVar13 & auVar12;
    *(undefined1 (*) [16])(param_2 + 0x670) =
         ~auVar14 & *(undefined1 (*) [16])(param_2 + 0x670) | auVar5 & auVar14;
    if ((char)uStack_174 == '\x02') {
      lVar3 = uStack_160 * 0x18;
      puStack_58 = (undefined8 *)uStack_160;
      lStack_f0 = lVar3;
      if (uStack_160 == 0) {
        lVar8 = 8;
      }
      else {
        lVar8 = func_0x000140613c10(lVar3,8);
        if (lVar8 == 0) {
          uStack_49 = 1;
          FUN_1428d8fe3(8,lVar3,&UNK_142b19920);
                    /* WARNING: Does not return */
          pcVar2 = (code *)invalidInstructionException();
          (*pcVar2)();
        }
        if (puStack_58 == (undefined8 *)0x1) {
          uVar9 = 0;
        }
        else {
          puVar11 = (undefined8 *)(lVar6 + 0x60);
          puVar10 = (undefined8 *)(lVar8 + 0x28);
          uVar9 = 0;
          do {
            uVar4 = puVar11[-10];
            uVar1 = puVar11[-7];
            puVar10[-5] = puVar11[-0xb];
            puVar10[-4] = uVar4;
            puVar10[-3] = uVar1;
            uVar4 = puVar11[-3];
            uVar1 = *puVar11;
            puVar10[-2] = puVar11[-4];
            puVar10[-1] = uVar4;
            *puVar10 = uVar1;
            uVar9 = uVar9 + 2;
            puVar11 = puVar11 + 0xe;
            puVar10 = puVar10 + 6;
          } while (((ulonglong)puStack_58 & 0xfffffffffffffffe) != uVar9);
        }
        if (((ulonglong)puStack_58 & 1) != 0) {
          puVar11 = (undefined8 *)(lVar6 + 8 + uVar9 * 0x38);
          uVar4 = puVar11[1];
          uVar1 = *(undefined8 *)(lVar6 + 0x28 + uVar9 * 0x38);
          puVar10 = (undefined8 *)(lVar8 + uVar9 * 0x18);
          *puVar10 = *puVar11;
          puVar10[1] = uVar4;
          *(undefined8 *)(lVar8 + 0x10 + uVar9 * 0x18) = uVar1;
        }
        puStack_e8 = &uStack_4a;
        if (puStack_58 != (undefined8 *)0x1) {
          lStack_60 = lVar8;
          if (puStack_58 < 0x15) {
            FUN_1418ba4e0(lVar8,puStack_58,1,&puStack_e8);
          }
          else {
            FUN_1418bb3a0(lVar8,puStack_58,&puStack_e8);
          }
        }
      }
      if (*plRam000000014362c3a8 != 0) {
        puStack_110 = &uStack_140;
        lStack_120 = param_3 + 0x20;
        puStack_100 = &uStack_138;
        pcStack_118 = FUN_140d8d6c0;
        pcStack_108 = FUN_140d8d6c0;
        pcStack_f8 = FUN_140d8d6c0;
        lStack_60 = lVar8;
        puVar10 = (undefined8 *)func_0x00014067dd80(&UNK_142b19b28);
        uStack_c8 = *puVar10;
        uStack_c0 = puVar10[1];
        uStack_b8 = 1;
        puStack_b0 = &UNK_142b19b10;
        uStack_a8 = 0x18;
        puStack_98 = &UNK_142b19ab8;
        uStack_90 = 4;
        plStack_88 = &lStack_120;
        uStack_80 = 3;
        uStack_78 = 0;
        puStack_e8 = (undefined1 *)0x0;
        puStack_e0 = &UNK_142b19b10;
        uStack_d8 = 0x18;
        uStack_d0 = 0;
        uStack_a0 = CONCAT44(*(undefined4 *)(puVar10 + 2),1);
        lStack_60 = lVar8;
        func_0x000140d4a330(&uStack_4a,&puStack_e8);
      }
      if (puStack_58 != (undefined8 *)0x0) {
        func_0x000140613c20(lVar8,lStack_f0,8);
      }
    }
    if (*(ulonglong *)(param_2 + 0x680) < uVar7) {
      *(ulonglong *)(param_2 + 0x680) = uVar7;
      uStack_a8 = uStack_130;
      uStack_a0 = uStack_128;
      uStack_b8 = uStack_140;
      puStack_b0 = (undefined *)uStack_138;
      uStack_c8 = uStack_150;
      uStack_c0 = uStack_148;
      uStack_d8 = uStack_160;
      uStack_d0 = uStack_158;
      puStack_e8 = puStack_170;
      puStack_e0 = (undefined *)lStack_168;
      puStack_58 = (undefined8 *)(param_2 + 0x620);
      if (!SBORROW8(0,*(longlong *)(param_2 + 0x620))) {
        FUN_140bdfe90(puStack_58);
      }
      puStack_58[8] = uStack_a8;
      puStack_58[9] = uStack_a0;
      puStack_58[6] = uStack_b8;
      puStack_58[7] = puStack_b0;
      puStack_58[4] = uStack_c8;
      puStack_58[5] = uStack_c0;
      puStack_58[2] = uStack_d8;
      puStack_58[3] = uStack_d0;
      *puStack_58 = puStack_e8;
      puStack_58[1] = puStack_e0;
    }
    else {
      FUN_140bdfe90(&puStack_170);
    }
  }
  else if ((char)uStack_174 == '\x02') {
    FUN_140c15150(&puStack_170,param_2);
    if (*plRam000000014362c3a8 != 0) {
      lStack_120 = param_3 + 0x20;
      puStack_110 = &uStack_140;
      puStack_100 = &uStack_138;
      pcStack_118 = FUN_140d8d6c0;
      pcStack_108 = FUN_140d8d6c0;
      pcStack_f8 = FUN_140d8d6c0;
      puVar10 = (undefined8 *)func_0x00014067dd80(&UNK_142b19af8);
      uStack_c8 = *puVar10;
      uStack_c0 = puVar10[1];
      uStack_b8 = 1;
      puStack_b0 = &UNK_142b19b10;
      uStack_a8 = 0x18;
      puStack_98 = &UNK_142b19ab8;
      uStack_90 = 4;
      plStack_88 = &lStack_120;
      uStack_80 = 3;
      uStack_78 = 0;
      puStack_e8 = (undefined1 *)0x0;
      puStack_e0 = &UNK_142b19b10;
      uStack_d8 = 0x18;
      uStack_d0 = 0;
      uStack_a0 = CONCAT44(*(undefined4 *)(puVar10 + 2),1);
      func_0x000140d4a330(&uStack_4a,&puStack_e8);
    }
    FUN_140bdfe90(&puStack_170);
  }
  param_1[8] = CONCAT44(uStack_174,uStack_178);
  param_1[6] = uStack_188;
  param_1[7] = uStack_180;
  param_1[4] = uStack_198;
  param_1[5] = uStack_190;
  param_1[2] = uStack_1a8;
  param_1[3] = uStack_1a0;
  *param_1 = uStack_1b8;
  param_1[1] = uStack_1b0;
  return param_1;
}

