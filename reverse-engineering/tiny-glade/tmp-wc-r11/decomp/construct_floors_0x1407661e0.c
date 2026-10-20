// name: construct_floors
// addr: 0x1407661e0
// size: 1855
// signature: void __cdecl construct_floors(void)


void __cdecl system_wall_constructor::construct_floors::construct_floors(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  ulonglong *puVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  float fVar11;
  char extraout_AL;
  char extraout_AL_00;
  longlong lVar12;
  longlong lVar13;
  ulonglong extraout_RAX;
  ulonglong *extraout_RAX_00;
  longlong *in_RCX;
  ulonglong uVar14;
  longlong lVar15;
  undefined8 *in_RDX;
  longlong lVar16;
  ushort uVar17;
  longlong lVar18;
  longlong lVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong lVar23;
  uint uVar24;
  longlong *plVar25;
  longlong *plVar26;
  byte bVar27;
  char cVar28;
  char cVar33;
  float fVar29;
  char cVar31;
  char cVar32;
  undefined1 auVar30 [16];
  undefined1 auVar34 [12];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  longlong in_stack_00000028;
  longlong local_1d0;
  undefined4 uStack_190;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined4 uStack_160;
  undefined8 uStack_150;
  undefined7 uStack_137;
  undefined8 uStack_130;
  undefined4 uStack_104;
  undefined8 uStack_100;
  undefined8 local_f8;
  longlong *plStack_f0;
  longlong *local_e8;
  undefined4 local_dc;
  undefined4 uStack_d8;
  undefined4 local_d4;
  undefined8 local_c8;
  undefined8 uStack_c0;
  
  puVar4 = (ulonglong *)*in_R8;
  lVar16 = in_R8[1];
  uVar22 = *puVar4;
  uVar20 = 0;
  uVar14 = uVar22 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar14 = uVar20;
  }
  uVar21 = uVar22 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar21 = uVar20;
  }
  lVar12 = uVar14 * 0x10 + *(longlong *)(lVar16 + 8);
  uVar22 = *(ulonglong *)(lVar16 + 0x10) - uVar14;
  if (*(ulonglong *)(lVar16 + 0x10) < uVar14) {
    lVar12 = 8;
    uVar22 = uVar20;
  }
  lVar15 = uVar21 * 0x10 + *(longlong *)(lVar16 + 0x28);
  uVar14 = *(ulonglong *)(lVar16 + 0x30) - uVar21;
  if (*(ulonglong *)(lVar16 + 0x30) < uVar21) {
    lVar15 = 8;
    uVar14 = uVar20;
  }
  *puVar4 = (*(longlong *)(lVar16 + 0x40) - uVar14) - uVar22;
  lVar16 = uVar22 * 0x10 + lVar12;
  lVar18 = uVar14 * 0x10 + lVar15;
  do {
    if ((lVar12 == 0) || (lVar12 == lVar16)) {
      if ((lVar15 == 0) || (lVar15 == lVar18)) {
        puVar4 = (ulonglong *)*in_RDX;
        lVar16 = in_RDX[1];
        uVar22 = *puVar4;
        uVar20 = 0;
        uVar14 = uVar22 - *(ulonglong *)(lVar16 + 0x18);
        if (uVar22 < *(ulonglong *)(lVar16 + 0x18)) {
          uVar14 = uVar20;
        }
        uVar21 = uVar22 - *(ulonglong *)(lVar16 + 0x38);
        if (uVar22 < *(ulonglong *)(lVar16 + 0x38)) {
          uVar21 = uVar20;
        }
        lVar12 = uVar14 * 0x10 + *(longlong *)(lVar16 + 8);
        uVar22 = *(ulonglong *)(lVar16 + 0x10) - uVar14;
        if (*(ulonglong *)(lVar16 + 0x10) < uVar14) {
          lVar12 = 8;
          uVar22 = uVar20;
        }
        local_1d0 = uVar21 * 0x10 + *(longlong *)(lVar16 + 0x28);
        uVar14 = *(ulonglong *)(lVar16 + 0x30) - uVar21;
        if (*(ulonglong *)(lVar16 + 0x30) < uVar21) {
          local_1d0 = 8;
          uVar14 = uVar20;
        }
        *puVar4 = (*(longlong *)(lVar16 + 0x40) - uVar14) - uVar22;
        lVar16 = uVar22 * 0x10 + lVar12;
        lVar15 = uVar14 * 0x10 + local_1d0;
        plVar25 = (longlong *)*in_RCX;
        iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                 CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                       CONCAT13(unaff_XMM6_Bd,
                                CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
        iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                 CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                       CONCAT13(unaff_XMM6_Bh,
                                CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
        iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                 CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                       CONCAT13(unaff_XMM6_Bl,
                                CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
        iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                  CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                        CONCAT13(unaff_XMM6_Bp,
                                 CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_14076649a:
        if ((lVar12 == 0) || (lVar12 == lVar16)) {
          if ((local_1d0 == 0) || (local_1d0 == lVar15)) {
            return;
          }
          lVar12 = 0;
          lVar18 = local_1d0;
          local_1d0 = local_1d0 + 0x10;
        }
        else {
          lVar18 = lVar12;
          lVar12 = lVar12 + 0x10;
        }
        *puVar4 = *puVar4 + 1;
        lVar18 = *(longlong *)(lVar18 + 8);
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        hashbrown::raw::RawTable::
        remove_entry<tuple_<country_core::resources::walls::WallId,bevy_ecs::entity::Entity>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,bevy_ecs::entity::Entity>_>
                  ();
        plVar26 = (longlong *)0x0;
        if (uStack_170._4_4_ != 0) {
          plVar26 = uStack_170;
        }
        if (((ulonglong)plVar26 >> 0x20 != 0) &&
           (bevy_ecs::entity::Entities::contains(), extraout_AL_00 != '\0')) {
          uStack_170 = *(longlong **)(in_stack_00000028 + 8);
          uStack_168 = *(undefined8 *)(in_stack_00000028 + 0x10);
          uStack_160 = (undefined4)*(undefined8 *)(in_stack_00000028 + 0x18);
          bevy_ecs::system::commands::EntityCommands::despawn();
        }
        if (plVar25[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar13 = *plVar25;
          bVar27 = (byte)(extraout_RAX >> 0x39);
          auVar30 = ZEXT216(CONCAT11(bVar27,bVar27));
          auVar30 = pshuflw(auVar30,auVar30,0);
          lVar23 = 0;
          uVar22 = extraout_RAX;
          while( true ) {
            uVar22 = plVar25[1] & uVar22;
            pcVar1 = (char *)(lVar13 + uVar22);
            cVar28 = auVar30[0];
            auVar36[0] = -(*pcVar1 == cVar28);
            cVar31 = auVar30[1];
            auVar36[1] = -(pcVar1[1] == cVar31);
            cVar32 = auVar30[2];
            auVar36[2] = -(pcVar1[2] == cVar32);
            cVar33 = auVar30[3];
            auVar36[3] = -(pcVar1[3] == cVar33);
            auVar36[4] = -(pcVar1[4] == cVar28);
            auVar36[5] = -(pcVar1[5] == cVar31);
            auVar36[6] = -(pcVar1[6] == cVar32);
            auVar36[7] = -(pcVar1[7] == cVar33);
            auVar36[8] = -(pcVar1[8] == cVar28);
            auVar36[9] = -(pcVar1[9] == cVar31);
            auVar36[10] = -(pcVar1[10] == cVar32);
            auVar36[0xb] = -(pcVar1[0xb] == cVar33);
            auVar36[0xc] = -(pcVar1[0xc] == cVar28);
            auVar36[0xd] = -(pcVar1[0xd] == cVar31);
            auVar36[0xe] = -(pcVar1[0xe] == cVar32);
            auVar36[0xf] = -(pcVar1[0xf] == cVar33);
            uVar17 = (ushort)(SUB161(auVar36 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar36[0xf] >> 7) << 0xf;
            uVar24 = (uint)uVar17;
            while (uVar17 != 0) {
              uVar5 = 0;
              for (uVar6 = uVar24; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                uVar5 = uVar5 + 1;
              }
              lVar19 = (uVar5 + uVar22 & plVar25[1]) * -0x2a8;
              if (lVar18 == *(longlong *)(lVar13 + -0x2a8 + lVar19)) {
                if ((0.0 < *(float *)(lVar13 + -0x20 + lVar19)) &&
                   (*(uint *)(lVar13 + lVar19 + -0x48) < 2)) {
                  country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
                            ();
                  uVar22 = *extraout_RAX_00;
                  fVar11 = *(float *)((longlong)extraout_RAX_00 + 0xc);
                  uVar14 = extraout_RAX_00[2];
                  uVar20 = extraout_RAX_00[3];
                  uStack_190 = (undefined4)extraout_RAX_00[1];
                  if ((uVar22 & 1) == 0) {
                    auVar34 = (undefined1  [12])0x0;
                    fVar29 = fVar11;
                    if (fVar11 <= 1.0) {
                      auVar34 = ZEXT812(0);
                      fVar29 = 1.0;
                    }
                    auVar30._0_4_ = fVar29 * 3.0;
                    auVar30._4_12_ = auVar34;
                    log2f(auVar30._0_8_);
                  }
                  country_core::resources::walls::decorator::DecoratorRank::new();
                  country_core::components::drawable::impl$1::from();
                  country_core::systems::wall::valid_enclosure::ShapeParameters::center();
                  ::utils::swizzlers::impl$1::xvy();
                  ::utils::components::transform::impl$1::default();
                  bevy_app::plugin::Plugin::
                  finish<country_core::systems::main_camera::MainCameraPlugin>();
                  uStack_170 = plStack_f0;
                  bevy_ecs::system::commands::Commands::spawn_empty();
                  plVar26 = plStack_f0;
                  if (plStack_f0 == (longlong *)0x0) {
                    plVar26 = local_e8;
                  }
                  lVar13 = plVar26[2];
                  if ((ulonglong)(*plVar26 - lVar13) < 0xb8) {
                    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  }
                  lVar23 = plVar26[1];
                  *(code **)(lVar23 + lVar13) =
                       core::ops::function::FnOnce::
                       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>,country_core::resources::walls::Floor>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
                  ;
                  puVar2 = (undefined8 *)(lVar23 + 8 + lVar13);
                  *puVar2 = local_f8;
                  puVar2[1] = plStack_f0;
                  puVar2 = (undefined8 *)(lVar23 + 0x18 + lVar13);
                  *puVar2 = uStack_168;
                  puVar2[1] = CONCAT44(local_dc,uStack_160);
                  puVar2 = (undefined8 *)(lVar23 + 0x28 + lVar13);
                  *puVar2 = CONCAT44(local_d4,uStack_d8);
                  puVar2[1] = uStack_150;
                  puVar2 = (undefined8 *)(lVar23 + 0x38 + lVar13);
                  *puVar2 = local_c8;
                  puVar2[1] = uStack_c0;
                  puVar2 = (undefined8 *)(lVar23 + 0x48 + lVar13);
                  *puVar2 = CONCAT71(uStack_137,9);
                  puVar2[1] = uStack_130;
                  plVar3 = (longlong *)(lVar23 + 0x58 + lVar13);
                  *plVar3 = lVar18;
                  plVar3[1] = uVar22;
                  puVar2 = (undefined8 *)(lVar23 + 0x68 + lVar13);
                  *puVar2 = CONCAT44(fVar11,uStack_190);
                  puVar2[1] = uVar14;
                  puVar2 = (undefined8 *)(lVar23 + 0x78 + lVar13);
                  *puVar2 = CONCAT44(uStack_104,(int)uVar20);
                  puVar2[1] = uStack_100;
                  *(undefined1 *)(lVar23 + 0x88 + lVar13) = 0;
                  *(ulonglong *)(lVar23 + 0x98 + lVar13) = CONCAT44(local_d4,uStack_d8);
                  *(code **)(lVar23 + 0xa8 + lVar13) = bevy_ecs::error::handler::panic;
                  plVar26[2] = lVar13 + 0xb8;
                  hashbrown::map::HashMap::
                  insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                            ();
                }
                goto LAB_14076649a;
              }
              uVar17 = (ushort)(uVar24 - 1) & (ushort)uVar24;
              uVar24 = CONCAT22((short)(uVar24 - 1 >> 0x10),uVar17);
            }
            auVar35[0] = -(*pcVar1 == (char)iVar7);
            auVar35[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
            auVar35[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
            auVar35[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
            auVar35[4] = -(pcVar1[4] == (char)iVar8);
            auVar35[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
            auVar35[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
            auVar35[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
            auVar35[8] = -(pcVar1[8] == (char)iVar9);
            auVar35[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
            auVar35[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
            auVar35[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
            auVar35[0xc] = -(pcVar1[0xc] == (char)iVar10);
            auVar35[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
            auVar35[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
            auVar35[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
            if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar35 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar35 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar35 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar35 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar35 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar35 >> 0x3f,0) & 1) != 0) || (SUB161(auVar35 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar35 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar35 >> 0x57,0) & 1) != 0) || (SUB161(auVar35 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar35 >> 0x67,0) & 1) != 0) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') break;
            uVar22 = uVar22 + lVar23 + 0x10;
            lVar23 = lVar23 + 0x10;
          }
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar12 = 0;
      lVar13 = lVar15;
      lVar15 = lVar15 + 0x10;
    }
    else {
      lVar13 = lVar12;
      lVar12 = lVar12 + 0x10;
    }
    *puVar4 = *puVar4 + 1;
    local_f8 = *(undefined8 *)(lVar13 + 8);
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::WallId,bevy_ecs::entity::Entity>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,bevy_ecs::entity::Entity>_>
              ();
    plVar25 = (longlong *)0x0;
    if (uStack_170._4_4_ != 0) {
      plVar25 = uStack_170;
    }
    if (((ulonglong)plVar25 >> 0x20 != 0) &&
       (bevy_ecs::entity::Entities::contains(), extraout_AL != '\0')) {
      uStack_170 = *(longlong **)(in_stack_00000028 + 8);
      uStack_168 = *(undefined8 *)(in_stack_00000028 + 0x10);
      uStack_160 = (undefined4)*(undefined8 *)(in_stack_00000028 + 0x18);
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  } while( true );
}

