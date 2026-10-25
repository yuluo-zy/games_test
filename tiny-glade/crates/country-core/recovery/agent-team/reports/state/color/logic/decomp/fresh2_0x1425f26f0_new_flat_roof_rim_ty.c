// name=new_flat_roof_rim_ty
// sig=void __cdecl new_flat_roof_rim_ty(void)
// size=201
// address=0x1425f26f0

void __cdecl system_color::radial_menu::IconInfo::new_flat_roof_rim_ty(void)

{
  char extraout_AL;
  undefined4 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  undefined4 *puVar1;
  undefined8 *in_RCX;
  undefined8 uVar2;
  
  country_core::resources::walls::decorator::ClutterVersion::is_initial();
  if (extraout_AL == '\0') {
    uVar2 = 0x1f;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined8 *)((longlong)extraout_RAX_00 + 0xf) = 0x5f79745f646e6168;
    *(undefined8 *)((longlong)extraout_RAX_00 + 0x17) = 0x746e6174736e6f63;
    puVar1 = extraout_RAX_00;
  }
  else {
    uVar2 = 0x1e;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    builtin_strncpy((char *)((longlong)extraout_RAX + 0xe),"ehand_ty",8);
    builtin_strncpy((char *)((longlong)extraout_RAX + 0x16),"_terrain",8);
    puVar1 = extraout_RAX;
  }
  *puVar1 = 0x6f6c6f63;
  puVar1[1] = 0x63695f72;
  puVar1[2] = 0x662f6e6f;
  puVar1[3] = 0x68656572;
  *in_RCX = uVar2;
  in_RCX[1] = puVar1;
  in_RCX[2] = uVar2;
  *(undefined4 *)((longlong)in_RCX + 0x24) = 0;
  *(undefined1 *)(in_RCX + 3) = 8;
  return;
}


