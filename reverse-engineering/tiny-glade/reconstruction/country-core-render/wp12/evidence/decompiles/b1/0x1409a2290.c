// set_wall_style addr=0x1409a2290 size=2642 sig=void __cdecl set_wall_style(void)

void __cdecl country_core::systems::wall::set_wall_style::set_wall_style(void)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  longlong *plVar10;
  undefined4 *puVar11;
  longlong lVar12;
  undefined8 uVar13;
  uint uVar14;
  uint uVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  uint extraout_EAX;
  longlong lVar20;
  ulonglong extraout_RAX;
  longlong lVar21;
  undefined8 *in_RCX;
  longlong lVar22;
  ulonglong uVar23;
  undefined8 *in_RDX;
  ulonglong uVar24;
  ulonglong uVar25;
  longlong lVar26;
  longlong lVar27;
  undefined8 *in_R8;
  longlong lVar28;
  ulonglong uVar29;
  uint uVar30;
  longlong in_R9;
  ushort uVar31;
  longlong lVar32;
  longlong lVar33;
  byte bVar34;
  char cVar35;
  char cVar39;
  float extraout_XMM0_Da;
  char cVar37;
  char cVar38;
  undefined1 auVar36 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  float fVar42;
  float fVar43;
  float fVar44;
  longlong *in_stack_00000028;
  longlong in_stack_00000030;
  longlong *in_stack_00000038;
  longlong local_100;
  
  puVar7 = (ulonglong *)*in_RCX;
  lVar26 = in_RCX[1];
  uVar24 = *puVar7;
  uVar25 = 0;
  uVar23 = uVar24 - *(ulonglong *)(lVar26 + 0x18);
  if (uVar24 < *(ulonglong *)(lVar26 + 0x18)) {
    uVar23 = uVar25;
  }
  uVar29 = uVar24 - *(ulonglong *)(lVar26 + 0x38);
  if (uVar24 < *(ulonglong *)(lVar26 + 0x38)) {
    uVar29 = uVar25;
  }
  lVar20 = uVar23 * 0x18 + *(longlong *)(lVar26 + 8);
  uVar24 = *(ulonglong *)(lVar26 + 0x10) - uVar23;
  if (*(ulonglong *)(lVar26 + 0x10) < uVar23) {
    lVar20 = 8;
    uVar24 = uVar25;
  }
  lVar22 = uVar29 * 0x18 + *(longlong *)(lVar26 + 0x28);
  uVar23 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
    lVar22 = 8;
    uVar23 = uVar25;
  }
  *puVar7 = (*(longlong *)(lVar26 + 0x40) - uVar23) - uVar24;
  lVar26 = lVar20 + uVar24 * 0x18;
  lVar33 = lVar22 + uVar23 * 0x18;
  uVar4 = *(undefined4 *)(in_R9 + 0x1c);
  puVar8 = *(undefined4 **)(in_R9 + 0x10);
  uVar5 = *(undefined4 *)(in_stack_00000030 + 0x1c);
  puVar9 = *(undefined4 **)(in_stack_00000030 + 0x10);
  plVar10 = (longlong *)*in_stack_00000028;
  uVar6 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar27 = *in_stack_00000038;
  puVar11 = (undefined4 *)in_stack_00000038[2];
  iVar16 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                  CONCAT13(unaff_XMM7_Bd,
                           CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                  CONCAT13(unaff_XMM7_Bh,
                           CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
  iVar18 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                  CONCAT13(unaff_XMM7_Bl,
                           CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
  iVar19 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                  CONCAT13(unaff_XMM7_Bp,
                           CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
  do {
    do {
      if ((lVar20 == 0) || (lVar20 == lVar26)) {
        if ((lVar22 == 0) || (lVar22 == lVar33)) {
          puVar7 = (ulonglong *)*in_R8;
          lVar26 = in_R8[1];
          uVar24 = *puVar7;
          uVar25 = 0;
          uVar23 = uVar24 - *(ulonglong *)(lVar26 + 0x18);
          if (uVar24 < *(ulonglong *)(lVar26 + 0x18)) {
            uVar23 = uVar25;
          }
          uVar29 = uVar24 - *(ulonglong *)(lVar26 + 0x38);
          if (uVar24 < *(ulonglong *)(lVar26 + 0x38)) {
            uVar29 = uVar25;
          }
          lVar20 = uVar23 * 0x20 + *(longlong *)(lVar26 + 8);
          uVar24 = *(ulonglong *)(lVar26 + 0x10) - uVar23;
          if (*(ulonglong *)(lVar26 + 0x10) < uVar23) {
            lVar20 = 8;
            uVar24 = uVar25;
          }
          lVar22 = uVar29 * 0x20 + *(longlong *)(lVar26 + 0x28);
          uVar23 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
          if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
            lVar22 = 8;
            uVar23 = uVar25;
          }
          *puVar7 = (*(longlong *)(lVar26 + 0x40) - uVar23) - uVar24;
          lVar33 = uVar24 * 0x20 + lVar20;
          lVar26 = lVar22;
          do {
            if ((lVar20 == 0) || (lVar20 == lVar33)) {
              if ((lVar26 == 0) || (lVar26 == uVar23 * 0x20 + lVar22)) {
                puVar7 = (ulonglong *)*in_RDX;
                lVar26 = in_RDX[1];
                uVar24 = *puVar7;
                uVar25 = 0;
                uVar23 = uVar24 - *(ulonglong *)(lVar26 + 0x18);
                if (uVar24 < *(ulonglong *)(lVar26 + 0x18)) {
                  uVar23 = uVar25;
                }
                uVar29 = uVar24 - *(ulonglong *)(lVar26 + 0x38);
                if (uVar24 < *(ulonglong *)(lVar26 + 0x38)) {
                  uVar29 = uVar25;
                }
                lVar20 = uVar23 * 0x18 + *(longlong *)(lVar26 + 8);
                uVar24 = *(ulonglong *)(lVar26 + 0x10) - uVar23;
                if (*(ulonglong *)(lVar26 + 0x10) < uVar23) {
                  lVar20 = 8;
                  uVar24 = uVar25;
                }
                local_100 = uVar29 * 0x18 + *(longlong *)(lVar26 + 0x28);
                uVar23 = *(ulonglong *)(lVar26 + 0x30) - uVar29;
                if (*(ulonglong *)(lVar26 + 0x30) < uVar29) {
                  local_100 = 8;
                  uVar23 = uVar25;
                }
                *puVar7 = (*(longlong *)(lVar26 + 0x40) - uVar23) - uVar24;
                lVar26 = lVar20 + uVar24 * 0x18;
                lVar22 = local_100 + uVar23 * 0x18;
                do {
                  if ((lVar20 == 0) || (lVar20 == lVar26)) {
                    if ((local_100 == 0) || (local_100 == lVar22)) {
                      return;
                    }
                    lVar20 = 0;
                    lVar33 = local_100;
                    local_100 = local_100 + 0x18;
                  }
                  else {
                    lVar33 = lVar20;
                    lVar20 = lVar20 + 0x18;
                  }
                  *puVar7 = *puVar7 + 1;
                  cVar35 = *(char *)(lVar33 + 0x12);
                  *puVar8 = uVar4;
                  resources::walls::inner_walls::InnerWalls::
                  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_freehand_ty::closure_env_0>
                            ();
                  *puVar8 = uVar4;
                  resources::walls::inner_walls::InnerWalls::
                  _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_arch_type::closure_env_0>
                            ();
                  if (cVar35 == '\x01') {
                    resources::history::wall_compound_edit::CompoundEditBuilder::next();
                    resources::history::wall_compound_edit::CompoundEditBuilder::next();
                    *puVar9 = uVar5;
                    alloc::boxed::iter::impl$13::
                    from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                              ();
                    resources::history::History::add();
                  }
                } while( true );
              }
              lVar27 = lVar26 + 0x20;
              lVar20 = 0;
              lVar21 = lVar26;
            }
            else {
              lVar21 = lVar20;
              lVar27 = lVar26;
              lVar20 = lVar20 + 0x20;
            }
            *puVar7 = *puVar7 + 1;
            cVar35 = *(char *)(lVar21 + 0x18);
            *puVar8 = uVar4;
            resources::walls::inner_walls::InnerWalls::
            _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_plaster_peel::closure_env_0>
                      ();
            lVar26 = lVar27;
            if (cVar35 != '\0') {
              resources::history::wall_compound_edit::CompoundEditBuilder::next();
              *puVar9 = uVar5;
              alloc::boxed::iter::impl$13::
              from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                        ();
              resources::history::History::add();
            }
          } while( true );
        }
        lVar20 = 0;
        lVar21 = lVar22;
        lVar22 = lVar22 + 0x18;
      }
      else {
        lVar21 = lVar20;
        lVar20 = lVar20 + 0x18;
      }
      *puVar7 = *puVar7 + 1;
      uVar3 = *(undefined1 *)(lVar21 + 0x11);
      lVar12 = *(longlong *)(lVar21 + 8);
      cVar35 = *(char *)(lVar21 + 0x13);
      cVar37 = *(char *)(lVar21 + 0x12);
      *puVar8 = uVar4;
      resources::walls::inner_walls::InnerWalls::
      _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_style::closure_env_0>
                ();
      if (cVar37 != '\0') {
        resources::history::wall_compound_edit::CompoundEditBuilder::next();
        *puVar9 = uVar5;
        alloc::boxed::iter::impl$13::
        from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                  ();
        resources::history::History::add();
      }
    } while (cVar35 == '\0');
    if (plVar10[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar21 = *plVar10;
      bVar34 = (byte)(extraout_RAX >> 0x39);
      auVar36 = ZEXT216(CONCAT11(bVar34,bVar34));
      auVar36 = pshuflw(auVar36,auVar36,0);
      lVar28 = 0;
      uVar24 = extraout_RAX;
      while( true ) {
        uVar24 = plVar10[1] & uVar24;
        pcVar1 = (char *)(lVar21 + uVar24);
        cVar35 = auVar36[0];
        auVar41[0] = -(*pcVar1 == cVar35);
        cVar37 = auVar36[1];
        auVar41[1] = -(pcVar1[1] == cVar37);
        cVar38 = auVar36[2];
        auVar41[2] = -(pcVar1[2] == cVar38);
        cVar39 = auVar36[3];
        auVar41[3] = -(pcVar1[3] == cVar39);
        auVar41[4] = -(pcVar1[4] == cVar35);
        auVar41[5] = -(pcVar1[5] == cVar37);
        auVar41[6] = -(pcVar1[6] == cVar38);
        auVar41[7] = -(pcVar1[7] == cVar39);
        auVar41[8] = -(pcVar1[8] == cVar35);
        auVar41[9] = -(pcVar1[9] == cVar37);
        auVar41[10] = -(pcVar1[10] == cVar38);
        auVar41[0xb] = -(pcVar1[0xb] == cVar39);
        auVar41[0xc] = -(pcVar1[0xc] == cVar35);
        auVar41[0xd] = -(pcVar1[0xd] == cVar37);
        auVar41[0xe] = -(pcVar1[0xe] == cVar38);
        auVar41[0xf] = -(pcVar1[0xf] == cVar39);
        uVar31 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf
        ;
        uVar30 = (uint)uVar31;
        while (uVar31 != 0) {
          uVar14 = 0;
          for (uVar15 = uVar30; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          lVar32 = (uVar14 + uVar24 & plVar10[1]) * -0x2a8;
          if (*(longlong *)(lVar21 + -0x2a8 + lVar32) == lVar12) {
            if ((*(ulonglong *)(lVar21 + -0x290 + lVar32) < 2) ||
               (*(float *)(lVar21 + -0x270 + lVar32) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                      ();
            uVar23 = (ulonglong)extraout_EAX;
            uVar24 = *(ulonglong *)(lVar21 + lVar32 + -600);
            if (uVar24 <= uVar23) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar25 = uVar23 + 1;
            if (uVar24 <= uVar25) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar21 = *(longlong *)(lVar21 + lVar32 + -0x260);
            fVar44 = 1.0 - extraout_XMM0_Da;
            uVar13 = *(undefined8 *)(lVar21 + uVar23 * 0xc);
            uVar2 = *(undefined8 *)(lVar21 + uVar25 * 0xc);
            fVar42 = extraout_XMM0_Da * (float)uVar2 + (float)uVar13 * fVar44;
            fVar43 = extraout_XMM0_Da * (float)((ulonglong)uVar2 >> 0x20) +
                     (float)((ulonglong)uVar13 >> 0x20) * fVar44;
            fVar44 = *(float *)(lVar21 + 8 + uVar25 * 0xc) * extraout_XMM0_Da +
                     fVar44 * *(float *)(lVar21 + 8 + uVar23 * 0xc);
            goto LAB_1409a277d;
          }
          uVar31 = (ushort)(uVar30 - 1) & (ushort)uVar30;
          uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar31);
        }
        auVar40[0] = -(*pcVar1 == (char)iVar16);
        auVar40[1] = -(pcVar1[1] == (char)((uint)iVar16 >> 8));
        auVar40[2] = -(pcVar1[2] == (char)((uint)iVar16 >> 0x10));
        auVar40[3] = -(pcVar1[3] == (char)((uint)iVar16 >> 0x18));
        auVar40[4] = -(pcVar1[4] == (char)iVar17);
        auVar40[5] = -(pcVar1[5] == (char)((uint)iVar17 >> 8));
        auVar40[6] = -(pcVar1[6] == (char)((uint)iVar17 >> 0x10));
        auVar40[7] = -(pcVar1[7] == (char)((uint)iVar17 >> 0x18));
        auVar40[8] = -(pcVar1[8] == (char)iVar18);
        auVar40[9] = -(pcVar1[9] == (char)((uint)iVar18 >> 8));
        auVar40[10] = -(pcVar1[10] == (char)((uint)iVar18 >> 0x10));
        auVar40[0xb] = -(pcVar1[0xb] == (char)((uint)iVar18 >> 0x18));
        auVar40[0xc] = -(pcVar1[0xc] == (char)iVar19);
        auVar40[0xd] = -(pcVar1[0xd] == (char)((uint)iVar19 >> 8));
        auVar40[0xe] = -(pcVar1[0xe] == (char)((uint)iVar19 >> 0x10));
        auVar40[0xf] = -(pcVar1[0xf] == (char)((uint)iVar19 >> 0x18));
        if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar40 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar40 >> 0x1f,0) & 1) != 0) || (SUB161(auVar40 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar40 >> 0x37,0) & 1) != 0) || (SUB161(auVar40 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar40 >> 0x47,0) & 1) != 0) || (SUB161(auVar40 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar40 >> 0x57,0) & 1) != 0) || (SUB161(auVar40 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar40 >> 0x67,0) & 1) != 0) || (SUB161(auVar40 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar40 >> 0x77,0) & 1) != 0) || auVar40[0xf] < '\0') break;
        uVar24 = uVar24 + lVar28 + 0x10;
        lVar28 = lVar28 + 0x10;
      }
    }
    fVar42 = 0.0;
    fVar43 = 0.0;
    fVar44 = 0.0;
LAB_1409a277d:
    *puVar11 = uVar6;
    lVar21 = *(longlong *)(lVar27 + 0x30);
    uVar13 = *(undefined8 *)(lVar27 + 0x40);
    if (lVar21 == *(longlong *)(lVar27 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar12 = *(longlong *)(lVar27 + 0x28);
    lVar28 = lVar21 * 0x20;
    *(undefined8 *)(lVar12 + lVar28) = uVar13;
    *(undefined1 *)(lVar12 + 8 + lVar28) = 0x10;
    *(undefined1 *)(lVar12 + 9 + lVar28) = uVar3;
    *(ulonglong *)(lVar12 + 0x10 + lVar28) = CONCAT44(fVar43,fVar42);
    *(float *)(lVar12 + 0x18 + lVar28) = fVar44;
    *(longlong *)(lVar27 + 0x30) = lVar21 + 1;
    *(longlong *)(lVar27 + 0x40) = *(longlong *)(lVar27 + 0x40) + 1;
  } while( true );
}

