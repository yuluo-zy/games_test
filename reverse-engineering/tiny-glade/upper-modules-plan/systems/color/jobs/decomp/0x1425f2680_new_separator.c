// name=new_separator
// sig=void __cdecl new_separator(void)
// size=106

void __cdecl system_color::radial_menu::IconInfo::new_separator(void)

{
  undefined4 *extraout_RAX;
  undefined8 *in_RCX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined4 *)0x0) {
    *(undefined1 *)(extraout_RAX + 1) = 0x65;
    *extraout_RAX = 0x74696877;
    *in_RCX = 5;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 5;
    *(undefined4 *)((longlong)in_RCX + 0x24) = 0;
    *(undefined1 *)(in_RCX + 3) = 5;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

