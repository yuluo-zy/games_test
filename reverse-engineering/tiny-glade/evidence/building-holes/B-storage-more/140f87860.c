
void __cdecl country_core::resources::walls::wall_holes::InProgressHoleStorage::add(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  undefined8 *in_RDX;
  undefined1 in_R8B;
  
  ::utils::
  calculate_hash<tuple_<array_<u32,2>,array_<u32,2>,country_core::resources::walls::wall_holes::HoleType>_>
            ();
  in_RCX[7] = in_RCX[7] + extraout_RAX;
  in_RCX[8] = in_RCX[8] ^ extraout_RAX;
  lVar2 = in_RCX[2];
  if (lVar2 == *in_RCX) {
    alloc::raw_vec::RawVec::
    grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
  }
  lVar3 = in_RCX[1];
  uVar4 = in_RDX[1];
  puVar1 = (undefined8 *)(lVar3 + lVar2 * 0x14);
  *puVar1 = *in_RDX;
  puVar1[1] = uVar4;
  *(undefined1 *)(lVar3 + 0x10 + lVar2 * 0x14) = in_R8B;
  in_RCX[2] = lVar2 + 1;
  return;
}

