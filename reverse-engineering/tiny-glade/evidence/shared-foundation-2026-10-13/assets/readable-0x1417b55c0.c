
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void rkyv__util__to_bytes(longlong *param_1,longlong param_2)
{
  undefined8 *puVar1;
  undefined1 uVar2;
  undefined8 uVar3;
  code *pcVar4;
  undefined1 uVar5;
  longlong lVar6;
  byte *pbVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong lStack_178;
  ulonglong uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 uStack_140;
  ulonglong uStack_138;
  ulonglong uStack_130;
  longlong lStack_128;
  longlong lStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong lStack_108;
  undefined8 uStack_100;
  longlong lStack_f8;
  ulonglong uStack_f0;
  undefined2 uStack_e8;
  undefined2 uStack_e6;
  undefined4 uStack_e4;
  undefined2 uStack_e0;
  undefined2 uStack_de;
  undefined4 uStack_dc;
  undefined2 uStack_d8;
  undefined2 uStack_d6;
  undefined4 uStack_d4;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  longlong lStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined2 uStack_74;
  undefined2 uStack_72;
  undefined2 uStack_70;
  undefined2 uStack_6e;
  undefined2 uStack_6c;
  undefined2 uStack_6a;
  undefined2 uStack_68;
  undefined4 uStack_66;
  undefined2 uStack_62;
  undefined6 uStack_60;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  uStack_48 = 0xfffffffffffffffe;
  uVar8 = 0x10;
  lVar6 = func_0x000140613c10(0x100);
  if (lVar6 == 0) {
    FUN_1428d9430(&UNK_142fda40b,0x20,&UNK_142fda488);
LAB_1417b5a98:
    uStack_49 = 1;
    FUN_1428d9330(&UNK_142cddc30,0x22,&UNK_142cddc58);
    goto LAB_1417b5b38;
  }
  uStack_c8 = 0;
  lStack_f8 = 0;
  uStack_e8 = 0;
  uStack_e6 = 0;
  uStack_e4 = 0;
  uStack_e0 = 8;
  uStack_de = 0;
  uStack_dc = 0;
  uStack_d8 = 0;
  uStack_d6 = 0;
  uStack_d4 = 0;
  uStack_d0 = lVar6;
  pbVar7 = (byte *)FUN_140d4d740();
  if ((*pbVar7 & 1) == 0) {
    lStack_108 = FUN_140d64e00();
    pbVar7[0] = 1;
    pbVar7[1] = 0;
    pbVar7[2] = 0;
    pbVar7[3] = 0;
    pbVar7[4] = 0;
    pbVar7[5] = 0;
    pbVar7[6] = 0;
    pbVar7[7] = 0;
    *(longlong *)(pbVar7 + 8) = lStack_108;
    *(undefined8 *)(pbVar7 + 0x10) = uVar8;
  }
  else {
    lStack_108 = *(longlong *)(pbVar7 + 8);
    uVar8 = *(undefined8 *)(pbVar7 + 0x10);
  }
  *(longlong *)(pbVar7 + 8) = lStack_108 + 1;
  uStack_140 = 1;
  uStack_138 = 0;
  uStack_130 = 0;
  uStack_168 = CONCAT22(uStack_e6,uStack_e8);
  uStack_160 = CONCAT22(uStack_de,uStack_e0);
  uStack_158 = CONCAT22(uStack_d6,uStack_d8);
  lStack_178 = lStack_f8;
  uStack_170 = uStack_f0;
  uStack_164 = uStack_e4;
  uStack_15c = uStack_dc;
  uStack_154 = uStack_d4;
  uStack_150 = (undefined4)uStack_d0;
  uStack_14c = uStack_d0._4_4_;
  uStack_148 = uStack_c8;
  lStack_128 = _DAT_142cdf550;
  lStack_120 = _UNK_142cdf558;
  uStack_118 = _DAT_142cdf560;
  uStack_114 = _UNK_142cdf564;
  uStack_110 = _UNK_142cdf568;
  uStack_10c = _UNK_142cdf56c;
  uVar5 = *(undefined1 *)(param_2 + 0x29);
  uVar2 = *(undefined1 *)(param_2 + 0x28);
  uVar3 = *(undefined8 *)(param_2 + 0x10);
  uStack_49 = 1;
  uStack_100 = uVar8;
  rkyv__impls__core__impl$9__serialize_unsized(&lStack_f8,*(undefined8 *)(param_2 + 8),uVar3,&lStack_178);
  uVar9 = uStack_130;
  if (lStack_f8 != -0x7ffffffffffffffc) {
    uStack_78 = (undefined4)CONCAT42(uStack_e4,uStack_e6);
    uStack_74 = (undefined2)((uint)uStack_e4 >> 0x10);
    uStack_72 = uStack_e0;
    uStack_70 = uStack_de;
    uStack_6e = (undefined2)uStack_dc;
    uStack_6c = (undefined2)((uint)uStack_dc >> 0x10);
    uStack_6a = uStack_d8;
    uStack_68 = uStack_d6;
    uStack_66 = uStack_d4;
    uStack_62 = (undefined2)uStack_d0;
    uStack_60 = (undefined6)((ulonglong)uStack_d0 >> 0x10);
    *(undefined2 *)((longlong)param_1 + 0x16) = uStack_74;
    *(undefined4 *)((longlong)param_1 + 0x12) = uStack_78;
    *(uint *)(param_1 + 3) = CONCAT22(uStack_de,uStack_e0);
    *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_dc;
    *(uint *)(param_1 + 4) = CONCAT22(uStack_d6,uStack_d8);
    *(undefined4 *)((longlong)param_1 + 0x24) = uStack_d4;
    param_1[5] = uStack_d0;
    *param_1 = lStack_f8;
    param_1[1] = uStack_f0;
    *(undefined2 *)(param_1 + 2) = uStack_e8;
    core__ptr__drop_in_place(&lStack_178);
    return;
  }
  uVar11 = -(int)uStack_130 & 3;
  uStack_4b = uVar2;
  uStack_4a = uVar5;
  if (uVar11 == 0) {
    uVar9 = uStack_130 + 0x1c;
    uVar10 = uStack_130;
    lVar6 = uStack_140;
    uVar12 = uStack_138;
    if (uStack_138 < uVar9) goto LAB_1417b5874;
  }
  else {
    uVar10 = uVar11 + uStack_130;
    lVar6 = uStack_140;
    uVar12 = uStack_138;
    if (uStack_138 < uVar10) {
      lVar6 = 0x3f;
      if (uVar10 - 1 != 0) {
        for (; uVar10 - 1 >> lVar6 == 0; lVar6 = lVar6 + -1) {
        }
      }
      uVar12 = 0;
      if (1 < uVar10) {
        uVar12 = 0xffffffffffffffff >> (~(byte)lVar6 & 0x3f);
      }
      if (uVar12 == 0xffffffffffffffff) {
        uStack_49 = 1;
        FUN_1428d9330(&UNK_142cddc30,0x22,&UNK_142cddc58);
        goto LAB_1417b5b38;
      }
      uVar12 = uVar12 + 1;
      if (uStack_138 == 0) {
        lVar6 = func_0x000140613c10(uVar12,0x10);
        if (lVar6 == 0) {
          uStack_49 = 1;
          FUN_1428d9310(&UNK_142cddc00);
          goto LAB_1417b5b38;
        }
      }
      else {
        lVar6 = func_0x000140613c30(uStack_140,uStack_138,0x10,uVar12);
        if (lVar6 == 0) {
          uStack_49 = 1;
          FUN_1428d9310(&UNK_142cddc18);
          goto LAB_1417b5b38;
        }
      }
    }
    uStack_138 = uVar12;
    uStack_140 = lVar6;
    func_0x000142923c20(uStack_140 + uVar9,0,(ulonglong)uVar11);
    uVar9 = uVar10 + 0x1c;
    lVar6 = uStack_140;
    uVar12 = uStack_138;
    uStack_130 = uVar10;
    if (uStack_138 < uVar9) {
LAB_1417b5874:
      uVar10 = uStack_130;
      lVar6 = 0x3f;
      if (uStack_130 + 0x1b != 0) {
        for (; uStack_130 + 0x1b >> lVar6 == 0; lVar6 = lVar6 + -1) {
        }
      }
      uVar12 = 0;
      if (1 < uVar9) {
        uVar12 = 0xffffffffffffffff >> (~(byte)lVar6 & 0x3f);
      }
      if (uVar12 == 0xffffffffffffffff) goto LAB_1417b5a98;
      uVar12 = uVar12 + 1;
      if (uStack_138 == 0) {
        lVar6 = func_0x000140613c10(uVar12,0x10);
        if (lVar6 == 0) {
          uStack_49 = 1;
          FUN_1428d9310(&UNK_142cddc00);
          goto LAB_1417b5b38;
        }
      }
      else {
        lVar6 = func_0x000140613c30(uStack_140,uStack_138,0x10,uVar12);
        if (lVar6 == 0) {
          uStack_49 = 1;
          FUN_1428d9310(&UNK_142cddc18);
          goto LAB_1417b5b38;
        }
      }
    }
  }
  uStack_138 = uVar12;
  uStack_140 = lVar6;
  puVar1 = (undefined8 *)(uStack_140 + 0xc + uVar10);
  *puVar1 = 0;
  puVar1[1] = 0;
  *(undefined8 *)(uStack_140 + uVar10) = 0;
  ((undefined8 *)(uStack_140 + uVar10))[1] = 0;
  *(undefined4 *)(uStack_140 + 0x10 + uVar10) = *(undefined4 *)(param_2 + 0x20);
  *(undefined8 *)(uStack_140 + 8 + uVar10) = *(undefined8 *)(param_2 + 0x18);
  *(undefined4 *)(uStack_140 + 0x14 + uVar10) = *(undefined4 *)(param_2 + 0x24);
  *(undefined1 *)(uStack_140 + 0x18 + uVar10) = uStack_4a;
  *(undefined1 *)(uStack_140 + 0x19 + uVar10) = uStack_4b;
  lVar6 = uStack_f0 - uVar10;
  uStack_130 = uVar9;
  if (uStack_f0 < uVar10) {
    if (lVar6 < 0) goto LAB_1417b594d;
LAB_1417b5944:
    uVar5 = 0;
  }
  else {
    if (lVar6 < 0) goto LAB_1417b5944;
LAB_1417b594d:
    if ((int)lVar6 == lVar6) {
      *(int *)(uStack_140 + uVar10) = (int)lVar6;
      ((int *)(uStack_140 + uVar10))[1] = (int)uVar3;
      lStack_88 = lStack_108;
      uStack_80 = uStack_100;
      uStack_98 = CONCAT44(uStack_114,uStack_118);
      uStack_90 = CONCAT44(uStack_10c,uStack_110);
      lStack_a8 = lStack_128;
      lStack_a0 = lStack_120;
      uStack_b8 = uStack_138;
      uStack_c8 = uStack_148;
      uStack_c0 = (undefined4)uStack_140;
      uStack_bc = uStack_140._4_4_;
      uStack_d8 = (undefined2)uStack_158;
      uStack_d6 = (undefined2)((uint)uStack_158 >> 0x10);
      uStack_d4 = uStack_154;
      uStack_d0 = CONCAT44(uStack_14c,uStack_150);
      uStack_e8 = (undefined2)uStack_168;
      uStack_e6 = (undefined2)((uint)uStack_168 >> 0x10);
      uStack_e4 = uStack_164;
      uStack_e0 = (undefined2)uStack_160;
      uStack_de = (undefined2)((uint)uStack_160 >> 0x10);
      uStack_dc = uStack_15c;
      lStack_f8 = lStack_178;
      uStack_f0 = uStack_170;
      uStack_78 = (undefined4)uStack_140;
      uStack_74 = (undefined2)((ulonglong)uStack_140 >> 0x20);
      uStack_72 = (undefined2)((ulonglong)uStack_140 >> 0x30);
      uStack_70 = (undefined2)uStack_138;
      uStack_6e = (undefined2)(uStack_138 >> 0x10);
      uStack_6c = (undefined2)(uStack_138 >> 0x20);
      uStack_6a = (undefined2)(uStack_138 >> 0x30);
      uStack_68 = (undefined2)uVar9;
      uStack_66 = (undefined4)(uVar9 >> 0x10);
      uStack_62 = (undefined2)(uVar9 >> 0x30);
      uStack_b0 = uVar9;
      func_0x000140613c20(uStack_d0,0x100,0x10);
      FUN_1420d57d0(&lStack_f8);
      lVar6 = CONCAT44(uStack_e4,CONCAT22(uStack_e6,uStack_e8));
      if (lVar6 != 0) {
        func_0x000140613c20(CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0)),lVar6 * 0x18,8);
      }
      if ((lStack_a0 != 0) && (lVar6 = lStack_a0 * 0x11 + 0x21, lVar6 != 0)) {
        func_0x000140613c20(lStack_a8 + lStack_a0 * -0x10 + -0x10,lVar6,0x10);
      }
      param_1[3] = CONCAT26(uStack_62,CONCAT42(uStack_66,uStack_68));
      param_1[1] = CONCAT26(uStack_72,CONCAT24(uStack_74,uStack_78));
      param_1[2] = CONCAT26(uStack_6a,CONCAT24(uStack_6c,CONCAT22(uStack_6e,uStack_70)));
      *param_1 = -0x7ffffffffffffffc;
      return;
    }
    uVar5 = 1;
  }
  lStack_f8 = CONCAT71(lStack_f8._1_7_,uVar5);
  uStack_49 = 1;
  FUN_1428d9760(&UNK_142cdda68,0x2b,&lStack_f8,&UNK_142cddb18,&UNK_142cddb90);
LAB_1417b5b38:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}
