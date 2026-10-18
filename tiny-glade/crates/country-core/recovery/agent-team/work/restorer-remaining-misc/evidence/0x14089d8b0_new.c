
void __cdecl country_core::systems::ui::signifiers::SignifierFactory::new(void)

{
  void *extraout_RAX;
  size_t *in_RCX;
  void *in_RDX;
  void *_Dst;
  size_t in_R8;
  size_t in_R9;
  size_t in_stack_00000028;
  
  if ((longlong)in_R8 < 0) {
LAB_14089d94d:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_14089d94d;
  }
  memcpy(_Dst,in_RDX,in_R8);
  *in_RCX = in_R8;
  in_RCX[1] = (size_t)_Dst;
  in_RCX[2] = in_R8;
  in_RCX[3] = in_R9;
  in_RCX[4] = in_stack_00000028;
  return;
}

