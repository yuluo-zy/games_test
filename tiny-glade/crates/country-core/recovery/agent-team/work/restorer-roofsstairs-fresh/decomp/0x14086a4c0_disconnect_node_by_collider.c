
// WARNING: Removing unreachable block (ram,0x00014086a68c)

void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::
disconnect_node_by_collider(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 *in_RDX;
  longlong lVar3;
  char cStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  char *pcStack_c8;
  longlong local_58;
  
  local_58 = 0;
  petgraph::graphmap::GraphMap::
  edges<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  while (petgraph::graphmap::impl$11::
         next<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                   (), cStack_d8 != '\x11') {
    if (*pcStack_c8 != '\0') {
      hashbrown::map::HashMap::
      insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      uVar1 = *(undefined4 *)(in_RDX + 1);
      uVar2 = *in_RDX;
      if (local_58 == 0) {
        alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
      }
      *(ulonglong *)(local_58 * 0x18 + 0x14) = CONCAT44(uStack_cc,uStack_d0);
      *(undefined8 *)(local_58 * 0x18 + 4) = uVar2;
      *(ulonglong *)(local_58 * 0x18 + 0xc) = CONCAT44(uStack_d4,uVar1);
      local_58 = local_58 + 1;
    }
  }
  if (local_58 != 0) {
    lVar3 = 0;
    do {
      if (*(char *)(lVar3 + 0xc) == '\x11') {
        return;
      }
      petgraph::graphmap::GraphMap::
      remove_edge<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                ();
      lVar3 = lVar3 + 0x18;
    } while (local_58 * 0x18 != lVar3);
  }
  return;
}

