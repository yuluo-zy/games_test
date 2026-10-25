// rfind<ref$<str$>_> 0x1422790e0 size=473
// signature: void __cdecl rfind<ref$<str$>_>(void)


void __cdecl core::str::impl$0::rfind<ref_<str_>_>(void)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  uint uVar4;
  ulonglong uVar5;
  byte bVar6;
  byte local_78;
  ulonglong local_68;
  byte local_5f;
  char local_5e;
  longlong local_30;
  ulonglong local_28;
  
  pattern::StrSearcher::new();
  if ((local_78 & 1) == 0) {
    bVar6 = local_5f;
    if (local_5e == '\0') {
      for (; local_68 != 0; local_68 = local_68 + uVar5) {
        if (local_68 < local_28) {
          if (*(char *)(local_30 + local_68) < -0x40) {
LAB_1422792ab:
                    // WARNING: Subroutine does not return
            slice_error_fail();
          }
        }
        else if (local_68 != local_28) goto LAB_1422792ab;
        bVar1 = *(byte *)(local_30 + -1 + local_68);
        uVar4 = (uint)(char)bVar1;
        if ((char)bVar1 < '\0') {
          bVar2 = *(byte *)(local_30 + -2 + local_68);
          if ((char)bVar2 < -0x40) {
            bVar3 = *(byte *)(local_30 + -3 + local_68);
            if ((char)bVar3 < -0x40) {
              uVar4 = bVar3 & 0x3f | (*(byte *)(local_30 + -4 + local_68) & 7) << 6;
            }
            else {
              uVar4 = bVar3 & 0xf;
            }
            uVar4 = bVar2 & 0x3f | uVar4 << 6;
          }
          else {
            uVar4 = bVar2 & 0x1f;
          }
          uVar4 = (uint)(bVar1 & 0x3f) | uVar4 << 6;
        }
        if ((bVar6 & 1) != 0) {
          return;
        }
        uVar5 = 0xffffffffffffffff;
        if ((0x7f < uVar4) && (uVar5 = 0xfffffffffffffffe, 0x7ff < uVar4)) {
          uVar5 = (ulonglong)(uVar4 < 0x10000) | 0xfffffffffffffffc;
        }
        bVar6 = local_5f != 1;
        local_5f = local_5f ^ 1;
      }
    }
  }
  else {
    pattern::TwoWaySearcher::next_back<enum2_<core::str::pattern::MatchOnly>_>();
  }
  return;
}

