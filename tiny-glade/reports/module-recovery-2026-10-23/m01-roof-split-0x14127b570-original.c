
void __cdecl
system_decorator::ui_systems::handle_splitting_roof_decorators::
display_roof_decorator_parts_and_get_hovered_part_id::closure_1(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_148;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = 0x6870735f706d6574;
    *(undefined4 *)((longlong)extraout_RAX_00 + 7) = 0x65726568;
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::scale();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::position();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    *extraout_RAX = local_148;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


