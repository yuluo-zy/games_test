
void __cdecl system_onboarding::hint_focused_edit::hint_focused_edit_hint(void)

{
  uint *puVar1;
  undefined4 uVar2;
  ulonglong *puVar3;
  ulonglong *puVar4;
  longlong lVar5;
  ulonglong uVar6;
  undefined4 *puVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  undefined8 *in_RCX;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong *in_R8;
  longlong *in_R9;
  uint *puVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined8 *in_stack_00000028;
  uint local_d8;
  int local_58;
  ulonglong uVar19;
  
  puVar3 = (ulonglong *)*in_R9;
  if ((char)puVar3[1] == '\0') {
    puVar4 = (ulonglong *)*in_stack_00000028;
    lVar5 = in_stack_00000028[1];
    uVar19 = *puVar4;
    uVar18 = 0;
    uVar11 = uVar19 - *(ulonglong *)(lVar5 + 0x18);
    if (uVar19 < *(ulonglong *)(lVar5 + 0x18)) {
      uVar11 = uVar18;
    }
    uVar16 = uVar19 - *(ulonglong *)(lVar5 + 0x38);
    if (uVar19 < *(ulonglong *)(lVar5 + 0x38)) {
      uVar16 = uVar18;
    }
    uVar19 = *(ulonglong *)(lVar5 + 0x30);
    uVar15 = uVar19 - uVar16;
    if (uVar19 < uVar16) {
      uVar15 = uVar18;
    }
    uVar6 = *(ulonglong *)(lVar5 + 0x10);
    uVar17 = uVar6 - uVar11;
    if (uVar6 < uVar11) {
      uVar17 = uVar18;
    }
    uVar18 = *(ulonglong *)(lVar5 + 0x40);
    *puVar4 = uVar18 - (uVar17 + uVar15);
    if (uVar6 >= uVar11 && uVar6 - uVar11 != 0 || uVar19 >= uVar16 && uVar19 - uVar16 != 0) {
      *puVar4 = uVar18;
      uVar2 = *(undefined4 *)((longlong)in_R9 + 0x1c);
      puVar7 = (undefined4 *)in_R9[2];
      *puVar7 = uVar2;
      uVar19 = *puVar3;
      *puVar3 = uVar19 + 1;
      *puVar7 = uVar2;
      *(bool *)((longlong)puVar3 + 9) = 4 < uVar19 + 1;
    }
    lVar5 = *in_R8;
    lVar8 = *(longlong *)(lVar5 + 0x100);
    if (lVar8 != 0) {
      puVar14 = *(uint **)(lVar5 + 0xf8);
      lVar9 = in_R8[1];
      if (*(char *)(lVar5 + 0x108) == '\x01') {
        lVar13 = 0;
        lVar12 = 0;
        do {
          lVar10 = *(longlong *)
                    (*(longlong *)(lVar9 + 0x1a8) + 0x10 +
                    (ulonglong)*(uint *)((longlong)puVar14 + lVar13) * 0x48);
          if (lVar10 != 0) {
            lVar12 = *(longlong *)(lVar9 + 0x1a8) +
                     (ulonglong)*(uint *)((longlong)puVar14 + lVar13) * 0x48;
            if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar12 + 0x40)) &&
               (uVar19 = *(ulonglong *)
                          (*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8),
               uVar19 != 0)) {
              lVar12 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar19 * 0x30);
            }
            else {
              lVar12 = 0;
            }
            lVar12 = (ulonglong)((int)lVar10 - 1) * 0x58 + lVar12;
          }
          lVar13 = lVar13 + 4;
        } while (lVar8 << 2 != lVar13);
      }
      else {
        puVar1 = puVar14 + lVar8;
        lVar8 = *(longlong *)(lVar9 + 0x100);
        lVar12 = 0;
        do {
          lVar13 = (ulonglong)*puVar14 * 0xa0;
          uVar18 = (ulonglong)*(uint *)(lVar8 + 0x94 + lVar13);
          uVar19 = *(longlong *)(lVar9 + 0x1a8) + uVar18 * 0x48;
          uVar11 = *(ulonglong *)(lVar8 + 0x58 + lVar13);
          if (*(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + uVar18 * 0x48) == uVar11) {
            if (uVar11 != 0) {
              if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(uVar19 + 0x40)) &&
                 (uVar18 = *(ulonglong *)
                            (*(longlong *)(uVar19 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8),
                 uVar18 != 0)) {
                lVar12 = *(longlong *)(*(longlong *)(uVar19 + 0x18) + 0x10 + ~uVar18 * 0x30);
              }
              else {
                lVar12 = 0;
              }
              if (uVar11 < 8) {
                uVar16 = (ulonglong)((uint)uVar11 & 7);
                uVar18 = 0;
                uVar11 = uVar11 & 7;
              }
              else {
                uVar18 = 0;
                do {
                  uVar19 = uVar18;
                  uVar18 = uVar19 + 8;
                } while ((uVar11 & 0x7fffffffffffff8) != uVar18);
                uVar16 = (ulonglong)((uint)uVar11 & 7);
                uVar19 = uVar19 + 7;
                uVar11 = uVar11 & 7;
              }
              while (uVar11 != 0) {
                uVar16 = uVar16 - 1;
                uVar19 = uVar18;
                uVar18 = uVar18 + 1;
                uVar11 = uVar16;
              }
              lVar12 = lVar12 + (uVar19 & 0xffffffff) * 0x58;
            }
          }
          else if (uVar11 != 0) {
            if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(uVar19 + 0x40)) &&
               (uVar18 = *(ulonglong *)
                          (*(longlong *)(uVar19 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8),
               uVar18 != 0)) {
              lVar12 = *(longlong *)(*(longlong *)(uVar19 + 0x18) + 0x10 + ~uVar18 * 0x30);
            }
            else {
              lVar12 = 0;
            }
            lVar12 = (ulonglong)*(uint *)(uVar11 * 0x10 + -8 + *(longlong *)(lVar13 + lVar8 + 0x50))
                     * 0x58 + lVar12;
          }
          puVar14 = puVar14 + 1;
        } while (puVar14 != puVar1);
      }
      if ((((lVar12 != 0) && (*(char *)((longlong)puVar3 + 9) != '\0')) && (*(int *)*in_RCX == 0))
         && (country_core::systems::collision::world::RaycastWorld::legacy_raycast(), local_58 != 2)
         ) {
        country_core::systems::ui_edit_focus::geo_src_to_edit_subject();
                    // WARNING: Could not recover jumptable at 0x00014227e09d. Too many branches
                    // WARNING: Treating indirect jump as call
        (*(code *)(&DAT_1430386a0 + *(int *)(&DAT_1430386a0 + (ulonglong)local_d8 * 4)))();
        return;
      }
    }
  }
  return;
}

