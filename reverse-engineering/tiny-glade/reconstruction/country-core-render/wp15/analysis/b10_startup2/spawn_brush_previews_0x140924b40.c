// addr=0x140924b40 name=spawn_brush_previews
// signature=void __cdecl spawn_brush_previews(void)


void __cdecl country_core::startup::spawn_brush_previews::spawn_brush_previews(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  longlong *extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  ulonglong extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  ulonglong extraout_RAX_11;
  undefined8 extraout_RAX_12;
  ulonglong extraout_RAX_13;
  undefined8 *in_RDX;
  char *pcVar8;
  longlong *plVar9;
  longlong *plVar10;
  undefined8 *in_R8;
  undefined8 *in_R9;
  longlong lVar11;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 local_280;
  undefined8 local_278;
  undefined8 local_270;
  undefined8 local_268;
  undefined8 local_260;
  undefined *local_258;
  undefined8 uStack_250;
  undefined8 **local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 *local_1d8;
  code *local_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  longlong *local_1a0 [24];
  longlong *plStack_e0;
  undefined8 **local_d8;
  undefined8 uStack_d0;
  undefined *local_c8;
  undefined8 uStack_c0;
  undefined *local_b8;
  undefined8 uStack_b0;
  undefined *local_a8;
  undefined8 uStack_a0;
  undefined *local_98;
  undefined8 uStack_90;
  undefined *local_88;
  undefined8 uStack_80;
  undefined *local_78;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  uVar2 = *in_R9;
  load_shader_into_library<ref_<str_>_>();
  local_1c8 = extraout_RAX;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f8000003f800000;
  uStack_250._0_4_ = 0x3f800000;
  country_core::render::mesh::Mesh::add_color_self();
  uVar3 = *in_R8;
  uVar4 = *in_RDX;
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f8000003f800000;
  uStack_250._0_4_ = 0x3f800000;
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  local_288 = extraout_RAX_01;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f000000;
  uStack_250._0_4_ = 0;
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  local_1a0[0] = extraout_RAX_03;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f8000003f800000;
  uStack_250._0_4_ = 0x3f800000;
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f8000003f800000;
  uStack_250._0_4_ = 0x3f800000;
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,(longlong)local_1a0[1])) {
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_78 = (undefined *)local_1a0[0xf];
  local_88 = (undefined *)local_1a0[0xd];
  uStack_80 = local_1a0[0xe];
  local_98 = (undefined *)local_1a0[0xb];
  uStack_90 = local_1a0[0xc];
  local_a8 = (undefined *)local_1a0[9];
  uStack_a0 = local_1a0[10];
  local_b8 = (undefined *)local_1a0[7];
  uStack_b0 = local_1a0[8];
  local_c8 = (undefined *)local_1a0[5];
  uStack_c0 = local_1a0[6];
  local_d8 = (undefined8 **)local_1a0[3];
  uStack_d0 = local_1a0[4];
  local_1a0[0x17] = local_1a0[1];
  plStack_e0 = local_1a0[2];
  local_258 = (undefined *)0x3f8000003f800000;
  uStack_250._0_4_ = 0x3f800000;
  local_280 = extraout_RAX_05;
  local_270 = extraout_RAX_04;
  local_268 = extraout_RAX_02;
  local_260 = extraout_RAX_00;
  local_1b8 = uVar2;
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  local_278 = extraout_RAX_06;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (!SBORROW8(0,(longlong)local_1a0[1])) {
    local_78 = (undefined *)local_1a0[0xf];
    local_88 = (undefined *)local_1a0[0xd];
    uStack_80 = local_1a0[0xe];
    local_98 = (undefined *)local_1a0[0xb];
    uStack_90 = local_1a0[0xc];
    local_a8 = (undefined *)local_1a0[9];
    uStack_a0 = local_1a0[10];
    local_b8 = (undefined *)local_1a0[7];
    uStack_b0 = local_1a0[8];
    local_c8 = (undefined *)local_1a0[5];
    uStack_c0 = local_1a0[6];
    local_d8 = (undefined8 **)local_1a0[3];
    uStack_d0 = local_1a0[4];
    local_1a0[0x17] = local_1a0[1];
    plStack_e0 = local_1a0[2];
    local_258 = (undefined *)0x3f8000003f800000;
    uStack_250 = CONCAT44(uStack_250._4_4_,0x3f800000);
    country_core::render::mesh::Mesh::add_color_self();
    local_1b0 = uVar3;
    local_1a8 = uVar4;
    load_mesh_into_library();
    local_1a0[1] = (longlong *)&DAT_142aaeccc;
    local_1a0[2] = (longlong *)&DAT_0000001a;
    local_1a0[3] = &DAT_142aaece6;
    local_1a0[4] = (longlong *)0x1e;
    local_1a0[5] = (longlong *)&DAT_142aaed04;
    local_1a0[6] = (longlong *)0x18;
    local_1a0[7] = (longlong *)&DAT_142aaed1c;
    local_1a0[8] = (longlong *)0xa;
    local_1a0[9] = (longlong *)&DAT_142aaed26;
    local_1a0[10] = (longlong *)0x12;
    local_1a0[0xb] = (longlong *)&DAT_142aaed38;
    local_1a0[0xc] = (longlong *)0xc;
    local_1a0[0xd] = (longlong *)&DAT_142aaed44;
    local_1a0[0xe] = (longlong *)0x14;
    local_1a0[0xf] = (longlong *)&DAT_142aaed58;
    local_1a0[0x10] = (longlong *)0x1d;
    local_1a0[0x11] = (longlong *)&DAT_142aaed75;
    local_1a0[0x12] = (longlong *)0xa;
    local_1a0[0x13] = (longlong *)&DAT_142aaed7f;
    local_1a0[0x14] = (longlong *)0x12;
    local_1a0[0x15] = (longlong *)0x0;
    local_1a0[0x16] = (longlong *)0xa;
    lVar11 = 8;
    local_1c0 = extraout_RAX_07;
    do {
      local_298 = *(undefined8 *)((longlong)local_1a0 + lVar11);
      uStack_290 = *(undefined8 *)((longlong)local_1a0 + lVar11 + 8);
      local_1d8 = &local_298;
      local_1d0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_1a0[0x17] = (longlong *)&DAT_142aaeef0;
      plStack_e0 = (longlong *)0x2;
      uStack_d0 = 1;
      local_c8 = (undefined *)0x0;
      local_d8 = &local_1d8;
      alloc::fmt::format::format_inner();
      country_core::utils::load_json::load_json_as_mesh<alloc::string::String>();
      if (SBORROW8(0,(longlong)local_1a0[0x17])) {
        local_258 = (undefined *)plStack_e0;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_1e8 = local_78;
      local_1f8 = local_88;
      uStack_1f0 = uStack_80;
      local_208 = local_98;
      uStack_200 = uStack_90;
      local_218 = local_a8;
      uStack_210 = uStack_a0;
      local_228 = local_b8;
      uStack_220 = uStack_b0;
      local_238 = local_c8;
      uStack_230 = uStack_c0;
      local_248 = local_d8;
      uStack_240 = uStack_d0;
      local_258 = (undefined *)local_1a0[0x17];
      uStack_250 = plStack_e0;
      local_1d8 = (undefined8 *)0x3f8000003f800000;
      local_1d0 = (code *)CONCAT44(local_1d0._4_4_,0x3f800000);
      country_core::render::mesh::Mesh::add_color_self();
      load_mesh_into_library();
      lVar11 = lVar11 + 0x10;
    } while (lVar11 != 0xa8);
    pcVar8 = "vertex_color_shader";
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_08 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::systems::brush_preview::BrushPreview,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar1 = (undefined8 *)(lVar5 + 8 + lVar11);
    *puVar1 = 0;
    puVar1[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = 0;
    *(undefined4 *)(lVar5 + 0x20 + lVar11) = 0;
    *(undefined8 *)(lVar5 + 0x24 + lVar11) = 0x3f8000003f800000;
    *(undefined4 *)(lVar5 + 0x2c + lVar11) = 0x3f800000;
    *(undefined8 *)(lVar5 + 0x38 + lVar11) = local_260;
    *(char **)(lVar5 + 0x40 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x48 + lVar11) = 1;
    *(undefined1 *)(lVar5 + 0x58 + lVar11) = 0;
    *(longlong **)(lVar5 + 0x68 + lVar11) = plVar6;
    *(code **)(lVar5 + 0x78 + lVar11) = bevy_ecs::error::handler::panic;
    plVar9[2] = lVar11 + 0x88;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::FollowMouse,country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_288;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 0x300;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_288;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 4;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_288;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 5;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::drawable::DontCastShadow,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_268;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 6;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::drawable::DontCastShadow,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(longlong **)(lVar5 + 0x18 + lVar11) = local_1a0[0];
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 7;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::drawable::DontCastShadow,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(longlong **)(lVar5 + 0x18 + lVar11) = local_1a0[0];
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 8;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::drawable::DontCastShadow,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(longlong **)(lVar5 + 0x18 + lVar11) = local_1a0[0];
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 9;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::drawable::DontCastShadow,country_core::components::FollowMouse>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(longlong **)(lVar5 + 0x18 + lVar11) = local_1a0[0];
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 10;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::look_at_camera::LookAtCamera,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_270;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined1 *)(lVar5 + 0x28 + lVar11) = 2;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::history::signifier_replay_active::SignifierReplayIndex,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_1c0;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined8 *)(lVar5 + 0x28 + lVar11) = 1;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0xd;
    *(undefined1 *)(lVar5 + 0x38 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x40;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::systems::history::signifier_replay_active::SignifierReplayIndex,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_1c0;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined8 *)(lVar5 + 0x28 + lVar11) = 2;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0xd;
    *(undefined1 *)(lVar5 + 0x38 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x40;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_280;
    *(undefined8 *)(lVar5 + 0x20 + lVar11) = local_1c8;
    *(undefined8 *)(lVar5 + 0x28 + lVar11) = 0;
    *(undefined2 *)(lVar5 + 0x30 + lVar11) = 0xb01;
    *(undefined1 *)(lVar5 + 0x38 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x40;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::brush_preview::BrushPreview,country_core::components::ArrowGizmo,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = local_278;
    *(undefined8 *)(lVar5 + 0x20 + lVar11) = local_1c8;
    *(undefined8 *)(lVar5 + 0x28 + lVar11) = 0;
    *(undefined2 *)(lVar5 + 0x30 + lVar11) = 0xc01;
    *(undefined1 *)(lVar5 + 0x38 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x40;
    geometry::sphere::impl$2::try_from();
    if (SBORROW8(0,(longlong)local_1a0[1])) {
      local_d8 = (undefined8 **)local_1a0[4];
      local_1a0[0x17] = local_1a0[2];
      plStack_e0 = local_1a0[3];
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_78 = (undefined *)local_1a0[0xf];
    local_88 = (undefined *)local_1a0[0xd];
    uStack_80 = local_1a0[0xe];
    local_98 = (undefined *)local_1a0[0xb];
    uStack_90 = local_1a0[0xc];
    local_a8 = (undefined *)local_1a0[9];
    uStack_a0 = local_1a0[10];
    local_b8 = (undefined *)local_1a0[7];
    uStack_b0 = local_1a0[8];
    local_c8 = (undefined *)local_1a0[5];
    uStack_c0 = local_1a0[6];
    local_d8 = (undefined8 **)local_1a0[3];
    uStack_d0 = local_1a0[4];
    local_1a0[0x17] = local_1a0[1];
    plStack_e0 = local_1a0[2];
    local_258 = (undefined *)0x3f8000003f800000;
    uStack_250 = CONCAT44(uStack_250._4_4_,0x3f800000);
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,(longlong)local_1a0[1])) {
      local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_78 = (undefined *)local_1a0[0xf];
    local_88 = (undefined *)local_1a0[0xd];
    uStack_80 = local_1a0[0xe];
    local_98 = (undefined *)local_1a0[0xb];
    uStack_90 = local_1a0[0xc];
    local_a8 = (undefined *)local_1a0[9];
    uStack_a0 = local_1a0[10];
    local_b8 = (undefined *)local_1a0[7];
    uStack_b0 = local_1a0[8];
    local_c8 = (undefined *)local_1a0[5];
    uStack_c0 = local_1a0[6];
    local_d8 = (undefined8 **)local_1a0[3];
    uStack_d0 = local_1a0[4];
    local_1a0[0x17] = local_1a0[1];
    plStack_e0 = local_1a0[2];
    load_mesh_into_library();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = local_1a0[5];
    plVar9 = local_1a0[2];
    if (local_1a0[2] == (longlong *)0x0) {
      plVar9 = local_1a0[3];
    }
    lVar11 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar11) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar5 = plVar9[1];
    *(code **)(lVar5 + lVar11) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::look_at_camera::LookAtCamera,country_core::components::FollowMouse,country_core::components::drawable::UiPass,country_core::systems::brush_preview::BrushPreview>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar5 + 8 + lVar11) = bevy_ecs::error::handler::panic;
    *(longlong **)(lVar5 + 0x10 + lVar11) = plVar6;
    *(undefined8 *)(lVar5 + 0x18 + lVar11) = extraout_RAX_09;
    *(char **)(lVar5 + 0x20 + lVar11) = pcVar8;
    *(undefined2 *)(lVar5 + 0x28 + lVar11) = 0xe00;
    *(undefined1 *)(lVar5 + 0x30 + lVar11) = 0;
    plVar9[2] = lVar11 + 0x38;
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (!SBORROW8(0,(longlong)local_1a0[1])) {
      local_78 = (undefined *)local_1a0[0xf];
      local_88 = (undefined *)local_1a0[0xd];
      uStack_80 = local_1a0[0xe];
      local_98 = (undefined *)local_1a0[0xb];
      uStack_90 = local_1a0[0xc];
      local_a8 = (undefined *)local_1a0[9];
      uStack_a0 = local_1a0[10];
      local_b8 = (undefined *)local_1a0[7];
      uStack_b0 = local_1a0[8];
      local_c8 = (undefined *)local_1a0[5];
      uStack_c0 = local_1a0[6];
      local_d8 = (undefined8 **)local_1a0[3];
      uStack_d0 = local_1a0[4];
      local_1a0[0x17] = local_1a0[1];
      plStack_e0 = local_1a0[2];
      load_mesh_into_library();
      pcVar8 = "vertex_color_shader";
      asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
      if ((extraout_RAX_11 & 1) == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      ::utils::components::transform::impl$1::default();
      plVar9 = local_1a0[2];
      plVar6 = local_1a0[1];
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar7 = local_1a0[5];
      plVar10 = local_1a0[2];
      if (local_1a0[2] == (longlong *)0x0) {
        plVar10 = local_1a0[3];
      }
      lVar11 = plVar10[2];
      if ((ulonglong)(*plVar10 - lVar11) < 0x78) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar5 = plVar10[1];
      *(code **)(lVar5 + lVar11) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::SnappingToEndPreview,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar5 + 8 + lVar11);
      *puVar1 = plVar6;
      puVar1[1] = plVar9;
      *(undefined8 *)(lVar5 + 0x18 + lVar11) = 0xbf00000080000000;
      *(undefined4 *)(lVar5 + 0x20 + lVar11) = 0x80000000;
      *(undefined8 *)(lVar5 + 0x24 + lVar11) = 0;
      *(undefined4 *)(lVar5 + 0x2c + lVar11) = 0;
      *(undefined8 *)(lVar5 + 0x38 + lVar11) = extraout_RAX_10;
      *(char **)(lVar5 + 0x40 + lVar11) = pcVar8;
      *(undefined1 *)(lVar5 + 0x48 + lVar11) = 0;
      *(longlong **)(lVar5 + 0x58 + lVar11) = plVar7;
      *(code **)(lVar5 + 0x68 + lVar11) = bevy_ecs::error::handler::panic;
      plVar10[2] = lVar11 + 0x78;
      country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
      if (!SBORROW8(0,(longlong)local_1a0[1])) {
        local_78 = (undefined *)local_1a0[0xf];
        local_88 = (undefined *)local_1a0[0xd];
        uStack_80 = local_1a0[0xe];
        local_98 = (undefined *)local_1a0[0xb];
        uStack_90 = local_1a0[0xc];
        local_a8 = (undefined *)local_1a0[9];
        uStack_a0 = local_1a0[10];
        local_b8 = (undefined *)local_1a0[7];
        uStack_b0 = local_1a0[8];
        local_c8 = (undefined *)local_1a0[5];
        uStack_c0 = local_1a0[6];
        local_d8 = (undefined8 **)local_1a0[3];
        uStack_d0 = local_1a0[4];
        local_1a0[0x17] = local_1a0[1];
        plStack_e0 = local_1a0[2];
        load_mesh_into_library();
        pcVar8 = "vertex_color_shader";
        asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
        if ((extraout_RAX_13 & 1) != 0) {
          ::utils::components::transform::impl$1::default();
          plVar9 = local_1a0[2];
          plVar6 = local_1a0[1];
          bevy_ecs::system::commands::Commands::spawn_empty();
          plVar7 = local_1a0[5];
          plVar10 = local_1a0[2];
          if (local_1a0[2] == (longlong *)0x0) {
            plVar10 = local_1a0[3];
          }
          lVar11 = plVar10[2];
          if ((ulonglong)(*plVar10 - lVar11) < 0x78) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar5 = plVar10[1];
          *(code **)(lVar5 + lVar11) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::SnappingToSegmentPreview,country_core::components::drawable::UiPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          puVar1 = (undefined8 *)(lVar5 + 8 + lVar11);
          *puVar1 = plVar6;
          puVar1[1] = plVar9;
          *(undefined8 *)(lVar5 + 0x18 + lVar11) = 0xbf00000080000000;
          *(undefined4 *)(lVar5 + 0x20 + lVar11) = 0x80000000;
          *(undefined8 *)(lVar5 + 0x24 + lVar11) = 0;
          *(undefined4 *)(lVar5 + 0x2c + lVar11) = 0;
          *(undefined8 *)(lVar5 + 0x38 + lVar11) = extraout_RAX_12;
          *(char **)(lVar5 + 0x40 + lVar11) = pcVar8;
          *(undefined1 *)(lVar5 + 0x48 + lVar11) = 0;
          *(longlong **)(lVar5 + 0x58 + lVar11) = plVar7;
          *(code **)(lVar5 + 0x68 + lVar11) = bevy_ecs::error::handler::panic;
          plVar10[2] = lVar11 + 0x78;
          return;
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_1a0[0x17] = local_1a0[2];
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


