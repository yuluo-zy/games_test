// addr 0x141a3c070 sig void __cdecl push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void) size 670

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  ulonglong *puVar1;
  undefined8 *puVar2;
  byte bVar3;
  uint uVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong *in_RCX;
  undefined8 *in_RDX;
  bool bVar18;
  
  uVar4 = *(uint *)(in_RCX + 10);
  uVar17 = (ulonglong)uVar4;
  if ((ulonglong)in_RCX[1] <= uVar17) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  bVar3 = *(byte *)(*in_RCX + uVar17);
  if ((ulonglong)bVar3 < 0x20) {
    uVar15 = uVar17 << 5 | (ulonglong)bVar3;
    *(byte *)(*in_RCX + uVar17) = bVar3 + 1;
    if ((ulonglong)in_RCX[6] <= uVar15) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    lVar5 = in_RCX[5];
    lVar16 = uVar15 * 0x40;
    uVar8 = *in_RDX;
    uVar9 = in_RDX[1];
    uVar10 = in_RDX[2];
    uVar11 = in_RDX[3];
    uVar12 = in_RDX[4];
    uVar13 = in_RDX[5];
    uVar14 = in_RDX[7];
    puVar2 = (undefined8 *)(lVar5 + 0x30 + lVar16);
    *puVar2 = in_RDX[6];
    puVar2[1] = uVar14;
    puVar2 = (undefined8 *)(lVar5 + 0x20 + lVar16);
    *puVar2 = uVar12;
    puVar2[1] = uVar13;
    puVar2 = (undefined8 *)(lVar5 + 0x10 + lVar16);
    *puVar2 = uVar10;
    puVar2[1] = uVar11;
    *(undefined8 *)(lVar5 + lVar16) = uVar8;
    ((undefined8 *)(lVar5 + lVar16))[1] = uVar9;
    if ((ulonglong)in_RCX[9] <= (ulonglong)(uVar4 >> 6)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    LOCK();
    puVar1 = (ulonglong *)(in_RCX[8] + (ulonglong)(uVar4 >> 6) * 8);
    *puVar1 = *puVar1 | 1L << ((byte)uVar4 & 0x3f);
    UNLOCK();
  }
  else {
    plVar6 = (longlong *)in_RCX[7];
    LOCK();
    lVar5 = *plVar6;
    if (lVar5 == 0) {
      *plVar6 = 8;
    }
    UNLOCK();
    if (lVar5 != 0) {
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    lVar5 = plVar6[3];
    if (lVar5 == 0) {
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar18 = *plVar6 == 8;
      if (bVar18) {
        *plVar6 = 0;
      }
      UNLOCK();
    }
    else {
      plVar6[3] = lVar5 + -1;
      uVar4 = *(uint *)(plVar6[2] + -4 + lVar5 * 4);
      uVar17 = (ulonglong)uVar4;
      plVar7 = (longlong *)in_RCX[4];
      lVar5 = plVar7[2];
      if (lVar5 == *plVar7) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      *(uint *)(plVar7[1] + lVar5 * 4) = uVar4;
      plVar7[2] = lVar5 + 1;
      if ((ulonglong)in_RCX[1] <= uVar17) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      *(undefined1 *)(*in_RCX + uVar17) = 1;
      if ((ulonglong)in_RCX[6] <= uVar17 << 5) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      lVar5 = in_RCX[5];
      lVar16 = uVar17 * 0x800;
      uVar8 = *in_RDX;
      uVar9 = in_RDX[1];
      uVar10 = in_RDX[2];
      uVar11 = in_RDX[3];
      uVar12 = in_RDX[4];
      uVar13 = in_RDX[5];
      uVar14 = in_RDX[7];
      puVar2 = (undefined8 *)(lVar5 + 0x30 + lVar16);
      *puVar2 = in_RDX[6];
      puVar2[1] = uVar14;
      puVar2 = (undefined8 *)(lVar5 + 0x20 + lVar16);
      *puVar2 = uVar12;
      puVar2[1] = uVar13;
      puVar2 = (undefined8 *)(lVar5 + 0x10 + lVar16);
      *puVar2 = uVar10;
      puVar2[1] = uVar11;
      *(undefined8 *)(lVar5 + lVar16) = uVar8;
      ((undefined8 *)(lVar5 + lVar16))[1] = uVar9;
      uVar17 = (ulonglong)(uVar4 >> 6);
      if ((ulonglong)in_RCX[3] <= uVar17) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar15 = 1L << ((byte)uVar4 & 0x3f);
      LOCK();
      puVar1 = (ulonglong *)(in_RCX[2] + uVar17 * 8);
      *puVar1 = *puVar1 | uVar15;
      UNLOCK();
      if ((ulonglong)in_RCX[9] <= uVar17) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      LOCK();
      puVar1 = (ulonglong *)(in_RCX[8] + uVar17 * 8);
      *puVar1 = *puVar1 | uVar15;
      UNLOCK();
      *(uint *)(in_RCX + 10) = uVar4;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar18 = *plVar6 == 8;
      if (bVar18) {
        *plVar6 = 0;
      }
      UNLOCK();
    }
    if (!bVar18) {
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
  }
  return;
}


// CALLEES: [{"address": "140c81c70", "name": "acquire_resource"}, {"address": "140b5f870", "name": "grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>"}, {"address": "1428cb5f0", "name": "lock_exclusive_slow"}, {"address": "1428d9518", "name": "panic_bounds_check"}, {"address": "140c81e10", "name": "release_resource"}, {"address": "1428cbf60", "name": "unlock_exclusive_slow"}]