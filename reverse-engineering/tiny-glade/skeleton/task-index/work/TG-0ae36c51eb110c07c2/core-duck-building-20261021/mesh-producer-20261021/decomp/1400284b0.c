
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_8<true>
          (void)

{
  char cVar1;
  uint *puVar2;
  longlong lVar3;
  longlong lVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong in_RCX;
  ulonglong uVar5;
  ulonglong uVar6;
  
  puVar2 = *(uint **)(in_RCX + 0x10);
  uVar5 = (ulonglong)puVar2[1];
  country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage();
  uVar6 = (ulonglong)*puVar2;
  country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage();
  if ((extraout_RAX == extraout_RAX_00) && ((extraout_RAX != 0 || (uVar5 == uVar6)))) {
    lVar3 = *(longlong *)(in_RCX + 0x18);
    uVar5 = *(ulonglong *)(lVar3 + 0x58);
    if (uVar5 == 0) {
      return;
    }
    if (uVar5 == 1) {
      lVar4 = *(longlong *)(lVar3 + 0x50);
      if (*(uint *)(lVar4 + 8) != *puVar2) {
        return;
      }
      if (*(uint *)(lVar4 + 0xc) != puVar2[1]) {
        return;
      }
      cVar1 = *(char *)(lVar4 + 0x10);
    }
    else {
      indexmap::map::IndexMap::
      hash<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      uVar6 = extraout_RAX_01;
      indexmap::map::core::IndexMapCore::
      get_index_of<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      if (extraout_RAX_02 != 1) {
        return;
      }
      if (uVar5 <= uVar6) goto LAB_1400286b8;
      cVar1 = *(char *)(*(longlong *)(lVar3 + 0x50) + 0x10 + uVar6 * 0x28);
    }
    if (((cVar1 == '\x01') && (uVar5 = *(ulonglong *)(lVar3 + 0x58), uVar5 != 0)) && (uVar5 != 1)) {
      indexmap::map::IndexMap::
      hash<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      uVar6 = extraout_RAX_03;
      indexmap::map::core::IndexMapCore::
      get_index_of<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      if ((extraout_RAX_04 == 1) && (uVar5 <= uVar6)) {
LAB_1400286b8:
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
    }
  }
                    // WARNING: Read-only address (ram,0x000142925930) is written
  return;
}

