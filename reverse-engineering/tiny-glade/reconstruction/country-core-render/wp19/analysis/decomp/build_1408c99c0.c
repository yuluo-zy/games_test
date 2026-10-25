// build  0x1408c99c0
// signature: void __cdecl build(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "handle_alloc_error", "address": "1428d9000"}, {"name": "insert_resource<country_core::systems::render_loop::menu_background::MenuBackgroundResources>", "address": "14153d180"}]


void __cdecl country_core::systems::render_loop::menu_background::impl$0::build(void)

{
  undefined8 *extraout_RAX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    *(undefined1 *)(extraout_RAX + 2) = 0;
    *(undefined1 *)((longlong)extraout_RAX + 0x4e) = 7;
    bevy_app::app::App::
    insert_resource<country_core::systems::render_loop::menu_background::MenuBackgroundResources>();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

