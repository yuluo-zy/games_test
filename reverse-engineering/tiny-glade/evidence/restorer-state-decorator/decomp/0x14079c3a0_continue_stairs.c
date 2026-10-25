
void __cdecl system_decorator::ui_systems::ui_stairs_misc::continue_stairs(void)

{
  uint uVar1;
  int iVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong *extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar5;
  int local_70;
  int local_60;
  
  uVar1 = *(uint *)*in_RCX;
  lVar5 = 0;
  if (uVar1 - 6 < 0xf) {
    lVar5 = (ulonglong)uVar1 - 5;
  }
  if (lVar5 == 0) {
    if (uVar1 != 4) {
      return;
    }
  }
  else {
    if (lVar5 != 6) {
      return;
    }
    if ((char)((uint *)*in_RCX)[1] != '?') {
      return;
    }
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  uVar3 = *extraout_RAX;
  if (uVar3 < 0xfffffffffffffff0 && (char)(((uint)uVar3 & 8) >> 3) == '\0') {
    LOCK();
    uVar4 = *extraout_RAX;
    if (uVar3 == uVar4) {
      *extraout_RAX = uVar3 + 0x10;
    }
    UNLOCK();
    if (uVar3 == uVar4) goto LAB_14079c445;
  }
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_14079c445:
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  uVar3 = extraout_RAX[3];
  if (uVar3 != 0) {
    uVar4 = extraout_RAX[2];
    lVar5 = 0;
    do {
      if ((*(int *)(uVar4 + lVar5) == 3) && (*(char *)(uVar4 + 0x81 + lVar5) == '?')) {
        iVar2 = *(int *)(uVar4 + 0x40 + lVar5);
        country_core::resources::stairs::stairs_state::StairsState::add_node();
        country_core::resources::stairs::stairs_state::StairsState::endpoint();
        if ((local_70 != 3) && (local_60 != iVar2)) {
          country_core::resources::stairs::stairs_state::StairsState::add_edge();
        }
        if (*(char *)(uVar4 + 0x80 + lVar5) == '\0') {
          country_core::resources::walls::decorator::DecoratorLocator::new();
          country_core::resources::stairs::stairs_state::StairsState::try_set_endpoint();
        }
      }
      lVar5 = lVar5 + 0x88;
    } while (uVar3 * 0x88 != lVar5);
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar3 = *extraout_RAX;
  *extraout_RAX = *extraout_RAX - 0x10;
  UNLOCK();
  if ((uVar3 & 0xfffffffffffffff2) != 0x12) {
    return;
  }
  parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  return;
}

