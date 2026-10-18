// 0x1421a2da0 void __cdecl roof_animation(void)

void __cdecl system_roof::visual::roof_animation::roof_animation(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  char cVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  char extraout_AL;
  longlong lVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong *in_RCX;
  longlong lVar11;
  longlong *in_RDX;
  longlong lVar12;
  ulonglong uVar13;
  float *pfVar14;
  ulonglong uVar15;
  uint *puVar16;
  bool bVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  longlong local_e0;
  longlong local_d8;
  
  lVar5 = *in_RCX;
  lVar6 = in_RCX[1];
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar16 = *(uint **)(lVar5 + 0xf8);
  puVar1 = puVar16 + *(longlong *)(lVar5 + 0x100);
  cVar3 = *(char *)(lVar5 + 0x108);
  lVar7 = *in_RDX;
  local_d8 = 8;
  bVar17 = false;
  lVar12 = 0;
  uVar15 = 0;
  uVar13 = 0;
  do {
    if (cVar3 == '\0') {
      if (uVar13 == uVar15) {
        if (puVar16 == puVar1) {
          return;
        }
        while( true ) {
          uVar15 = *(ulonglong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)*puVar16 * 0xa0);
          if (uVar15 != 0) break;
          puVar16 = puVar16 + 1;
          if (puVar16 == puVar1) {
            return;
          }
        }
        lVar8 = *(longlong *)(lVar6 + 0x100) + (ulonglong)*puVar16 * 0xa0;
        uVar13 = (ulonglong)*(uint *)(lVar8 + 0x94);
        lVar9 = *(longlong *)(lVar6 + 0x1a8);
        lVar12 = lVar9 + uVar13 * 0x48;
        lVar11 = *(longlong *)(lVar9 + 0x38 + uVar13 * 0x48);
        if (*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar9 + 0x40 + uVar13 * 0x48)) {
          lVar9 = *(longlong *)(lVar12 + 0x18);
          if (*(longlong *)(lVar11 + *(ulonglong *)(lVar5 + 0x110) * 8) == 0) goto LAB_1421a306a;
          uVar13 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar13 < *(ulonglong *)(lVar8 + 0x88)) goto LAB_1421a308c;
LAB_1421a2fff:
          bVar17 = false;
        }
        else {
          lVar9 = *(longlong *)(lVar12 + 0x18);
LAB_1421a306a:
          uVar13 = *(ulonglong *)(lVar5 + 0x120);
          if (*(ulonglong *)(lVar8 + 0x88) <= uVar13) goto LAB_1421a2fff;
LAB_1421a308c:
          bVar17 = *(longlong *)(*(longlong *)(lVar8 + 0x80) + uVar13 * 8) != 0;
        }
        puVar16 = puVar16 + 1;
        lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar5 + 0x118) * 8) * 0x30;
        lVar12 = *(longlong *)(lVar9 + 0x10 + lVar11);
        local_e0 = *(longlong *)(lVar9 + 0x28 + lVar11);
        local_d8 = *(longlong *)(lVar8 + 0x50);
        uVar13 = 0;
      }
      uVar10 = (ulonglong)*(uint *)(local_d8 + 8 + uVar13 * 0x10);
    }
    else {
      if (uVar13 == uVar15) {
        if (puVar16 == puVar1) {
          return;
        }
        while( true ) {
          uVar15 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)*puVar16 * 0x48);
          if (uVar15 != 0) break;
          puVar16 = puVar16 + 1;
          if (puVar16 == puVar1) {
            return;
          }
        }
        lVar12 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*puVar16 * 0x48;
        lVar9 = *(longlong *)(lVar12 + 0x38);
        uVar13 = *(ulonglong *)(lVar12 + 0x40);
        if (*(ulonglong *)(lVar5 + 0x110) < uVar13) {
          lVar11 = *(longlong *)(lVar12 + 0x18);
          if (*(longlong *)(lVar9 + *(ulonglong *)(lVar5 + 0x110) * 8) == 0) goto LAB_1421a300b;
          uVar10 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar10 < uVar13) goto LAB_1421a3029;
LAB_1421a2f7f:
          bVar17 = false;
        }
        else {
          lVar11 = *(longlong *)(lVar12 + 0x18);
LAB_1421a300b:
          uVar10 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar13 <= uVar10) goto LAB_1421a2f7f;
LAB_1421a3029:
          bVar17 = *(longlong *)(lVar9 + uVar10 * 8) != 0;
        }
        puVar16 = puVar16 + 1;
        lVar9 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar5 + 0x118) * 8) * 0x30;
        lVar12 = *(longlong *)(lVar11 + 0x10 + lVar9);
        local_e0 = *(longlong *)(lVar11 + 0x28 + lVar9);
        uVar13 = 0;
      }
      uVar10 = uVar13 & 0xffffffff;
    }
    pfVar14 = (float *)(uVar10 * 8 + lVar12);
    puVar2 = (undefined4 *)(local_e0 + uVar10 * 4);
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (bVar17) {
      fVar18 = *(float *)(lVar7 + 0x28);
      fVar19 = fVar18 * -4.0;
      fVar18 = fVar18 * -10.0;
      if (fVar19 != 0.0) goto LAB_1421a316f;
joined_r0x0001421a316d:
      if (NAN(fVar19)) goto LAB_1421a316f;
    }
    else {
      fVar18 = 2.0;
      if (extraout_AL == '\0') {
        fVar18 = 1.0;
      }
      fVar20 = *(float *)(lVar7 + 0x28);
      fVar19 = fVar20 * 2.857143 * fVar18;
      fVar18 = (float)(-(uint)(0.99 < *pfVar14) & (uint)(fVar20 * 3.3333333)) * fVar18;
      if (fVar19 == 0.0) goto joined_r0x0001421a316d;
LAB_1421a316f:
      *puVar2 = uVar4;
      fVar20 = 0.0;
      if (0.0 <= fVar19 + *pfVar14) {
        fVar20 = fVar19 + *pfVar14;
      }
      fVar19 = 1.0;
      if (fVar20 <= 1.0) {
        fVar19 = fVar20;
      }
      *pfVar14 = fVar19;
    }
    uVar13 = uVar13 + 1;
    if ((fVar18 != 0.0) || (NAN(fVar18))) {
      *puVar2 = uVar4;
      fVar20 = 0.0;
      if (0.0 <= fVar18 + pfVar14[1]) {
        fVar20 = fVar18 + pfVar14[1];
      }
      fVar18 = 1.0;
      if (fVar20 <= 1.0) {
        fVar18 = fVar20;
      }
      pfVar14[1] = fVar18;
    }
  } while( true );
}


