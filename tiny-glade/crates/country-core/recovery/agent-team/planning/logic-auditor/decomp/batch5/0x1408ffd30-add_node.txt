
void __cdecl country_core::resources::stairs::stairs_state::StairsState::add_node(void)

{
  undefined8 *in_RCX;
  
  *in_RCX = 0;
  *(undefined1 *)(in_RCX + 0x2f) = 0;
  petgraph::graphmap::GraphMap::
  add_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  hashbrown::map::HashMap::
  insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  return;
}

