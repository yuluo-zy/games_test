
void __cdecl tiny_glade::crash_pickup::CrashReporter::spawn_server(void)

{
  longlong *plVar1;
  undefined **ppuVar2;
  HANDLE hObject;
  ulonglong uVar3;
  undefined8 *puVar4;
  HANDLE hObject_00;
  void *pvVar5;
  char extraout_AL;
  void *_Dst;
  ulonglong *extraout_RAX;
  int *in_RCX;
  longlong *in_RDX;
  undefined *_Src;
  byte in_R8B;
  ulonglong local_278;
  ulonglong uStack_270;
  undefined **local_268;
  HANDLE pvStack_260;
  ulonglong local_258;
  HANDLE pvStack_250;
  undefined8 *local_248;
  undefined **ppuStack_240;
  HANDLE local_238;
  HANDLE pvStack_230;
  undefined **local_1c0;
  HANDLE pvStack_1b8;
  ulonglong uStack_1b0;
  HANDLE pvStack_1a8;
  undefined8 *puStack_1a0;
  undefined **local_198;
  HANDLE pvStack_190;
  HANDLE local_188;
  void *local_108;
  void *pvStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  longlong local_e8;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  int iStack_b0;
  int iStack_ac;
  undefined8 *puStack_a8;
  undefined **local_a0;
  HANDLE pvStack_98;
  HANDLE local_90;
  void *local_88;
  void *pvStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  void *local_68;
  longlong *local_60;
  undefined **local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_60 = in_RDX;
  std::env::temp_dir();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  uuid::Uuid::new_v4();
  local_c8 = (undefined **)0x0;
  uStack_c0 = (void *)0x1;
  uStack_b8 = 0;
  local_278 = 0;
  local_268 = (undefined **)0x0;
  local_258 = 0x20;
  pvStack_250 = (HANDLE)CONCAT71(pvStack_250._1_7_,3);
  local_248 = &local_c8;
  ppuStack_240 = &anon_71092468c3c6f796af31fbdc895ed37f_16_llvm_593389665763099172;
  uuid::fmt::impl$1::fmt();
  if (extraout_AL != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_58 = local_c8;
  local_68 = uStack_c0;
  std::path::Path::_join();
  if (local_58 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_1c0 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::ffi::os_str::impl$41::try_from();
  if ((local_278 & 1) == 0) {
    std::env::current_exe();
    if (!SBORROW8(0,(longlong)local_108)) {
      local_78 = local_f8;
      uStack_70 = uStack_f0;
      local_88 = local_108;
      pvStack_80 = pvStack_100;
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::sys::pal::windows::process::Command::new();
      memcpy(&local_1c0,&local_278,0xb8);
      if (local_88 != (void *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::sys::pal::windows::process::Command::arg();
      std::sys::pal::windows::process::Command::arg();
      std::sys::pal::windows::process::Command::arg();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      _Src = &DAT_14295ba60;
      if (in_R8B != 0) {
        _Src = &DAT_14295ba65;
      }
      memcpy(_Dst,_Src,(size_t)((ulonglong)in_R8B ^ 5));
      local_68 = _Dst;
      local_58 = (undefined **)((ulonglong)in_R8B ^ 5);
      std::sys::pal::windows::process::Command::arg();
      std::alloc::__default_lib_allocator::__rust_dealloc();
      std::process::Command::spawn();
      core::ptr::drop_in_place<std::process::Command>();
      pvStack_100 = uStack_c0;
      if (local_c8 != (undefined **)0x2) {
        pvStack_1a8 = (HANDLE)CONCAT44(iStack_ac,iStack_b0);
        local_188 = local_90;
        local_198 = local_a0;
        pvStack_190 = pvStack_98;
        puStack_1a0 = puStack_a8;
        local_1c0 = local_c8;
        pvStack_1b8 = uStack_c0;
        uStack_1b0 = uStack_b8;
        local_49 = 1;
        std::process::Child::id();
        local_49 = 1;
        client::start();
        pvVar5 = uStack_c0;
        hObject_00 = local_188;
        puVar4 = puStack_1a0;
        uVar3 = uStack_1b0;
        hObject = pvStack_1b8;
        ppuVar2 = local_1c0;
        if ((int)local_c8 == 10) {
          local_238 = pvStack_190;
          pvStack_230 = local_188;
          local_248 = puStack_1a0;
          ppuStack_240 = local_198;
          local_258 = uStack_1b0;
          pvStack_250 = pvStack_1a8;
          local_268 = local_1c0;
          pvStack_260 = pvStack_1b8;
          local_88 = uStack_c0;
          local_278 = 1;
          uStack_270 = 1;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX == (ulonglong *)0x0) {
            local_58 = pvVar5;
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          extraout_RAX[8] = (ulonglong)local_238;
          extraout_RAX[9] = (ulonglong)pvStack_230;
          extraout_RAX[6] = (ulonglong)local_248;
          extraout_RAX[7] = (ulonglong)ppuStack_240;
          extraout_RAX[4] = local_258;
          extraout_RAX[5] = (ulonglong)pvStack_250;
          extraout_RAX[2] = (ulonglong)local_268;
          extraout_RAX[3] = (ulonglong)pvStack_260;
          *extraout_RAX = local_278;
          extraout_RAX[1] = uStack_270;
          *(void **)(in_RCX + 2) = pvVar5;
          *(ulonglong **)(in_RCX + 4) = extraout_RAX;
          in_RCX[0] = 10;
          in_RCX[1] = 0;
          plVar1 = local_60;
        }
        else {
          local_68 = pvStack_1a8;
          local_58 = local_198;
          *(undefined8 **)(in_RCX + 8) = puStack_a8;
          in_RCX[4] = (int)uStack_b8;
          in_RCX[5] = uStack_b8._4_4_;
          in_RCX[6] = iStack_b0;
          in_RCX[7] = iStack_ac;
          *in_RCX = (int)local_c8;
          in_RCX[1] = local_c8._4_4_;
          in_RCX[2] = (int)uStack_c0;
          in_RCX[3] = uStack_c0._4_4_;
          CloseHandle(pvStack_190);
          CloseHandle(hObject_00);
          if (ppuVar2 != (undefined **)0x0) {
            CloseHandle(hObject);
          }
          if (uVar3 != 0) {
            CloseHandle(local_68);
          }
          plVar1 = local_60;
          if (puVar4 != (undefined8 *)0x0) {
            CloseHandle(local_58);
          }
        }
        goto joined_r0x00014015b204;
      }
    }
    in_RCX[0] = 6;
    in_RCX[1] = 0;
    *(void **)(in_RCX + 2) = pvStack_100;
    plVar1 = local_60;
  }
  else {
    in_RCX[0] = 9;
    in_RCX[1] = 0;
    *(undefined **)(in_RCX + 2) = &DAT_14295bc2f;
    in_RCX[4] = 0x21;
    in_RCX[5] = 0;
    plVar1 = local_60;
  }
joined_r0x00014015b204:
  if (local_e8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (*plVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

