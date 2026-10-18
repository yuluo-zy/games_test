// dump_screenshot @ 0x14058a060 size=1429
// sig: void __cdecl dump_screenshot(void)


void __cdecl tiny_glade::panic_reporter::panic_screenshot::dump_screenshot(void)

{
  undefined1 auVar1 [16];
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong lVar2;
  longlong extraout_RAX_05;
  longlong *in_RCX;
  ulonglong uVar3;
  bool bVar4;
  undefined8 *****local_160;
  code *pcStack_158;
  uint *local_150;
  code *pcStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  longlong local_120;
  longlong local_110;
  longlong local_108;
  longlong lStack_100;
  longlong local_f8;
  longlong lStack_f0;
  undefined8 *****local_e8;
  code *pcStack_e0;
  uint *local_d8;
  code *pcStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined **local_a0;
  longlong lStack_98;
  undefined8 *****local_90;
  code *pcStack_88;
  uint *puStack_80;
  code *pcStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  uint local_44;
  uint local_40;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  std::path::Path::_join();
  LOCK();
  bVar4 = DAT_14362b0f0 == '\0';
  if (bVar4) {
    DAT_14362b0f0 = '\x01';
  }
  UNLOCK();
  if (!bVar4) {
    local_39 = 1;
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (DAT_14362b128._6_1_ == '\x05') {
    local_a0 = &PTR_DAT_1429fd620;
    lStack_98 = 1;
    local_90 = (undefined8 *****)&DAT_00000008;
    pcStack_88 = (code *)0x0;
    puStack_80 = (uint *)0x0;
    anyhow::__private::format_err();
    lVar2 = extraout_RAX;
  }
  else {
    if (DAT_14362b100 < 4) {
                    // WARNING: Subroutine does not return
      core::slice::index::slice_end_index_len_fail();
    }
    local_40 = *DAT_14362b0f8;
    if (DAT_14362b100 < 8) {
                    // WARNING: Subroutine does not return
      core::slice::index::slice_end_index_len_fail();
    }
    local_44 = DAT_14362b0f8[1];
    if (local_40 == 0 || (ulonglong)local_44 == 0) {
      local_e8 = (undefined8 *****)&local_40;
      pcStack_e0 = core::fmt::num::imp::impl$6::fmt;
      local_d8 = &local_44;
      pcStack_d0 = core::fmt::num::imp::impl$6::fmt;
      local_a0 = &PTR_s_Invalid_screenshot_data_dimensio_1429fd548;
      lStack_98 = 3;
      local_90 = &local_e8;
      pcStack_88 = (code *)0x2;
      puStack_80 = (uint *)0x0;
      alloc::fmt::format::format_inner();
      anyhow::Error::msg<alloc::string::String>();
      lVar2 = extraout_RAX_00;
    }
    else {
      uVar3 = (ulonglong)local_44 * (ulonglong)local_40;
      if (uVar3 < 0x38401) {
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        std::path::Path::parent();
        if (extraout_RAX_02 != 0) {
          local_a0 = (undefined **)CONCAT71(local_a0._1_7_,1);
          std::fs::DirBuilder::_create();
          if (extraout_RAX_03 != 0) {
            anyhow::error::impl$1::from<std::io::error::Error>();
            lVar2 = extraout_RAX_04;
            goto LAB_14058a2bd;
          }
        }
        if (DAT_14362b100 < uVar3 * 2 + 8) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        lStack_98 = uVar3 * 2;
        local_a0 = (undefined **)(DAT_14362b0f8 + 2);
        local_90 = (undefined8 *****)0x4;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<array_<array_<u8,3>,2>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env_0>_>
                  ();
        auVar1._8_8_ = 0;
        auVar1._0_8_ = (ulonglong)local_40 * 3;
        if ((SUB168(auVar1 * ZEXT416(local_44),8) != 0) ||
           ((ulonglong)(local_110 * 6) < SUB168(auVar1 * ZEXT416(local_44),0))) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        image::io::free_functions::save_buffer_impl();
        if (local_160 == (undefined8 *****)&DAT_800000000000000a) {
          in_RCX[2] = local_f8;
          in_RCX[3] = lStack_f0;
          *in_RCX = local_108;
          in_RCX[1] = lStack_100;
          if (local_120 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          local_39 = 0;
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          bVar4 = DAT_14362b0f0 == '\x01';
          if (bVar4) {
            DAT_14362b0f0 = '\0';
          }
          UNLOCK();
          if (bVar4) {
            return;
          }
          local_39 = 0;
          parking_lot::raw_mutex::RawMutex::unlock_slow();
          return;
        }
        local_b8 = local_130;
        uStack_b0 = uStack_128;
        local_c8 = local_140;
        uStack_c0 = uStack_138;
        local_d8 = local_150;
        pcStack_d0 = pcStack_148;
        local_e8 = local_160;
        pcStack_e0 = pcStack_158;
        local_3a = 1;
        std::backtrace::Backtrace::capture();
        local_60 = local_b8;
        uStack_58 = uStack_b0;
        local_70 = local_c8;
        uStack_68 = uStack_c0;
        puStack_80 = local_d8;
        pcStack_78 = pcStack_d0;
        local_90 = local_e8;
        pcStack_88 = pcStack_e0;
        local_a0 = (undefined **)0x1429fd5a8;
        lStack_98 = 0xc;
        local_3a = 0;
        anyhow::Error::
        construct<anyhow::error::ContextError<ref_<str_>,enum2_<image::error::ImageError>_>_>();
        in_RCX[1] = extraout_RAX_05;
        *in_RCX = -0x8000000000000000;
        if (local_120 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_14058a2ce;
      }
      local_e8 = (undefined8 *****)&local_40;
      pcStack_e0 = core::fmt::num::imp::impl$6::fmt;
      local_d8 = &local_44;
      pcStack_d0 = core::fmt::num::imp::impl$6::fmt;
      local_a0 = &PTR_s_Invalid_screenshot_data_dimensio_1429fd5d0;
      lStack_98 = 3;
      local_90 = &local_e8;
      pcStack_88 = (code *)0x2;
      puStack_80 = (uint *)0x0;
      alloc::fmt::format::format_inner();
      anyhow::Error::msg<alloc::string::String>();
      lVar2 = extraout_RAX_01;
    }
  }
LAB_14058a2bd:
  in_RCX[1] = lVar2;
  *in_RCX = -0x8000000000000000;
LAB_14058a2ce:
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar4 = DAT_14362b0f0 == '\x01';
  if (bVar4) {
    DAT_14362b0f0 = '\0';
  }
  UNLOCK();
  if (!bVar4) {
    local_39 = 1;
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  if (local_108 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


