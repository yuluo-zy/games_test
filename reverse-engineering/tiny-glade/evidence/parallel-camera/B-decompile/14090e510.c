
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::MainCamera::init_walk_camera(void)

{
  resources::terrain_height::TerrainHeightsData::
  sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
  ::utils::swizzlers::impl$1::xvy();
  collision::world::Rapier::raycast_simple();
  ::utils::swizzlers::impl$0::x0y();
  walk::WalkCamera::teleport();
  return;
}

