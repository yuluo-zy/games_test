// signature: void __cdecl roof_tiles_gravity_compute(void)

void __cdecl
country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
          (void)

{
  int *piVar1;
  char *pcVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined8 *extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RDX;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  uint uVar11;
  ushort uVar12;
  ulonglong uVar13;
  bool bVar14;
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined *local_3b8;
  undefined8 uStack_3b0;
  undefined8 local_3a8;
  longlong local_2c0;
  longlong local_2b8;
  undefined1 local_2b0;
  char local_282;
  undefined8 local_220 [20];
  undefined8 local_180;
  longlong local_e0;
  longlong local_d0;
  longlong local_c8;
  undefined **local_c0;
  longlong local_b8;
  longlong local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined1 local_90 [16];
  undefined1 local_80 [16];
  undefined1 local_70 [16];
  undefined8 local_60;
  undefined1 local_58;
  undefined3 uStack_53;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  if (*(longlong *)(in_RDX + 0x88) != 0) {
    lVar3 = *(longlong *)(in_RDX + 0x70);
    uVar9 = *(ulonglong *)(in_RDX + 0x78);
    uVar7 = uVar9 & 0x65cfda6a55a841ac;
    lVar10 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar3 + uVar7);
      auVar17[0] = -(*pcVar2 == '2');
      auVar17[1] = -(pcVar2[1] == '2');
      auVar17[2] = -(pcVar2[2] == '2');
      auVar17[3] = -(pcVar2[3] == '2');
      auVar17[4] = -(pcVar2[4] == '2');
      auVar17[5] = -(pcVar2[5] == '2');
      auVar17[6] = -(pcVar2[6] == '2');
      auVar17[7] = -(pcVar2[7] == '2');
      auVar17[8] = -(pcVar2[8] == '2');
      auVar17[9] = -(pcVar2[9] == '2');
      auVar17[10] = -(pcVar2[10] == '2');
      auVar17[0xb] = -(pcVar2[0xb] == '2');
      auVar17[0xc] = -(pcVar2[0xc] == '2');
      auVar17[0xd] = -(pcVar2[0xd] == '2');
      auVar17[0xe] = -(pcVar2[0xe] == '2');
      auVar17[0xf] = -(pcVar2[0xf] == '2');
      uVar12 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar13 = uVar4 + uVar7 & uVar9;
        if ((*(longlong *)(lVar3 + -0x18 + uVar13 * -0x18) == -0x7dd9338791986cab) &&
           (*(longlong *)(lVar3 + -0x10 + uVar13 * -0x18) == 0x65cfda6a55a841ac)) {
          uVar9 = *(ulonglong *)(lVar3 + uVar13 * -0x18 + -8);
          if (*(ulonglong *)(in_RDX + 0x218) <= uVar9) goto LAB_140a2a05d;
          goto LAB_140a29cfc;
        }
        uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
      }
      auVar15[0] = -(*pcVar2 == -1);
      auVar15[1] = -(pcVar2[1] == -1);
      auVar15[2] = -(pcVar2[2] == -1);
      auVar15[3] = -(pcVar2[3] == -1);
      auVar15[4] = -(pcVar2[4] == -1);
      auVar15[5] = -(pcVar2[5] == -1);
      auVar15[6] = -(pcVar2[6] == -1);
      auVar15[7] = -(pcVar2[7] == -1);
      auVar15[8] = -(pcVar2[8] == -1);
      auVar15[9] = -(pcVar2[9] == -1);
      auVar15[10] = -(pcVar2[10] == -1);
      auVar15[0xb] = -(pcVar2[0xb] == -1);
      auVar15[0xc] = -(pcVar2[0xc] == -1);
      auVar15[0xd] = -(pcVar2[0xd] == -1);
      auVar15[0xe] = -(pcVar2[0xe] == -1);
      auVar15[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar15 >> 7,0) & 1) != 0 || (SUB161(auVar15 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar15 >> 0x17,0) & 1) != 0) || (SUB161(auVar15 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar15 >> 0x27,0) & 1) != 0) || (SUB161(auVar15 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar15 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar15 >> 0x3f,0) & 1) != 0) || (SUB161(auVar15 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar15 >> 0x4f,0) & 1) != 0) || (SUB161(auVar15 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar15 >> 0x5f,0) & 1) != 0) || (SUB161(auVar15 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar15 >> 0x6f,0) & 1) != 0) || (SUB161(auVar15 >> 0x77,0) & 1) != 0) ||
          auVar15[0xf] < '\0') break;
      lVar8 = uVar7 + lVar10;
      lVar10 = lVar10 + 0x10;
      uVar7 = lVar8 + 0x10U & uVar9;
    }
  }
  uVar11 = *(uint *)(in_RDX + 0x90);
  if (uVar11 < 0x3ffffffe) {
    LOCK();
    bVar14 = uVar11 == *(uint *)(in_RDX + 0x90);
    if (bVar14) {
      *(uint *)(in_RDX + 0x90) = uVar11 + 1;
    }
    UNLOCK();
    if (bVar14) {
      lVar3 = *(longlong *)(in_RDX + 0xf0);
      goto joined_r0x000140a2a03d;
    }
  }
  std::sys::sync::rwlock::futex::RwLock::read_contended();
  lVar3 = *(longlong *)(in_RDX + 0xf0);
