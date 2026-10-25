// addr=0x140858830 name=update
// signature=void __cdecl update(void)


void __cdecl country_core::geometry::instanced_wall::InstancedWall::update(void)

{
  undefined8 uVar1;
  longlong in_RCX;
  undefined8 *in_RDX;
  
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 8))) && (*(longlong *)(in_RCX + 8) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined8 *)(in_RCX + 0x18) = in_RDX[2];
  uVar1 = in_RDX[1];
  *(undefined8 *)(in_RCX + 8) = *in_RDX;
  *(undefined8 *)(in_RCX + 0x10) = uVar1;
  return;
}


