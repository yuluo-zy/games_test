// 0x140683c60 with_nav
// signature: void __cdecl with_nav(void)

void __cdecl toolbar::mei::core::Node::with_nav(void)

{
  longlong in_RCX;
  char in_DL;
  
  *(byte *)(in_RCX + 0xb7) = in_DL << 5 | *(byte *)(in_RCX + 0xb7) & 0xdf;
  return;
}

