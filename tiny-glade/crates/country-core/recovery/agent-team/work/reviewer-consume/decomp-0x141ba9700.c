
void __cdecl system_water::draw_water::draw_water::closure_0(void)

{
  undefined8 *extraout_RAX;
  char *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_b8;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (char *)0x0) {
    builtin_strncpy(extraout_RAX_00,"brush_circle_scribble",0x15);
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::position();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::scale();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    *extraout_RAX = local_b8;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

