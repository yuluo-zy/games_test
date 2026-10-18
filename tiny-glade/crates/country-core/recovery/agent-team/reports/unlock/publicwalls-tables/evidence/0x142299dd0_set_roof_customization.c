
void __cdecl system_color::set_roof_customization::set_roof_customization(void)

{
  undefined8 *puVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  undefined4 *puVar14;
  undefined8 uVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined8 uVar18;
  longlong lVar19;
  char cVar20;
  ulonglong extraout_RAX;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong extraout_RAX_00;
  ulonglong uVar23;
  longlong lVar24;
  ulonglong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong lVar25;
  longlong lVar26;
  ulonglong uVar27;
  longlong lVar28;
  undefined8 *in_RDX;
  longlong lVar29;
  ulonglong uVar30;
  longlong *in_R9;
  longlong lVar31;
  longlong lVar32;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong in_stack_00000040;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined4 local_88;
  
  puVar9 = (ulonglong *)*in_RCX;
  lVar10 = in_RCX[1];
  uVar21 = *puVar9;
  uVar23 = 0;
  uVar30 = uVar21 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar30 = uVar23;
  }
  uVar27 = uVar21 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar27 = uVar23;
  }
  lVar25 = uVar30 * 0x18 + *(longlong *)(lVar10 + 8);
  uVar21 = *(ulonglong *)(lVar10 + 0x10) - uVar30;
  if (*(ulonglong *)(lVar10 + 0x10) < uVar30) {
    lVar25 = 8;
    uVar21 = uVar23;
  }
  lVar31 = uVar27 * 0x18 + *(longlong *)(lVar10 + 0x28);
  uVar30 = *(ulonglong *)(lVar10 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar27) {
    lVar31 = 8;
    uVar30 = uVar23;
  }
  *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar30) - uVar21;
  lVar10 = lVar25 + uVar21 * 0x18;
  lVar29 = lVar31 + uVar30 * 0x18;
  lVar11 = *in_R9;
  lVar12 = in_R9[1];
  uVar4 = *(undefined4 *)((longlong)in_R9 + 0x14);
  uVar5 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar13 = *in_stack_00000028;
  puVar14 = (undefined4 *)in_stack_00000028[2];
  do {
    if ((lVar25 == 0) || (lVar25 == lVar10)) {
      if ((lVar31 == 0) || (lVar31 == lVar29)) {
        puVar9 = (ulonglong *)*in_RDX;
        lVar10 = in_RDX[1];
        uVar21 = *puVar9;
        uVar23 = 0;
        uVar30 = uVar21 - *(ulonglong *)(lVar10 + 0x18);
        if (uVar21 < *(ulonglong *)(lVar10 + 0x18)) {
          uVar30 = uVar23;
        }
        uVar27 = uVar21 - *(ulonglong *)(lVar10 + 0x38);
        if (uVar21 < *(ulonglong *)(lVar10 + 0x38)) {
          uVar27 = uVar23;
        }
        lVar25 = uVar30 * 0x20 + *(longlong *)(lVar10 + 8);
        uVar21 = *(ulonglong *)(lVar10 + 0x10) - uVar30;
        if (*(ulonglong *)(lVar10 + 0x10) < uVar30) {
          lVar25 = 8;
          uVar21 = uVar23;
        }
        lVar31 = uVar27 * 0x20 + *(longlong *)(lVar10 + 0x28);
        uVar30 = *(ulonglong *)(lVar10 + 0x30) - uVar27;
        if (*(ulonglong *)(lVar10 + 0x30) < uVar27) {
          lVar31 = 8;
          uVar30 = uVar23;
        }
        *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar30) - uVar21;
        uVar7 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        lVar10 = *in_stack_00000030;
        puVar16 = (undefined4 *)in_stack_00000030[2];
        lVar29 = lVar25;
        lVar32 = lVar31;
        do {
          if ((lVar29 == 0) || (lVar29 == uVar21 * 0x20 + lVar25)) {
            if ((lVar32 == 0) || (lVar32 == uVar30 * 0x20 + lVar31)) {
              puVar9 = (ulonglong *)*in_stack_00000038;
              lVar25 = in_stack_00000038[1];
              uVar21 = *puVar9;
              uVar23 = 0;
              uVar30 = uVar21 - *(ulonglong *)(lVar25 + 0x18);
              if (uVar21 < *(ulonglong *)(lVar25 + 0x18)) {
                uVar30 = uVar23;
              }
              uVar27 = uVar21 - *(ulonglong *)(lVar25 + 0x38);
              if (uVar21 < *(ulonglong *)(lVar25 + 0x38)) {
                uVar27 = uVar23;
              }
              lVar31 = uVar30 * 0x18 + *(longlong *)(lVar25 + 8);
              uVar21 = *(ulonglong *)(lVar25 + 0x10) - uVar30;
              if (*(ulonglong *)(lVar25 + 0x10) < uVar30) {
                lVar31 = 8;
                uVar21 = uVar23;
              }
              lVar29 = uVar27 * 0x18 + *(longlong *)(lVar25 + 0x28);
              uVar30 = *(ulonglong *)(lVar25 + 0x30) - uVar27;
              if (*(ulonglong *)(lVar25 + 0x30) < uVar27) {
                lVar29 = 8;
                uVar30 = uVar23;
              }
              *puVar9 = (*(longlong *)(lVar25 + 0x40) - uVar30) - uVar21;
              lVar25 = lVar31 + uVar21 * 0x18;
              lVar13 = lVar29 + uVar30 * 0x18;
              uVar5 = *(undefined4 *)(in_stack_00000040 + 0x1c);
              puVar14 = *(undefined4 **)(in_stack_00000040 + 0x10);
              do {
                if ((lVar31 == 0) || (lVar31 == lVar25)) {
                  if ((lVar29 == 0) || (lVar29 == lVar13)) {
                    return;
                  }
                  lVar32 = lVar29 + 0x18;
                  lVar24 = 0;
                  lVar26 = lVar29;
                }
                else {
                  lVar32 = lVar29;
                  lVar24 = lVar31 + 0x18;
                  lVar26 = lVar31;
                }
                lVar31 = lVar24;
                lVar29 = lVar32;
                *puVar9 = *puVar9 + 1;
                country_core::resources::walls::public_walls::PublicWalls::roof_entity();
                if (((extraout_RAX_01 >> 0x20 != 0) &&
                    (uVar21 = extraout_RAX_01 & 0xffffffff, uVar21 < *(ulonglong *)(lVar12 + 0x10)))
                   && (*(int *)(*(longlong *)(lVar12 + 8) + uVar21 * 0x14) ==
                       (int)(extraout_RAX_01 >> 0x20))) {
                  lVar32 = *(longlong *)(lVar12 + 8) + uVar21 * 0x14;
                  uVar21 = (ulonglong)*(uint *)(lVar32 + 4);
                  if (((uVar21 != 0xffffffff) && (uVar21 < *(ulonglong *)(lVar11 + 0x38))) &&
                     ((*(ulonglong *)
                        (*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar32 + 4) >> 6) * 8)
                       >> (uVar21 & 0x3f) & 1) != 0)) {
                    uVar6 = *(uint *)(lVar32 + 0x10);
                    lVar24 = *(longlong *)
                              (*(longlong *)(lVar12 + 0x1a8) + 0x18 +
                              (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48);
                    lVar22 = ~*(ulonglong *)
                               (*(longlong *)
                                 (*(longlong *)(lVar12 + 0x1a8) + 0x38 +
                                 (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48) +
                               *(longlong *)(lVar11 + 0x110) * 8) * 0x30;
                    lVar32 = *(longlong *)(lVar24 + 0x10 + lVar22);
                    lVar24 = *(longlong *)(lVar24 + 0x28 + lVar22);
                    lVar22 = (ulonglong)uVar6 * 0x58;
                    cVar20 = *(char *)(lVar32 + 0x3d + lVar22);
                    if (cVar20 != *(char *)(lVar26 + 0x11)) {
                      uVar15 = *(undefined8 *)(lVar26 + 8);
                      *puVar16 = uVar7;
                      lVar17 = *(longlong *)(lVar10 + 0x30);
                      uVar18 = *(undefined8 *)(lVar10 + 0x40);
                      if (lVar17 == *(longlong *)(lVar10 + 0x20)) {
                        alloc::raw_vec::RawVec::
                        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                                  ();
                      }
                      lVar28 = *(longlong *)(lVar10 + 0x28);
                      *(undefined8 *)(lVar28 + lVar17 * 0x10) = uVar18;
                      *(undefined8 *)(lVar28 + 8 + lVar17 * 0x10) = uVar15;
                      *(longlong *)(lVar10 + 0x30) = lVar17 + 1;
                      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
                      cVar20 = *(char *)(lVar26 + 0x11);
                    }
                    *(undefined4 *)(lVar24 + (ulonglong)uVar6 * 4) = uVar4;
                    *(char *)(lVar32 + lVar22 + 0x3d) = cVar20;
                    if (*(char *)(lVar26 + 0x12) == '\x01') {
                      *puVar14 = uVar5;
                      uVar15 = *(undefined8 *)(lVar26 + 8);
                      uVar2 = *(undefined1 *)(lVar26 + 0x10);
                      uVar3 = *(undefined1 *)(lVar26 + 0x11);
                      std::alloc::__default_lib_allocator::__rust_alloc();
                      if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                        alloc::alloc::handle_alloc_error();
                      }
                      *extraout_RAX_02 = 1;
                      extraout_RAX_02[1] = 1;
                      *(undefined1 *)(extraout_RAX_02 + 2) = 4;
                      *(undefined1 *)((longlong)extraout_RAX_02 + 0x11) = uVar2;
                      *(undefined1 *)((longlong)extraout_RAX_02 + 0x12) = uVar3;
                      extraout_RAX_02[3] = uVar15;
                      country_core::resources::history::History::add();
                    }
                  }
                }
              } while( true );
            }
            lVar26 = lVar32 + 0x20;
            lVar24 = 0;
            lVar22 = lVar32;
          }
          else {
            lVar24 = lVar29 + 0x20;
            lVar26 = lVar32;
            lVar22 = lVar29;
          }
          *puVar9 = *puVar9 + 1;
          country_core::resources::walls::public_walls::PublicWalls::roof_entity();
          lVar29 = lVar24;
          lVar32 = lVar26;
          if (((extraout_RAX_00 >> 0x20 != 0) &&
              (uVar23 = extraout_RAX_00 & 0xffffffff, uVar23 < *(ulonglong *)(lVar12 + 0x10))) &&
             (*(int *)(*(longlong *)(lVar12 + 8) + uVar23 * 0x14) == (int)(extraout_RAX_00 >> 0x20))
             ) {
            lVar24 = *(longlong *)(lVar12 + 8) + uVar23 * 0x14;
            uVar23 = (ulonglong)*(uint *)(lVar24 + 4);
            if (((uVar23 != 0xffffffff) && (uVar23 < *(ulonglong *)(lVar11 + 0x38))) &&
               ((*(ulonglong *)
                  (*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar24 + 4) >> 6) * 8) >>
                 (uVar23 & 0x3f) & 1) != 0)) {
              uVar6 = *(uint *)(lVar24 + 0x10);
              lVar26 = *(longlong *)
                        (*(longlong *)(lVar12 + 0x1a8) + 0x18 +
                        (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48);
              lVar24 = ~*(ulonglong *)
                         (*(longlong *)
                           (*(longlong *)(lVar12 + 0x1a8) + 0x38 +
                           (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48) +
                         *(longlong *)(lVar11 + 0x110) * 8) * 0x30;
              lVar17 = *(longlong *)(lVar26 + 0x10 + lVar24);
              lVar26 = *(longlong *)(lVar26 + 0x28 + lVar24);
              lVar28 = (ulonglong)uVar6 * 0x58;
              lVar24 = lVar17 + lVar28;
              cVar20 = *(char *)(lVar17 + 0x40 + lVar28);
              if ((cVar20 != *(char *)(lVar22 + 0x10)) ||
                 ((cVar20 == '\x02' &&
                  (((*(int *)(lVar24 + 0x44) != *(int *)(lVar22 + 0x14) ||
                    (*(char *)(lVar24 + 0x41) != *(char *)(lVar22 + 0x11))) ||
                   (*(int *)(lVar24 + 0x48) != *(int *)(lVar22 + 0x18))))))) {
                uVar15 = *(undefined8 *)(lVar22 + 8);
                *puVar16 = uVar7;
                lVar24 = *(longlong *)(lVar10 + 0x30);
                uVar18 = *(undefined8 *)(lVar10 + 0x40);
                if (lVar24 == *(longlong *)(lVar10 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar19 = *(longlong *)(lVar10 + 0x28);
                *(undefined8 *)(lVar19 + lVar24 * 0x10) = uVar18;
                *(undefined8 *)(lVar19 + 8 + lVar24 * 0x10) = uVar15;
                *(longlong *)(lVar10 + 0x30) = lVar24 + 1;
                *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
              }
              uVar8 = *(undefined4 *)(lVar22 + 0x18);
              uStack_90 = CONCAT44(uStack_90._4_4_,uVar8);
              uVar15 = *(undefined8 *)(lVar22 + 0x10);
              *(undefined4 *)(lVar26 + (ulonglong)uVar6 * 4) = uVar4;
              *(undefined4 *)(lVar17 + lVar28 + 0x48) = uVar8;
              *(undefined8 *)(lVar17 + lVar28 + 0x40) = uVar15;
              if (*(char *)(lVar22 + 0x1c) == '\x01') {
                country_core::resources::roofs::roof_shape::Roof::pivot_3d();
                local_98._1_7_ = (undefined7)((ulonglong)uVar15 >> 8);
                local_98 = CONCAT71(local_98._1_7_,0xb);
                *puVar14 = uVar5;
                lVar24 = *(longlong *)(lVar13 + 0x30);
                uVar15 = *(undefined8 *)(lVar13 + 0x40);
                if (lVar24 == *(longlong *)(lVar13 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                            ();
                }
                lVar26 = *(longlong *)(lVar13 + 0x28);
                lVar22 = lVar24 * 0x20;
                *(undefined8 *)(lVar26 + lVar22) = uVar15;
                *(undefined4 *)(lVar26 + 0x18 + lVar22) = local_88;
                puVar1 = (undefined8 *)(lVar26 + 8 + lVar22);
                *puVar1 = local_98;
                puVar1[1] = uStack_90;
                *(longlong *)(lVar13 + 0x30) = lVar24 + 1;
                *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
              }
            }
          }
        } while( true );
      }
      lVar32 = 0;
      lVar24 = lVar31;
      lVar31 = lVar31 + 0x18;
    }
    else {
      lVar32 = lVar25 + 0x18;
      lVar24 = lVar25;
    }
    lVar25 = lVar32;
    *puVar9 = *puVar9 + 1;
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (((extraout_RAX >> 0x20 != 0) &&
        (uVar21 = extraout_RAX & 0xffffffff, uVar21 < *(ulonglong *)(lVar12 + 0x10))) &&
       (*(int *)(*(longlong *)(lVar12 + 8) + uVar21 * 0x14) == (int)(extraout_RAX >> 0x20))) {
      lVar32 = *(longlong *)(lVar12 + 8) + uVar21 * 0x14;
      uVar21 = (ulonglong)*(uint *)(lVar32 + 4);
      if (((uVar21 != 0xffffffff) && (uVar21 < *(ulonglong *)(lVar11 + 0x38))) &&
         ((*(ulonglong *)
            (*(longlong *)(lVar11 + 0x28) + (ulonglong)(*(uint *)(lVar32 + 4) >> 6) * 8) >>
           (uVar21 & 0x3f) & 1) != 0)) {
        uVar6 = *(uint *)(lVar32 + 0x10);
        lVar26 = *(longlong *)
                  (*(longlong *)(lVar12 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48)
        ;
        lVar22 = ~*(ulonglong *)
                   (*(longlong *)
                     (*(longlong *)(lVar12 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48) + *(longlong *)(lVar11 + 0x110) * 8)
                 * 0x30;
        lVar32 = *(longlong *)(lVar26 + 0x10 + lVar22);
        uVar7 = *(undefined4 *)(lVar24 + 0x10);
        *(undefined4 *)(*(longlong *)(lVar26 + 0x28 + lVar22) + (ulonglong)uVar6 * 4) = uVar4;
        *(undefined4 *)(lVar32 + 0x50 + (ulonglong)uVar6 * 0x58) = uVar7;
        if (*(char *)(lVar24 + 0x14) == '\x01') {
          country_core::resources::roofs::roof_shape::Roof::pivot_3d();
          local_98 = CONCAT71(local_98._1_7_,0xb);
          *puVar14 = uVar5;
          lVar32 = *(longlong *)(lVar13 + 0x30);
          uVar15 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar32 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          lVar24 = *(longlong *)(lVar13 + 0x28);
          lVar26 = lVar32 * 0x20;
          *(undefined8 *)(lVar24 + lVar26) = uVar15;
          *(undefined4 *)(lVar24 + 0x18 + lVar26) = local_88;
          puVar1 = (undefined8 *)(lVar24 + 8 + lVar26);
          *puVar1 = local_98;
          puVar1[1] = uStack_90;
          *(longlong *)(lVar13 + 0x30) = lVar32 + 1;
          *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        }
      }
    }
  } while( true );
}

