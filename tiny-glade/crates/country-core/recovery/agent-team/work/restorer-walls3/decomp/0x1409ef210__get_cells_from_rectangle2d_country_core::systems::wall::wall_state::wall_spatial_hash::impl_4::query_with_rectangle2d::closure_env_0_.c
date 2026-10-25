// get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0>  size=1616
// void __cdecl get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0>(void)

void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_rectangle2d::closure_env_0>
          (void)

{
  longlong *plVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  char extraout_AL;
  undefined4 extraout_EAX;
  int iVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  longlong in_RCX;
  int iVar13;
  ulonglong uVar14;
  int iVar15;
  longlong *in_R8;
  longlong in_R9;
  ulonglong uVar16;
  longlong lVar17;
  int iVar18;
  int iVar19;
  ulonglong uVar20;
  int iVar21;
  uint uVar22;
  int iVar23;
  float fVar24;
  int iVar25;
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  float fVar28;
  float fVar34;
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar33 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  float fVar37;
  float fVar38;
  float fVar39;
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  float fVar42;
  float fVar43;
  ulonglong local_108;
  ulonglong local_100;
  undefined1 auVar29 [16];
  undefined1 auVar32 [16];
  
  uVar2 = *(ulonglong *)(in_RCX + 0x50);
  auVar41._8_8_ = 0;
  auVar41._0_8_ = uVar2;
  fVar43 = (float)(uVar2 >> 0x20);
  ::utils::geometry::rectangle::Rectangle2d::aabb();
  ::utils::geometry::aabb::Aabb2::from_center_dims();
  ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
  fVar42 = (float)uVar2;
  fVar39 = fVar42 / (float)*(int *)(in_RCX + 0x48);
  fVar37 = fVar43 / (float)*(int *)(in_RCX + 0x4c);
  ::utils::geometry::rectangle::Rectangle2d::new();
  auVar36._8_8_ = 0;
  auVar36._0_8_ = local_108;
  fVar28 = fVar42 * -0.5;
  fVar34 = fVar43 * -0.5;
  auVar35._4_4_ = fVar34;
  auVar35._0_4_ = fVar28;
  auVar35._8_8_ = 0;
  auVar36 = maxps(auVar35,auVar36);
  uVar22 = -(uint)NAN((float)(local_108 >> 0x20));
  fVar24 = (float)(~-(uint)NAN((float)local_108) & auVar36._0_4_ |
                  (uint)fVar28 & -(uint)NAN((float)local_108));
  fVar38 = (float)(~uVar22 & auVar36._4_4_ | (uint)fVar34 & uVar22);
  iVar21 = -(uint)(fVar28 <= fVar24);
  iVar23 = -(uint)(fVar34 <= fVar38);
  auVar30._4_4_ = iVar23;
  auVar30._0_4_ = iVar21;
  auVar30._8_4_ = iVar23;
  auVar30._12_4_ = iVar23;
  auVar29._8_8_ = auVar30._8_8_;
  auVar29._4_4_ = iVar21;
  auVar29._0_4_ = iVar21;
  iVar21 = movmskpd(extraout_EAX,auVar29);
  if (iVar21 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar42 = fVar42 * 0.5;
  fVar43 = fVar43 * 0.5;
  auVar40._8_8_ = 0;
  auVar40._0_8_ = local_100;
  auVar31._4_4_ = fVar43;
  auVar31._0_4_ = fVar42;
  auVar31._8_8_ = 0;
  auVar36 = minps(auVar31,auVar40);
  uVar22 = -(uint)NAN((float)(local_100 >> 0x20));
  fVar28 = (float)(~-(uint)NAN((float)local_100) & auVar36._0_4_ |
                  (uint)fVar42 & -(uint)NAN((float)local_100));
  fVar34 = (float)(~uVar22 & auVar36._4_4_ | (uint)fVar43 & uVar22);
  iVar21 = -(uint)(fVar28 <= fVar42);
  iVar23 = -(uint)(fVar34 <= fVar43);
  auVar33._4_4_ = iVar23;
  auVar33._0_4_ = iVar21;
  auVar33._8_4_ = iVar23;
  auVar33._12_4_ = iVar23;
  auVar32._8_8_ = auVar33._8_8_;
  auVar32._4_4_ = iVar21;
  auVar32._0_4_ = iVar21;
  iVar21 = movmskpd(3,auVar32);
  if (iVar21 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (fVar39 <= 0.5) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar24 = floorf((fVar24 + fVar42) / fVar39);
  iVar21 = (int)fVar24;
  if (2.1474835e+09 < fVar24) {
    iVar21 = 0x7fffffff;
  }
  if (NAN(fVar24)) {
    iVar21 = 0;
  }
  fVar24 = floorf((fVar38 + fVar43) / fVar37);
  iVar23 = (int)fVar24;
  if (2.1474835e+09 < fVar24) {
    iVar23 = 0x7fffffff;
  }
  if (NAN(fVar24)) {
    iVar23 = 0;
  }
  if (iVar21 < 1) {
    iVar21 = 0;
  }
  if (iVar23 < 1) {
    iVar23 = 0;
  }
  fVar24 = floorf((fVar28 + 0.0 + fVar42) / fVar39);
  iVar18 = (int)fVar24;
  if (2.1474835e+09 < fVar24) {
    iVar18 = 0x7fffffff;
  }
  if (NAN(fVar24)) {
    iVar18 = 0;
  }
  fVar24 = floorf((fVar34 + 0.0 + fVar43) / fVar37);
  iVar19 = (int)fVar24;
  if (2.1474835e+09 < fVar24) {
    iVar19 = 0x7fffffff;
  }
  if (NAN(fVar24)) {
    iVar19 = 0;
  }
  iVar25 = (int)*(undefined8 *)(in_RCX + 0x48);
  iVar9 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
  iVar13 = iVar9 + -1;
  if (iVar25 + -1 < iVar18) {
    iVar18 = iVar25 + -1;
  }
  if (iVar19 <= iVar13) {
    iVar13 = iVar19;
  }
  if (iVar23 <= iVar13) {
    if (iVar18 < iVar21) {
      do {
        if (iVar13 <= iVar23) {
          return;
        }
        iVar23 = iVar23 + (uint)(iVar23 < iVar13);
      } while (iVar23 <= iVar13);
    }
    else {
      if (iVar25 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_const::panic_const_rem_by_zero();
      }
      lVar3 = *(longlong *)(in_R9 + 8);
      uVar2 = *(ulonglong *)(in_R9 + 0x10);
      fVar24 = (float)iVar25;
      fVar38 = (float)iVar9;
      auVar7._4_4_ = fVar38;
      auVar7._0_4_ = fVar24;
      auVar7._8_8_ = 0;
      divps(auVar41,auVar7);
      do {
        iVar9 = (uint)(iVar23 < iVar13) + iVar23;
        iVar19 = iVar21;
        do {
          iVar15 = iVar23 * iVar25 + iVar19;
          if ((iVar25 == -1) && (SBORROW4(0,iVar15))) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_const::panic_const_rem_overflow();
          }
          fVar28 = floorf((float)iVar15 / fVar24);
          iVar10 = (int)fVar28;
          if (2.1474835e+09 < fVar28) {
            iVar10 = 0x7fffffff;
          }
          if (NAN(fVar28)) {
            iVar10 = 0;
          }
          auVar26._0_4_ = (float)(iVar15 % iVar25);
          auVar26._4_4_ = (float)iVar10;
          auVar26._8_8_ = 0;
          auVar8._4_4_ = fVar38;
          auVar8._0_4_ = fVar24;
          auVar8._8_8_ = 0;
          divps(auVar26,auVar8);
          ::utils::geometry::rectangle::Rectangle2d::aabb();
          ::utils::geometry::aabb::Aabb2::intersects();
          if (extraout_AL != '\0') {
            if (uVar2 <= (ulonglong)(longlong)iVar15) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar11 = (longlong)iVar15 * 0x30;
            lVar4 = *(longlong *)(lVar3 + 8 + lVar11);
            uVar5 = *(ulonglong *)(lVar3 + 0x10 + lVar11);
            lVar11 = in_R8[2];
            if ((ulonglong)(*in_R8 - lVar11) < uVar5) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              lVar11 = in_R8[2];
LAB_1409ef6d3:
              lVar6 = in_R8[1];
              uVar14 = uVar5 * 0x38 - 0x38;
              lVar12 = lVar4;
              if (0xa7 < uVar14) {
                uVar14 = uVar14 / 0x38 + 1;
                uVar16 = uVar14 & 0xfffffffffffffffc;
                lVar12 = lVar6 + lVar11 * 8;
                lVar11 = lVar11 + uVar16;
                uVar20 = 0;
                lVar17 = lVar4;
                do {
                  auVar27._8_4_ = (int)lVar17;
                  auVar27._0_8_ = lVar17;
                  auVar27._12_4_ = (int)((ulonglong)lVar17 >> 0x20);
                  plVar1 = (longlong *)(lVar12 + uVar20 * 8);
                  *plVar1 = lVar17;
                  plVar1[1] = auVar27._8_8_ + 0x38;
                  plVar1 = (longlong *)(lVar12 + 0x10 + uVar20 * 8);
                  *plVar1 = lVar17 + 0x70;
                  plVar1[1] = auVar27._8_8_ + 0xa8;
                  uVar20 = uVar20 + 4;
                  lVar17 = lVar17 + 0xe0;
                } while (uVar16 != uVar20);
                lVar12 = uVar16 * 0x38 + lVar4;
                if (uVar14 == uVar16) goto LAB_1409ef790;
              }
              do {
                *(longlong *)(lVar6 + lVar11 * 8) = lVar12;
                lVar12 = lVar12 + 0x38;
                lVar11 = lVar11 + 1;
              } while (lVar12 != lVar4 + uVar5 * 0x38);
            }
            else if (uVar5 != 0) goto LAB_1409ef6d3;
LAB_1409ef790:
            in_R8[2] = lVar11;
          }
        } while ((iVar19 < iVar18) && (iVar19 = iVar19 + (uint)(iVar19 < iVar18), iVar19 <= iVar18))
        ;
      } while ((iVar23 < iVar13) && (iVar23 = iVar9, iVar9 <= iVar13));
    }
  }
  return;
}



// CALLEES: [{'name': 'aabb', 'address': '140c9b680'}, {'name': 'do_reserve_and_handle<alloc::alloc::Global>', 'address': '1428a9a60'}, {'name': 'floorf', 'address': '142923ee0'}, {'name': 'from_center_dims', 'address': '14123fff0'}, {'name': 'intersects', 'address': '141240170'}, {'name': 'new', 'address': '140c98980'}, {'name': 'orientation_mat2', 'address': '140c99540'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_bounds_check', 'address': '1428d9518'}, {'name': 'panic_const_rem_by_zero', 'address': '1428d9a20'}, {'name': 'panic_const_rem_overflow', 'address': '1428d99a0'}]