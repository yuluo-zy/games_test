// address 0x140992220
// name export_from_rg
// signature void __cdecl export_from_rg(void)

void __cdecl country_core::render::nani::gpu_world::GpuWorld::export_from_rg(void)

{
  longlong in_RCX;
  longlong lVar1;
  
  for (lVar1 = 0xa0; lVar1 != 0x14e0; lVar1 = lVar1 + 0xc0) {
    if (-0x7fffffffffffffff < *(longlong *)(in_RCX + -0x98 + lVar1)) {
      (**(code **)(*(longlong *)(in_RCX + lVar1) + 0x48))(*(undefined8 *)(in_RCX + -8 + lVar1));
    }
  }
  return;
}

