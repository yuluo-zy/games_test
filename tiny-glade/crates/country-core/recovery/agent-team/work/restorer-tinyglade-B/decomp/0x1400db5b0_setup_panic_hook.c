// setup_panic_hook @ 0x1400db5b0
// sig: void __cdecl setup_panic_hook(void)


void __cdecl tiny_glade::panic_reporter::setup_panic_hook(void)

{
  undefined8 extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 in_RCX;
  undefined8 in_RDX;
  undefined8 uVar1;
  
  uVar1 = in_RDX;
  std::panicking::take_hook();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    extraout_RAX_00[2] = in_RCX;
    extraout_RAX_00[3] = in_RDX;
    *extraout_RAX_00 = extraout_RAX;
    extraout_RAX_00[1] = uVar1;
    std::panicking::set_hook();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

