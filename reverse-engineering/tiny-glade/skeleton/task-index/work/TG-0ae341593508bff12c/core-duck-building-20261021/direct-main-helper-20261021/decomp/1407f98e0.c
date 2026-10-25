
// WARNING: Removing unreachable block (ram,0x0001407f9b92)
// WARNING: Removing unreachable block (ram,0x0001407fb6de)
// WARNING: Removing unreachable block (ram,0x0001407fb700)
// WARNING: Removing unreachable block (ram,0x0001407fb759)
// WARNING: Removing unreachable block (ram,0x0001407fc070)
// WARNING: Removing unreachable block (ram,0x0001407fb773)
// WARNING: Removing unreachable block (ram,0x0001407fb793)
// WARNING: Removing unreachable block (ram,0x0001407fb79b)
// WARNING: Removing unreachable block (ram,0x0001407fc086)
// WARNING: Removing unreachable block (ram,0x0001407fb7b0)
// WARNING: Removing unreachable block (ram,0x0001407fb740)
// WARNING: Removing unreachable block (ram,0x0001407fb80e)
// WARNING: Removing unreachable block (ram,0x0001407fc09c)
// WARNING: Removing unreachable block (ram,0x0001407fb81b)
// WARNING: Removing unreachable block (ram,0x0001407fb939)
// WARNING: Removing unreachable block (ram,0x0001407fb93d)
// WARNING: Removing unreachable block (ram,0x0001407fb94b)
// WARNING: Removing unreachable block (ram,0x0001407fb94f)
// WARNING: Removing unreachable block (ram,0x0001407fb963)
// WARNING: Removing unreachable block (ram,0x0001407fb96b)
// WARNING: Removing unreachable block (ram,0x0001407fb9e0)
// WARNING: Removing unreachable block (ram,0x0001407fb9d8)
// WARNING: Removing unreachable block (ram,0x0001407fb9f7)
// WARNING: Removing unreachable block (ram,0x0001407fbe40)
// WARNING: Removing unreachable block (ram,0x0001407fba4e)
// WARNING: Removing unreachable block (ram,0x0001407fba6c)
// WARNING: Removing unreachable block (ram,0x0001407fbe4e)
// WARNING: Removing unreachable block (ram,0x0001407fba77)
// WARNING: Removing unreachable block (ram,0x0001407fbe65)
// WARNING: Removing unreachable block (ram,0x0001407fba80)
// WARNING: Removing unreachable block (ram,0x0001407fbb50)
// WARNING: Removing unreachable block (ram,0x0001407fbb70)
// WARNING: Removing unreachable block (ram,0x0001407fbb59)
// WARNING: Removing unreachable block (ram,0x0001407fbb62)
// WARNING: Removing unreachable block (ram,0x0001407fbb76)
// WARNING: Removing unreachable block (ram,0x0001407fbd0f)
// WARNING: Removing unreachable block (ram,0x0001407fbd20)
// WARNING: Removing unreachable block (ram,0x0001407fbd3f)
// WARNING: Removing unreachable block (ram,0x0001407fbd49)
// WARNING: Removing unreachable block (ram,0x0001407fbdee)
// WARNING: Removing unreachable block (ram,0x0001407fbdfa)
// WARNING: Removing unreachable block (ram,0x0001407fbe10)
// WARNING: Removing unreachable block (ram,0x0001407fbe31)
// WARNING: Removing unreachable block (ram,0x0001407fbe6a)

void __cdecl system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings(void)

{
  longlong lVar1;
  ulonglong uVar2;
  longlong in_R9;
  byte in_stack_00000050;
  byte in_stack_00000058;
  char in_stack_00000068;
  longlong in_stack_00000088;
  
  if (*(longlong *)(in_R9 + 0x10) == in_stack_00000088) {
    lVar1 = 0;
    if ((byte)(in_stack_00000058 - 2) < 3) {
      lVar1 = (ulonglong)in_stack_00000058 - 1;
    }
    uVar2 = (ulonglong)in_stack_00000050;
    if (in_stack_00000068 != '\0') {
      uVar2 = 0;
    }
    if (lVar1 == 2) {
      uVar2 = 0;
    }
                    // WARNING: Could not recover jumptable at 0x0001407f9bc6. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)&DAT_142a8cc40 + (longlong)(int)(&DAT_142a8cc40)[uVar2]))();
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::assert_failed<usize,usize>();
}

