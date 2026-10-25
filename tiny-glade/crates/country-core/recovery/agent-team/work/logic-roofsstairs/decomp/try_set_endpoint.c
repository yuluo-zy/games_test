
void __cdecl country_core::resources::stairs::stairs_state::StairsState::try_set_endpoint(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar3;
  longlong in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar4;
  
  uVar1 = *(ulonglong *)(in_RCX + 0x38);
  if (uVar1 == 0) goto LAB_1408fe846;
  if (uVar1 == 1) {
    lVar3 = *(longlong *)(in_RCX + 0x30);
    if (*(int *)(lVar3 + 0x20) != *(int *)(in_RDX + 2)) goto LAB_1408fe846;
    uVar4 = 0;
  }
  else {
    indexmap::map::IndexMap::
    hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    uVar4 = extraout_RAX;
    indexmap::map::core::IndexMapCore::
    get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
              ();
    if (extraout_RAX_00 != 1) goto LAB_1408fe846;
    if (uVar1 <= uVar4) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    lVar3 = *(longlong *)(in_RCX + 0x30);
    if (lVar3 == 0) goto LAB_1408fe846;
  }
  lVar3 = *(longlong *)(lVar3 + 0x10 + uVar4 * 0x28);
  if ((lVar3 != 0) && (7 < lVar3 * 8 - 8U)) {
    *(undefined8 *)(in_RCX + 0x10) = 3;
    return;
  }
LAB_1408fe846:
  *(undefined8 *)(in_RCX + 0x20) = in_RDX[2];
  uVar2 = in_RDX[1];
  *(undefined8 *)(in_RCX + 0x10) = *in_RDX;
  *(undefined8 *)(in_RCX + 0x18) = uVar2;
  return;
}

