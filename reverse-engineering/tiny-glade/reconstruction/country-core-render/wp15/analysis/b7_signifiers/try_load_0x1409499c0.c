// addr=0x1409499c0 name=try_load
// signature=void __cdecl try_load(void)


void __cdecl country_core::input::input_stream::InputRecordReplayState::try_load(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong local_e8;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  longlong lStack_80;
  
  std::fs::read_to_string::inner();
  if (SBORROW8(0,local_e8)) {
    std::backtrace::Backtrace::capture();
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    in_RCX[1] = extraout_RAX;
    *in_RCX = -0x8000000000000000;
  }
  else {
    serde_json::de::from_str<country_core::input::input_stream::InputStream>();
    if (SBORROW8(0,local_b8)) {
      std::backtrace::Backtrace::capture();
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
      in_RCX[1] = extraout_RAX_00;
      *in_RCX = -0x8000000000000000;
    }
    else {
      in_RCX[6] = local_88;
      in_RCX[7] = lStack_80;
      in_RCX[4] = local_98;
      in_RCX[5] = lStack_90;
      in_RCX[2] = local_a8;
      in_RCX[3] = lStack_a0;
      *in_RCX = local_b8;
      in_RCX[1] = lStack_b0;
      in_RCX[8] = (longlong)&DAT_8000000000000002;
      in_RCX[9] = -1;
    }
    if (local_e8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}


