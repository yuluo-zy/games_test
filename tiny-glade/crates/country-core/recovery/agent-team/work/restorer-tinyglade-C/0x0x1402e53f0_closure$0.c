// closure$0 @ 0x0x1402e53f0
// sig: void __cdecl closure$0(void)
// size: 423


void __cdecl tiny_glade::logging::set_up_logging::closure_0(void)

{
  longlong in_R9;
  int local_e8;
  
  time::offset_date_time::OffsetDateTime::now_utc();
  if (DAT_143634b80 != '\x01') {
    time::formatting::formattable::sealed::Sealed::
    format<ref_<slice2_<enum2_<time::format_description::borrowed_format_item::BorrowedFormatItem>_>_>_>
              ();
    if (local_e8 != 1) {
                    // WARNING: Could not recover jumptable at 0x0001402e5525. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_1429b4dc0 + *(int *)(&DAT_1429b4dc0 + (*(longlong *)(in_R9 + 0x30) + -1) * 4))
      )(&DAT_1429b4dc0 + *(int *)(&DAT_1429b4dc0 + (*(longlong *)(in_R9 + 0x30) + -1) * 4));
      return;
    }
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  time::offset_date_time::OffsetDateTime::to_offset();
  time::formatting::formattable::sealed::Sealed::
  format<ref_<slice2_<enum2_<time::format_description::borrowed_format_item::BorrowedFormatItem>_>_>_>
            ();
  if (local_e8 != 1) {
                    // WARNING: Could not recover jumptable at 0x0001402e54bc. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429b4dc0 + *(int *)(&DAT_1429b4dc0 + (*(longlong *)(in_R9 + 0x30) + -1) * 4)))
              (&DAT_1429b4dc0 + *(int *)(&DAT_1429b4dc0 + (*(longlong *)(in_R9 + 0x30) + -1) * 4));
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

