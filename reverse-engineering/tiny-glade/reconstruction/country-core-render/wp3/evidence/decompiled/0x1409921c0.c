// address 0x1409921c0
// name import_to_rg
// signature void __cdecl import_to_rg(void)

void __cdecl country_core::render::nani::gpu_world::GpuWorld::import_to_rg(void)

{
  longlong in_RCX;
  longlong lVar1;
  
  for (lVar1 = 0xa0; lVar1 != 0x14e0; lVar1 = lVar1 + 0xc0) {
    if (-0x7fffffffffffffff < *(longlong *)(in_RCX + -0x98 + lVar1)) {
      (**(code **)(*(longlong *)(in_RCX + lVar1) + 0x40))(*(undefined8 *)(in_RCX + -8 + lVar1));
    }
  }
  return;
}

