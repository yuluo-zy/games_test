
void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_collider(void)

{
  byte extraout_AL;
  undefined1 *extraout_RAX;
  
  petgraph::graphmap::GraphMap::
  add_edge<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  if (((extraout_AL != 2 ^ extraout_AL) & 1) != 0) {
    indexmap::map::IndexMap::
    get_mut<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::resources::stairs::structural_graph::StructuralGraphConnection,std::hash::random::RandomState,tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>_>
              ();
    if (extraout_RAX == (undefined1 *)0x0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    *extraout_RAX = 0;
  }
  return;
}

