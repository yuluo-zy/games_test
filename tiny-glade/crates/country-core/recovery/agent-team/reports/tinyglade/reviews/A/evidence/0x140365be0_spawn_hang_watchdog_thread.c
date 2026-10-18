// spawn_hang_watchdog_thread @ 0x140365be0 size=358
// sig: void __cdecl spawn_hang_watchdog_thread(void)


// WARNING: Removing unreachable block (ram,0x000140365c8c)
// WARNING: Removing unreachable block (ram,0x000140365c9c)
// WARNING: Removing unreachable block (ram,0x000140365ca5)
// WARNING: Removing unreachable block (ram,0x000140365caf)

void __cdecl tiny_glade::hang_watchdog::spawn_hang_watchdog_thread(void)

{
  undefined8 *puVar1;
  code *pcVar2;
  undefined8 *extraout_RAX;
  longlong local_58;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX + 5) = 0x676f646863746177;
  *extraout_RAX = 0x74617720676e6168;
  std::thread::Builder::name();
  std::thread::Builder::spawn<void_(*)(),tuple_<>_>();
  if (((uint)local_58 & 3) != 1) {
    return;
  }
  puVar1 = *(undefined8 **)(local_58 + 7);
  pcVar2 = (code *)*puVar1;
  if (pcVar2 != (code *)0x0) {
    (*pcVar2)(*(undefined8 *)(local_58 + -1));
  }
  if (puVar1[1] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}


