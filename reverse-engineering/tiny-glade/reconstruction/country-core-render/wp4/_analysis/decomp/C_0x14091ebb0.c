// ADDRESS: 0x14091ebb0
// SIZE: 430
// SIGNATURE: void __cdecl export_from_rg(void)

void __cdecl country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg(void)

{
  undefined1 auVar1 [16];
  char cVar2;
  longlong lVar3;
  ushort uVar4;
  int iVar5;
  longlong in_RCX;
  uint uVar6;
  uint uVar7;
  undefined1 (*pauVar8) [16];
  undefined1 (*pauVar9) [16];
  longlong lVar10;
  bool bVar11;
  
  if (*(char *)(in_RCX + 8) != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  lVar10 = *(longlong *)(in_RCX + 0x28);
  if (lVar10 != 0) {
    pauVar8 = *(undefined1 (**) [16])(in_RCX + 0x10);
    pauVar9 = pauVar8 + 1;
    auVar1 = *pauVar8;
    uVar6 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
    do {
      while( true ) {
        if ((short)uVar6 == 0) {
          do {
            auVar1 = *pauVar9;
            uVar4 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1
                    | (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
            pauVar8 = pauVar8 + -0x10;
            pauVar9 = pauVar9 + 1;
          } while (uVar4 == 0xffff);
          uVar7 = ~(uint)uVar4;
          uVar6 = uVar7 & -(uint)uVar4 - 2;
        }
        else {
          uVar7 = uVar6;
          uVar6 = uVar6 - 1 & uVar6;
        }
        iVar5 = 0;
        for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          iVar5 = iVar5 + 1;
        }
        lVar3 = *(longlong *)((longlong)pauVar8 + (-8 - (ulonglong)(uint)(iVar5 << 4)));
        LOCK();
        bVar11 = *(longlong *)(lVar3 + 0x20) == 0;
        if (bVar11) {
          *(longlong *)(lVar3 + 0x20) = 8;
        }
        UNLOCK();
        if (!bVar11) {
          parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
        }
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        cVar2 = *(char *)(lVar3 + 0x36);
        *(undefined1 *)(lVar3 + 0x36) = 5;
        if (cVar2 != '\x05') {
          rhapsody::rg::import_export::impl$1::export();
        }
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar11 = *(longlong *)(lVar3 + 0x20) == 8;
        if (bVar11) {
          *(longlong *)(lVar3 + 0x20) = 0;
        }
        UNLOCK();
        if (!bVar11) break;
        lVar10 = lVar10 + -1;
        if (lVar10 == 0) {
          return;
        }
      }
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
      lVar10 = lVar10 + -1;
    } while (lVar10 != 0);
  }
  return;
}

