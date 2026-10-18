
void __cdecl country_core::resources::stairs::stairs_state::StairsState::remove_node(void)

{
  ulonglong uVar1;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar2;
  undefined8 *in_RCX;
  int in_EDX;
  ulonglong uVar3;
  longlong lVar4;
  
  *in_RCX = 0;
  *(undefined1 *)(in_RCX + 0x2f) = 0;
  uVar1 = in_RCX[7];
  if (uVar1 != 0) {
    if (uVar1 == 1) {
      lVar2 = in_RCX[6];
      if (*(int *)(lVar2 + 0x20) == in_EDX) {
        uVar3 = 0;
LAB_1408fff74:
        lVar2 = *(longlong *)(lVar2 + 0x10 + uVar3 * 0x28);
        if (lVar2 != 0) {
          lVar4 = 0;
          do {
            hashbrown::map::HashMap::
            insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            lVar4 = lVar4 + 8;
          } while (lVar2 << 3 != lVar4);
          goto LAB_1408fffd8;
        }
      }
    }
    else {
      indexmap::map::IndexMap::
      hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                ();
      uVar3 = extraout_RAX;
      indexmap::map::core::IndexMapCore::
      get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                ();
      if (extraout_RAX_00 == 1) {
        if (uVar1 <= uVar3) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar2 = in_RCX[6];
        if (lVar2 != 0) goto LAB_1408fff74;
      }
    }
  }
  hashbrown::map::HashMap::
  insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
LAB_1408fffd8:
  petgraph::graphmap::GraphMap::
  remove_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  return;
}

