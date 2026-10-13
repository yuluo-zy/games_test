// 0x140a83fb0 new_height_changed

void __cdecl country_core::resources::history::EditType::new_height_changed(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *_Dst;
  undefined8 extraout_RAX_01;
  undefined8 *in_RCX;
  undefined8 *_Src;
  undefined8 local_168;
  undefined8 local_160;
  undefined1 *local_158;
  undefined8 *local_150;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_168 = 1;
    local_160 = 1;
    local_158 = &DAT_8000000000000002;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst != (void *)0x0) {
      _Src = &local_168;
      memcpy(_Dst,_Src,0x108);
      *extraout_RAX_00 = _Dst;
      *extraout_RAX = 1;
      extraout_RAX[1] = extraout_RAX_00;
      extraout_RAX[2] = 1;
      local_150 = extraout_RAX + 3;
      local_158 = (undefined1 *)0x1;
      alloc::boxed::iter::impl$13::
      from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                ();
      in_RCX[1] = extraout_RAX_01;
      in_RCX[2] = _Src;
      *in_RCX = 7;
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

