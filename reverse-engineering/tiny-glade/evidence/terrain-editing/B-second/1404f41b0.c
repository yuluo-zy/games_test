
void __cdecl
country_core::resources::history::terrain_edit::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  ulonglong uVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong lVar3;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  ulonglong *in_RCX;
  longlong *in_RDX;
  
  uVar2 = 2;
  if ((*in_RCX ^ 0x8000000000000000) < 2) {
    uVar2 = *in_RCX ^ 0x8000000000000000;
  }
  if (uVar2 == 0) {
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
    plVar1[2] = lVar3 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX_04 != 0) goto LAB_1404f4554;
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x3a;
    plVar1[2] = lVar3 + 1;
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
    plVar1[2] = lVar3 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::terrain_editing::TerrainStrokeId>
              ();
    if (extraout_RAX_05 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>_>
              ();
    lVar3 = extraout_RAX_06;
  }
  else {
    if (uVar2 != 1) {
      plVar1 = (longlong *)*in_RDX;
      lVar3 = plVar1[2];
      if (*plVar1 == lVar3) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar3 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
      plVar1[2] = lVar3 + 1;
      serde_json::ser::
      format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
                ();
      if (extraout_RAX_08 != 0) {
LAB_1404f4554:
        serde_json::error::Error::io();
        return;
      }
      plVar1 = (longlong *)*in_RDX;
      lVar3 = plVar1[2];
      if (*plVar1 == lVar3) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar3 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar3) = 0x3a;
      plVar1[2] = lVar3 + 1;
      plVar1 = (longlong *)*in_RDX;
      lVar3 = plVar1[2];
      if (*plVar1 == lVar3) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar3 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
      plVar1[2] = lVar3 + 1;
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::terrain_editing::TerrainStrokeId>
                ();
      if (extraout_RAX_09 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>_>
                ();
      if (extraout_RAX_10 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,glam::f32::vec2::Vec2>
                ();
      lVar3 = extraout_RAX_11;
      goto joined_r0x0001404f440e;
    }
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
    plVar1[2] = lVar3 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX != 0) goto LAB_1404f4554;
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x3a;
    plVar1[2] = lVar3 + 1;
    plVar1 = (longlong *)*in_RDX;
    lVar3 = plVar1[2];
    if (*plVar1 == lVar3) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar3) = 0x7b;
    plVar1[2] = lVar3 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::terrain_editing::TerrainStrokeId>
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
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_02 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
              ();
    lVar3 = extraout_RAX_03;
  }
  if (lVar3 != 0) {
    return;
  }
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
            ();
  lVar3 = extraout_RAX_07;
joined_r0x0001404f440e:
  if (lVar3 == 0) {
    alloc::vec::Vec::extend_from_slice<u8,alloc::alloc::Global>();
    alloc::vec::Vec::extend_from_slice<u8,alloc::alloc::Global>();
  }
  return;
}

