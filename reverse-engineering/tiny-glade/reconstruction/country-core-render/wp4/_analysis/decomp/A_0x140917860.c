// ADDRESS: 0x140917860
// SIZE: 2323
// SIGNATURE: void __cdecl load_compiled_texture_image(void)

void __cdecl country_core::render::load_texture::load_compiled_texture_image(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  longlong **pplVar7;
  char extraout_AL;
  uint uVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  HANDLE extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  undefined8 *in_RCX;
  longlong *plVar9;
  ulonglong in_RDX;
  code *pcVar10;
  undefined *puVar11;
  longlong local_1b0 [6];
  ulonglong local_180;
  HANDLE local_178;
  int *local_170;
  ulonglong uStack_168;
  undefined8 local_160;
  longlong local_158;
  longlong *local_148;
  longlong local_140 [2];
  longlong *local_130;
  code *pcStack_128;
  ulonglong *puStack_120;
  code *pcStack_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  longlong local_e0;
  undefined **local_d8;
  code *pcStack_d0;
  longlong **pplStack_c8;
  undefined4 local_c0;
  undefined4 uStack_bc;
  undefined4 local_b8;
  undefined2 uStack_b4;
  undefined2 uStack_b2;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined *puStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined *local_88;
  undefined8 local_80;
  longlong **local_78;
  undefined8 local_70;
  undefined8 local_68;
  longlong *local_58;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_158 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x3f) {
          if ((char)(&DAT_142aac740)[in_RDX] < -0x40) {
LAB_1409178e0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x3f) goto LAB_1409178e0;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x3f) {
          if ("untry_core::render::load_texture::load_compiled_texture_image::f"[in_RDX] < -0x40) {
LAB_140917934:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x3f) goto LAB_140917934;
      }
    }
    puVar11 = &DAT_142aacc4e;
    do {
      bVar2 = puVar11[-1];
      uVar8 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar8 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar8 = bVar4 & 0xf;
          }
          uVar8 = bVar3 & 0x3f | uVar8 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar8 = bVar3 & 0x1f;
        }
        uVar8 = (uint)(bVar2 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto LAB_140917a04;
LAB_140917a15:
        if (puVar11 == &UNK_142aacc1f) break;
        if (puVar11 + -0x142aacc1f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142aacc1f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar8 == 0x5c) goto LAB_140917a15;
LAB_140917a04:
      if (uVar8 == 0x2f) goto LAB_140917a15;
    } while (puVar11 != &DAT_142aacc20);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar9 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
    goto LAB_140917f1a;
    if (*plVar9 != 0) goto LAB_140917f26;
    *plVar9 = -1;
    local_148 = plVar9;
    puffin::ThreadProfiler::begin_scope();
    *local_148 = *local_148 + 1;
    local_158 = 1;
  }
  local_e0 = local_158;
  local_58 = (longlong *)local_158;
  std::path::Path::to_path_buf();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_with_extension();
  if (local_130 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_d8 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_130 = local_140;
    pcStack_128 = core::fmt::impl$74::fmt<str_>;
    pcStack_118 = std::path::impl$44::fmt;
    puStack_120 = (ulonglong *)local_1b0;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    uVar5 = *extraout_RAX_02;
    uStack_b0 = extraout_RAX_02[1];
    local_a8 = 5;
    puStack_a0 = &DAT_142aaccc8;
    local_98 = 0x22;
    local_88 = &DAT_142aacc90;
    local_80 = 2;
    local_70 = 2;
    local_68 = 0;
    local_d8 = (undefined **)0x0;
    pcStack_d0 = (code *)&DAT_142aaccc8;
    pplStack_c8 = (longlong **)0x22;
    local_c0 = 0;
    uStack_bc = 0;
    local_b8 = (undefined4)uVar5;
    uStack_b4 = (undefined2)((ulonglong)uVar5 >> 0x20);
    uStack_b2 = (undefined2)((ulonglong)uVar5 >> 0x30);
    uStack_90 = CONCAT44(*(undefined4 *)(extraout_RAX_02 + 2),1);
    local_78 = &local_130;
    ::log::__private_api::impl$2::log();
  }
  local_d8 = (undefined **)((ulonglong)local_d8 & 0xffffffff00000000);
  pcStack_d0 = (HANDLE)0x0;
  pplStack_c8 = (longlong **)0x0;
  local_c0 = 7;
  uStack_bc = 0;
  uStack_b4 = 0;
  local_b8 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  pcVar10 = extraout_RAX_03;
  std::fs::OpenOptions::_open();
  if ((extraout_RAX_04 & 1) != 0) {
    pcStack_128 = std::path::impl$44::fmt;
    puStack_120 = &local_180;
    pcStack_118 = std::io::error::impl$0::fmt;
    local_d8 = &PTR_DAT_142aace28;
    pcStack_d0 = (code *)0x2;
    local_b8 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    local_c0 = 2;
    uStack_bc = 0;
    local_130 = local_1b0;
    pplStack_c8 = &local_130;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  local_58 = (longlong *)pcVar10;
  memmap2::os::file_len();
  if ((extraout_RAX_05 != 0) ||
     (memmap2::os::MmapInner::map(), pplVar7 = pplStack_c8, pcVar10 = pcStack_d0,
     local_d8 == (undefined **)0x2)) {
    local_d8 = (undefined **)pcVar10;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_160 = CONCAT26(uStack_b2,CONCAT24(uStack_b4,local_b8));
  uVar6 = CONCAT44(uStack_bc,local_c0);
  local_170 = (int *)pplStack_c8;
  local_180 = (ulonglong)local_d8;
  local_178 = pcStack_d0;
  uStack_168 = uVar6;
  if (uVar6 < 8) {
                    // WARNING: Subroutine does not return
    core::slice::index::slice_end_index_len_fail();
  }
  local_130 = (longlong *)((ulonglong)pplStack_c8 & 3);
  if (local_130 != (longlong *)0x0) {
    local_d8 = (undefined **)0x0;
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<usize,usize>();
  }
  if (*(int *)pplStack_c8 != 0x48502183) {
    pcStack_128 = std::path::impl$44::fmt;
    local_d8 = &PTR_s_Image_has_an_incorrect_magic_ide_142aacd58;
    pcStack_d0 = (code *)0x2;
    local_c0 = 1;
    uStack_bc = 0;
    local_b8 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    local_130 = local_1b0;
    pplStack_c8 = &local_130;
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<u32,u32>();
  }
  if (*(int *)((longlong)pplStack_c8 + 4) != 8) {
    pcStack_128 = std::path::impl$44::fmt;
    local_d8 = &PTR_s_Image_has_an_incorrect_magic_ide_142aacdb0;
    pcStack_d0 = (code *)0x2;
    local_c0 = 1;
    uStack_bc = 0;
    local_b8 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    local_130 = local_1b0;
    pplStack_c8 = &local_130;
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<u32,u32>();
  }
  pcStack_128 = (code *)CONCAT44(pcStack_128._4_4_,
                                 *(undefined4 *)((longlong)pplStack_c8 + (uVar6 - 0xc)));
  local_130 = *(longlong **)((longlong)pplStack_c8 + (uVar6 - 0x14));
  rhapsody::raw::texture::TextureDesc::new();
  rhapsody::raw::texture::TextureDesc::sampled();
  if (0xff < *(ushort *)((longlong)pplVar7 + (uVar6 - 8))) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  rhapsody::raw::texture::TextureDesc::mip_levels();
  rhapsody::raw::texture::TextureDesc::array_elements();
  ustr::Ustr::from();
  rhapsody::raw::device::GraphicsDevice::create_texture<ref_<slice2_<u8>_>_>();
  if ((int)local_130 != 0xb) {
    local_98 = local_f0;
    uStack_90 = uStack_e8;
    local_a8 = uStack_100;
    puStack_a0 = (undefined *)local_f8;
    local_b8 = (undefined4)uStack_110;
    uStack_b4 = (undefined2)((ulonglong)uStack_110 >> 0x20);
    uStack_b2 = (undefined2)((ulonglong)uStack_110 >> 0x30);
    uStack_b0 = local_108;
    pplStack_c8 = (longlong **)puStack_120;
    local_c0 = SUB84(pcStack_118,0);
    uStack_bc = (undefined4)((ulonglong)pcStack_118 >> 0x20);
    local_d8 = (undefined **)local_130;
    pcStack_d0 = pcStack_128;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  in_RCX[6] = local_f8;
  in_RCX[4] = local_108;
  in_RCX[5] = uStack_100;
  in_RCX[2] = pcStack_118;
  in_RCX[3] = uStack_110;
  *in_RCX = pcStack_128;
  in_RCX[1] = puStack_120;
  core::ptr::drop_in_place<memmap2::os::MmapInner>();
  CloseHandle(local_58);
  if (local_1b0[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_158 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      local_58 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_58 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_140917f1a:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*local_58 != 0) {
LAB_140917f26:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_58 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_58 = *local_58 + 1;
  }
  return;
}

