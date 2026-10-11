
void __cdecl
country_core::resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted(void)

{
  size_t sVar1;
  void *pvVar2;
  undefined1 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  longlong in_R8;
  undefined1 *puVar3;
  
  sVar1 = *(ulonglong *)(in_R8 + 0x10) * 8;
  if ((*(ulonglong *)(in_R8 + 0x10) >> 0x3d != 0) || (0x7ffffffffffffffc < sVar1)) {
LAB_140a878e6:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  pvVar2 = *(void **)(in_R8 + 8);
  if (sVar1 == 0) {
    puVar3 = &DAT_00000004;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar3 = extraout_RAX;
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_140a878e6;
  }
  memcpy(puVar3,pvVar2,sVar1);
  sVar1 = *(ulonglong *)(in_R8 + 0x28) * 4;
  if ((*(ulonglong *)(in_R8 + 0x28) >> 0x3e == 0) && (sVar1 < 0x7ffffffffffffffd)) {
    pvVar2 = *(void **)(in_R8 + 0x20);
    if (sVar1 == 0) {
      puVar3 = &DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar3 = extraout_RAX_00;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140a878f8;
    }
    memcpy(puVar3,pvVar2,sVar1);
    next();
    return;
  }
LAB_140a878f8:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

