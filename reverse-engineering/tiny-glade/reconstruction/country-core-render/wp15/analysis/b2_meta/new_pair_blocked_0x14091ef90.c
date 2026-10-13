// addr=0x14091ef90 name=new_pair_blocked
// signature=void __cdecl new_pair_blocked(void)


void __cdecl country_core::meta_commander::MetaCommandFence::new_pair_blocked(void)

{
  longlong lVar1;
  code *pcVar2;
  longlong *extraout_RAX;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  *(undefined1 *)(extraout_RAX + 2) = 1;
  LOCK();
  lVar1 = *extraout_RAX;
  *extraout_RAX = *extraout_RAX + 1;
  UNLOCK();
  if (*extraout_RAX != 0 && SCARRY8(lVar1,1) == *extraout_RAX < 0) {
    return;
  }
                    // WARNING: Does not return
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}


