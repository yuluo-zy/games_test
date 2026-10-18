// mock_crash_reporter
// sig: void __cdecl mock_crash_reporter(void)
// size: 681

void __cdecl tiny_glade::mock_crash_reporter(void)

{
  longlong *in_RCX;
  ulonglong *local_188;
  code *local_180;
  undefined8 local_178;
  undefined8 local_170;
  ulonglong uStack_168;
  undefined **local_160;
  undefined8 uStack_158;
  undefined4 local_150;
  ulonglong local_148;
  undefined **ppuStack_140;
  undefined **ppuStack_138;
  undefined8 **ppuStack_130;
  undefined8 **ppuStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined **local_e8;
  undefined8 uStack_e0;
  undefined8 **local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  ulonglong local_98;
  undefined **ppuStack_90;
  undefined8 local_88;
  undefined8 **ppuStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  longlong *local_38;
  longlong local_30;
  undefined4 local_28;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_30 = in_RCX[1];
  local_88 = 0;
  local_98 = 1;
  ppuStack_90 = (undefined **)0x80;
  local_38 = in_RCX;
  ron::options::Options::
  from_str_seed<core::marker::PhantomData<enum2_<crash_reporter_gui::CrashAdvice>_>,enum2_<crash_reporter_gui::CrashAdvice>_>
            ();
  if ((int)local_148 == 1) {
    local_48 = local_f0;
    local_58 = local_100;
    uStack_50 = uStack_f8;
    local_68 = local_110;
    uStack_60 = uStack_108;
    local_78 = local_120;
    uStack_70 = uStack_118;
    local_88 = ppuStack_130;
    ppuStack_80 = ppuStack_128;
    local_98 = (ulonglong)ppuStack_140;
    ppuStack_90 = ppuStack_138;
    local_180 = ron::error::impl$0::fmt;
    local_e8 = &PTR_DAT_1429d3f70;
    uStack_e0 = 2;
    local_c8 = 0;
    local_d8 = &local_188;
    uStack_d0 = 1;
    local_188 = &local_98;
    std::io::stdio::_print();
    core::ptr::drop_in_place<ron::error::SpannedError>();
  }
  else {
    local_e8 = ppuStack_138;
    uStack_e0 = ppuStack_130;
    local_d8 = ppuStack_128;
    uStack_d0 = local_120;
    local_c8 = uStack_118;
    uStack_c0 = local_110;
    local_b8 = uStack_108;
    uStack_b0 = local_100;
    local_a8 = uStack_f8;
    uStack_a0 = local_f0;
    if (ppuStack_140 != (undefined **)0x800000000000000d) {
      local_98 = (ulonglong)ppuStack_140;
      ppuStack_90 = ppuStack_138;
      local_88 = ppuStack_130;
      ppuStack_80 = ppuStack_128;
      local_78 = local_120;
      uStack_70 = uStack_118;
      local_68 = local_110;
      uStack_60 = uStack_108;
      local_58 = local_100;
      uStack_50 = uStack_f8;
      local_48 = local_f0;
      local_28 = (undefined4)
                 CONCAT71((int7)((ulonglong)ppuStack_140 >> 8),ppuStack_140 < 0x8000000000000005);
      goto LAB_1403df2fd;
    }
  }
  local_98 = 0x800000000000000d;
  local_28 = 0;
LAB_1403df2fd:
  local_21 = 1;
  std::path::Path::to_path_buf();
  local_160 = ppuStack_138;
  uStack_158 = ppuStack_130;
  local_170 = local_148;
  uStack_168 = (ulonglong)ppuStack_140;
  local_150 = 0x1000100;
  local_188 = (ulonglong *)0x0;
  local_180 = (code *)0x8;
  local_178 = 0;
  local_e8 = (undefined **)0x8000000000000004;
  uStack_f8 = local_48;
  uStack_108 = local_58;
  local_100 = uStack_50;
  uStack_118 = local_68;
  local_110 = uStack_60;
  ppuStack_128 = (undefined8 **)local_78;
  local_120 = uStack_70;
  ppuStack_138 = (undefined **)local_88;
  ppuStack_130 = ppuStack_80;
  local_148 = local_98;
  ppuStack_140 = ppuStack_90;
  local_21 = 0;
  crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env_1>();
  if (*local_38 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

