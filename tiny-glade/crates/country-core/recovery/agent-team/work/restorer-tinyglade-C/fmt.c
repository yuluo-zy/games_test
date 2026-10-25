
// WARNING: Removing unreachable block (ram,0x00014032c88c)

void __cdecl
time::formatting::formattable::sealed::Sealed::
format<ref_<slice2_<enum2_<time::format_description::borrowed_format_item::BorrowedFormatItem>_>_>_>
          (void)

{
  longlong lVar1;
  void *extraout_RAX;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong lVar2;
  void *_Dst;
  size_t local_80;
  void *local_78;
  size_t sStack_70;
  
  if (in_RDX[1] != 0) {
    lVar2 = *in_RDX;
    lVar1 = lVar2 + in_RDX[1] * 0x18;
    do {
      formattable::impl$8::format_into<alloc::vec::Vec<u8,alloc::alloc::Global>_>();
      if (local_80 != 3) {
        in_RCX[1] = local_80;
        in_RCX[2] = local_78;
        in_RCX[3] = sStack_70;
        *in_RCX = 1;
        return;
      }
      lVar2 = lVar2 + 0x18;
    } while (lVar2 != lVar1);
  }
  alloc::string::String::from_utf8_lossy();
  _Dst = local_78;
  if (SBORROW8(0,local_80)) {
    if ((longlong)sStack_70 < 0) {
LAB_14032c8b2:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (sStack_70 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (void *)0x0) goto LAB_14032c8b2;
    }
    memcpy(_Dst,local_78,sStack_70);
    local_80 = sStack_70;
  }
  in_RCX[1] = local_80;
  in_RCX[2] = _Dst;
  in_RCX[3] = sStack_70;
  *in_RCX = 0;
  return;
}

