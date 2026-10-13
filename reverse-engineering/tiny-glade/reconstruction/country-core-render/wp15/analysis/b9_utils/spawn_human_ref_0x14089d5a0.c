// addr=0x14089d5a0 name=spawn_human_ref
// signature=void __cdecl spawn_human_ref(void)


void __cdecl country_core::startup::spawn_human_ref::spawn_human_ref(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined *puVar3;
  longlong *local_98;
  longlong *local_90;
  undefined8 local_80;
  
  geometry::cube::impl$5::from();
  load_mesh_into_library();
  puVar3 = &anon_b7cdbc01553587cd8295e3d0308c6d7c_176_llvm_5445834821627626614;
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_00 & 1) != 0) {
    bevy_ecs::system::commands::Commands::spawn_empty();
    if (local_98 == (longlong *)0x0) {
      local_98 = local_90;
    }
    lVar1 = local_98[2];
    if ((ulonglong)(*local_98 - lVar1) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar2 = local_98[1];
    *(code **)(lVar2 + lVar1) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,country_core::systems::debug::human_scale_ref::HumanScaleRef>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar2 + 8 + lVar1) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar2 + 0x10 + lVar1) = local_80;
    *(undefined8 *)(lVar2 + 0x18 + lVar1) = extraout_RAX;
    *(undefined **)(lVar2 + 0x20 + lVar1) = puVar3;
    *(undefined8 *)(lVar2 + 0x28 + lVar1) = 0x3fe666663f000000;
    *(undefined4 *)(lVar2 + 0x30 + lVar1) = 0x3f000000;
    *(undefined1 *)(lVar2 + 0x38 + lVar1) = 0;
    local_98[2] = lVar1 + 0x40;
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


