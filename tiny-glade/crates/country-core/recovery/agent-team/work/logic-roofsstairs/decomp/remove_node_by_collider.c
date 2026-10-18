
void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::remove_node_by_collider
          (void)

{
  undefined8 *puVar1;
  char cVar2;
  char extraout_AL;
  undefined8 *extraout_RAX;
  undefined8 *in_RDX;
  undefined8 *puVar3;
  undefined8 local_70;
  undefined4 local_68;
  undefined8 local_64;
  undefined4 local_5c;
  undefined8 local_58;
  char local_50;
  undefined2 local_4f;
  undefined1 local_4d;
  undefined8 local_48;
  undefined4 local_40;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
            ();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::systems::collision::world::SubworldColliderHandle,f32>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle,f32>_>
            ();
  if ((char)local_68 != '\x11') {
    local_68 = *(undefined4 *)(in_RDX + 1);
    local_70 = *in_RDX;
    puVar3 = &local_70;
    petgraph::graphmap::GraphMap::
    neighbors<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    cVar2 = local_50;
    for (puVar1 = extraout_RAX;
        (puVar1 != puVar3 && (local_50 = *(char *)(puVar1 + 1), local_50 != '\x11'));
        puVar1 = puVar1 + 2) {
      local_58 = *puVar1;
      local_4f = *(undefined2 *)((longlong)puVar1 + 9);
      local_4d = *(undefined1 *)((longlong)puVar1 + 0xb);
      hashbrown::map::HashMap::
      insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      cVar2 = local_50;
    }
    local_68 = *(undefined4 *)(in_RDX + 1);
    local_70 = *in_RDX;
    local_50 = cVar2;
    petgraph::graphmap::GraphMap::
    remove_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    if (extraout_AL == '\0') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
            ();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
            ();
  if ((char)local_68 != '\x11') {
    local_40 = local_5c;
    local_48 = local_64;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
              ();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::SubworldColliderHandle>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::SubworldColliderHandle>_>
              ();
    if ((char)local_5c == '\x11') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
  }
  return;
}

