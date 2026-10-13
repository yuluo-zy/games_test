// ADDRESS: 0x1409378e0
// SIZE: 446
// SIGNATURE: void __cdecl get_compute_by_name(void)

void __cdecl
country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name(void)

{
  ulonglong *puVar1;
  char *pcVar2;
  ulonglong uVar3;
  uint uVar4;
  uint uVar5;
  longlong in_RCX;
  ulonglong *in_RDX;
  ushort uVar6;
  ulonglong uVar7;
  longlong lVar8;
  uint uVar9;
  byte bVar10;
  char cVar11;
  char cVar13;
  char cVar14;
  char cVar15;
  undefined1 auVar12 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  
  puVar1 = (ulonglong *)(in_RCX + 0xb8);
  uVar3 = *(ulonglong *)(in_RCX + 0xb8);
  if (uVar3 < 0xfffffffffffffff0 && (char)(((uint)uVar3 & 8) >> 3) == '\0') {
    LOCK();
    uVar7 = *puVar1;
    if (uVar3 == uVar7) {
      *puVar1 = uVar3 + 0x10;
    }
    UNLOCK();
    if (uVar3 == uVar7) goto LAB_140937932;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_140937932:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (*(longlong *)(in_RCX + 0xd8) != 0) {
    uVar3 = *in_RDX;
    bVar10 = (byte)(uVar3 >> 0x39);
    auVar12 = ZEXT216(CONCAT11(bVar10,bVar10));
    auVar12 = pshuflw(auVar12,auVar12,0);
    lVar8 = 0;
    uVar7 = uVar3;
    while( true ) {
      uVar7 = *(ulonglong *)(in_RCX + 200) & uVar7;
      pcVar2 = (char *)(*(longlong *)(in_RCX + 0xc0) + uVar7);
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
      uVar6 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
      uVar9 = (uint)uVar6;
      while (uVar6 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        if (uVar3 == *(ulonglong *)
                      (*(longlong *)(in_RCX + 0xc0) + -0x10 +
                      (uVar4 + uVar7 & *(ulonglong *)(in_RCX + 200)) * -0x10)) {
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          uVar3 = *puVar1;
          *puVar1 = *puVar1 - 0x10;
          UNLOCK();
          if ((uVar3 & 0xfffffffffffffff2) == 0x12) {
            parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
          }
          frozen::FrozenMap::
          get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>_>
                    ();
          return;
        }
        uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
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
      uVar7 = uVar7 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar3 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
  if ((uVar3 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  return;
}

