// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::
closure_0(void)

{
  size_t _Size;
  void *_Src;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  void *_Dst;
  undefined8 local_68;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  _Size = in_RDX[1];
  if ((longlong)_Size < 0) {
LAB_14083e145:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  _Src = (void *)*in_RDX;
  if (_Size == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_14083e145;
  }
  memcpy(_Dst,_Src,_Size);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
  *extraout_RAX_01 = 0x635f786574726576;
  extraout_RAX_01[1] = 0x6168735f726f6c6f;
  systems::ui::signifiers::SignifierEntityBuilder::build();
  *extraout_RAX = local_68;
  *in_RCX = 1;
  in_RCX[1] = extraout_RAX;
  in_RCX[2] = 1;
  return;
}

