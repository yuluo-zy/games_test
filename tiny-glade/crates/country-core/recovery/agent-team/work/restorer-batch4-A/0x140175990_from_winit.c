// from_winit
// sig: void __cdecl from_winit(void)
// size: 91

void __cdecl tiny_glade::setup::impl$1::from_winit(void)

{
  undefined2 uVar1;
  undefined4 uVar2;
  uint extraout_EAX;
  longlong extraout_RAX;
  ulonglong *in_RCX;
  longlong in_RDX;
  
  winit::platform_impl::platform::monitor::VideoMode::size();
  winit::dpi::impl$2::from_f64();
  winit::dpi::impl$2::from_f64();
  uVar1 = *(undefined2 *)(in_RDX + 0x1c);
  uVar2 = *(undefined4 *)(in_RDX + 0x18);
  *in_RCX = (ulonglong)extraout_EAX | extraout_RAX << 0x20;
  *(undefined2 *)((longlong)in_RCX + 0xc) = uVar1;
  *(undefined4 *)(in_RCX + 1) = uVar2;
  return;
}

