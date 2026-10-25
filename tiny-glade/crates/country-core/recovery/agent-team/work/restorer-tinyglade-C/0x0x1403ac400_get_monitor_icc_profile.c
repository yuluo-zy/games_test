// get_monitor_icc_profile @ 0x0x1403ac400
// sig: void __cdecl get_monitor_icc_profile(void)
// size: 1443


void __cdecl tiny_glade::icc::get_monitor_icc_profile(void)

{
  BOOL BVar1;
  undefined1 *extraout_RAX;
  longlong extraout_RAX_00;
  LPCSTR pszDriver;
  HDC hdc;
  LPSTR pszFilename;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong *in_RCX;
  undefined1 *puVar2;
  undefined1 *_Dst;
  longlong local_138;
  longlong local_130;
  LPSTR local_128;
  char cStack_120;
  undefined4 local_117;
  undefined3 uStack_113;
  int local_110;
  undefined4 uStack_10c;
  void *pvStack_108;
  undefined1 *local_100;
  int local_e8;
  undefined4 uStack_e4;
  DWORD local_6c [7];
  LPCSTR local_50;
  undefined1 *local_48;
  undefined1 local_3b;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  winit::platform_impl::platform::monitor::get_monitor_info();
  if (local_e8 == 1) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  winit::platform_impl::platform::util::decode_wide();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::os_str::wtf8::Slice::to_string_lossy();
  if ((longlong)local_100 < 0) {
LAB_1403ac978:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (local_100 == (undefined1 *)0x0) {
    _Dst = (undefined1 *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_1403ac978;
  }
  memcpy(_Dst,pvStack_108,(size_t)local_100);
  if ((CONCAT44(uStack_10c,local_110) & 0x7fffffffffffffff) == 0) {
    puVar2 = (undefined1 *)0x0;
    if (CONCAT44(uStack_e4,local_e8) != 0) {
LAB_1403ac51e:
      puVar2 = (undefined1 *)CONCAT44(uStack_e4,local_e8);
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    if (CONCAT44(uStack_e4,local_e8) != 0) goto LAB_1403ac51e;
    puVar2 = (undefined1 *)0x0;
  }
  if (&DAT_0000000f < local_100) {
    local_39 = 0;
    core::slice::memchr::memchr_aligned();
    puVar2 = _Dst;
    if (extraout_RAX_00 != 0) {
LAB_1403ac904:
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  else if (local_100 != (undefined1 *)0x0) {
    puVar2 = (undefined1 *)0x0;
    do {
      if (_Dst[(longlong)puVar2] == '\0') goto LAB_1403ac904;
      puVar2 = puVar2 + 1;
    } while (local_100 != puVar2);
  }
  local_39 = 1;
  alloc::ffi::c_str::CString::_from_vec_unchecked();
  hdc = CreateICA(pszDriver,(LPCSTR)0x0,(LPCSTR)0x0,(DEVMODEA *)0x0);
  std::alloc::__default_lib_allocator::__rust_alloc_zeroed();
  local_48 = puVar2;
  if (pszFilename == (LPSTR)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_6c[0] = 0x800;
  local_50 = pszDriver;
  BVar1 = GetICMProfileA(hdc,local_6c,pszFilename);
  if (BVar1 == 0) {
    anyhow::__private::format_err();
    DeleteDC(hdc);
    std::alloc::__default_lib_allocator::__rust_dealloc();
    local_138 = -0x8000000000000000;
    local_130 = extraout_RAX_01;
    local_128 = pszFilename;
LAB_1403ac6fc:
    *local_50 = '\0';
    if (local_48 != (undefined1 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *in_RCX = local_138;
    in_RCX[1] = local_130;
    in_RCX[2] = (longlong)local_128;
    *(undefined1 *)(in_RCX + 3) = 1;
    *(undefined4 *)((longlong)in_RCX + 0x19) = local_117;
    *(uint *)((longlong)in_RCX + 0x1c) = CONCAT31(uStack_113,local_117._3_1_);
  }
  else {
    alloc::ffi::c_str::CString::from_vec_with_nul();
    if (local_110 == 2) {
      alloc::ffi::c_str::CString::into_string();
      if (cStack_120 == '\x02') {
        if (!SBORROW8(0,local_138)) {
          DeleteDC(hdc);
          goto LAB_1403ac6fc;
        }
      }
      else {
        local_3b = 1;
        std::backtrace::Backtrace::capture();
        local_3b = 0;
        anyhow::Error::
        construct<anyhow::error::ContextError<ref_<str_>,alloc::ffi::c_str::IntoStringError>_>();
        local_130 = extraout_RAX_03;
      }
      in_RCX[1] = local_130;
      *in_RCX = -0x8000000000000000;
      *local_50 = '\0';
    }
    else {
      local_3a = 0;
      std::backtrace::Backtrace::capture();
      local_3a = 1;
      anyhow::Error::
      construct<anyhow::error::ContextError<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>_>();
      in_RCX[1] = extraout_RAX_02;
      *in_RCX = -0x8000000000000000;
      *local_50 = '\0';
    }
    if (local_48 != (undefined1 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

