
void __cdecl country_core::resources::stairs::stairs_state::StairsState::disconnect_edge(void)

{
  char extraout_AL;
  char extraout_AL_00;
  char cVar1;
  undefined8 *in_RCX;
  int in_EDX;
  int in_R8D;
  
  *in_RCX = 0;
  if (in_EDX == in_R8D) {
    petgraph::graphmap::GraphMap::
    remove_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    cVar1 = extraout_AL;
  }
  else {
    petgraph::graphmap::GraphMap::
    remove_edge<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
              ();
    cVar1 = extraout_AL_00;
  }
  if (cVar1 != '\0') {
    hashbrown::map::HashMap::
    insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    *(undefined1 *)(in_RCX + 0x2f) = 0;
  }
  return;
}

