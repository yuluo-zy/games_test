
void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::add(void)

{
  void *_Src;
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  void *extraout_RAX;
  void *_Dst;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RCX;
  undefined8 *in_RDX;
  size_t _Size;
  bool bVar16;
  bool bVar17;
  
  LOCK();
  HANDLE_COUNT = HANDLE_COUNT + 1;
  UNLOCK();
  if (SBORROW8(0,in_RDX[3])) {
    _Size = 0x8000000000000000;
  }
  else {
    _Size = in_RDX[5];
    if ((longlong)_Size < 0) {
LAB_14092cfff:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    _Src = (void *)in_RDX[4];
    if (_Size == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (void *)0x0) goto LAB_14092cfff;
    }
    memcpy(_Dst,_Src,_Size);
  }
  uVar1 = in_RDX[0xe];
  uVar2 = in_RDX[0xc];
  uVar3 = in_RDX[0xd];
  uVar4 = in_RDX[10];
  uVar5 = in_RDX[0xb];
  uVar6 = in_RDX[8];
  uVar7 = in_RDX[9];
  uVar8 = *in_RDX;
  uVar9 = in_RDX[1];
  uVar10 = in_RDX[2];
  uVar11 = in_RDX[3];
  uVar12 = in_RDX[4];
  uVar13 = in_RDX[5];
  uVar14 = in_RDX[6];
  uVar15 = in_RDX[7];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  extraout_RAX_00[0xe] = uVar1;
  extraout_RAX_00[0xc] = uVar2;
  extraout_RAX_00[0xd] = uVar3;
  extraout_RAX_00[10] = uVar4;
  extraout_RAX_00[0xb] = uVar5;
  extraout_RAX_00[8] = uVar6;
  extraout_RAX_00[9] = uVar7;
  extraout_RAX_00[6] = uVar14;
  extraout_RAX_00[7] = uVar15;
  extraout_RAX_00[4] = uVar12;
  extraout_RAX_00[5] = uVar13;
  extraout_RAX_00[2] = uVar10;
  extraout_RAX_00[3] = uVar11;
  *extraout_RAX_00 = uVar8;
  extraout_RAX_00[1] = uVar9;
  frozen::FrozenMap::
  insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>
            ();
  if (extraout_RAX_01 != 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  bVar16 = true;
  if (!SBORROW8(0,in_RDX[0xf])) {
    LOCK();
    bVar16 = *(longlong *)(in_RCX + 0x60) == 0;
    if (bVar16) {
      *(longlong *)(in_RCX + 0x60) = 8;
    }
    UNLOCK();
    if (!bVar16) {
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    hashbrown::map::HashMap::
    insert<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    bVar16 = false;
    LOCK();
    bVar17 = *(longlong *)(in_RCX + 0x60) == 8;
    if (bVar17) {
      *(longlong *)(in_RCX + 0x60) = 0;
    }
    UNLOCK();
    if (!bVar17) {
      bVar16 = false;
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
  }
  if (!SBORROW8(0,_Size)) {
    LOCK();
    bVar17 = *(longlong *)(in_RCX + 0x98) == 0;
    if (bVar17) {
      *(longlong *)(in_RCX + 0x98) = 8;
    }
    UNLOCK();
    if (!bVar17) {
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    hashbrown::map::HashMap::
    insert<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar17 = *(longlong *)(in_RCX + 0x98) == 8;
    if (bVar17) {
      *(longlong *)(in_RCX + 0x98) = 0;
    }
    UNLOCK();
    if (!bVar17) {
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
  }
  if (((!SBORROW8(0,in_RDX[0xf])) && (bVar16)) && (in_RDX[0xf] != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

