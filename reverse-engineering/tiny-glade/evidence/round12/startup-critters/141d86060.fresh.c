
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141d86060(longlong *param_1,undefined8 *param_2,longlong *param_3)

{
  int *piVar1;
  undefined4 *puVar2;
  float fVar3;
  float fVar4;
  char cVar5;
  undefined4 uVar6;
  int iVar7;
  undefined8 uVar8;
  longlong lVar9;
  bool bVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  uint uVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong lVar19;
  longlong lVar20;
  uint *puVar21;
  ulonglong uVar22;
  longlong lVar23;
  uint *puVar24;
  uint *puVar25;
  float fVar26;
  ulonglong uVar27;
  longlong lVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  longlong lStack_168;
  longlong lStack_160;
  longlong lStack_150;
  longlong lStack_148;
  float fStack_13c;
  float fStack_138;
  float fStack_134;
  longlong lStack_130;
  longlong lStack_128;
  uint uStack_11c;
  longlong lStack_118;
  undefined4 *puStack_110;
  uint *puStack_108;
  longlong lStack_100;
  undefined8 uStack_f8;
  ulonglong uStack_f0;
  
  *(undefined4 *)param_2[2] = *(undefined4 *)((longlong)param_2 + 0x1c);
  fVar15 = _DAT_142f67d30;
  fVar14 = _DAT_142a83814;
  fVar13 = _DAT_142934604;
  fVar12 = _DAT_142925904;
  fVar11 = _DAT_142925870;
  uVar8 = *param_2;
  fVar3 = *(float *)(*param_3 + 0x28);
  lVar9 = *param_1;
  lStack_100 = param_1[1];
  uVar6 = *(undefined4 *)((longlong)param_1 + 0x14);
  puVar25 = *(uint **)(lVar9 + 0xf8);
  puStack_108 = puVar25 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  lStack_118 = 8;
  lStack_160 = 0;
  lStack_168 = 0;
  lStack_148 = 0;
  lStack_150 = 0;
  uVar22 = 0;
  uStack_f0 = 0;
  lVar28 = lStack_100;
  puVar24 = puStack_108;
  uStack_f8 = uVar8;
  do {
    if (cVar5 == '\0') {
      if (uStack_f0 == uVar22) {
        do {
          if (puVar25 == puVar24) {
            return;
          }
          uVar16 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar22 = *(ulonglong *)(*(longlong *)(lVar28 + 0x100) + 0x58 + (ulonglong)uVar16 * 0xa0);
        } while (uVar22 == 0);
        lVar17 = *(longlong *)(lVar28 + 0x100) + (ulonglong)uVar16 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar17 + 0x94);
        lVar20 = *(longlong *)(lVar28 + 0x1a8) + uVar18 * 0x48;
        uVar18 = *(ulonglong *)(*(longlong *)(lVar28 + 0x1a8) + 0x40 + uVar18 * 0x48);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar18) &&
           (uVar27 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar27 != 0)) {
          lStack_160 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar27 * 0x30);
        }
        else {
          lStack_160 = 0;
        }
        lVar19 = *(longlong *)(lVar20 + 0x38);
        if (*(ulonglong *)(lVar9 + 0x118) < uVar18) {
          uVar18 = *(ulonglong *)(lVar19 + *(ulonglong *)(lVar9 + 0x118) * 8);
          lVar20 = *(longlong *)(lVar20 + 0x18);
          if (uVar18 == 0) goto LAB_141d863ef;
          lStack_168 = *(longlong *)(lVar20 + 0x10 + ~uVar18 * 0x30);
        }
        else {
          lVar20 = *(longlong *)(lVar20 + 0x18);
LAB_141d863ef:
          lStack_168 = 0;
        }
        lVar23 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        lStack_148 = *(longlong *)(lVar20 + 0x10 + lVar23);
        lStack_128 = *(longlong *)(lVar20 + 0x28 + lVar23);
        lVar19 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar9 + 0x128) * 8) * 0x30;
        lStack_150 = *(longlong *)(lVar20 + 0x10 + lVar19);
        lStack_130 = *(longlong *)(lVar20 + 0x28 + lVar19);
        lStack_118 = *(longlong *)(lVar17 + 0x50);
        uStack_f0 = 0;
      }
      uVar18 = (ulonglong)*(uint *)(lStack_118 + 8 + uStack_f0 * 0x10);
    }
    else {
      if (uStack_f0 == uVar22) {
        do {
          if (puVar25 == puVar24) {
            return;
          }
          uVar16 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar22 = *(ulonglong *)(*(longlong *)(lVar28 + 0x1a8) + 0x10 + (ulonglong)uVar16 * 0x48);
        } while (uVar22 == 0);
        lVar20 = *(longlong *)(lVar28 + 0x1a8) + (ulonglong)uVar16 * 0x48;
        if ((*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar20 + 0x40)) &&
           (uVar18 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar18 != 0)) {
          lStack_160 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          lStack_160 = 0;
        }
        lVar17 = *(longlong *)(lVar20 + 0x38);
        if (*(ulonglong *)(lVar9 + 0x118) < *(ulonglong *)(lVar20 + 0x40)) {
          uVar18 = *(ulonglong *)(lVar17 + *(ulonglong *)(lVar9 + 0x118) * 8);
          lVar20 = *(longlong *)(lVar20 + 0x18);
          if (uVar18 == 0) goto LAB_141d86380;
          lStack_168 = *(longlong *)(lVar20 + 0x10 + ~uVar18 * 0x30);
        }
        else {
          lVar20 = *(longlong *)(lVar20 + 0x18);
LAB_141d86380:
          lStack_168 = 0;
        }
        lVar19 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        lStack_148 = *(longlong *)(lVar20 + 0x10 + lVar19);
        lStack_128 = *(longlong *)(lVar20 + 0x28 + lVar19);
        lVar17 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x128) * 8) * 0x30;
        lStack_150 = *(longlong *)(lVar20 + 0x10 + lVar17);
        lStack_130 = *(longlong *)(lVar20 + 0x28 + lVar17);
        uStack_f0 = 0;
      }
      uVar18 = uStack_f0 & 0xffffffff;
    }
    piVar1 = (int *)(lStack_148 + uVar18 * 0x18);
    lVar20 = uVar18 * 0x30 + lStack_160;
    puVar2 = (undefined4 *)(lStack_128 + uVar18 * 4);
    puStack_110 = (undefined4 *)(lStack_130 + uVar18 * 4);
    puVar21 = (uint *)(uVar18 * 0x10 + lStack_150);
    uStack_f0 = uStack_f0 + 1;
    *puVar2 = uVar6;
    fVar26 = (float)piVar1[2] - fVar3;
    piVar1[2] = (int)fVar26;
    uStack_11c = *puVar21;
    if (((fVar26 <= 0.0) || ((uStack_11c & 1) == 0)) ||
       (fVar26 = (float)puVar21[2] - *(float *)(lVar20 + 0x14),
       fVar29 = (float)puVar21[1] - *(float *)(lVar20 + 0x10),
       fVar30 = (float)puVar21[3] - *(float *)(lVar20 + 0x18),
       fVar30 * fVar30 + fVar26 * fVar26 + fVar29 * fVar29 < _DAT_142f67d00)) {
      fVar26 = (float)func_0x000141625ae0(uVar8);
      fVar26 = fVar26 * fVar11 + _DAT_142925984;
      *puVar2 = uVar6;
      piVar1[2] = (int)fVar26;
      iVar7 = *piVar1;
      uVar27 = func_0x000141625ae0(uVar8);
      lVar28 = func_0x000141625ae0(uVar8);
      FUN_140c907d0(&fStack_13c,uVar27 & 0xffffffff | lVar28 << 0x20);
      fVar30 = fStack_134;
      fVar29 = fStack_138;
      fVar26 = fStack_13c;
      func_0x000140c90030(&fStack_13c,lStack_168 + uVar18 * 0xc);
      fVar31 = (float)(-(uint)(iVar7 == _DAT_142aebb80) & _DAT_142f67d20 |
                      ~-(uint)(iVar7 == _DAT_142aebb80) & _DAT_142f67d10);
      fVar33 = (float)(-(uint)(uStack_11c == _UNK_142aebb84) & _UNK_142f67d24 |
                      ~-(uint)(uStack_11c == _UNK_142aebb84) & _UNK_142f67d14);
      fVar32 = fVar29 * fVar31 * fVar11 + fStack_138 * fVar33;
      fVar29 = (float)piVar1[2] * fVar15;
      fVar26 = fVar33 * fStack_13c + fVar31 * fVar26;
      fVar30 = fVar33 * fStack_134 + fVar31 * fVar30;
      fVar31 = fVar12 / SQRT(fVar30 * fVar30 + fVar32 * fVar32 + fVar26 * fVar26);
      bVar10 = (int)fVar31 - 1U < 0x7fffff ||
               (int)ABS(fVar31) - 0x800000U < 0x7f000000 && -1 < (int)fVar31;
      if (bVar10) {
        fVar32 = fVar32 * fVar31;
      }
      else {
        fVar32 = 0.0;
      }
      fVar33 = *(float *)(lVar20 + 0x10);
      fVar4 = *(float *)(lVar20 + 0x18);
      uVar16 = (uint)bVar10;
      fVar32 = fVar32 * fVar29 + *(float *)(lVar20 + 0x14);
      fVar26 = fVar33 + fVar29 * (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar26 * fVar31));
      fVar29 = fVar4 + fVar29 * (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar30 * fVar31));
      if (fVar13 <= fVar32) {
        fVar32 = fVar13;
      }
      fVar30 = SQRT(fVar29 * fVar29 + fVar26 * fVar26);
      lVar28 = lStack_100;
      puVar24 = puStack_108;
      if ((fVar30 < fVar14) || (fVar30 < SQRT(fVar4 * fVar4 + fVar33 * fVar33))) {
        *puStack_110 = uVar6;
        *puVar21 = 1;
        puVar21[1] = (uint)fVar26;
        puVar21[2] = (uint)fVar32;
        puVar21[3] = (uint)fVar29;
      }
      else {
        *puStack_110 = uVar6;
        *puVar21 = 0;
      }
    }
  } while( true );
}

