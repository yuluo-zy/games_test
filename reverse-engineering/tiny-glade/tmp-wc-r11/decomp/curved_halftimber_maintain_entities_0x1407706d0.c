// name: curved_halftimber_maintain_entities
// addr: 0x1407706d0
// size: 3309
// signature: void __cdecl curved_halftimber_maintain_entities(void)


void __cdecl system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  undefined4 *puVar5;
  undefined8 uVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  char extraout_AL;
  char extraout_AL_00;
  longlong lVar16;
  longlong lVar17;
  ulonglong extraout_RAX;
  ulonglong uVar18;
  longlong *in_RCX;
  ulonglong uVar19;
  undefined8 *puVar20;
  undefined8 *in_RDX;
  longlong lVar21;
  longlong *plVar22;
  longlong *plVar23;
  ushort uVar24;
  longlong lVar25;
  undefined8 *in_R8;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong in_R9;
  uint uVar28;
  longlong lVar29;
  undefined8 *puVar30;
  byte bVar31;
  char cVar32;
  char cVar34;
  char cVar35;
  char cVar36;
  undefined1 auVar33 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
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
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 uStack_1b8;
  undefined8 local_178;
  undefined7 uStack_170;
  undefined1 uStack_169;
  undefined7 uStack_168;
  undefined1 uStack_161;
  undefined7 uStack_160;
  undefined1 uStack_159;
  undefined7 uStack_158;
  undefined1 uStack_151;
  undefined7 uStack_150;
  undefined1 local_149;
  undefined7 uStack_148;
  undefined1 uStack_141;
  undefined7 uStack_140;
  undefined1 local_139;
  longlong local_100;
  longlong local_f8;
  longlong *plStack_f0;
  longlong *local_e8;
  longlong local_e0;
  undefined8 local_d8;
  longlong local_c8;
  longlong local_c0;
  longlong local_a0;
  
  puVar4 = (ulonglong *)*in_RDX;
  lVar17 = in_RDX[1];
  uVar26 = *puVar4;
  uVar19 = 0;
  uVar18 = uVar26 - *(ulonglong *)(lVar17 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar17 + 0x18)) {
    uVar18 = uVar19;
  }
  uVar27 = uVar26 - *(ulonglong *)(lVar17 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar17 + 0x38)) {
    uVar27 = uVar19;
  }
  lVar16 = uVar18 * 0x10 + *(longlong *)(lVar17 + 8);
  uVar26 = *(ulonglong *)(lVar17 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar17 + 0x10) < uVar18) {
    lVar16 = 8;
    uVar26 = uVar19;
  }
  lVar29 = uVar27 * 0x10 + *(longlong *)(lVar17 + 0x28);
  uVar18 = *(ulonglong *)(lVar17 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar17 + 0x30) < uVar27) {
    lVar29 = 8;
    uVar18 = uVar19;
  }
  *puVar4 = (*(longlong *)(lVar17 + 0x40) - uVar18) - uVar26;
  lVar21 = uVar18 * 0x10 + lVar29;
  uVar3 = *(undefined4 *)(in_R9 + 0x1c);
  puVar5 = *(undefined4 **)(in_R9 + 0x10);
  lVar17 = lVar16;
  do {
    if ((lVar17 == 0) || (lVar17 == uVar26 * 0x10 + lVar16)) {
      if ((lVar29 == 0) || (lVar29 == lVar21)) {
        puVar4 = (ulonglong *)*in_R8;
        lVar17 = in_R8[1];
        uVar26 = *puVar4;
        uVar19 = 0;
        uVar18 = uVar26 - *(ulonglong *)(lVar17 + 0x18);
        if (uVar26 < *(ulonglong *)(lVar17 + 0x18)) {
          uVar18 = uVar19;
        }
        uVar27 = uVar26 - *(ulonglong *)(lVar17 + 0x38);
        if (uVar26 < *(ulonglong *)(lVar17 + 0x38)) {
          uVar27 = uVar19;
        }
        lVar16 = uVar18 * 0x18 + *(longlong *)(lVar17 + 8);
        uVar26 = *(ulonglong *)(lVar17 + 0x10) - uVar18;
        if (*(ulonglong *)(lVar17 + 0x10) < uVar18) {
          lVar16 = 8;
          uVar26 = uVar19;
        }
        lVar29 = uVar27 * 0x18 + *(longlong *)(lVar17 + 0x28);
        uVar18 = *(ulonglong *)(lVar17 + 0x30) - uVar27;
        if (*(ulonglong *)(lVar17 + 0x30) < uVar27) {
          lVar29 = 8;
          uVar18 = uVar19;
        }
        *puVar4 = (*(longlong *)(lVar17 + 0x40) - uVar18) - uVar26;
        lVar17 = lVar16 + uVar26 * 0x18;
        lVar21 = lVar29 + uVar18 * 0x18;
        do {
          if ((lVar16 == 0) || (lVar16 == lVar17)) {
            if ((lVar29 == 0) || (lVar29 == lVar21)) {
              puVar4 = (ulonglong *)*in_stack_00000030;
              lVar17 = in_stack_00000030[1];
              uVar26 = *puVar4;
              uVar19 = 0;
              uVar18 = uVar26 - *(ulonglong *)(lVar17 + 0x18);
              if (uVar26 < *(ulonglong *)(lVar17 + 0x18)) {
                uVar18 = uVar19;
              }
              uVar27 = uVar26 - *(ulonglong *)(lVar17 + 0x38);
              if (uVar26 < *(ulonglong *)(lVar17 + 0x38)) {
                uVar27 = uVar19;
              }
              local_c0 = uVar18 * 0x28 + *(longlong *)(lVar17 + 8);
              uVar26 = *(ulonglong *)(lVar17 + 0x10) - uVar18;
              if (*(ulonglong *)(lVar17 + 0x10) < uVar18) {
                local_c0 = 8;
                uVar26 = uVar19;
              }
              local_c8 = uVar27 * 0x28 + *(longlong *)(lVar17 + 0x28);
              uVar18 = *(ulonglong *)(lVar17 + 0x30) - uVar27;
              if (*(ulonglong *)(lVar17 + 0x30) < uVar27) {
                local_c8 = 8;
                uVar18 = uVar19;
              }
              *puVar4 = (*(longlong *)(lVar17 + 0x40) - uVar18) - uVar26;
              lVar17 = local_c0 + uVar26 * 0x28;
              lVar16 = local_c8 + uVar18 * 0x28;
              plVar23 = (longlong *)*in_RCX;
              iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                                        CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba)
                                                )) ==
                              CONCAT13(unaff_XMM7_Bd,
                                       CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))
                                      ));
              iVar11 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                                        CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be)
                                                )) ==
                              CONCAT13(unaff_XMM7_Bh,
                                       CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))
                                      ));
              iVar12 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                                        CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi)
                                                )) ==
                              CONCAT13(unaff_XMM7_Bl,
                                       CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))
                                      ));
              iVar13 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                                        CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm)
                                                )) ==
                              CONCAT13(unaff_XMM7_Bp,
                                       CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))
                                      ));
              do {
                if ((local_c0 == 0) || (local_c0 == lVar17)) {
                  if ((local_c8 == 0) || (local_c8 == lVar16)) {
                    puVar4 = (ulonglong *)*in_stack_00000038;
                    lVar17 = in_stack_00000038[1];
                    uVar26 = *puVar4;
                    uVar19 = 0;
                    uVar18 = uVar26 - *(ulonglong *)(lVar17 + 0x18);
                    if (uVar26 < *(ulonglong *)(lVar17 + 0x18)) {
                      uVar18 = uVar19;
                    }
                    uVar27 = uVar26 - *(ulonglong *)(lVar17 + 0x38);
                    if (uVar26 < *(ulonglong *)(lVar17 + 0x38)) {
                      uVar27 = uVar19;
                    }
                    local_100 = uVar18 * 0x28 + *(longlong *)(lVar17 + 8);
                    uVar26 = *(ulonglong *)(lVar17 + 0x10) - uVar18;
                    if (*(ulonglong *)(lVar17 + 0x10) < uVar18) {
                      local_100 = 8;
                      uVar26 = uVar19;
                    }
                    local_c0 = uVar27 * 0x28 + *(longlong *)(lVar17 + 0x28);
                    uVar18 = *(ulonglong *)(lVar17 + 0x30) - uVar27;
                    if (*(ulonglong *)(lVar17 + 0x30) < uVar27) {
                      local_c0 = 8;
                      uVar18 = uVar19;
                    }
                    *puVar4 = (*(longlong *)(lVar17 + 0x40) - uVar18) - uVar26;
                    lVar17 = local_100 + uVar26 * 0x28;
                    lVar16 = local_c0 + uVar18 * 0x28;
                    do {
                      if ((local_100 == 0) || (local_100 == lVar17)) {
                        if ((local_c0 == 0) || (local_c0 == lVar16)) {
                          return;
                        }
                        local_100 = 0;
                        lVar29 = local_c0;
                        local_c0 = local_c0 + 0x28;
                      }
                      else {
                        lVar29 = local_100;
                        local_100 = local_100 + 0x28;
                      }
                      *puVar4 = *puVar4 + 1;
                      *puVar5 = uVar3;
                      core::hash::BuildHasher::
                      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                                ();
                      hashbrown::raw::RawTable::
                      remove_entry<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
                                ();
                      if (!SBORROW8(0,(longlong)plStack_f0)) {
                        if (local_e0 != 0) {
                          lVar21 = 0;
                          do {
                            if (*(ulonglong *)((longlong)local_e8 + lVar21) >> 0x20 == 0) break;
                            uVar6 = *in_stack_00000028;
                            bevy_ecs::entity::Entities::contains();
                            if (extraout_AL_00 != '\0') {
                              uStack_170 = (undefined7)in_stack_00000028[1];
                              bevy_ecs::system::commands::EntityCommands::despawn();
                              local_178 = uVar6;
                            }
                            lVar21 = lVar21 + 8;
                          } while (local_e0 << 3 != lVar21);
                        }
                        if (plStack_f0 != (longlong *)0x0) {
                          std::alloc::__default_lib_allocator::__rust_dealloc();
                        }
                      }
                      local_a0 = 0;
                      puVar20 = *(undefined8 **)(lVar29 + 8);
                      puVar30 = puVar20 + *(longlong *)(lVar29 + 0x10) * 8;
                      for (; puVar20 != puVar30; puVar20 = puVar20 + 8) {
                        country_core::resources::walls::decorator::DecoratorRank::new();
                        country_core::components::drawable::impl$1::from();
                        bevy_app::plugin::Plugin::
                        finish<country_core::systems::main_camera::MainCameraPlugin>();
                        uVar6 = *(undefined8 *)(lVar29 + 0x18);
                        uVar14 = puVar20[6];
                        uVar15 = puVar20[7];
                        local_139 = (undefined1)uVar14;
                        local_149 = (undefined1)puVar20[4];
                        uStack_148 = (undefined7)((ulonglong)puVar20[4] >> 8);
                        uStack_141 = (undefined1)puVar20[5];
                        uStack_140 = (undefined7)((ulonglong)puVar20[5] >> 8);
                        uStack_159 = (undefined1)puVar20[2];
                        uStack_158 = (undefined7)((ulonglong)puVar20[2] >> 8);
                        uStack_151 = (undefined1)puVar20[3];
                        uStack_150 = (undefined7)((ulonglong)puVar20[3] >> 8);
                        uStack_169 = (undefined1)*puVar20;
                        uStack_168 = (undefined7)((ulonglong)*puVar20 >> 8);
                        uStack_161 = (undefined1)puVar20[1];
                        uStack_160 = (undefined7)((ulonglong)puVar20[1] >> 8);
                        bevy_ecs::system::commands::Commands::spawn_empty();
                        plVar23 = plStack_f0;
                        if (plStack_f0 == (longlong *)0x0) {
                          plVar23 = local_e8;
                        }
                        lVar21 = plVar23[2];
                        if ((ulonglong)(*plVar23 - lVar21) < 0xd8) {
                          alloc::raw_vec::impl$4::reserve::
                          do_reserve_and_handle<alloc::alloc::Global>();
                        }
                        lVar25 = plVar23[1];
                        *(code **)(lVar25 + lVar21) =
                             core::ops::function::FnOnce::
                             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::resources::walls::curved_halftimber::CurvedHalfTimber>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
                        ;
                        puVar2 = (undefined8 *)(lVar25 + 8 + lVar21);
                        *puVar2 = 0;
                        puVar2[1] = 0x3f80000000000000;
                        *(undefined8 *)(lVar25 + 0x18 + lVar21) = 0;
                        *(undefined4 *)(lVar25 + 0x20 + lVar21) = 0;
                        *(undefined8 *)(lVar25 + 0x24 + lVar21) = 0x3f8000003f800000;
                        *(undefined4 *)(lVar25 + 0x2c + lVar21) = 0x3f800000;
                        puVar2 = (undefined8 *)(lVar25 + 0x30 + lVar21);
                        *puVar2 = uStack_1b8;
                        puVar2[1] = local_f8;
                        *(longlong **)(lVar25 + 0x40 + lVar21) = plStack_f0;
                        *(undefined1 *)(lVar25 + 0x48 + lVar21) = 0x18;
                        puVar2 = (undefined8 *)(lVar25 + 0x49 + lVar21);
                        *puVar2 = local_178;
                        puVar2[1] = CONCAT17(uStack_169,uStack_170);
                        puVar2 = (undefined8 *)(lVar25 + 0x59 + lVar21);
                        *puVar2 = CONCAT17(uStack_161,uStack_168);
                        puVar2[1] = CONCAT17(uStack_159,uStack_160);
                        puVar2 = (undefined8 *)(lVar25 + 0x69 + lVar21);
                        *puVar2 = CONCAT17(uStack_151,uStack_158);
                        puVar2[1] = CONCAT17(local_149,uStack_150);
                        puVar2 = (undefined8 *)(lVar25 + 0x79 + lVar21);
                        *puVar2 = CONCAT17(uStack_141,uStack_148);
                        puVar2[1] = CONCAT17(local_139,uStack_140);
                        puVar2 = (undefined8 *)(lVar25 + 0x88 + lVar21);
                        *puVar2 = uVar14;
                        puVar2[1] = uVar15;
                        *(undefined8 *)(lVar25 + 0x98 + lVar21) = uVar6;
                        *(undefined1 *)(lVar25 + 0xa0 + lVar21) = 2;
                        *(undefined1 *)(lVar25 + 0xa8 + lVar21) = 0;
                        *(undefined8 *)(lVar25 + 0xb8 + lVar21) = local_d8;
                        *(code **)(lVar25 + 200 + lVar21) = bevy_ecs::error::handler::panic;
                        plVar23[2] = lVar21 + 0xd8;
                        if (local_a0 == 0) {
                          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                        }
                        *(undefined8 *)(local_a0 * 8 + 8) = local_d8;
                        local_a0 = local_a0 + 1;
                      }
                      *puVar5 = uVar3;
                      local_178 = 0;
                      uStack_170 = 8;
                      hashbrown::map::HashMap::
                      insert<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
                                ();
                      if ((!SBORROW8(0,local_f8)) && (local_f8 != 0)) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                    } while( true );
                  }
                  local_c0 = 0;
                  lVar29 = local_c8;
                  local_c8 = local_c8 + 0x28;
                }
                else {
                  lVar29 = local_c0;
                  local_c0 = local_c0 + 0x28;
                }
                *puVar4 = *puVar4 + 1;
                *puVar5 = uVar3;
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                hashbrown::raw::RawTable::
                remove_entry<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
                          ();
                if (!SBORROW8(0,(longlong)plStack_f0)) {
                  if (local_e0 != 0) {
                    lVar21 = 0;
                    do {
                      if (*(ulonglong *)((longlong)local_e8 + lVar21) >> 0x20 == 0) break;
                      uVar6 = *in_stack_00000028;
                      bevy_ecs::entity::Entities::contains();
                      if (extraout_AL != '\0') {
                        uStack_170 = (undefined7)in_stack_00000028[1];
                        bevy_ecs::system::commands::EntityCommands::despawn();
                        local_178 = uVar6;
                      }
                      lVar21 = lVar21 + 8;
                    } while (local_e0 << 3 != lVar21);
                  }
                  if (plStack_f0 != (longlong *)0x0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                }
                if (plVar23[3] == 0) {
LAB_140770f22:
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                          ();
                bVar31 = (byte)(extraout_RAX >> 0x39);
                auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
                auVar33 = pshuflw(auVar33,auVar33,0);
                lVar21 = 0;
                uVar26 = extraout_RAX;
                while( true ) {
                  uVar26 = plVar23[1] & uVar26;
                  pcVar1 = (char *)(*plVar23 + uVar26);
                  cVar32 = auVar33[0];
                  auVar38[0] = -(*pcVar1 == cVar32);
                  cVar34 = auVar33[1];
                  auVar38[1] = -(pcVar1[1] == cVar34);
                  cVar35 = auVar33[2];
                  auVar38[2] = -(pcVar1[2] == cVar35);
                  cVar36 = auVar33[3];
                  auVar38[3] = -(pcVar1[3] == cVar36);
                  auVar38[4] = -(pcVar1[4] == cVar32);
                  auVar38[5] = -(pcVar1[5] == cVar34);
                  auVar38[6] = -(pcVar1[6] == cVar35);
                  auVar38[7] = -(pcVar1[7] == cVar36);
                  auVar38[8] = -(pcVar1[8] == cVar32);
                  auVar38[9] = -(pcVar1[9] == cVar34);
                  auVar38[10] = -(pcVar1[10] == cVar35);
                  auVar38[0xb] = -(pcVar1[0xb] == cVar36);
                  auVar38[0xc] = -(pcVar1[0xc] == cVar32);
                  auVar38[0xd] = -(pcVar1[0xd] == cVar34);
                  auVar38[0xe] = -(pcVar1[0xe] == cVar35);
                  auVar38[0xf] = -(pcVar1[0xf] == cVar36);
                  uVar24 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar38[0xf] >> 7) << 0xf;
                  uVar28 = (uint)uVar24;
                  if (uVar24 != 0) break;
LAB_140770bd0:
                  auVar37[0] = -(*pcVar1 == (char)iVar10);
                  auVar37[1] = -(pcVar1[1] == (char)((uint)iVar10 >> 8));
                  auVar37[2] = -(pcVar1[2] == (char)((uint)iVar10 >> 0x10));
                  auVar37[3] = -(pcVar1[3] == (char)((uint)iVar10 >> 0x18));
                  auVar37[4] = -(pcVar1[4] == (char)iVar11);
                  auVar37[5] = -(pcVar1[5] == (char)((uint)iVar11 >> 8));
                  auVar37[6] = -(pcVar1[6] == (char)((uint)iVar11 >> 0x10));
                  auVar37[7] = -(pcVar1[7] == (char)((uint)iVar11 >> 0x18));
                  auVar37[8] = -(pcVar1[8] == (char)iVar12);
                  auVar37[9] = -(pcVar1[9] == (char)((uint)iVar12 >> 8));
                  auVar37[10] = -(pcVar1[10] == (char)((uint)iVar12 >> 0x10));
                  auVar37[0xb] = -(pcVar1[0xb] == (char)((uint)iVar12 >> 0x18));
                  auVar37[0xc] = -(pcVar1[0xc] == (char)iVar13);
                  auVar37[0xd] = -(pcVar1[0xd] == (char)((uint)iVar13 >> 8));
                  auVar37[0xe] = -(pcVar1[0xe] == (char)((uint)iVar13 >> 0x10));
                  auVar37[0xf] = -(pcVar1[0xf] == (char)((uint)iVar13 >> 0x18));
                  if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar37 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar37 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar37 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar37 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar37 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar37 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar37 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar37 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar37 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar37 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar37 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x6f,0) & 1) != 0) || (SUB161(auVar37 >> 0x77,0) & 1) != 0
                      ) || auVar37[0xf] < '\0') goto LAB_140770f22;
                  uVar26 = uVar26 + lVar21 + 0x10;
                  lVar21 = lVar21 + 0x10;
                }
                while( true ) {
                  uVar8 = 0;
                  for (uVar9 = uVar28; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                    uVar8 = uVar8 + 1;
                  }
                  if (*(longlong *)(lVar29 + 0x18) ==
                      *(longlong *)(*plVar23 + -0x2a8 + (uVar8 + uVar26 & plVar23[1]) * -0x2a8))
                  break;
                  uVar24 = (ushort)(uVar28 - 1) & (ushort)uVar28;
                  uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar24);
                  if (uVar24 == 0) goto LAB_140770bd0;
                }
                local_a0 = 0;
                wooden_floor_borders::wooden_floor_borders();
                puVar20 = *(undefined8 **)(lVar29 + 8);
                puVar30 = puVar20 + *(longlong *)(lVar29 + 0x10) * 8;
                for (; puVar20 != puVar30; puVar20 = puVar20 + 8) {
                  country_core::resources::walls::decorator::DecoratorRank::new();
                  country_core::components::drawable::impl$1::from();
                  bevy_app::plugin::Plugin::
                  finish<country_core::systems::main_camera::MainCameraPlugin>();
                  lVar21 = *(longlong *)(lVar29 + 0x18);
                  uVar6 = puVar20[6];
                  uVar14 = puVar20[7];
                  local_139 = (undefined1)uVar6;
                  local_149 = (undefined1)puVar20[4];
                  uStack_148 = (undefined7)((ulonglong)puVar20[4] >> 8);
                  uStack_141 = (undefined1)puVar20[5];
                  uStack_140 = (undefined7)((ulonglong)puVar20[5] >> 8);
                  uStack_159 = (undefined1)puVar20[2];
                  uStack_158 = (undefined7)((ulonglong)puVar20[2] >> 8);
                  uStack_151 = (undefined1)puVar20[3];
                  uStack_150 = (undefined7)((ulonglong)puVar20[3] >> 8);
                  uStack_169 = (undefined1)*puVar20;
                  uStack_168 = (undefined7)((ulonglong)*puVar20 >> 8);
                  uStack_161 = (undefined1)puVar20[1];
                  uStack_160 = (undefined7)((ulonglong)puVar20[1] >> 8);
                  bevy_ecs::system::commands::Commands::spawn_empty();
                  plVar22 = plStack_f0;
                  if (plStack_f0 == (longlong *)0x0) {
                    plVar22 = local_e8;
                  }
                  lVar25 = plVar22[2];
                  if ((ulonglong)(*plVar22 - lVar25) < 0xd8) {
                    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  }
                  lVar7 = plVar22[1];
                  *(code **)(lVar7 + lVar25) =
                       core::ops::function::FnOnce::
                       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::resources::walls::curved_halftimber::CurvedHalfTimber>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
                  ;
                  puVar2 = (undefined8 *)(lVar7 + 8 + lVar25);
                  *puVar2 = 0;
                  puVar2[1] = 0x3f80000000000000;
                  *(undefined8 *)(lVar7 + 0x18 + lVar25) = 0;
                  *(undefined4 *)(lVar7 + 0x20 + lVar25) = 0;
                  *(undefined8 *)(lVar7 + 0x24 + lVar25) = 0x3f8000003f800000;
                  *(undefined4 *)(lVar7 + 0x2c + lVar25) = 0x3f800000;
                  puVar2 = (undefined8 *)(lVar7 + 0x30 + lVar25);
                  *puVar2 = uStack_1b8;
                  puVar2[1] = local_f8;
                  *(longlong **)(lVar7 + 0x40 + lVar25) = plStack_f0;
                  *(undefined1 *)(lVar7 + 0x48 + lVar25) = 0x18;
                  puVar2 = (undefined8 *)(lVar7 + 0x49 + lVar25);
                  *puVar2 = local_178;
                  puVar2[1] = CONCAT17(uStack_169,uStack_170);
                  puVar2 = (undefined8 *)(lVar7 + 0x59 + lVar25);
                  *puVar2 = CONCAT17(uStack_161,uStack_168);
                  puVar2[1] = CONCAT17(uStack_159,uStack_160);
                  puVar2 = (undefined8 *)(lVar7 + 0x69 + lVar25);
                  *puVar2 = CONCAT17(uStack_151,uStack_158);
                  puVar2[1] = CONCAT17(local_149,uStack_150);
                  puVar2 = (undefined8 *)(lVar7 + 0x79 + lVar25);
                  *puVar2 = CONCAT17(uStack_141,uStack_148);
                  puVar2[1] = CONCAT17(local_139,uStack_140);
                  puVar2 = (undefined8 *)(lVar7 + 0x88 + lVar25);
                  *puVar2 = uVar6;
                  puVar2[1] = uVar14;
                  *(longlong *)(lVar7 + 0x98 + lVar25) = lVar21;
                  *(undefined1 *)(lVar7 + 0xa0 + lVar25) = 1;
                  *(undefined1 *)(lVar7 + 0xa8 + lVar25) = 0;
                  *(undefined8 *)(lVar7 + 0xb8 + lVar25) = local_d8;
                  *(code **)(lVar7 + 200 + lVar25) = bevy_ecs::error::handler::panic;
                  plVar22[2] = lVar25 + 0xd8;
                  if (local_a0 == 0) {
                    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                  }
                  *(undefined8 *)(local_a0 * 8 + 8) = local_d8;
                  local_a0 = local_a0 + 1;
                }
                *puVar5 = uVar3;
                local_178 = 0;
                uStack_170 = 8;
                hashbrown::map::HashMap::
                insert<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
                          ();
                if ((!SBORROW8(0,local_f8)) && (local_f8 != 0)) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
              } while( true );
            }
            lVar29 = lVar29 + 0x18;
            lVar16 = 0;
          }
          else {
            lVar16 = lVar16 + 0x18;
          }
          *puVar4 = *puVar4 + 1;
          *puVar5 = uVar3;
          country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::
          remove_wall_id();
        } while( true );
      }
      lVar25 = 0;
      lVar17 = lVar29;
      lVar29 = lVar29 + 0x10;
    }
    else {
      lVar25 = lVar17 + 0x10;
    }
    *puVar4 = *puVar4 + 1;
    local_178 = *(undefined8 *)(lVar17 + 8);
    *puVar5 = uVar3;
    country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::remove_wall_id();
    lVar17 = lVar25;
  } while( true );
}

