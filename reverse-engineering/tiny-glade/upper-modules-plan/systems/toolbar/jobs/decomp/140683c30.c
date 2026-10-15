// 0x140683c30 ignore_input
// signature: void __cdecl ignore_input(void)

void __cdecl toolbar::mei::core::Node::ignore_input(void)

{
  longlong in_RCX;
  
  *(byte *)(in_RCX + 0xb7) = *(byte *)(in_RCX + 0xb7) | 2;
  return;
}

