// name: try_chunk size: 148
// void __cdecl try_chunk(void)

void __cdecl country_core::systems::save::snapshot::try_chunk(void)

{
  longlong lVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar2;
  longlong *in_RCX;
  longlong *in_RDX;
  
  if (*in_RDX == 1) {
    lVar2 = in_RDX[2];
    if (in_RDX[1] != 0) {
      lVar1 = in_RDX[3];
      *in_RCX = in_RDX[1];
      in_RCX[1] = lVar2;
      in_RCX[2] = lVar1;
      return;
    }
    anyhow::error::impl$1::from<std::io::error::Error>();
    lVar2 = extraout_RAX_00;
  }
  else {
    anyhow::__private::format_err();
    lVar2 = extraout_RAX;
  }
  in_RCX[1] = lVar2;
  *in_RCX = 0;
  return;
}


