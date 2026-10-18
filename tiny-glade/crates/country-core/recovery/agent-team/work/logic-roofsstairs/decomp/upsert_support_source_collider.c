
void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::
upsert_support_source_collider(void)

{
  petgraph::graphmap::GraphMap::
  add_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  hashbrown::map::HashMap::
  insert<country_core::systems::collision::world::SubworldColliderHandle,f32,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  hashbrown::map::HashMap::
  insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  return;
}

