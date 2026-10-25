// closure$1  size=550
// void __cdecl closure$1(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$1::signifier::closure_1
          (void)

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
LAB_14155fbad:
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
    if (extraout_RAX_00 == (void *)0x0) goto LAB_14155fbad;
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



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': 'build', 'address': '14089dd50'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}]