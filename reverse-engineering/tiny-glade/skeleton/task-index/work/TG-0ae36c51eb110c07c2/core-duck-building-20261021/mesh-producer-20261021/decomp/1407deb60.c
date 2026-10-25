
void __cdecl system_decorator::decorator_visual::stairs_assemble::create_stairs_collider(void)

{
  longlong *plVar1;
  char *in_R8;
  char local_48;
  longlong *local_30;
  
  if (*in_R8 == '\x11') {
    country_core::systems::collision::world::RaycastWorld::
    remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
    if (local_48 != '\x11') {
      LOCK();
      *local_30 = *local_30 + -1;
      UNLOCK();
      if (*local_30 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
      }
    }
    if (*in_R8 != '\x11') {
      plVar1 = *(longlong **)(in_R8 + 0x18);
      LOCK();
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if (*plVar1 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
        return;
      }
    }
  }
  else {
    country_core::systems::collision::world::RaycastWorld::
    upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform>
              ();
  }
  return;
}

