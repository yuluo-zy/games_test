// addr=0x140a92670 name=glade_startup_walls
// signature=void __cdecl glade_startup_walls(void)


void __cdecl country_core::startup::startup_walls::glade_startup_walls(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong *plVar4;
  longlong local_d8;
  
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_d8)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    *extraout_RAX_00 = 0x500000007;
    extraout_RAX_00[1] = 0x400000004;
    extraout_RAX_00[2] = 0x700000006;
    extraout_RAX_00[3] = 0x700000003;
    extraout_RAX_00[4] = 0x600000006;
    extraout_RAX_00[5] = 0x300000002;
    extraout_RAX_00[6] = 0x500000001;
    extraout_RAX_00[7] = 0x700000007;
    extraout_RAX_00[8] = 0x100000003;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_01 != (longlong *)0x0) {
      *extraout_RAX_01 = *extraout_RAX_01 + 1;
      load_mesh_into_library();
      load_shader_into_library<ref_<str_>_>();
      load_shader_into_library<ref_<str_>_>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy,country_core::components::drawable::OcclusionCulled>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_03;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 0;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy,country_core::components::drawable::OcclusionCulled>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_02;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 0;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy,country_core::components::drawable::OcclusionCulled>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_03;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 1;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy,country_core::components::drawable::OcclusionCulled>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_02;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 1;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_02;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 0x100;
      *(undefined1 *)(lVar3 + 0x4a + lVar2) = 0;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_02;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 0x100;
      *(undefined1 *)(lVar3 + 0x4a + lVar2) = 1;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar4 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) {
        plVar4 = (longlong *)&DAT_00000012;
      }
      lVar2 = plVar4[2];
      if ((ulonglong)(*plVar4 - lVar2) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar3 = plVar4[1];
      *(code **)(lVar3 + lVar2) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
      *puVar1 = 0;
      puVar1[1] = 0x3f80000000000000;
      *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
      *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
      *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
      *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
      *(undefined8 *)(lVar3 + 0x38 + lVar2) = extraout_RAX_02;
      *(undefined8 *)(lVar3 + 0x40 + lVar2) = extraout_RAX_04;
      *(undefined2 *)(lVar3 + 0x48 + lVar2) = 0x100;
      *(undefined1 *)(lVar3 + 0x4a + lVar2) = 2;
      *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
      *(longlong *)(lVar3 + 0x68 + lVar2) = local_d8;
      *(code **)(lVar3 + 0x78 + lVar2) = bevy_ecs::error::handler::panic;
      plVar4[2] = lVar2 + 0x88;
      return;
    }
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


