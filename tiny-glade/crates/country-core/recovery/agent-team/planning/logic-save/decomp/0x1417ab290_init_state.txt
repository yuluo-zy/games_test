
void __cdecl country_core::systems::save::snapshot::_::impl$0::init_state(void)

{
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  undefined8 extraout_RAX_12;
  undefined8 extraout_RAX_13;
  undefined8 extraout_RAX_14;
  undefined8 extraout_RAX_15;
  undefined8 extraout_RAX_16;
  undefined8 extraout_RAX_17;
  undefined8 extraout_RAX_18;
  undefined8 *in_RCX;
  
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::PathRaster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::Cobblestone1Raster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::Cobblestone2Raster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::Cobblestone3Raster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::WaterRaster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::world_raster_defs::GardenRaster>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::walls::decorator_storage::DecoratorStorage>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::walls::decorator::DecoratorIdGen>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::terrain_editing::TerrainEditingData>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::roofs::commands::CreateRoofCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::color_id::SetWallColorIdCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::color_id::SetGateColorIdCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::color_id::SetHalftimberColorIdCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<bevy_ecs::event::collections::Events<country_core::resources::history::tree_edit::InsertTreeCmd>_>
            ();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::systems::ui_clutter::DeletedAutoClutter>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::stairs::stairs_state::StairsState>();
  bevy_ecs::system::system_param::impl$12::
  init_state<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
  *in_RCX = extraout_RAX;
  in_RCX[1] = extraout_RAX_00;
  in_RCX[2] = extraout_RAX_01;
  in_RCX[3] = extraout_RAX_02;
  in_RCX[4] = extraout_RAX_03;
  in_RCX[5] = extraout_RAX_04;
  in_RCX[6] = extraout_RAX_05;
  in_RCX[7] = extraout_RAX_06;
  in_RCX[8] = extraout_RAX_07;
  in_RCX[9] = extraout_RAX_08;
  in_RCX[10] = extraout_RAX_09;
  in_RCX[0xb] = extraout_RAX_10;
  in_RCX[0xc] = extraout_RAX_11;
  in_RCX[0xd] = extraout_RAX_12;
  in_RCX[0xe] = extraout_RAX_13;
  in_RCX[0xf] = extraout_RAX_14;
  in_RCX[0x10] = extraout_RAX_15;
  in_RCX[0x11] = extraout_RAX_16;
  in_RCX[0x12] = extraout_RAX_17;
  in_RCX[0x13] = extraout_RAX_18;
  return;
}

