// name=set_decorator_color_id
// sig=void __cdecl set_decorator_color_id(void)
// size=2879

void __cdecl system_color::set_decorator_color_id::set_decorator_color_id(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  undefined4 *puVar7;
  longlong lVar8;
  undefined4 *puVar9;
  undefined8 uVar10;
  byte bVar11;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar12;
  longlong extraout_RAX_01;
  undefined4 *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong in_RCX;
  longlong lVar13;
  longlong lVar14;
  undefined8 *in_RDX;
  ulonglong uVar15;
  longlong lVar16;
  undefined1 uVar17;
  longlong lVar18;
  longlong lVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  undefined8 *in_R9;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong uVar23;
  uint uVar24;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined4 local_e4;
  undefined4 local_88;
  undefined4 uStack_84;
  longlong lStack_80;
  undefined4 local_78;
  undefined4 uStack_74;
  longlong local_70;
  
  puVar5 = (ulonglong *)*in_RDX;
  lVar22 = in_RDX[1];
  uVar6 = *puVar5;
  uVar15 = 0;
  uVar21 = uVar6 - *(ulonglong *)(lVar22 + 0x18);
  if (uVar6 < *(ulonglong *)(lVar22 + 0x18)) {
    uVar21 = uVar15;
  }
  uVar20 = uVar6 - *(ulonglong *)(lVar22 + 0x38);
  if (uVar6 < *(ulonglong *)(lVar22 + 0x38)) {
    uVar20 = uVar15;
  }
  lVar16 = uVar21 * 0x18 + *(longlong *)(lVar22 + 8);
  uVar6 = *(ulonglong *)(lVar22 + 0x10);
  uVar23 = uVar6 - uVar21;
  if (uVar6 < uVar21) {
    uVar23 = uVar15;
  }
  if (uVar6 < uVar21) {
    lVar16 = 8;
  }
  local_70 = uVar20 * 0x18 + *(longlong *)(lVar22 + 0x28);
  uVar6 = *(ulonglong *)(lVar22 + 0x30) - uVar20;
  if (*(ulonglong *)(lVar22 + 0x30) < uVar20) {
    local_70 = 8;
    uVar6 = uVar15;
  }
  *puVar5 = (*(longlong *)(lVar22 + 0x40) - uVar6) - uVar23;
  lVar22 = local_70 + uVar6 * 0x18;
  uVar3 = *(undefined4 *)(in_RCX + 0x1c);
  puVar7 = *(undefined4 **)(in_RCX + 0x10);
  uVar4 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar8 = *in_stack_00000038;
  puVar9 = (undefined4 *)in_stack_00000038[2];
  lVar18 = lVar16;
  do {
    if ((lVar18 == 0) || (lVar18 == lVar16 + uVar23 * 0x18)) {
      if ((local_70 == 0) || (local_70 == lVar22)) {
        puVar5 = (ulonglong *)*in_R8;
        lVar22 = in_R8[1];
        uVar6 = *puVar5;
        uVar15 = 0;
        uVar21 = uVar6 - *(ulonglong *)(lVar22 + 0x18);
        if (uVar6 < *(ulonglong *)(lVar22 + 0x18)) {
          uVar21 = uVar15;
        }
        uVar20 = uVar6 - *(ulonglong *)(lVar22 + 0x38);
        if (uVar6 < *(ulonglong *)(lVar22 + 0x38)) {
          uVar20 = uVar15;
        }
        lVar16 = uVar21 * 0x18 + *(longlong *)(lVar22 + 8);
        uVar6 = *(ulonglong *)(lVar22 + 0x10) - uVar21;
        if (*(ulonglong *)(lVar22 + 0x10) < uVar21) {
          lVar16 = 8;
          uVar6 = uVar15;
        }
        lVar18 = uVar20 * 0x18 + *(longlong *)(lVar22 + 0x28);
        uVar21 = *(ulonglong *)(lVar22 + 0x30) - uVar20;
        if (*(ulonglong *)(lVar22 + 0x30) < uVar20) {
          lVar18 = 8;
          uVar21 = uVar15;
        }
        *puVar5 = (*(longlong *)(lVar22 + 0x40) - uVar21) - uVar6;
        lVar22 = lVar16 + uVar6 * 0x18;
        lVar12 = uVar21 * 0x18 + lVar18;
        do {
          if ((lVar16 == 0) || (lVar16 == lVar22)) {
            if ((lVar18 == 0) || (lVar18 == lVar12)) {
              puVar5 = (ulonglong *)*in_R9;
              lVar22 = in_R9[1];
              uVar6 = *puVar5;
              uVar15 = 0;
              uVar21 = uVar6 - *(ulonglong *)(lVar22 + 0x18);
              if (uVar6 < *(ulonglong *)(lVar22 + 0x18)) {
                uVar21 = uVar15;
              }
              uVar20 = uVar6 - *(ulonglong *)(lVar22 + 0x38);
              if (uVar6 < *(ulonglong *)(lVar22 + 0x38)) {
                uVar20 = uVar15;
              }
              lVar16 = uVar21 * 0x18 + *(longlong *)(lVar22 + 8);
              uVar6 = *(ulonglong *)(lVar22 + 0x10) - uVar21;
              if (*(ulonglong *)(lVar22 + 0x10) < uVar21) {
                lVar16 = 8;
                uVar6 = uVar15;
              }
              lVar18 = uVar20 * 0x18 + *(longlong *)(lVar22 + 0x28);
              uVar21 = *(ulonglong *)(lVar22 + 0x30) - uVar20;
              if (*(ulonglong *)(lVar22 + 0x30) < uVar20) {
                lVar18 = 8;
                uVar21 = uVar15;
              }
              *puVar5 = (*(longlong *)(lVar22 + 0x40) - uVar21) - uVar6;
              lVar22 = lVar16 + uVar6 * 0x18;
              lVar12 = uVar21 * 0x18 + lVar18;
              do {
                if ((lVar16 == 0) || (lVar16 == lVar22)) {
                  if ((lVar18 == 0) || (lVar18 == lVar12)) {
                    puVar5 = (ulonglong *)*in_stack_00000028;
                    lVar22 = in_stack_00000028[1];
                    uVar6 = *puVar5;
                    uVar15 = 0;
                    uVar21 = uVar6 - *(ulonglong *)(lVar22 + 0x18);
                    if (uVar6 < *(ulonglong *)(lVar22 + 0x18)) {
                      uVar21 = uVar15;
                    }
                    uVar20 = uVar6 - *(ulonglong *)(lVar22 + 0x38);
                    if (uVar6 < *(ulonglong *)(lVar22 + 0x38)) {
                      uVar20 = uVar15;
                    }
                    lVar16 = uVar21 * 0x10 + *(longlong *)(lVar22 + 8);
                    uVar6 = *(ulonglong *)(lVar22 + 0x10) - uVar21;
                    if (*(ulonglong *)(lVar22 + 0x10) < uVar21) {
                      lVar16 = 8;
                      uVar6 = uVar15;
                    }
                    lVar18 = uVar20 * 0x10 + *(longlong *)(lVar22 + 0x28);
                    uVar21 = *(ulonglong *)(lVar22 + 0x30) - uVar20;
                    if (*(ulonglong *)(lVar22 + 0x30) < uVar20) {
                      lVar18 = 8;
                      uVar21 = uVar15;
                    }
                    *puVar5 = (*(longlong *)(lVar22 + 0x40) - uVar21) - uVar6;
                    lVar22 = uVar6 * 0x10 + lVar16;
                    lVar12 = uVar21 * 0x10 + lVar18;
                    do {
                      if ((lVar16 == 0) || (lVar16 == lVar22)) {
                        if ((lVar18 == 0) || (lVar18 == lVar12)) {
                          puVar5 = (ulonglong *)*in_stack_00000030;
                          lVar22 = in_stack_00000030[1];
                          uVar6 = *puVar5;
                          uVar15 = 0;
                          uVar21 = uVar6 - *(ulonglong *)(lVar22 + 0x18);
                          if (uVar6 < *(ulonglong *)(lVar22 + 0x18)) {
                            uVar21 = uVar15;
                          }
                          uVar20 = uVar6 - *(ulonglong *)(lVar22 + 0x38);
                          if (uVar6 < *(ulonglong *)(lVar22 + 0x38)) {
                            uVar20 = uVar15;
                          }
                          lVar16 = uVar21 * 0x18 + *(longlong *)(lVar22 + 8);
                          uVar6 = *(ulonglong *)(lVar22 + 0x10) - uVar21;
                          if (*(ulonglong *)(lVar22 + 0x10) < uVar21) {
                            lVar16 = 8;
                            uVar6 = uVar15;
                          }
                          lVar18 = uVar20 * 0x18 + *(longlong *)(lVar22 + 0x28);
                          uVar21 = *(ulonglong *)(lVar22 + 0x30) - uVar20;
                          if (*(ulonglong *)(lVar22 + 0x30) < uVar20) {
                            lVar18 = 8;
                            uVar21 = uVar15;
                          }
                          *puVar5 = (*(longlong *)(lVar22 + 0x40) - uVar21) - uVar6;
                          lVar22 = lVar16 + uVar6 * 0x18;
                          lVar12 = uVar21 * 0x18 + lVar18;
                          do {
                            if ((lVar16 == 0) || (lVar16 == lVar22)) {
                              if ((lVar18 == 0) || (lVar18 == lVar12)) {
                                return;
                              }
                              lVar16 = 0;
                              lVar19 = lVar18;
                              lVar18 = lVar18 + 0x18;
                            }
                            else {
                              lVar19 = lVar16;
                              lVar16 = lVar16 + 0x18;
                            }
                            *puVar5 = *puVar5 + 1;
                            *puVar7 = uVar3;
                            country_core::resources::walls::decorator_storage::DecoratorStorage::
                            lookup_decorator_info_mut();
                            if ((extraout_RAX_09 != 0) && (*(char *)(extraout_RAX_09 + 0x46) == '?')
                               ) {
                              country_core::resources::stairs::stairs_state::StairsState::
                              get_all_connected_nodes();
                              for (lVar13 = lStack_80;
                                  lVar13 != lStack_80 + CONCAT44(uStack_74,local_78) * 4;
                                  lVar13 = lVar13 + 4) {
                                *puVar7 = uVar3;
                                country_core::resources::walls::decorator_storage::DecoratorStorage
                                ::lookup_decorator_info_mut();
                                if (extraout_RAX_10 == 0) {
                    // WARNING: Subroutine does not return
                                  core::option::unwrap_failed();
                                }
                                uVar2 = *(undefined4 *)(lVar19 + 0xc);
                                country_core::resources::walls::decorator::DecoratorExtraInfo::
                                as_stair();
                                *(undefined4 *)(extraout_RAX_11 + 0xc) = uVar2;
                              }
                              if (CONCAT44(uStack_84,local_88) != 0) {
                                std::alloc::__default_lib_allocator::__rust_dealloc();
                              }
                              if (*(char *)(lVar19 + 0x10) == '\x01') {
                                country_core::resources::walls::cached_decorator_transforms::
                                CachedDecoratorTransforms::position();
                                local_88 = CONCAT31(local_88._1_3_,10);
                                *puVar9 = uVar4;
                                lVar19 = *(longlong *)(lVar8 + 0x30);
                                uVar10 = *(undefined8 *)(lVar8 + 0x40);
                                if (lVar19 == *(longlong *)(lVar8 + 0x20)) {
                                  alloc::raw_vec::RawVec::
                                  grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                                            ();
                                }
                                lVar13 = *(longlong *)(lVar8 + 0x28);
                                lVar14 = lVar19 * 0x20;
                                *(undefined8 *)(lVar13 + lVar14) = uVar10;
                                *(undefined4 *)(lVar13 + 0x18 + lVar14) = local_78;
                                puVar1 = (undefined8 *)(lVar13 + 8 + lVar14);
                                *puVar1 = CONCAT44(uStack_84,local_88);
                                puVar1[1] = lStack_80;
                                *(longlong *)(lVar8 + 0x30) = lVar19 + 1;
                                *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
                              }
                            }
                          } while( true );
                        }
                        lVar16 = 0;
                        lVar19 = lVar18;
                        lVar18 = lVar18 + 0x10;
                      }
                      else {
                        lVar19 = lVar16;
                        lVar16 = lVar16 + 0x10;
                      }
                      *puVar5 = *puVar5 + 1;
                      *puVar7 = uVar3;
                      country_core::resources::walls::decorator_storage::DecoratorStorage::
                      lookup_decorator_info_mut();
                      if ((extraout_RAX_06 != 0) && (*(char *)(extraout_RAX_06 + 0x46) == '?')) {
                        country_core::resources::stairs::stairs_state::StairsState::
                        get_all_connected_nodes();
                        for (lVar13 = lStack_80;
                            lVar13 != lStack_80 + CONCAT44(uStack_74,local_78) * 4;
                            lVar13 = lVar13 + 4) {
                          *puVar7 = uVar3;
                          country_core::resources::walls::decorator_storage::DecoratorStorage::
                          lookup_decorator_info_mut();
                          if (extraout_RAX_07 == 0) {
                    // WARNING: Subroutine does not return
                            core::option::unwrap_failed();
                          }
                          uVar17 = *(undefined1 *)(lVar19 + 0xc);
                          country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
                          *(undefined1 *)(extraout_RAX_08 + 0x10) = uVar17;
                        }
                        if (CONCAT44(uStack_84,local_88) != 0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                        if (*(char *)(lVar19 + 0xd) == '\x01') {
                          country_core::resources::walls::cached_decorator_transforms::
                          CachedDecoratorTransforms::position();
                          local_88 = CONCAT31(local_88._1_3_,10);
                          *puVar9 = uVar4;
                          lVar19 = *(longlong *)(lVar8 + 0x30);
                          uVar10 = *(undefined8 *)(lVar8 + 0x40);
                          if (lVar19 == *(longlong *)(lVar8 + 0x20)) {
                            alloc::raw_vec::RawVec::
                            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                                      ();
                          }
                          lVar13 = *(longlong *)(lVar8 + 0x28);
                          lVar14 = lVar19 * 0x20;
                          *(undefined8 *)(lVar13 + lVar14) = uVar10;
                          *(undefined4 *)(lVar13 + 0x18 + lVar14) = local_78;
                          puVar1 = (undefined8 *)(lVar13 + 8 + lVar14);
                          *puVar1 = CONCAT44(uStack_84,local_88);
                          puVar1[1] = lStack_80;
                          *(longlong *)(lVar8 + 0x30) = lVar19 + 1;
                          *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
                        }
                      }
                    } while( true );
                  }
                  lVar16 = 0;
                  lVar19 = lVar18;
                  lVar18 = lVar18 + 0x18;
                }
                else {
                  lVar19 = lVar16;
                  lVar16 = lVar16 + 0x18;
                }
                *puVar5 = *puVar5 + 1;
                *puVar7 = uVar3;
                country_core::resources::walls::decorator_storage::DecoratorStorage::
                lookup_decorator_info_mut();
                if ((extraout_RAX_03 != 0) && (*(char *)(extraout_RAX_03 + 0x46) == '?')) {
                  country_core::resources::stairs::stairs_state::StairsState::
                  get_all_connected_nodes();
                  for (lVar13 = lStack_80; lVar13 != lStack_80 + CONCAT44(uStack_74,local_78) * 4;
                      lVar13 = lVar13 + 4) {
                    *puVar7 = uVar3;
                    country_core::resources::walls::decorator_storage::DecoratorStorage::
                    lookup_decorator_info_mut();
                    if (extraout_RAX_04 == 0) {
                    // WARNING: Subroutine does not return
                      core::option::unwrap_failed();
                    }
                    uVar2 = *(undefined4 *)(lVar19 + 0xc);
                    country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
                    *(undefined4 *)(extraout_RAX_05 + 8) = uVar2;
                  }
                  if (CONCAT44(uStack_84,local_88) != 0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  if (*(char *)(lVar19 + 0x10) == '\x01') {
                    country_core::resources::walls::cached_decorator_transforms::
                    CachedDecoratorTransforms::position();
                    local_88 = CONCAT31(local_88._1_3_,10);
                    *puVar9 = uVar4;
                    lVar19 = *(longlong *)(lVar8 + 0x30);
                    uVar10 = *(undefined8 *)(lVar8 + 0x40);
                    if (lVar19 == *(longlong *)(lVar8 + 0x20)) {
                      alloc::raw_vec::RawVec::
                      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                                ();
                    }
                    lVar13 = *(longlong *)(lVar8 + 0x28);
                    lVar14 = lVar19 * 0x20;
                    *(undefined8 *)(lVar13 + lVar14) = uVar10;
                    *(undefined4 *)(lVar13 + 0x18 + lVar14) = local_78;
                    puVar1 = (undefined8 *)(lVar13 + 8 + lVar14);
                    *puVar1 = CONCAT44(uStack_84,local_88);
                    puVar1[1] = lStack_80;
                    *(longlong *)(lVar8 + 0x30) = lVar19 + 1;
                    *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
                  }
                }
              } while( true );
            }
            lVar19 = lVar18 + 0x18;
            lVar13 = 0;
            lVar14 = lVar18;
          }
          else {
            lVar19 = lVar18;
            lVar13 = lVar16 + 0x18;
            lVar14 = lVar16;
          }
          lVar16 = lVar13;
          lVar18 = lVar19;
          *puVar5 = *puVar5 + 1;
          *puVar7 = uVar3;
          country_core::resources::walls::decorator_storage::DecoratorStorage::
          lookup_decorator_info_mut();
          if ((extraout_RAX_01 != 0) && (*(char *)(extraout_RAX_01 + 0x46) == '<')) {
            uVar2 = *(undefined4 *)(lVar14 + 0xc);
            country_core::resources::walls::decorator::DecoratorExtraInfo::as_lantern();
            *extraout_RAX_02 = uVar2;
            if (*(char *)(lVar14 + 0x10) == '\x01') {
              uStack_84 = *(undefined4 *)(lVar14 + 0xc);
              country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms
              ::position();
              local_88 = CONCAT31(local_88._1_3_,0xe);
              *puVar9 = uVar4;
              lVar19 = *(longlong *)(lVar8 + 0x30);
              uVar10 = *(undefined8 *)(lVar8 + 0x40);
              if (lVar19 == *(longlong *)(lVar8 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                          ();
              }
              lVar13 = *(longlong *)(lVar8 + 0x28);
              lVar14 = lVar19 * 0x20;
              *(undefined8 *)(lVar13 + lVar14) = uVar10;
              *(undefined4 *)(lVar13 + 0x18 + lVar14) = local_78;
              puVar1 = (undefined8 *)(lVar13 + 8 + lVar14);
              *puVar1 = CONCAT44(uStack_84,local_88);
              puVar1[1] = lStack_80;
              *(longlong *)(lVar8 + 0x30) = lVar19 + 1;
              *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
            }
          }
        } while( true );
      }
      lVar12 = 0;
      lVar19 = local_70;
      local_70 = local_70 + 0x18;
    }
    else {
      lVar12 = lVar18 + 0x18;
      lVar19 = lVar18;
    }
    *puVar5 = *puVar5 + 1;
    *puVar7 = uVar3;
    country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info_mut()
    ;
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    *(undefined4 *)(extraout_RAX + 0x28) = *(undefined4 *)(lVar19 + 0xc);
    bVar11 = *(char *)(extraout_RAX + 0x46) - 0x38;
    uVar24 = (uint)bVar11;
    if (7 < bVar11) {
      uVar24 = 6;
    }
    if ((char)uVar24 == '\a') {
      country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes();
      for (lVar18 = lStack_80; lVar18 != lStack_80 + CONCAT44(uStack_74,local_78) * 4;
          lVar18 = lVar18 + 4) {
        *puVar7 = uVar3;
        country_core::resources::walls::decorator_storage::DecoratorStorage::
        lookup_decorator_info_mut();
        if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        *(undefined4 *)(extraout_RAX_00 + 0x28) = *(undefined4 *)(lVar19 + 0xc);
      }
      if (CONCAT44(uStack_84,local_88) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    lVar18 = lVar12;
    if (*(char *)(lVar19 + 0x10) != '\0') {
      uVar17 = 10;
      if ((0xcfU >> (uVar24 & 0x1f) & 1) == 0) {
        if (uVar24 == 4) {
          local_e4 = *(undefined4 *)(lVar19 + 0xc);
          uVar17 = 0xd;
        }
        else {
          uVar17 = 8;
        }
      }
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
      position();
      local_88 = CONCAT31(local_88._1_3_,uVar17);
      *puVar9 = uVar4;
      lVar12 = *(longlong *)(lVar8 + 0x30);
      uVar10 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar12 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      uStack_84 = local_e4;
      lVar19 = *(longlong *)(lVar8 + 0x28);
      lVar13 = lVar12 * 0x20;
      *(undefined8 *)(lVar19 + lVar13) = uVar10;
      *(undefined4 *)(lVar19 + 0x18 + lVar13) = local_78;
      puVar1 = (undefined8 *)(lVar19 + 8 + lVar13);
      *puVar1 = CONCAT44(local_e4,local_88);
      puVar1[1] = lStack_80;
      *(longlong *)(lVar8 + 0x30) = lVar12 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
  } while( true );
}

