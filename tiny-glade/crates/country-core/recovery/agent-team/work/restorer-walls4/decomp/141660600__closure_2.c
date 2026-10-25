// void __cdecl closure$2(void)

void __cdecl
country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle::closure_2(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  undefined8 local_60;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    builtin_strncpy((char *)((longlong)extraout_RAX_00 + 0xe),"at_singl",8);
    builtin_strncpy((char *)((longlong)extraout_RAX_00 + 0x16),"e_foward",8);
    *extraout_RAX_00 = 0x72615f6873757262;
    extraout_RAX_00[1] = 0x74616c665f776f72;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (undefined8 *)0x0) {
      *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
      *extraout_RAX_01 = 0x635f786574726576;
      extraout_RAX_01[1] = 0x6168735f726f6c6f;
      systems::ui::signifiers::SignifierEntityBuilder::build();
      *extraout_RAX = local_60;
      *in_RCX = 1;
      in_RCX[1] = extraout_RAX;
      in_RCX[2] = 1;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

