
// WARNING: Removing unreachable block (ram,0x000141bf21d5)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl forest::manage_tree_prefabs::manage_tree_prefabs(void)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  float fVar4;
  byte bVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong lVar11;
  undefined8 uVar12;
  longlong lVar13;
  ulonglong *puVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  longlong lVar19;
  code *pcVar20;
  ulonglong *puVar21;
  char extraout_AL;
  byte extraout_AL_00;
  uint uVar22;
  int iVar23;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar24;
  ulonglong uVar25;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  ulonglong uVar26;
  longlong lVar27;
  longlong *plVar28;
  ulonglong in_RDX;
  byte *pbVar29;
  ulonglong uVar30;
  longlong lVar31;
  byte bVar32;
  byte bVar33;
  byte *pbVar34;
  longlong lVar35;
  longlong lVar36;
  byte *pbVar37;
  longlong *in_R8;
  undefined **ppuVar38;
  ulonglong *puVar39;
  longlong in_R9;
  byte *pbVar40;
  longlong lVar41;
  byte *pbVar42;
  bool bVar43;
  bool bVar44;
  bool bVar45;
  bool bVar46;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float fVar47;
  undefined4 uVar48;
  float fVar49;
  float fVar50;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000070;
  undefined8 local_220;
  float local_1f4;
  float local_1ec;
  undefined4 uStack_164;
  undefined4 uStack_160;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x30) {
          if ((char)(&DAT_142efd668)[in_RDX] < -0x40) {
LAB_141bf1790:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x30) goto LAB_141bf1790;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x30) {
          if ("rest::manage_tree_prefabs::manage_tree_prefabs::f"[in_RDX] < -0x40) {
LAB_141bf17e4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x30) goto LAB_141bf17e4;
      }
    }
    ppuVar38 = &PTR_DAT_142efd798;
    do {
      bVar32 = *(byte *)((longlong)ppuVar38 + -1);
      uVar22 = (uint)(char)bVar32;
      if ((char)bVar32 < '\0') {
        bVar33 = *(byte *)((longlong)ppuVar38 + -2);
        if ((char)bVar33 < -0x40) {
          bVar5 = *(byte *)((longlong)ppuVar38 + -3);
          if ((char)bVar5 < -0x40) {
            pbVar40 = (byte *)((longlong)ppuVar38 + -4);
            ppuVar38 = (undefined **)((longlong)ppuVar38 + -4);
            uVar22 = bVar5 & 0x3f | (*pbVar40 & 7) << 6;
          }
          else {
            ppuVar38 = (undefined **)((longlong)ppuVar38 + -3);
            uVar22 = bVar5 & 0xf;
          }
          uVar22 = bVar33 & 0x3f | uVar22 << 6;
        }
        else {
          ppuVar38 = (undefined **)((longlong)ppuVar38 + -2);
          uVar22 = bVar33 & 0x1f;
        }
        uVar22 = (uint)(bVar32 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_141bf18a4;
LAB_141bf18b5:
        if (ppuVar38 == (undefined **)&UNK_142efd767) break;
        if ((undefined *)((longlong)ppuVar38 + -0x142efd767) < (undefined *)0x30) {
          if (-0x41 < *(char *)((longlong)ppuVar38 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar38 + -0x142efd767) == (undefined *)0x30) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar38 = (undefined **)((longlong)ppuVar38 + -1);
      if (uVar22 == 0x5c) goto LAB_141bf18b5;
LAB_141bf18a4:
      if (uVar22 == 0x2f) goto LAB_141bf18b5;
    } while (ppuVar38 != (undefined **)&DAT_142efd768);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar28 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_141bf2676;
    if (*plVar28 != 0) goto LAB_141bf26b3;
    *plVar28 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar28 = *plVar28 + 1;
  }
  in_stack_00000070[2] = 0;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    if ((extraout_RAX_02[3] != 0x3f) ||
       (iVar23 = memcmp("country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\raw_vec.rs"
                        ,(void *)extraout_RAX_02[2],0x3f), iVar23 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar24 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (lVar24 != 0) {
      lVar24 = in_stack_00000038[1];
      uVar26 = *(ulonglong *)*in_stack_00000038;
      pbVar29 = (byte *)0x0;
      pbVar40 = (byte *)(uVar26 - *(ulonglong *)(lVar24 + 0x18));
      if (uVar26 < *(ulonglong *)(lVar24 + 0x18)) {
        pbVar40 = pbVar29;
      }
      pbVar34 = (byte *)(uVar26 - *(ulonglong *)(lVar24 + 0x38));
      if (uVar26 < *(ulonglong *)(lVar24 + 0x38)) {
        pbVar34 = pbVar29;
      }
      lVar36 = (longlong)pbVar40 * 0x28 + *(longlong *)(lVar24 + 8);
      lVar41 = (longlong)pbVar34 * 0x28 + *(longlong *)(lVar24 + 0x28);
      bVar43 = *(byte **)(lVar24 + 0x10) < pbVar40;
      pbVar40 = *(byte **)(lVar24 + 0x10) + -(longlong)pbVar40;
      bVar45 = pbVar40 == (byte *)0x0;
      if (bVar43) {
        lVar36 = 8;
        pbVar40 = pbVar29;
      }
      pbVar37 = (byte *)(lVar36 + -0x28 + (longlong)pbVar40 * 0x28);
      if (bVar43 || bVar45) {
        pbVar37 = pbVar29;
      }
      bVar44 = *(byte **)(lVar24 + 0x30) < pbVar34;
      pbVar34 = *(byte **)(lVar24 + 0x30) + -(longlong)pbVar34;
      bVar46 = pbVar34 == (byte *)0x0;
      if (bVar44) {
        lVar41 = 8;
        pbVar34 = pbVar29;
      }
      pbVar42 = (byte *)(lVar41 + -0x28 + (longlong)pbVar34 * 0x28);
      if (bVar44 || bVar46) {
        pbVar42 = pbVar37;
      }
      pbVar40 = pbVar34 + (longlong)pbVar40;
      if (!bVar44 && !bVar46 || !bVar43 && !bVar45) {
        pbVar40 = pbVar29;
      }
      *(ulonglong *)*in_stack_00000038 = *(longlong *)(lVar24 + 0x40) - (longlong)pbVar40;
      lVar24 = *in_R8;
      lVar36 = *(longlong *)(lVar24 + 8);
      lVar41 = *(longlong *)(lVar24 + 0x10) * 0x50 + lVar36;
      for (; lVar36 != lVar41; lVar36 = lVar36 + 0x50) {
        country_core::resources::placement_data::PlacementDataSsbo::position();
        if ((65.0 < ABS(local_1f4)) || (65.0 < ABS(local_1ec))) {
          bVar32 = 1;
        }
        else {
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          if ((0.0 < extraout_XMM0_Da_03) || (0.0 < extraout_XMM0_Da_04)) {
            bVar32 = 0;
          }
          else {
            country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz();
            fVar50 = extraout_XMM0_Da + extraout_XMM0_Da_00 + extraout_XMM0_Da_01 +
                     extraout_XMM0_Da_02;
            fVar47 = 0.0;
            if (0.0 <= fVar50) {
              fVar47 = fVar50;
            }
            fVar50 = 1.0;
            if (fVar47 <= 1.0) {
              fVar50 = fVar47;
            }
            bVar32 = 0.0 < fVar50 | extraout_AL_00;
            bVar33 = bVar32 ^ 1;
            if ((((bVar44 || bVar46) && (bVar43 || bVar45)) | bVar32) != 1) {
              if ((*pbVar42 & 1) == 0) {
                fVar47 = (float)*(undefined8 *)(pbVar42 + 4) - local_1f4;
                fVar50 = (float)((ulonglong)*(undefined8 *)(pbVar42 + 4) >> 0x20) - local_1ec;
                fVar47 = fVar50 * fVar50 + fVar47 * fVar47;
                fVar50 = *(float *)(pbVar42 + 0xc) + 7.0;
              }
              else {
                fVar47 = local_1f4;
                ::utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter();
                fVar47 = (fVar47 - local_1ec) * (fVar47 - local_1ec) +
                         (extraout_XMM0_Da_05 - local_1f4) * (extraout_XMM0_Da_05 - local_1f4);
                fVar50 = 7.0;
              }
              bVar33 = fVar50 <= SQRT(fVar47);
            }
            bVar32 = 0;
            if (bVar33 != 0) {
              ::utils::geometry::circle::Circle2d::new();
              country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
              query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env_0>
                        ();
              bVar32 = bVar33;
            }
          }
        }
        uVar12 = *(undefined8 *)(lVar36 + 0x40);
        lVar13 = in_stack_00000070[2];
        if (lVar13 == *in_stack_00000070) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar11 = in_stack_00000070[1];
        *(undefined8 *)(lVar11 + lVar13 * 0x10) = uVar12;
        *(byte *)(lVar11 + 8 + lVar13 * 0x10) = bVar32;
        in_stack_00000070[2] = lVar13 + 1;
      }
      ::utils::rng::init_rng();
      puVar14 = (ulonglong *)in_stack_00000070[1];
      lVar36 = in_stack_00000070[2];
      lVar41 = *in_stack_00000030;
      lVar13 = in_stack_00000030[1];
      uVar6 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar15 = (undefined4 *)in_R8[2];
      uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar11 = *in_stack_00000028;
      puVar16 = (undefined4 *)in_stack_00000028[2];
      uVar26 = 0;
      puVar21 = puVar14;
LAB_141bf1f20:
      while (puVar39 = puVar21, puVar39 != puVar14 + lVar36 * 2) {
        if (*(char *)(lVar24 + 0x18) == '\x01') {
          fastrand::Rng::f32();
          fVar50 = extraout_XMM0_Da_06 * 0.75 + (1.0 - extraout_XMM0_Da_06) * 0.3;
          uVar48 = 0;
          fVar47 = 0.0;
        }
        else {
          if (*(ulonglong *)(lVar24 + 0x10) <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          fVar47 = *(float *)(*(longlong *)(lVar24 + 8) + 0x1c + uVar26 * 0x50);
          fVar50 = 0.0;
          uVar48 = 0x3f800000;
        }
        puVar21 = puVar39 + 2;
        uVar1 = uVar26 + 1;
        if ((char)puVar39[1] == '\0') goto LAB_141bf20b0;
        if (*(int *)((longlong)puVar39 + 4) == 0) {
          bevy_ecs::system::commands::Commands::spawn_empty();
          lVar31 = _DAT_0000004f;
          if (*(ulonglong *)(lVar24 + 0x10) <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar35 = uVar26 * 0x50;
          uVar3 = *(undefined4 *)(*(longlong *)(lVar24 + 8) + 0x1c + lVar35);
          if ((ulonglong)(_DAT_0000003f - _DAT_0000004f) < 0x38) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar19 = _DAT_00000047;
          *(code **)(_DAT_00000047 + lVar31) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::tree_storage::InteractableForestTree>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar19 + 8 + lVar31) = bevy_ecs::error::handler::panic;
          *(undefined8 *)(lVar19 + 0x10 + lVar31) = local_220;
          *(ulonglong *)(lVar19 + 0x18 + lVar31) = uVar26;
          *(undefined4 *)(lVar19 + 0x20 + lVar31) = uVar3;
          *(undefined4 *)(lVar19 + 0x24 + lVar31) = uVar48;
          *(float *)(lVar19 + 0x28 + lVar31) = fVar50;
          *(undefined1 *)(lVar19 + 0x30 + lVar31) = 0;
          _DAT_0000004f = lVar31 + 0x38;
          uVar30 = *(ulonglong *)(lVar24 + 0x10);
          if (uVar30 <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar31 = *(longlong *)(lVar24 + 8);
          fVar50 = *(float *)(lVar31 + 0x10 + lVar35);
          fVar49 = *(float *)(lVar31 + 0x14 + lVar35);
          fVar4 = *(float *)(lVar31 + 0x18 + lVar35);
          if ((((-65.0 <= fVar50) && (fVar50 < 65.0)) && (-65.0 <= fVar4)) && (fVar4 < 65.0)) {
            country_core::resources::terrain_height::TerrainHeightsData::
            sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
            lVar31 = _DAT_0000004f;
            if ((ulonglong)(_DAT_0000003f - _DAT_0000004f) < 0x28) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            }
            lVar19 = _DAT_00000047;
            *(code **)(_DAT_00000047 + lVar31) =
                 core::ops::function::FnOnce::
                 call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAmbience>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
            ;
            *(code **)(lVar19 + 8 + lVar31) = bevy_ecs::error::handler::panic;
            *(undefined8 *)(lVar19 + 0x10 + lVar31) = local_220;
            *(ulonglong *)(lVar19 + 0x18 + lVar31) =
                 CONCAT44(extraout_XMM0_Da_07 + fVar47 * 10.0,fVar50 + fVar47 * 0.0);
            *(float *)(lVar19 + 0x20 + lVar31) = fVar47 * 0.0 + fVar4;
            *(undefined1 *)(lVar19 + 0x24 + lVar31) = 0;
            _DAT_0000004f = lVar31 + 0x28;
            uVar30 = *(ulonglong *)(lVar24 + 0x10);
            fVar49 = extraout_XMM0_Da_07;
          }
          if (uVar30 <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          plVar28 = *(longlong **)(*(longlong *)(lVar24 + 8) + lVar35);
          uVar12 = *(undefined8 *)(*(longlong *)(lVar24 + 8) + 8 + lVar35);
          LOCK();
          lVar31 = *plVar28;
          *plVar28 = *plVar28 + 1;
          UNLOCK();
          if (*plVar28 == 0 || SCARRY8(lVar31,1) != *plVar28 < 0) {
                    // WARNING: Does not return
            pcVar20 = (code *)invalidInstructionException();
            (*pcVar20)();
          }
          ::utils::pack_2x16f_uint();
          if (*(ulonglong *)(lVar24 + 0x10) <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar31 = *(longlong *)(lVar24 + 8);
          uVar3 = *(undefined4 *)(lVar31 + 0x48 + lVar35);
          uVar48 = *(undefined4 *)(lVar31 + 0x20 + lVar35);
          uVar8 = *(undefined4 *)(lVar31 + 0x24 + lVar35);
          uVar9 = *(undefined4 *)(lVar31 + 0x2c + lVar35);
          uVar17 = *(undefined8 *)(lVar31 + 0x34 + lVar35);
          uVar10 = *(undefined4 *)(lVar31 + 0x3c + lVar35);
          *puVar16 = uVar7;
          uVar18 = *(undefined8 *)(lVar11 + 0x40);
          uStack_164 = (undefined4)uVar17;
          uStack_160 = (undefined4)((ulonglong)uVar17 >> 0x20);
          lVar31 = *(longlong *)(lVar11 + 0x30);
          if (lVar31 == *(longlong *)(lVar11 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
          }
          lVar19 = *(longlong *)(lVar11 + 0x28);
          lVar27 = lVar31 * 0x50;
          puVar2 = (undefined8 *)(lVar19 + 0x40 + lVar27);
          *puVar2 = CONCAT44(uVar10,uStack_160);
          puVar2[1] = uVar18;
          puVar2 = (undefined8 *)(lVar19 + 0x30 + lVar27);
          *puVar2 = CONCAT44(uVar9,uVar3);
          puVar2[1] = CONCAT44(uStack_164,extraout_EAX);
          puVar2 = (undefined8 *)(lVar19 + 0x20 + lVar27);
          *puVar2 = CONCAT44(fVar47,fVar4);
          puVar2[1] = CONCAT44(uVar8,uVar48);
          puVar2 = (undefined8 *)(lVar19 + 0x10 + lVar27);
          *puVar2 = local_220;
          puVar2[1] = CONCAT44(fVar49,fVar50);
          *(undefined8 *)(lVar19 + lVar27) = plVar28;
          ((undefined8 *)(lVar19 + lVar27))[1] = uVar12;
          *(longlong *)(lVar11 + 0x30) = lVar31 + 1;
          *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
          *puVar15 = uVar6;
          if (*(ulonglong *)(lVar24 + 0x10) <= uVar26) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          *(undefined8 *)(*(longlong *)(lVar24 + 8) + 0x40 + lVar35) = local_220;
          uVar26 = uVar1;
        }
        else {
          uVar30 = *puVar39;
          if (uVar30 >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          uVar25 = uVar30 & 0xffffffff;
          uVar26 = uVar1;
          if ((uVar25 < *(ulonglong *)(lVar13 + 0x10)) &&
             (*(int *)(*(longlong *)(lVar13 + 8) + uVar25 * 0x14) == (int)(uVar30 >> 0x20))) {
            uVar22 = *(uint *)(*(longlong *)(lVar13 + 8) + uVar25 * 0x14 + 4);
            uVar25 = (ulonglong)uVar22;
            if ((uVar25 != 0xffffffff) &&
               ((uVar25 < *(ulonglong *)(lVar41 + 0x38) &&
                ((*(ulonglong *)(*(longlong *)(lVar41 + 0x28) + (ulonglong)(uVar22 >> 6) * 8) >>
                  (uVar25 & 0x3f) & 1) != 0)))) {
              plVar28 = *(longlong **)(in_R9 + 8);
              if (plVar28 == (longlong *)0x0) {
                plVar28 = *(longlong **)(in_R9 + 0x10);
              }
              lVar31 = plVar28[2];
              if ((ulonglong)(*plVar28 - lVar31) < 0x18) {
                alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              }
              lVar35 = plVar28[1];
              *(code **)(lVar35 + lVar31) =
                   core::ops::function::FnOnce::
                   call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<country_core::components::TreeMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
              ;
              *(code **)(lVar35 + 8 + lVar31) = bevy_ecs::error::handler::warn;
              *(ulonglong *)(lVar35 + 0x10 + lVar31) = uVar30;
              lVar31 = lVar31 + 0x18;
              goto LAB_141bf218b;
            }
          }
        }
      }
      *puVar15 = uVar6;
      *(undefined1 *)(lVar24 + 0x18) = 1;
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_03 == 1) {
          plVar28 = extraout_RAX_03 + 1;
        }
        else if ((*extraout_RAX_03 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar28 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_141bf2676:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        if (*plVar28 != 0) {
LAB_141bf26b3:
                    // WARNING: Subroutine does not return
          core::cell::panic_already_borrowed();
        }
        *plVar28 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar28 = *plVar28 + 1;
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_141bf20b0:
  uVar26 = uVar1;
  if (*(int *)((longlong)puVar39 + 4) != 0) {
    uVar30 = *puVar39;
    if (uVar30 >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar26 = uVar30 & 0xffffffff;
    if ((uVar26 < *(ulonglong *)(lVar13 + 0x10)) &&
       (*(int *)(*(longlong *)(lVar13 + 8) + uVar26 * 0x14) == (int)(uVar30 >> 0x20))) {
      uVar22 = *(uint *)(*(longlong *)(lVar13 + 8) + uVar26 * 0x14 + 4);
      uVar25 = (ulonglong)uVar22;
      if ((uVar25 != 0xffffffff) &&
         ((uVar25 < *(ulonglong *)(lVar41 + 0x38) &&
          (uVar26 = uVar1,
          (*(ulonglong *)(*(longlong *)(lVar41 + 0x28) + (ulonglong)(uVar22 >> 6) * 8) >>
           (uVar25 & 0x3f) & 1) != 0)))) goto LAB_141bf1f20;
    }
    plVar28 = *(longlong **)(in_R9 + 8);
    if (plVar28 == (longlong *)0x0) {
      plVar28 = *(longlong **)(in_R9 + 0x10);
    }
    lVar31 = plVar28[2];
    if ((ulonglong)(*plVar28 - lVar31) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar35 = plVar28[1];
    *(code **)(lVar35 + lVar31) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar35 + 8 + lVar31) = bevy_ecs::error::handler::panic;
    *(ulonglong *)(lVar35 + 0x10 + lVar31) = uVar30;
    *(undefined1 *)(lVar35 + 0x18 + lVar31) = 0;
    lVar31 = lVar31 + 0x20;
LAB_141bf218b:
    plVar28[2] = lVar31;
    uVar26 = uVar1;
  }
  goto LAB_141bf1f20;
}

