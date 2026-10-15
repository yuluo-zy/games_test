// 0x140683c50 default_focus
// signature: void __cdecl default_focus(void)

void __cdecl toolbar::mei::core::Node::default_focus(void)

{
  longlong in_RCX;
  
  *(byte *)(in_RCX + 0xb7) = *(byte *)(in_RCX + 0xb7) | 4;
  return;
}

