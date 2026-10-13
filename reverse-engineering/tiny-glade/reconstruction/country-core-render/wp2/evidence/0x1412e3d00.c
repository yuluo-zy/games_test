// address 0x1412e3d00  name take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

// WARNING: Removing unreachable block (ram,0x0001412e406c)
// WARNING: Removing unreachable block (ram,0x0001412e407d)
// WARNING: Removing unreachable block (ram,0x0001412e4096)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  ulonglong *puVar1;
  undefined4 *puVar2;
  longlong lVar3;
  uint uVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong lVar8;
  ulonglong uVar9;
  byte bVar10;
  code *pcVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  longlong *plVar19;
  longlong *in_RCX;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  undefined4 *puVar26;
  bool bVar27;
  
  plVar5 = (longlong *)in_RCX[0xc];
  LOCK();
  lVar3 = *plVar5;
  *plVar5 = *plVar5 + 1;
  UNLOCK();
  if (*plVar5 == 0 || SCARRY8(lVar3,1) != *plVar5 < 0) {
                    // WARNING: Does not return
    pcVar11 = (code *)invalidInstructionException();
    (*pcVar11)();
  }
  LOCK();
  bVar27 = (char)plVar5[2] == '\0';
  if (bVar27) {
    *(char *)(plVar5 + 2) = '\x01';
  }
  UNLOCK();
  if (!bVar27) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  plVar6 = (longlong *)plVar5[4];
  lVar3 = plVar5[5];
  plVar5[5] = 0;
  plVar19 = plVar6;
  do {
    if (plVar19 == plVar6 + lVar3) {
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar27 = (char)plVar5[2] == '\x01';
      if (bVar27) {
        *(char *)(plVar5 + 2) = '\0';
      }
      UNLOCK();
      if (!bVar27) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      LOCK();
      *plVar5 = *plVar5 + -1;
      UNLOCK();
      if (*plVar5 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::components::transient_mesh::TransientMeshRegistry,alloc::alloc::Global>
                  ();
      }
      return;
    }
    plVar7 = (longlong *)*plVar19;
    if (plVar7 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar20 = plVar7[2];
    if (lVar20 != 0) {
      lVar8 = plVar7[1];
      lVar23 = 0;
      do {
        uVar4 = *(uint *)(lVar8 + lVar23);
        if ((ulonglong)in_RCX[5] <= (ulonglong)uVar4) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        *(undefined1 *)(in_RCX[4] + (ulonglong)uVar4) = 0;
        if ((ulonglong)in_RCX[7] <= (ulonglong)(uVar4 >> 6)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        bVar10 = (byte)uVar4 & 0x3f;
        LOCK();
        puVar1 = (ulonglong *)(in_RCX[6] + (ulonglong)(uVar4 >> 6) * 8);
        *puVar1 = *puVar1 & (-2L << bVar10 | 0xfffffffffffffffeU >> 0x40 - bVar10);
        UNLOCK();
        lVar23 = lVar23 + 4;
      } while (lVar20 << 2 != lVar23);
    }
    LOCK();
    lVar20 = *in_RCX;
    if (lVar20 == 0) {
      *in_RCX = 8;
    }
    UNLOCK();
    if (lVar20 != 0) {
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    uVar9 = plVar7[1];
    uVar24 = plVar7[2];
    plVar7[2] = 0;
    uVar22 = uVar9 + uVar24 * 4;
    lVar20 = in_RCX[3];
    if ((ulonglong)(in_RCX[1] - lVar20) < uVar24) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar20 = in_RCX[3];
    }
    lVar8 = in_RCX[2];
    if (uVar9 != uVar22) {
      uVar24 = (uVar22 - uVar9) - 4;
      if ((0x3b < uVar24) &&
         ((uVar25 = lVar8 + lVar20 * 4, uVar22 <= uVar25 ||
          (uVar21 = (uVar22 - 4) - uVar9 & 0xfffffffffffffffc,
          uVar25 + uVar21 + 4 <= (uVar22 - 4) - uVar21)))) {
        uVar24 = (uVar24 >> 2) + 1;
        uVar25 = uVar24 & 0xfffffffffffffff8;
        puVar26 = (undefined4 *)(lVar8 + lVar20 * 4 + 0x10);
        lVar20 = lVar20 + uVar25;
        lVar23 = 0;
        do {
          puVar2 = (undefined4 *)((uVar22 - 0x20) + lVar23 * 4);
          uVar12 = *puVar2;
          uVar13 = puVar2[1];
          uVar14 = puVar2[2];
          uVar15 = puVar2[3];
          puVar2 = (undefined4 *)((uVar22 - 0x10) + lVar23 * 4);
          uVar16 = *puVar2;
          uVar17 = puVar2[1];
          uVar18 = puVar2[2];
          puVar26[-4] = puVar2[3];
          puVar26[-3] = uVar18;
          puVar26[-2] = uVar17;
          puVar26[-1] = uVar16;
          *puVar26 = uVar15;
          puVar26[1] = uVar14;
          puVar26[2] = uVar13;
          puVar26[3] = uVar12;
          puVar26 = puVar26 + 8;
          lVar23 = lVar23 + -8;
        } while (-lVar23 != uVar25);
        uVar22 = uVar22 + uVar25 * -4;
        if (uVar24 == uVar25) goto LAB_1412e3fc8;
      }
      do {
        uVar24 = uVar22 - 4;
        *(undefined4 *)(lVar8 + lVar20 * 4) = *(undefined4 *)(uVar22 - 4);
        lVar20 = lVar20 + 1;
        uVar22 = uVar24;
      } while (uVar24 != uVar9);
    }
LAB_1412e3fc8:
    in_RCX[3] = lVar20;
    core::ptr::
    drop_in_place<alloc::vec::drain::Drain<rhapsody::rg::resource::ResourceHandle,alloc::alloc::Global>_>
              ();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    lVar20 = *in_RCX;
    if (lVar20 == 8) {
      *in_RCX = 0;
    }
    UNLOCK();
    if (lVar20 != 8) {
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
    if (*plVar7 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    plVar19 = plVar19 + 1;
  } while( true );
}


