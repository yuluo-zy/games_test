
void __cdecl country_core::systems::collision::world::RaycastWorld::subscribe_to_changes(void)

{
  tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>();
  return;
}

