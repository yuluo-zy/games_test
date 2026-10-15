
void __cdecl country_core::systems::backend::flush_screen_log::flush_error_log_to_screen(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  longlong lVar3;
  char cVar4;
  byte extraout_AL;
  char extraout_AL_00;
  longlong lVar5;
  longlong in_RCX;
  longlong lVar6;
  ulonglong uVar7;
  byte bVar8;
  ulonglong *puVar9;
  longlong lVar10;
  bool bVar11;
  
  LOCK();
  bVar11 = SCREEN_LOG_BUFFER == '\0';
  if (bVar11) {
    SCREEN_LOG_BUFFER = '\x01';
  }
  UNLOCK();
  if (!bVar11) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar9 = (ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
  ;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar8 = 0;
    lVar10 = DAT_14362bff8;
    lVar3 = DAT_14362c000;
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar8 = extraout_AL ^ 1;
    lVar10 = DAT_14362bff8;
    lVar3 = DAT_14362c000;
  }
  DAT_14362bff8 = lVar10;
  if (DAT_14362bfe9 != '\0') {
    DAT_14362c000 = lVar3;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  DAT_14362c000 = 0;
  lVar6 = lVar3 * 0x20;
  DAT_14362bfe9 = 0;
  if (lVar3 != 0) {
    uVar1 = *(undefined4 *)(in_RCX + 0x1c);
    puVar2 = *(undefined4 **)(in_RCX + 0x10);
    lVar3 = 0;
    do {
      lVar5 = lVar3;
      lVar3 = *(longlong *)(lVar10 + lVar5);
      if (SBORROW8(0,lVar3)) break;
      *puVar2 = uVar1;
      resources::gui::DebugGui::log_to_screen();
      if (lVar3 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar3 = lVar5 + 0x20;
    } while (lVar6 != lVar5 + 0x20);
    puVar9 = (ulonglong *)
             __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
    lVar3 = DAT_14362bff8;
    if (lVar6 + -0x20 != lVar5) {
      uVar7 = (lVar6 - lVar5) - 0x20U >> 5;
      lVar10 = lVar10 + 0x28 + lVar5 + DAT_14362bff8;
      do {
        if (*(longlong *)((lVar10 + -8) - lVar3) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar10 = lVar10 + 0x20;
        uVar7 = uVar7 - 1;
      } while (uVar7 != 0);
    }
  }
  if (((bVar8 == 0) && ((*puVar9 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    DAT_14362bfe9 = 1;
  }
  cVar4 = SCREEN_LOG_BUFFER;
  LOCK();
  SCREEN_LOG_BUFFER = 0;
  UNLOCK();
  if (cVar4 != '\x02') {
    return;
  }
  std::sys::sync::mutex::futex::Mutex::wake();
  return;
}


