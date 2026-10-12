
void __cdecl
country_core::systems::camera_rig::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>_>
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
  longlong *in_RDX;
  
  plVar1 = (longlong *)*in_RDX;
  in_RDX[3] = in_RDX[3] + 1;
  *(undefined1 *)(in_RDX + 4) = 0;
  lVar2 = plVar1[2];
  if (*plVar1 == lVar2) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar2 = plVar1[2];
  }
  *(undefined1 *)(plVar1[1] + lVar2) = 0x7b;
  plVar1[2] = lVar2 + 1;
  serde::ser::SerializeMap::
  serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,f32>
            ();
  if (extraout_RAX == 0) {
    serde::ser::SerializeMap::
    serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,f32>
              ();
    if (extraout_RAX_00 == 0) {
      serde::ser::SerializeMap::
      serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,array_<f32,3>_>
                ();
      if (extraout_RAX_01 == 0) {
        serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,f32>
                  ();
        if (extraout_RAX_02 == 0) {
          serde::ser::SerializeMap::
          serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,f32>
                    ();
          if (extraout_RAX_03 == 0) {
            serde::ser::SerializeMap::
            serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>_>,str_,f32>
                      ();
            if (extraout_RAX_04 == 0) {
              serde_json::ser::impl$9::
              end<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::PrettyFormatter>
                        ();
            }
          }
        }
      }
    }
  }
  return;
}

