// 14048a170 void __cdecl start(void)

void __cdecl tiny_glade::crash_pickup::client::start(void)

{
  longlong lVar1;
  code *pcVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 *in_RCX;
  HANDLE hObject;
  undefined **ppuVar3;
  HANDLE in_R8;
  ulonglong in_R9;
  ulonglong uVar4;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  
  hObject = in_R8;
  minidumper::ipc::windows::UnixStream::connect<ref_<std::path::Path>_>();
  if (extraout_RAX != 0) {
    uVar4 = 0xffffffffffffffce;
    do {
      uVar4 = uVar4 + 0x32;
      if (in_R9 <= uVar4) {
        in_RCX[4] = local_88;
        in_RCX[2] = local_98;
        in_RCX[3] = uStack_90;
        *in_RCX = 1;
        in_RCX[1] = hObject;
        return;
      }
      std::thread::sleep();
      core::ptr::drop_in_place<enum2_<minidumper::errors::Error>_>();
      hObject = in_R8;
      minidumper::ipc::windows::UnixStream::connect<ref_<std::path::Path>_>();
    } while (extraout_RAX_00 != 0);
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 != (longlong *)0x0) {
    extraout_RAX_01[2] = (longlong)hObject;
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = 1;
    LOCK();
    lVar1 = *extraout_RAX_01;
    *extraout_RAX_01 = *extraout_RAX_01 + 1;
    UNLOCK();
    if (*extraout_RAX_01 != 0 && SCARRY8(lVar1,1) == *extraout_RAX_01 < 0) {
      std::thread::spawn<tiny_glade::crash_pickup::client::start::closure_env_0,tuple_<>_>();
      CloseHandle(hObject);
      LOCK();
      lRam0000000000000001 = lRam0000000000000001 + -1;
      UNLOCK();
      if (lRam0000000000000001 == 0) {
        alloc::sync::Arc::drop_slow<std::thread::Inner,alloc::alloc::Global>();
      }
      LOCK();
      lRam0000000000000001 = lRam0000000000000001 + -1;
      UNLOCK();
      if (lRam0000000000000001 == 0) {
        alloc::sync::Arc::drop_slow<std::thread::Packet<tuple_<>_>,alloc::alloc::Global>();
      }
      LOCK();
      lVar1 = *extraout_RAX_01;
      *extraout_RAX_01 = *extraout_RAX_01 + 1;
      UNLOCK();
      if (*extraout_RAX_01 != 0 && SCARRY8(lVar1,1) == *extraout_RAX_01 < 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_02 != (undefined8 *)0x0) {
          *extraout_RAX_02 = extraout_RAX_01;
          ppuVar3 = &
                    PTR_drop_in_place<crash_handler::make_crash_event::Wrapper<tiny_glade::crash_pickup::client::start::closure_env_1>_>_1429dd218
          ;
          crash_handler::windows::CrashHandler::attach();
          if (extraout_RAX_03 == 3) {
            in_RCX[1] = extraout_RAX_01;
            *in_RCX = 10;
          }
          else {
            in_RCX[1] = extraout_RAX_03;
            in_RCX[2] = ppuVar3;
            *in_RCX = 7;
            LOCK();
            *extraout_RAX_01 = *extraout_RAX_01 + -1;
            UNLOCK();
            if (*extraout_RAX_01 == 0) {
              alloc::sync::Arc::drop_slow<minidumper::ipc::client::Client,alloc::alloc::Global>();
            }
          }
          return;
        }
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
    }
                    // WARNING: Does not return
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

