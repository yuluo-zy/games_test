
void __cdecl
country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub_impl(void)

{
  ulonglong uVar1;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RCX;
  int in_EDX;
  ulonglong uVar2;
  int in_R8D;
  
  if (in_EDX == in_R8D) {
    uVar1 = *(ulonglong *)(in_RCX + 0x10);
    if ((uVar1 != 1) && (uVar1 != 0)) {
      indexmap::map::IndexMap::
      hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                ();
      indexmap::map::core::IndexMapCore::
      get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                ();
      if (extraout_RAX == 1) {
        indexmap::map::IndexMap::
        hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                  ();
        uVar2 = extraout_RAX_00;
        indexmap::map::core::IndexMapCore::
        get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                  ();
        if ((extraout_RAX_01 == 1) && (uVar1 <= uVar2)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
      }
    }
  }
  else if ((*(longlong *)(in_RCX + 0x58) != 0) && (*(longlong *)(in_RCX + 0x58) != 1)) {
    indexmap::map::IndexMap::
    hash<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
              ();
    indexmap::map::core::IndexMapCore::
    get_index_of<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
              ();
  }
  return;
}

