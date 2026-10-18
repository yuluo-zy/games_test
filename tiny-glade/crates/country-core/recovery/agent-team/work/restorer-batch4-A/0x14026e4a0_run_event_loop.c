// run_event_loop
// sig: void __cdecl run_event_loop(void)
// size: 2447

void __cdecl tiny_glade::event_loop::run_event_loop(void)

{
  char cVar1;
  code *pcVar2;
  undefined4 *puVar3;
  longlong *plVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined1 extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  ushort uVar12;
  int extraout_EAX;
  undefined4 extraout_EAX_00;
  undefined4 extraout_EAX_01;
  undefined4 extraout_EAX_02;
  LONG extraout_EAX_03;
  LONG extraout_EAX_04;
  BOOL BVar13;
  uint uVar14;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  tagPOINT extraout_RAX_04;
  longlong *extraout_RAX_05;
  char *extraout_RAX_06;
  tagPOINT tVar15;
  longlong *extraout_RAX_07;
  undefined8 extraout_RAX_08;
  ulonglong extraout_RAX_09;
  uint uVar16;
  longlong in_RCX;
  LONG LVar17;
  int iVar18;
  longlong *in_RDX;
  tagPOINT tVar19;
  tagPOINT tVar20;
  LPPOINT ptVar21;
  tagPOINT tVar22;
  tagPOINT tVar23;
  longlong in_R8;
  bool bVar24;
  bool bVar25;
  longlong in_stack_00000030;
  tagPOINT tStack_3d0;
  code *pcStack_3c8;
  undefined8 uStack_3c0;
  undefined4 *puStack_3b8;
  tagPOINT local_3b0;
  tagPOINT tStack_3a8;
  LONG local_3a0;
  LONG LStack_39c;
  LONG LStack_398;
  LONG LStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined4 uStack_388;
  undefined8 uStack_384;
  undefined8 uStack_37c;
  undefined8 uStack_374;
  undefined8 uStack_36c;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined8 uStack_35c;
  undefined8 uStack_354;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined8 uStack_340;
  tagPOINT tStack_338;
  code *pcStack_330;
  undefined4 local_324 [3];
  undefined8 uStack_318;
  undefined8 local_310;
  LONG local_308;
  undefined8 local_300;
  undefined1 local_2f8 [8];
  longlong local_2f0;
  tagPOINT local_2e0;
  tagPOINT tStack_2d8;
  LONG LStack_2d0;
  LONG LStack_2cc;
  undefined8 uStack_2c8;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined4 uStack_2b8;
  undefined4 uStack_2b4;
  undefined4 uStack_2b0;
  undefined4 uStack_2ac;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined2 uStack_2a0;
  undefined2 uStack_29e;
  char cStack_29c;
  byte bStack_29b;
  undefined2 uStack_29a;
  LONG LStack_298;
  LONG LStack_294;
  LONG LStack_290;
  LONG LStack_28c;
  undefined8 uStack_288;
  undefined8 uStack_280;
  tagPOINT local_278;
  undefined8 local_270;
  tagPOINT local_268;
  tagPOINT local_260;
  tagPOINT local_258;
  LONG LStack_250;
  LONG LStack_24c;
  undefined8 uStack_248;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 uStack_1f8;
  tagPOINT tStack_1d8;
  code *pcStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined4 local_1b4;
  undefined1 local_1b0 [16];
  longlong local_1a0;
  longlong local_198;
  tagPOINT local_190;
  tagPOINT local_188;
  LONG local_180;
  LONG local_17c;
  tagPOINT local_178;
  longlong local_170;
  longlong local_168;
  tagPOINT tStack_160;
  tagPOINT local_158;
  tagPOINT local_150;
  tagPOINT tStack_148;
  tagPOINT local_140;
  undefined4 *puStack_138;
  undefined8 *puStack_130;
  char *local_128;
  char *local_120;
  tagPOINT *local_118;
  undefined8 local_110;
  undefined **local_108;
  undefined8 local_100;
  tagPOINT *local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  uint local_cc;
  char *pcStack_c8;
  int local_bc;
  longlong *local_b8;
  undefined1 uStack_aa;
  char local_a9;
  undefined8 uStack_a8;
  tagPOINT tStack_a0;
  char local_95 [2];
  undefined1 uStack_93;
  undefined1 uStack_92;
  undefined1 uStack_91;
  undefined8 local_90;
  
  puVar3 = (undefined4 *)CONCAT44(uStack_38c,uStack_390);
  tVar23.y = LStack_39c;
  tVar23.x = local_3a0;
  local_90 = 0xfffffffffffffffe;
  local_2f0 = in_stack_00000030;
  local_1b4 = 0;
  local_1a0 = in_RCX;
  local_198 = in_R8;
  local_b8 = in_RDX;
  winit::platform_impl::platform::monitor::current_monitor();
  LVar17 = (LONG)in_RDX;
  local_300 = 1;
  local_95[0] = '\x01';
  local_324[0] = 2;
  local_178.x = 0;
  local_178.y = 0;
  local_168 = 0;
  tStack_160.x = 0;
  tStack_160.y = 0;
  local_170 = 8;
  local_cc = 0;
  local_190.x = 0;
  local_190.y = 0;
  local_180 = 0;
  std::time::Instant::now();
  local_a9 = '\x01';
  local_278 = (tagPOINT)(local_198 + 8);
  local_bc = 0;
  local_270 = extraout_RAX;
  local_188 = local_278;
  local_17c = LVar17;
  do {
    LStack_39c = tVar23.y;
    local_3a0 = tVar23.x;
    uVar5 = uStack_23c;
    uVar6 = uStack_238;
    uVar7 = uStack_234;
    uVar8 = uStack_230;
    uVar9 = uStack_22c;
    uVar10 = uStack_228;
    uVar11 = uStack_224;
    if (local_a9 == '\0') {
LAB_14026fa20:
      uStack_224 = uVar11;
      uStack_228 = uVar10;
      uStack_22c = uVar9;
      uStack_230 = uVar8;
      uStack_234 = uVar7;
      uStack_238 = uVar6;
      uStack_23c = uVar5;
      if (local_178 != (tagPOINT)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::
      drop_in_place<std::sync::mpsc::Receiver<enum2_<tiny_glade::event_loop::GameEvent>_>_>();
      core::ptr::
      drop_in_place<std::sync::mpsc::Sender<enum2_<tiny_glade::event_loop::OsWindowEvent>_>_>();
      core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>();
      LOCK();
      *local_b8 = *local_b8 + -1;
      UNLOCK();
      if (*local_b8 == 0) {
        alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>();
      }
      core::ptr::drop_in_place<winit::event_loop::EventLoop<tuple_<>_>_>();
      return;
    }
    *(undefined4 *)(*(longlong *)(local_1a0 + 0x10) + 0x18) = 1000000000;
    local_158 = local_278;
    local_150 = (tagPOINT)&local_b8;
    tStack_148 = (tagPOINT)local_1b0;
    local_140 = (tagPOINT)&local_a9;
    puStack_138 = &local_1b4;
    puStack_130 = &local_300;
    local_128 = local_95;
    local_120 = (char *)local_324;
    local_118 = &local_190;
    local_110 = &local_cc;
    local_108 = (undefined **)&local_180;
    winit::platform_impl::platform::event_loop::EventLoop::
    pump_events<tuple_<>,tiny_glade::event_loop::run_event_loop::closure_env_0>();
    uVar5 = uStack_23c;
    uVar6 = uStack_238;
    uVar7 = uStack_234;
    uVar8 = uStack_230;
    uVar9 = uStack_22c;
    uVar10 = uStack_228;
    uVar11 = uStack_224;
    if (extraout_EAX != 0) goto LAB_14026fa20;
    tVar15.x = 4;
    tVar15.y = 0;
    bVar24 = false;
    while( true ) {
      if (local_2f0 == 0) {
        std::sync::mpmc::array::Channel::try_recv<enum2_<tiny_glade::event_loop::GameEvent>_>();
      }
      else if ((int)local_2f0 == 1) {
        std::sync::mpmc::list::Channel::try_recv<enum2_<tiny_glade::event_loop::GameEvent>_>();
      }
      else {
        std::sync::mpmc::zero::Channel::try_recv<enum2_<tiny_glade::event_loop::GameEvent>_>();
      }
      plVar4 = local_b8;
      uVar11 = uStack_224;
      uVar10 = uStack_228;
      uVar9 = uStack_22c;
      uVar8 = uStack_230;
      uVar7 = uStack_234;
      uVar6 = uStack_238;
      uVar5 = uStack_23c;
      if (local_3b0 == (tagPOINT)0x8000000000000007) break;
      uStack_22c = (undefined4)uStack_374;
      uStack_228 = (undefined4)((ulonglong)uStack_374 >> 0x20);
      uStack_224 = (undefined4)uStack_36c;
      uStack_23c = (undefined4)uStack_384;
      uStack_238 = (undefined4)((ulonglong)uStack_384 >> 0x20);
      uStack_234 = (undefined4)uStack_37c;
      uStack_230 = (undefined4)((ulonglong)uStack_37c >> 0x20);
      switch((ulonglong)local_3b0 ^ 0x8000000000000000) {
      case 0:
        local_188 = (tagPOINT)((ulonglong)tStack_3a8 >> 0x20);
        tVar15 = (tagPOINT)((ulonglong)tStack_3a8 & 0xffffffff);
        uStack_23c = uVar5;
        uStack_238 = uVar6;
        uStack_234 = uVar7;
        uStack_230 = uVar8;
        uStack_22c = uVar9;
        uStack_228 = uVar10;
        uStack_224 = uVar11;
        break;
      case 1:
        if (local_3a0 == 0) {
          uStack_23c = uVar5;
          uStack_238 = uVar6;
          uStack_234 = uVar7;
          uStack_230 = uVar8;
          uStack_22c = uVar9;
          uStack_228 = uVar10;
          uStack_224 = uVar11;
          winit::window::Window::set_cursor_grab();
          if ((local_158.x == 2) && ((local_140.x & 3U) == 1)) {
            pcStack_c8 = (char *)((longlong)local_140 + -1);
            tStack_a0 = *(tagPOINT *)((longlong)local_140 + -1);
            uStack_a8 = *(undefined8 **)((longlong)local_140 + 7);
            if ((code *)*uStack_a8 != (code *)0x0) {
              (*(code *)*uStack_a8)(tStack_a0);
            }
            if (uStack_a8[1] != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          local_324[0] = 0;
          uStack_318 = CONCAT44(LStack_398,LStack_39c);
          if (tStack_160 == local_178) goto code_r0x00014026f378;
        }
        else if (local_3a0 == 1) {
          uStack_23c = uVar5;
          uStack_238 = uVar6;
          uStack_234 = uVar7;
          uStack_230 = uVar8;
          uStack_22c = uVar9;
          uStack_228 = uVar10;
          uStack_224 = uVar11;
          winit::window::Window::set_cursor_grab();
          if ((local_158.x == 2) && ((local_140.x & 3U) == 1)) {
            pcStack_c8 = (char *)((longlong)local_140 + -1);
            tStack_a0 = *(tagPOINT *)((longlong)local_140 + -1);
            uStack_a8 = *(undefined8 **)((longlong)local_140 + 7);
            if ((code *)*uStack_a8 != (code *)0x0) {
              (*(code *)*uStack_a8)(tStack_a0);
            }
            if (uStack_a8[1] != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          local_324[0] = 2;
          bVar24 = false;
          if (tStack_160 == local_178) {
code_r0x00014026f378:
            alloc::collections::vec_deque::VecDeque::
            grow<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>();
          }
        }
        else {
          bVar24 = true;
          uStack_23c = uVar5;
          uStack_238 = uVar6;
          uStack_234 = uVar7;
          uStack_230 = uVar8;
          uStack_22c = uVar9;
          uStack_228 = uVar10;
          uStack_224 = uVar11;
          if (tStack_160 == local_178) goto code_r0x00014026f378;
        }
        tVar19.x = 0;
        tVar19.y = 0;
        if ((ulonglong)local_178 <= (ulonglong)((longlong)tStack_160 + local_168)) {
          tVar19 = local_178;
        }
        *(tagPOINT *)(local_170 + (((longlong)tStack_160 + local_168) - (longlong)tVar19) * 8) =
             tStack_3a8;
        tStack_160 = (tagPOINT)((longlong)tStack_160 + 1);
        break;
      case 2:
        tVar20 = *(tagPOINT *)(local_b8 + 3);
        tVar19 = *(tagPOINT *)(local_b8 + 2);
        LOCK();
        tVar22 = *(tagPOINT *)tVar19;
        *(longlong *)tVar19 = (longlong)*(tagPOINT *)tVar19 + 1;
        UNLOCK();
        if (*(tagPOINT *)tVar19 == (tagPOINT)0x0 ||
            SCARRY8((longlong)tVar22,1) != (longlong)*(tagPOINT *)tVar19 < 0) {
                    // WARNING: Does not return
          pcVar2 = (code *)invalidInstructionException();
          uStack_23c = uVar5;
          uStack_238 = uVar6;
          uStack_234 = uVar7;
          uStack_230 = uVar8;
          uStack_22c = uVar9;
          uStack_228 = uVar10;
          uStack_224 = uVar11;
          (*pcVar2)();
        }
        uStack_aa = 1;
        local_268 = tVar19;
        uStack_23c = uVar5;
        uStack_238 = uVar6;
        uStack_234 = uVar7;
        uStack_230 = uVar8;
        uStack_22c = uVar9;
        uStack_228 = uVar10;
        uStack_224 = uVar11;
        tStack_a0 = tVar19;
        winit::platform_impl::platform::util::is_minimized();
        local_150 = tStack_a0;
        tStack_148.x._1_1_ = 1;
        tStack_148.x._0_1_ = extraout_AL;
        uStack_aa = 0;
        local_158 = tVar20;
        winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::
        execute_in_thread<winit::platform_impl::platform::window::impl_0::set_minimized::closure_env_0>
                  ();
        break;
      default:
        local_268 = local_3b0;
        local_260 = tStack_3a8;
        LStack_250 = LStack_398;
        uStack_1fc = uStack_344;
        uStack_1f8 = uStack_340;
        uStack_20c = (undefined4)uStack_354;
        uStack_208 = (undefined4)((ulonglong)uStack_354 >> 0x20);
        uStack_204 = uStack_34c;
        uStack_200 = uStack_348;
        uStack_21c = uStack_364;
        uStack_218 = uStack_360;
        uStack_214 = (undefined4)uStack_35c;
        uStack_210 = (undefined4)((ulonglong)uStack_35c >> 0x20);
        uStack_220 = (undefined4)((ulonglong)uStack_36c >> 0x20);
        LStack_24c = LStack_394;
        uStack_240 = uStack_388;
        tVar20 = *(tagPOINT *)local_278;
        tVar22 = (tagPOINT)((longlong)tVar20 + 0x10);
        LOCK();
        bVar25 = *(char *)((longlong)tVar20 + 0x10) == '\0';
        if (bVar25) {
          *(char *)((longlong)tVar20 + 0x10) = '\x01';
        }
        UNLOCK();
        local_258 = tVar23;
        uStack_248 = puVar3;
        if (!bVar25) {
          uStack_93 = 1;
          std::sys::sync::mutex::futex::Mutex::lock_contended();
        }
        if ((*(ulonglong *)
              __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
            0x7fffffffffffffff) == 0) {
          uStack_a8 = (undefined8 *)((ulonglong)uStack_a8 & 0xffffffff00000000);
          cVar1 = *(char *)((longlong)tVar20 + 0x11);
        }
        else {
          uStack_93 = 1;
          std::panicking::panic_count::is_zero_slow_path();
          uStack_a8 = (undefined8 *)(CONCAT44(uStack_a8._4_4_,extraout_EAX_02) ^ 1);
          cVar1 = *(char *)((longlong)tVar20 + 0x11);
        }
        tStack_a0 = tVar22;
        if (cVar1 != '\0') {
          local_150.x._0_1_ = (char)uStack_a8;
          local_158 = tVar22;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        local_f8 = (tagPOINT *)CONCAT44(uStack_204,uStack_208);
        local_f0 = CONCAT44(uStack_1fc,uStack_200);
        local_108 = (undefined **)CONCAT44(uStack_214,uStack_218);
        local_100 = CONCAT44(uStack_20c,uStack_210);
        local_118 = (tagPOINT *)CONCAT44(uStack_224,uStack_228);
        local_110 = (uint *)CONCAT44(uStack_21c,uStack_220);
        local_140.y = LStack_24c;
        local_140.x = LStack_250;
        puStack_130 = (undefined8 *)CONCAT44(uStack_23c,uStack_240);
        local_128 = (char *)CONCAT44(uStack_234,uStack_238);
        local_120 = (char *)CONCAT44(uStack_22c,uStack_230);
        local_158 = local_268;
        local_150 = local_260;
        tStack_148 = local_258;
        puStack_138 = uStack_248;
        local_e8 = uStack_1f8;
        egui_winit::State::handle_platform_output();
        tVar22 = tStack_a0;
        if (((char)uStack_a8 == '\0') &&
           ((*(ulonglong *)
              __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
            0x7fffffffffffffff) != 0)) {
          uStack_93 = 0;
          std::panicking::panic_count::is_zero_slow_path();
          if (extraout_AL_00 == '\0') {
            *(undefined1 *)((longlong)tVar20 + 0x11) = 1;
          }
        }
        LOCK();
        cVar1 = *(char *)tVar22;
        *(char *)tVar22 = '\0';
        UNLOCK();
        if (cVar1 == '\x02') {
          uStack_93 = 0;
          std::sys::sync::mutex::futex::Mutex::wake();
        }
        break;
      case 4:
        local_2e0 = tStack_3a8;
        LStack_2d0 = LStack_398;
        cStack_29c = (char)uStack_364;
        bStack_29b = (byte)((uint)uStack_364 >> 8);
        uStack_29a = (undefined2)((uint)uStack_364 >> 0x10);
        uStack_2a0 = (undefined2)((ulonglong)uStack_36c >> 0x20);
        uStack_29e = (undefined2)((ulonglong)uStack_36c >> 0x30);
        LStack_2cc = LStack_394;
        uStack_2c0 = uStack_388;
        uStack_91 = 1;
        uStack_2bc = uStack_23c;
        uStack_2b8 = uStack_238;
        uStack_2b4 = uStack_234;
        uStack_2b0 = uStack_230;
        uStack_2ac = uStack_22c;
        uStack_2a8 = uStack_228;
        uStack_2a4 = uStack_224;
        uStack_23c = uVar5;
        uStack_238 = uVar6;
        uStack_234 = uVar7;
        uStack_230 = uVar8;
        uStack_22c = uVar9;
        uStack_228 = uVar10;
        uStack_224 = uVar11;
        tStack_2d8 = tVar23;
        uStack_2c8 = puVar3;
        winit::platform_impl::platform::monitor::available_monitors();
        tVar22 = (tagPOINT)&local_2e0;
        alloc::collections::vec_deque::into_iter::impl$2::
        try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple_<>,core::iter::adapters::map::map_try_fold::closure_env_0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple_<>,enum2_<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple_<>_>_>,winit::window::impl_14::available_monitors::closure_0::closure_env_0,core::iter::traits::iterator::Iterator::find::check::closure_env_0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env_2>_>,enum2_<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple_<>_>_>_>
                  ();
        if ((extraout_RAX_00 & 1) == 0) {
          winit::platform_impl::platform::monitor::current_monitor();
          tVar22 = extraout_RAX_04;
        }
        if (local_158 != (tagPOINT)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (cStack_29c == '\0') {
          local_158.x = 0;
          local_158.y = 0;
          uStack_91 = 1;
          winit::window::Window::try_set_fullscreen();
          tStack_2d8 = tStack_2d8;
          uStack_2c8 = uStack_2c8;
          if (CONCAT44(LStack_294,LStack_298) != 0) {
            local_258.x = (LONG)uStack_288;
            local_258.y = (LONG)((ulonglong)uStack_288 >> 0x20);
            LStack_250 = (LONG)uStack_280;
            LStack_24c = (LONG)((ulonglong)uStack_280 >> 0x20);
            local_268.y = LStack_294;
            local_268.x = LStack_298;
            local_260.y = LStack_28c;
            local_260.x = LStack_290;
            if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
              tStack_1d8 = (tagPOINT)&local_268;
              pcStack_1d0 = winit::error::impl$14::fmt;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              puStack_138 = (undefined4 *)*extraout_RAX_05;
              puStack_130 = (undefined8 *)extraout_RAX_05[1];
              local_128 = (char *)0x1;
              local_120 = "tiny_glade::event_loopApplying exclusive fullscreen mode: ";
              local_118 = (tagPOINT *)0x16;
              local_108 = (undefined **)&UNK_1429a7b58;
              local_100 = 1;
              local_f8 = &tStack_1d8;
              local_f0 = 1;
              local_e8 = 0;
              local_158.x = 0;
              local_158.y = 0;
              local_150 = (tagPOINT)0x1429a7b80;
              tStack_148.x = 0x16;
              tStack_148.y = 0;
              local_140.x = 0;
              local_140.y = 0;
              local_110 = (uint *)CONCAT44((int)extraout_RAX_05[2],1);
              ::log::__private_api::impl$2::log();
            }
            if ((local_258.x & 3U) == 1) {
              pcStack_c8 = (char *)((longlong)local_258 + -1);
              tStack_a0 = *(tagPOINT *)((longlong)local_258 + -1);
              uStack_a8 = *(undefined8 **)((longlong)local_258 + 7);
              if ((code *)*uStack_a8 != (code *)0x0) {
                (*(code *)*uStack_a8)(tStack_a0);
              }
              goto code_r0x00014026f1eb;
            }
          }
        }
        else {
          if ((bStack_29b & 1) != 0) {
            uStack_91 = 1;
            setup::find_best_fullscreen_video_mode();
            if (tStack_3d0 != (tagPOINT)0x0) {
              uStack_1c8 = uStack_3c0;
              uStack_1c0 = puStack_3b8;
              tStack_1d8 = tStack_3d0;
              pcStack_1d0 = pcStack_3c8;
              uStack_92 = 1;
              winit::platform_impl::platform::monitor::VideoMode::size();
              uStack_92 = 1;
              winit::dpi::impl$2::from_f64();
              uStack_92 = 1;
              winit::dpi::impl$2::from_f64();
              uStack_2a4 = (undefined4)uStack_1c0;
              uStack_2a0 = uStack_1c0._4_2_;
              uStack_2b0 = 1;
              uStack_2ac = extraout_EAX_00;
              uStack_2a8 = extraout_EAX_01;
              if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
                local_268 = (tagPOINT)&tStack_1d8;
                local_260 = (tagPOINT)winit::monitor::impl$4::fmt;
                uStack_92 = 1;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                puStack_138 = (undefined4 *)*extraout_RAX_01;
                puStack_130 = (undefined8 *)extraout_RAX_01[1];
                local_128 = (char *)0x3;
                local_120 = "tiny_glade::event_loopApplying exclusive fullscreen mode: ";
                local_118 = (tagPOINT *)0x16;
                local_108 = (undefined **)&UNK_1429a7bc0;
                local_100 = 1;
                local_f8 = &local_268;
                local_f0 = 1;
                local_e8 = 0;
                local_158.x = 0;
                local_158.y = 0;
                local_150 = (tagPOINT)0x1429a7b80;
                tStack_148.x = 0x16;
                tStack_148.y = 0;
                local_140.x = 0;
                local_140.y = 0;
                local_110 = (uint *)CONCAT44(*(undefined4 *)(extraout_RAX_01 + 2),1);
                uStack_92 = 1;
                ::log::__private_api::impl$2::log();
              }
              local_140.x = (undefined4)uStack_1c8;
              local_140.y = uStack_1c8._4_4_;
              puStack_138 = uStack_1c0;
              local_150 = tStack_1d8;
              tStack_148 = (tagPOINT)pcStack_1d0;
              local_158.x = 1;
              local_158.y = 0;
              uStack_92 = 0;
              winit::window::Window::try_set_fullscreen();
              tStack_2d8 = tStack_2d8;
              uStack_2c8 = uStack_2c8;
              if (CONCAT44(LStack_294,LStack_298) == 0) goto code_r0x00014026f222;
              local_258.x = (LONG)uStack_288;
              local_258.y = (LONG)((ulonglong)uStack_288 >> 0x20);
              LStack_250 = (LONG)uStack_280;
              LStack_24c = (LONG)((ulonglong)uStack_280 >> 0x20);
              local_268.y = LStack_294;
              local_268.x = LStack_298;
              local_260.y = LStack_28c;
              local_260.x = LStack_290;
              bStack_29b = 0;
              if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
                tStack_338 = (tagPOINT)&local_268;
                pcStack_330 = winit::error::impl$14::fmt;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                puStack_138 = (undefined4 *)*extraout_RAX_02;
                puStack_130 = (undefined8 *)extraout_RAX_02[1];
                local_128 = (char *)0x1;
                local_120 = "tiny_glade::event_loopApplying exclusive fullscreen mode: ";
                local_118 = (tagPOINT *)0x16;
                local_108 = (undefined **)&UNK_1429a7c10;
                local_100 = 1;
                local_f8 = &tStack_338;
                local_f0 = 1;
                local_e8 = 0;
                local_158.x = 0;
                local_158.y = 0;
                local_150 = (tagPOINT)0x1429a7b80;
                tStack_148 = (tagPOINT)0x16;
                local_140.x = 0;
                local_140.y = 0;
                local_110 = (uint *)CONCAT44((int)extraout_RAX_02[2],1);
                ::log::__private_api::impl$2::log();
              }
              if ((local_258.x & 3U) == 1) {
                pcStack_c8 = (char *)((longlong)local_258 + -1);
                tStack_a0 = *(tagPOINT *)((longlong)local_258 + -1);
                uStack_a8 = *(undefined8 **)((longlong)local_258 + 7);
                if ((code *)*uStack_a8 != (code *)0x0) {
                  (*(code *)*uStack_a8)(tStack_a0);
                }
                if (uStack_a8[1] != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
          }
          local_150.x = 0;
          local_150.y = 0;
          tStack_148.x = 1;
          tStack_148.y = 0;
          local_158.x = 1;
          local_158.y = 0;
          uStack_91 = 1;
          local_140 = tVar22;
          winit::window::Window::try_set_fullscreen();
          tStack_2d8 = tStack_2d8;
          uStack_2c8 = uStack_2c8;
          if (CONCAT44(LStack_294,LStack_298) != 0) {
            local_258.x = (LONG)uStack_288;
            local_258.y = (LONG)((ulonglong)uStack_288 >> 0x20);
            LStack_250 = (LONG)uStack_280;
            LStack_24c = (LONG)((ulonglong)uStack_280 >> 0x20);
            local_268.y = LStack_294;
            local_268.x = LStack_298;
            local_260.y = LStack_28c;
            local_260.x = LStack_290;
            cStack_29c = '\0';
            if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
              tStack_1d8 = (tagPOINT)&local_268;
              pcStack_1d0 = winit::error::impl$14::fmt;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              puStack_138 = (undefined4 *)*extraout_RAX_03;
              puStack_130 = (undefined8 *)extraout_RAX_03[1];
              local_128 = (char *)0x1;
              local_120 = "tiny_glade::event_loopApplying exclusive fullscreen mode: ";
              local_118 = (tagPOINT *)0x16;
              local_108 = (undefined **)&UNK_1429a7c60;
              local_100 = 1;
              local_f8 = &tStack_1d8;
              local_f0 = 1;
              local_e8 = 0;
              local_158.x = 0;
              local_158.y = 0;
              local_150 = (tagPOINT)0x1429a7b80;
              tStack_148.x = 0x16;
              tStack_148.y = 0;
              local_140.x = 0;
              local_140.y = 0;
              local_110 = (uint *)CONCAT44((int)extraout_RAX_03[2],1);
              ::log::__private_api::impl$2::log();
            }
            if ((local_258.x & 3U) == 1) {
              pcStack_c8 = (char *)((longlong)local_258 + -1);
              tStack_a0 = *(tagPOINT *)((longlong)local_258 + -1);
              uStack_a8 = *(undefined8 **)((longlong)local_258 + 7);
              if ((code *)*uStack_a8 != (code *)0x0) {
                (*(code *)*uStack_a8)(tStack_a0);
              }
code_r0x00014026f1eb:
              if (uStack_a8[1] != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              std::alloc::__default_lib_allocator::__rust_dealloc();
              tStack_2d8 = tStack_2d8;
              uStack_2c8 = uStack_2c8;
            }
          }
        }
code_r0x00014026f222:
        puStack_138 = uStack_2c8;
        tStack_148 = tStack_2d8;
        local_110 = (uint *)CONCAT26(uStack_29a,
                                     CONCAT15(bStack_29b,
                                              CONCAT14(cStack_29c,CONCAT22(uStack_29e,uStack_2a0))))
        ;
        local_140.y = LStack_2cc;
        local_140.x = LStack_2d0;
        puStack_130 = (undefined8 *)CONCAT44(uStack_2bc,uStack_2c0);
        local_128 = (char *)CONCAT44(uStack_2b4,uStack_2b8);
        local_120 = (char *)CONCAT44(uStack_2ac,uStack_2b0);
        local_118 = (tagPOINT *)CONCAT44(uStack_2a4,uStack_2a8);
        local_150 = local_2e0;
        local_158.x._0_2_ = 9;
        uStack_91 = 0;
        tStack_2d8 = tStack_148;
        uStack_2c8 = puStack_138;
        std::sync::mpmc::Sender::send<enum2_<tiny_glade::event_loop::OsWindowEvent>_>();
        if ((short)local_268.x != 0x12) {
          uVar12 = 7;
          if ((ushort)((short)local_268.x - 4U) < 0xe) {
            uVar12 = (short)local_268.x - 4U;
          }
          uVar5 = uStack_23c;
          uVar6 = uStack_238;
          uVar7 = uStack_234;
          uVar8 = uStack_230;
          uVar9 = uStack_22c;
          uVar10 = uStack_228;
          uVar11 = uStack_224;
          if (uVar12 < 0xd) {
            if ((0x1f5fU >> (uVar12 & 0x1f) & 1) != 0) goto LAB_14026fa20;
            if (uVar12 != 5) {
              uStack_91 = 0;
              core::ptr::drop_in_place<winit::event::KeyEvent>();
              uVar5 = uStack_23c;
              uVar6 = uStack_238;
              uVar7 = uStack_234;
              uVar8 = uStack_230;
              uVar9 = uStack_22c;
              uVar10 = uStack_228;
              uVar11 = uStack_224;
              goto LAB_14026fa20;
            }
            if ((longlong)local_260 < -0x7ffffffffffffffe) goto LAB_14026fa20;
          }
          if (local_260 != (tagPOINT)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
            uVar5 = uStack_23c;
            uVar6 = uStack_238;
            uVar7 = uStack_234;
            uVar8 = uStack_230;
            uVar9 = uStack_22c;
            uVar10 = uStack_228;
            uVar11 = uStack_224;
          }
          goto LAB_14026fa20;
        }
        break;
      case 5:
        goto LAB_14026fa20;
      case 6:
        local_190 = tStack_3a8;
        uStack_23c = uVar5;
        uStack_238 = uVar6;
        uStack_234 = uVar7;
        uStack_230 = uVar8;
        uStack_22c = uVar9;
        uStack_228 = uVar10;
        uStack_224 = uVar11;
      }
    }
    if ((char)tStack_3a8.x != '\0') goto LAB_14026fa20;
    if (bVar24) {
      tVar23 = tVar15;
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      winit::platform_impl::platform::window::Window::window_state_lock();
      if ((((ulonglong)tVar23 & 1) == 0) &&
         ((*(ulonglong *)
            __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
          0x7fffffffffffffff) != 0)) {
        std::panicking::panic_count::is_zero_slow_path();
        if (extraout_AL_01 == '\0') {
          extraout_RAX_06[1] = '\x01';
        }
      }
      LOCK();
      cVar1 = *extraout_RAX_06;
      *extraout_RAX_06 = '\0';
      UNLOCK();
      if (cVar1 == '\x02') {
        std::sys::sync::mutex::futex::Mutex::wake();
      }
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      local_158.y = extraout_EAX_04;
      local_158.x = extraout_EAX_03;
      BVar13 = ClientToScreen((HWND)plVar4[3],&local_158);
      if (BVar13 != 0) {
        BVar13 = SetCursorPos(local_158.x,local_158.y);
        if (BVar13 != 0) goto LAB_14026f50e;
      }
      std::sys::pal::windows::os::errno();
    }
LAB_14026f50e:
    iVar18 = tVar15.x;
    if (iVar18 != 4) {
      uVar14 = iVar18 - 2;
      if (1 < uVar14) {
        uVar14 = 2;
      }
      uVar16 = local_188.x;
      if (uVar14 == 0) {
        if (uVar16 == 0) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        uVar16 = local_cc | uVar16;
        bVar24 = local_cc == 0;
        tVar15 = local_188;
        local_cc = uVar16;
        if (bVar24) {
          tVar22.x = 0;
          tVar22.y = 0;
          if (local_bc != 0) goto LAB_14026f795;
LAB_14026f7e2:
          winit::window::Window::set_cursor_visible();
LAB_14026f959:
          local_bc = 0;
          tVar15 = tVar22;
        }
      }
      else {
        tVar22._1_7_ = tVar15._1_7_;
        if (uVar14 == 1) {
          if (uVar16 == 0) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          uVar14 = ~uVar16 & local_cc;
          bVar24 = local_cc != 0;
          local_cc = uVar14;
          if ((bVar24) && (tVar22.x._0_1_ = 1, tVar15 = tVar22, uVar14 == 0)) {
            if (local_bc == 0) goto LAB_14026f7e2;
LAB_14026f795:
            tVar15 = tVar22;
            if (local_190 != (tagPOINT)0x0) {
              ((tagPOINT *)((longlong)local_190 + 8))->x = tVar22.x & 0xff;
            }
          }
        }
        else if (local_bc == 0) {
          local_bc = iVar18;
          if (iVar18 == 1) {
            tVar15.x = 0;
            tVar15.y = 0;
            winit::window::Window::set_cursor_visible();
LAB_14026f910:
            local_180 = local_188.x;
            local_bc = 1;
            if (local_190 != (tagPOINT)0x0) {
              *(LONG *)((longlong)local_190 + 0xc) = local_180;
            }
          }
        }
        else {
          if (iVar18 == 0) {
            tVar22.x._0_1_ = local_cc == 0;
            winit::window::Window::set_cursor_visible();
            goto LAB_14026f959;
          }
          local_bc = iVar18;
          if (iVar18 == 1) goto LAB_14026f910;
        }
      }
    }
    tStack_3a8 = tStack_160;
    tStack_160.x = 0;
    tStack_160.y = 0;
    local_3b0 = (tagPOINT)&local_178;
    LStack_398 = 0;
    LStack_394 = 0;
    tVar23.x = 0;
    tVar23.y = 0;
    tVar22 = tStack_3a8;
    while( true ) {
      LVar17 = tVar15.x;
      bVar24 = tVar22 == (tagPOINT)0x0;
      tVar22 = (tagPOINT)((longlong)tVar22 + -1);
      if (bVar24) break;
      tVar15 = (tagPOINT)(local_168 + (longlong)tVar23);
      tVar20.x = 0;
      tVar20.y = 0;
      if ((ulonglong)local_178 <= (ulonglong)tVar15) {
        tVar20 = local_178;
      }
      tVar23 = (tagPOINT)((longlong)tVar23 + 1);
      local_150 = *(tagPOINT *)(local_170 + ((longlong)tVar15 - (longlong)tVar20) * 8);
      local_158.x._0_2_ = 0xf;
      tVar15 = (tagPOINT)local_1b0;
      std::sync::mpmc::Sender::send<enum2_<tiny_glade::event_loop::OsWindowEvent>_>();
      if ((short)local_268.x != 0x12) {
        uVar12 = 7;
        if ((ushort)((short)local_268.x - 4U) < 0xe) {
          uVar12 = (short)local_268.x - 4U;
        }
        if (uVar12 < 0xd) {
          if ((0x1f5fU >> (uVar12 & 0x1f) & 1) == 0) {
            if (uVar12 == 5) {
              if (-0x7fffffffffffffff < (longlong)local_260) goto LAB_14026faf6;
            }
            else {
              core::ptr::drop_in_place<winit::event::KeyEvent>();
            }
          }
        }
        else {
LAB_14026faf6:
          if (local_260 != (tagPOINT)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        if (tStack_160 != (tagPOINT)0x0) {
          alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::
          join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>
                    ();
        }
        local_168 = 0;
        tStack_160.x = 0;
        tStack_160.y = 0;
        uVar5 = uStack_23c;
        uVar6 = uStack_238;
        uVar7 = uStack_234;
        uVar8 = uStack_230;
        uVar9 = uStack_22c;
        uVar10 = uStack_228;
        uVar11 = uStack_224;
        goto LAB_14026fa20;
      }
    }
    if (tStack_160 != (tagPOINT)0x0) {
      tVar15 = tStack_3a8;
      alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::
      join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>();
      LVar17 = tVar15.x;
    }
    local_168 = 0;
    tStack_160.x = 0;
    tStack_160.y = 0;
    if (local_95[0] != '\0') {
      local_95[0] = '\0';
      if ((int)local_300 == 1) {
        LVar17 = (LONG)local_2f8;
        icc::get_monitor_icc_profile();
        if (SBORROW8(0,(longlong)local_3b0)) {
          local_2e0 = tStack_3a8;
          if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_268 = (tagPOINT)&local_2e0;
            local_260 = (tagPOINT)anyhow::error::impl$5::fmt;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            puStack_138 = (undefined4 *)*extraout_RAX_07;
            puStack_130 = (undefined8 *)extraout_RAX_07[1];
            local_128 = (char *)0x2;
            local_120 = "tiny_glade::event_loopApplying exclusive fullscreen mode: ";
            local_118 = (tagPOINT *)0x16;
            local_108 = &PTR_s_Failed_to_obtain_the_monitor_s_I_1429a7d10;
            local_100 = 1;
            local_f8 = &local_268;
            local_f0 = 1;
            local_e8 = 0;
            local_158.x = 0;
            local_158.y = 0;
            local_150 = (tagPOINT)0x1429a7b80;
            tStack_148.x = 0x16;
            tStack_148.y = 0;
            local_140.x = 0;
            local_140.y = 0;
            local_110 = (uint *)CONCAT44((int)extraout_RAX_07[2],1);
            ptVar21 = &local_158;
            ::log::__private_api::impl$2::log();
            LVar17 = (LONG)ptVar21;
          }
          anyhow::error::impl$6::drop();
        }
        else {
          puStack_138 = (undefined4 *)0x0;
          tStack_148 = tStack_3a8;
          local_158.x._0_2_ = 0x11;
          LVar17 = (LONG)local_1b0;
          local_150 = local_3b0;
          local_140 = tVar23;
          std::sync::mpmc::Sender::send<enum2_<tiny_glade::event_loop::OsWindowEvent>_>();
          if ((short)local_268.x != 0x12) {
            core::ptr::drop_in_place<enum2_<tiny_glade::event_loop::OsWindowEvent>_>();
            uVar5 = uStack_23c;
            uVar6 = uStack_238;
            uVar7 = uStack_234;
            uVar8 = uStack_230;
            uVar9 = uStack_22c;
            uVar10 = uStack_228;
            uVar11 = uStack_224;
            goto LAB_14026fa20;
          }
        }
      }
    }
    std::time::Instant::now();
    iVar18 = (int)local_270;
    local_310 = extraout_RAX_08;
    local_308 = LVar17;
    std::time::Instant::duration_since();
    bVar24 = iVar18 != 0;
    if (extraout_RAX_09 != 1) {
      bVar24 = 1 < extraout_RAX_09;
    }
    if (bVar24) {
      if (local_bc == 1) {
        winit::window::Window::set_cursor_visible();
      }
      local_270 = local_310;
      local_17c = local_308;
    }
    std::thread::sleep();
    puVar3 = (undefined4 *)0x0;
  } while( true );
}

