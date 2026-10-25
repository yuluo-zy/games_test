// 0x1421ac4f0 switch_roof_type
// sig: void __cdecl switch_roof_type(void)
// size: 2392


void __cdecl system_roof::visual::switch_roof_type::switch_roof_type(void)

{
  undefined8 *puVar1;
  char cVar2;
  char cVar3;
  undefined1 uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  longlong lVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 *puVar10;
  undefined **ppuVar11;
  longlong lVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined *puVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  ulonglong **ppuVar18;
  char extraout_AL;
  uint extraout_EAX;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong lVar22;
  uint uVar23;
  longlong in_RDX;
  undefined **ppuVar24;
  undefined **ppuVar25;
  longlong *plVar26;
  ulonglong uVar27;
  undefined4 uVar28;
  undefined8 *in_R8;
  ulonglong uVar29;
  longlong *in_R9;
  int iVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  undefined8 *puVar33;
  ulonglong *in_stack_00000028;
  longlong *in_stack_00000030;
  ulonglong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong local_1d0;
  ulonglong *local_170;
  code *local_168;
  undefined4 *local_160;
  longlong local_150;
  undefined8 local_148;
  undefined4 *local_140;
  longlong local_138;
  undefined **local_130;
  undefined8 local_128;
  undefined **local_120;
  undefined8 local_118;
  undefined4 uStack_110;
  int iStack_10c;
  undefined8 local_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined1 local_d8;
  undefined7 uStack_d7;
  undefined8 uStack_d0;
  undefined4 local_c8;
  undefined4 local_c4;
  ulonglong local_c0;
  undefined **local_b8;
  undefined **local_b0;
  longlong local_a8;
  undefined4 local_9c;
  undefined4 local_98;
  char local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  puVar5 = (ulonglong *)*in_RCX;
  lVar6 = in_RCX[1];
  uVar29 = *puVar5;
  uVar27 = 0;
  uVar21 = uVar29 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar29 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar21 = uVar27;
  }
  uVar31 = uVar29 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar29 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar31 = uVar27;
  }
  lVar19 = uVar21 * 0x20 + *(longlong *)(lVar6 + 8);
  uVar29 = *(ulonglong *)(lVar6 + 0x10);
  uVar32 = uVar29 - uVar21;
  if (uVar29 < uVar21) {
    uVar32 = uVar27;
  }
  if (uVar29 < uVar21) {
    lVar19 = 8;
  }
  lVar22 = uVar31 * 0x20 + *(longlong *)(lVar6 + 0x28);
  uVar29 = *(ulonglong *)(lVar6 + 0x30) - uVar31;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar31) {
    lVar22 = 8;
    uVar29 = uVar27;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar29) - uVar32;
  plVar26 = *(longlong **)(in_RDX + 8);
  if (plVar26 == (longlong *)0x0) {
    plVar26 = *(longlong **)(in_RDX + 0x10);
  }
  local_150 = uVar32 * 0x20 + lVar19;
  local_138 = uVar29 * 0x20 + lVar22;
  lVar6 = *in_stack_00000030;
  lVar7 = in_stack_00000030[1];
  local_128 = *in_stack_00000040;
  puVar8 = (undefined4 *)in_stack_00000040[2];
  local_c4 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  local_148 = *in_R8;
  local_9c = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_b8 = (undefined **)*in_stack_00000038;
  local_160 = (undefined4 *)in_stack_00000038[2];
  local_98 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_b0 = (undefined **)*in_stack_00000028;
  local_140 = (undefined4 *)in_stack_00000028[2];
  local_c8 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar9 = *in_R9;
  puVar10 = (undefined4 *)in_R9[2];
  local_130 = local_b8 + 4;
  local_120 = local_b0 + 4;
  ppuVar25 = (undefined **)(lVar9 + 0x20);
  while( true ) {
    if ((lVar19 == 0) || (lVar19 == local_150)) {
      if ((lVar22 == 0) || (lVar22 == local_138)) {
        return;
      }
      lVar19 = 0;
      lVar20 = lVar22;
      lVar22 = lVar22 + 0x20;
    }
    else {
      lVar20 = lVar19;
      lVar19 = lVar19 + 0x20;
    }
    *puVar5 = *puVar5 + 1;
    local_c0 = *(ulonglong *)(lVar20 + 0x10);
    cVar2 = *(char *)(lVar20 + 0x19);
    uVar29 = local_c0 & 0xffffffff;
    iVar30 = (int)(local_c0 >> 0x20);
    local_91 = cVar2;
    if ((*(ulonglong *)(lVar7 + 0x10) <= uVar29) ||
       (ppuVar11 = *(undefined ***)(lVar7 + 8), ppuVar25 = ppuVar11,
       *(int *)((longlong)ppuVar11 + uVar29 * 0x14) != iVar30)) break;
    uVar23 = *(uint *)((longlong)ppuVar11 + uVar29 * 0x14 + 4);
    ppuVar25 = (undefined **)(ulonglong)uVar23;
    if (ppuVar25 == (undefined **)0xffffffff) break;
    uVar28 = 0;
    if ((*(undefined ***)(lVar6 + 0x38) <= ppuVar25) ||
       ((*(ulonglong *)(*(longlong *)(lVar6 + 0x28) + (ulonglong)(uVar23 >> 6) * 8) >>
         ((ulonglong)ppuVar25 & 0x3f) & 1) == 0)) goto LAB_1421ace38;
    ppuVar24 = *(undefined ***)(lVar20 + 8);
    cVar3 = *(char *)(lVar20 + 0x18);
    uVar21 = (ulonglong)*(uint *)((longlong)ppuVar11 + uVar29 * 0x14 + 0xc);
    puVar33 = (undefined8 *)
              ((ulonglong)*(uint *)((longlong)ppuVar11 + uVar29 * 0x14 + 0x10) * 0x58 +
              *(longlong *)
               (*(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x18 + uVar21 * 0x48) + 0x10 +
               ~*(ulonglong *)
                 (*(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x38 + uVar21 * 0x48) +
                 *(longlong *)(lVar6 + 0x110) * 8) * 0x30));
    country_core::resources::roofs::roof_shape::Roof::ty();
    uVar29 = local_c0;
    local_170 = (ulonglong *)CONCAT71(local_170._1_7_,extraout_AL);
    if (cVar2 != extraout_AL) {
      local_118 = (undefined **)0x0;
                    // WARNING: Subroutine does not return
      core::panicking::
      assert_failed<country_core::resources::roofs::roof_shape::RoofType,country_core::resources::roofs::roof_shape::RoofType>
                ();
    }
    if (cVar2 == '\0') {
      country_core::resources::walls::decorator::DecoratorRank::new();
      country_core::components::drawable::impl$1::from();
      uStack_100 = 0;
      local_108._0_4_ = 0;
      local_108._4_4_ = 0;
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      local_d8 = 9;
      local_118 = (undefined **)0x0;
      uStack_110 = 0;
      iStack_10c = 0x3f800000;
      uStack_fc = 0x3f800000;
      uStack_f8 = 0x3f800000;
      uStack_f4 = 0x3f800000;
      lVar20 = plVar26[2];
      if ((ulonglong)(*plVar26 - lVar20) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar12 = plVar26[1];
      *(code **)(lVar12 + lVar20) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar12 + 8 + lVar20);
      *puVar1 = local_118;
      puVar1[1] = CONCAT44(iStack_10c,uStack_110);
      puVar1 = (undefined8 *)(lVar12 + 0x18 + lVar20);
      *puVar1 = CONCAT44(local_108._4_4_,(undefined4)local_108);
      puVar1[1] = CONCAT44(uStack_fc,uStack_100);
      puVar1 = (undefined8 *)(lVar12 + 0x28 + lVar20);
      *puVar1 = CONCAT44(uStack_f4,uStack_f8);
      puVar1[1] = uStack_f0;
      puVar1 = (undefined8 *)(lVar12 + 0x38 + lVar20);
      *puVar1 = local_e8;
      puVar1[1] = uStack_e0;
      puVar1 = (undefined8 *)(lVar12 + 0x48 + lVar20);
      *puVar1 = CONCAT71(uStack_d7,local_d8);
      puVar1[1] = uStack_d0;
      *(undefined1 *)(lVar12 + 0x58 + lVar20) = 0;
      *(ulonglong *)(lVar12 + 0x68 + lVar20) = uVar29;
      *(code **)(lVar12 + 0x78 + lVar20) = bevy_ecs::error::handler::panic;
      plVar26[2] = lVar20 + 0x88;
      uVar4 = *(undefined1 *)((longlong)puVar33 + 0x54);
      country_core::resources::roofs::roof_shape::Roof::pivot_3d();
      local_118._0_2_ = CONCAT11(uVar4,6);
      *local_160 = local_9c;
      puVar13 = local_b8[6];
      puVar14 = local_b8[8];
      if (puVar13 == local_b8[4]) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      puVar15 = local_b8[5];
      *(undefined **)(puVar15 + (longlong)puVar13 * 0x28) = puVar14;
      *(ulonglong *)(puVar15 + (longlong)puVar13 * 0x28 + 0x14) =
           CONCAT44((undefined4)local_108,iStack_10c);
      *(ulonglong *)((longlong)(puVar15 + (longlong)puVar13 * 0x28 + 0x14) + 8) =
           CONCAT44(uStack_100,local_108._4_4_);
      *(undefined ***)(puVar15 + (longlong)puVar13 * 0x28 + 8) = local_118;
      *(ulonglong *)((longlong)(puVar15 + (longlong)puVar13 * 0x28 + 8) + 8) =
           CONCAT44(iStack_10c,uStack_110);
      local_b8[6] = puVar13 + 1;
      local_b8[8] = local_b8[8] + 1;
      ppuVar25 = local_b8;
      if ((*(float *)((longlong)puVar33 + 0x34) == 1.0) &&
         (!NAN(*(float *)((longlong)puVar33 + 0x34)))) {
        uVar16 = *puVar33;
        *local_140 = local_98;
        puVar13 = local_b0[6];
        puVar14 = local_b0[8];
        if (puVar13 == local_b0[4]) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puVar15 = local_b0[5];
        *(undefined **)(puVar15 + (longlong)puVar13 * 0x10) = puVar14;
        *(undefined8 *)(puVar15 + (longlong)puVar13 * 0x10 + 8) = uVar16;
        local_b0[6] = puVar13 + 1;
        local_b0[8] = local_b0[8] + 1;
        ppuVar25 = local_b0;
      }
      if (cVar3 == '\x01') {
        ppuVar25 = ppuVar24;
        country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped();
        if ((*(float *)(extraout_RAX_00 + 0x54) != 1.6) || (NAN(*(float *)(extraout_RAX_00 + 0x54)))
           ) {
          *puVar8 = local_c4;
          country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height();
          ppuVar25 = ppuVar24;
        }
      }
    }
    else {
      lVar20 = plVar26[2];
      if ((ulonglong)(*plVar26 - lVar20) < 0x18) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar12 = plVar26[1];
      *(code **)(lVar12 + lVar20) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar12 + 8 + lVar20) = bevy_ecs::error::handler::warn;
      *(ulonglong *)(lVar12 + 0x10 + lVar20) = uVar29;
      plVar26[2] = lVar20 + 0x18;
      local_170 = &local_c0;
      local_168 = bevy_ecs::entity::impl$10::fmt;
      local_118 = &PTR_s_roof__142ff7830;
      uStack_110 = 1;
      iStack_10c = 0;
      local_108 = &local_170;
      uStack_100 = 1;
      uStack_fc = 0;
      uStack_f8 = 0;
      uStack_f4 = 0;
      alloc::fmt::format::format_inner();
      local_a8 = local_1d0;
      country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name();
      if (extraout_RAX == 1) {
        _ZN120__LT_bevy_ecs__system__schedule_system__InfallibleSystemWrapper_LT_S_GT__u20_as_u20_bevy_ecs__system__system__System_GT_16component_access17h024209426c3fd862E_llvm_6988214418030372215
                  ();
        country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh();
      }
      if (local_a8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      country_core::resources::roofs::roof_shape::Roof::pivot_3d();
      local_118 = (undefined **)((ulonglong)local_118 & 0xffffffffffffff00);
      *local_160 = local_9c;
      puVar13 = local_b8[6];
      puVar14 = local_b8[8];
      ppuVar18 = local_108;
      if (puVar13 == local_b8[4]) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
        ppuVar18 = local_108;
      }
      uVar29 = local_c0;
      local_108._4_4_ = (undefined4)((ulonglong)ppuVar18 >> 0x20);
      local_108._0_4_ = SUB84(ppuVar18,0);
      puVar15 = local_b8[5];
      *(undefined **)(puVar15 + (longlong)puVar13 * 0x28) = puVar14;
      *(ulonglong *)(puVar15 + (longlong)puVar13 * 0x28 + 0x14) =
           CONCAT44((undefined4)local_108,iStack_10c);
      *(ulonglong *)((longlong)(puVar15 + (longlong)puVar13 * 0x28 + 0x14) + 8) =
           CONCAT44(uStack_100,local_108._4_4_);
      *(undefined ***)(puVar15 + (longlong)puVar13 * 0x28 + 8) = local_118;
      *(ulonglong *)((longlong)(puVar15 + (longlong)puVar13 * 0x28 + 8) + 8) =
           CONCAT44(iStack_10c,uStack_110);
      local_b8[6] = puVar13 + 1;
      local_b8[8] = local_b8[8] + 1;
      local_108 = ppuVar18;
      country_core::resources::walls::decorator::DecoratorRank::new();
      ppuVar25 = (undefined **)(ulonglong)extraout_EAX;
      country_core::components::drawable::impl$1::from();
      uStack_100 = 0;
      local_108._0_4_ = 0;
      local_108._4_4_ = 0;
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      local_d8 = 9;
      local_118 = (undefined **)0x0;
      uStack_110 = 0;
      iStack_10c = 0x3f800000;
      uStack_fc = 0x3f800000;
      uStack_f8 = 0x3f800000;
      uStack_f4 = 0x3f800000;
      ppuVar11 = (undefined **)plVar26[2];
      if ((ulonglong)(*plVar26 - (longlong)ppuVar11) < 0x88) {
        ppuVar25 = ppuVar11;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar20 = plVar26[1];
      *(code **)(lVar20 + (longlong)ppuVar11) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar20 + 8 + (longlong)ppuVar11);
      *puVar1 = local_118;
      puVar1[1] = CONCAT44(iStack_10c,uStack_110);
      puVar1 = (undefined8 *)(lVar20 + 0x18 + (longlong)ppuVar11);
      *puVar1 = CONCAT44(local_108._4_4_,(undefined4)local_108);
      puVar1[1] = CONCAT44(uStack_fc,uStack_100);
      puVar1 = (undefined8 *)(lVar20 + 0x28 + (longlong)ppuVar11);
      *puVar1 = CONCAT44(uStack_f4,uStack_f8);
      puVar1[1] = uStack_f0;
      puVar1 = (undefined8 *)(lVar20 + 0x38 + (longlong)ppuVar11);
      *puVar1 = local_e8;
      puVar1[1] = uStack_e0;
      puVar1 = (undefined8 *)(lVar20 + 0x48 + (longlong)ppuVar11);
      *puVar1 = CONCAT71(uStack_d7,local_d8);
      puVar1[1] = uStack_d0;
      *(undefined1 *)(lVar20 + 0x58 + (longlong)ppuVar11) = 0;
      *(ulonglong *)(lVar20 + 0x68 + (longlong)ppuVar11) = uVar29;
      *(code **)(lVar20 + 0x78 + (longlong)ppuVar11) = bevy_ecs::error::handler::panic;
      plVar26[2] = (longlong)(ppuVar11 + 0x11);
      if ((*(float *)((longlong)puVar33 + 0x34) == 1.0) &&
         (!NAN(*(float *)((longlong)puVar33 + 0x34)))) {
        uVar16 = *puVar33;
        *local_140 = local_98;
        puVar13 = local_b0[6];
        puVar14 = local_b0[8];
        if (puVar13 == local_b0[4]) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puVar15 = local_b0[5];
        *(undefined **)(puVar15 + (longlong)puVar13 * 0x10) = puVar14;
        *(undefined8 *)(puVar15 + (longlong)puVar13 * 0x10 + 8) = uVar16;
        local_b0[6] = puVar13 + 1;
        local_b0[8] = local_b0[8] + 1;
        ppuVar25 = local_b0;
      }
    }
    uVar29 = local_c0;
    ppuVar11 = (undefined **)plVar26[2];
    if ((ulonglong)(*plVar26 - (longlong)ppuVar11) < 0x18) {
      ppuVar25 = ppuVar11;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar20 = plVar26[1];
    *(code **)(lVar20 + (longlong)ppuVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<country_core::asset_libraries::Handle<country_core::render::vao::Vao>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar20 + 8 + (longlong)ppuVar11) = bevy_ecs::error::handler::warn;
    *(ulonglong *)(lVar20 + 0x10 + (longlong)ppuVar11) = uVar29;
    plVar26[2] = (longlong)(ppuVar11 + 3);
    uVar16 = *puVar33;
    *puVar10 = local_c8;
    lVar20 = *(longlong *)(lVar9 + 0x30);
    uVar17 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar20 == *(longlong *)(lVar9 + 0x20)) {
      ppuVar25 = &anon_84ab5937f659760f8f47d75421f37c6b_1_llvm_9510372700382549680;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar12 = *(longlong *)(lVar9 + 0x28);
    *(undefined8 *)(lVar12 + lVar20 * 0x10) = uVar17;
    *(undefined8 *)(lVar12 + 8 + lVar20 * 0x10) = uVar16;
    *(longlong *)(lVar9 + 0x30) = lVar20 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
  }
  uVar23 = (uint)ppuVar25;
  uVar28 = 1;
LAB_1421ace38:
  local_118 = (undefined **)CONCAT44(uVar23,uVar28);
  uStack_110 = (undefined4)local_c0;
  iStack_10c = iVar30;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

