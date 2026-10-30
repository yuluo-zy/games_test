
void __cdecl
bevy_ecs::schedule::config::impl$4::
into_configs<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnCircleRotated>),tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnCircleRotated>)>_>
          (void)

{
  void *_Dst;
  undefined8 local_1b8 [11];
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined4 local_d0;
  undefined8 local_c8;
  undefined *local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined4 local_38;
  undefined4 local_30;
  undefined2 local_2c;
  undefined4 local_28;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_140 = 0;
  uStack_138 = 0;
  local_128 = 0;
  uStack_120 = 0;
  local_110 = 0;
  uStack_108 = 0;
  local_f8 = 0;
  uStack_f0 = 0;
  local_e0 = 0;
  uStack_d8 = 0;
  local_d0 = 0;
  local_a8 = 0;
  uStack_a0 = 0;
  local_90 = 0;
  uStack_88 = 0;
  local_78 = 0;
  uStack_70 = 0;
  local_60 = 0;
  uStack_58 = 0;
  local_48 = 0;
  uStack_40 = 0;
  local_38 = 0;
  local_1b8[0] = 0;
  local_160 = 0;
  local_158 = 8;
  local_150 = 0;
  local_148 = 0x10;
  local_130 = 0x10;
  local_118 = 0x10;
  local_100 = 0x10;
  local_e8 = 0x10;
  local_c8 = 0x8000000000000000;
  local_c0 = &anon_b67fbe2b328f070066eafd86f11bd576_45_llvm_12057283020190492742;
  local_b8 = 0x32;
  local_b0 = 0x10;
  local_98 = 0x10;
  local_80 = 0x10;
  local_68 = 0x10;
  local_50 = 0x10;
  local_30 = 0;
  local_2c = 1;
  local_28 = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,local_1b8,0x198);
    config::impl$0::into_config();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

