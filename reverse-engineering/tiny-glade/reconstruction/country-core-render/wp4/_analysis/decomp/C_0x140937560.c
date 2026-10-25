// ADDRESS: 0x140937560
// SIZE: 654
// SIGNATURE: void __cdecl add_compute(void)

void __cdecl country_core::asset_libraries::shader_library::AssetShaderLibrary::add_compute(void)

{
  longlong *plVar1;
  char *pcVar2;
  uint uVar3;
  uint uVar4;
  void *_Dst;
  longlong extraout_RAX;
  longlong in_RCX;
  void *in_RDX;
  ushort uVar5;
  ulonglong in_R8;
  ulonglong uVar6;
  longlong lVar7;
  uint uVar8;
  bool bVar9;
  byte bVar10;
  char cVar11;
  char cVar13;
  char cVar14;
  char cVar15;
  undefined1 auVar12 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined **local_110;
  undefined8 local_108;
  ulonglong **local_100;
  undefined8 local_f8;
  undefined8 local_f0;
  ulonglong *local_70;
  code *local_68;
  ulonglong local_60;
  longlong *local_58;
  ulonglong local_50;
  void *local_48;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  plVar1 = (longlong *)(in_RCX + 0xb8);
  LOCK();
  bVar9 = *(longlong *)(in_RCX + 0xb8) == 0;
  if (bVar9) {
    *(longlong *)(in_RCX + 0xb8) = 8;
  }
  UNLOCK();
  local_60 = in_R8;
  local_48 = in_RDX;
  if (!bVar9) {
    local_39 = 1;
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_58 = plVar1;
  local_50 = (ulonglong)plVar1 | 1;
  if (*(longlong *)(in_RCX + 0xd8) != 0) {
    bVar10 = (byte)(local_60 >> 0x39);
    auVar12 = ZEXT216(CONCAT11(bVar10,bVar10));
    auVar12 = pshuflw(auVar12,auVar12,0);
    lVar7 = 0;
    uVar6 = local_60;
    while( true ) {
      uVar6 = *(ulonglong *)(in_RCX + 200) & uVar6;
      pcVar2 = (char *)(*(longlong *)(in_RCX + 0xc0) + uVar6);
      cVar11 = auVar12[0];
      auVar17[0] = -(*pcVar2 == cVar11);
      cVar13 = auVar12[1];
      auVar17[1] = -(pcVar2[1] == cVar13);
      cVar14 = auVar12[2];
      auVar17[2] = -(pcVar2[2] == cVar14);
      cVar15 = auVar12[3];
      auVar17[3] = -(pcVar2[3] == cVar15);
      auVar17[4] = -(pcVar2[4] == cVar11);
      auVar17[5] = -(pcVar2[5] == cVar13);
      auVar17[6] = -(pcVar2[6] == cVar14);
      auVar17[7] = -(pcVar2[7] == cVar15);
      auVar17[8] = -(pcVar2[8] == cVar11);
      auVar17[9] = -(pcVar2[9] == cVar13);
      auVar17[10] = -(pcVar2[10] == cVar14);
      auVar17[0xb] = -(pcVar2[0xb] == cVar15);
      auVar17[0xc] = -(pcVar2[0xc] == cVar11);
      auVar17[0xd] = -(pcVar2[0xd] == cVar13);
      auVar17[0xe] = -(pcVar2[0xe] == cVar14);
      auVar17[0xf] = -(pcVar2[0xf] == cVar15);
      uVar5 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar5;
      while (uVar5 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        if (local_60 ==
            *(ulonglong *)
             (*(longlong *)(in_RCX + 0xc0) + -0x10 +
             (uVar3 + uVar6 & *(ulonglong *)(in_RCX + 200)) * -0x10)) {
          local_70 = &local_60;
          local_68 = country_core::render::pass_builder::shader_uri::impl$18::fmt;
          local_110 = &PTR_s_AssetShaderLibrary__the_shader_n_142ab10d0;
          local_108 = 2;
          local_f0 = 0;
          local_100 = &local_70;
          local_f8 = 1;
          local_3a = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar5 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar5);
      }
      auVar16[0] = -(*pcVar2 == -1);
      auVar16[1] = -(pcVar2[1] == -1);
      auVar16[2] = -(pcVar2[2] == -1);
      auVar16[3] = -(pcVar2[3] == -1);
      auVar16[4] = -(pcVar2[4] == -1);
      auVar16[5] = -(pcVar2[5] == -1);
      auVar16[6] = -(pcVar2[6] == -1);
      auVar16[7] = -(pcVar2[7] == -1);
      auVar16[8] = -(pcVar2[8] == -1);
      auVar16[9] = -(pcVar2[9] == -1);
      auVar16[10] = -(pcVar2[10] == -1);
      auVar16[0xb] = -(pcVar2[0xb] == -1);
      auVar16[0xc] = -(pcVar2[0xc] == -1);
      auVar16[0xd] = -(pcVar2[0xd] == -1);
      auVar16[0xe] = -(pcVar2[0xe] == -1);
      auVar16[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') break;
      uVar6 = uVar6 + lVar7 + 0x10;
      lVar7 = lVar7 + 0x10;
    }
  }
  LOCK();
  HANDLE_COUNT = HANDLE_COUNT + 1;
  UNLOCK();
  local_3a = 1;
  hashbrown::map::HashMap::
  insert<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash>_>,alloc::alloc::Global>
            ();
  memcpy(&local_110,local_48,0xa0);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_110,0xa0);
  local_3a = 0;
  frozen::FrozenMap::
  insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
            ();
  plVar1 = local_58;
  if (extraout_RAX == 0) {
    local_39 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    local_39 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar9 = *plVar1 == 8;
    if (bVar9) {
      *plVar1 = 0;
    }
    UNLOCK();
    if (!bVar9) {
      local_39 = 0;
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
    return;
  }
  local_3a = 0;
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

