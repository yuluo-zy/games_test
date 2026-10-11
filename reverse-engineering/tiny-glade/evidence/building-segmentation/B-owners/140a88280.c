
void __cdecl country_core::resources::walls::material_stats::MaterialStats::init_this_update(void)

{
  longlong in_RCX;
  char in_R8B;
  undefined4 in_XMM1_Da;
  
  if (*(longlong *)(in_RCX + 0x70) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined1 *)(in_RCX + 0x68) = 0;
  *(undefined8 *)(in_RCX + 0x70) = 0;
  *(undefined8 *)(in_RCX + 0x78) = 4;
  *(undefined8 *)(in_RCX + 0x80) = 0;
  *(undefined8 *)(in_RCX + 0x88) = 0;
  *(undefined8 *)(in_RCX + 0x90) = 0;
  *(undefined8 *)(in_RCX + 0x98) = 0;
  *(undefined8 *)(in_RCX + 0xa0) = 0;
  *(undefined8 *)(in_RCX + 0xa8) = 0;
  *(undefined8 *)(in_RCX + 0xb0) = 0;
  *(undefined8 *)(in_RCX + 0xb8) = 0;
  *(undefined8 *)(in_RCX + 0xbc) = 0;
  *(undefined8 *)(in_RCX + 0xc4) = 0;
  if (in_R8B == '\0') {
    *(undefined4 *)(in_RCX + 0x90) = in_XMM1_Da;
    in_XMM1_Da = 0;
  }
  else {
    *(undefined4 *)(in_RCX + 0x90) = 0;
  }
  *(undefined4 *)(in_RCX + 0x9c) = in_XMM1_Da;
  return;
}

