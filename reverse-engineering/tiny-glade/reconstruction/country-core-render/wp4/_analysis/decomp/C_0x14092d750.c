// ADDRESS: 0x14092d750
// SIZE: 285
// SIGNATURE: void __cdecl remove(void)

void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::remove(void)

{
  longlong extraout_RAX;
  longlong in_RCX;
  longlong lVar1;
  bool bVar2;
  
  if (*(char *)(in_RCX + 0x28) != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar1 = extraout_RAX;
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>_>_>
            ();
  if (lVar1 != 0) {
    core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  LOCK();
  bVar2 = *(longlong *)(in_RCX + 0x60) == 0;
  if (bVar2) {
    *(longlong *)(in_RCX + 0x60) = 8;
  }
  UNLOCK();
  if (!bVar2) {
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  hashbrown::map::HashMap::
  retain<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global,country_core::asset_libraries::mesh_library::impl_1::remove::closure_env_0>
            ();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar2 = *(longlong *)(in_RCX + 0x60) == 8;
  if (bVar2) {
    *(longlong *)(in_RCX + 0x60) = 0;
  }
  UNLOCK();
  if (!bVar2) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  return;
}

