
void __cdecl
serde_json::value::de::impl$3::
variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::__Field>_>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  bool bVar8;
  bool bVar9;
  undefined8 extraout_RAX;
  undefined1 *in_RCX;
  longlong *in_RDX;
  undefined1 unaff_BL;
  undefined1 uVar10;
  undefined8 unaff_RDI;
  undefined7 uStack_8f;
  undefined1 local_88;
  undefined7 uStack_87;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 local_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  
  lVar4 = in_RDX[3];
  lVar5 = in_RDX[4];
  lVar6 = in_RDX[5];
  lVar7 = in_RDX[6];
  lVar1 = *in_RDX;
  plVar2 = (longlong *)in_RDX[1];
  lVar3 = in_RDX[2];
  if (lVar3 == 8) {
    if (*plVar2 == 0x6f546b6e69726853) {
      uVar10 = 1;
      unaff_BL = 1;
joined_r0x0001415d32ff:
      bVar9 = false;
      bVar8 = false;
      goto joined_r0x0001415d332f;
    }
  }
  else if (lVar3 == 10) {
    if ((short)plVar2[1] == 0x6d6f && *plVar2 == 0x7246646e61707845) {
      uVar10 = 0;
      unaff_BL = 0;
      goto joined_r0x0001415d32ff;
    }
  }
  else if ((lVar3 == 0xb) &&
          (*(longlong *)((longlong)plVar2 + 3) == 0x746e6174736e4970 &&
           *plVar2 == 0x74736e4970617753)) {
    unaff_BL = 2;
    bVar9 = false;
    bVar8 = false;
    uVar10 = unaff_BL;
    goto joined_r0x0001415d332f;
  }
  serde::de::Error::unknown_variant<serde_json::error::Error>();
  bVar9 = true;
  bVar8 = true;
  unaff_RDI = extraout_RAX;
  uVar10 = unaff_BL;
joined_r0x0001415d332f:
  if (lVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    bVar8 = bVar9;
    unaff_BL = uVar10;
  }
  if (bVar8) {
    *(undefined8 *)(in_RCX + 8) = unaff_RDI;
    *in_RCX = 3;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
  }
  else {
    local_78 = (undefined1)lVar6;
    uStack_77 = (undefined7)((ulonglong)lVar6 >> 8);
    uStack_70 = (undefined1)lVar7;
    local_88 = (undefined1)lVar4;
    uStack_87 = (undefined7)((ulonglong)lVar4 >> 8);
    uStack_80 = (undefined1)lVar5;
    uStack_7f = (undefined7)((ulonglong)lVar5 >> 8);
    *in_RCX = unaff_BL;
    *(ulonglong *)(in_RCX + 1) = CONCAT17(local_88,uStack_8f);
    *(ulonglong *)(in_RCX + 9) = CONCAT17(uStack_80,uStack_87);
    *(ulonglong *)(in_RCX + 0x11) = CONCAT17(local_78,uStack_7f);
    *(ulonglong *)(in_RCX + 0x19) = CONCAT17(uStack_70,uStack_77);
    *(longlong *)(in_RCX + 0x20) = lVar7;
  }
  return;
}

