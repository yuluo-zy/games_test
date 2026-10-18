
void __cdecl
serde_json::value::de::impl$3::
variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Field>_>
          (void)

{
  longlong lVar1;
  int *piVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  bool bVar7;
  bool bVar8;
  undefined8 extraout_RAX;
  undefined1 *in_RCX;
  longlong *in_RDX;
  undefined1 unaff_BL;
  undefined1 uVar9;
  undefined8 unaff_RDI;
  undefined7 uStack_8f;
  undefined1 local_88;
  undefined7 uStack_87;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 local_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  
  lVar3 = in_RDX[3];
  lVar4 = in_RDX[4];
  lVar5 = in_RDX[5];
  lVar6 = in_RDX[6];
  lVar1 = *in_RDX;
  piVar2 = (int *)in_RDX[1];
  if (in_RDX[2] == 4) {
    if (*piVar2 == 0x6c6c6157) {
      uVar9 = 0;
      unaff_BL = 0;
joined_r0x0001415d2d7a:
      bVar8 = false;
      bVar7 = false;
      goto joined_r0x0001415d2e1b;
    }
  }
  else if ((in_RDX[2] == 5) && ((char)piVar2[1] == 'e' && *piVar2 == 0x636e6546)) {
    uVar9 = 1;
    unaff_BL = 1;
    goto joined_r0x0001415d2d7a;
  }
  serde::de::Error::unknown_variant<serde_json::error::Error>();
  bVar8 = true;
  bVar7 = true;
  unaff_RDI = extraout_RAX;
  uVar9 = unaff_BL;
joined_r0x0001415d2e1b:
  if (lVar1 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    bVar7 = bVar8;
    unaff_BL = uVar9;
  }
  if (bVar7) {
    *(undefined8 *)(in_RCX + 8) = unaff_RDI;
    *in_RCX = 2;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
  }
  else {
    local_78 = (undefined1)lVar5;
    uStack_77 = (undefined7)((ulonglong)lVar5 >> 8);
    uStack_70 = (undefined1)lVar6;
    local_88 = (undefined1)lVar3;
    uStack_87 = (undefined7)((ulonglong)lVar3 >> 8);
    uStack_80 = (undefined1)lVar4;
    uStack_7f = (undefined7)((ulonglong)lVar4 >> 8);
    *in_RCX = unaff_BL;
    *(ulonglong *)(in_RCX + 1) = CONCAT17(local_88,uStack_8f);
    *(ulonglong *)(in_RCX + 9) = CONCAT17(uStack_80,uStack_87);
    *(ulonglong *)(in_RCX + 0x11) = CONCAT17(local_78,uStack_7f);
    *(ulonglong *)(in_RCX + 0x19) = CONCAT17(uStack_70,uStack_77);
    *(longlong *)(in_RCX + 0x20) = lVar6;
  }
  return;
}

