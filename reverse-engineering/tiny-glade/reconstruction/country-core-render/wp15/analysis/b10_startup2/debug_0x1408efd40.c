// addr=0x1408efd40 name=debug
// signature=void __cdecl debug(void)


void __cdecl country_core::startup::debug::debug(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  char *pcVar5;
  longlong *plVar6;
  longlong local_108;
  longlong *plStack_100;
  longlong *local_f8;
  undefined8 local_e8;
  
  load_shader_into_library<ref_<str_>_>();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar6 = plStack_100;
  if (plStack_100 == (longlong *)0x0) {
    plVar6 = local_f8;
  }
  lVar3 = plVar6[2];
  if ((ulonglong)(*plVar6 - lVar3) < 0x78) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plVar6[1];
  *(code **)(lVar4 + lVar3) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::KeepOnLevelUnload,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
  *puVar1 = 0;
  puVar1[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
  *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
  *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
  *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
  *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX_00;
  *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX;
  *(undefined1 *)(lVar4 + 0x48 + lVar3) = 0;
  *(undefined8 *)(lVar4 + 0x58 + lVar3) = local_e8;
  *(code **)(lVar4 + 0x68 + lVar3) = bevy_ecs::error::handler::panic;
  plVar6[2] = lVar3 + 0x78;
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  country_core::render::mesh::Mesh::add_color_self();
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (!SBORROW8(0,local_108)) {
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (!SBORROW8(0,local_108)) {
      load_mesh_into_library();
      geometry::cube::impl$5::from();
      load_mesh_into_library();
      geometry::cube::impl$5::from();
      country_core::render::mesh::Mesh::add_color_self();
      load_mesh_into_library();
      country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
      if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      load_mesh_into_library();
      pcVar5 = "solid_vertex_color";
      asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
      if ((extraout_RAX_02 & 1) != 0) {
        ::utils::components::transform::impl$1::default();
        bevy_ecs::system::commands::Commands::spawn_empty();
        plVar6 = plStack_100;
        if (plStack_100 == (longlong *)0x0) {
          plVar6 = local_f8;
        }
        lVar3 = plVar6[2];
        if ((ulonglong)(*plVar6 - lVar3) < 0x78) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar4 = plVar6[1];
        *(code **)(lVar4 + lVar3) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::TestCube>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar2 = (longlong *)(lVar4 + 8 + lVar3);
        *plVar2 = local_108;
        plVar2[1] = (longlong)plStack_100;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0xbf00000080000000;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0x80000000;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX_01;
        *(char **)(lVar4 + 0x40 + lVar3) = pcVar5;
        *(undefined1 *)(lVar4 + 0x48 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x58 + lVar3) = local_e8;
        *(code **)(lVar4 + 0x68 + lVar3) = bevy_ecs::error::handler::panic;
        plVar6[2] = lVar3 + 0x78;
        country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
        if (!SBORROW8(0,local_108)) {
          load_mesh_into_library();
          load_shader_into_library<ref_<str_>_>();
          bevy_ecs::system::commands::Commands::spawn_empty();
          if (plStack_100 == (longlong *)0x0) {
            plStack_100 = local_f8;
          }
          lVar3 = plStack_100[2];
          if ((ulonglong)(*plStack_100 - lVar3) < 0x78) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar4 = plStack_100[1];
          *(code **)(lVar4 + lVar3) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
          *puVar1 = 0;
          puVar1[1] = 0x3f80000000000000;
          *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
          *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
          *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
          *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
          *(undefined8 *)(lVar4 + 0x38 + lVar3) = extraout_RAX_03;
          *(undefined8 *)(lVar4 + 0x40 + lVar3) = extraout_RAX_04;
          *(undefined1 *)(lVar4 + 0x48 + lVar3) = 0;
          *(undefined8 *)(lVar4 + 0x58 + lVar3) = local_e8;
          *(code **)(lVar4 + 0x68 + lVar3) = bevy_ecs::error::handler::panic;
          plStack_100[2] = lVar3 + 0x78;
          load_shader_into_library<ref_<str_>_>();
          geometry::sphere::impl$2::try_from();
          if (!SBORROW8(0,local_108)) {
            country_core::render::mesh::Mesh::add_color_self();
            load_mesh_into_library();
            geometry::sphere::impl$2::try_from();
            if (!SBORROW8(0,local_108)) {
              country_core::render::mesh::Mesh::add_color_self();
              load_mesh_into_library();
              geometry::sphere::impl$2::try_from();
              if (!SBORROW8(0,local_108)) {
                country_core::render::mesh::Mesh::add_color_self();
                load_mesh_into_library();
                geometry::sphere::impl$2::try_from();
                if (!SBORROW8(0,local_108)) {
                  country_core::render::mesh::Mesh::add_color_self();
                  load_mesh_into_library();
                  geometry::sphere::impl$2::try_from();
                  if (!SBORROW8(0,local_108)) {
                    country_core::render::mesh::Mesh::add_color_self();
                    load_mesh_into_library();
                    return;
                  }
                }
              }
            }
          }
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


