// signature: void __cdecl set_uvs(void)

void __cdecl country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_uvs(void)

{
  undefined8 uVar1;
  longlong in_RCX;
  undefined8 *in_RDX;
  
  if (*(longlong *)(in_RCX + 0x30) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined8 *)(in_RCX + 0x40) = in_RDX[2];
  uVar1 = in_RDX[1];
  *(undefined8 *)(in_RCX + 0x30) = *in_RDX;
  *(undefined8 *)(in_RCX + 0x38) = uVar1;
  if (*(longlong *)(in_RCX + 0x10) == *(longlong *)(in_RCX + 0x40)) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

