
void __cdecl country_core::resources::logo_animation::startup_animated_logo(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong *plVar8;
  undefined8 *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 *in_RDX;
  longlong *plVar9;
  undefined8 *in_R8;
  undefined8 *in_stack_00000028;
  longlong *local_240 [18];
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  longlong local_198;
  longlong *plStack_190;
  longlong *local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  longlong local_118;
  longlong *plStack_110;
  longlong *local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_90;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_118)) {
    local_240[0] = plStack_110;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_128 = local_a8;
  local_138 = local_b8;
  uStack_130 = uStack_b0;
  local_148 = local_c8;
  uStack_140 = uStack_c0;
  local_158 = local_d8;
  uStack_150 = uStack_d0;
  local_168 = local_e8;
  uStack_160 = uStack_e0;
  local_178 = local_f8;
  uStack_170 = uStack_f0;
  local_188 = local_108;
  uStack_180 = uStack_100;
  local_198 = local_118;
  plStack_190 = plStack_110;
  uVar3 = *in_R8;
  uVar4 = *in_RDX;
  local_81 = 1;
  asset_libraries::meshwatch::MeshWatch::watch();
  local_a8 = local_128;
  local_b8 = local_138;
  uStack_b0 = uStack_130;
  local_c8 = local_148;
  uStack_c0 = uStack_140;
  local_d8 = local_158;
  uStack_d0 = uStack_150;
  local_e8 = local_168;
  uStack_e0 = uStack_160;
  local_f8 = local_178;
  uStack_f0 = uStack_170;
  local_108 = local_188;
  uStack_100 = uStack_180;
  local_118 = local_198;
  plStack_110 = plStack_190;
  local_a0 = 0x8000000000000000;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *(undefined8 *)((longlong)extraout_RAX + 7) = 0x6f676f6c5f656461;
    *extraout_RAX = 0x616c675f796e6974;
    local_a0 = 0xf;
    local_90 = 0xf;
    memcpy(local_240,&local_118,0x90);
    local_81 = 0;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    local_1a0 = *in_stack_00000028;
    local_1b0 = uVar3;
    local_1a8 = uVar4;
    startup::load_shader_into_library<ref_<str_>_>();
    ::utils::components::transform::impl$1::default();
    plVar8 = plStack_110;
    lVar7 = local_118;
    bevy_ecs::system::commands::Commands::spawn_empty();
    uVar3 = local_f8;
    plVar9 = plStack_110;
    if (plStack_110 == (longlong *)0x0) {
      plVar9 = local_108;
    }
    lVar5 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar5) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar6 = plVar9[1];
    *(code **)(lVar6 + lVar5) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar1 = (longlong *)(lVar6 + 8 + lVar5);
    *plVar1 = lVar7;
    plVar1[1] = (longlong)plVar8;
    puVar2 = (undefined8 *)(lVar6 + 0x18 + lVar5);
    *puVar2 = 0x40c0000000000000;
    puVar2[1] = 0x40a0000000000000;
    *(undefined8 *)(lVar6 + 0x28 + lVar5) = 0x40a0000040a00000;
    *(undefined8 *)(lVar6 + 0x38 + lVar5) = extraout_RAX_00;
    *(undefined8 *)(lVar6 + 0x40 + lVar5) = extraout_RAX_01;
    *(undefined4 *)(lVar6 + 0x48 + lVar5) = 0x12;
    *(undefined1 *)(lVar6 + 0x58 + lVar5) = 0;
    *(undefined8 *)(lVar6 + 0x68 + lVar5) = uVar3;
    *(code **)(lVar6 + 0x78 + lVar5) = bevy_ecs::error::handler::panic;
    plVar9[2] = lVar5 + 0x88;
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (!SBORROW8(0,local_118)) {
      local_128 = local_a8;
      local_138 = local_b8;
      uStack_130 = uStack_b0;
      local_148 = local_c8;
      uStack_140 = uStack_c0;
      local_158 = local_d8;
      uStack_150 = uStack_d0;
      local_168 = local_e8;
      uStack_160 = uStack_e0;
      local_178 = local_f8;
      uStack_170 = uStack_f0;
      local_188 = local_108;
      uStack_180 = uStack_100;
      local_198 = local_118;
      plStack_190 = plStack_110;
      local_82 = 1;
      asset_libraries::meshwatch::MeshWatch::watch();
      local_a8 = local_128;
      local_b8 = local_138;
      uStack_b0 = uStack_130;
      local_c8 = local_148;
      uStack_c0 = uStack_140;
      local_d8 = local_158;
      uStack_d0 = uStack_150;
      local_e8 = local_168;
      uStack_e0 = uStack_160;
      local_f8 = local_178;
      uStack_f0 = uStack_170;
      local_108 = local_188;
      uStack_100 = uStack_180;
      local_118 = local_198;
      plStack_110 = plStack_190;
      local_a0 = 0x8000000000000000;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 != (undefined8 *)0x0) {
        *(undefined8 *)((longlong)extraout_RAX_02 + 5) = 0x736568636e617262;
        *extraout_RAX_02 = 0x6172625f6f676f6c;
        local_a0 = 0xd;
        local_90 = 0xd;
        memcpy(local_240,&local_118,0x90);
        local_82 = 0;
        asset_libraries::mesh_library::AssetMeshLibrary::add();
        startup::load_shader_into_library<ref_<str_>_>();
        ::utils::components::transform::impl$1::default();
        plVar8 = plStack_110;
        lVar7 = local_118;
        bevy_ecs::system::commands::Commands::spawn_empty();
        uVar3 = local_f8;
        plVar9 = plStack_110;
        if (plStack_110 == (longlong *)0x0) {
          plVar9 = local_108;
        }
        lVar5 = plVar9[2];
        if ((ulonglong)(*plVar9 - lVar5) < 0x88) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar6 = plVar9[1];
        *(code **)(lVar6 + lVar5) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar1 = (longlong *)(lVar6 + 8 + lVar5);
        *plVar1 = lVar7;
        plVar1[1] = (longlong)plVar8;
        puVar2 = (undefined8 *)(lVar6 + 0x18 + lVar5);
        *puVar2 = 0x40c0000000000000;
        puVar2[1] = 0x40a0000000000000;
        *(undefined8 *)(lVar6 + 0x28 + lVar5) = 0x40a0000040a00000;
        *(undefined8 *)(lVar6 + 0x38 + lVar5) = extraout_RAX_03;
        *(undefined8 *)(lVar6 + 0x40 + lVar5) = extraout_RAX_04;
        *(undefined4 *)(lVar6 + 0x48 + lVar5) = 0x12;
        *(undefined1 *)(lVar6 + 0x58 + lVar5) = 0;
        *(undefined8 *)(lVar6 + 0x68 + lVar5) = uVar3;
        *(code **)(lVar6 + 0x78 + lVar5) = bevy_ecs::error::handler::panic;
        plVar9[2] = lVar5 + 0x88;
        return;
      }
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_240[0] = plStack_110;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

