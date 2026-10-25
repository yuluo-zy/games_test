// name: ignore_snapshot size: 35
// void __cdecl ignore_snapshot(void)

void __cdecl country_core::resources::session::NextSession::ignore_snapshot(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined1 in_R8B;
  
  *(undefined1 *)((longlong)in_RDX + 0x98) = in_R8B;
  memcpy(in_RCX,in_RDX,0xa0);
  return;
}


