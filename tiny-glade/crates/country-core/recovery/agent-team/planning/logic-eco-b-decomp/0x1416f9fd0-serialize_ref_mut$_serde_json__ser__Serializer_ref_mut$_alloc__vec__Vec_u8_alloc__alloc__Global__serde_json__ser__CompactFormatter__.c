
void __cdecl
country_core::resources::critters::sheep::control::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong extraout_RAX;
  longlong *in_RDX;
  
  plVar1 = (longlong *)*in_RDX;
  lVar2 = plVar1[2];
  if (*plVar1 == lVar2) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar2 = plVar1[2];
  }
  *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
  plVar1[2] = lVar2 + 1;
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,alloc::vec::Vec<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>_>
            ();
  if (extraout_RAX == 0) {
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7d;
    plVar1[2] = lVar2 + 1;
  }
  return;
}

