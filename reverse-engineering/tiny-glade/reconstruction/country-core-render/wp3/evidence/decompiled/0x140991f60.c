// address 0x140991f60
// name remove_instance
// signature void __cdecl remove_instance(void)

void __cdecl country_core::render::nani::gpu_world::GpuWorld::remove_instance(void)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong *extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong in_RDX;
  ulonglong uVar10;
  bool bVar11;
  
  uVar10 = (ulonglong)*(byte *)(in_RDX + 8);
  owned_subset_mut();
  ::utils::index_condenser::IndexCondenser::
  remove<country_core::render::nani::gpu_world::InstanceKey>();
  if (extraout_RAX_00 == 1) {
    if (*(longlong *)(uVar10 + 0xa0) == 0) {
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    uVar8 = *(longlong *)(uVar10 + 0xa0) - 1;
    *(ulonglong *)(uVar10 + 0xa0) = uVar8;
    if (*(ulonglong *)(uVar10 + 0x60) <= uVar8) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(uVar10 + 0x60) <= in_RDX) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    puVar1 = (undefined4 *)(*(longlong *)(uVar10 + 0x58) + uVar8 * 0x10);
    uVar4 = puVar1[1];
    uVar5 = puVar1[2];
    uVar6 = puVar1[3];
    puVar2 = (undefined4 *)(*(longlong *)(uVar10 + 0x58) + in_RDX * 0x10);
    *puVar2 = *puVar1;
    puVar2[1] = uVar4;
    puVar2[2] = uVar5;
    puVar2[3] = uVar6;
    uVar8 = *(ulonglong *)(uVar10 + 0x48);
    do {
      uVar9 = in_RDX;
      if (uVar8 <= in_RDX) {
        uVar9 = uVar8;
      }
      LOCK();
      uVar7 = *(ulonglong *)(uVar10 + 0x48);
      bVar11 = uVar8 == uVar7;
      if (bVar11) {
        *(ulonglong *)(uVar10 + 0x48) = uVar9;
        uVar7 = uVar8;
      }
      UNLOCK();
      uVar8 = uVar7;
    } while (!bVar11);
    uVar8 = *(ulonglong *)(uVar10 + 0x50);
    do {
      uVar9 = in_RDX;
      if (in_RDX < uVar8) {
        uVar9 = uVar8;
      }
      LOCK();
      uVar7 = *(ulonglong *)(uVar10 + 0x50);
      bVar11 = uVar8 == uVar7;
      if (bVar11) {
        *(ulonglong *)(uVar10 + 0x50) = uVar9;
        uVar7 = uVar8;
      }
      UNLOCK();
      uVar8 = uVar7;
    } while (!bVar11);
    (**(code **)(*(longlong *)(uVar10 + 0x98) + 0x50))(*(undefined8 *)(uVar10 + 0x90));
  }
  else if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    ::log::__private_api::impl$2::log();
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar3 = *extraout_RAX;
  if (lVar3 == 8) {
    *extraout_RAX = 0;
  }
  UNLOCK();
  if (lVar3 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  return;
}

