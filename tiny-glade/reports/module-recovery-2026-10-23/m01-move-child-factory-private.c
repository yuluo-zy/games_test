
void __cdecl
system_decorator::ui_systems::ui_move_decorator::highlight_child_autoclutter::closure_0(void)

{
  undefined8 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_e0;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
  country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined4 *)0x0) {
    *extraout_RAX_00 = 0x6b6e696c;
    bevy_ecs::system::commands::EntityCommands::
    insert<tuple_<country_core::components::Ui2dWsExtent,country_core::components::Ui2dImage>_>();
    *extraout_RAX = local_e0;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

