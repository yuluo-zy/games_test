// ADDRESS: 0x14093bb10
// SIZE: 461
// SIGNATURE: void __cdecl new(void)

void __cdecl country_core::asset_libraries::texture_library::AssetTextureLibrary::new(void)

{
  char *extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar1;
  undefined8 *in_RCX;
  undefined8 uVar2;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (char *)0x0) {
    builtin_strncpy(extraout_RAX,"bindless_textures",0x11);
    rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set();
    uVar2 = 0x11;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_00 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_00[0] = 1;
      extraout_RAX_00[1] = 0;
      extraout_RAX_00[2] = 0;
      extraout_RAX_00[3] = 0;
      extraout_RAX_00[4] = 0;
      extraout_RAX_00[5] = 0;
      extraout_RAX_00[6] = 0;
      extraout_RAX_00[7] = 0;
      *(longlong *)(extraout_RAX_00 + 8) = extraout_RAX_01;
      *(undefined8 *)(extraout_RAX_00 + 0x10) = uVar2;
      lVar1 = extraout_RAX_01;
    }
    else {
      lVar1 = *(longlong *)(extraout_RAX_00 + 8);
      uVar2 = *(undefined8 *)(extraout_RAX_00 + 0x10);
    }
    *(longlong *)(extraout_RAX_00 + 8) = lVar1 + 1;
    in_RCX[5] = local_98;
    in_RCX[3] = local_a8;
    in_RCX[4] = uStack_a0;
    in_RCX[0x14] = 0;
    in_RCX[0x15] = 0;
    *in_RCX = 0;
    in_RCX[1] = 8;
    in_RCX[2] = 0;
    in_RCX[6] = &anon_faeddf8c9d121b2250423bc45b590cfa_52_llvm_951737264325729122;
    in_RCX[7] = 0;
    in_RCX[8] = 0;
    in_RCX[9] = 0;
    in_RCX[10] = &anon_faeddf8c9d121b2250423bc45b590cfa_52_llvm_951737264325729122;
    in_RCX[0xb] = 0;
    in_RCX[0xc] = 0;
    in_RCX[0xd] = 0;
    in_RCX[0xe] = lVar1;
    in_RCX[0xf] = uVar2;
    in_RCX[0x10] = 0;
    *(undefined1 *)(in_RCX + 0x11) = 0;
    in_RCX[0x12] = 0;
    in_RCX[0x13] = 8;
    in_RCX[0x16] = 4;
    in_RCX[0x17] = 0;
    *(undefined4 *)(in_RCX + 0x18) = 0;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

