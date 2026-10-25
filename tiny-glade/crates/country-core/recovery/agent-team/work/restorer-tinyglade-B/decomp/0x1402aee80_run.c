// run @ 0x1402aee80
// sig: void __cdecl run(void)


void __cdecl tiny_glade::crash_pickup::server::run(void)

{
  longlong lVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  longlong *plVar4;
  code *pcVar5;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong *plVar6;
  undefined8 *in_R9;
  bool bVar7;
  longlong *local_500;
  longlong *local_4f8;
  longlong *local_4f0;
  longlong *plStack_4e8;
  longlong *local_4e0;
  longlong *plStack_4d8;
  longlong *local_2e0;
  longlong *plStack_2d8;
  longlong *local_2d0;
  longlong *plStack_2c8;
  longlong *local_2c0;
  longlong *plStack_2b8;
  undefined8 local_2b0;
  undefined *local_2a8;
  undefined8 local_2a0;
  undefined4 local_298;
  undefined4 local_294;
  undefined **local_290;
  undefined8 local_288;
  undefined8 local_280;
  undefined8 local_278;
  undefined8 uStack_270;
  longlong *local_b8;
  longlong *plStack_b0;
  longlong *local_a8;
  longlong *plStack_a0;
  longlong *local_98;
  longlong *plStack_90;
  longlong *local_88;
  longlong *plStack_80;
  longlong *local_78;
  longlong *plStack_70;
  longlong *local_68;
  undefined8 *local_60;
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  undefined1 local_3e;
  undefined1 local_3d;
  undefined1 local_3c;
  undefined1 local_3b;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_60 = in_R9;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
    local_3b = 1;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  *(undefined1 *)(extraout_RAX + 2) = 0;
  extraout_RAX[3] = -0x8000000000000000;
  local_48 = extraout_RAX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (longlong *)0x0) {
    local_3c = 1;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_00 = 1;
  extraout_RAX_00[1] = 1;
  *(undefined1 *)((longlong)extraout_RAX_00 + 0x12) = 2;
  *(undefined2 *)(extraout_RAX_00 + 2) = 0x200;
  local_39 = 1;
  local_50 = extraout_RAX_00;
  minidumper::ipc::server::Server::with_name<ref_<str_>_>();
  plVar6 = plStack_2d8;
  local_a8 = local_2d0;
  plStack_a0 = plStack_2c8;
  local_98 = local_2c0;
  plStack_90 = plStack_2b8;
  plVar4 = (longlong *)*local_60;
  if (local_2e0 != (longlong *)0x2) {
    local_b8 = (longlong *)local_60[2];
    plStack_b0 = (longlong *)local_60[3];
    local_500 = local_2e0;
    local_4f8 = plStack_2d8;
    local_4f0 = local_2d0;
    plStack_4e8 = plStack_2c8;
    local_4e0 = local_2c0;
    plStack_4d8 = plStack_2b8;
    local_68 = (longlong *)local_60[1];
    local_58 = plVar4;
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_3a = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_2c0 = (longlong *)*extraout_RAX_01;
      plStack_2b8 = (longlong *)extraout_RAX_01[1];
      local_294 = *(undefined4 *)(extraout_RAX_01 + 2);
      local_2b0 = 3;
      local_2a8 = &DAT_1429ad260;
      local_2a0 = 0x20;
      local_290 = &PTR_s_Crash_reporter_server_initialize_1429ad310;
      local_288 = 1;
      local_280 = 8;
      local_2e0 = (longlong *)0x0;
      local_278 = 0;
      uStack_270 = 0;
      plStack_2d8 = (longlong *)&DAT_1429ad260;
      local_2d0 = (longlong *)&DAT_00000020;
      plStack_2c8 = (longlong *)0x0;
      local_298 = 1;
      local_3a = 1;
      ::log::__private_api::impl$2::log();
    }
    LOCK();
    lVar1 = *local_48;
    *local_48 = *local_48 + 1;
    UNLOCK();
    if (*local_48 != 0 && SCARRY8(lVar1,1) == *local_48 < 0) {
      LOCK();
      lVar1 = *local_50;
      *local_50 = *local_50 + 1;
      UNLOCK();
      if (*local_50 != 0 && SCARRY8(lVar1,1) == *local_50 < 0) {
        local_2d0 = local_b8;
        plStack_2c8 = plStack_b0;
        local_2c0 = local_48;
        plStack_2b8 = local_50;
        local_2e0 = local_58;
        plStack_2d8 = local_68;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        extraout_RAX_02[4] = local_2c0;
        extraout_RAX_02[5] = plStack_2b8;
        extraout_RAX_02[2] = local_2d0;
        extraout_RAX_02[3] = plStack_2c8;
        *extraout_RAX_02 = local_2e0;
        extraout_RAX_02[1] = plStack_2d8;
        local_3d = 0;
        local_3a = 0;
        minidumper::ipc::server::Server::run();
        plVar6 = (longlong *)&DAT_0000000a;
        if (local_2e0 != (longlong *)&DAT_00000006) {
          local_78 = plStack_2c8;
          plStack_70 = local_2c0;
          local_88 = plStack_2d8;
          plStack_80 = local_2d0;
          plVar6 = local_2e0;
        }
        local_3e = 0;
        core::ptr::drop_in_place<minidumper::ipc::server::Server>();
        goto LAB_1402af218;
      }
    }
                    // WARNING: Does not return
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
  local_78 = local_2c0;
  plStack_70 = plStack_2b8;
  local_88 = local_2d0;
  plStack_80 = plStack_2c8;
  if (plVar4 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_1402af218:
  if (plVar6 == (longlong *)&DAT_0000000a) {
    if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_39 = 0;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_2c0 = (longlong *)*extraout_RAX_03;
      plStack_2b8 = (longlong *)extraout_RAX_03[1];
      local_294 = *(undefined4 *)(extraout_RAX_03 + 2);
      local_2b0 = 4;
      local_2a8 = &DAT_1429ad260;
      local_2a0 = 0x20;
      local_290 = &PTR_s_Shutting_down_the_crash_server__1429ad238;
      local_288 = 1;
      local_280 = 8;
      local_2e0 = (longlong *)0x0;
      local_278 = 0;
      uStack_270 = 0;
      plStack_2d8 = (longlong *)&DAT_1429ad260;
      local_2d0 = (longlong *)&DAT_00000020;
      plStack_2c8 = (longlong *)0x0;
      local_298 = 1;
      local_39 = 0;
      ::log::__private_api::impl$2::log();
    }
    plVar4 = local_48;
    LOCK();
    bVar7 = local_48[1] == 1;
    if (bVar7) {
      local_48[1] = -1;
    }
    UNLOCK();
    if ((!bVar7) || (local_48[1] = 1, *local_48 != 1)) {
      local_39 = 0;
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    memcpy(&local_2e0,local_48 + 3,0x228);
    plVar6 = local_2e0;
    plVar4[3] = -0x8000000000000000;
    LOCK();
    bVar7 = local_50[1] == 1;
    if (bVar7) {
      local_50[1] = -1;
    }
    UNLOCK();
    if ((!bVar7) || (local_50[1] = 1, *local_50 != 1)) {
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    uVar2 = *(undefined1 *)((longlong)local_50 + 0x11);
    uVar3 = *(undefined1 *)((longlong)local_50 + 0x12);
    *(undefined1 *)((longlong)local_50 + 0x11) = 2;
    *(undefined1 *)((longlong)local_50 + 0x12) = 2;
    if (!SBORROW8(0,(longlong)local_2e0)) {
      memcpy(&local_500,&plStack_2d8,0x220);
    }
    *in_RCX = plVar6;
    memcpy(in_RCX + 1,&local_500,0x220);
    *(undefined1 *)(in_RCX + 0x45) = uVar2;
    *(undefined1 *)((longlong)in_RCX + 0x229) = uVar3;
    LOCK();
    *local_50 = *local_50 + -1;
    UNLOCK();
    if (*local_50 == 0) {
      local_3c = 0;
      alloc::sync::Arc::
      drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                ();
    }
    LOCK();
    *local_48 = *local_48 + -1;
    UNLOCK();
    if (*local_48 == 0) {
      local_3b = 0;
      alloc::sync::Arc::
      drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>_>_>,alloc::alloc::Global>
                ();
    }
  }
  else {
    in_RCX[4] = local_78;
    in_RCX[5] = plStack_70;
    in_RCX[2] = local_88;
    in_RCX[3] = plStack_80;
    in_RCX[1] = plVar6;
    *in_RCX = 0x8000000000000001;
    LOCK();
    *local_50 = *local_50 + -1;
    UNLOCK();
    if (*local_50 == 0) {
      local_3c = 0;
      alloc::sync::Arc::
      drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                ();
    }
    LOCK();
    *local_48 = *local_48 + -1;
    UNLOCK();
    if (*local_48 == 0) {
      local_3b = 0;
      alloc::sync::Arc::
      drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2_<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>_>_>,alloc::alloc::Global>
                ();
    }
  }
  return;
}

