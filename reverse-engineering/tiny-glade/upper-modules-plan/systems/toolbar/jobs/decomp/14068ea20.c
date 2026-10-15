// 0x14068ea20 place_childen
// signature: void __cdecl place_childen(void)

void __cdecl toolbar::mei::layout::LayoutStyle::place_childen(void)

{
  longlong in_RDX;
  
  if (*(char *)(in_RDX + 0x44) == '\0') {
    place_childen_impl<toolbar::mei::layout::SwizzleXY>();
  }
  else {
    place_childen_impl<toolbar::mei::layout::SwizzleYX>();
  }
  return;
}

