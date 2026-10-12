
void __cdecl
bevy_ecs::system::commands::Commands::
spawn<tuple_<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>_>_>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  undefined8 uVar3;
  undefined8 *in_RCX;
  longlong lVar4;
  longlong *plVar5;
  void *in_R8;
  undefined1 local_318 [336];
  undefined1 local_1c8 [288];
  undefined1 local_a8;
  undefined8 local_98;
  code *local_88;
  undefined8 local_70;
  longlong *plStack_68;
  longlong *local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  void *local_48;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_39 = 1;
  local_48 = in_R8;
  spawn_empty();
  uVar3 = local_50;
  plVar5 = local_60;
  plVar2 = plStack_68;
  if (plStack_68 == (longlong *)0x0) {
    memcpy(local_1c8,local_48,0x120);
    local_a8 = 0;
    local_98 = uVar3;
    local_88 = bevy_ecs::error::handler::panic;
    lVar4 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar4) < 0x158) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  else {
    memcpy(local_1c8,local_48,0x120);
    local_a8 = 0;
    local_98 = uVar3;
    local_88 = bevy_ecs::error::handler::panic;
    lVar4 = plVar2[2];
    plVar5 = plVar2;
    if ((ulonglong)(*plVar2 - lVar4) < 0x158) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
  }
  lVar1 = plVar5[1];
  memcpy(local_318,local_1c8,0x150);
  *(code **)(lVar1 + lVar4) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>_>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  memcpy((void *)(lVar1 + lVar4 + 8),local_318,0x150);
  plVar5[2] = lVar4 + 0x158;
  in_RCX[4] = local_50;
  in_RCX[2] = local_60;
  in_RCX[3] = uStack_58;
  *in_RCX = local_70;
  in_RCX[1] = plStack_68;
  return;
}

