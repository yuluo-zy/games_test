// signature: void __cdecl set_triangles(void)

void __cdecl
country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_triangles(void)

{
  longlong lVar1;
  longlong *in_RCX;
  longlong *in_RDX;
  
  if (*in_RCX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  in_RCX[2] = in_RDX[2];
  lVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = lVar1;
  return;
}

