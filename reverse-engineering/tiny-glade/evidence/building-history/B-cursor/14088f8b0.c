
void __cdecl core::ptr::drop_in_place<country_core::resources::history::HistoryMutationToken>(void)

{
  char extraout_AL;
  char *in_RCX;
  
  if ((*in_RCX == '\x01') &&
     (((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
       0x7fffffffffffffff) == 0 ||
      (std::panicking::panic_count::is_zero_slow_path(), extraout_AL != '\0')))) {
                    /* WARNING: Subroutine does not return */
    panicking::panic_fmt();
  }
  return;
}

