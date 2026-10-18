// closure$1 @ 0x0x1402e5740
// sig: void __cdecl closure$1(void)
// size: 546


void __cdecl tiny_glade::logging::set_up_logging::closure_1(void)

{
  longlong in_R9;
  undefined8 local_f8;
  undefined1 local_f0 [8];
  longlong *local_e8;
  code *local_e0;
  undefined8 *puStack_d8;
  code *local_d0;
  undefined8 *local_c8;
  code *local_c0;
  undefined4 *local_b8;
  code *local_b0;
  undefined1 *local_a8;
  code *local_a0;
  longlong local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined *local_68;
  undefined8 uStack_60;
  undefined8 **local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined4 local_2c;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  time::offset_date_time::OffsetDateTime::now_utc();
  if (DAT_143634b80 == '\x01') {
    time::offset_date_time::OffsetDateTime::to_offset();
    time::formatting::formattable::sealed::Sealed::
    format<ref_<slice2_<enum2_<time::format_description::borrowed_format_item::BorrowedFormatItem>_>_>_>
              ();
    if ((int)local_e8 == 1) {
      local_58 = (undefined8 **)local_d0;
      local_68 = local_e0;
      uStack_60 = puStack_d8;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  else {
    time::formatting::formattable::sealed::Sealed::
    format<ref_<slice2_<enum2_<time::format_description::borrowed_format_item::BorrowedFormatItem>_>_>_>
              ();
    if ((int)local_e8 == 1) {
      local_58 = (undefined8 **)local_d0;
      local_68 = local_e0;
      uStack_60 = puStack_d8;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  local_88 = local_d0;
  local_98 = (longlong)local_e0;
  uStack_90 = puStack_d8;
  local_78 = *(undefined8 *)(in_R9 + 0x38);
  uStack_70 = *(undefined8 *)(in_R9 + 0x40);
  local_f8 = *(undefined8 *)(in_R9 + 0x30);
  local_2c = *(undefined4 *)__imp__ZN12country_core7systems8TICK_IDX17h88d558048ca3865aE;
  local_e8 = &local_98;
  local_e0 = alloc::string::impl$23::fmt;
  puStack_d8 = &local_f8;
  local_d0 = ::log::impl$3::fmt;
  local_c0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_b8 = &local_2c;
  local_b0 = core::fmt::num::imp::impl$6::fmt;
  local_a8 = local_f0;
  local_a0 = core::fmt::impl$76::fmt<core::fmt::Arguments>;
  local_68 = &DAT_1429b4d40;
  uStack_60 = 5;
  local_48 = 0;
  local_58 = &local_e8;
  local_50 = 5;
  local_c8 = &local_78;
  fern::log_impl::FormatCallback::finish();
  if (local_98 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

