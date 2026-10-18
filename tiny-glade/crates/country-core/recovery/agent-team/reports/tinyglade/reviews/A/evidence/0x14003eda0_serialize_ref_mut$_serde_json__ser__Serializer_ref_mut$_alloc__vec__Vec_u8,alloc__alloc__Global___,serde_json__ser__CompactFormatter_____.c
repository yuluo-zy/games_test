// serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_> @ 0x14003eda0 size=67
// sig: void __cdecl serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>(void)


void __cdecl
tiny_glade::crash_pickup::crash_payload::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *in_RCX;
  longlong lVar1;
  
  lVar1 = 0;
  if (*in_RCX < -0x7ffffffffffffffc) {
    lVar1 = *in_RCX + -0x7fffffffffffffff;
  }
                    // WARNING: Could not emulate address calculation at 0x00014003edd0
                    // WARNING: Treating indirect jump as call
  (*(code *)((longlong)&DAT_14292c708 + (longlong)(int)(&DAT_14292c708)[lVar1]))
            ((code *)((longlong)&DAT_14292c708 + (longlong)(int)(&DAT_14292c708)[lVar1]));
  return;
}


