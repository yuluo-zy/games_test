// make_temp_window
// sig: void __cdecl make_temp_window(void)
// size: 2463

void __cdecl tiny_glade::run_game::make_temp_window(void)

{
  char cVar1;
  undefined *puVar2;
  undefined **ppuVar3;
  code *pcVar4;
  uint6 uVar5;
  char *pcVar6;
  byte extraout_AL;
  byte bVar7;
  char extraout_AL_00;
  char extraout_AL_01;
  BOOL BVar8;
  undefined4 extraout_EAX;
  uint extraout_EAX_00;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  char *extraout_RAX_01;
  void *_Dst;
  undefined8 *extraout_RAX_02;
  ulonglong in_RCX;
  longlong in_RDX;
  tagRECT *_Src;
  ulonglong unaff_R13;
  void *pvVar9;
  ulonglong uVar10;
  bool bVar11;
  byte local_3a0;
  longlong local_398;
  HWND pHStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  tagRECT local_378 [2];
  undefined8 local_350;
  undefined8 local_340;
  undefined4 local_338;
  undefined4 local_320;
  undefined4 local_308;
  undefined4 local_2f0;
  undefined4 local_2d8;
  undefined4 local_2c0;
  undefined8 local_2a8;
  undefined4 local_2a0;
  undefined1 local_29c;
  undefined4 local_29b;
  undefined1 local_297;
  undefined4 local_296;
  undefined8 local_290;
  undefined8 uStack_288;
  undefined8 local_280;
  undefined8 uStack_278;
  undefined8 local_270;
  undefined8 uStack_268;
  undefined8 local_260;
  undefined8 uStack_258;
  undefined8 local_250;
  longlong local_228;
  HWND pHStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined **local_208;
  void *pvStack_200;
  ulonglong local_1f8;
  HWND pHStack_1f0;
  ulonglong local_1e8;
  longlong lStack_1e0;
  undefined8 local_1d0;
  undefined4 local_180;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  ulonglong local_d0;
  void *local_c8;
  HWND pHStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  tagRECT *local_80;
  undefined8 local_78;
  char *local_70;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_208 = *(undefined ***)(in_RDX + 0x10);
  pvStack_200 = *(void **)(in_RDX + 0x18);
  LOCK();
  puVar2 = *local_208;
  *local_208 = *local_208 + 1;
  UNLOCK();
  if (*local_208 == (undefined *)0x0 || SCARRY8((longlong)puVar2,1) != (longlong)*local_208 < 0) {
                    // WARNING: Does not return
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
  local_1f8 = local_1f8 & 0xffffffffffffff00;
  winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::
  execute_in_thread<winit::platform_impl::platform::window::impl_0::set_visible::closure_env_0>();
  local_378[0].left = 0;
  local_378[0].top = 0;
  local_378[0].right = 0;
  local_378[0].bottom = 0;
  BVar8 = GetClientRect(*(HWND *)(in_RDX + 0x18),local_378);
  if (BVar8 == 0) {
    local_208 = &PTR_s_Unexpected_GetClientRect_failure_1429feba8;
    pvStack_200 = (void *)0x1;
    local_1f8 = 8;
    pHStack_1f0 = (HWND)0x0;
    local_1e8 = 0;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  winit::dpi::impl$2::from_f64();
  local_78 = CONCAT44(local_78._4_4_,extraout_EAX);
  winit::dpi::impl$2::from_f64();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX = 0x69772074696e6977;
  *(undefined4 *)(extraout_RAX + 1) = 0x776f646e;
  local_338 = 2;
  local_320 = 2;
  local_308 = 2;
  local_2f0 = 2;
  local_29c = 1;
  local_2a0 = 7;
  local_350 = 0xc;
  local_340 = 0xc;
  local_29b = 0x100;
  local_297 = 1;
  local_2a8 = 0;
  local_2d8 = 2;
  local_296 = 0x1020100;
  local_2c0 = 0xf;
  local_378[0].left = 0;
  local_378[0].top = 0;
  winit::platform_impl::platform::impl$0::default();
  memcpy(&local_208,local_378,0xe8);
  local_120 = local_c8;
  uStack_118 = pHStack_c0;
  local_110 = local_b8;
  uStack_108 = uStack_b0;
  local_100 = local_a8;
  uStack_f8 = uStack_a0;
  local_f0 = local_98;
  uStack_e8 = uStack_90;
  local_e0 = local_88;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = 0x616c4720796e6954;
    *(undefined2 *)(extraout_RAX_00 + 1) = 0x6564;
    if (lStack_1e0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lStack_1e0 = 10;
    local_1d0 = 10;
    memcpy(local_378,&local_208,0x130);
    winit::dpi::impl$2::from_f64();
    uVar10 = (ulonglong)extraout_EAX_00;
    winit::dpi::impl$2::from_f64();
    pvVar9 = (void *)(in_RDX + 0x10);
    local_338 = 0;
    _Src = local_378;
    memcpy(&local_208,_Src,0x130);
    winit::platform_impl::platform::window::Window::window_state_lock();
    ppuVar3 = *(undefined ***)(extraout_RAX_01 + 0x28);
    pcVar6 = extraout_RAX_01;
    if (ppuVar3 == (undefined **)0x1) {
      pvVar9 = *(void **)(extraout_RAX_01 + 0x30);
      unaff_R13 = *(ulonglong *)(extraout_RAX_01 + 0x38);
      if (pvVar9 == (void *)0x0) {
        if ((unaff_R13 & 1) == 0) {
          unaff_R13 = 0;
          pvVar9 = (void *)0x0;
          uVar10 = 0;
        }
        else {
          uVar10 = *(ulonglong *)(extraout_RAX_01 + 0x40);
          unaff_R13 = 1;
          pvVar9 = (void *)0x0;
        }
      }
      else {
        local_80 = (tagRECT *)CONCAT71(local_80._1_7_,(char)_Src);
        uVar5 = *(uint6 *)(extraout_RAX_01 + 0x48);
        local_d0 = *(ulonglong *)(extraout_RAX_01 + 0x40);
        local_70 = extraout_RAX_01;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        memcpy(_Dst,pvVar9,0xdc);
        in_RCX = (ulonglong)uVar5;
        pcVar6 = local_70;
        pvVar9 = _Dst;
        uVar10 = local_d0;
        _Src = local_80;
      }
    }
    if (((((ulonglong)_Src & 1) == 0) &&
        ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
         & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
      pcVar6[1] = '\x01';
    }
    LOCK();
    cVar1 = *pcVar6;
    *pcVar6 = '\0';
    UNLOCK();
    if (cVar1 == '\x02') {
      std::sys::sync::mutex::futex::Mutex::wake();
    }
    if (pvStack_200 != (void *)0x0 && local_208 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_1f8 = (ulonglong)(unaff_R13 != 0);
    if (pvVar9 != (void *)0x0) {
      local_1f8 = unaff_R13;
    }
    local_208 = ppuVar3;
    pvStack_200 = pvVar9;
    pHStack_1f0 = (HWND)uVar10;
    local_1e8 = in_RCX;
    memcpy(local_378,&local_208,0x130);
    local_61 = 1;
    winit::platform_impl::platform::monitor::primary_monitor();
    local_61 = 1;
    winit::platform_impl::platform::monitor::get_monitor_info();
    if ((int)local_208 != 1) {
      local_61 = 1;
      winit::dpi::impl$2::from_f64();
      local_61 = 1;
      winit::dpi::impl$2::from_f64();
      local_61 = 1;
      winit::platform_impl::platform::monitor::get_monitor_info();
      if ((((ulonglong)local_208 & 1) != 0) && (((uint)pvStack_200 & 3) == 1)) {
        local_80 = (tagRECT *)((longlong)pvStack_200 + -1);
        local_78 = *(undefined8 *)((longlong)pvStack_200 + -1);
        local_70 = *(char **)((longlong)pvStack_200 + 7);
        if (*(code **)local_70 != (code *)0x0) {
          (**(code **)local_70)(local_78);
        }
        if (*(longlong *)(local_70 + 8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_61 = 1;
      winit::dpi::impl$5::from_f64();
      local_61 = 1;
      winit::dpi::impl$5::from_f64();
      memcpy(&local_208,local_378,0x130);
      local_62 = 1;
      winit::dpi::impl$5::from_f64();
      local_62 = 1;
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      local_180 = 0;
      memcpy(local_378,&local_208,0x130);
      memcpy(&local_208,local_378,0xe8);
      local_88 = local_250;
      local_98 = local_260;
      uStack_90 = uStack_258;
      local_a8 = local_270;
      uStack_a0 = uStack_268;
      local_b8 = local_280;
      uStack_b0 = uStack_278;
      local_c8 = (void *)local_290;
      pHStack_c0 = (HWND)uStack_288;
      local_61 = 0;
      winit::platform_impl::platform::window::init<tuple_<>_>();
      if ((local_3a0 & 1) != 0) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_218 = local_388;
      uStack_210 = uStack_380;
      local_228 = local_398;
      pHStack_220 = pHStack_390;
      ppuVar3 = (undefined **)(local_398 + 0x10);
      LOCK();
      bVar11 = *(char *)(local_398 + 0x10) == '\0';
      if (bVar11) {
        *(char *)(local_398 + 0x10) = '\x01';
      }
      UNLOCK();
      if (!bVar11) {
        std::sys::sync::mutex::futex::Mutex::lock_contended();
      }
      puVar2 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
      if ((*(ulonglong *)
            __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
          0x7fffffffffffffff) == 0) {
        bVar7 = 0;
        cVar1 = *(char *)(local_398 + 0x11);
      }
      else {
        std::panicking::panic_count::is_zero_slow_path();
        bVar7 = extraout_AL ^ 1;
        cVar1 = *(char *)(local_398 + 0x11);
      }
      if (cVar1 != '\0') {
        uVar10 = (ulonglong)pvStack_200 >> 8;
        pvStack_200 = (void *)CONCAT71((int7)uVar10,bVar7);
        local_208 = ppuVar3;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      *(undefined1 *)(local_398 + 0xe4) = 1;
      if (((bVar7 == 0) && ((*(ulonglong *)puVar2 & 0x7fffffffffffffff) != 0)) &&
         (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
        *(undefined1 *)(local_398 + 0x11) = 1;
      }
      LOCK();
      cVar1 = *(char *)ppuVar3;
      *(char *)ppuVar3 = '\0';
      UNLOCK();
      if (cVar1 == '\x02') {
        std::sys::sync::mutex::futex::Mutex::wake();
      }
      RedrawWindow(pHStack_220,(RECT *)0x0,(HRGN)0x0,2);
      local_b8 = local_218;
      uStack_b0 = uStack_210;
      local_c8 = (void *)local_228;
      pHStack_c0 = pHStack_220;
      local_1e8 = local_218;
      lStack_1e0 = uStack_210;
      local_1f8 = local_228;
      pHStack_1f0 = pHStack_220;
      local_208 = (undefined **)0x1;
      pvStack_200 = (void *)0x1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 != (undefined8 *)0x0) {
        extraout_RAX_02[4] = local_1e8;
        extraout_RAX_02[5] = lStack_1e0;
        extraout_RAX_02[2] = local_1f8;
        extraout_RAX_02[3] = pHStack_1f0;
        *extraout_RAX_02 = local_208;
        extraout_RAX_02[1] = pvStack_200;
        return;
      }
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    local_c8 = pvStack_200;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

