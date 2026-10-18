// name=autoclose_color_menu
// sig=void __cdecl autoclose_color_menu(void)
// size=1207
// address=0x142298d30

void __cdecl system_color::autoclose_color_menu::autoclose_color_menu(void)

{
  uint *puVar1;
  undefined8 uVar2;
  uint uVar3;
  ulonglong *puVar4;
  undefined8 *puVar5;
  ulonglong uVar6;
  longlong lVar7;
  int *piVar8;
  undefined8 *in_RCX;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  undefined8 *in_RDX;
  ulonglong uVar12;
  undefined8 uVar13;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  longlong lVar17;
  longlong *in_R8;
  undefined8 *in_R9;
  ulonglong uVar18;
  longlong lVar19;
  int *piVar20;
  ulonglong uVar21;
  uint *puVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  longlong lVar26;
  bool bVar27;
  bool bVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  longlong *in_stack_00000030;
  float local_8c;
  float local_88;
  float local_84;
  
  lVar16 = in_RCX[1];
  uVar11 = *(ulonglong *)*in_RCX;
  uVar18 = 0;
  uVar12 = uVar11 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar11 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar12 = uVar18;
  }
  uVar6 = uVar11 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar11 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar6 = uVar18;
  }
  uVar11 = *(ulonglong *)(lVar16 + 0x10);
  uVar21 = *(ulonglong *)(lVar16 + 0x30);
  bVar27 = uVar21 < uVar6;
  uVar9 = uVar21 - uVar6;
  uVar10 = uVar9;
  if (bVar27) {
    uVar10 = uVar18;
  }
  bVar28 = uVar11 < uVar12;
  uVar24 = uVar11 - uVar12;
  uVar25 = uVar24;
  if (bVar28) {
    uVar25 = uVar18;
  }
  uVar23 = uVar10 + uVar25;
  if (!bVar28 && uVar24 != 0 || !bVar27 && uVar9 != 0) {
    uVar23 = uVar18;
  }
  lVar19 = *(longlong *)(lVar16 + 8);
  lVar7 = *(longlong *)(lVar16 + 0x28);
  *(ulonglong *)*in_RCX = *(longlong *)(lVar16 + 0x40) - uVar23;
  if ((bVar28 || uVar24 == 0) && (bVar27 || uVar9 == 0)) {
LAB_142298e06:
    puVar4 = (ulonglong *)*in_RDX;
    lVar16 = in_RDX[1];
    uVar11 = *puVar4;
    uVar18 = 0;
    uVar12 = uVar11 - *(ulonglong *)(lVar16 + 0x18);
    if (uVar11 < *(ulonglong *)(lVar16 + 0x18)) {
      uVar12 = uVar18;
    }
    uVar6 = uVar11 - *(ulonglong *)(lVar16 + 0x38);
    if (uVar11 < *(ulonglong *)(lVar16 + 0x38)) {
      uVar6 = uVar18;
    }
    uVar11 = *(ulonglong *)(lVar16 + 0x30);
    uVar21 = uVar11 - uVar6;
    if (uVar6 > uVar11) {
      uVar21 = uVar18;
    }
    uVar10 = *(ulonglong *)(lVar16 + 0x10);
    uVar9 = uVar10 - uVar12;
    if (uVar12 > uVar10) {
      uVar9 = uVar18;
    }
    uVar18 = *(ulonglong *)(lVar16 + 0x40);
    *puVar4 = uVar18 - (uVar9 + uVar21);
    if (uVar12 <= uVar10 && uVar10 - uVar12 != 0 || uVar6 <= uVar11 && uVar11 - uVar6 != 0) {
      *puVar4 = uVar18;
      goto LAB_142298fef;
    }
    lVar16 = *in_R8;
    lVar19 = in_R8[1];
    puVar22 = *(uint **)(lVar16 + 0xf8);
    puVar1 = puVar22 + *(longlong *)(lVar16 + 0x100);
    puVar5 = (undefined8 *)*in_R9;
    if (*(char *)(lVar16 + 0x108) == '\0') {
      lVar17 = 8;
      lVar14 = 0;
      lVar7 = 0;
      lVar15 = 0;
      do {
        if (lVar15 == lVar7) {
          do {
            if (puVar22 == puVar1) {
              return;
            }
            uVar3 = *puVar22;
            puVar22 = puVar22 + 1;
            lVar7 = *(longlong *)(*(longlong *)(lVar19 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0);
          } while (lVar7 == 0);
          lVar17 = *(longlong *)(lVar19 + 0x100) + (ulonglong)uVar3 * 0xa0;
          uVar11 = (ulonglong)*(uint *)(lVar17 + 0x94);
          if (*(ulonglong *)(lVar16 + 0x110) <
              *(ulonglong *)(*(longlong *)(lVar19 + 0x1a8) + 0x40 + uVar11 * 0x48)) {
            lVar14 = *(longlong *)(lVar19 + 0x1a8) + uVar11 * 0x48;
            uVar11 = *(ulonglong *)
                      (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar16 + 0x110) * 8);
            if (uVar11 == 0) goto LAB_1422990f4;
            lVar14 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar11 * 0x30);
          }
          else {
LAB_1422990f4:
            lVar14 = 0;
          }
          lVar17 = *(longlong *)(lVar17 + 0x50);
          lVar15 = 0;
        }
        lVar26 = (ulonglong)*(uint *)(lVar17 + 8 + lVar15 * 0x10) * 0xa0;
        uVar13 = *puVar5;
        uVar2 = *(undefined8 *)(lVar14 + 0x50 + lVar26);
        fVar29 = (float)uVar13 - (float)uVar2;
        fVar30 = (float)((ulonglong)uVar13 >> 0x20) - (float)((ulonglong)uVar2 >> 0x20);
        uVar13 = *(undefined8 *)(lVar14 + 0x8c + lVar26);
        if ((float)((ulonglong)uVar13 >> 0x20) * 2.0 + (float)uVar13 * 1.2 <
            SQRT(fVar30 * fVar30 + fVar29 * fVar29)) break;
        lVar26 = lVar26 + lVar14;
        lVar15 = lVar15 + 1;
        country_core::systems::camera_rig::MainCamera::eye_position();
        fVar29 = local_88 - *(float *)(lVar26 + 0x98);
        fVar30 = local_8c - *(float *)(lVar26 + 0x94);
        fVar31 = local_84 - *(float *)(lVar26 + 0x9c);
      } while (SQRT(fVar31 * fVar31 + fVar29 * fVar29 + fVar30 * fVar30) <= 10.0);
    }
    else {
      lVar7 = 0;
      uVar11 = 0;
      uVar12 = 0;
      do {
        if (uVar12 == uVar11) {
          do {
            if (puVar22 == puVar1) {
              return;
            }
            uVar3 = *puVar22;
            puVar22 = puVar22 + 1;
            uVar11 = *(ulonglong *)(*(longlong *)(lVar19 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48);
          } while (uVar11 == 0);
          lVar7 = *(longlong *)(lVar19 + 0x1a8) + (ulonglong)uVar3 * 0x48;
          if ((*(ulonglong *)(lVar16 + 0x110) < *(ulonglong *)(lVar7 + 0x40)) &&
             (uVar12 = *(ulonglong *)
                        (*(longlong *)(lVar7 + 0x38) + *(ulonglong *)(lVar16 + 0x110) * 8),
             uVar12 != 0)) {
            lVar7 = *(longlong *)(*(longlong *)(lVar7 + 0x18) + 0x10 + ~uVar12 * 0x30);
          }
          else {
            lVar7 = 0;
          }
          uVar12 = 0;
        }
        lVar14 = (uVar12 & 0xffffffff) * 0xa0;
        uVar13 = *puVar5;
        uVar2 = *(undefined8 *)(lVar7 + 0x50 + lVar14);
        fVar29 = (float)uVar13 - (float)uVar2;
        fVar30 = (float)((ulonglong)uVar13 >> 0x20) - (float)((ulonglong)uVar2 >> 0x20);
        uVar13 = *(undefined8 *)(lVar7 + 0x8c + lVar14);
        if ((float)((ulonglong)uVar13 >> 0x20) * 2.0 + (float)uVar13 * 1.2 <
            SQRT(fVar30 * fVar30 + fVar29 * fVar29)) break;
        lVar14 = lVar14 + lVar7;
        uVar12 = uVar12 + 1;
        country_core::systems::camera_rig::MainCamera::eye_position();
        fVar29 = local_88 - *(float *)(lVar14 + 0x98);
        fVar30 = local_8c - *(float *)(lVar14 + 0x94);
        fVar31 = local_84 - *(float *)(lVar14 + 0x9c);
      } while (SQRT(fVar31 * fVar31 + fVar29 * fVar29 + fVar30 * fVar30) <= 10.0);
    }
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    lVar16 = *in_stack_00000030;
    lVar19 = *(longlong *)(lVar16 + 0x30);
    uVar13 = *(undefined8 *)(lVar16 + 0x40);
    if (lVar19 != *(longlong *)(lVar16 + 0x20)) goto LAB_14229901b;
  }
  else {
    lVar19 = lVar19 + uVar12 * 0x28;
    if (uVar11 < uVar12) {
      lVar19 = 8;
    }
    piVar20 = (int *)0x0;
    if (uVar12 < uVar11) {
      piVar20 = (int *)(lVar19 + -0x28 + uVar25 * 0x28);
    }
    lVar7 = lVar7 + uVar6 * 0x28;
    if (uVar21 < uVar6) {
      lVar7 = 8;
    }
    piVar8 = (int *)(lVar7 + -0x28 + uVar10 * 0x28);
    if (uVar21 <= uVar6) {
      piVar8 = piVar20;
    }
    if (*piVar8 == 0xc) goto LAB_142298e06;
LAB_142298fef:
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    lVar16 = *in_stack_00000030;
    lVar19 = *(longlong *)(lVar16 + 0x30);
    uVar13 = *(undefined8 *)(lVar16 + 0x40);
    if (lVar19 != *(longlong *)(lVar16 + 0x20)) goto LAB_14229901b;
  }
  alloc::raw_vec::RawVec::
  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
            ();
LAB_14229901b:
  lVar7 = *(longlong *)(lVar16 + 0x28);
  *(undefined8 *)(lVar7 + lVar19 * 0x10) = uVar13;
  *(undefined1 *)(lVar7 + 8 + lVar19 * 0x10) = 1;
  *(longlong *)(lVar16 + 0x30) = lVar19 + 1;
  *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
  return;
}


