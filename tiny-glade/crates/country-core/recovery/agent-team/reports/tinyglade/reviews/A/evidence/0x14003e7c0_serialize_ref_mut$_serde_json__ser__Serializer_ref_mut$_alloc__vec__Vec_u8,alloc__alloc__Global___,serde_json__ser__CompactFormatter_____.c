// serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_> @ 0x14003e7c0 size=516
// sig: void __cdecl serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>(void)


void __cdecl
tiny_glade::crash_pickup::crash_payload::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
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
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,alloc::vec::Vec<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global>_>
            ();
  if ((((((extraout_RAX == 0) &&
         (serde::ser::SerializeMap::
          serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                    (), extraout_RAX_00 == 0)) &&
        (serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                   (), extraout_RAX_01 == 0)) &&
       ((serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2_<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>_>_>_>
                   (), extraout_RAX_02 == 0 &&
        (serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                   (), extraout_RAX_03 == 0)))) &&
      ((serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                  (), extraout_RAX_04 == 0 &&
       ((serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                   (), extraout_RAX_05 == 0 &&
        (serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                   (), extraout_RAX_06 == 0)))))) &&
     ((serde::ser::SerializeMap::
       serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                 (), extraout_RAX_07 == 0 &&
      (serde::ser::SerializeMap::
       serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>
                 (), extraout_RAX_08 == 0)))) {
    alloc::vec::Vec::extend_from_slice<u8,alloc::alloc::Global>();
  }
  return;
}


