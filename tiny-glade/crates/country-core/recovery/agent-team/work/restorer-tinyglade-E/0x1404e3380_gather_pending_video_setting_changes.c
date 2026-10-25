
void __cdecl tiny_glade::game_loop::gather_pending_video_setting_changes(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  undefined *puVar3;
  char cVar4;
  char extraout_AL;
  byte extraout_AL_00;
  byte bVar5;
  longlong lVar6;
  undefined8 *in_RCX;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  bool bVar14;
  
  puVar1 = (ulonglong *)*in_RCX;
  lVar10 = in_RCX[1];
  uVar2 = *puVar1;
  uVar7 = 0;
  uVar9 = uVar2 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar2 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar9 = uVar7;
  }
  uVar8 = uVar2 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar2 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar8 = uVar7;
  }
  lVar11 = uVar9 * 0x10 + *(longlong *)(lVar10 + 8);
  uVar2 = *(ulonglong *)(lVar10 + 0x10) - uVar9;
  if (*(ulonglong *)(lVar10 + 0x10) < uVar9) {
    lVar11 = 8;
    uVar2 = uVar7;
  }
  lVar13 = uVar8 * 0x10 + *(longlong *)(lVar10 + 0x28);
  uVar9 = *(ulonglong *)(lVar10 + 0x30) - uVar8;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar8) {
    lVar13 = 8;
    uVar9 = uVar7;
  }
  *puVar1 = (*(longlong *)(lVar10 + 0x40) - uVar9) - uVar2;
  puVar3 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  lVar12 = uVar9 * 0x10 + lVar13;
  lVar10 = lVar11;
  if (lVar11 != 0) goto LAB_1404e3441;
LAB_1404e3470:
  lVar10 = lVar13;
  if ((lVar10 == 0) || (lVar10 == lVar12)) {
    return;
  }
  lVar13 = lVar10 + 0x10;
  lVar6 = 0;
  do {
    *puVar1 = *puVar1 + 1;
    LOCK();
    bVar14 = PENDING_VIDEO_SETTING_CHANGES == '\0';
    if (bVar14) {
      PENDING_VIDEO_SETTING_CHANGES = '\x01';
    }
    UNLOCK();
    if (bVar14) {
      if ((*(ulonglong *)puVar3 & 0x7fffffffffffffff) != 0) goto LAB_1404e3515;
LAB_1404e34a8:
      bVar5 = 0;
    }
    else {
      std::sys::sync::mutex::futex::Mutex::lock_contended();
      if ((*(ulonglong *)puVar3 & 0x7fffffffffffffff) == 0) goto LAB_1404e34a8;
LAB_1404e3515:
      std::panicking::panic_count::is_zero_slow_path();
      bVar5 = extraout_AL_00 ^ 1;
    }
    if (DAT_143634b92 != '\0') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    DAT_143634b93 = DAT_143634b93 | *(byte *)(lVar10 + 8);
    if (((bVar5 == 0) && ((*(ulonglong *)puVar3 & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL == '\0')) {
      DAT_143634b92 = '\x01';
    }
    cVar4 = PENDING_VIDEO_SETTING_CHANGES;
    LOCK();
    PENDING_VIDEO_SETTING_CHANGES = '\0';
    UNLOCK();
    if (cVar4 == '\x02') {
      std::sys::sync::mutex::futex::Mutex::wake();
    }
    lVar10 = lVar6;
    if (lVar6 == 0) goto LAB_1404e3470;
LAB_1404e3441:
    if (lVar10 == uVar2 * 0x10 + lVar11) goto LAB_1404e3470;
    lVar6 = lVar10 + 0x10;
  } while( true );
}

