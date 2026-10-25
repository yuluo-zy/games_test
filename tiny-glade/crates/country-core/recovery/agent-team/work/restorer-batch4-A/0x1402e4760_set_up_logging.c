// set_up_logging
// sig: void __cdecl set_up_logging(void)
// size: 2253

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl tiny_glade::logging::set_up_logging(void)

{
  undefined8 uVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined4 *puVar11;
  undefined *puVar12;
  char extraout_AL;
  uint extraout_EAX;
  undefined4 *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 in_RCX;
  char in_DL;
  undefined8 uVar13;
  undefined8 local_130;
  code *local_128;
  undefined8 uStack_120;
  undefined4 local_118;
  undefined8 local_114;
  undefined2 local_10c;
  undefined8 local_108;
  longlong lStack_100;
  undefined8 *puStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined4 *puStack_b8;
  undefined *local_b0;
  undefined *local_a8;
  longlong lStack_a0;
  undefined8 *puStack_98;
  longlong lStack_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined4 *puStack_58;
  undefined *local_50;
  undefined1 local_3c;
  undefined1 local_3b;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  time::offset_date_time::OffsetDateTime::now_utc();
  time::utc_offset::UtcOffset::local_offset_at();
  _DAT_143634b80 = extraout_EAX & 0xffffff01 ^ 1;
  colored::control::set_virtual_terminal();
  lStack_a0 = 0;
  puStack_98 = (undefined8 *)&DAT_00000008;
  lStack_90 = 0;
  uStack_88 = 0;
  local_a8 = (undefined *)0x5;
  local_80 = 8;
  uStack_78 = 0;
  uStack_70 = 0;
  uStack_68 = 8;
  uStack_60 = 0;
  puStack_58 = (undefined4 *)0x0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *(undefined1 *)(extraout_RAX + 1) = 8;
  *extraout_RAX = 0x7020301;
  local_50 = &DAT_1429b46a8;
  lStack_100 = lStack_a0;
  puStack_f8 = puStack_98;
  lStack_f0 = lStack_90;
  uStack_e8 = uStack_88;
  local_e0 = local_80;
  uStack_d8 = uStack_78;
  local_d0 = uStack_70;
  uStack_c8 = uStack_68;
  local_c0 = uStack_60;
  local_b0 = &DAT_1429b46a8;
  local_3b = 1;
  local_108 = in_RCX;
  puStack_b8 = extraout_RAX;
  puStack_58 = extraout_RAX;
  std::io::stdio::stdout();
  lVar2 = lStack_f0;
  local_a8 = (undefined *)0x6;
  lStack_a0 = 0x8000000000000000;
  puStack_98 = (undefined8 *)&DAT_1429b47b7;
  lStack_90 = 1;
  uStack_88 = extraout_RAX_00;
  if (lStack_f0 == lStack_100) {
    alloc::raw_vec::RawVec::grow_one<enum2_<fern::builders::OutputInner>,alloc::alloc::Global>();
  }
  puStack_f8[lVar2 * 0xc + 10] = puStack_58;
  (puStack_f8 + lVar2 * 0xc + 10)[1] = local_50;
  puStack_f8[lVar2 * 0xc + 8] = uStack_68;
  (puStack_f8 + lVar2 * 0xc + 8)[1] = uStack_60;
  puStack_f8[lVar2 * 0xc + 6] = uStack_78;
  (puStack_f8 + lVar2 * 0xc + 6)[1] = uStack_70;
  puStack_f8[lVar2 * 0xc + 4] = uStack_88;
  (puStack_f8 + lVar2 * 0xc + 4)[1] = local_80;
  puStack_f8[lVar2 * 0xc + 2] = puStack_98;
  (puStack_f8 + lVar2 * 0xc + 2)[1] = lStack_90;
  puStack_f8[lVar2 * 0xc] = local_a8;
  (puStack_f8 + lVar2 * 0xc)[1] = lStack_a0;
  lStack_90 = lVar2 + 1;
  puStack_58 = puStack_b8;
  local_50 = local_b0;
  uStack_68 = uStack_c8;
  uStack_60 = local_c0;
  uStack_78 = uStack_d8;
  uStack_70 = local_d0;
  uStack_88 = uStack_e8;
  local_80 = local_e0;
  puStack_98 = puStack_f8;
  local_a8 = (undefined *)local_108;
  lStack_a0 = lStack_100;
  lStack_100 = 0;
  puStack_f8 = (undefined8 *)&DAT_00000008;
  lStack_f0 = 0;
  uStack_e8 = 0;
  local_108 = 5;
  local_e0 = 8;
  uStack_d8 = 0;
  local_d0 = 0;
  uStack_c8 = 8;
  local_c0 = 0;
  puStack_b8 = (undefined4 *)0x0;
  alloc::raw_vec::RawVec::grow_one<enum2_<fern::builders::OutputInner>,alloc::alloc::Global>();
  puVar12 = local_b0;
  puVar11 = puStack_b8;
  uVar10 = local_c0;
  uVar9 = uStack_c8;
  uVar8 = local_d0;
  uVar7 = uStack_d8;
  uVar6 = local_e0;
  uVar5 = uStack_e8;
  puVar3 = puStack_f8;
  lVar2 = lStack_100;
  uVar1 = local_108;
  puStack_f8[10] = puStack_58;
  puStack_f8[0xb] = local_50;
  puStack_f8[8] = uStack_68;
  puStack_f8[9] = uStack_60;
  puStack_f8[6] = uStack_78;
  puStack_f8[7] = uStack_70;
  puStack_f8[4] = uStack_88;
  puStack_f8[5] = local_80;
  puStack_f8[2] = puStack_98;
  puStack_f8[3] = lStack_90;
  *puStack_f8 = local_a8;
  puStack_f8[1] = lStack_a0;
  lStack_f0 = 1;
  if (in_DL != '\0') {
    uStack_78 = 0;
    uStack_70 = 0;
    local_a8 = (undefined *)0x4;
    lStack_a0 = 0;
    puStack_98 = (undefined8 *)0x8;
    lStack_90 = 0;
    uStack_88 = 0;
    local_80 = 8;
    uStack_68 = 8;
    uStack_60 = 0;
    puStack_58 = (undefined4 *)0x1;
    local_50 = &DAT_1429b4678;
    local_39 = 1;
    fern::builders::Dispatch::level_for<ref_<str_>_>();
    local_39 = 1;
    fern::builders::Dispatch::level_for<ref_<str_>_>();
    local_39 = 1;
    fern::builders::Dispatch::level_for<ref_<str_>_>();
    local_39 = 1;
    fern::builders::Dispatch::level_for<ref_<str_>_>();
    local_39 = 1;
    fern::builders::Dispatch::level_for<ref_<str_>_>();
    local_130 = (undefined1 *)((ulonglong)local_130._4_4_ << 0x20);
    local_128 = (code *)0x0;
    uStack_120 = 0;
    local_118 = 7;
    local_10c = 1;
    local_114 = 0x100010000000000;
    local_3a = 1;
    paths::tmp();
    local_3a = 1;
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    uVar13 = extraout_RAX_01;
    std::fs::OpenOptions::_open();
    if (local_a8 != (undefined *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar4 = lStack_f0;
    if ((extraout_RAX_02 & 1) != 0) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_a8 = (undefined *)0x8;
    lStack_a0 = 0x8000000000000000;
    puStack_98 = (undefined8 *)&DAT_1429b47b7;
    lStack_90 = 1;
    uStack_88 = uVar13;
    if (lStack_f0 == lStack_100) {
      alloc::raw_vec::RawVec::grow_one<enum2_<fern::builders::OutputInner>,alloc::alloc::Global>();
    }
    puStack_f8[lVar4 * 0xc + 10] = puStack_58;
    (puStack_f8 + lVar4 * 0xc + 10)[1] = local_50;
    puStack_f8[lVar4 * 0xc + 8] = uStack_68;
    (puStack_f8 + lVar4 * 0xc + 8)[1] = uStack_60;
    puStack_f8[lVar4 * 0xc + 6] = uStack_78;
    (puStack_f8 + lVar4 * 0xc + 6)[1] = uStack_70;
    puStack_f8[lVar4 * 0xc + 4] = uStack_88;
    (puStack_f8 + lVar4 * 0xc + 4)[1] = local_80;
    puStack_f8[lVar4 * 0xc + 2] = puStack_98;
    (puStack_f8 + lVar4 * 0xc + 2)[1] = lStack_90;
    puStack_f8[lVar4 * 0xc] = local_a8;
    (puStack_f8 + lVar4 * 0xc)[1] = lStack_a0;
    lStack_90 = lVar4 + 1;
    puStack_58 = puStack_b8;
    local_50 = local_b0;
    uStack_68 = uStack_c8;
    uStack_60 = local_c0;
    uStack_78 = uStack_d8;
    uStack_70 = local_d0;
    uStack_88 = uStack_e8;
    local_80 = local_e0;
    puStack_98 = puStack_f8;
    local_a8 = (undefined *)local_108;
    lStack_a0 = lStack_100;
    lStack_f0 = 1;
    local_108 = uVar1;
    lStack_100 = lVar2;
    puStack_f8 = puVar3;
    uStack_e8 = uVar5;
    local_e0 = uVar6;
    uStack_d8 = uVar7;
    local_d0 = uVar8;
    uStack_c8 = uVar9;
    local_c0 = uVar10;
    puStack_b8 = puVar11;
    local_b0 = puVar12;
    if (lVar2 == 1) {
      alloc::raw_vec::RawVec::grow_one<enum2_<fern::builders::OutputInner>,alloc::alloc::Global>();
    }
    puStack_f8[0x16] = puStack_58;
    puStack_f8[0x17] = local_50;
    puStack_f8[0x14] = uStack_68;
    puStack_f8[0x15] = uStack_60;
    puStack_f8[0x12] = uStack_78;
    puStack_f8[0x13] = uStack_70;
    puStack_f8[0x10] = uStack_88;
    puStack_f8[0x11] = local_80;
    puStack_f8[0xe] = puStack_98;
    puStack_f8[0xf] = lStack_90;
    puStack_f8[0xc] = local_a8;
    puStack_f8[0xd] = lStack_a0;
    lStack_f0 = 2;
  }
  local_39 = 0;
  fern::builders::Dispatch::apply();
  if (extraout_AL != '\0') {
    local_130 = &local_3c;
    local_128 = ::log::impl$70::fmt;
    local_a8 = &DAT_1429b4d90;
    lStack_a0 = 1;
    puStack_98 = &local_130;
    lStack_90 = 1;
    uStack_88 = 0;
    local_39 = 0;
    alloc::fmt::format::format_inner();
    local_39 = 0;
    anyhow::Error::msg<alloc::string::String>();
  }
  return;
}

