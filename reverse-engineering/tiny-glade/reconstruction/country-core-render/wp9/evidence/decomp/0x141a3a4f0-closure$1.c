// signature: void __cdecl closure$1(void)

void __cdecl system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir::closure_1(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_d8;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    builtin_strncpy((char *)((longlong)extraout_RAX_00 + 0xd),"switch_r",8);
    builtin_strncpy((char *)((longlong)extraout_RAX_00 + 0x15),"idge_dir",8);
    *extraout_RAX_00 = 0x615f72656e726f63;
    extraout_RAX_00[1] = 0x6977735f776f7272;
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::scale();
    ::utils::xz_rotation_bad();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::position();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    *extraout_RAX = local_d8;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

