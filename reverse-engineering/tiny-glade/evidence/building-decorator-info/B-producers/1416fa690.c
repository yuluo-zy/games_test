
void __cdecl
country_core::resources::walls::wall_attachment::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  char cVar4;
  char cVar5;
  longlong *plVar6;
  longlong lVar7;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar8;
  
  fVar1 = *(float *)(in_RCX + 1);
  fVar2 = *(float *)(in_RCX + 2);
  fVar3 = *(float *)((longlong)in_RCX + 0xc);
  lVar8 = *in_RCX;
  cVar4 = *(char *)((longlong)in_RCX + 0x19);
  cVar5 = *(char *)((longlong)in_RCX + 0x1a);
  plVar6 = (longlong *)*in_RDX;
  lVar7 = plVar6[2];
  if (*plVar6 == lVar7) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar7 = plVar6[2];
  }
  *(undefined1 *)(plVar6[1] + lVar7) = 0x7b;
  plVar6[2] = lVar7 + 1;
  if (((((((fVar1 == 0.0) && (!NAN(fVar1))) ||
         (serde::ser::SerializeMap::
          serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,f32>
                    (), extraout_RAX_03 == 0)) &&
        (((fVar2 == 0.0 && fVar3 == 0.0) && *(float *)((longlong)in_RCX + 0x14) == 0.0 ||
         (serde::ser::SerializeMap::
          serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,glam::f32::vec3::Vec3>
                    (), extraout_RAX == 0)))) &&
       ((serde::ser::SerializeMap::
         serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::wall_attachment::WallCornerAttachment>
                   (), extraout_RAX_00 == 0 &&
        ((lVar8 == 0 ||
         (serde::ser::SerializeMap::
          serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::WallId>
                    (), extraout_RAX_01 == 0)))))) &&
      ((cVar4 == '\0' ||
       (serde::ser::SerializeMap::
        serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,bool>
                  (), extraout_RAX_02 == 0)))) &&
     ((cVar5 != '\0' ||
      (serde::ser::SerializeMap::
       serialize_entry<enum2_<serde_json::ser::Compound<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>,str_,country_core::resources::walls::wall_attachment::WallAttachmentSide>
                 (), extraout_RAX_04 == 0)))) {
    plVar6 = (longlong *)*in_RDX;
    lVar8 = plVar6[2];
    if (*plVar6 == lVar8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar8 = plVar6[2];
    }
    *(undefined1 *)(plVar6[1] + lVar8) = 0x7d;
    plVar6[2] = lVar8 + 1;
  }
  return;
}

