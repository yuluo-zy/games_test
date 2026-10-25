// 0x140b46a20 ignore_snapshot
// signature: void __cdecl ignore_snapshot(void)
// callees: [{"name": "memcpy", "address": "142923c00"}]


void __cdecl country_core::resources::session::NextSession::ignore_snapshot(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined1 in_R8B;
  
  *(undefined1 *)((longlong)in_RDX + 0x98) = in_R8B;
  memcpy(in_RCX,in_RDX,0xa0);
  return;
}

