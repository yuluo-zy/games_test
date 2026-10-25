// rectangle_signifier  size=418
// void __cdecl rectangle_signifier(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier(void)

{
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  size_t *in_RCX;
  undefined8 *in_RDX;
  undefined1 in_R8B;
  void *_Dst;
  undefined1 local_c8 [8];
  code *local_c0;
  undefined1 *local_b8;
  code *local_b0;
  longlong local_a8;
  void *local_a0;
  size_t local_98;
  undefined *local_88;
  undefined *puStack_80;
  undefined1 *local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_c0 = ::utils::geometry::rectangle::impl$7::fmt;
  local_b8 = &local_49;
  local_b0 = core::fmt::impl$17::fmt;
  local_88 = &DAT_142c6e320;
  puStack_80 = (undefined *)0x2;
  local_78 = local_c8;
  local_70 = 2;
  local_68 = 0;
  local_49 = in_R8B;
  alloc::fmt::format::format_inner();
  local_78 = (undefined1 *)in_RDX[2];
  local_88 = (undefined *)*in_RDX;
  puStack_80 = (undefined *)in_RDX[1];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  extraout_RAX[2] = local_88;
  extraout_RAX[3] = puStack_80;
  extraout_RAX[4] = local_78;
  *(undefined1 *)(extraout_RAX + 5) = local_49;
  puStack_80 = &DAT_142c6e340;
  if ((longlong)local_98 < 0) {
LAB_14155e5ee:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (local_98 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_14155e5ee;
  }
  memcpy(_Dst,local_a0,local_98);
  *in_RCX = local_98;
  in_RCX[1] = (size_t)_Dst;
  in_RCX[2] = local_98;
  in_RCX[3] = (size_t)extraout_RAX;
  in_RCX[4] = (size_t)&DAT_142c6e340;
  if (local_a8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}]