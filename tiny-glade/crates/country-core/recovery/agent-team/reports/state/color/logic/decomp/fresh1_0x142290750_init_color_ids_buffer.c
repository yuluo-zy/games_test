// name=init_color_ids_buffer
// sig=void __cdecl init_color_ids_buffer(void)
// size=198
// address=0x142290750

void __cdecl system_color::set_wall_color_id::init_color_ids_buffer(void)

{
  country_core::render::impl$0::deref();
  country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
  add_new<country_core::resources::color_id::WallColorIdSsbo>();
  country_core::render::impl$0::deref();
  country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
  add_new<country_core::resources::color_id_roof::RoofColorIdSsbo>();
  country_core::resources::color_id::WallColorIds::new();
  bevy_ecs::system::commands::Commands::
  insert_resource<country_core::resources::color_id::WallColorIds>();
  country_core::render::impl$0::deref();
  country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
  add_new<country_core::resources::color_id::GateColorIdSsbo>();
  country_core::resources::color_id::GateColorIds::new();
  bevy_ecs::system::commands::Commands::
  insert_resource<country_core::resources::color_id::GateColorIds>();
  country_core::render::impl$0::deref();
  country_core::asset_libraries::ssbo_library::AssetSsboLibrary::
  add_new<country_core::resources::color_id::HalftimberColorIdSsbo>();
  country_core::resources::color_id::HalftimberColorIds::new();
  bevy_ecs::system::commands::Commands::
  insert_resource<country_core::resources::color_id::HalftimberColorIds>();
  return;
}


