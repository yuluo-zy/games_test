
void __cdecl taffy::node::Taffy::new_leaf(void)

{
  uint uVar1;
  ulonglong uVar2;
  longlong lVar3;
  undefined4 extraout_EAX;
  ulonglong uVar4;
  undefined4 *in_RCX;
  longlong in_RDX;
  undefined8 *_Dst;
  void *in_R8;
  int iVar5;
  undefined8 local_358;
  undefined8 local_350;
  undefined8 local_348;
  undefined4 local_200;
  undefined4 local_1c4;
  undefined4 local_188;
  undefined4 local_14c;
  undefined4 local_110;
  undefined4 local_d4;
  undefined4 local_98;
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined1 uStack_44;
  undefined7 uStack_43;
  undefined1 uStack_3c;
  undefined8 uStack_3b;
  
  _Dst = &local_358;
  memcpy(_Dst,in_R8,0x158);
  local_200 = 2;
  local_1c4 = 2;
  local_188 = 2;
  local_14c = 2;
  local_110 = 2;
  local_d4 = 2;
  local_98 = 2;
  local_5c = 0;
  uStack_54 = 0;
  local_4c = 0;
  uStack_44 = 0;
  uStack_43 = 0;
  uStack_3c = 0;
  uStack_3b = 0;
  slotmap::basic::SlotMap::
  insert_with_key<slotmap::DefaultKey,taffy::data::NodeData,slotmap::basic::impl_5::insert::closure_env_0<slotmap::DefaultKey,taffy::data::NodeData>_>
            ();
  local_358 = 0;
  local_350 = 4;
  local_348 = 0;
  slotmap::basic::SlotMap::
  insert_with_key<slotmap::DefaultKey,alloc::vec::Vec<slotmap::DefaultKey,alloc::alloc::Global>,slotmap::basic::impl_5::insert::closure_env_0<slotmap::DefaultKey,alloc::vec::Vec<slotmap::DefaultKey,alloc::alloc::Global>_>_>
            ();
  iVar5 = *(int *)(in_RDX + 0x5c) + 1;
  if (iVar5 != -1) {
    uVar2 = *(ulonglong *)(in_RDX + 0x50);
    uVar4 = (ulonglong)*(uint *)(in_RDX + 0x58);
    if (uVar4 < uVar2) {
      lVar3 = *(longlong *)(in_RDX + 0x48);
      uVar1 = *(uint *)(lVar3 + 8 + uVar4 * 0xc);
      *(undefined4 *)(in_RDX + 0x58) = *(undefined4 *)(lVar3 + uVar4 * 0xc);
      *(undefined4 *)(lVar3 + uVar4 * 0xc) = 0;
      *(uint *)(lVar3 + 8 + uVar4 * 0xc) = uVar1 | 1;
    }
    else {
      if (uVar2 == *(ulonglong *)(in_RDX + 0x40)) {
        alloc::raw_vec::RawVec::
        grow_one<slotmap::basic::Slot<enum2_<core::option::Option<slotmap::DefaultKey>_>_>,alloc::alloc::Global>
                  ();
      }
      lVar3 = *(longlong *)(in_RDX + 0x48);
      *(undefined4 *)(lVar3 + uVar2 * 0xc) = 0;
      *(undefined4 *)(lVar3 + 8 + uVar2 * 0xc) = 1;
      *(ulonglong *)(in_RDX + 0x50) = uVar2 + 1;
      *(int *)(in_RDX + 0x58) = (int)uVar2 + 1;
    }
    *(int *)(in_RDX + 0x5c) = iVar5;
    in_RCX[1] = extraout_EAX;
    in_RCX[2] = (int)_Dst;
    *in_RCX = 4;
    return;
  }
                    // WARNING: Subroutine does not return
  std::panicking::begin_panic<ref_<str_>_>();
}

