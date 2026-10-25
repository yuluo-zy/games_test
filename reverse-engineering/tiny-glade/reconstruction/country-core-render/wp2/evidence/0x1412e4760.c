// address 0x1412e4760  name clear<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl clear<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
clear<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  ulonglong *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong *plVar7;
  byte bVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  longlong *in_RCX;
  longlong lVar16;
  longlong lVar17;
  undefined4 *puVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  
  uVar19 = in_RCX[1];
  if (uVar19 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  *(int *)(in_RCX + 10) = (int)uVar19 + -1;
  lVar16 = *(longlong *)(in_RCX[4] + 0x10);
  if (lVar16 != 0) {
    lVar4 = *(longlong *)(in_RCX[4] + 8);
    lVar22 = *in_RCX;
    lVar5 = in_RCX[2];
    uVar6 = in_RCX[3];
    lVar17 = 0;
    do {
      uVar3 = *(uint *)(lVar4 + lVar17);
      if (uVar19 <= uVar3) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      *(undefined1 *)(lVar22 + (ulonglong)uVar3) = 0;
      if (uVar6 <= uVar3 >> 6) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      bVar8 = (byte)uVar3 & 0x3f;
      LOCK();
      puVar1 = (ulonglong *)(lVar5 + (ulonglong)(uVar3 >> 6) * 8);
      *puVar1 = *puVar1 & (-2L << bVar8 | 0xfffffffffffffffeU >> 0x40 - bVar8);
      UNLOCK();
      lVar17 = lVar17 + 4;
    } while (lVar16 << 2 != lVar17);
  }
  plVar7 = (longlong *)in_RCX[7];
  LOCK();
  lVar16 = *plVar7;
  if (lVar16 == 0) {
    *plVar7 = 8;
  }
  UNLOCK();
  if (lVar16 != 0) {
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar16 = in_RCX[4];
  uVar6 = *(ulonglong *)(lVar16 + 8);
  uVar20 = *(ulonglong *)(lVar16 + 0x10);
  *(undefined8 *)(lVar16 + 0x10) = 0;
  uVar19 = uVar6 + uVar20 * 4;
  lVar16 = plVar7[3];
  if ((ulonglong)(plVar7[1] - lVar16) < uVar20) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar16 = plVar7[3];
  }
  lVar4 = plVar7[2];
  if (uVar6 != uVar19) {
    uVar20 = (uVar19 - uVar6) - 4;
    if ((0x6b < uVar20) &&
       ((uVar21 = lVar4 + lVar16 * 4, uVar19 <= uVar21 ||
        (uVar21 + (uVar20 & 0xfffffffffffffffc) + 4 <= (uVar19 - (uVar20 & 0xfffffffffffffffc)) - 4)
        ))) {
      uVar20 = (uVar20 >> 2) + 1;
      uVar21 = uVar20 & 0xfffffffffffffff8;
      puVar18 = (undefined4 *)(lVar4 + lVar16 * 4 + 0x10);
      lVar16 = lVar16 + uVar21;
      lVar22 = 0;
      do {
        puVar2 = (undefined4 *)((uVar19 - 0x20) + lVar22 * 4);
        uVar9 = *puVar2;
        uVar10 = puVar2[1];
        uVar11 = puVar2[2];
        uVar12 = puVar2[3];
        puVar2 = (undefined4 *)((uVar19 - 0x10) + lVar22 * 4);
        uVar13 = *puVar2;
        uVar14 = puVar2[1];
        uVar15 = puVar2[2];
        puVar18[-4] = puVar2[3];
        puVar18[-3] = uVar15;
        puVar18[-2] = uVar14;
        puVar18[-1] = uVar13;
        *puVar18 = uVar12;
        puVar18[1] = uVar11;
        puVar18[2] = uVar10;
        puVar18[3] = uVar9;
        puVar18 = puVar18 + 8;
        lVar22 = lVar22 + -8;
      } while (-lVar22 != uVar21);
      uVar19 = uVar19 + uVar21 * -4;
      if (uVar20 == uVar21) goto LAB_1412e4978;
    }
    do {
      uVar20 = uVar19 - 4;
      *(undefined4 *)(lVar4 + lVar16 * 4) = *(undefined4 *)(uVar19 - 4);
      lVar16 = lVar16 + 1;
      uVar19 = uVar20;
    } while (uVar20 != uVar6);
  }
LAB_1412e4978:
  plVar7[3] = lVar16;
  core::ptr::
  drop_in_place<alloc::vec::drain::Drain<rhapsody::rg::resource::ResourceHandle,alloc::alloc::Global>_>
            ();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar16 = *plVar7;
  if (lVar16 == 8) {
    *plVar7 = 0;
  }
  UNLOCK();
  if (lVar16 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    return;
  }
  return;
}


