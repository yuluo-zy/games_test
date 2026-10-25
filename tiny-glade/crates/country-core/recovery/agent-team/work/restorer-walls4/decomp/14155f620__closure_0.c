// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$1::signifier::closure_0
          (void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = 0x7272615f74616c66;
    *(undefined2 *)(extraout_RAX_00 + 1) = 0x776f;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (undefined8 *)0x0) {
      *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
      *extraout_RAX_01 = 0x635f786574726576;
      extraout_RAX_01[1] = 0x6168735f726f6c6f;
      ::utils::swizzlers::impl$0::x0y();
      ::utils::rotation_from_normal();
      systems::ui::signifiers::SignifierEntityBuilder::build();
      *extraout_RAX = extraout_RAX_00;
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

