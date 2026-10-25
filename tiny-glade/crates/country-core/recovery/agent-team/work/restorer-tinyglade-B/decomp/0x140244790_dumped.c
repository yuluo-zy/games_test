// dumped @ 0x140244790
// sig: void __cdecl dumped(void)


void __cdecl tiny_glade::aftermath::impl$1::dumped(void)

{
  char cVar1;
  undefined1 auVar2 [16];
  ushort uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  undefined ********extraout_RAX;
  undefined ********extraout_RAX_00;
  longlong extraout_RAX_01;
  char *in_RCX;
  undefined1 (*pauVar7) [16];
  longlong lVar8;
  undefined1 (*pauVar9) [16];
  bool bVar10;
  longlong local_270;
  uint uStack_234;
  undefined8 uStack_230;
  undefined2 uStack_226;
  undefined1 uStack_21d;
  undefined4 uStack_21c;
  undefined *******local_158;
  code *pcStack_150;
  longlong local_148;
  code *local_140;
  longlong local_138;
  undefined *******local_118;
  code *pcStack_110;
  undefined *******local_108;
  char *local_100;
  undefined *******local_f8;
  code *pcStack_f0;
  undefined *******local_e8;
  undefined8 uStack_e0;
  undefined *******local_d8;
  longlong lStack_d0;
  undefined1 local_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined1 local_90;
  undefined *******local_88;
  code *pcStack_80;
  undefined *******local_78;
  undefined8 uStack_70;
  undefined *******local_68;
  longlong lStack_60;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    ::log::__private_api::impl$2::log();
  }
  local_118 = (undefined *******)0x0;
  pcStack_110 = (code *)0x1;
  local_108 = (undefined *******)0x0;
  pcStack_f0 = (code *)0x8000000000000000;
  local_e8 = (undefined *******)&local_118;
  uStack_e0 = 0;
  local_f8 = (undefined *******)0x8000000000000001;
  local_88 = (undefined *******)0x0;
  pcStack_80 = (code *)0x8;
  local_78 = (undefined *******)0x0;
  crc32fast::Hasher::new();
  LOCK();
  cVar1 = *in_RCX;
  if (cVar1 == '\0') {
    *in_RCX = '\x01';
  }
  UNLOCK();
  if (cVar1 != '\0') {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  pauVar7 = *(undefined1 (**) [16])(in_RCX + 8);
  lVar8 = *(longlong *)(in_RCX + 0x20);
  auVar2 = *pauVar7;
  uVar5 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
  pauVar9 = pauVar7 + 1;
  local_100 = in_RCX;
  while (bVar10 = lVar8 != 0, lVar8 = lVar8 + -1, bVar10) {
    if ((short)uVar5 == 0) {
      do {
        auVar2 = *pauVar9;
        uVar3 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
        pauVar7 = pauVar7 + -0x28;
        pauVar9 = pauVar9 + 1;
      } while (uVar3 == 0xffff);
      uVar6 = ~(uint)uVar3;
      uVar5 = uVar6 & -(uint)uVar3 - 2;
    }
    else {
      uVar6 = uVar5;
      uVar5 = uVar5 - 1 & uVar5;
    }
    uVar4 = 0;
    for (; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    local_158 = (undefined *******)((longlong)pauVar7 + (ulonglong)uVar4 * -0x28 + -0x28);
    local_148 = (longlong)pauVar7 + (ulonglong)uVar4 * -0x28 + -0x20;
    pcStack_150 = core::fmt::num::impl$64::fmt;
    local_140 = core::fmt::num::impl$64::fmt;
    local_f8 = (undefined *******)0x2;
    local_e8 = (undefined *******)0x0;
    uStack_e0 = 0x10;
    local_d8 = (undefined *******)0x0;
    lStack_d0 = 0x800000020;
    local_c8 = 3;
    uStack_c0 = 2;
    uStack_b0 = 0;
    local_a8 = 0x10;
    uStack_a0 = 1;
    local_98 = 0x800000020;
    local_90 = 3;
    local_88 = (undefined *******)&PTR_s_shader_nvdbg___nvdbg_1429a0718;
    pcStack_80 = (code *)0x3;
    local_78 = (undefined *******)&local_158;
    uStack_70 = 2;
    local_68 = (undefined *******)&local_f8;
    lStack_60 = 2;
    alloc::fmt::format::format_inner();
    local_138 = local_270;
    local_e8 = (undefined *******)((ulonglong)uStack_234 << 0x20);
    local_d8 = (undefined *******)CONCAT26(0x101,CONCAT24(0x7bc,CONCAT22(uStack_226,1)));
    lStack_d0 = ((ulonglong)CONCAT41(uStack_21c,uStack_21d) & 0xffffff00ff) << 0x18;
    uStack_e0 = uStack_230;
    local_f8 = (undefined *******)0x0;
    pcStack_f0 = (code *)0x1ed00000001;
    zip::write::zip_writer::ZipWriter::
    start_file<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>,ref_<alloc::string::String>_>
              ();
    if (local_88 != (undefined *******)&DAT_00000004) {
      pcStack_f0 = pcStack_80;
      local_e8 = local_78;
      local_f8 = local_88;
      anyhow::error::impl$1::from<enum2_<zip::result::ZipError>_>();
LAB_140244db4:
      if (local_138 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      cVar1 = *local_100;
      if (cVar1 == '\x01') {
        *local_100 = '\0';
      }
      UNLOCK();
      if (cVar1 != '\x01') {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      goto LAB_140244e2f;
    }
    std::io::Write::
    write_all<zip::write::zip_writer::ZipWriter<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>_>
              ();
    if (extraout_RAX != (undefined ********)0x0) {
      local_4b = 1;
      local_158 = (undefined *******)extraout_RAX;
      std::backtrace::Backtrace::capture();
      local_68 = local_d8;
      lStack_60 = lStack_d0;
      local_78 = local_e8;
      uStack_70 = uStack_e0;
      local_88 = local_f8;
      pcStack_80 = pcStack_f0;
      local_f8 = (undefined *******)0x1429a0748;
      pcStack_f0 = (code *)0x17;
      local_4b = 0;
      local_e8 = (undefined *******)extraout_RAX;
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
      goto LAB_140244db4;
    }
    if (local_138 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  cVar1 = *local_100;
  if (cVar1 == '\x01') {
    *local_100 = '\0';
  }
  UNLOCK();
  if (cVar1 != '\x01') {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  zip::write::zip_writer::ZipWriter::
  start_file<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>,ref_<str_>_>
            ();
  if (local_88 == (undefined *******)&DAT_00000004) {
    std::io::Write::
    write_all<zip::write::zip_writer::ZipWriter<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>_>
              ();
    if (extraout_RAX_00 == (undefined ********)0x0) {
      zip::write::zip_writer::ZipWriter::
      finish<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>();
      if (local_f8 == (undefined *******)&DAT_00000004) {
        local_49 = 1;
        core::ptr::
        drop_in_place<zip::write::zip_writer::ZipWriter<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>_>
                  ();
        local_49 = 1;
        paths::tmp();
        local_49 = 1;
        std::path::Path::_join();
        local_e8 = local_108;
        local_f8 = local_118;
        pcStack_f0 = pcStack_110;
        local_49 = 0;
        std::fs::write<std::path::PathBuf,alloc::vec::Vec<u8,alloc::alloc::Global>_>();
        if (extraout_RAX_01 == 0) {
          if (*(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E < 3) {
            return;
          }
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          local_f8 = (undefined *******)0x1429a0590;
          pcStack_f0 = (code *)0x15;
          local_e8 = (undefined *******)0x1429a0590;
          uStack_e0 = 0x15;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
          return;
        }
        local_49 = 0;
        anyhow::error::impl$1::from<std::io::error::Error>();
        goto LAB_140244e5c;
      }
      anyhow::error::impl$1::from<enum2_<zip::result::ZipError>_>();
    }
    else {
      local_4a = 1;
      local_158 = (undefined *******)extraout_RAX_00;
      std::backtrace::Backtrace::capture();
      local_68 = local_d8;
      lStack_60 = lStack_d0;
      local_78 = local_e8;
      uStack_70 = uStack_e0;
      local_88 = local_f8;
      pcStack_80 = pcStack_f0;
      local_f8 = (undefined *******)&DAT_1429a06ee;
      pcStack_f0 = (code *)0x11;
      local_4a = 0;
      local_e8 = (undefined *******)extraout_RAX_00;
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    }
  }
  else {
    pcStack_f0 = pcStack_80;
    local_e8 = local_78;
    local_f8 = local_88;
    anyhow::error::impl$1::from<enum2_<zip::result::ZipError>_>();
  }
LAB_140244e2f:
  local_49 = 1;
  core::ptr::
  drop_in_place<zip::write::zip_writer::ZipWriter<std::io::cursor::Cursor<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>_>_>_>
            ();
  if ((undefined ********)local_118 != (undefined ********)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_140244e5c:
  if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
    pcStack_f0 = anyhow::error::impl$5::fmt;
    local_f8 = (undefined *******)&local_88;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    ::log::__private_api::impl$2::log();
  }
  anyhow::error::impl$6::drop();
  return;
}

