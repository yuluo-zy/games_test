// serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_> @ 0x14003e350 size=359
// sig: void __cdecl serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>(void)


void __cdecl
tiny_glade::crash_pickup::crash_payload::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar2;
  
  if (SBORROW8(0,*in_RCX)) {
    serde_json::ser::
    format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
              ();
    if (extraout_RAX == 0) {
      return;
    }
  }
  else {
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
    if (extraout_RAX_00 == 0) {
      plVar1 = (longlong *)*in_RDX;
      lVar2 = plVar1[2];
      if (*plVar1 == lVar2) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar2 = plVar1[2];
      }
      *(undefined1 *)(plVar1[1] + lVar2) = 0x3a;
      plVar1[2] = lVar2 + 1;
      serde_json::ser::
      format_escaped_str<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>
                ();
      if (extraout_RAX_01 == 0) {
        plVar1 = (longlong *)*in_RDX;
        lVar2 = plVar1[2];
        if (*plVar1 == lVar2) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          lVar2 = plVar1[2];
        }
        *(undefined1 *)(plVar1[1] + lVar2) = 0x7d;
        plVar1[2] = lVar2 + 1;
        return;
      }
    }
  }
  serde_json::error::Error::io();
  return;
}


