// build  0x14090e080
// signature: void __cdecl build(void)
// callees: [{"name": "grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>", "address": "1405629f0"}, {"name": "insert_resource<country_core::resources::gpu_lut::GpuLutTracker>", "address": "14154cfb0"}]


// WARNING: Removing unreachable block (ram,0x00014090e159)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::resources::gpu_lut::impl$0::build(void)

{
  undefined4 uStack_2f;
  undefined2 uStack_2b;
  undefined1 uStack_29;
  
  alloc::raw_vec::RawVec::
  grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  _DAT_00000030 = (ulonglong)CONCAT16(uStack_29,CONCAT24(uStack_2b,uStack_2f)) << 8;
  _DAT_00000028 = &anon_e1ef625e541d4204a9786b2dedd5ac6d_57_llvm_16024945903336668838;
  _DAT_00000018 = 0;
  _DAT_00000020 = 1;
  _DAT_00000008 = 0;
  _DAT_00000010 = 8;
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
    _DAT_00000058 = &anon_e1ef625e541d4204a9786b2dedd5ac6d_56_llvm_16024945903336668838;
    DAT_00000060 = 0;
    _DAT_00000061 = uStack_2f;
    _DAT_00000065 = uStack_2b;
    DAT_00000067 = uStack_29;
    _DAT_00000048 = 0;
    _DAT_00000050 = 1;
    _DAT_00000038 = 0;
    _DAT_00000040 = 8;
  }
  bevy_app::app::App::insert_resource<country_core::resources::gpu_lut::GpuLutTracker>();
  return;
}

