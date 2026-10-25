
void __cdecl
serde_json::value::de::impl$3::
variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::__Field>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *in_RCX;
  longlong *in_RDX;
  undefined7 uStack_8f;
  undefined1 local_88;
  undefined7 uStack_87;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 local_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  char local_40;
  undefined1 local_3f;
  undefined8 local_38;
  
  lVar2 = in_RDX[3];
  lVar3 = in_RDX[4];
  lVar4 = in_RDX[5];
  lVar5 = in_RDX[6];
  lVar1 = *in_RDX;
  country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::
  visit_str<serde_json::error::Error>();
  if (lVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_40 == '\0') {
    local_78 = (undefined1)lVar4;
    uStack_77 = (undefined7)((ulonglong)lVar4 >> 8);
    uStack_70 = (undefined1)lVar5;
    local_88 = (undefined1)lVar2;
    uStack_87 = (undefined7)((ulonglong)lVar2 >> 8);
    uStack_80 = (undefined1)lVar3;
    uStack_7f = (undefined7)((ulonglong)lVar3 >> 8);
    *(undefined1 *)in_RCX = local_3f;
    *(ulonglong *)((longlong)in_RCX + 1) = CONCAT17(local_88,uStack_8f);
    *(ulonglong *)((longlong)in_RCX + 9) = CONCAT17(uStack_80,uStack_87);
    *(ulonglong *)((longlong)in_RCX + 0x11) = CONCAT17(local_78,uStack_7f);
    *(ulonglong *)((longlong)in_RCX + 0x19) = CONCAT17(uStack_70,uStack_77);
    in_RCX[4] = lVar5;
  }
  else {
    *in_RCX = local_38;
    *(undefined1 *)(in_RCX + 1) = 7;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
  }
  return;
}

