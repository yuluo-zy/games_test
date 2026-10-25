// run_vulkan_configuration_app @ 0x1403e74b0
// sig: void __cdecl run_vulkan_configuration_app(void)


void __cdecl tiny_glade::vulkan_config_main::run_vulkan_configuration_app(void)

{
  char *pcVar1;
  longlong *plVar2;
  char cVar3;
  longlong lVar4;
  uint uVar5;
  undefined *puVar6;
  undefined1 uVar7;
  undefined1 uVar8;
  longlong lVar9;
  byte extraout_AL;
  byte bVar10;
  char extraout_AL_00;
  uint extraout_EAX;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RCX;
  longlong lVar11;
  longlong lVar12;
  bool bVar13;
  undefined4 uVar14;
  undefined8 *local_538;
  undefined8 uStack_530;
  undefined8 local_528;
  undefined8 uStack_520;
  undefined4 local_518;
  undefined8 *local_450;
  undefined8 uStack_448;
  undefined8 local_440;
  undefined8 uStack_438;
  undefined8 local_430;
  undefined8 uStack_428;
  undefined8 local_420;
  undefined8 uStack_418;
  undefined8 local_410;
  undefined8 *local_408;
  undefined8 uStack_400;
  undefined8 local_3f8;
  undefined8 uStack_3f0;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  undefined8 uStack_3d0;
  undefined8 local_3c8;
  longlong *plStack_3c0;
  undefined8 local_3b8;
  undefined8 local_3a8;
  undefined4 uStack_3a0;
  undefined4 uStack_39c;
  undefined4 uStack_398;
  undefined4 uStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined4 uStack_388;
  undefined4 local_384;
  undefined4 local_380;
  undefined4 uStack_37c;
  longlong *local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  ulonglong local_2c0;
  undefined8 *puStack_2b8;
  HWND local_2b0;
  undefined8 uStack_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined8 local_290;
  undefined8 uStack_288;
  undefined8 local_280;
  longlong local_270;
  undefined4 local_268;
  undefined8 *local_264;
  undefined8 uStack_25c;
  undefined8 local_254;
  undefined8 uStack_24c;
  undefined4 local_244;
  undefined8 *local_238;
  HWND pHStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 local_210;
  longlong *plStack_208;
  undefined8 local_200;
  longlong local_1f8;
  undefined4 local_1e0;
  undefined4 local_1c8;
  undefined4 local_1b0;
  undefined4 local_198;
  undefined4 local_180;
  undefined8 local_168;
  undefined4 local_160;
  undefined1 local_15c;
  undefined4 local_15b;
  undefined1 local_157;
  undefined4 local_156;
  undefined8 *local_108;
  HWND pHStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 *local_e8;
  HWND pHStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 *local_c8;
  HWND pHStack_c0;
  HWND local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  ulonglong local_80;
  undefined8 *local_78;
  longlong local_70;
  undefined1 local_64;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_c8 = (undefined8 *)0x0;
  local_b8 = (HWND)CONCAT62(local_b8._2_6_,0x100);
  LOCK();
  cVar3 = *__imp__ZN5winit10event_loop18EVENT_LOOP_CREATED17h3040f7edfbf9ff64E;
  *__imp__ZN5winit10event_loop18EVENT_LOOP_CREATED17h3040f7edfbf9ff64E = 1;
  UNLOCK();
  local_70 = in_RCX;
  if (cVar3 == '\0') {
    winit::platform_impl::platform::event_loop::EventLoop::new<tuple_<>_>();
    local_538 = (undefined8 *)CONCAT44(uStack_398,uStack_39c);
    uStack_530 = CONCAT44(uStack_390,uStack_394);
    local_528 = CONCAT44(uStack_388,uStack_38c);
    uStack_520 = CONCAT44(local_380,local_384);
    uStack_218 = CONCAT44(uStack_218._4_4_,uStack_37c);
    local_518 = uStack_37c;
    lVar4 = CONCAT44(local_3a8._4_4_,(int)local_3a8);
    local_238 = local_538;
    pHStack_230 = (HWND)uStack_530;
    uStack_228 = local_528;
    uStack_220 = uStack_520;
    uVar14 = uStack_3a0;
    if (CONCAT44(local_3a8._4_4_,(int)local_3a8) == 3) {
      lVar4 = 3;
    }
  }
  else {
    lVar4 = 3;
    uVar14 = 3;
  }
  local_80 = (ulonglong)local_c8;
  if (local_c8 != (undefined8 *)0x0) {
    local_78 = (undefined8 *)pHStack_c0;
    if (*(code **)pHStack_c0 != (code *)0x0) {
      (**(code **)pHStack_c0)(local_c8);
    }
    if (local_78[1] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if ((int)lVar4 == 3) {
    local_3a8._4_4_ = SUB84(local_538,0);
    uStack_3a0 = (undefined4)((ulonglong)local_538 >> 0x20);
    uStack_39c = (undefined4)uStack_530;
    uStack_398 = (undefined4)((ulonglong)uStack_530 >> 0x20);
    uStack_394 = (undefined4)local_528;
    uStack_390 = (undefined4)((ulonglong)local_528 >> 0x20);
    uStack_38c = (undefined4)uStack_520;
    uStack_388 = (undefined4)((ulonglong)uStack_520 >> 0x20);
    local_384 = local_518;
    local_3a8._0_4_ = uVar14;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_264 = local_538;
  uStack_25c = uStack_530;
  local_254 = local_528;
  uStack_24c = uStack_520;
  local_244 = local_518;
  local_270 = lVar4;
  local_268 = uVar14;
  rhapsody::raw::init::GraphicsInstanceLoader::new();
  if ((int)local_238 != 0x18) {
    local_368 = local_1f8;
    local_378 = plStack_208;
    uStack_370 = local_200;
    uStack_388 = (undefined4)uStack_218;
    local_384 = (undefined4)((ulonglong)uStack_218 >> 0x20);
    local_380 = (undefined4)local_210;
    uStack_37c = (undefined4)((ulonglong)local_210 >> 0x20);
    uStack_398 = (undefined4)uStack_228;
    uStack_394 = (undefined4)((ulonglong)uStack_228 >> 0x20);
    uStack_390 = (undefined4)uStack_220;
    uStack_38c = (undefined4)((ulonglong)uStack_220 >> 0x20);
    local_3a8._4_4_ = (undefined4)((ulonglong)local_238 >> 0x20);
    uStack_3a0 = SUB84(pHStack_230,0);
    uStack_39c = (undefined4)((ulonglong)pHStack_230 >> 0x20);
    local_3a8._0_4_ = (int)local_238;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_3b8 = local_200;
  local_3c8 = local_210;
  plStack_3c0 = plStack_208;
  local_3d8 = uStack_220;
  uStack_3d0 = uStack_218;
  local_3e8 = pHStack_230;
  uStack_3e0 = uStack_228;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX = 0x69772074696e6977;
  *(undefined4 *)(extraout_RAX + 1) = 0x776f646e;
  local_1f8 = CONCAT44(local_1f8._4_4_,2);
  local_1e0 = 2;
  local_1c8 = 2;
  local_1b0 = 2;
  local_15c = 1;
  local_160 = 7;
  local_210 = 0xc;
  local_200 = 0xc;
  local_15b = 0x100;
  local_157 = 1;
  local_168 = 0;
  local_198 = 2;
  local_156 = 0x1020100;
  local_180 = 0xf;
  local_238 = (undefined8 *)0x0;
  winit::platform_impl::platform::impl$0::default();
  memcpy(&local_3a8,&local_238,0xe8);
  local_2c0 = (ulonglong)local_c8;
  puStack_2b8 = (undefined8 *)pHStack_c0;
  local_2b0 = local_b8;
  uStack_2a8 = uStack_b0;
  local_2a0 = local_a8;
  uStack_298 = uStack_a0;
  local_290 = local_98;
  uStack_288 = uStack_90;
  local_280 = local_88;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX_00 + 0xd) = 0x666e6f63206e616b;
  *(undefined8 *)((longlong)extraout_RAX_00 + 0x15) = 0x726f746172756769;
  *extraout_RAX_00 = 0x616c4720796e6954;
  extraout_RAX_00[1] = 0x6e616b7556206564;
  if (CONCAT44(uStack_37c,local_380) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_380 = 0x1d;
  uStack_37c = 0;
  uStack_370 = 0x1d;
  memcpy(&local_238,&local_3a8,0x130);
  winit::dpi::impl$2::from_f64();
  winit::dpi::impl$2::from_f64();
  local_1f8 = (ulonglong)extraout_EAX << 0x20;
  memcpy(&local_538,&local_238,0x130);
  memcpy(&local_3a8,&local_538,0xe8);
  local_1f8 = local_410;
  plStack_208 = (longlong *)local_420;
  local_200 = uStack_418;
  uStack_218 = local_430;
  local_210 = uStack_428;
  uStack_228 = local_440;
  uStack_220 = uStack_438;
  local_238 = local_450;
  pHStack_230 = (HWND)uStack_448;
  winit::platform_impl::platform::window::init<tuple_<>_>();
  local_e8 = (undefined8 *)pHStack_c0;
  pHStack_e0 = local_b8;
  local_d8 = uStack_b0;
  uStack_d0 = local_a8;
  if (((ulonglong)local_c8 & 1) != 0) {
    uStack_228 = uStack_b0;
    uStack_220 = local_a8;
    local_238 = (undefined8 *)pHStack_c0;
    pHStack_230 = local_b8;
    uStack_398 = (undefined4)uStack_b0;
    uStack_394 = (undefined4)((ulonglong)uStack_b0 >> 0x20);
    uStack_390 = (undefined4)local_a8;
    uStack_38c = (undefined4)((ulonglong)local_a8 >> 0x20);
    local_3a8._0_4_ = (int)pHStack_c0;
    local_3a8._4_4_ = (undefined4)((ulonglong)pHStack_c0 >> 0x20);
    uStack_3a0 = SUB84(local_b8,0);
    uStack_39c = (undefined4)((ulonglong)local_b8 >> 0x20);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_f8 = uStack_b0;
  uStack_f0 = local_a8;
  local_108 = (undefined8 *)pHStack_c0;
  pHStack_100 = local_b8;
  pcVar1 = (char *)((longlong)pHStack_c0 + 0x10);
  LOCK();
  bVar13 = *(char *)((longlong)pHStack_c0 + 0x10) == '\0';
  if (bVar13) {
    *(char *)((longlong)pHStack_c0 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar13) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar6 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar10 = 0;
    cVar3 = *(char *)((longlong)pHStack_c0 + 0x11);
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar10 = extraout_AL ^ 1;
    cVar3 = *(char *)((longlong)pHStack_c0 + 0x11);
  }
  if (cVar3 != '\0') {
    uVar5 = (uint)uStack_3a0 >> 8;
    uStack_3a0 = CONCAT31((int3)uVar5,bVar10);
    local_3a8 = pcVar1;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  *(undefined1 *)((longlong)pHStack_c0 + 0xe4) = 1;
  if (((bVar10 == 0) && ((*(ulonglong *)puVar6 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    *(undefined1 *)((longlong)pHStack_c0 + 0x11) = 1;
  }
  LOCK();
  cVar3 = *pcVar1;
  *pcVar1 = '\0';
  UNLOCK();
  if (cVar3 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
  RedrawWindow(pHStack_100,(RECT *)0x0,(HRGN)0x0,2);
  uStack_228 = local_f8;
  uStack_220 = uStack_f0;
  local_c8 = local_108;
  pHStack_c0 = pHStack_100;
  local_b8 = (HWND)local_f8;
  uStack_b0 = uStack_f0;
  local_61 = 1;
  rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers();
  uStack_398 = (undefined4)local_528;
  uStack_394 = (undefined4)((ulonglong)local_528 >> 0x20);
  uStack_390 = (undefined4)uStack_520;
  uStack_38c = (undefined4)((ulonglong)uStack_520 >> 0x20);
  local_3a8._0_4_ = (int)local_538;
  local_3a8._4_4_ = (undefined4)((ulonglong)local_538 >> 0x20);
  uStack_3a0 = (undefined4)uStack_530;
  uStack_39c = (undefined4)((ulonglong)uStack_530 >> 0x20);
  local_61 = 1;
  alloc::vec::in_place_collect::impl$1::
  from_iter<rhapsody::raw::init::GraphicsInstanceLayer,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl_4::implicit_layers::closure_env_0>_>
            ();
  lVar4 = local_70;
  plVar2 = (longlong *)(local_70 + 0x18);
  uVar14 = *(undefined4 *)(local_70 + 0x30);
  local_62 = 1;
  local_63 = 1;
  alloc::vec::impl$11::
  clone<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>();
  uStack_520 = CONCAT44(uStack_520._4_4_,uVar14);
  uStack_388 = SUB84(local_b8,0);
  local_384 = (undefined4)((ulonglong)local_b8 >> 0x20);
  local_380 = (undefined4)uStack_b0;
  uStack_37c = (undefined4)((ulonglong)uStack_b0 >> 0x20);
  uStack_398 = SUB84(local_c8,0);
  uStack_394 = (undefined4)((ulonglong)local_c8 >> 0x20);
  uStack_390 = SUB84(pHStack_c0,0);
  uStack_38c = (undefined4)((ulonglong)pHStack_c0 >> 0x20);
  local_3a8._0_4_ = 1;
  local_3a8._4_4_ = 0;
  uStack_3a0 = 1;
  uStack_39c = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 != (undefined8 *)0x0) {
    extraout_RAX_01[4] = CONCAT44(local_384,uStack_388);
    extraout_RAX_01[5] = CONCAT44(uStack_37c,local_380);
    extraout_RAX_01[2] = CONCAT44(uStack_394,uStack_398);
    extraout_RAX_01[3] = CONCAT44(uStack_38c,uStack_390);
    *extraout_RAX_01 = CONCAT44(local_3a8._4_4_,(int)local_3a8);
    extraout_RAX_01[1] = CONCAT44(uStack_39c,uStack_3a0);
    local_64 = 0;
    tg_gpu_selector::run_vulkan_configuration_gui();
    lVar9 = local_70;
    if ((longlong)local_238 < -0x7ffffffffffffffe) {
      if (local_238 == (undefined8 *)0x8000000000000001) {
        local_61 = 0;
        anyhow::error::impl$6::drop();
      }
    }
    else {
      local_3f8 = uStack_228;
      uStack_3f0 = uStack_220;
      local_408 = local_238;
      uStack_400 = pHStack_230;
      uVar14 = (undefined4)uStack_218;
      uVar7 = uStack_218._4_1_;
      uVar8 = uStack_218._5_1_;
      lVar12 = *(longlong *)(local_70 + 0x28);
      if (lVar12 != 0) {
        lVar11 = *(longlong *)(local_70 + 0x20) + 0x10;
        do {
          if (*(longlong *)(lVar11 + -8) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          lVar11 = lVar11 + 0x20;
          lVar12 = lVar12 + -1;
        } while (lVar12 != 0);
      }
      if (*plVar2 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined8 *)(lVar4 + 0x28) = local_3f8;
      *(undefined8 *)(lVar4 + 0x30) = uStack_3f0;
      *plVar2 = (longlong)local_408;
      *(undefined8 *)(lVar4 + 0x20) = uStack_400;
      *(undefined4 *)(lVar9 + 0x90) = uVar14;
      *(undefined1 *)(lVar9 + 0xe4) = uVar7;
      *(undefined1 *)(lVar9 + 0xbe) = uVar8;
      *(undefined1 *)(lVar9 + 0xbf) = uVar8;
      app_config::write_game_config_to_disk();
      if (local_238 == (undefined8 *)0x8000000000000001) {
        local_61 = 0;
        anyhow::error::impl$6::drop();
      }
    }
    if (plStack_3c0 != (longlong *)0x0) {
      LOCK();
      *plStack_3c0 = *plStack_3c0 + -1;
      UNLOCK();
      if (*plStack_3c0 == 0) {
        alloc::sync::Arc::drop_slow<libloading::safe::Library,alloc::alloc::Global>();
      }
    }
    core::ptr::drop_in_place<winit::event_loop::EventLoop<tuple_<>_>_>();
    core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>();
    if ((!SBORROW8(0,*(longlong *)(local_70 + 200))) && (*(longlong *)(local_70 + 200) != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