joined_r0x000140a2a03d:
  if (lVar3 != 0) {
    lVar3 = *(longlong *)(in_RDX + 0xd8);
    uVar9 = *(ulonglong *)(in_RDX + 0xe0);
    uVar7 = uVar9 & 0x65cfda6a55a841ac;
    lVar10 = 0;
    do {
      pcVar2 = (char *)(lVar3 + uVar7);
      auVar18[0] = -(*pcVar2 == '2');
      auVar18[1] = -(pcVar2[1] == '2');
      auVar18[2] = -(pcVar2[2] == '2');
      auVar18[3] = -(pcVar2[3] == '2');
      auVar18[4] = -(pcVar2[4] == '2');
      auVar18[5] = -(pcVar2[5] == '2');
      auVar18[6] = -(pcVar2[6] == '2');
      auVar18[7] = -(pcVar2[7] == '2');
      auVar18[8] = -(pcVar2[8] == '2');
      auVar18[9] = -(pcVar2[9] == '2');
      auVar18[10] = -(pcVar2[10] == '2');
      auVar18[0xb] = -(pcVar2[0xb] == '2');
      auVar18[0xc] = -(pcVar2[0xc] == '2');
      auVar18[0xd] = -(pcVar2[0xd] == '2');
      auVar18[0xe] = -(pcVar2[0xe] == '2');
      auVar18[0xf] = -(pcVar2[0xf] == '2');
      uVar12 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar8 = (uVar4 + uVar7 & uVar9) * -0x88;
        if ((*(longlong *)(lVar3 + -0x88 + lVar8) == -0x7dd9338791986cab) &&
           (*(longlong *)(lVar3 + -0x80 + lVar8) == 0x65cfda6a55a841ac)) {
          uVar9 = *(ulonglong *)(lVar3 + lVar8 + -8);
          LOCK();
          piVar1 = (int *)(in_RDX + 0x90);
          iVar6 = *piVar1;
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
            std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
            if (*(ulonglong *)(in_RDX + 0x218) <= uVar9) goto LAB_140a2a05d;
            goto LAB_140a29cfc;
          }
          if (uVar9 < *(ulonglong *)(in_RDX + 0x218)) {
LAB_140a29cfc:
            uVar9 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar9 * 8);
            if ((uVar9 != 0) &&
               (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar9 * 0x60) != 0)) {
              frozen::FrozenMap::
              get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                        ();
              if (extraout_RAX != (undefined8 *)0x0) {
                local_3b8 = &anon_107183261458dea85466985370debbb6_82_llvm_3641509820080252961;
                uStack_3b0 = 0x35;
                if ((extraout_RAX[3] != 0x35) ||
                   (iVar6 = memcmp(&
                                   anon_107183261458dea85466985370debbb6_82_llvm_3641509820080252961
                                   ,(void *)extraout_RAX[2],0x35), iVar6 != 0)) {
                  local_220[0] = 0;
                    // WARNING: Subroutine does not return
                  core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
                }
                local_e0 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
                if (local_e0 != 0) {
                  bevy_ecs::query::state::QueryState::
                  new<tuple_<ref_<country_core::resources::roofs::roof_tiles::RoofTiles>,ref_<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,tuple_<>_>
                            ();
                  bevy_ecs::query::state::QueryState::
                  update_archetypes_unsafe_world_cell<tuple_<ref_<country_core::resources::roofs::roof_tiles::RoofTiles>,ref_<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,tuple_<>_>
                            ();
                  if (local_3b8 != *(undefined **)(in_RDX + 0x5a0)) {
                    // WARNING: Subroutine does not return
                    bevy_ecs::query::state::impl$2::validate_world::panic_mismatched();
                  }
                  local_b0 = local_2c0 + local_2b8 * 4;
                  local_70 = (undefined1  [16])0x0;
                  local_80 = (undefined1  [16])0x0;
                  local_90 = (undefined1  [16])0x0;
                  local_60 = 0;
                  local_c8 = in_RDX + 0xf8;
                  local_d0 = in_RDX + 0x1a0;
                  local_b8 = local_2c0;
                  local_a8 = 8;
                  local_a0 = 0;
                  local_98 = 8;
                  local_58 = local_2b0;
                  uStack_53 = (undefined3)uStack_4c;
                  local_220[0] = 0;
                  local_180 = 0;
                  local_c0 = &local_3b8;
                  alloc::vec::spec_from_iter::impl$0::
                  from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_tiles::RoofTiles>,ref_<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,tuple_<>_>,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>_>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_0::closure_0::closure_env_0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_0::closure_env_0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env_0>_>
                            ();
                  local_31 = 1;
                  core::ptr::
                  drop_in_place<bevy_ecs::query::state::QueryState<ref_<country_core::components::WoodenGate>,tuple_<>_>_>
                            ();
                  local_31 = 1;
                  country_core::render::pass_builder::compute_pass_builder::ComputePass::
                  new<ref_<str_>_>();
                  local_31 = 1;
                  country_core::render::pass_builder::compute_pass_builder::impl$3::
                  read_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
                  local_32 = 1;
                  uVar9 = 0x4bc362195b80f729;
                  bevy_ecs::component::Components::get_resource_id();
                  if ((((extraout_RAX_00 != 0) && (uVar9 < *(ulonglong *)(in_RDX + 0x218))) &&
                      (uVar9 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar9 * 8), uVar9 != 0
                      )) && (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar9 * 0x60) !=
                             0)) {
                    if (local_282 != '\b') {
                      country_core::render::pass_builder::uniform_binder::UniformBinder::
                      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                                ();
                    }
                    memcpy(local_220,&local_3b8,400);
                    local_3a8 = local_40;
                    uStack_3b0 = uStack_48;
                    local_31 = 0;
                    country_core::render::pass_builder::compute_pass_builder::ComputePass::
                    dispatch_with<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env_1>
                              ();
                    return;
                  }
                  local_32 = 1;
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
              }
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
          }
          goto LAB_140a2a05d;
        }
        uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
      }
      auVar16[0] = -(*pcVar2 == -1);
      auVar16[1] = -(pcVar2[1] == -1);
      auVar16[2] = -(pcVar2[2] == -1);
      auVar16[3] = -(pcVar2[3] == -1);
      auVar16[4] = -(pcVar2[4] == -1);
      auVar16[5] = -(pcVar2[5] == -1);
      auVar16[6] = -(pcVar2[6] == -1);
      auVar16[7] = -(pcVar2[7] == -1);
      auVar16[8] = -(pcVar2[8] == -1);
      auVar16[9] = -(pcVar2[9] == -1);
      auVar16[10] = -(pcVar2[10] == -1);
      auVar16[0xb] = -(pcVar2[0xb] == -1);
      auVar16[0xc] = -(pcVar2[0xc] == -1);
      auVar16[0xd] = -(pcVar2[0xd] == -1);
      auVar16[0xe] = -(pcVar2[0xe] == -1);
      auVar16[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') break;
      lVar8 = uVar7 + lVar10;
      lVar10 = lVar10 + 0x10;
      uVar7 = lVar8 + 0x10U & uVar9;
    } while( true );
  }
  LOCK();
  piVar1 = (int *)(in_RDX + 0x90);
  iVar6 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_140a2a05d:
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

