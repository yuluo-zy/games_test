
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x0001429a5029) overlaps instruction at (ram,0x0001429a5028)
    */

void __cdecl
country_core::resources::history::roof_edit::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong extraout_RAX_13;
  longlong extraout_RAX_14;
  longlong extraout_RAX_15;
  longlong extraout_RAX_16;
  longlong extraout_RAX_17;
  longlong extraout_RAX_18;
  longlong extraout_RAX_19;
  longlong extraout_RAX_20;
  longlong extraout_RAX_21;
  longlong extraout_RAX_22;
  longlong extraout_RAX_23;
  longlong lVar2;
  undefined1 *in_RCX;
  longlong *in_RDX;
  
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(*in_RCX) {
  case 0:
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    break;
  case 1:
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    break;
  case 2:
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX_03 == 0) {
      plVar1 = (longlong *)*in_RDX;
      lVar2 = plVar1[2];
      if (*plVar1 == lVar2) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar2 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar2) = 0x3a;
      plVar1[2] = lVar2 + 1;
      plVar1 = (longlong *)*in_RDX;
      lVar2 = plVar1[2];
      if (*plVar1 == lVar2) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar2 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
      plVar1[2] = lVar2 + 1;
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::WallId>
                ();
      if (extraout_RAX_04 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_05 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_06 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,array_<f32,2>_>
                ();
      if (extraout_RAX_07 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_08 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_09 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_10 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_11 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,array_<f32,2>_>
                ();
      if (extraout_RAX_12 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      if (extraout_RAX_13 != 0) {
        return;
      }
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                ();
      lVar2 = extraout_RAX_14;
      goto code_r0x00014025a388;
    }
    goto code_r0x00014025a3a3;
  case 3:
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX_15 != 0) goto code_r0x00014025a3a3;
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x3a;
    plVar1[2] = lVar2 + 1;
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::WallId>
              ();
    if (extraout_RAX_16 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::RoofRidgeDirection>
              ();
    if (extraout_RAX_17 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::RoofRidgeDirection>
              ();
    lVar2 = extraout_RAX_18;
    goto code_r0x00014025a388;
  case 4:
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX != 0) goto code_r0x00014025a3a3;
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x3a;
    plVar1[2] = lVar2 + 1;
    plVar1 = (longlong *)*in_RDX;
    lVar2 = plVar1[2];
    if (*plVar1 == lVar2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = plVar1[2];
    }
    *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
    plVar1[2] = lVar2 + 1;
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::WallId>
              ();
    if (extraout_RAX_00 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::FlatRoofRimStyle>
              ();
    if (extraout_RAX_01 != 0) {
      return;
    }
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::FlatRoofRimStyle>
              ();
    lVar2 = extraout_RAX_02;
    goto code_r0x00014025a388;
  default:
    halt_baddata();
  case 10:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  serde_json::ser::
  format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
            ();
  if (extraout_RAX_19 != 0) {
code_r0x00014025a3a3:
    serde_json::error::Error::io();
    return;
  }
  plVar1 = (longlong *)*in_RDX;
  lVar2 = plVar1[2];
  if (*plVar1 == lVar2) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar2 = plVar1[2];
  }
  *(undefined1 *)(plVar1[1] + lVar2) = 0x3a;
  plVar1[2] = lVar2 + 1;
  plVar1 = (longlong *)*in_RDX;
  lVar2 = plVar1[2];
  if (*plVar1 == lVar2) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar2 = plVar1[2];
  }
  *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
  plVar1[2] = lVar2 + 1;
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::WallId>
            ();
  if (extraout_RAX_20 != 0) {
    return;
  }
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::roofs::roof_shape::RoofShapeParams>
            ();
  if (extraout_RAX_21 != 0) {
    return;
  }
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::terrain_editing::TerrainStrokeId>
            ();
  if (extraout_RAX_22 != 0) {
    return;
  }
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<country_core::resources::roofs::roof_shape::FlatRoofStyle>_>
            ();
  lVar2 = extraout_RAX_23;
code_r0x00014025a388:
  if (lVar2 == 0) {
    serde_json::ser::impl$10::
    end<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>();
  }
  return;
}

