
void __cdecl
serde::ser::SerializeMap::
serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::RoofRidgeDirection>
          (void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 *in_RCX;
  
  plVar1 = (longlong *)*in_RCX;
  if (*(char *)(in_RCX + 1) != '\x01') {
    plVar2 = (longlong *)*plVar1;
    lVar3 = plVar2[2];
    if (*plVar2 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar3) = 0x2c;
    plVar2[2] = lVar3 + 1;
  }
  *(undefined1 *)(in_RCX + 1) = 2;
  serde_json::ser::
  format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
            ();
  plVar1 = (longlong *)*plVar1;
  lVar3 = plVar1[2];
  if (*plVar1 == lVar3) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar3 = plVar1[2];
  }
  *(undefined1 *)(plVar1[1] + lVar3) = 0x3a;
  plVar1[2] = lVar3 + 1;
  serde_json::ser::
  format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
            ();
  return;
}

