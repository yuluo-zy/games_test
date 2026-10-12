
void __cdecl country_core::resources::history::History::finish_input_recording(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  int iVar9;
  longlong extraout_RAX;
  longlong *in_RCX;
  longlong lVar10;
  longlong *in_RDX;
  
  lVar2 = in_RCX[6];
  in_RCX[6] = -0x8000000000000000;
  if (SBORROW8(0,lVar2)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  lVar5 = in_RCX[7];
  lVar7 = in_RCX[8];
  lVar10 = in_RCX[9];
  lVar8 = in_RCX[10];
  if ((((lVar7 == 0) && (*(longlong *)(lVar10 + 0x28) == 0)) && (*(longlong *)(lVar8 + 0x40) == 0))
     && (*(longlong *)(lVar8 + 0x58) == 0)) {
    core::ptr::drop_in_place<country_core::resources::history::MultiEdit>();
  }
  else {
    lVar3 = in_RCX[2];
    if (lVar3 == *in_RCX) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar4 = in_RCX[1];
    *(longlong *)(lVar4 + 0x20 + lVar3 * 0x28) = lVar8;
    plVar1 = (longlong *)(lVar4 + 0x10 + lVar3 * 0x28);
    *plVar1 = lVar7;
    plVar1[1] = lVar10;
    plVar1 = (longlong *)(lVar4 + lVar3 * 0x28);
    *plVar1 = lVar2;
    plVar1[1] = lVar5;
    in_RCX[2] = lVar3 + 1;
    std::thread::local::LocalKey::
    with<fastrand::Rng,fastrand::u64::closure_env_0<core::ops::range::RangeInclusive<u64>_>,u64>();
    in_RCX[0xc] = extraout_RAX;
    if ((int)in_RCX[0xd] < -1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    iVar9 = (int)in_RCX[0xd] + 1;
    *(int *)(in_RCX + 0xd) = iVar9;
    if (iVar9 != (int)in_RCX[2] + -1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::assert_failed<i32,i32>();
    }
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar2 = *in_RDX;
    lVar5 = *(longlong *)(lVar2 + 0x30);
    uVar6 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar5 == *(longlong *)(lVar2 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar7 = *(longlong *)(lVar2 + 0x28);
    lVar10 = lVar5 * 0x30;
    *(undefined8 *)(lVar7 + lVar10) = 0x8000000000000000;
    *(undefined8 *)(lVar7 + 8 + lVar10) = 1;
    *(undefined8 *)(lVar7 + 0x28 + lVar10) = uVar6;
    *(longlong *)(lVar2 + 0x30) = lVar5 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  }
  return;
}

