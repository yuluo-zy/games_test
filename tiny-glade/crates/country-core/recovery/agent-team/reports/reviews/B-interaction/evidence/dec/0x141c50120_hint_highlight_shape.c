
void __cdecl system_onboarding::hint_display_system::hint_highlight_shape(void)

{
  int *piVar1;
  uint *puVar2;
  char cVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  uint uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  longlong lVar14;
  undefined8 uVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar18;
  longlong lVar19;
  longlong *in_R8;
  longlong lVar20;
  longlong lVar21;
  uint *puVar22;
  ulonglong uVar23;
  
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  puVar22 = *(uint **)(lVar7 + 0xf8);
  puVar2 = puVar22 + *(longlong *)(lVar7 + 0x100);
  cVar3 = *(char *)(lVar7 + 0x108);
  uVar4 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar9 = *in_RDX;
  puVar10 = (undefined4 *)in_RDX[2];
  uVar5 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar11 = *in_R8;
  puVar12 = (undefined4 *)in_R8[2];
  lVar21 = 8;
  lVar20 = 0;
  lVar19 = 0;
  uVar18 = 0;
  uVar23 = 0;
  do {
    if (cVar3 == '\0') {
      if (uVar23 == uVar18) {
        do {
          if (puVar22 == puVar2) {
            return;
          }
          uVar6 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar6 * 0xa0);
        } while (uVar18 == 0);
        lVar21 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar6 * 0xa0;
        uVar23 = (ulonglong)*(uint *)(lVar21 + 0x94);
        lVar19 = *(longlong *)(lVar8 + 0x1a8) + uVar23 * 0x48;
        uVar23 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x40 + uVar23 * 0x48);
        if ((*(ulonglong *)(lVar7 + 0x110) < uVar23) &&
           (uVar17 = *(ulonglong *)
                      (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8),
           uVar17 != 0)) {
          lVar20 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar17 * 0x30);
          uVar17 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar23 <= uVar17) goto LAB_141c50362;
LAB_141c50335:
          uVar23 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + uVar17 * 8);
          if (uVar23 == 0) goto LAB_141c50362;
          lVar19 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar23 * 0x30);
        }
        else {
          lVar20 = 0;
          uVar17 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar17 < uVar23) goto LAB_141c50335;
LAB_141c50362:
          lVar19 = 0;
        }
        lVar21 = *(longlong *)(lVar21 + 0x50);
        uVar23 = 0;
      }
      uVar17 = (ulonglong)*(uint *)(lVar21 + 8 + uVar23 * 0x10);
    }
    else {
      if (uVar23 == uVar18) {
        do {
          if (puVar22 == puVar2) {
            return;
          }
          uVar6 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar6 * 0x48);
        } while (uVar18 == 0);
        lVar19 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar6 * 0x48;
        uVar23 = *(ulonglong *)(lVar19 + 0x40);
        if ((*(ulonglong *)(lVar7 + 0x110) < uVar23) &&
           (uVar17 = *(ulonglong *)
                      (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8),
           uVar17 != 0)) {
          lVar20 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar17 * 0x30);
          uVar17 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar23 <= uVar17) goto LAB_141c50358;
LAB_141c50303:
          uVar23 = *(ulonglong *)(*(longlong *)(lVar19 + 0x38) + uVar17 * 8);
          if (uVar23 == 0) goto LAB_141c50358;
          lVar19 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar23 * 0x30);
        }
        else {
          lVar20 = 0;
          uVar17 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar17 < uVar23) goto LAB_141c50303;
LAB_141c50358:
          lVar19 = 0;
        }
        uVar23 = 0;
      }
      uVar17 = uVar23 & 0xffffffff;
    }
    piVar1 = (int *)(lVar20 + uVar17 * 0x18);
    uVar23 = uVar23 + 1;
    if ((((0.9 < *(float *)(lVar19 + uVar17 * 0xc + 4)) &&
         (uVar6 = *piVar1 - 6, 4 < uVar6 || uVar6 == 3)) && (*piVar1 - 1U < 2)) &&
       (*(char *)((longlong)piVar1 + 0x11) != '\0')) {
      country_core::resources::walls::public_walls::PublicWalls::roof_entity();
      if (extraout_RAX >> 0x20 != 0) {
        uVar13 = *(undefined8 *)(piVar1 + 2);
        *puVar10 = uVar4;
        lVar14 = *(longlong *)(lVar9 + 0x30);
        uVar15 = *(undefined8 *)(lVar9 + 0x40);
        if (lVar14 == *(longlong *)(lVar9 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                    ();
        }
        lVar16 = *(longlong *)(lVar9 + 0x28);
        *(undefined8 *)(lVar16 + lVar14 * 0x18) = uVar15;
        *(undefined8 *)(lVar16 + 8 + lVar14 * 0x18) = uVar13;
        *(undefined1 *)(lVar16 + 0x10 + lVar14 * 0x18) = 2;
        *(longlong *)(lVar9 + 0x30) = lVar14 + 1;
        *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
      }
      uVar13 = *(undefined8 *)(piVar1 + 2);
      *puVar12 = uVar5;
      lVar14 = *(longlong *)(lVar11 + 0x30);
      uVar15 = *(undefined8 *)(lVar11 + 0x40);
      if (lVar14 == *(longlong *)(lVar11 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar11 + 0x28);
      *(undefined8 *)(lVar16 + lVar14 * 0x18) = uVar15;
      *(undefined8 *)(lVar16 + 8 + lVar14 * 0x18) = uVar13;
      *(undefined1 *)(lVar16 + 0x10 + lVar14 * 0x18) = 2;
      *(longlong *)(lVar11 + 0x30) = lVar14 + 1;
      *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
    }
  } while( true );
}

