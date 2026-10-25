// 1403ace70 void __cdecl dump_inputs(void)

void __cdecl tiny_glade::panic_reporter::panic_dump::dump_inputs(void)

{
  ulonglong uVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong *in_RCX;
  ulonglong uVar2;
  longlong lVar3;
  undefined **local_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong *local_d8;
  code *local_d0;
  undefined ***local_c8;
  code *local_c0;
  longlong **local_b8;
  undefined8 local_b0;
  undefined ***local_a8;
  undefined8 local_a0;
  longlong local_98;
  undefined **local_90;
  longlong local_88;
  undefined ****local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined1 local_60;
  longlong local_58;
  longlong local_50;
  undefined **local_48;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if (*(int *)
       __imp__ZN12country_core7startup22panic_reporter_startup19PANIC_REPORTER_INFO17hbf0a1901d5a3098fE
      == 2) {
    local_90 = (undefined **)&DAT_1429cb9b8;
    local_88 = 1;
    local_80 = (undefined ****)0x8;
    local_78 = 0;
    uStack_70 = 0;
    anyhow::__private::format_err();
    in_RCX[1] = extraout_RAX_10;
    *in_RCX = -0x8000000000000000;
    return;
  }
  lVar3 = *(longlong *)
           (
           __imp__ZN12country_core7startup22panic_reporter_startup19PANIC_REPORTER_INFO17hbf0a1901d5a3098fE
           + 0x30);
  std::path::Path::_join();
  uVar2 = *(ulonglong *)(lVar3 + 0x40) ^ 0x8000000000000000;
  uVar1 = 1;
  if (uVar2 < 5) {
    uVar1 = uVar2;
  }
  if (uVar1 == 1) {
    local_100 = (undefined **)(lVar3 + 0x40);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_90 = (undefined **)&DAT_00000080;
    local_80 = (undefined ****)0x0;
    local_c8 = &local_90;
    local_88 = extraout_RAX_01;
    country_core::input::input_stream::_::impl$0::
    serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
              ();
    if (extraout_RAX_02 == 0) {
      if (!SBORROW8(0,(longlong)local_90)) {
        local_48 = local_90;
        local_50 = local_88;
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::fs::write::inner();
        if (local_48 != (undefined **)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (extraout_RAX_08 == 0) {
          local_c8 = &local_100;
          local_c0 = core::fmt::impl$74::fmt<std::path::PathBuf>;
          local_90 = (undefined **)&DAT_1429cba18;
          local_88 = 1;
          uStack_70 = 0;
          local_78 = 1;
          local_80 = &local_c8;
          anyhow::__private::format_err();
          lVar3 = extraout_RAX_11;
          goto LAB_1403ad03c;
        }
        local_d8 = &local_58;
        local_d0 = std::io::error::impl$9::fmt;
        local_90 = (undefined **)0x2;
        local_80 = (undefined ****)0x2;
        uStack_70 = 0;
        local_68 = 0x400000020;
        local_60 = 3;
        local_c8 = (undefined ***)&DAT_1429cba70;
        local_c0 = (code *)0x1;
        local_a0 = 1;
        local_b8 = &local_d8;
        local_b0 = 1;
        local_a8 = &local_90;
        local_58 = extraout_RAX_08;
        anyhow::__private::format_err();
        lVar3 = extraout_RAX_09;
        if (((uint)local_58 & 3) != 1) goto LAB_1403ad03c;
        local_98 = local_58 + -1;
        local_50 = *(longlong *)(local_58 + -1);
        local_48 = *(undefined ***)(local_58 + 7);
        if ((code *)*local_48 != (code *)0x0) {
          (*(code *)*local_48)(local_50);
        }
LAB_1403ad270:
        if (local_48[1] != (undefined *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        goto LAB_1403ad03c;
      }
    }
    else if (local_90 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    anyhow::error::impl$1::from<serde_json::error::Error>();
    lVar3 = extraout_RAX_05;
  }
  else {
    if (uVar1 != 0) {
      local_90 = &PTR_s_Input_record_replay_state_was_no_1429cba60;
      local_88 = 1;
      local_80 = (undefined ****)&DAT_00000008;
      local_78 = 0;
      uStack_70 = 0;
      anyhow::__private::format_err();
      lVar3 = extraout_RAX_03;
      goto LAB_1403ad03c;
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_90 = (undefined **)&DAT_00000080;
    local_80 = (undefined ****)0x0;
    local_c8 = &local_90;
    local_88 = extraout_RAX;
    country_core::input::input_stream::_::impl$0::
    serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
              ();
    if (extraout_RAX_00 == 0) {
      if (!SBORROW8(0,(longlong)local_90)) {
        local_48 = local_90;
        local_50 = local_88;
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::fs::write::inner();
        if (local_48 != (undefined **)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (extraout_RAX_06 == 0) {
          in_RCX[2] = local_e8;
          in_RCX[3] = lStack_e0;
          *in_RCX = local_f8;
          in_RCX[1] = lStack_f0;
          return;
        }
        local_d8 = &local_58;
        local_d0 = std::io::error::impl$9::fmt;
        local_90 = (undefined **)0x2;
        local_80 = (undefined ****)0x2;
        uStack_70 = 0;
        local_68 = 0x400000020;
        local_60 = 3;
        local_c8 = (undefined ***)&DAT_1429cba70;
        local_c0 = (code *)0x1;
        local_a0 = 1;
        local_b8 = &local_d8;
        local_b0 = 1;
        local_a8 = &local_90;
        local_58 = extraout_RAX_06;
        anyhow::__private::format_err();
        lVar3 = extraout_RAX_07;
        if (((uint)local_58 & 3) != 1) goto LAB_1403ad03c;
        local_98 = local_58 + -1;
        local_50 = *(longlong *)(local_58 + -1);
        local_48 = *(undefined ***)(local_58 + 7);
        if ((code *)*local_48 != (code *)0x0) {
          (*(code *)*local_48)(local_50);
        }
        goto LAB_1403ad270;
      }
    }
    else if (local_90 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    anyhow::error::impl$1::from<serde_json::error::Error>();
    lVar3 = extraout_RAX_04;
  }
LAB_1403ad03c:
  in_RCX[1] = lVar3;
  *in_RCX = -0x8000000000000000;
  if (local_f8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

