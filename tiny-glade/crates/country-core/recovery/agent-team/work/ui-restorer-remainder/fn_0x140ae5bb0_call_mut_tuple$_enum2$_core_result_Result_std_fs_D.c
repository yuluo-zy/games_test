void __cdecl call_mut<tuple$<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error>_>_>,country_core::resources::grading_lut::impl$3::load::closure_env$0>(void)


void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<enum2_<core::result::Result<std::fs::DirEntry,std::io::error::Error>_>_>,country_core::resources::grading_lut::impl_3::load::closure_env_0>
          (void)

{
  longlong *plVar1;
  int *_Src;
  char **ppcVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 *in_RCX;
  undefined8 *in_R8;
  char **local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  longlong *local_100;
  code *local_f8;
  int **local_f0;
  code *local_e8;
  longlong *local_e0;
  code *local_d8;
  char *local_d0;
  undefined8 local_c8;
  longlong local_b8 [4];
  char **local_98;
  undefined1 *local_90;
  char **local_88;
  undefined8 local_80;
  undefined8 local_78;
  int *local_68;
  undefined1 *local_60;
  char **local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  plVar1 = (longlong *)*in_R8;
  _Src = (int *)in_R8[1];
  if (plVar1 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    result::unwrap_failed();
  }
  memcpy(&local_348,in_R8 + 2,0x248);
  std::fs::DirEntry::path();
  LOCK();
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if (*plVar1 == 0) {
    ::alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
  }
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::extension();
  if ((extraout_RAX != 0) &&
     (std::ffi::os_str::impl$41::try_from(), ppcVar2 = local_348, ((ulonglong)plVar1 & 1) == 0)) {
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::path::Path::file_stem();
    if ((extraout_RAX_00 != 0) &&
       (std::ffi::os_str::impl$41::try_from(), ((ulonglong)plVar1 & 1) == 0)) {
      local_58 = local_348;
      if ((longlong)local_348 < 0) {
LAB_140ae609a:
                    // WARNING: Subroutine does not return
        ::alloc::raw_vec::handle_error();
      }
      if ((longlong **)local_348 == (longlong **)0x0) {
        local_60 = (undefined1 *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_60 = extraout_RAX_01;
        if (extraout_RAX_01 == (undefined1 *)0x0) goto LAB_140ae609a;
      }
      memcpy(local_60,_Src,(size_t)local_58);
      if (ppcVar2 == (char **)&DAT_00000004) {
        if (*_Src == 0x65627563) {
          local_49 = 1;
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          local_98 = local_58;
          local_90 = local_60;
          local_88 = local_58;
          local_49 = 0;
          country_core::resources::grading_lut::GradingLut::load_cube();
          if (SBORROW8(0,(longlong)plVar1)) {
            local_68 = _Src;
            if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
              local_f8 = std::path::impl$44::fmt;
              local_f0 = &local_68;
              local_e8 = anyhow::error::impl$5::fmt;
              local_98 = (char **)&DAT_142ae32f0;
              local_90 = (undefined1 *)0x2;
              local_78 = 0;
              local_88 = (char **)&local_100;
              local_80 = 2;
              local_100 = local_b8;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_e0 = (longlong *)0x142ae32b0;
              local_d8 = (code *)0x24;
              local_d0 = "country_core::resources::grading_lutFailed parsing LUT : ";
              local_c8 = 0x24;
              ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
            }
            local_49 = 0;
            anyhow::error::impl$6::drop();
          }
          else {
            in_RCX[0xb] = uStack_300;
            in_RCX[9] = uStack_310;
            in_RCX[10] = local_308;
            in_RCX[7] = uStack_320;
            in_RCX[8] = local_318;
            in_RCX[5] = uStack_330;
            in_RCX[6] = local_328;
            in_RCX[3] = uStack_340;
            in_RCX[4] = local_338;
            in_RCX[1] = _Src;
            in_RCX[2] = local_348;
          }
          *in_RCX = plVar1;
          goto LAB_140ae5ca7;
        }
      }
      else if (((longlong **)ppcVar2 == (longlong **)0x3) &&
              (*(char *)((longlong)_Src + 2) == 'g' && (short)*_Src == 0x6e70)) {
        local_98 = local_58;
        local_90 = local_60;
        local_88 = local_58;
        local_49 = 0;
        country_core::resources::grading_lut::GradingLut::load_unwrapped();
        in_RCX[10] = local_308;
        in_RCX[0xb] = uStack_300;
        in_RCX[8] = local_318;
        in_RCX[9] = uStack_310;
        in_RCX[6] = local_328;
        in_RCX[7] = uStack_320;
        in_RCX[4] = local_338;
        in_RCX[5] = uStack_330;
        in_RCX[2] = local_348;
        in_RCX[3] = uStack_340;
        *in_RCX = plVar1;
        in_RCX[1] = _Src;
        goto LAB_140ae5ca7;
      }
      if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_d8 = std::path::impl$44::fmt;
        local_338 = 0;
        local_348 = (char **)&local_e0;
        uStack_340 = 1;
        local_49 = 1;
        local_e0 = local_b8;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_98 = (char **)0x142ae32b0;
        local_90 = &DAT_00000024;
        local_88 = (char **)0x142ae32b0;
        local_80 = 0x24;
        local_49 = 1;
        local_78 = extraout_RAX_02;
        ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
      }
      *in_RCX = 0x8000000000000000;
      if ((longlong **)local_58 != (longlong **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_140ae5ca7;
    }
  }
  *in_RCX = 0x8000000000000000;
LAB_140ae5ca7:
  if (local_b8[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

