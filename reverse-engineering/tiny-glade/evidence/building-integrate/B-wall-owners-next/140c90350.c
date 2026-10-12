
void __cdecl utils::random_splits(void)

{
  ulonglong extraout_RAX;
  ulonglong *in_RCX;
  ulonglong in_RDX;
  
  if (in_RDX < 2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((in_RDX >> 0x3e == 0) && (in_RDX * 4 < 0x7ffffffffffffffd)) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX != 0) {
      random_splits_into();
      in_RCX[2] = 0;
      *in_RCX = in_RDX;
      in_RCX[1] = extraout_RAX;
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

