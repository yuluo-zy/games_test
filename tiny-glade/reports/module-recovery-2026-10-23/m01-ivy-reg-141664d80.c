
void __cdecl
bevy_ecs::schedule::config::IntoScheduleConfigs::
after_ignore_deferred<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)>_>,tuple_<>,country_core::systems::GameSystemSet>
          (void)

{
  void *_Dst;
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0 [9];
  undefined8 local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined4 local_d8;
  undefined8 local_d0;
  undefined *local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined4 local_40;
  undefined4 local_38;
  undefined2 local_34;
  undefined4 local_30;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  local_148 = 0;
  uStack_140 = 0;
  local_130 = 0;
  uStack_128 = 0;
  local_118 = 0;
  uStack_110 = 0;
  local_100 = 0;
  uStack_f8 = 0;
  local_e8 = 0;
  uStack_e0 = 0;
  local_d8 = 0;
  local_b0 = 0;
  uStack_a8 = 0;
  local_98 = 0;
  uStack_90 = 0;
  local_80 = 0;
  uStack_78 = 0;
  local_68 = 0;
  uStack_60 = 0;
  local_50 = 0;
  uStack_48 = 0;
  local_40 = 0;
  local_1b0[0] = 0;
  local_168 = 0;
  local_160 = 8;
  local_158 = 0;
  local_150 = 0x10;
  local_138 = 0x10;
  local_120 = 0x10;
  local_108 = 0x10;
  local_f0 = 0x10;
  local_d0 = 0x8000000000000000;
  local_c8 = &anon_b67fbe2b328f070066eafd86f11bd576_47_llvm_12057283020190492742;
  local_c0 = 0x34;
  local_b8 = 0x10;
  local_a0 = 0x10;
  local_88 = 0x10;
  local_70 = 0x10;
  local_58 = 0x10;
  local_38 = 0;
  local_34 = 1;
  local_30 = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,local_1b0,0x188);
    config::impl$0::into_config();
    local_1b0[0] = CONCAT71(local_1b0[0]._1_7_,in_DL);
    intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
    ScheduleConfigs::
    after_ignore_deferred_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
              ();
    in_RCX[0xc] = local_1c0;
    in_RCX[0xd] = uStack_1b8;
    in_RCX[10] = local_1d0;
    in_RCX[0xb] = uStack_1c8;
    in_RCX[8] = local_1e0;
    in_RCX[9] = uStack_1d8;
    in_RCX[6] = local_1f0;
    in_RCX[7] = uStack_1e8;
    in_RCX[4] = local_200;
    in_RCX[5] = uStack_1f8;
    in_RCX[2] = local_210;
    in_RCX[3] = uStack_208;
    *in_RCX = local_220;
    in_RCX[1] = uStack_218;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

