// name: insert_starting_edit size: 240
// void __cdecl insert_starting_edit(void)

void __cdecl country_core::resources::history::History::insert_starting_edit(void)

{
  undefined8 uVar1;
  longlong lVar2;
  undefined8 *_Src;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong extraout_RAX;
  longlong *in_RCX;
  undefined8 *in_RDX;
  
  if (-2 < (int)in_RCX[0xd]) {
    *(int *)(in_RCX + 0xd) = (int)in_RCX[0xd] + 1;
    uVar1 = in_RDX[4];
    uVar3 = *in_RDX;
    uVar4 = in_RDX[1];
    uVar5 = in_RDX[2];
    uVar6 = in_RDX[3];
    lVar2 = in_RCX[2];
    if (lVar2 == *in_RCX) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    _Src = (undefined8 *)in_RCX[1];
    if (lVar2 != 0) {
      memmove(_Src + 5,_Src,lVar2 * 0x28);
    }
    _Src[4] = uVar1;
    _Src[2] = uVar5;
    _Src[3] = uVar6;
    *_Src = uVar3;
    _Src[1] = uVar4;
    in_RCX[2] = lVar2 + 1;
    std::thread::local::LocalKey::
    with<fastrand::Rng,fastrand::u64::closure_env_0<core::ops::range::RangeInclusive<u64>_>,u64>();
    in_RCX[0xc] = extraout_RAX;
    in_RCX[0xb] = 0;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}


