// 0x14067dc40 format
// signature: void __cdecl format(void)

void __cdecl toolbar::mei::widgets::tweaker::impl$6::format(void)

{
  void *_Src;
  void *extraout_RAX;
  size_t *in_RCX;
  undefined8 *in_RDX;
  size_t _Size;
  void *_Dst;
  
  if (*(char *)*in_RDX == '\0') {
    _Size = 10;
  }
  else {
    _Size = 0xb;
  }
  lang_lib::lookup();
  if ((longlong)_Size < 0) {
LAB_14067dcd7:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (_Size == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_14067dcd7;
  }
  memcpy(_Dst,_Src,_Size);
  *in_RCX = _Size;
  in_RCX[1] = (size_t)_Dst;
  in_RCX[2] = _Size;
  return;
}

