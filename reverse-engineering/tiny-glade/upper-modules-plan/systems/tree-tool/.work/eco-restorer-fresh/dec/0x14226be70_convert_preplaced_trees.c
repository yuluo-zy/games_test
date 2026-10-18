// ADDRESS 0x14226be70 SIZE 3202 SIGNATURE void __cdecl convert_preplaced_trees(void)

void __cdecl system_tree::convert_preplaced_trees::convert_preplaced_trees(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  undefined4 uVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  ushort uVar8;
  float fVar9;
  char extraout_AL;
  int iVar10;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *puVar11;
  void *extraout_RAX_01;
  void *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *in_RCX;
  uint uVar12;
  longlong *in_RDX;
  void *pvVar13;
  ulonglong uVar14;
  longlong lVar15;
  ulonglong uVar16;
  char *pcVar17;
  ulonglong uVar18;
  undefined1 (*pauVar19) [16];
  longlong lVar20;
  ulonglong *in_R8;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  uint uVar25;
  undefined4 *puVar26;
  void *pvVar27;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar28;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000050;
  longlong in_stack_00000058;
  undefined8 local_238;
  undefined8 local_230;
  undefined8 local_228;
  longlong local_220;
  undefined8 local_218;
  longlong local_210;
  undefined8 local_208;
  undefined8 *local_200;
  code *pcStack_1f8;
  undefined8 local_1f0;
  char *local_1e8;
  undefined1 (*pauStack_1e0) [16];
  longlong *local_1d8;
  undefined8 *local_1d0;
  undefined8 *local_1c8;
  longlong lStack_1c0;
  undefined8 local_1b8;
  undefined *local_1b0;
  undefined8 local_1a8;
  undefined4 local_1a0;
  undefined4 local_19c;
  undefined **local_198;
  undefined8 local_190;
  undefined8 **local_188;
  undefined8 local_180;
  undefined8 uStack_178;
  char *local_168;
  code *pcStack_160;
  undefined8 local_158;
  undefined8 *local_150;
  undefined8 *local_148;
  undefined4 local_13c;
  undefined4 *local_138;
  ulonglong local_130;
  longlong local_128;
  longlong local_120;
  longlong local_118;
  undefined8 *local_110;
  void *local_108;
  void *local_100;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar6 = (ulonglong *)*in_RCX;
  lVar15 = in_RCX[1];
  uVar21 = *puVar6;
  uVar24 = 0;
  uVar16 = uVar21 - *(ulonglong *)(lVar15 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar15 + 0x18)) {
    uVar16 = uVar24;
  }
  uVar22 = uVar21 - *(ulonglong *)(lVar15 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar15 + 0x38)) {
    uVar22 = uVar24;
  }
  uVar21 = *(ulonglong *)(lVar15 + 0x30);
  uVar18 = uVar21 - uVar22;
  if (uVar22 > uVar21) {
    uVar18 = uVar24;
  }
  uVar7 = *(ulonglong *)(lVar15 + 0x10);
  uVar14 = uVar7 - uVar16;
  if (uVar16 > uVar7) {
    uVar14 = uVar24;
  }
  uVar24 = *(ulonglong *)(lVar15 + 0x40);
  *puVar6 = uVar24 - (uVar14 + uVar18);
  if (uVar16 <= uVar7 && uVar7 - uVar16 != 0 || uVar22 <= uVar21 && uVar21 - uVar22 != 0) {
    *puVar6 = uVar24;
    *(undefined1 *)(in_R8 + 1) = 1;
    *in_R8 = 1;
  }
  else if (((in_R8[1] & 1) != 0) && (uVar21 = *in_R8, *in_R8 = uVar21 + 1, 2 < uVar21 + 1)) {
    *(undefined1 *)(in_R8 + 1) = 0;
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_1c8 = (undefined8 *)*extraout_RAX;
      lStack_1c0 = extraout_RAX[1];
      local_19c = (undefined4)extraout_RAX[2];
      local_1b8 = 3;
      local_1b0 = &DAT_1430356b0;
      local_1a8 = 0x24;
      local_198 = &PTR_s_Adding_converted_pre_placed_tree_143035650;
      local_190 = 1;
      local_188 = (undefined8 **)&DAT_00000008;
      local_1e8 = (char *)0x0;
      local_180 = 0;
      uStack_178 = 0;
      pauStack_1e0 = (undefined1 (*) [16])&DAT_1430356b0;
      local_1d8 = (longlong *)0x24;
      local_1d0 = (undefined8 *)0x0;
      local_1a0 = 1;
      ::log::__private_api::impl$2::log();
    }
    local_128 = 0;
    local_120 = 8;
    local_118 = 0;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      local_168 = "country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo";
      pcStack_160 = (code *)0x3f;
      if ((extraout_RAX_00[3] != 0x3f) ||
         (iVar10 = memcmp("country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo",
                          (void *)extraout_RAX_00[2],0x3f), iVar10 != 0)) {
        local_1e8 = (char *)0x0;
                    // WARNING: Subroutine does not return
        core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
      }
      puVar11 = (undefined8 *)(**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
      if (puVar11 != (undefined8 *)0x0) {
        local_230 = *puVar11;
        local_228 = puVar11[1];
        country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
        uVar25 = (uint)(ushort)local_1d0;
        lVar15 = *in_stack_00000050;
        lVar20 = in_stack_00000050[1];
        local_218 = *in_stack_00000038;
        local_13c = *(undefined4 *)((longlong)in_RDX + 0x1c);
        local_110 = (undefined8 *)*in_RDX;
        local_220 = (longlong)local_110 + 0x20;
        pcVar17 = local_1e8;
        pauVar19 = pauStack_1e0;
        puVar11 = local_1c8;
        puVar26 = (undefined4 *)in_RDX[2];
LAB_14226c1f0:
        do {
          do {
            lVar1 = local_118;
            if (puVar11 == (undefined8 *)0x0) {
LAB_14226c70b:
              local_118 = 0;
              pauStack_1e0 = (undefined1 (*) [16])(local_120 + lVar1 * 0x48);
              local_1e8 = (char *)local_120;
              local_1d8 = &local_128;
              local_1d0 = (undefined8 *)lVar1;
              local_1c8 = (undefined8 *)0x0;
              alloc::vec::spec_from_iter::impl$0::
              from_iter<enum2_<country_core::resources::history::EditType>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>,system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env_1>_>
                        ();
              local_1e8 = (char *)0x0;
              pauStack_1e0 = (undefined1 (*) [16])0x8;
              local_1d8 = (longlong *)0x0;
              std::alloc::__default_lib_allocator::__rust_alloc();
              if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              local_110 = extraout_RAX_03;
              core::ops::function::FnOnce::
              call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                        ();
              if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
                std::thread::local::panic_access_error();
              }
              lVar15 = *extraout_RAX_04;
              lVar20 = extraout_RAX_04[1];
              *extraout_RAX_04 = *extraout_RAX_04 + 1;
              *local_110 = 1;
              local_110[1] = 1;
              local_110[2] = &anon_7af864f7e249a888c0b2f574c9edb60a_1_llvm_10518705225844230474;
              local_110[3] = 0;
              local_110[4] = 0;
              local_110[5] = 0;
              local_110[6] = lVar15;
              local_110[7] = lVar20;
              local_168 = (char *)local_110;
              std::alloc::__default_lib_allocator::__rust_alloc();
              if (extraout_RAX_05 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              *extraout_RAX_05 = 1;
              extraout_RAX_05[1] = 1;
              extraout_RAX_05[2] = 0;
              extraout_RAX_05[4] = 0;
              extraout_RAX_05[6] = 0;
              extraout_RAX_05[7] = 4;
              *(undefined1 (*) [16])(extraout_RAX_05 + 8) = (undefined1  [16])0x0;
              extraout_RAX_05[10] = 4;
              extraout_RAX_05[0xb] = 0;
              local_168 = (char *)local_200;
              pcStack_160 = pcStack_1f8;
              local_158 = local_1f0;
              local_150 = local_110;
              local_148 = extraout_RAX_05;
              if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
                local_238 = local_1f0;
                local_200 = &local_238;
                pcStack_1f8 = core::fmt::num::imp::impl$9::fmt;
                local_f1 = 1;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                local_1c8 = (undefined8 *)*extraout_RAX_06;
                lStack_1c0 = extraout_RAX_06[1];
                local_19c = *(undefined4 *)(extraout_RAX_06 + 2);
                local_1b8 = 3;
                local_1b0 = &DAT_1430356b0;
                local_1a8 = 0x24;
                local_198 = &PTR_s_History_inserted_starting_edit_w_143035720;
                local_190 = 2;
                local_180 = 1;
                uStack_178 = 0;
                local_1e8 = (char *)0x0;
                pauStack_1e0 = (undefined1 (*) [16])&DAT_1430356b0;
                local_1d8 = (longlong *)0x24;
                local_1d0 = (undefined8 *)0x0;
                local_1a0 = 1;
                local_f1 = 1;
                local_188 = &local_200;
                ::log::__private_api::impl$2::log();
              }
              **(undefined4 **)(in_stack_00000058 + 0x10) =
                   *(undefined4 *)(in_stack_00000058 + 0x1c);
              local_1c8 = local_148;
              local_1d8 = (longlong *)local_158;
              local_1d0 = local_150;
              local_1e8 = local_168;
              pauStack_1e0 = (undefined1 (*) [16])pcStack_160;
              local_f1 = 0;
              country_core::resources::history::History::insert_starting_edit();
              if (local_118 != 0) {
                lVar15 = local_120 + 0x18;
                lVar20 = local_118;
                do {
                  if ((*(int *)(lVar15 + -0x18) != 1) && (*(longlong *)(lVar15 + -8) != 0)) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  lVar15 = lVar15 + 0x48;
                  lVar20 = lVar20 + -1;
                } while (lVar20 != 0);
              }
              if (local_128 == 0) {
                return;
              }
              std::alloc::__default_lib_allocator::__rust_dealloc();
              return;
            }
            if ((short)uVar25 == 0) {
              do {
                auVar4 = *pauVar19;
                uVar8 = (ushort)(SUB161(auVar4 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
                pcVar17 = pcVar17 + -0x100;
                pauVar19 = pauVar19 + 1;
              } while (uVar8 == 0xffff);
              uVar12 = ~(uint)uVar8;
              uVar25 = uVar12 & -(uint)uVar8 - 2;
            }
            else {
              if (pcVar17 == (char *)0x0) goto LAB_14226c70b;
              uVar12 = uVar25;
              uVar25 = uVar25 - 1 & uVar25;
            }
            puVar11 = (undefined8 *)((longlong)puVar11 + -1);
            iVar10 = 0;
            for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
              iVar10 = iVar10 + 1;
            }
            uVar21 = *(ulonglong *)(pcVar17 + (-8 - (ulonglong)(uint)(iVar10 << 4))) & 0xffffffff;
          } while ((*(ulonglong *)(lVar20 + 0x10) <= uVar21) ||
                  (*(int *)(*(longlong *)(lVar20 + 8) + uVar21 * 0x14) !=
                   (int)(*(ulonglong *)(pcVar17 + (-8 - (ulonglong)(uint)(iVar10 << 4))) >> 0x20)));
          lVar1 = *(longlong *)(lVar20 + 8) + uVar21 * 0x14;
          uVar21 = (ulonglong)*(uint *)(lVar1 + 4);
        } while ((uVar21 == 0xffffffff) ||
                ((*(ulonglong *)(lVar15 + 0x38) <= uVar21 ||
                 ((*(ulonglong *)
                    (*(longlong *)(lVar15 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
                   (uVar21 & 0x3f) & 1) == 0))));
        lVar2 = *(longlong *)(lVar20 + 0x1a8) + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48;
        uVar21 = *(ulonglong *)
                  (*(longlong *)(lVar20 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48);
        if ((*(ulonglong *)(lVar15 + 0x110) < uVar21) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar15 + 0x110) * 8),
           uVar16 != 0)) {
          lVar23 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar16 * 0x30);
        }
        else {
          lVar23 = 0;
        }
        if ((*(ulonglong *)(lVar15 + 0x118) < uVar21) &&
           (uVar21 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar15 + 0x118) * 8),
           uVar21 != 0)) {
          local_100 = *(void **)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar21 * 0x30);
        }
        else {
          local_100 = (void *)0x0;
        }
        local_208 = *(undefined8 *)(pcVar17 + (-0x10 - (ulonglong)(uint)(iVar10 << 4)));
        local_130 = (ulonglong)*(uint *)(lVar1 + 0x10);
        pvVar27 = (void *)(lVar23 + local_130 * 0x30);
        local_138 = puVar26;
        country_core::resources::placement_data::PlacementDataSsbo::position();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        if (((0.0 < extraout_XMM0_Da_03) || (0.0 < extraout_XMM0_Da_04)) ||
           (country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz(),
           extraout_AL != '\0')) {
          local_200 = (undefined8 *)((ulonglong)local_200 & 0xffffffffffffff00);
        }
        else {
          fVar28 = extraout_XMM0_Da + extraout_XMM0_Da_00 + extraout_XMM0_Da_01 +
                   extraout_XMM0_Da_02;
          fVar9 = 0.0;
          if (0.0 <= fVar28) {
            fVar9 = fVar28;
          }
          fVar28 = 1.0;
          if (fVar9 <= 1.0) {
            fVar28 = fVar9;
          }
          local_200 = (undefined8 *)CONCAT71(local_200._1_7_,fVar28 <= 0.0);
          if (fVar28 <= 0.0) {
            ::utils::geometry::circle::Circle2d::new();
            country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
            query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env_0>
                      ();
            if (((ulonglong)local_200 & 1) != 0) {
              pvVar13 = pvVar27;
              country_core::resources::placement_data::PlacementDataSsbo::position();
              uVar3 = *(undefined8 *)((longlong)pvVar27 + 0xc);
              uVar5 = *(undefined4 *)((longlong)pvVar27 + 0x14);
              asset::prefab::PrefabName::as_name_ref();
              puVar26 = local_138;
              local_130 = CONCAT44(local_130._4_4_,uVar5);
              if ((longlong)pvVar13 < 0) {
LAB_14226cae9:
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
              local_108 = extraout_RAX_01;
              local_100 = pvVar13;
              if (pvVar13 == (void *)0x0) {
                pvVar27 = (void *)0x1;
              }
              else {
                std::alloc::__default_lib_allocator::__rust_alloc();
                pvVar27 = extraout_RAX_02;
                if (extraout_RAX_02 == (void *)0x0) {
                  local_108 = local_100;
                  goto LAB_14226cae9;
                }
              }
              pvVar13 = local_108;
              local_108 = pvVar27;
              memcpy(pvVar27,pvVar13,(size_t)local_100);
              local_168 = local_1e8;
              pcStack_160 = (code *)CONCAT44(pcStack_160._4_4_,pauStack_1e0._0_4_);
              local_210 = local_118;
              if (local_118 == local_128) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                          ();
              }
              *(undefined4 *)(local_120 + local_210 * 0x48) = 0;
              *(undefined8 *)(local_120 + 8 + local_210 * 0x48) = local_208;
              *(void **)(local_120 + 0x10 + local_210 * 0x48) = local_100;
              *(void **)(local_120 + 0x18 + local_210 * 0x48) = local_108;
              *(void **)(local_120 + 0x20 + local_210 * 0x48) = local_100;
              *(char **)(local_120 + 0x28 + local_210 * 0x48) = local_168;
              *(undefined4 *)(local_120 + 0x30 + local_210 * 0x48) = pcStack_160._0_4_;
              *(undefined8 *)(local_120 + 0x34 + local_210 * 0x48) = uVar3;
              *(undefined4 *)(local_120 + 0x3c + local_210 * 0x48) = (undefined4)local_130;
              *(undefined1 *)(local_120 + 0x40 + local_210 * 0x48) = 1;
              local_118 = local_210 + 1;
              goto LAB_14226c1f0;
            }
          }
        }
        *local_138 = local_13c;
        lVar1 = *(longlong *)((longlong)local_110 + 0x30);
        local_100 = *(void **)((longlong)local_110 + 0x40);
        if (lVar1 == *(longlong *)((longlong)local_110 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar2 = *(longlong *)((longlong)local_110 + 0x28);
        *(void **)(lVar2 + lVar1 * 0x10) = local_100;
        *(undefined8 *)(lVar2 + 8 + lVar1 * 0x10) = local_208;
        *(longlong *)((longlong)local_110 + 0x30) = lVar1 + 1;
        *(longlong *)((longlong)local_110 + 0x40) = *(longlong *)((longlong)local_110 + 0x40) + 1;
        puVar26 = local_138;
        goto LAB_14226c1f0;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  return;
}


// LOCALS:
[
 {
  "name": "lStack_1c0",
  "type": "longlong",
  "storage": "Stack[-0x1c0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcStack_1f8",
  "type": "undefined *",
  "storage": "Stack[-0x1f8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pauStack_1e0",
  "type": "undefined1[16] *",
  "storage": "Stack[-0x1e0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_238",
  "type": "undefined8",
  "storage": "Stack[-0x238]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_230",
  "type": "undefined8",
  "storage": "Stack[-0x230]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_228",
  "type": "undefined8",
  "storage": "Stack[-0x228]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_220",
  "type": "longlong",
  "storage": "Stack[-0x220]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_218",
  "type": "undefined8",
  "storage": "Stack[-0x218]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_210",
  "type": "longlong",
  "storage": "Stack[-0x210]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_208",
  "type": "undefined8",
  "storage": "Stack[-0x208]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_200",
  "type": "undefined8 *",
  "storage": "Stack[-0x200]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar4",
  "type": "undefined1[16]",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1a0",
  "type": "undefined4",
  "storage": "Stack[-0x1a0]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_19c",
  "type": "undefined4",
  "storage": "Stack[-0x19c]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar3",
  "type": "undefined8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_198",
  "type": "undefined * *",
  "storage": "Stack[-0x198]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar6",
  "type": "ulonglong *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_190",
  "type": "undefined8",
  "storage": "Stack[-0x190]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar5",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_188",
  "type": "undefined8 * *",
  "storage": "Stack[-0x188]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar8",
  "type": "ushort",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_180",
  "type": "undefined8",
  "storage": "Stack[-0x180]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar7",
  "type": "ulonglong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_AL",
  "type": "char",
  "storage": "AL:1",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_168",
  "type": "char *",
  "storage": "Stack[-0x168]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar9",
  "type": "float",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_158",
  "type": "undefined8",
  "storage": "Stack[-0x158]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1f0",
  "type": "undefined8",
  "storage": "Stack[-0x1f0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1e8",
  "type": "char *",
  "storage": "Stack[-0x1e8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1d8",
  "type": "longlong *",
  "storage": "Stack[-0x1d8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1d0",
  "type": "undefined8 *",
  "storage": "Stack[-0x1d0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1c8",
  "type": "undefined8 *",
  "storage": "Stack[-0x1c8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1b8",
  "type": "undefined8",
  "storage": "Stack[-0x1b8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar2",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1b0",
  "type": "undefined *",
  "storage": "Stack[-0x1b0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar1",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_1a8",
  "type": "undefined8",
  "storage": "Stack[-0x1a8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_110",
  "type": "undefined8 *",
  "storage": "Stack[-0x110]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_108",
  "type": "void *",
  "storage": "Stack[-0x108]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_100",
  "type": "void *",
  "storage": "Stack[-0x100]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_f1",
  "type": "undefined1",
  "storage": "Stack[-0xf1]:1",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_f0",
  "type": "undefined8",
  "storage": "Stack[-0xf0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_150",
  "type": "undefined8 *",
  "storage": "Stack[-0x150]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_148",
  "type": "undefined8 *",
  "storage": "Stack[-0x148]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_13c",
  "type": "undefined4",
  "storage": "Stack[-0x13c]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_138",
  "type": "undefined4 *",
  "storage": "Stack[-0x138]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_178",
  "type": "undefined8",
  "storage": "Stack[-0x178]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_130",
  "type": "ulonglong",
  "storage": "Stack[-0x130]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_128",
  "type": "longlong",
  "storage": "Stack[-0x128]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcStack_160",
  "type": "undefined *",
  "storage": "Stack[-0x160]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_120",
  "type": "longlong",
  "storage": "Stack[-0x120]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_118",
  "type": "longlong",
  "storage": "Stack[-0x118]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar24",
  "type": "ulonglong",
  "storage": "R11:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar23",
  "type": "longlong",
  "storage": "R10:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar26",
  "type": "undefined4 *",
  "storage": "R14:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar25",
  "type": "uint",
  "storage": "R13D:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "pvVar27",
  "type": "void *",
  "storage": "R14:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_01",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_00",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "pcVar17",
  "type": "char *",
  "storage": "RSI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar16",
  "type": "ulonglong",
  "storage": "RSI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pauVar19",
  "type": "undefined1[16] *",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar18",
  "type": "ulonglong",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_R8",
  "type": "ulonglong *",
  "storage": "R8:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar20",
  "type": "longlong",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar22",
  "type": "ulonglong",
  "storage": "R10:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar21",
  "type": "ulonglong",
  "storage": "R8:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_06",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_05",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "uVar12",
  "type": "uint",
  "storage": "EDX:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_RCX",
  "type": "undefined8 *",
  "storage": "RCX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "pvVar13",
  "type": "void *",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_RDX",
  "type": "longlong *",
  "storage": "RDX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar15",
  "type": "longlong",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar14",
  "type": "ulonglong",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX",
  "type": "longlong *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "iVar10",
  "type": "int",
  "storage": "EAX:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar11",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_00",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_02",
  "type": "void *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_01",
  "type": "void *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_04",
  "type": "longlong *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX_03",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_03",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_02",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "fVar28",
  "type": "float",
  "storage": "XMM10_Da:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_04",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000050",
  "type": "longlong *",
  "storage": "Stack[0x50]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000038",
  "type": "undefined8 *",
  "storage": "Stack[0x38]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000058",
  "type": "longlong",
  "storage": "Stack[0x58]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 }
]