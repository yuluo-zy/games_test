
void __cdecl country_core::resources::history::History::nuke_future_states(void)

{
  ulonglong uVar1;
  undefined8 extraout_RAX;
  uint uVar2;
  undefined8 *in_RCX;
  ulonglong uVar3;
  uint uVar4;
  bool bVar5;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined1 local_68;
  undefined7 uStack_67;
  undefined8 uStack_60;
  longlong local_38;
  
  uVar2 = *(uint *)(in_RCX + 0xd);
  uVar1 = in_RCX[2];
  uVar4 = (int)uVar1 - 1;
  if (uVar2 != uVar4) {
    if ((int)uVar2 < 0) {
      core::ptr::drop_in_place<country_core::resources::history::History>();
      *in_RCX = 0;
      in_RCX[1] = 8;
      in_RCX[2] = 0;
      in_RCX[3] = 0;
      in_RCX[4] = 8;
      in_RCX[5] = 0;
      in_RCX[6] = 0x8000000000000000;
      in_RCX[7] = local_78;
      in_RCX[8] = uStack_70;
      in_RCX[9] = CONCAT71(uStack_67,local_68);
      in_RCX[10] = uStack_60;
      in_RCX[0xb] = 0;
      in_RCX[0xc] = 0;
      in_RCX[0xd] = 0xffffffffffffffff;
      *(undefined4 *)(in_RCX + 0xe) = 0;
    }
    else {
      uVar3 = (ulonglong)uVar2 + 1;
      local_38 = uVar1 - uVar3;
      if (uVar3 <= uVar1) {
        in_RCX[2] = uVar3;
        while (bVar5 = local_38 != 0, local_38 = local_38 + -1, bVar5) {
          core::ptr::drop_in_place<country_core::resources::history::MultiEdit>();
        }
        uVar2 = *(uint *)(in_RCX + 0xd);
        uVar1 = in_RCX[2];
        uVar4 = (int)uVar1 - 1;
      }
      if (uVar2 != uVar4) {
                    /* WARNING: Subroutine does not return */
        core::panicking::assert_failed<i32,i32>();
      }
      uVar3 = in_RCX[0xb];
      if (uVar1 < (ulonglong)in_RCX[0xb]) {
        uVar3 = uVar1;
      }
      in_RCX[0xb] = uVar3;
      alloc::vec::Vec::
      retain<country_core::resources::history::HistoryChainSnapshot,alloc::alloc::Global,country_core::resources::history::impl_15::nuke_future_states::closure_env_0>
                ();
    }
  }
  std::thread::local::LocalKey::
  with<fastrand::Rng,fastrand::u64::closure_env_0<core::ops::range::RangeInclusive<u64>_>,u64>();
  in_RCX[0xc] = extraout_RAX;
  return;
}

