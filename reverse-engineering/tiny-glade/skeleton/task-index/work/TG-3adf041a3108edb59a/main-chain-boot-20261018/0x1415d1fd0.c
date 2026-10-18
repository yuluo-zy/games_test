
void __cdecl
serde_json::value::de::impl$3::
variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Field>_>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
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
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
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
  plVar2 = (longlong *)in_RDX[1];
  if (in_RDX[2] == 0xd) {
    if (*(longlong *)((longlong)plVar2 + 5) == 0x6e69617272655477 && *plVar2 == 0x6554776f6c6c6f46)
    {
      uVar9 = 0;
      unaff_BL = 0;
joined_r0x0001415d208d:
      bVar8 = false;
      bVar7 = false;
      goto joined_r0x0001415d2136;
    }
  }
  else if ((in_RDX[2] == 0x11) &&
          (auVar10[0] = -((char)*plVar2 == 'C'),
          auVar10[1] = -(*(char *)((longlong)plVar2 + 1) == 'o'),
          auVar10[2] = -(*(char *)((longlong)plVar2 + 2) == 'n'),
          auVar10[3] = -(*(char *)((longlong)plVar2 + 3) == 's'),
          auVar10[4] = -(*(char *)((longlong)plVar2 + 4) == 't'),
          auVar10[5] = -(*(char *)((longlong)plVar2 + 5) == 'a'),
          auVar10[6] = -(*(char *)((longlong)plVar2 + 6) == 'n'),
          auVar10[7] = -(*(char *)((longlong)plVar2 + 7) == 't'),
          auVar10[8] = -((char)plVar2[1] == 'M'),
          auVar10[9] = -(*(char *)((longlong)plVar2 + 9) == 'a'),
          auVar10[10] = -(*(char *)((longlong)plVar2 + 10) == 'x'),
          auVar10[0xb] = -(*(char *)((longlong)plVar2 + 0xb) == 'H'),
          auVar10[0xc] = -(*(char *)((longlong)plVar2 + 0xc) == 'e'),
          auVar10[0xd] = -(*(char *)((longlong)plVar2 + 0xd) == 'i'),
          auVar10[0xe] = -(*(char *)((longlong)plVar2 + 0xe) == 'g'),
          auVar10[0xf] = -(*(char *)((longlong)plVar2 + 0xf) == 'h'),
          auVar11[0] = -((char)plVar2[2] == 't'), auVar11[1] = 0xff, auVar11[2] = 0xff,
          auVar11[3] = 0xff, auVar11[4] = 0xff, auVar11[5] = 0xff, auVar11[6] = 0xff,
          auVar11[7] = 0xff, auVar11[8] = 0xff, auVar11[9] = 0xff, auVar11[10] = 0xff,
          auVar11[0xb] = 0xff, auVar11[0xc] = 0xff, auVar11[0xd] = 0xff, auVar11[0xe] = 0xff,
          auVar11[0xf] = 0xff, auVar11 = auVar11 & auVar10,
          (ushort)((ushort)(SUB161(auVar11 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar11 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar11 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar11 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar11 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar11 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar11 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar11 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar11 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar11 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar11 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar11 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar11 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar11 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar11 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar11[0xf] >> 7) << 0xf) == 0xffff)) {
    uVar9 = 1;
    unaff_BL = 1;
    goto joined_r0x0001415d208d;
  }
  serde::de::Error::unknown_variant<serde_json::error::Error>();
  bVar8 = true;
  bVar7 = true;
  unaff_RDI = extraout_RAX;
  uVar9 = unaff_BL;
joined_r0x0001415d2136:
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

