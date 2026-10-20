// name: closure$0
// addr: 0x140782f50
// size: 360
// signature: void __cdecl closure$0(void)


void __cdecl system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_0(void)

{
  char *pcVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  char *extraout_RAX_01;
  longlong extraout_RAX_02;
  char *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  float local_38;
  
  country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info();
  if (((((extraout_RAX == 0) || ((byte)(*(char *)(extraout_RAX + 0x22) - 2U) < 4)) ||
       (country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage(),
       extraout_RAX_00 != 0)) ||
      ((country_core::resources::walls::public_walls::PublicWalls::get(), extraout_RAX_04 == 0 ||
       (country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        position(), *(float *)(extraout_RAX_04 + 0x274) <= local_38 + -0.4)))) &&
     ((indexmap::map::IndexMap::
       get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                 (), extraout_RAX_01 == (char *)0x0 || (*extraout_RAX_01 != '\0')))) {
    indexmap::map::IndexMap::
    get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    if (extraout_RAX_02 == 0) {
      lVar4 = 4;
      lVar2 = 4;
    }
    else {
      lVar2 = *(longlong *)(extraout_RAX_02 + 8);
      lVar4 = lVar2 + *(longlong *)(extraout_RAX_02 + 0x10) * 8;
    }
    do {
      do {
        if (lVar2 == lVar4) {
          return;
        }
        lVar3 = lVar2 + 8;
        pcVar1 = (char *)(lVar2 + 4);
        lVar2 = lVar3;
      } while (*pcVar1 != '\0');
      indexmap::map::IndexMap::
      get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      if (extraout_RAX_03 == (char *)0x0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
    } while (*extraout_RAX_03 != '\0');
  }
  return;
}

