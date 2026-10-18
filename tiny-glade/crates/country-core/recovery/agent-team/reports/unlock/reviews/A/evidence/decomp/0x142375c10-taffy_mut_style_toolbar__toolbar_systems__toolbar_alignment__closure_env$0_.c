// addr 0x142375c10 sig void __cdecl taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>(void) size 206

// WARNING: Removing unreachable block (ram,0x000142375ca1)

void __cdecl
toolbar::resouces_and_components::taffy_utils::
taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_0>(void)

{
  undefined4 local_1b8;
  
  taffy::node::Taffy::style();
  if (local_1b8 == 4) {
    taffy::style::impl$16::clone();
    taffy::node::Taffy::set_style();
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

