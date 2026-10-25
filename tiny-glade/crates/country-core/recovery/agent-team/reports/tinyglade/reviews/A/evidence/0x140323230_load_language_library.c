// load_language_library @ 0x140323230 size=994
// sig: void __cdecl load_language_library(void)


void __cdecl tiny_glade::load_language_library::load_language_library(void)

{
  longlong lVar1;
  code *pcVar2;
  void *_Src;
  longlong **_Size;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  char *_Str;
  void *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong in_RCX;
  void *_Dst;
  longlong *local_e8;
  code *local_e0;
  undefined **local_d8;
  code *local_d0;
  longlong **local_c8;
  void *local_c0;
  longlong **local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  longlong *local_98;
  undefined8 local_90;
  undefined *local_88;
  undefined8 local_80;
  void *local_68;
  longlong local_60;
  longlong *local_58;
  undefined8 local_50;
  longlong **local_48;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX + 3) = 0x6873696c;
  *(int *)extraout_RAX = 0x6c676e65;
  if (in_RCX == 0) {
    local_60 = 7;
    local_50 = 7;
    local_58 = extraout_RAX;
LAB_14032342d:
    if ((((*(int *)((longlong)local_58 + 3) == 0x6873696c && (int)*local_58 == 0x6c676e65) ||
         ((*(int *)((longlong)local_58 + 3) == 0x6e61696c && (int)*local_58 == 0x6c617469 ||
          (*(int *)((longlong)local_58 + 3) == 0x616e6165 && (int)*local_58 == 0x65726f6b)))) ||
        (*(int *)((longlong)local_58 + 3) == 0x6e616973 && (int)*local_58 == 0x73737572)) ||
       (((*(int *)((longlong)local_58 + 3) == 0x6873696e && (int)*local_58 == 0x6e617073 ||
         (*(int *)((longlong)local_58 + 3) == 0x68736964 && (int)*local_58 == 0x64657773)) ||
        (*(int *)((longlong)local_58 + 3) == 0x6873696b && (int)*local_58 == 0x6b727574))))
    goto LAB_140323686;
  }
  else {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_90 = SteamAPI_SteamApps_v008();
    local_98 = (longlong *)*extraout_RAX_00;
    LOCK();
    lVar1 = *local_98;
    *local_98 = *local_98 + 1;
    UNLOCK();
    if (*local_98 == 0 || SCARRY8(lVar1,1) != *local_98 < 0) {
                    // WARNING: Does not return
      pcVar2 = (code *)invalidInstructionException();
      (*pcVar2)();
    }
    _Str = (char *)SteamAPI_ISteamApps_GetCurrentGameLanguage(local_90);
    strlen(_Str);
    core::ffi::c_str::CStr::to_string_lossy();
    _Size = local_b8;
    _Src = local_c0;
    if (SBORROW8(0,(longlong)local_c8)) {
      if ((longlong)local_b8 < 0) {
LAB_140323715:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (local_b8 == (longlong **)0x0) {
        _Dst = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX_01;
        if (extraout_RAX_01 == (void *)0x0) goto LAB_140323715;
      }
      memcpy(_Dst,_Src,(size_t)_Size);
      local_48 = _Size;
      local_68 = _Dst;
    }
    else {
      local_48 = local_c8;
      local_68 = local_c0;
    }
    alloc::str::impl$5::to_lowercase();
    if (local_48 != (longlong **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    LOCK();
    *local_98 = *local_98 + -1;
    UNLOCK();
    if (*local_98 == 0) {
      alloc::sync::Arc::drop_slow<steamworks::Inner<steamworks::ClientManager>,alloc::alloc::Global>
                ();
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    switch(local_50) {
    case 6:
      if (((*(short *)((longlong)local_58 + 4) == 0x6863 && (int)*local_58 == 0x6e657266) ||
          (*(short *)((longlong)local_58 + 4) == 0x6e61 && (int)*local_58 == 0x6d726567)) ||
         (*(short *)((longlong)local_58 + 4) == 0x6873 && (int)*local_58 == 0x696c6f70))
      goto LAB_140323686;
      break;
    case 7:
      goto LAB_14032342d;
    case 8:
      if ((*local_58 == 0x6573656e6170616a) || (*local_58 == 0x6573656e69686373))
      goto LAB_140323686;
      break;
    case 9:
      if ((((char)local_58[1] == 'n' && *local_58 == 0x61696c697a617262) ||
          ((char)local_58[1] == 'n' && *local_58 == 0x6169676577726f6e)) ||
         ((char)local_58[1] == 'n' && *local_58 == 0x61696e6961726b75)) goto LAB_140323686;
    }
  }
  if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_e8 = &local_60;
    local_e0 = alloc::string::impl$23::fmt;
    local_d8 = &PTR_DAT_1429bd640;
    local_d0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_c8 = (longlong **)&PTR_s_Unsupported_Steam_language___Fal_1429bd680;
    local_c0 = (void *)0x2;
    local_a8 = 0;
    local_b8 = &local_e8;
    local_b0 = 2;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_98 = (longlong *)&DAT_1429bd6e8;
    local_90 = 0x21;
    local_88 = &DAT_1429bd6e8;
    local_80 = 0x21;
    ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
  }
LAB_140323686:
  lang_lib::load_locale();
  if (extraout_RAX_02 != 0) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (local_60 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


