// write_image_with_callback_impl @ 0x1403206a0
// sig: void __cdecl write_image_with_callback_impl(void)


// WARNING: Function: __chkstk replaced with injection: alloca_probe

void __cdecl tiny_glade::screenshot::write_image_with_callback_impl(void)

{
  byte bVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined8 uVar7;
  undefined4 *extraout_RAX;
  longlong lVar8;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  short *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *in_RCX;
  ulonglong in_RDX;
  ulonglong uVar9;
  ulonglong in_R9;
  bool bVar10;
  byte *in_stack_00000028;
  undefined8 **local_2220;
  undefined1 local_2218;
  undefined7 uStack_2217;
  undefined1 uStack_2210;
  undefined7 uStack_220f;
  undefined4 local_2208;
  uint uStack_2204;
  undefined8 uStack_2200;
  undefined1 local_21f8;
  undefined7 uStack_21f7;
  undefined1 uStack_21f0;
  undefined7 uStack_21ef;
  undefined1 local_21e8;
  undefined7 uStack_21e7;
  undefined8 **local_11b8;
  undefined1 uStack_11b0;
  undefined7 uStack_11af;
  undefined1 uStack_11a8;
  undefined7 uStack_11a7;
  undefined1 uStack_11a0;
  undefined7 uStack_119f;
  undefined4 uStack_1198;
  undefined4 uStack_1194;
  undefined1 local_1190;
  undefined7 uStack_118f;
  undefined1 uStack_1188;
  undefined1 uStack_1187;
  undefined6 uStack_1186;
  undefined1 local_1180;
  undefined7 uStack_117f;
  undefined4 local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  undefined8 **local_120;
  code *local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  longlong *local_d8;
  longlong local_d0;
  longlong local_c8;
  undefined8 **local_a8;
  undefined8 uStack_a0;
  longlong lStack_98;
  undefined4 uStack_90;
  uint uStack_8c;
  undefined8 *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_68;
  longlong local_60;
  undefined8 *local_58;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = (in_RDX & 0xffffffff) << 2;
  auVar5._8_8_ = 0;
  auVar5._0_8_ = in_RDX >> 0x20;
  if (in_R9 < SUB168(auVar3 * auVar5,0) || SUB168(auVar3 * auVar5,8) != 0) {
    local_11b8 = (undefined8 **)&PTR_s_Failed_to_create_ImageBuffer_1429bcb88;
    uStack_11b0 = 1;
    uStack_11af = 0;
    uStack_11a8 = 8;
    uStack_11a7 = 0;
    uStack_11a0 = 0;
    uStack_119f = 0;
    uStack_1198 = 0;
    uStack_1194 = 0;
    local_49 = 0;
    anyhow::__private::format_err();
  }
  else {
    lVar8 = 0;
    if (*in_RCX < -0x7ffffffffffffffe) {
      lVar8 = *in_RCX + -0x7fffffffffffffff;
    }
    if (lVar8 == 0) {
      local_49 = 0;
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      local_49 = 0;
      std::path::Path::parent();
      if (extraout_RAX_00 != 0) {
        local_11b8 = (undefined8 **)CONCAT71(local_11b8._1_7_,1);
        local_49 = 0;
        std::fs::DirBuilder::_create();
        if (extraout_RAX_01 != 0) {
          local_49 = 0;
          anyhow::error::impl$1::from<std::io::error::Error>();
          goto LAB_14032073e;
        }
      }
      local_49 = 0;
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      local_49 = 0;
      std::path::Path::parent();
      if (extraout_RAX_02 == 0) {
        local_49 = 0;
        std::backtrace::Backtrace::capture();
        local_49 = 0;
        anyhow::Error::construct<anyhow::wrapper::DisplayError<ref_<str_>_>_>();
        bVar1 = *in_stack_00000028;
        goto joined_r0x000140320d87;
      }
      local_1190 = 6;
      uStack_118f = 0;
      uStack_11b0 = 0xa3;
      uStack_11af = 0x1429345;
      uStack_11a8 = 4;
      uStack_11a7 = 0;
      uStack_11a0 = 1;
      uStack_119f = 0;
      uStack_1198 = 0;
      uStack_1194 = 0;
      uStack_1188 = 0;
      uStack_1187 = 0;
      local_11b8 = (undefined8 **)((ulonglong)local_11b8 & 0xffffffff00000000);
      local_49 = 0;
      tempfile::util::
      create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl_1::tempfile_in::closure_env_0<ref_<std::path::Path>_>_>
                ();
      if ((char)local_2208 == '\x02') {
        local_49 = 0;
        anyhow::error::impl$1::from<std::io::error::Error>();
      }
      else {
        uStack_a0 = CONCAT71(uStack_2217,local_2218);
        lStack_98 = CONCAT71(uStack_220f,uStack_2210);
        uVar9 = (ulonglong)uStack_2204;
        uStack_8c = uStack_2204;
        local_a8 = local_2220;
        uStack_90 = local_2208;
        local_4a = 1;
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        local_4a = 1;
        std::path::Path::extension();
        if ((uVar9 == 3 && extraout_RAX_03 != (short *)0x0) &&
           ((char)extraout_RAX_03[1] == 'g' && *extraout_RAX_03 == 0x706a)) {
          local_4a = 1;
          image::buffer_::impl$50::
          convert<ref_<slice2_<u8>_>,image::color::Rgba<u8>,image::color::Rgb<u8>_>();
          std::io::buffered::bufwriter::BufWriter::
          new<ref_mut_<tempfile::file::NamedTempFile<std::fs::File>_>_>();
          local_d8 = &local_d0;
          image::codecs::jpeg::encoder::JpegEncoder::
          new_with_quality<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>();
          memcpy(&local_11b8,&local_2220,0x1068);
          image::codecs::jpeg::encoder::impl$4::
          write_image<ref_mut_<ref_mut_<std::io::buffered::bufwriter::BufWriter<ref_mut_<tempfile::file::NamedTempFile<std::fs::File>_>_>_>_>_>
                    ();
          if (local_120 == (undefined8 **)&DAT_800000000000000a) {
            std::io::buffered::bufwriter::BufWriter::
            flush_buf<ref_mut_<tempfile::file::NamedTempFile<std::fs::File>_>_>();
            if (extraout_RAX_04 == 0) {
              core::ptr::
              drop_in_place<std::io::buffered::bufwriter::BufWriter<ref_mut_<tempfile::file::NamedTempFile<std::fs::File>_>_>_>
                        ();
              if (local_88 != (undefined8 *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
LAB_140320eae:
              uStack_11a8 = (undefined1)lStack_98;
              uStack_11a7 = (undefined7)((ulonglong)lStack_98 >> 8);
              uStack_11a0 = (undefined1)uStack_90;
              uStack_119f = (undefined7)(CONCAT44(uStack_8c,uStack_90) >> 8);
              local_11b8 = local_a8;
              uStack_11b0 = (undefined1)uStack_a0;
              uStack_11af = (undefined7)((ulonglong)uStack_a0 >> 8);
              local_4a = 0;
              tempfile::file::NamedTempFile::persist<std::fs::File,ref_<std::path::PathBuf>_>();
              if ((char)uStack_2200 == '\x02') {
                CloseHandle(local_2220);
                bVar1 = *in_stack_00000028;
joined_r0x000140320f0f:
                if ((bVar1 & 1) == 0) {
                  return;
                }
                local_68 = 0;
                lVar8 = *(longlong *)(in_stack_00000028 + 8);
                lVar2 = *(longlong *)(in_stack_00000028 + 0x10);
                local_2218 = (undefined1)lVar2;
                uStack_2217 = (undefined7)((ulonglong)lVar2 >> 8);
                local_2220 = (undefined8 **)0x0;
                if (lVar8 != 0) {
                  local_11b8 = (undefined8 **)0x0;
                  uStack_11b0 = (undefined1)in_RDX;
                  uStack_11af = (undefined7)(in_RDX >> 8);
                  local_60 = lVar8;
                  local_58 = (undefined8 *)lVar2;
                  (**(code **)(lVar2 + 0x18))(lVar8,&local_11b8);
                  if (*(longlong *)((longlong)local_58 + 8) != 0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>();
                  return;
                }
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              uStack_11a0 = (undefined1)local_2208;
              uStack_119f = (undefined7)(CONCAT44(uStack_2204,local_2208) >> 8);
              uStack_11b0 = local_2218;
              uStack_11af = uStack_2217;
              uStack_1194 = uStack_2200._4_4_;
              local_11b8 = local_2220;
              local_4a = 0;
              anyhow::error::impl$1::from<tempfile::file::PersistError<std::fs::File>_>();
              goto LAB_14032073e;
            }
            anyhow::error::impl$1::from<std::io::error::Error>();
          }
          else {
            local_1180 = (undefined1)local_e8;
            uStack_117f = (undefined7)((ulonglong)local_e8 >> 8);
            local_1190 = (undefined1)local_f8;
            uStack_118f = (undefined7)((ulonglong)local_f8 >> 8);
            uStack_1188 = (undefined1)uStack_f0;
            uStack_1187 = (undefined1)((ulonglong)uStack_f0 >> 8);
            uStack_1186 = (undefined6)((ulonglong)uStack_f0 >> 0x10);
            uStack_11a0 = (undefined1)local_108;
            uStack_119f = (undefined7)((ulonglong)local_108 >> 8);
            uStack_1198 = (undefined4)uStack_100;
            uStack_1194 = (undefined4)((ulonglong)uStack_100 >> 0x20);
            uStack_11b0 = SUB81(local_118,0);
            uStack_11af = (undefined7)((ulonglong)local_118 >> 8);
            uStack_11a8 = (undefined1)uStack_110;
            uStack_11a7 = (undefined7)((ulonglong)uStack_110 >> 8);
            local_11b8 = local_120;
            anyhow::error::impl$1::from<enum2_<image::error::ImageError>_>();
          }
          core::ptr::
          drop_in_place<std::io::buffered::bufwriter::BufWriter<ref_mut_<tempfile::file::NamedTempFile<std::fs::File>_>_>_>
                    ();
          if (local_88 != (undefined8 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        else {
          local_4a = 1;
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          local_4a = 1;
          image::image::impl$0::from_path::inner();
          if (local_2220 == (undefined8 **)&DAT_800000000000000a) {
            auVar4._8_8_ = 0;
            auVar4._0_8_ = (in_RDX & 0xffffffff) << 2;
            auVar6._8_8_ = 0;
            auVar6._0_8_ = in_RDX >> 0x20;
            if (SUB168(auVar4 * auVar6,8) != 0) {
              local_4a = 1;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            if (in_R9 < SUB168(auVar4 * auVar6,0)) {
              local_4a = 1;
                    // WARNING: Subroutine does not return
              core::slice::index::slice_end_index_len_fail();
            }
            local_4a = 1;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_4a = 1;
            image::io::free_functions::
            write_buffer_impl<tempfile::file::NamedTempFile<std::fs::File>_>();
            if (local_2220 == (undefined8 **)&DAT_800000000000000a) goto LAB_140320eae;
            local_1180 = local_21e8;
            uStack_117f = uStack_21e7;
            local_1190 = local_21f8;
            uStack_118f = uStack_21f7;
            uStack_1188 = uStack_21f0;
            uStack_1187 = (undefined1)uStack_21ef;
            uStack_1186 = (undefined6)((uint7)uStack_21ef >> 8);
            uStack_11a0 = (undefined1)local_2208;
            uStack_119f = (undefined7)(CONCAT44(uStack_2204,local_2208) >> 8);
            uStack_1198 = (undefined4)uStack_2200;
            uStack_1194 = uStack_2200._4_4_;
            uStack_11b0 = local_2218;
            uStack_11af = uStack_2217;
            uStack_11a8 = uStack_2210;
            uStack_11a7 = uStack_220f;
            local_11b8 = local_2220;
            local_4a = 1;
            anyhow::error::impl$1::from<enum2_<image::error::ImageError>_>();
          }
          else {
            uStack_117f = uStack_21e7;
            uStack_119f = CONCAT43(uStack_2204,local_2208._1_3_);
            uStack_118f = uStack_21f7;
            uStack_1188 = uStack_21f0;
            uStack_1187 = (undefined1)uStack_21ef;
            uStack_1186 = (undefined6)((uint7)uStack_21ef >> 8);
            local_1180 = local_21e8;
            uStack_1198 = (undefined4)uStack_2200;
            uStack_1194 = uStack_2200._4_4_;
            local_1190 = local_21f8;
            uStack_11af = uStack_2217;
            uStack_11a8 = uStack_2210;
            uStack_11a7 = uStack_220f;
            uStack_11a0 = (char)local_2208;
            local_11b8 = local_2220;
            uStack_11b0 = local_2218;
            local_4a = 1;
            anyhow::error::impl$1::from<enum2_<image::error::ImageError>_>();
          }
        }
        tempfile::file::impl$6::drop();
        if (lStack_98 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        CloseHandle(local_a8);
      }
    }
    else {
      if (lVar8 != 1) {
        local_49 = 0;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_49 = 0;
      image::buffer_::impl$50::
      convert<ref_<slice2_<u8>_>,image::color::Rgba<u8>,image::color::Rgb<u8>_>();
      local_88 = (undefined8 *)0x0;
      local_80 = 1;
      local_78 = 0;
      local_4b = 1;
      image::codecs::jpeg::encoder::JpegEncoder::
      new_with_quality<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>();
      memcpy(&local_11b8,&local_2220,0x1068);
      local_4b = 1;
      image::codecs::jpeg::encoder::impl$4::
      write_image<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>();
      uVar7 = local_78;
      if (local_120 == (undefined8 **)&DAT_800000000000000a) {
        local_58 = local_88;
        local_60 = local_80;
        lVar8 = in_RCX[1];
        LOCK();
        bVar10 = *(char *)(lVar8 + 0x10) == '\0';
        if (bVar10) {
          *(char *)(lVar8 + 0x10) = '\x01';
        }
        UNLOCK();
        if (!bVar10) {
          parking_lot::raw_mutex::RawMutex::lock_slow();
        }
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        if ((!SBORROW8(0,*(longlong *)(lVar8 + 0x18))) && (*(longlong *)(lVar8 + 0x18) != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        *(undefined8 **)(lVar8 + 0x18) = local_58;
        *(longlong *)(lVar8 + 0x20) = local_60;
        *(undefined8 *)(lVar8 + 0x28) = uVar7;
        local_4b = 0;
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar10 = *(char *)(lVar8 + 0x10) == '\x01';
        if (bVar10) {
          *(char *)(lVar8 + 0x10) = '\0';
        }
        UNLOCK();
        if (!bVar10) {
          local_4b = 0;
          parking_lot::raw_mutex::RawMutex::unlock_slow();
        }
        if (local_d0 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        bVar1 = *in_stack_00000028;
        goto joined_r0x000140320f0f;
      }
      local_1180 = (undefined1)local_e8;
      uStack_117f = (undefined7)((ulonglong)local_e8 >> 8);
      local_1190 = (undefined1)local_f8;
      uStack_118f = (undefined7)((ulonglong)local_f8 >> 8);
      uStack_1188 = (undefined1)uStack_f0;
      uStack_1187 = (undefined1)((ulonglong)uStack_f0 >> 8);
      uStack_1186 = (undefined6)((ulonglong)uStack_f0 >> 0x10);
      uStack_11a0 = (undefined1)local_108;
      uStack_119f = (undefined7)((ulonglong)local_108 >> 8);
      uStack_1198 = (undefined4)uStack_100;
      uStack_1194 = (undefined4)((ulonglong)uStack_100 >> 0x20);
      uStack_11b0 = SUB81(local_118,0);
      uStack_11af = (undefined7)((ulonglong)local_118 >> 8);
      uStack_11a8 = (undefined1)uStack_110;
      uStack_11a7 = (undefined7)((ulonglong)uStack_110 >> 8);
      local_11b8 = local_120;
      local_4b = 1;
      anyhow::error::impl$1::from<enum2_<image::error::ImageError>_>();
      if (local_88 != (undefined8 *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_d0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
LAB_14032073e:
  bVar1 = *in_stack_00000028;
joined_r0x000140320d87:
  if ((bVar1 & 1) != 0) {
    local_88 = &local_68;
    lVar8 = *(longlong *)(in_stack_00000028 + 8);
    lVar2 = *(longlong *)(in_stack_00000028 + 0x10);
    local_120 = &local_88;
    local_118 = core::fmt::impl$74::fmt<anyhow::Error>;
    local_11b8 = (undefined8 **)0x2;
    uStack_11a8 = 2;
    uStack_11a7 = 0;
    uStack_1198 = 0;
    uStack_1194 = 0;
    local_1190 = 0x20;
    uStack_118f = 0x4000000;
    uStack_1188 = 3;
    local_2220 = (undefined8 **)&PTR_DAT_1429bca30;
    local_2218 = 1;
    uStack_2217 = 0;
    uStack_2210 = SUB81(&local_120,0);
    uStack_220f = (undefined7)((ulonglong)&local_120 >> 8);
    local_2208 = 1;
    uStack_2204 = 0;
    uStack_2200 = &local_11b8;
    local_21f8 = 1;
    uStack_21f7 = 0;
    local_4c = 1;
    local_d0 = lVar8;
    local_c8 = lVar2;
    alloc::fmt::format::format_inner();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *(undefined8 *)(extraout_RAX + 4) = local_128;
    *extraout_RAX = local_138;
    extraout_RAX[1] = uStack_134;
    extraout_RAX[2] = uStack_130;
    extraout_RAX[3] = uStack_12c;
    local_2218 = (undefined1)lVar2;
    uStack_2217 = (undefined7)((ulonglong)lVar2 >> 8);
    local_2220 = (undefined8 **)0x0;
    local_60 = lVar8;
    if (lVar8 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uStack_11b0 = 0x90;
    uStack_11af = 0x1429bc5;
    local_58 = (undefined8 *)lVar2;
    (**(code **)(lVar2 + 0x18))(lVar8,&local_11b8);
    if (*(longlong *)((longlong)local_58 + 8) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_4c = 0;
    core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>();
  }
  return;
}

