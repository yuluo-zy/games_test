
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
  float fVar18;
  char extraout_AL;
  undefined1 extraout_AL_00;
  char extraout_AL_01;
  ulonglong extraout_RAX;
  longlong lVar19;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar20;
  longlong lVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong lVar24;
  ulonglong uVar25;
  longlong lVar26;
  undefined8 *in_R8;
  longlong lVar27;
  longlong *in_R9;
  ulonglong uVar28;
  ulonglong uVar29;
  longlong lVar30;
  longlong lVar31;
  undefined4 extraout_XMM0_Da;
  float fVar32;
  undefined4 local_98;
  float fStack_94;
  undefined4 local_90;
  
  puVar5 = (ulonglong *)*in_R8;
  lVar6 = in_R8[1];
  uVar22 = *puVar5;
  uVar28 = 0;
  uVar25 = uVar22 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar25 = uVar28;
  }
  uVar29 = uVar22 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar29 = uVar28;
  }
  lVar27 = uVar25 * 0x10 + *(longlong *)(lVar6 + 8);
  uVar22 = *(ulonglong *)(lVar6 + 0x10) - uVar25;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar25) {
    lVar27 = 8;
    uVar22 = uVar28;
  }
  lVar30 = uVar29 * 0x10 + *(longlong *)(lVar6 + 0x28);
  uVar25 = *(ulonglong *)(lVar6 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar29) {
    lVar30 = 8;
    uVar25 = uVar28;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar25) - uVar22;
  fVar18 = __real_bf30a3d7;
  lVar31 = uVar22 * 0x10 + lVar27;
  lVar24 = uVar25 * 0x10 + lVar30;
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar2 = *(undefined4 *)((longlong)in_RCX + 0x14);
  uVar3 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar8 = *in_R9;
  puVar9 = (undefined4 *)in_R9[2];
  do {
    do {
      do {
        if ((lVar27 == 0) || (lVar27 == lVar31)) {
          if ((lVar30 == 0) || (lVar30 == lVar24)) {
            return;
          }
          lVar27 = 0;
          lVar26 = lVar30;
          lVar30 = lVar30 + 0x10;
        }
        else {
          lVar26 = lVar27;
          lVar27 = lVar27 + 0x10;
        }
        *puVar5 = *puVar5 + 1;
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
      } while (((extraout_RAX >> 0x20 == 0) ||
               (uVar22 = extraout_RAX & 0xffffffff, *(ulonglong *)(lVar7 + 0x10) <= uVar22)) ||
              (*(int *)(*(longlong *)(lVar7 + 8) + uVar22 * 0x14) != (int)(extraout_RAX >> 0x20)));
      lVar1 = *(longlong *)(lVar7 + 8) + uVar22 * 0x14;
      uVar22 = (ulonglong)*(uint *)(lVar1 + 4);
    } while (((uVar22 == 0xffffffff) || (*(ulonglong *)(lVar6 + 0x38) <= uVar22)) ||
            ((*(ulonglong *)
               (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
              (uVar22 & 0x3f) & 1) == 0));
    uVar22 = (ulonglong)*(uint *)(lVar1 + 0x10);
    lVar21 = *(longlong *)
              (*(longlong *)(lVar7 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar1 = *(longlong *)
             (*(longlong *)(lVar7 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
    lVar20 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
    lVar10 = *(longlong *)(lVar21 + 0x28 + lVar20);
    lVar19 = ~*(ulonglong *)(lVar1 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
    lVar1 = *(longlong *)(lVar21 + 0x28 + lVar19);
    lVar20 = uVar22 * 0x58 + *(longlong *)(lVar21 + 0x10 + lVar20);
    lVar21 = uVar22 * 0x30 + *(longlong *)(lVar21 + 0x10 + lVar19);
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_00 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    *(undefined4 *)(lVar10 + uVar22 * 4) = uVar2;
    *(undefined4 *)(lVar20 + 0x4c) = extraout_XMM0_Da;
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    if (*(char *)(lVar20 + 0x54) != extraout_AL) {
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      *(undefined4 *)(lVar10 + uVar22 * 4) = uVar2;
      *(undefined1 *)(lVar20 + 0x54) = extraout_AL_00;
      uVar11 = *(undefined8 *)(lVar26 + 8);
      *puVar9 = uVar3;
      lVar19 = *(longlong *)(lVar8 + 0x30);
      uVar12 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar19 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar8 + 0x28);
      lVar23 = lVar19 * 0x20;
      *(undefined8 *)(lVar13 + lVar23) = uVar12;
      *(ulonglong *)(lVar13 + 8 + lVar23) = extraout_RAX;
      *(undefined8 *)(lVar13 + 0x10 + lVar23) = uVar11;
      *(undefined1 *)(lVar13 + 0x18 + lVar23) = 2;
      *(longlong *)(lVar8 + 0x30) = lVar19 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
    uVar4 = *(uint *)(extraout_RAX_00 + 600);
    if (uVar4 < 2) {
      fVar14 = *(float *)(extraout_RAX_00 + 0x25c);
      fVar15 = *(float *)(extraout_RAX_00 + 0x260);
      fVar16 = *(float *)(extraout_RAX_00 + 0x264);
      fVar17 = *(float *)(extraout_RAX_00 + 0x268);
      uVar11 = *(undefined8 *)(extraout_RAX_00 + 0x26c);
      if (uVar4 == *(uint *)(lVar20 + 8)) {
        if ((uVar4 & 1) == 0) {
          if ((fVar14 == *(float *)(lVar20 + 0xc)) &&
             (!NAN(fVar14) && !NAN(*(float *)(lVar20 + 0xc)))) {
            if ((fVar15 == *(float *)(lVar20 + 0x10)) &&
               (!NAN(fVar15) && !NAN(*(float *)(lVar20 + 0x10)))) {
LAB_1421a7ae8:
              if ((fVar16 == *(float *)(lVar20 + 0x14)) &&
                 (!NAN(fVar16) && !NAN(*(float *)(lVar20 + 0x14)))) {
                if ((fVar17 == *(float *)(lVar20 + 0x18)) &&
                   (!NAN(fVar17) && !NAN(*(float *)(lVar20 + 0x18)))) goto LAB_1421a7b9c;
              }
            }
          }
        }
        else if ((fVar14 == *(float *)(lVar20 + 0xc)) &&
                (!NAN(fVar14) && !NAN(*(float *)(lVar20 + 0xc)))) {
          if ((fVar15 == *(float *)(lVar20 + 0x10)) &&
             (!NAN(fVar15) && !NAN(*(float *)(lVar20 + 0x10)))) {
            if (((float)uVar11 == *(float *)(lVar20 + 0x1c)) &&
               (!NAN((float)uVar11) && !NAN(*(float *)(lVar20 + 0x1c)))) {
              fVar32 = (float)((ulonglong)uVar11 >> 0x20);
              if ((fVar32 == *(float *)(lVar20 + 0x20)) &&
                 (!NAN(fVar32) && !NAN(*(float *)(lVar20 + 0x20)))) goto LAB_1421a7ae8;
            }
          }
        }
      }
      *(undefined4 *)(lVar10 + uVar22 * 4) = uVar2;
      *(uint *)(lVar20 + 8) = uVar4;
      *(float *)(lVar20 + 0xc) = fVar14;
      *(float *)(lVar20 + 0x10) = fVar15;
      *(float *)(lVar20 + 0x14) = fVar16;
      *(float *)(lVar20 + 0x18) = fVar17;
      *(undefined8 *)(lVar20 + 0x1c) = uVar11;
      uVar11 = *(undefined8 *)(lVar26 + 8);
      *puVar9 = uVar3;
      lVar26 = *(longlong *)(lVar8 + 0x30);
      uVar12 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar26 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar10 = *(longlong *)(lVar8 + 0x28);
      lVar19 = lVar26 * 0x20;
      *(undefined8 *)(lVar10 + lVar19) = uVar12;
      *(ulonglong *)(lVar10 + 8 + lVar19) = extraout_RAX;
      *(undefined8 *)(lVar10 + 0x10 + lVar19) = uVar11;
      *(undefined1 *)(lVar10 + 0x18 + lVar19) = 2;
      *(longlong *)(lVar8 + 0x30) = lVar26 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
LAB_1421a7b9c:
    country_core::resources::roofs::roof_shape::Roof::ty();
    if (extraout_AL_01 == '\0') {
      country_core::resources::roofs::roof_shape::Roof::pivot_3d();
      *(undefined4 *)(lVar1 + uVar22 * 4) = uVar2;
      *(undefined4 *)(lVar21 + 0x10) = local_98;
      *(float *)(lVar21 + 0x14) = fStack_94 + fVar18;
      *(undefined4 *)(lVar21 + 0x18) = local_90;
    }
    else {
      *(undefined4 *)(lVar1 + uVar22 * 4) = uVar2;
      *(undefined4 *)(lVar21 + 0x18) = 0;
      *(undefined8 *)(lVar21 + 0x10) = 0;
    }
  } while( true );
}

