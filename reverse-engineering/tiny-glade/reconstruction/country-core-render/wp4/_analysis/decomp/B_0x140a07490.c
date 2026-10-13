// ADDRESS: 0x140a07490
// SIZE: 60
// SIGNATURE: void __cdecl set_indices(void)

void __cdecl country_core::render::mesh::Mesh::set_indices(void)

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

