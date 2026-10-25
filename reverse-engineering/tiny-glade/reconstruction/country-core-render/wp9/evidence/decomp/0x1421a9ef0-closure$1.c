// signature: void __cdecl closure$1(void)

void __cdecl system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length::closure_1(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_c8;
  
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
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::scale();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::position();
    ::utils::rotation_from_normal();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    *extraout_RAX = local_c8;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

