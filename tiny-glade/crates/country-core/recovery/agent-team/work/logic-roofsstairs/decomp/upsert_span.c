
void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_span(void)

{
  undefined8 *puVar1;
  char cVar2;
  char extraout_AL;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  undefined8 *puVar3;
  undefined8 *in_R8;
  undefined8 local_b8;
  undefined4 local_b0;
  undefined8 local_ac;
  char local_a4;
  undefined2 local_a3;
  undefined1 local_a1;
  undefined8 local_a0;
  undefined4 local_98;
  undefined8 local_90;
  undefined4 local_88;
  undefined8 local_80;
  char local_78;
  undefined2 local_77;
  undefined1 local_75;
  undefined8 local_70;
  undefined4 local_68;
  longlong local_60;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
            ();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::SubworldColliderHandle>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::SubworldColliderHandle>_>
            ();
  if (local_a4 != '\x11') {
    local_a0 = local_ac;
    local_98 = CONCAT31(CONCAT12(local_a1,local_a3),local_a4);
    local_60 = in_RCX + 0xc0;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
              ();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
              ();
    local_b0 = local_98;
    local_b8 = local_a0;
    puVar3 = &local_b8;
    petgraph::graphmap::GraphMap::
    neighbors<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    cVar2 = local_78;
    for (puVar1 = extraout_RAX;
        (puVar1 != puVar3 && (local_78 = *(char *)(puVar1 + 1), local_78 != '\x11'));
        puVar1 = puVar1 + 2) {
      local_80 = *puVar1;
      local_77 = *(undefined2 *)((longlong)puVar1 + 9);
      local_75 = *(undefined1 *)((longlong)puVar1 + 0xb);
      hashbrown::map::HashMap::
      insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      cVar2 = local_78;
    }
    local_b8 = local_a0;
    local_78 = cVar2;
    petgraph::graphmap::GraphMap::
    remove_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    if (extraout_AL == '\0') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
              ();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::systems::collision::world::SubworldColliderHandle,f32>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle,f32>_>
              ();
  }
  petgraph::graphmap::GraphMap::
  add_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  local_68 = *(undefined4 *)(in_R8 + 1);
  local_70 = *in_R8;
  local_88 = *(undefined4 *)(in_RDX + 1);
  local_90 = *in_RDX;
  hashbrown::map::HashMap::
  insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  local_88 = *(undefined4 *)(in_RDX + 1);
  local_90 = *in_RDX;
  local_b0 = *(undefined4 *)(in_R8 + 1);
  local_b8 = *in_R8;
  hashbrown::map::HashMap::
  insert<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::SubworldColliderHandle,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  local_b0 = *(undefined4 *)(in_R8 + 1);
  local_b8 = *in_R8;
  hashbrown::map::HashMap::
  insert<country_core::systems::collision::world::SubworldColliderHandle,f32,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  hashbrown::map::HashMap::
  insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  return;
}

