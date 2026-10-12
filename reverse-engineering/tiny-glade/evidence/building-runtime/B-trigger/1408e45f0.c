
void __cdecl
country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_normals(void)

{
  undefined8 uVar1;
  longlong in_RCX;
  undefined8 *in_RDX;
  
  if (*(longlong *)(in_RCX + 0x18) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined8 *)(in_RCX + 0x28) = in_RDX[2];
  uVar1 = in_RDX[1];
  *(undefined8 *)(in_RCX + 0x18) = *in_RDX;
  *(undefined8 *)(in_RCX + 0x20) = uVar1;
  if (*(longlong *)(in_RCX + 0x10) == *(longlong *)(in_RCX + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

