
void __cdecl country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *extraout_RAX;
  void *_Dst;
  longlong *in_RCX;
  void *in_RDX;
  undefined8 local_140;
  undefined8 *puStack_138;
  undefined8 local_130 [33];
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  memcpy(local_130,in_RDX,0xf8);
  local_140 = 1;
  puStack_138 = (undefined8 *)0x1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    memcpy(_Dst,&local_140,0x108);
    *extraout_RAX = _Dst;
    local_140 = 1;
    local_130[0] = 1;
    lVar2 = in_RCX[2];
    puStack_138 = extraout_RAX;
    if (lVar2 == *in_RCX) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                ();
    }
    lVar3 = in_RCX[1];
    *(undefined8 *)(lVar3 + 0x10 + lVar2 * 0x18) = local_130[0];
    puVar1 = (undefined8 *)(lVar3 + lVar2 * 0x18);
    *puVar1 = local_140;
    puVar1[1] = puStack_138;
    in_RCX[2] = lVar2 + 1;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

