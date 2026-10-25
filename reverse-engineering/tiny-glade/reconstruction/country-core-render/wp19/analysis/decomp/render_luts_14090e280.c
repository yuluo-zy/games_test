// render_luts  0x14090e280
// signature: void __cdecl render_luts(void)


void __cdecl country_core::resources::gpu_lut::GpuLutTracker::render_luts(void)

{
  longlong in_RCX;
  longlong lVar1;
  longlong lVar2;
  undefined1 local_48 [8];
  longlong local_40;
  
  if (*(longlong *)(in_RCX + 0x10) != 0) {
    lVar2 = *(longlong *)(in_RCX + 8);
    lVar1 = *(longlong *)(in_RCX + 0x10) * 0x30;
    do {
      if (*(char *)(lVar2 + 0x28) == '\0') {
        *(undefined8 *)(lVar2 + 0x10) = 0;
        local_40 = lVar2;
        (**(code **)(*(longlong *)(lVar2 + 0x20) + 0x28))
                  (*(undefined8 *)(lVar2 + 0x18),local_48,&DAT_142aab6e0);
        *(undefined1 *)(lVar2 + 0x28) = 1;
      }
      lVar2 = lVar2 + 0x30;
      lVar1 = lVar1 + -0x30;
    } while (lVar1 != 0);
  }
  return;
}

