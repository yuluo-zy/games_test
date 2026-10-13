// ADDRESS: 0x140937cb0
// SIZE: 340
// SIGNATURE: void __cdecl debug_raster_get_name(void)

void __cdecl
country_core::asset_libraries::shader_library::AssetShaderLibrary::debug_raster_get_name(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  undefined1 auVar3 [16];
  ulonglong uVar4;
  ushort uVar5;
  uint uVar6;
  undefined1 (*pauVar7) [16];
  longlong in_RCX;
  longlong lVar8;
  longlong in_RDX;
  undefined1 (*pauVar9) [16];
  uint uVar10;
  uint uVar11;
  bool bVar12;
  
  puVar1 = (ulonglong *)(in_RCX + 0x80);
  uVar4 = *(ulonglong *)(in_RCX + 0x80);
  if (uVar4 < 0xfffffffffffffff0 && (char)(((uint)uVar4 & 8) >> 3) == '\0') {
    LOCK();
    uVar2 = *puVar1;
    if (uVar4 == uVar2) {
      *puVar1 = uVar4 + 0x10;
    }
    UNLOCK();
    if (uVar4 == uVar2) goto LAB_140937cfd;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_140937cfd:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  pauVar7 = *(undefined1 (**) [16])(in_RCX + 0x88);
  lVar8 = *(longlong *)(in_RCX + 0xa0);
  auVar3 = *pauVar7;
  uVar10 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
  pauVar9 = pauVar7 + 1;
  do {
    bVar12 = lVar8 == 0;
    lVar8 = lVar8 + -1;
    if (bVar12) {
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      uVar4 = *puVar1;
      *puVar1 = *puVar1 - 0x10;
      UNLOCK();
      goto LAB_140937df0;
    }
    if ((short)uVar10 == 0) {
      do {
        auVar3 = *pauVar9;
        uVar5 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
        pauVar7 = pauVar7 + -0x18;
        pauVar9 = pauVar9 + 1;
      } while (uVar5 == 0xffff);
      uVar11 = ~(uint)uVar5;
      uVar10 = ~(uint)uVar5 & -(uint)uVar5 - 2;
    }
    else {
      uVar11 = uVar10;
      uVar10 = uVar10 - 1 & uVar10;
    }
    uVar6 = 0;
    for (; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
  } while (*(longlong *)((longlong)pauVar7 + (ulonglong)uVar6 * -0x18 + -8) != in_RDX);
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar4 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
LAB_140937df0:
  if ((uVar4 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  return;
}

