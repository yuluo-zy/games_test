// take_egui_input
// sig: void __cdecl take_egui_input(void)
// size: 853

void __cdecl tiny_glade::event_loop::EguiEventIo::take_egui_input(void)

{
  byte *pbVar1;
  char cVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  longlong lVar6;
  undefined *puVar7;
  char extraout_AL;
  byte extraout_AL_00;
  char extraout_AL_01;
  undefined *puVar8;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  uint uVar9;
  longlong *plVar10;
  longlong in_RDX;
  bool bVar11;
  byte local_4c;
  
  lVar6 = *(longlong *)(in_RDX + 8);
  LOCK();
  bVar11 = *(char *)(lVar6 + 0x10) == '\0';
  if (bVar11) {
    *(char *)(lVar6 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar11) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar7 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    local_4c = 0;
    cVar2 = *(char *)(lVar6 + 0x11);
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    local_4c = extraout_AL_00 ^ 1;
    cVar2 = *(char *)(lVar6 + 0x11);
  }
  if (cVar2 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    puVar8 = &DAT_1429a7acb;
    do {
      bVar3 = puVar8[-1];
      uVar9 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar8[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar8[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar8 + -4;
            puVar8 = puVar8 + -4;
            uVar9 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar8 = puVar8 + -3;
            uVar9 = bVar5 & 0xf;
          }
          uVar9 = bVar4 & 0x3f | uVar9 << 6;
        }
        else {
          puVar8 = puVar8 + -2;
          uVar9 = bVar4 & 0x1f;
        }
        uVar9 = (uint)(bVar3 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_14026e0d9;
LAB_14026e0f6:
        if (puVar8 == &UNK_1429a7aa7) break;
        if (puVar8 + -0x1429a7aa7 < (undefined *)0x23) {
          if (-0x41 < (char)puVar8[1]) break;
        }
        else if (puVar8 + -0x1429a7aa7 == (undefined *)0x23) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar8 = puVar8 + -1;
      if (uVar9 == 0x5c) goto LAB_14026e0f6;
LAB_14026e0d9:
      if (uVar9 == 0x2f) goto LAB_14026e0f6;
    } while (puVar8 != &DAT_1429a7aa8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar10 = extraout_RAX_00;
      if (extraout_RAX_00 == (longlong *)0x0) goto LAB_14026e323;
    }
    else {
      if ((int)*extraout_RAX != 1) {
LAB_14026e323:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar10 = extraout_RAX + 1;
    }
    if (*plVar10 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
  }
  egui_winit::State::take_egui_input();
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar10 = extraout_RAX_02;
      if (extraout_RAX_02 == (longlong *)0x0) goto LAB_14026e331;
    }
    else {
      if ((int)*extraout_RAX_01 != 1) {
LAB_14026e331:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar10 = extraout_RAX_01 + 1;
    }
    if (*plVar10 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  if (((local_4c == 0) && ((*(ulonglong *)puVar7 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
    *(undefined1 *)(lVar6 + 0x11) = 1;
  }
  LOCK();
  cVar2 = *(char *)(lVar6 + 0x10);
  *(char *)(lVar6 + 0x10) = '\0';
  UNLOCK();
  if (cVar2 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
  return;
}

