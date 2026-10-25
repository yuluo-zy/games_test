
void __cdecl
bevy_app::plugin::sealed::impl$0::
add_to_app<country_core::systems::render_loop::vermeer::VermeerPlugin>(void)

{
  undefined1 *extraout_RAX;
  undefined1 in_CL;
  undefined1 in_DL;
  longlong local_70;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined1 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = in_CL;
  extraout_RAX[1] = in_DL;
  app::App::add_boxed_plugin();
  if (SBORROW8(0,local_70)) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

