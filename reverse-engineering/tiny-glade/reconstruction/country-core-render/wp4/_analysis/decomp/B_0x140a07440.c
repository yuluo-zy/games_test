// ADDRESS: 0x140a07440
// SIZE: 69
// SIGNATURE: void __cdecl set_source_file(void)

void __cdecl country_core::render::mesh::Mesh::set_source_file(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong in_RCX;
  undefined8 *in_RDX;
  
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x18))) && (*(longlong *)(in_RCX + 0x18) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  uVar3 = in_RDX[3];
  *(undefined8 *)(in_RCX + 0x28) = in_RDX[2];
  *(undefined8 *)(in_RCX + 0x30) = uVar3;
  *(undefined8 *)(in_RCX + 0x18) = uVar1;
  *(undefined8 *)(in_RCX + 0x20) = uVar2;
  return;
}

