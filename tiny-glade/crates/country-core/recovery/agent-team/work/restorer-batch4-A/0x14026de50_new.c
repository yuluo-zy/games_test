// new
// sig: void __cdecl new(void)
// size: 237

void __cdecl tiny_glade::event_loop::EguiEventIo::new(void)

{
  void *_Dst;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  void *in_R8;
  undefined8 in_R9;
  undefined1 local_2ae [6];
  undefined1 local_2a8 [288];
  undefined8 local_188;
  undefined8 local_180;
  undefined2 local_178;
  undefined1 local_176 [326];
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  memcpy(local_2a8,in_R8,0x120);
  local_188 = 1;
  local_180 = 1;
  local_178 = 0;
  memcpy(local_176,local_2ae,0x126);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,&local_188,0x138);
    *in_RCX = in_RDX;
    in_RCX[1] = _Dst;
    in_RCX[2] = in_R9;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

