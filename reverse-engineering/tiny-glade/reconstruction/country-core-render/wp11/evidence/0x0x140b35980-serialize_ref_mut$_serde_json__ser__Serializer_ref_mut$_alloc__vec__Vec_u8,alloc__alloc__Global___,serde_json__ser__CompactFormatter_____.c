// 0x140b35980 serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
// sig: void __cdecl serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>(void)

// WARNING: Type propagation algorithm not settling

void __cdecl
country_core::resources::walls::_::impl$0::
serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
          (void)

{
  longlong *plVar1;
  bool bVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  undefined2 uStack_4c;
  byte abStack_49 [41];
  
  uVar5 = *in_RCX;
  lVar8 = 0x14;
  if (9999 < uVar5) {
    uVar4 = uVar5;
    lVar6 = 0x14;
    do {
      uVar5 = uVar4 / 10000;
      iVar7 = (int)uVar4 + (int)uVar5 * -10000;
      uVar3 = (uint)(iVar7 * 0x147b) >> 0x13;
      lVar8 = lVar6 + -4;
      *(undefined2 *)((longlong)&uStack_4c + lVar6) =
           *(undefined2 *)
            (&anon_f854cc98b5e1efe54f6ab98d53b9f85b_4_llvm_5637972437638668905 +
            (ulonglong)uVar3 * 2);
      *(undefined2 *)(&stack0xffffffffffffffb6 + lVar6) =
           *(undefined2 *)
            (&anon_f854cc98b5e1efe54f6ab98d53b9f85b_4_llvm_5637972437638668905 +
            (ulonglong)(iVar7 + uVar3 * -100 & 0xffff) * 2);
      bVar2 = 99999999 < uVar4;
      uVar4 = uVar5;
      lVar6 = lVar8;
    } while (bVar2);
  }
  uVar4 = uVar5;
  if (99 < uVar5) {
    uVar3 = (uint)((uVar5 & 0xffff) >> 2) / 0x19;
    uVar4 = (ulonglong)uVar3;
    *(undefined2 *)(&stack0xffffffffffffffb6 + lVar8) =
         *(undefined2 *)
          (&anon_f854cc98b5e1efe54f6ab98d53b9f85b_4_llvm_5637972437638668905 +
          (ulonglong)((int)uVar5 + uVar3 * -100 & 0xffff) * 2);
    lVar8 = lVar8 + -2;
  }
  plVar1 = (longlong *)*in_RDX;
  if (uVar4 < 10) {
    abStack_49[lVar8] = (byte)uVar4 | 0x30;
    lVar8 = lVar8 + -1;
  }
  else {
    *(undefined2 *)(&stack0xffffffffffffffb6 + lVar8) =
         *(undefined2 *)
          (&anon_f854cc98b5e1efe54f6ab98d53b9f85b_4_llvm_5637972437638668905 + uVar4 * 2);
    lVar8 = lVar8 + -2;
  }
  uVar5 = 0x14 - lVar8;
  lVar6 = plVar1[2];
  if ((ulonglong)(*plVar1 - lVar6) < uVar5) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar6 = plVar1[2];
  }
  memcpy((void *)(plVar1[1] + lVar6),abStack_49 + lVar8 + 1,uVar5);
  plVar1[2] = lVar6 + uVar5;
  return;
}

