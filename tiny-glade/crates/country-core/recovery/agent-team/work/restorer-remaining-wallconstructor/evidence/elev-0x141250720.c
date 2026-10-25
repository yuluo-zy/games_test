
// WARNING: Control flow encountered bad instruction data

void __cdecl
system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y
          (void)

{
  undefined1 local_c0;
  
  country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id();
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(local_c0) {
  case 0x12:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x13:
  case 0x14:
  case 0x15:
  case 0x16:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  default:
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::new();
    ::utils::geometry::rectangle::Rectangle2d::as_points2();
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    return;
  }
}

