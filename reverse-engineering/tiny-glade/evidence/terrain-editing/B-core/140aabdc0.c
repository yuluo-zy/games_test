
void __cdecl
country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::
on_terrain_modified(void)

{
  longlong *in_RCX;
  
  LOCK();
  *in_RCX = *in_RCX + 1;
  UNLOCK();
  return;
}

