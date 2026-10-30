
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::ui_systems::move_ops::show_decorator_remove_signifier::closure_0(void)

{
  undefined8 uVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 local_158;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  uVar1 = s__delete_scribblecrates_systems_d_142c24839._8_8_;
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *(ulonglong *)((longlong)extraout_RAX_00 + 9) =
         CONCAT17(s__delete_scribblecrates_systems_d_142c24839[7],
                  s__delete_scribblecrates_systems_d_142c24839._0_7_);
    *(undefined8 *)((longlong)extraout_RAX_00 + 0x11) = uVar1;
    uVar1 = CONCAT71(s__delete_scribblecrates_systems_d_142c24839._0_7_,UNK_142c24838);
    *extraout_RAX_00 = _DAT_142c24830;
    extraout_RAX_00[1] = uVar1;
    country_core::systems::ui::signifiers::SignifierEntityBuilder::new();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::position();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation();
    country_core::systems::ui::signifiers::SignifierEntityBuilder::build();
    *extraout_RAX = local_158;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

