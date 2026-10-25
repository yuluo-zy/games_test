// setup_window
// sig: void __cdecl setup_window(void)
// size: 5072

void __cdecl tiny_glade::setup::setup_window(void)

{
  byte *pbVar1;
  HWND pHVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  int iVar6;
  ulonglong uVar7;
  undefined *puVar8;
  char extraout_AL;
  byte extraout_AL_00;
  byte bVar9;
  char extraout_AL_01;
  uint uVar10;
  uint extraout_EAX;
  undefined4 extraout_EAX_00;
  uint extraout_EAX_01;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong *extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong *extraout_RAX_06;
  HWND extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  HWND extraout_RAX_12;
  undefined8 *in_RCX;
  HWND pHVar11;
  ulonglong *_Src;
  longlong *plVar12;
  ulonglong in_RDX;
  HWND in_R8;
  char *pcVar13;
  size_t _Size;
  bool bVar14;
  ulonglong local_408;
  HWND local_400;
  ulonglong local_3f8;
  HWND pHStack_3f0;
  ulonglong local_3e8;
  undefined8 local_3e0;
  undefined8 local_3d0;
  undefined4 local_3c8;
  undefined4 local_3b0;
  undefined4 local_398;
  undefined4 local_380;
  undefined4 local_368;
  undefined4 local_350;
  undefined8 local_338;
  undefined4 local_330;
  undefined1 local_32c;
  undefined1 local_32b;
  undefined1 uStack_32a;
  undefined2 uStack_329;
  undefined1 uStack_327;
  undefined4 uStack_326;
  undefined1 uStack_322;
  undefined7 uStack_320;
  undefined1 local_319;
  undefined7 uStack_318;
  undefined1 uStack_311;
  undefined7 uStack_310;
  undefined1 local_309;
  undefined7 uStack_308;
  undefined1 uStack_301;
  undefined7 uStack_300;
  undefined1 local_2f9;
  undefined7 uStack_2f8;
  undefined1 uStack_2f1;
  undefined7 uStack_2f0;
  undefined1 local_2e9;
  undefined7 uStack_2e8;
  undefined1 uStack_2e1;
  undefined7 uStack_2e0;
  undefined1 uStack_2d9;
  HWND local_2c8;
  HWND pHStack_2c0;
  HWND local_2b8;
  HWND pHStack_2b0;
  HWND local_2a8;
  HWND pHStack_2a0;
  HWND local_298;
  HWND pHStack_290;
  HWND pHStack_288;
  HWND local_280;
  longlong local_268;
  ulonglong local_258;
  HWND pHStack_250;
  HWND local_248;
  HWND pHStack_240;
  HWND local_238;
  HWND pHStack_230;
  HWND local_228;
  longlong lStack_220;
  undefined8 *local_218;
  undefined8 uStack_210;
  longlong local_208;
  undefined8 uStack_200;
  undefined **local_1f8;
  undefined8 local_1f0;
  HWND *local_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined4 local_1c0;
  HWND local_178;
  undefined8 uStack_169;
  undefined7 local_160;
  undefined1 uStack_159;
  undefined7 uStack_158;
  undefined1 uStack_151;
  undefined7 local_150;
  undefined1 uStack_149;
  undefined7 uStack_148;
  undefined1 uStack_141;
  undefined7 local_140;
  undefined1 uStack_139;
  undefined7 uStack_138;
  undefined1 uStack_131;
  undefined7 local_130;
  undefined1 uStack_129;
  undefined7 uStack_128;
  undefined1 uStack_121;
  undefined7 local_120;
  undefined1 uStack_119;
  HWND local_118;
  HWND pHStack_110;
  HWND local_108;
  undefined8 uStack_100;
  HWND local_f8;
  longlong lStack_f0;
  undefined8 *local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  undefined8 uStack_d0;
  undefined **local_c8;
  HWND local_a8;
  HWND pHStack_a0;
  HWND local_98;
  HWND pHStack_90;
  undefined1 *local_80;
  undefined8 *local_78;
  HWND local_70;
  undefined1 local_65;
  undefined1 local_64;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_268 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x1f) {
          if ((char)(&DAT_14292d768)[in_RDX] < -0x40) {
LAB_14004372c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x1f) goto LAB_14004372c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x1f) {
          if ("ny_glade::setup::setup_window::f"[in_RDX] < -0x40) {
LAB_140043780:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x1f) goto LAB_140043780;
      }
    }
    pcVar13 = "exclusive fullscreen requires a monitor";
    do {
      bVar9 = pcVar13[-1];
      uVar10 = (uint)(char)bVar9;
      if ((char)bVar9 < '\0') {
        bVar3 = pcVar13[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = pcVar13[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)(pcVar13 + -4);
            pcVar13 = pcVar13 + -4;
            uVar10 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar13 = pcVar13 + -3;
            uVar10 = bVar4 & 0xf;
          }
          uVar10 = bVar3 & 0x3f | uVar10 << 6;
        }
        else {
          pcVar13 = pcVar13 + -2;
          uVar10 = bVar3 & 0x1f;
        }
        uVar10 = (uint)(bVar9 & 0x3f) | uVar10 << 6;
        if (uVar10 != 0x5c) goto LAB_140043844;
LAB_140043856:
        if (pcVar13 == "") break;
        if (pcVar13 + -0x14292d8af < (char *)0x1e) {
          if (-0x41 < pcVar13[1]) break;
        }
        else if (pcVar13 + -0x14292d8af == (char *)0x1e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar13 = pcVar13 + -1;
      if (uVar10 == 0x5c) goto LAB_140043856;
LAB_140043844:
      if (uVar10 == 0x2f) goto LAB_140043856;
    } while (pcVar13 != "crates/tiny-glade/src/setup.rsexclusive fullscreen requires a monitor");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      pHVar11 = (HWND)(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), pHVar11 = extraout_RAX_12, extraout_RAX_12 == (HWND)0x0))
    goto LAB_14004491e;
    if (*(HWND *)pHVar11 != (HWND)0x0) goto LAB_14004492a;
    *(HWND *)pHVar11 = (HWND)0xffffffffffffffff;
    local_70 = pHVar11;
    puffin::ThreadProfiler::begin_scope();
    *(HWND *)local_70 = (HWND)((longlong)&(*(HWND *)local_70)->unused + 1);
    local_268 = 1;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_02 = 0x69772074696e6977;
  *(undefined4 *)(extraout_RAX_02 + 1) = 0x776f646e;
  local_3c8 = 2;
  local_3b0 = 2;
  local_398 = 2;
  local_380 = 2;
  local_32c = 1;
  local_330 = 7;
  local_3e0 = 0xc;
  local_3d0 = 0xc;
  local_32b = 0;
  uStack_32a = 1;
  uStack_329 = 0;
  uStack_327 = 1;
  local_338 = 0;
  local_368 = 2;
  uStack_326 = 0x1020100;
  local_350 = 0xf;
  local_408 = 0;
  winit::platform_impl::platform::impl$0::default();
  memcpy(&local_248,&local_408,0xe8);
  local_160 = SUB87(local_118,0);
  uStack_159 = (undefined1)((ulonglong)local_118 >> 0x38);
  uStack_158 = SUB87(pHStack_110,0);
  uStack_151 = (undefined1)((ulonglong)pHStack_110 >> 0x38);
  local_150 = SUB87(local_108,0);
  uStack_149 = (undefined1)((ulonglong)local_108 >> 0x38);
  uStack_148 = SUB87(uStack_100,0);
  uStack_141 = (undefined1)((ulonglong)uStack_100 >> 0x38);
  local_140 = SUB87(local_f8,0);
  uStack_139 = (undefined1)((ulonglong)local_f8 >> 0x38);
  uStack_138 = (undefined7)lStack_f0;
  uStack_131 = (undefined1)((ulonglong)lStack_f0 >> 0x38);
  local_130 = SUB87(local_e8,0);
  uStack_129 = (undefined1)((ulonglong)local_e8 >> 0x38);
  uStack_128 = (undefined7)uStack_e0;
  uStack_121 = (undefined1)((ulonglong)uStack_e0 >> 0x38);
  local_120 = (undefined7)local_d8;
  uStack_119 = (undefined1)((ulonglong)local_d8 >> 0x38);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_03 = 0x616c4720796e6954;
  *(undefined2 *)(extraout_RAX_03 + 1) = 0x6564;
  if (lStack_220 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lStack_220 = 10;
  uStack_210 = 10;
  local_218 = extraout_RAX_03;
  memcpy(&local_408,&local_248,0xde);
  uStack_329 = (undefined2)uStack_169;
  uStack_327 = (undefined1)((ulonglong)uStack_169 >> 0x10);
  uStack_326 = (undefined4)((ulonglong)uStack_169 >> 0x18);
  uStack_322 = (undefined1)((ulonglong)uStack_169 >> 0x38);
  uStack_320 = local_160;
  local_319 = uStack_159;
  uStack_318 = uStack_158;
  uStack_311 = uStack_151;
  uStack_310 = local_150;
  local_309 = uStack_149;
  uStack_308 = uStack_148;
  uStack_301 = uStack_141;
  uStack_300 = local_140;
  local_2f9 = uStack_139;
  uStack_2f8 = uStack_138;
  uStack_2f1 = uStack_131;
  uStack_2f0 = local_130;
  local_2e9 = uStack_129;
  uStack_2e8 = uStack_128;
  uStack_2e1 = uStack_121;
  uStack_2e0 = local_120;
  uStack_2d9 = uStack_119;
  uStack_32a = 0;
  if (*(HWND *)in_R8 == (HWND)0x8000000000000001) {
LAB_140043c98:
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_61 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_228 = (HWND)*extraout_RAX_06;
      lStack_220 = extraout_RAX_06[1];
      local_218 = (undefined8 *)0x3;
      uStack_210 = 0x14292d950;
      local_208 = 0x11;
      local_1f8 = &PTR_s_Desired_monitor_not_found__using_14292da18;
      local_1f0 = 1;
      local_1e8 = (HWND *)&DAT_00000008;
      local_248 = (HWND)0x0;
      local_1e0 = 0;
      uStack_1d8 = 0;
      pHStack_240 = (HWND)((longlong)&DAT_14292d94f + 1);
      local_238 = (HWND)&DAT_00000011;
      pHStack_230 = (HWND)0x0;
      uStack_200 = CONCAT44((int)extraout_RAX_06[2],1);
      local_61 = 1;
      ::log::__private_api::impl$2::log();
    }
    local_61 = 1;
    winit::platform_impl::platform::monitor::primary_monitor();
    pHVar11 = extraout_RAX_07;
  }
  else {
    local_2a8 = in_R8;
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_118 = (HWND)&local_2a8;
      pHStack_110 = (HWND)core::fmt::impl$74::fmt<country_core::render::render_config::MonitorMatch>
      ;
      local_61 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_228 = (HWND)*extraout_RAX_04;
      lStack_220 = extraout_RAX_04[1];
      local_218 = (undefined8 *)0x3;
      uStack_210 = 0x14292d950;
      local_208 = 0x11;
      local_1f8 = &PTR_s_Attempting_to_place_window_on_mo_14292d9c0;
      local_1f0 = 1;
      local_1e0 = 1;
      uStack_1d8 = 0;
      local_248 = (HWND)0x0;
      pHStack_240 = (HWND)((longlong)&DAT_14292d94f + 1);
      local_238 = (HWND)&DAT_00000011;
      pHStack_230 = (HWND)0x0;
      uStack_200 = CONCAT44((int)extraout_RAX_04[2],1);
      local_61 = 1;
      local_1e8 = &local_118;
      ::log::__private_api::impl$2::log();
    }
    local_61 = 1;
    winit::platform_impl::platform::monitor::available_monitors();
    pHVar11 = local_2a8;
    alloc::collections::vec_deque::into_iter::impl$2::
    try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple_<>,core::iter::adapters::map::map_try_fold::closure_env_0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple_<>,enum2_<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple_<>_>_>,winit::event_loop::impl_9::available_monitors::closure_env_0<tuple_<>_>,core::iter::traits::iterator::Iterator::find::check::closure_env_0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure_0::closure_env_0>_>,enum2_<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple_<>_>_>_>
              ();
    if (local_248 != (HWND)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((extraout_RAX_05 & 1) == 0) goto LAB_140043c98;
  }
  local_258 = 1;
  local_61 = 1;
  pHStack_250 = pHVar11;
  impl$0::from_winit();
  local_f8 = local_228;
  local_108 = local_238;
  uStack_100 = pHStack_230;
  local_118 = local_248;
  pHStack_110 = pHStack_240;
  pHVar11 = *(HWND *)in_R8;
  if (((pHVar11 != (HWND)0x8000000000000001) && (!SBORROW8(0,(longlong)pHVar11))) &&
     (pHVar11 != (HWND)0x0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(HWND *)(in_R8 + 8) = local_f8;
  *(HWND *)(in_R8 + 4) = local_108;
  *(HWND *)(in_R8 + 6) = uStack_100;
  *(HWND *)in_R8 = local_118;
  *(HWND *)(in_R8 + 2) = pHStack_110;
  memcpy(&local_248,&local_408,0x130);
  winit::dpi::impl$2::from_f64();
  winit::dpi::impl$2::from_f64();
  local_208 = (ulonglong)extraout_EAX << 0x20;
  uStack_200 = CONCAT44(uStack_200._4_4_,extraout_EAX_00);
  memcpy(&local_408,&local_248,0x130);
  if ((char)in_R8[0x11].unused == '\x01') {
    if (*(char *)((longlong)&in_R8[0x11].unused + 1) == '\0') {
LAB_1400443e8:
      _Src = &local_3e8;
      bVar14 = local_408 != 0;
      memcpy(&local_248,_Src,0x110);
      local_3f8 = local_258;
      if (local_400 != (HWND)0x0 && bVar14) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_400 = (HWND)0x0;
      pHStack_3f0 = pHStack_250;
      _Size = 0x110;
    }
    else {
      if ((local_258 & 1) == 0) {
        local_61 = 1;
                    // WARNING: Subroutine does not return
        core::option::expect_failed();
      }
      memcpy(&local_248,&local_408,0x130);
      local_63 = 1;
      winit::platform_impl::platform::monitor::get_monitor_info();
      if ((((ulonglong)local_118 & 1) != 0) && (((uint)pHStack_110 & 3) == 1)) {
        local_80 = (undefined1 *)((longlong)&pHStack_110[-1].unused + 3);
        local_70 = *(HWND *)((longlong)&pHStack_110[-1].unused + 3);
        local_78 = *(undefined8 **)((longlong)&pHStack_110[1].unused + 3);
        if ((code *)*local_78 != (code *)0x0) {
          (*(code *)*local_78)(local_70);
        }
        if (local_78[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      winit::dpi::impl$5::from_f64();
      winit::dpi::impl$5::from_f64();
      local_1c0 = 0;
      memcpy(&local_408,&local_248,0x130);
      local_61 = 1;
      find_best_fullscreen_video_mode();
      if (local_2a8 == (HWND)0x0) goto LAB_1400443e8;
      local_108 = local_298;
      uStack_100 = pHStack_290;
      local_118 = local_2a8;
      pHStack_110 = pHStack_2a0;
      winit::platform_impl::platform::monitor::VideoMode::size();
      winit::dpi::impl$2::from_f64();
      winit::dpi::impl$2::from_f64();
      in_R8[0xc].unused = 1;
      *(ulonglong *)(in_R8 + 0xd) = (ulonglong)extraout_EAX_01 | extraout_RAX_08 << 0x20;
      in_R8[0xf].unused = (int)uStack_100;
      *(undefined2 *)&in_R8[0x10].unused = uStack_100._4_2_;
      if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        pHStack_a0 = (HWND)winit::monitor::impl$4::fmt;
        local_a8 = (HWND)&local_118;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_228 = (HWND)*extraout_RAX_09;
        lStack_220 = extraout_RAX_09[1];
        local_218 = (undefined8 *)0x3;
        uStack_210 = 0x14292d950;
        local_208 = 0x11;
        local_1f8 = &PTR_s_Chosen_video_mode__14292d928;
        local_1f0 = 1;
        local_1e8 = &local_a8;
        local_1e0 = 1;
        uStack_1d8 = 0;
        local_248 = (HWND)0x0;
        pHStack_240 = (HWND)((longlong)&DAT_14292d94f + 1);
        local_238 = (HWND)&DAT_00000011;
        pHStack_230 = (HWND)0x0;
        uStack_200 = CONCAT44((int)extraout_RAX_09[2],1);
        ::log::__private_api::impl$2::log();
      }
      _Src = &local_3f8;
      bVar14 = local_408 != 0;
      memcpy(&local_248,_Src,0x120);
      pHVar11 = local_118;
      local_98 = uStack_100;
      local_a8 = pHStack_110;
      pHStack_a0 = local_108;
      if (local_400 != (HWND)0x0 && bVar14) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_238 = local_98;
      local_248 = local_a8;
      pHStack_240 = pHStack_a0;
      local_400 = pHVar11;
      _Size = 0x120;
    }
    local_408 = 1;
  }
  else {
    local_61 = 1;
    winit::platform_impl::platform::monitor::get_monitor_info();
    if ((int)local_248 == 1) {
      local_118 = pHStack_240;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_61 = 1;
    winit::dpi::impl$2::from_f64();
    local_61 = 1;
    winit::dpi::impl$2::from_f64();
    local_61 = 1;
    winit::platform_impl::platform::monitor::get_monitor_info();
    if ((((ulonglong)local_248 & 1) != 0) && (((uint)pHStack_240 & 3) == 1)) {
      local_80 = (undefined1 *)((longlong)&pHStack_240[-1].unused + 3);
      local_70 = *(HWND *)((longlong)&pHStack_240[-1].unused + 3);
      local_78 = *(undefined8 **)((longlong)&pHStack_240[1].unused + 3);
      if ((code *)*local_78 != (code *)0x0) {
        (*(code *)*local_78)(local_70);
      }
      if (local_78[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_61 = 1;
    winit::dpi::impl$5::from_f64();
    local_61 = 1;
    winit::dpi::impl$5::from_f64();
    memcpy(&local_248,&local_408,0x130);
    local_62 = 1;
    winit::dpi::impl$5::from_f64();
    local_62 = 1;
    winit::dpi::impl$5::from_f64();
    winit::dpi::impl$5::from_f64();
    winit::dpi::impl$5::from_f64();
    local_1c0 = 0;
    _Size = 0x130;
    _Src = &local_408;
  }
  memcpy(_Src,&local_248,_Size);
  local_61 = 1;
  image::io::reader::Reader::open_impl();
  if (local_118 == (HWND)0x2) {
    local_2a8 = (HWND)0x8000000000000009;
    pHStack_2a0 = pHStack_110;
LAB_1400445f4:
    local_61 = 1;
    core::ptr::
    drop_in_place<enum2_<core::result::Result<enum2_<image::dynimage::DynamicImage>,enum2_<image::error::ImageError>_>_>_>
              ();
  }
  else {
    local_1f8 = local_c8;
    local_208 = local_d8;
    uStack_200 = uStack_d0;
    local_218 = local_e8;
    uStack_210 = uStack_e0;
    local_228 = local_f8;
    lStack_220 = lStack_f0;
    local_238 = local_108;
    pHStack_230 = uStack_100;
    local_248 = local_118;
    pHStack_240 = pHStack_110;
    local_61 = 1;
    image::io::reader::Reader::decode<std::io::buffered::bufreader::BufReader<std::fs::File>_>();
    if (local_2a8 != (HWND)&DAT_800000000000000a) goto LAB_1400445f4;
    local_f8 = local_280;
    local_108 = pHStack_290;
    uStack_100 = pHStack_288;
    local_118 = pHStack_2a0;
    pHStack_110 = local_298;
    local_65 = 1;
    image::dynimage::DynamicImage::to_rgba8();
    memcpy(&local_248,&local_408,0x130);
    local_64 = 1;
    winit::icon::Icon::from_rgba();
    local_70 = pHStack_a0;
    if ((uint)local_a8 != 3) {
      if (((uint)pHStack_a0 & 3) == 1 && 1 < (uint)local_a8) {
        local_80 = (undefined1 *)((longlong)&pHStack_a0[-1].unused + 3);
        local_70 = *(HWND *)((longlong)&pHStack_a0[-1].unused + 3);
        local_78 = *(undefined8 **)((longlong)&pHStack_a0[1].unused + 3);
        if ((code *)*local_78 != (code *)0x0) {
          (*(code *)*local_78)(local_70);
        }
        if (local_78[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_70 = (HWND)0x0;
    }
    if (local_178 != (HWND)0x0) {
      LOCK();
      *(HWND *)local_178 = (HWND)((longlong)&(*(HWND *)local_178)[-1].unused + 3);
      UNLOCK();
      if (*(HWND *)local_178 == (HWND)0x0) {
        alloc::sync::Arc::
        drop_slow<winit::platform_impl::platform::icon::RaiiIcon,alloc::alloc::Global>();
      }
    }
    local_178 = local_70;
    memcpy(&local_408,&local_248,0x130);
    core::ptr::drop_in_place<enum2_<image::dynimage::DynamicImage>_>();
    if (local_2a8 != (HWND)&DAT_800000000000000a) {
      local_61 = 1;
      core::ptr::
      drop_in_place<enum2_<core::result::Result<enum2_<image::dynimage::DynamicImage>,enum2_<image::error::ImageError>_>_>_>
                ();
    }
  }
  memcpy(&local_248,&local_408,0xe8);
  local_d8 = CONCAT17(uStack_2d9,uStack_2e0);
  local_118 = (HWND)CONCAT17(local_319,uStack_320);
  pHStack_110 = (HWND)CONCAT17(uStack_311,uStack_318);
  local_108 = (HWND)CONCAT17(local_309,uStack_310);
  uStack_100 = (HWND)CONCAT17(uStack_301,uStack_308);
  local_f8 = (HWND)CONCAT17(local_2f9,uStack_300);
  lStack_f0 = CONCAT17(uStack_2f1,uStack_2f8);
  local_e8 = (undefined8 *)CONCAT17(local_2e9,uStack_2f0);
  uStack_e0 = CONCAT17(uStack_2e1,uStack_2e8);
  local_61 = 0;
  winit::platform_impl::platform::window::init<tuple_<>_>();
  pHVar11 = pHStack_2a0;
  local_a8 = pHStack_2a0;
  pHStack_a0 = local_298;
  local_98 = pHStack_290;
  pHStack_90 = pHStack_288;
  if (((ulonglong)local_2a8 & 1) != 0) {
    local_108 = pHStack_290;
    uStack_100 = pHStack_288;
    local_118 = pHStack_2a0;
    pHStack_110 = local_298;
    local_238 = pHStack_290;
    pHStack_230 = pHStack_288;
    local_248 = pHStack_2a0;
    pHStack_240 = local_298;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_2b8 = pHStack_290;
  pHStack_2b0 = pHStack_288;
  local_2c8 = pHStack_2a0;
  pHStack_2c0 = local_298;
  pHVar2 = pHStack_2a0 + 4;
  LOCK();
  bVar14 = (char)pHStack_2a0[4].unused == '\0';
  if (bVar14) {
    *(undefined1 *)&pHStack_2a0[4].unused = 1;
  }
  UNLOCK();
  if (!bVar14) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar8 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar9 = 0;
    cVar5 = *(char *)((longlong)&pHVar11[4].unused + 1);
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar9 = extraout_AL_00 ^ 1;
    cVar5 = *(char *)((longlong)&pHVar11[4].unused + 1);
  }
  if (cVar5 != '\0') {
    uVar7 = (ulonglong)pHStack_240 >> 8;
    pHStack_240 = (HWND)CONCAT71((int7)uVar7,bVar9);
    local_248 = pHVar2;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  *(undefined1 *)&pHVar11[0x39].unused = 1;
  if (((bVar9 == 0) && ((*(ulonglong *)puVar8 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
    *(undefined1 *)((longlong)&pHVar11[4].unused + 1) = 1;
  }
  LOCK();
  iVar6 = pHVar2->unused;
  *(undefined1 *)&pHVar2->unused = 0;
  UNLOCK();
  if ((char)iVar6 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
  RedrawWindow(pHStack_2c0,(RECT *)0x0,(HRGN)0x0,2);
  local_108 = local_2b8;
  uStack_100 = pHStack_2b0;
  local_118 = local_2c8;
  pHStack_110 = pHStack_2c0;
  in_RCX[2] = local_2b8;
  in_RCX[3] = pHStack_2b0;
  *in_RCX = local_2c8;
  in_RCX[1] = pHStack_2c0;
  if (local_268 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_10 == 1) {
      plVar12 = extraout_RAX_10 + 1;
    }
    else if ((*extraout_RAX_10 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_11, extraout_RAX_11 == (longlong *)0x0)) {
LAB_14004491e:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar12 != 0) {
LAB_14004492a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    local_70 = (HWND)plVar12;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_70 = *(longlong *)local_70 + 1;
  }
  return;
}

