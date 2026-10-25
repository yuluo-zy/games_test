// addr=0x140a24000 name=slice
// signature=void __cdecl slice(void)


void __cdecl country_core::utils::child_wall::ChildWall::slice(void)

{
  undefined8 *in_RCX;
  undefined8 in_R8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined1 local_9c [4];
  undefined1 *local_98;
  code *local_90;
  undefined **local_88;
  undefined8 local_80;
  undefined1 **local_78;
  undefined8 local_70;
  undefined8 local_68;
  
  ::utils::lerp();
  if ((extraout_XMM0_Da < 0.0) || (1.0 < extraout_XMM0_Da)) {
    local_98 = local_9c;
    local_90 = core::fmt::float::impl$5::fmt;
    local_88 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
    local_80 = 1;
    local_68 = 0;
    local_78 = &local_98;
    local_70 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(extraout_XMM0_Da)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  ::utils::lerp();
  if ((0.0 <= extraout_XMM0_Da_00) && (extraout_XMM0_Da_00 <= 1.0)) {
    if (!NAN(extraout_XMM0_Da_00)) {
      *in_RCX = in_R8;
      *(float *)(in_RCX + 1) = extraout_XMM0_Da;
      *(float *)((longlong)in_RCX + 0xc) = extraout_XMM0_Da_00;
      return;
    }
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_98 = local_9c;
  local_90 = core::fmt::float::impl$5::fmt;
  local_88 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
  local_80 = 1;
  local_68 = 0;
  local_78 = &local_98;
  local_70 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


