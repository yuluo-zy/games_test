
// WARNING: Removing unreachable block (ram,0x0001413ff642)

void __cdecl
country_core::resources::critters::sheep::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  char *in_RCX;
  longlong *in_RDX;
  longlong lVar1;
  longlong *plVar2;
  
  if (*in_RCX == '\0') {
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
    plVar2[2] = lVar1 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX_03 != 0) goto LAB_1413ff76c;
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x3a;
    plVar2[2] = lVar1 + 1;
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
    plVar2[2] = lVar1 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_04 != 0) {
      return;
    }
    plVar2 = (longlong *)*in_RDX;
  }
  else {
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
    plVar2[2] = lVar1 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX != 0) {
LAB_1413ff76c:
      serde_json::error::Error::io();
      return;
    }
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x3a;
    plVar2[2] = lVar1 + 1;
    plVar2 = (longlong *)*in_RDX;
    lVar1 = plVar2[2];
    if (*plVar2 == lVar1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar1 = plVar2[2];
    }
    *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
    plVar2[2] = lVar1 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_00 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_01 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,bool>
              ();
    if (extraout_RAX_02 != 0) {
      return;
    }
    plVar2 = (longlong *)*in_RDX;
  }
  lVar1 = plVar2[2];
  if (*plVar2 == lVar1) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar1 = plVar2[2];
  }
  *(undefined1 *)(plVar2[1] + lVar1) = 0x7d;
  lVar1 = lVar1 + 1;
  plVar2[2] = lVar1;
  if (*plVar2 == lVar1) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar1 = plVar2[2];
  }
  *(undefined1 *)(plVar2[1] + lVar1) = 0x7d;
  plVar2[2] = lVar1 + 1;
  return;
}

