
void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader(void)

{
  void *extraout_RAX;
  void *in_RCX;
  void *in_RDX;
  void *_Dst;
  void *in_R8;
  size_t in_R9;
  
  if ((longlong)in_R9 < 0) {
LAB_14089dbb8:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R9 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_14089dbb8;
  }
  memcpy(_Dst,in_R8,in_R9);
  if (*(longlong *)((longlong)in_RDX + 0x60) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(size_t *)((longlong)in_RDX + 0x60) = in_R9;
  *(void **)((longlong)in_RDX + 0x68) = _Dst;
  *(size_t *)((longlong)in_RDX + 0x70) = in_R9;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}

