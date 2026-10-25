void __cdecl add(void)


void __cdecl country_core::resources::prefab::PrefabLibrary::add(void)

{
  undefined8 local_60;
  undefined8 local_50;
  undefined8 local_40;
  
  hashbrown::map::HashMap::
  insert<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (!SBORROW8(0,local_60)) {
    local_40 = 0;
    while (local_50 != local_40) {
      local_40 = local_40 + 1;
      core::ptr::drop_in_place<country_core::resources::prefab::PrefabPart>();
    }
    if (local_60 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
  }
  return;
}

