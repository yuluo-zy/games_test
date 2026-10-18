// closure$0 @ 0x1400db6b0
// sig: void __cdecl closure$0(void)


void __cdecl tiny_glade::panic_reporter::setup_panic_hook::closure_0(void)

{
  longlong lVar1;
  code *pcVar2;
  undefined *puVar3;
  char cVar4;
  byte extraout_AL;
  char extraout_AL_00;
  undefined8 *in_RCX;
  bool bVar5;
  char *local_4d0;
  undefined8 local_4c8;
  char *local_4c0;
  undefined8 local_4b8;
  undefined8 ***local_2e8;
  code *pcStack_2e0;
  undefined8 ***local_2d8;
  undefined8 ***pppuStack_2d0;
  code *local_2c8;
  undefined8 ***pppuStack_2c0;
  undefined1 local_2b8 [496];
  undefined8 ***local_c8;
  code *pcStack_c0;
  undefined8 ***local_b8;
  ulonglong local_b0;
  undefined1 *local_a8;
  byte local_a0;
  undefined8 ***local_98;
  code *pcStack_90;
  undefined8 ***local_88;
  undefined8 ***local_78;
  code *pcStack_70;
  undefined8 ***local_68;
  undefined8 ***pppuStack_60;
  code *pcStack_58;
  undefined8 ***pppuStack_50;
  undefined8 **local_40;
  char local_33;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  lVar1 = in_RCX[2];
  LOCK();
  bVar5 = _bss == '\0';
  if (bVar5) {
    _bss = '\x01';
  }
  UNLOCK();
  if (!bVar5) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar3 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    local_a0 = 0;
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    local_a0 = extraout_AL ^ 1;
  }
  local_b0 = (ulonglong)(DAT_143634b79 != '\0');
  local_a8 = &_bss;
  if (DAT_143634b79 != '\0') {
    local_33 = DAT_143634b79;
    local_2e8 = (undefined8 ***)&PTR_DAT_14293aa80;
    pcStack_2e0 = (code *)0x1;
    local_2d8 = (undefined8 ***)0x8;
    pppuStack_2d0 = (undefined8 ***)0x0;
    local_2c8 = (code *)0x0;
    std::io::stdio::_eprint();
    pcVar2 = (code *)swi(0x29);
    (*pcVar2)(7);
                    // WARNING: Does not return
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  if (DAT_143634b7a != '\x01') {
    local_33 = DAT_143634b79;
    DAT_143634b7a = 1;
    core::ptr::
    drop_in_place<std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState>_>_>
              ();
    backtrace::capture::Backtrace::new();
    puVar3 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_78 = &local_98;
      pcStack_70 = reverse_backtrace::impl$0::fmt;
      local_2e8 = (undefined8 ***)&PTR_s_Reverse_backtrace__14293a978;
      pcStack_2e0 = (code *)0x1;
      local_2c8 = (code *)0x0;
      local_2d8 = &local_78;
      pppuStack_2d0 = (undefined8 ****)0x1;
      local_31 = 1;
      local_98 = &local_c8;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_4d0 = "tiny_glade::panic_reporterPANIC: ";
      local_4c8 = 0x1a;
      local_4c0 = "tiny_glade::panic_reporterPANIC: ";
      local_4b8 = 0x1a;
      local_31 = 1;
      ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
    }
    if (*(longlong *)puVar3 != 0) {
      local_78 = &local_40;
      pcStack_70 = core::fmt::impl$76::fmt<std::panic::PanicHookInfo>;
      local_2e8 = (undefined8 ***)&DAT_14293a9f8;
      pcStack_2e0 = (code *)0x1;
      local_2c8 = (code *)0x0;
      local_2d8 = &local_78;
      pppuStack_2d0 = (undefined8 ****)0x1;
      local_31 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_4d0 = "tiny_glade::panic_reporterPANIC: ";
      local_4c8 = 0x1a;
      local_4c0 = "tiny_glade::panic_reporterPANIC: ";
      local_4b8 = 0x1a;
      local_31 = 1;
      ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
    }
    local_31 = 1;
    recover_game_state_after_crash();
    if (lVar1 != 0) {
      local_88 = local_b8;
      local_98 = local_c8;
      pcStack_90 = pcStack_c0;
      sentry_reporter::panic_info_to_message();
      alloc::str::impl$4::to_owned();
      local_68 = local_88;
      local_78 = local_98;
      pcStack_70 = pcStack_90;
      pppuStack_60 = local_2e8;
      pcStack_58 = pcStack_2e0;
      pppuStack_50 = local_2d8;
      memcpy(local_2b8,&local_4d0,0x1e8);
      local_2e8 = local_78;
      pcStack_2e0 = pcStack_70;
      local_2d8 = local_68;
      pppuStack_2d0 = pppuStack_60;
      local_2c8 = pcStack_58;
      pppuStack_2c0 = pppuStack_50;
      local_32 = 0;
      crash_pickup::ClientHandle::send_message();
      local_32 = 0;
      core::result::Result::expect<tuple_<>,enum2_<tiny_glade::crash_pickup::Error>_>();
    }
    pcVar2 = (code *)swi(0x29);
    (*pcVar2)(7);
                    // WARNING: Does not return
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  if (((local_a0 == 0) && ((*(ulonglong *)puVar3 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    DAT_143634b79 = '\x01';
  }
  cVar4 = _bss;
  LOCK();
  _bss = 0;
  UNLOCK();
  if (cVar4 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
                    // WARNING: Could not recover jumptable at 0x0001400db77d. Too many branches
                    // WARNING: Treating indirect jump as call
  (**(code **)(in_RCX[1] + 0x28))(*in_RCX);
  return;
}

