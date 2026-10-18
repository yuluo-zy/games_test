// create_sentry_attachment @ 0x1404b4d50
// sig: void __cdecl create_sentry_attachment(void)


// WARNING: Type propagation algorithm not settling

void __cdecl tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment(void)

{
  ulonglong uVar1;
  int *******pppppppiVar2;
  undefined8 extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  void *extraout_RAX_03;
  longlong extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 uVar3;
  void *_Dst;
  int *******extraout_RAX_08;
  int *******pppppppiVar4;
  undefined8 *in_RCX;
  longlong *in_RDX;
  undefined *_Src;
  ulonglong *in_R8;
  size_t _Size;
  int *******local_180;
  int *******local_178;
  longlong local_148;
  undefined8 *******local_128;
  int *******pppppppiStack_120;
  int *******local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong lStack_b8;
  void *local_b0;
  void *local_a8;
  undefined8 *******local_a0;
  int *******pppppppiStack_98;
  int *******local_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  size_t local_48;
  int *******local_40;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_d0 = *in_RDX;
  if (SBORROW8(0,local_d0)) {
    local_118 = (int *******)in_RDX[3];
    local_128 = (undefined8 *******)in_RDX[1];
    pppppppiStack_120 = (int *******)in_RDX[2];
    local_178 = (int *******)alloc::string::impl$23::fmt;
    local_a0 = (undefined8 *******)&DAT_1429e8f48;
    pppppppiStack_98 = (int *******)0x1;
    uStack_80 = 0;
    local_90 = (int *******)&local_180;
    uStack_88 = 1;
    local_180 = (int *******)&local_128;
    anyhow::__private::format_err();
    if (local_128 != (undefined8 *******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    in_RCX[1] = extraout_RAX;
    *in_RCX = 0x8000000000000000;
    return;
  }
  local_c8 = in_RDX[1];
  local_c0 = in_RDX[2];
  lStack_b8 = in_RDX[3];
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::file_name();
  if (extraout_RAX_00 == 0) {
    std::backtrace::Backtrace::capture();
    anyhow::Error::construct<anyhow::wrapper::DisplayError<ref_<str_>_>_>();
    uVar3 = extraout_RAX_02;
  }
  else {
    std::ffi::os_str::impl$41::try_from();
    pppppppiVar4 = local_90;
    pppppppiVar2 = pppppppiStack_98;
    if ((int)local_a0 != 1) {
      if ((longlong)local_90 < 0) {
LAB_1404b537f:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (local_90 == (int *******)0x0) {
        local_b0 = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_b0 = extraout_RAX_03;
        if (extraout_RAX_03 == (void *)0x0) goto LAB_1404b537f;
      }
      local_40 = pppppppiVar4;
      memcpy(local_b0,pppppppiVar2,(size_t)pppppppiVar4);
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::path::Path::extension();
      if (extraout_RAX_04 == 0) {
        std::backtrace::Backtrace::capture();
        anyhow::Error::construct<anyhow::wrapper::DisplayError<ref_<str_>_>_>();
        uVar3 = extraout_RAX_06;
LAB_1404b5112:
        in_RCX[1] = uVar3;
        *in_RCX = 0x8000000000000000;
        pppppppiVar2 = local_40;
      }
      else {
        std::ffi::os_str::impl$41::try_from();
        if ((int)local_a0 == 1) {
          local_a0 = (undefined8 *******)&PTR_s_Corrupt_attachment_extension_1429e9010;
          pppppppiStack_98 = (int *******)0x1;
          local_90 = (int *******)&DAT_00000008;
          uStack_88 = 0;
          uStack_80 = 0;
          anyhow::__private::format_err();
          uVar3 = extraout_RAX_05;
          goto LAB_1404b5112;
        }
        local_180 = pppppppiStack_98;
        local_178 = local_90;
        if (local_90 == (int *******)&DAT_00000004) {
          if (*(int *)pppppppiStack_98 != 0x6e6f736a) goto LAB_1404b50ba;
          _Size = 0x10;
          _Src = &DAT_1429e908b;
        }
        else {
          if (local_90 != (int *******)0x3) {
LAB_1404b50ba:
            local_128 = &local_180;
            pppppppiStack_120 = (int *******)core::fmt::impl$74::fmt<str_>;
            local_a0 = (undefined8 *******)&PTR_s_Unrecognized_attachment_file_nam_1429e9050;
            pppppppiStack_98 = (int *******)0x1;
            uStack_80 = 0;
            uStack_88 = 1;
            local_90 = (int *******)&local_128;
            anyhow::__private::format_err();
            uVar3 = extraout_RAX_07;
            goto LAB_1404b5112;
          }
          _Size = 10;
          if (*(char *)((longlong)pppppppiStack_98 + 2) == 'g' &&
              *(short *)pppppppiStack_98 == 0x706a) {
            _Src = &DAT_1429e909b;
          }
          else {
            _Src = &DAT_1429e9081;
            if (((*(char *)((longlong)pppppppiStack_98 + 2) != 't' ||
                  *(short *)pppppppiStack_98 != 0x7874) &&
                (*(char *)((longlong)pppppppiStack_98 + 2) != 'n' ||
                 *(short *)pppppppiStack_98 != 0x6f72)) &&
               (*(char *)((longlong)pppppppiStack_98 + 2) != 'g' ||
                *(short *)pppppppiStack_98 != 0x6f6c)) {
              if (*(char *)((longlong)pppppppiStack_98 + 2) == 'p' &&
                  *(short *)pppppppiStack_98 == 0x6d64) {
                _Size = 0x12;
                _Src = &DAT_1429e906f;
              }
              else {
                if (*(char *)((longlong)pppppppiStack_98 + 2) != 'p' ||
                    *(short *)pppppppiStack_98 != 0x697a) goto LAB_1404b50ba;
                _Size = 0xf;
                _Src = &DAT_1429e9060;
              }
            }
          }
        }
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        local_a8 = _Dst;
        local_48 = _Size;
        memcpy(_Dst,_Src,_Size);
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::sys::pal::windows::fs::stat();
        if ((int)local_a0 == 2) {
          local_128 = (undefined8 *******)0x2;
        }
        else {
          local_d8 = local_50;
          local_e8 = local_60;
          uStack_e0 = uStack_58;
          local_f8 = local_70;
          uStack_f0 = uStack_68;
          local_108 = uStack_80;
          uStack_100 = uStack_78;
          local_118 = local_90;
          uStack_110 = uStack_88;
          local_128 = local_a0;
        }
        pppppppiStack_120 = pppppppiStack_98;
        anyhow::context::impl$0::
        with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env_3>
                  ();
        pppppppiVar4 = local_178;
        if ((int)local_180 != 2) {
          uVar1 = *in_R8;
          if (local_148 + uVar1 < 0x1400001) {
            std::sys::os_str::wtf8::Buf::as_mut_slice();
            std::fs::read::inner();
            anyhow::context::impl$0::
            with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env_4>
                      ();
            pppppppiVar4 = pppppppiStack_120;
            if (!SBORROW8(0,(longlong)local_128)) {
              *in_R8 = local_148 + uVar1;
              in_RCX[6] = local_48;
              in_RCX[7] = local_a8;
              in_RCX[8] = local_48;
              in_RCX[3] = local_40;
              in_RCX[4] = local_b0;
              in_RCX[5] = local_40;
              *in_RCX = local_128;
              in_RCX[1] = pppppppiStack_120;
              in_RCX[2] = local_118;
              *(undefined1 *)(in_RCX + 9) = 5;
              goto LAB_1404b513e;
            }
          }
          else {
            local_a0 = (undefined8 *******)&DAT_1429e8fb0;
            pppppppiStack_98 = (int *******)0x1;
            local_90 = (int *******)&DAT_00000008;
            uStack_88 = 0;
            uStack_80 = 0;
            anyhow::__private::format_err();
            pppppppiVar4 = extraout_RAX_08;
          }
        }
        pppppppiVar2 = local_40;
        in_RCX[1] = pppppppiVar4;
        *in_RCX = 0x8000000000000000;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (pppppppiVar2 != (int *******)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_1404b513e;
    }
    local_a0 = (undefined8 *******)&PTR_s_Corrupt_attachment_file_name_1429e8fe0;
    pppppppiStack_98 = (int *******)0x1;
    local_90 = (int *******)&DAT_00000008;
    uStack_88 = 0;
    uStack_80 = 0;
    anyhow::__private::format_err();
    uVar3 = extraout_RAX_01;
  }
  in_RCX[1] = uVar3;
  *in_RCX = 0x8000000000000000;
LAB_1404b513e:
  if (local_d0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

