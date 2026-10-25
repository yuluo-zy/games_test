
void __cdecl toolbar::mei::config::load_font_table(void)

{
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined1 local_e8;
  undefined8 local_e0;
  undefined8 local_d0;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined1 local_b0;
  undefined **local_a8;
  undefined8 local_a0;
  undefined1 **local_98;
  undefined8 local_90;
  undefined8 *local_88;
  undefined8 local_80;
  undefined1 *local_78;
  code *local_70;
  undefined8 *local_68;
  code *local_60;
  longlong local_58;
  undefined8 local_50;
  undefined1 local_40 [16];
  undefined8 local_30;
  undefined8 local_28;
  longlong local_20;
  undefined8 local_18;
  
  local_18 = 0xfffffffffffffffe;
  std::fs::read_to_string::inner();
  local_20 = local_58;
  if (!SBORROW8(0,local_58)) {
    local_108 = 0;
    local_118 = 1;
    local_110 = 0x80;
    local_28 = local_50;
    ron::options::Options::
    from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>
              ();
    if (local_20 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
  local_30 = local_50;
  local_78 = local_40;
  local_70 = ::core::fmt::impl$74::fmt<std::ffi::os_str::OsStr>;
  local_68 = &local_30;
  local_60 = std::io::error::impl$9::fmt;
  local_118 = 2;
  local_108 = 2;
  local_f8 = 0;
  local_f0 = 0x20;
  local_e8 = 3;
  local_e0 = 2;
  local_d0 = 2;
  local_c0 = 1;
  local_b8 = 0x400000020;
  local_b0 = 3;
  local_a8 = &anon_e8cbb23549544aea119b2951538bea82_21_llvm_10678364600062022854;
  local_a0 = 2;
  local_88 = &local_118;
  local_80 = 2;
  local_98 = &local_78;
  local_90 = 2;
                    // WARNING: Subroutine does not return
  ::core::panicking::panic_fmt();
}

