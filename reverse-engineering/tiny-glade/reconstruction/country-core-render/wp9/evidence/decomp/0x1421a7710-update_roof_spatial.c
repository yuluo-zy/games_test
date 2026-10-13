// signature: void __cdecl update_roof_spatial(void)

void __cdecl system_roof::update_roof_spatial::update_roof_spatial(void)

{
  longlong lVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  uint uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  undefined4 *puVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  longlong lVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  char extraout_AL;
  undefined1 extraout_AL_00;
  char extraout_AL_01;
  ulonglong extraout_RAX;
  longlong lVar18;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  longlong lVar23;
  ulonglong uVar24;
  longlong lVar25;
  undefined8 *in_R8;
  longlong lVar26;
  longlong *in_R9;
  ulonglong uVar27;
  ulonglong uVar28;
  longlong lVar29;
  longlong lVar30;
  undefined4 extraout_XMM0_Da;
  float fVar31;
  undefined4 local_98;
  float fStack_94;
  undefined4 local_90;
  
  puVar5 = (ulonglong *)*in_R8;
  lVar6 = in_R8[1];
  uVar21 = *puVar5;
  uVar27 = 0;
  uVar24 = uVar21 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar24 = uVar27;
  }
  uVar28 = uVar21 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar28 = uVar27;
  }
  lVar26 = uVar24 * 0x10 + *(longlong *)(lVar6 + 8);
  uVar21 = *(ulonglong *)(lVar6 + 0x10) - uVar24;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar24) {
    lVar26 = 8;
    uVar21 = uVar27;
  }
  lVar29 = uVar28 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar24 = *(ulonglong *)(lVar6 + 0x30) - uVar28;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar28) {
    lVar29 = 8;
    uVar24 = uVar27;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar24) - uVar21;
  lVar30 = uVar21 * 0x10 + lVar26;
  lVar23 = uVar24 * 0x10 + lVar29;
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar2 = *(undefined4 *)((longlong)in_RCX + 0x14);
  uVar3 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar8 = *in_R9;
  puVar9 = (undefined4 *)in_R9[2];
  do {
    do {
      do {
        if ((lVar26 == 0) || (lVar26 == lVar30)) {
          if ((lVar29 == 0) || (lVar29 == lVar23)) {
            return;
          }
          lVar26 = 0;
          lVar25 = lVar29;
          lVar29 = lVar29 + 0x10;
        }
        else {
          lVar25 = lVar26;
          lVar26 = lVar26 + 0x10;
        }
        *puVar5 = *puVar5 + 1;
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
      } while (((extraout_RAX >> 0x20 == 0) ||
               (uVar21 = extraout_RAX & 0xffffffff, *(ulonglong *)(lVar7 + 0x10) <= uVar21)) ||
              (*(int *)(*(longlong *)(lVar7 + 8) + uVar21 * 0x14) != (int)(extraout_RAX >> 0x20)));
      lVar1 = *(longlong *)(lVar7 + 8) + uVar21 * 0x14;
      uVar21 = (ulonglong)*(uint *)(lVar1 + 4);
    } while (((uVar21 == 0xffffffff) || (*(ulonglong *)(lVar6 + 0x38) <= uVar21)) ||
            ((*(ulonglong *)
               (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
              (uVar21 & 0x3f) & 1) == 0));
    uVar21 = (ulonglong)*(uint *)(lVar1 + 0x10);
    lVar20 = *(longlong *)
              (*(longlong *)(lVar7 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar1 = *(longlong *)
             (*(longlong *)(lVar7 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar19 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
    lVar10 = *(longlong *)(lVar20 + 0x28 + lVar19);
    lVar18 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
    lVar1 = *(longlong *)(lVar20 + 0x28 + lVar18);
    lVar19 = uVar21 * 0x58 + *(longlong *)(lVar20 + 0x10 + lVar19);
    lVar20 = uVar21 * 0x30 + *(longlong *)(lVar20 + 0x10 + lVar18);
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    *(undefined4 *)(lVar10 + uVar21 * 4) = uVar2;
    *(undefined4 *)(lVar19 + 0x4c) = extraout_XMM0_Da;
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    if (*(char *)(lVar19 + 0x54) != extraout_AL) {
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      *(undefined4 *)(lVar10 + uVar21 * 4) = uVar2;
      *(undefined1 *)(lVar19 + 0x54) = extraout_AL_00;
      uVar11 = *(undefined8 *)(lVar25 + 8);
      *puVar9 = uVar3;
      lVar18 = *(longlong *)(lVar8 + 0x30);
      uVar12 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar18 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar8 + 0x28);
      lVar22 = lVar18 * 0x20;
      *(undefined8 *)(lVar13 + lVar22) = uVar12;
      *(ulonglong *)(lVar13 + 8 + lVar22) = extraout_RAX;
      *(undefined8 *)(lVar13 + 0x10 + lVar22) = uVar11;
      *(undefined1 *)(lVar13 + 0x18 + lVar22) = 2;
      *(longlong *)(lVar8 + 0x30) = lVar18 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
    uVar4 = *(uint *)(extraout_RAX_00 + 600);
    if (uVar4 < 2) {
      fVar14 = *(float *)(extraout_RAX_00 + 0x25c);
      fVar15 = *(float *)(extraout_RAX_00 + 0x260);
      fVar16 = *(float *)(extraout_RAX_00 + 0x264);
      fVar17 = *(float *)(extraout_RAX_00 + 0x268);
      uVar11 = *(undefined8 *)(extraout_RAX_00 + 0x26c);
      if (uVar4 == *(uint *)(lVar19 + 8)) {
        if ((uVar4 & 1) == 0) {
          if ((fVar14 == *(float *)(lVar19 + 0xc)) &&
             (!NAN(fVar14) && !NAN(*(float *)(lVar19 + 0xc)))) {
            if ((fVar15 == *(float *)(lVar19 + 0x10)) &&
               (!NAN(fVar15) && !NAN(*(float *)(lVar19 + 0x10)))) {
LAB_1421a7ae8:
              if ((fVar16 == *(float *)(lVar19 + 0x14)) &&
                 (!NAN(fVar16) && !NAN(*(float *)(lVar19 + 0x14)))) {
                if ((fVar17 == *(float *)(lVar19 + 0x18)) &&
                   (!NAN(fVar17) && !NAN(*(float *)(lVar19 + 0x18)))) goto LAB_1421a7b9c;
              }
            }
          }
        }
        else if ((fVar14 == *(float *)(lVar19 + 0xc)) &&
                (!NAN(fVar14) && !NAN(*(float *)(lVar19 + 0xc)))) {
          if ((fVar15 == *(float *)(lVar19 + 0x10)) &&
             (!NAN(fVar15) && !NAN(*(float *)(lVar19 + 0x10)))) {
            if (((float)uVar11 == *(float *)(lVar19 + 0x1c)) &&
               (!NAN((float)uVar11) && !NAN(*(float *)(lVar19 + 0x1c)))) {
              fVar31 = (float)((ulonglong)uVar11 >> 0x20);
              if ((fVar31 == *(float *)(lVar19 + 0x20)) &&
                 (!NAN(fVar31) && !NAN(*(float *)(lVar19 + 0x20)))) goto LAB_1421a7ae8;
            }
          }
        }
      }
      *(undefined4 *)(lVar10 + uVar21 * 4) = uVar2;
      *(uint *)(lVar19 + 8) = uVar4;
      *(float *)(lVar19 + 0xc) = fVar14;
      *(float *)(lVar19 + 0x10) = fVar15;
      *(float *)(lVar19 + 0x14) = fVar16;
      *(float *)(lVar19 + 0x18) = fVar17;
      *(undefined8 *)(lVar19 + 0x1c) = uVar11;
      uVar11 = *(undefined8 *)(lVar25 + 8);
      *puVar9 = uVar3;
      lVar25 = *(longlong *)(lVar8 + 0x30);
      uVar12 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar25 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar10 = *(longlong *)(lVar8 + 0x28);
      lVar18 = lVar25 * 0x20;
      *(undefined8 *)(lVar10 + lVar18) = uVar12;
      *(ulonglong *)(lVar10 + 8 + lVar18) = extraout_RAX;
      *(undefined8 *)(lVar10 + 0x10 + lVar18) = uVar11;
      *(undefined1 *)(lVar10 + 0x18 + lVar18) = 2;
      *(longlong *)(lVar8 + 0x30) = lVar25 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
LAB_1421a7b9c:
    country_core::resources::roofs::roof_shape::Roof::ty();
    if (extraout_AL_01 == '\0') {
      country_core::resources::roofs::roof_shape::Roof::pivot_3d();
      *(undefined4 *)(lVar1 + uVar21 * 4) = uVar2;
      *(undefined4 *)(lVar20 + 0x10) = local_98;
      *(float *)(lVar20 + 0x14) = fStack_94 + -0.69;
      *(undefined4 *)(lVar20 + 0x18) = local_90;
    }
    else {
      *(undefined4 *)(lVar1 + uVar21 * 4) = uVar2;
      *(undefined4 *)(lVar20 + 0x18) = 0;
      *(undefined8 *)(lVar20 + 0x10) = 0;
    }
  } while( true );
}

