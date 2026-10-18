
void __cdecl country_core::resources::stairs::stairs_state::StairsState::apply_diff(void)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  ulonglong uVar4;
  char extraout_AL;
  char extraout_AL_00;
  char cVar5;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong lVar6;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar7;
  ulonglong uVar8;
  int *piVar9;
  char in_R8B;
  int *piVar10;
  
  compute_checksum();
  lVar6 = in_RDX + 0x20;
  if (in_R8B != '\0') {
    lVar6 = in_RDX + 0x38;
  }
  lVar7 = 0x38;
  if (in_R8B != '\0') {
    lVar7 = 0x20;
  }
  if (*(longlong *)(lVar6 + 0x10) != 0) {
    piVar9 = *(int **)(lVar6 + 8);
    piVar1 = piVar9 + *(longlong *)(lVar6 + 0x10) * 2;
    piVar10 = piVar9;
    do {
      iVar2 = *piVar10;
      iVar3 = piVar10[1];
      *in_RCX = 0;
      if (iVar2 == iVar3) {
        petgraph::graphmap::GraphMap::
        remove_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                  ();
        cVar5 = extraout_AL;
      }
      else {
        petgraph::graphmap::GraphMap::
        remove_edge<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                  ();
        cVar5 = extraout_AL_00;
      }
      if (cVar5 != '\0') {
        hashbrown::map::HashMap::
        insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        *(undefined1 *)(in_RCX + 0x2f) = 0;
      }
      piVar10 = piVar10 + 2;
    } while (piVar10 != piVar1);
    do {
      iVar2 = piVar9[1];
      uVar4 = in_RCX[7];
      if (uVar4 == 0) {
LAB_140900d20:
        petgraph::graphmap::GraphMap::
        remove_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                  ();
      }
      else {
        if (uVar4 != 1) {
          indexmap::map::IndexMap::
          hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          uVar8 = extraout_RAX;
          indexmap::map::core::IndexMapCore::
          get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_00 == 1) {
            if (uVar4 <= uVar8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar6 = in_RCX[6];
            if (lVar6 != 0) goto LAB_140900d06;
          }
          goto LAB_140900d20;
        }
        lVar6 = in_RCX[6];
        if (*(int *)(lVar6 + 0x20) != *piVar9) goto LAB_140900d20;
        uVar8 = 0;
LAB_140900d06:
        if (*(longlong *)(lVar6 + 0x10 + uVar8 * 0x28) == 0) goto LAB_140900d20;
      }
      uVar4 = in_RCX[7];
      if (uVar4 == 0) {
LAB_140900c90:
        petgraph::graphmap::GraphMap::
        remove_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                  ();
      }
      else {
        if (uVar4 != 1) {
          indexmap::map::IndexMap::
          hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          uVar8 = extraout_RAX_01;
          indexmap::map::core::IndexMapCore::
          get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_02 == 1) {
            if (uVar4 <= uVar8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar6 = in_RCX[6];
            if (lVar6 != 0) goto LAB_140900da0;
          }
          goto LAB_140900c90;
        }
        lVar6 = in_RCX[6];
        if (*(int *)(lVar6 + 0x20) != iVar2) goto LAB_140900c90;
        uVar8 = 0;
LAB_140900da0:
        if (*(longlong *)(lVar6 + 0x10 + uVar8 * 0x28) == 0) goto LAB_140900c90;
      }
      piVar9 = piVar9 + 2;
    } while (piVar9 != piVar1);
  }
  lVar6 = *(longlong *)(in_RDX + 0x10 + lVar7);
  if (lVar6 != 0) {
    piVar9 = *(int **)(in_RDX + 8 + lVar7);
    piVar1 = piVar9 + lVar6 * 2;
    do {
      while (*piVar9 == piVar9[1]) {
        *in_RCX = 0;
        *(undefined1 *)(in_RCX + 0x2f) = 0;
        petgraph::graphmap::GraphMap::
        add_node<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                  ();
        hashbrown::map::HashMap::
        insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        piVar9 = piVar9 + 2;
        if (piVar9 == piVar1) goto LAB_140900e2d;
      }
      add_edge();
      piVar9 = piVar9 + 2;
    } while (piVar9 != piVar1);
  }
LAB_140900e2d:
  *in_RCX = 0;
  compute_checksum();
  return;
}

