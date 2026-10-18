// void __cdecl circle_signifier(void)

void __cdecl country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier(void)

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
  undefined *local_a8;
  undefined *puStack_a0;
  undefined1 *local_98;
  undefined8 local_90;
  undefined8 local_88;
  longlong local_70;
  void *local_68;
  size_t local_60;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_c0 = ::utils::geometry::circle::impl$3::fmt;
  local_b8 = &local_49;
  local_b0 = core::fmt::impl$17::fmt;
  local_a8 = &DAT_142c82220;
  puStack_a0 = (undefined *)0x2;
  local_98 = local_c8;
  local_90 = 2;
  local_88 = 0;
  local_49 = in_R8B;
  alloc::fmt::format::format_inner();
  local_a8 = (undefined *)*in_RDX;
  puStack_a0 = (undefined *)in_RDX[1];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  extraout_RAX[2] = local_a8;
  extraout_RAX[3] = puStack_a0;
  *(undefined1 *)(extraout_RAX + 4) = local_49;
  puStack_a0 = &DAT_142c82240;
  if ((longlong)local_60 < 0) {
LAB_14165cebe:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (local_60 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_14165cebe;
  }
  memcpy(_Dst,local_68,local_60);
  *in_RCX = local_60;
  in_RCX[1] = (size_t)_Dst;
  in_RCX[2] = local_60;
  in_RCX[3] = (size_t)extraout_RAX;
  in_RCX[4] = (size_t)&DAT_142c82240;
  if (local_70 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

