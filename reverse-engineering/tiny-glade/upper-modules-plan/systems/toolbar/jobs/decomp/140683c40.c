// 0x140683c40 force_interactive
// signature: void __cdecl force_interactive(void)

void __cdecl toolbar::mei::core::Node::force_interactive(void)

{
  longlong in_RCX;
  
  *(byte *)(in_RCX + 0xb7) = *(byte *)(in_RCX + 0xb7) | 1;
  return;
}

