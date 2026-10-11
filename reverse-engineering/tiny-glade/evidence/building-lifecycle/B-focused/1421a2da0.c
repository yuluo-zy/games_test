
void __cdecl system_roof::visual::roof_animation::roof_animation(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  char cVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  char extraout_AL;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong *in_RCX;
  longlong lVar18;
  longlong *in_RDX;
  longlong lVar19;
  ulonglong uVar20;
  float *pfVar21;
  ulonglong uVar22;
  uint *puVar23;
  bool bVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  longlong local_e0;
  longlong local_d8;
  
  fVar14 = __real_40555555;
  fVar13 = __real_4036db6e;
  fVar12 = __real_3f7d70a4;
  fVar11 = __real_c0800000;
  fVar10 = __real_c1200000;
  fVar9 = __real_40000000;
  fVar8 = __real_3f800000;
  lVar5 = *in_RCX;
  lVar6 = in_RCX[1];
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar23 = *(uint **)(lVar5 + 0xf8);
  puVar1 = puVar23 + *(longlong *)(lVar5 + 0x100);
  cVar3 = *(char *)(lVar5 + 0x108);
  lVar7 = *in_RDX;
  local_d8 = 8;
  bVar24 = false;
  lVar19 = 0;
  uVar22 = 0;
  uVar20 = 0;
  do {
    if (cVar3 == '\0') {
      if (uVar20 == uVar22) {
        if (puVar23 == puVar1) {
          return;
        }
        while( true ) {
          uVar22 = *(ulonglong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)*puVar23 * 0xa0);
          if (uVar22 != 0) break;
          puVar23 = puVar23 + 1;
          if (puVar23 == puVar1) {
            return;
          }
        }
        lVar15 = *(longlong *)(lVar6 + 0x100) + (ulonglong)*puVar23 * 0xa0;
        uVar20 = (ulonglong)*(uint *)(lVar15 + 0x94);
        lVar16 = *(longlong *)(lVar6 + 0x1a8);
        lVar19 = lVar16 + uVar20 * 0x48;
        lVar18 = *(longlong *)(lVar16 + 0x38 + uVar20 * 0x48);
        if (*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar16 + 0x40 + uVar20 * 0x48)) {
          lVar16 = *(longlong *)(lVar19 + 0x18);
          if (*(longlong *)(lVar18 + *(ulonglong *)(lVar5 + 0x110) * 8) == 0) goto LAB_1421a306a;
          uVar20 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar20 < *(ulonglong *)(lVar15 + 0x88)) goto LAB_1421a308c;
LAB_1421a2fff:
          bVar24 = false;
        }
        else {
          lVar16 = *(longlong *)(lVar19 + 0x18);
LAB_1421a306a:
          uVar20 = *(ulonglong *)(lVar5 + 0x120);
          if (*(ulonglong *)(lVar15 + 0x88) <= uVar20) goto LAB_1421a2fff;
LAB_1421a308c:
          bVar24 = *(longlong *)(*(longlong *)(lVar15 + 0x80) + uVar20 * 8) != 0;
        }
        puVar23 = puVar23 + 1;
        lVar18 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar5 + 0x118) * 8) * 0x30;
        lVar19 = *(longlong *)(lVar16 + 0x10 + lVar18);
        local_e0 = *(longlong *)(lVar16 + 0x28 + lVar18);
        local_d8 = *(longlong *)(lVar15 + 0x50);
        uVar20 = 0;
      }
      uVar17 = (ulonglong)*(uint *)(local_d8 + 8 + uVar20 * 0x10);
    }
    else {
      if (uVar20 == uVar22) {
        if (puVar23 == puVar1) {
          return;
        }
        while( true ) {
          uVar22 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)*puVar23 * 0x48);
          if (uVar22 != 0) break;
          puVar23 = puVar23 + 1;
          if (puVar23 == puVar1) {
            return;
          }
        }
        lVar19 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*puVar23 * 0x48;
        lVar16 = *(longlong *)(lVar19 + 0x38);
        uVar20 = *(ulonglong *)(lVar19 + 0x40);
        if (*(ulonglong *)(lVar5 + 0x110) < uVar20) {
          lVar18 = *(longlong *)(lVar19 + 0x18);
          if (*(longlong *)(lVar16 + *(ulonglong *)(lVar5 + 0x110) * 8) == 0) goto LAB_1421a300b;
          uVar17 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar17 < uVar20) goto LAB_1421a3029;
LAB_1421a2f7f:
          bVar24 = false;
        }
        else {
          lVar18 = *(longlong *)(lVar19 + 0x18);
LAB_1421a300b:
          uVar17 = *(ulonglong *)(lVar5 + 0x120);
          if (uVar20 <= uVar17) goto LAB_1421a2f7f;
LAB_1421a3029:
          bVar24 = *(longlong *)(lVar16 + uVar17 * 8) != 0;
        }
        puVar23 = puVar23 + 1;
        lVar16 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar5 + 0x118) * 8) * 0x30;
        lVar19 = *(longlong *)(lVar18 + 0x10 + lVar16);
        local_e0 = *(longlong *)(lVar18 + 0x28 + lVar16);
        uVar20 = 0;
      }
      uVar17 = uVar20 & 0xffffffff;
    }
    pfVar21 = (float *)(uVar17 * 8 + lVar19);
    puVar2 = (undefined4 *)(local_e0 + uVar17 * 4);
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (bVar24) {
      fVar25 = *(float *)(lVar7 + 0x28);
      fVar26 = fVar25 * fVar11;
      fVar25 = fVar25 * fVar10;
      if (fVar26 != 0.0) goto LAB_1421a316f;
joined_r0x0001421a316d:
      if (NAN(fVar26)) goto LAB_1421a316f;
    }
    else {
      fVar25 = fVar9;
      if (extraout_AL == '\0') {
        fVar25 = fVar8;
      }
      fVar27 = *(float *)(lVar7 + 0x28);
      fVar26 = fVar27 * fVar13 * fVar25;
      fVar25 = (float)(-(uint)(fVar12 < *pfVar21) & (uint)(fVar27 * fVar14)) * fVar25;
      if (fVar26 == 0.0) goto joined_r0x0001421a316d;
LAB_1421a316f:
      *puVar2 = uVar4;
      fVar27 = 0.0;
      if (0.0 <= fVar26 + *pfVar21) {
        fVar27 = fVar26 + *pfVar21;
      }
      fVar26 = fVar8;
      if (fVar27 <= fVar8) {
        fVar26 = fVar27;
      }
      *pfVar21 = fVar26;
    }
    uVar20 = uVar20 + 1;
    if ((fVar25 != 0.0) || (NAN(fVar25))) {
      *puVar2 = uVar4;
      fVar27 = 0.0;
      if (0.0 <= fVar25 + pfVar21[1]) {
        fVar27 = fVar25 + pfVar21[1];
      }
      fVar25 = fVar8;
      if (fVar27 <= fVar8) {
        fVar25 = fVar27;
      }
      pfVar21[1] = fVar25;
    }
  } while( true );
}

