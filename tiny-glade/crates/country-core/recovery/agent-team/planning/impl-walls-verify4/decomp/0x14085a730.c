
void __cdecl country_core::resources::walls::inner_walls::InnerWalls::remove_all(void)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  ushort uVar3;
  uint uVar4;
  void *_Dst;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined8 *in_RCX;
  longlong in_RDX;
  uint uVar7;
  longlong lVar8;
  uint uVar9;
  undefined1 (*pauVar10) [16];
  undefined1 (*pauVar11) [16];
  longlong lStack_120;
  
  pauVar10 = (undefined1 (*) [16])*in_RCX;
  auVar1 = *pauVar10;
  pauVar11 = pauVar10 + 1;
  lVar8 = in_RCX[3];
  _Dst = (void *)*in_RCX;
  uVar5 = in_RCX[1];
  *in_RCX = &anon_6652254e5a159f1dca07c38add8ba93c_0_llvm_3554112267207565025;
  in_RCX[1] = 0;
  in_RCX[2] = 0;
  in_RCX[3] = 0;
  uVar7 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
  while (lVar8 != 0) {
    if ((short)uVar7 == 0) {
      do {
        auVar1 = *pauVar11;
        uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
        pauVar10 = pauVar10 + -0x78;
        pauVar11 = pauVar11 + 1;
      } while (uVar3 == 0xffff);
      uVar7 = ~(uint)uVar3;
      uVar9 = uVar7 & -(uint)uVar3 - 2;
    }
    else {
      uVar9 = uVar7 - 1 & uVar7;
    }
    lVar8 = lVar8 + -1;
    uVar4 = 0;
    for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    lVar2 = *(longlong *)((longlong)pauVar10 + (ulonglong)uVar4 * -0x78 + -0x58);
    lStack_120 = SUB168(*(undefined1 (*) [16])
                         ((longlong)pauVar10 + (ulonglong)uVar4 * -0x78 + -0x78),8);
    if (SBORROW8(0,lStack_120)) goto joined_r0x00014085a8e5;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if (lStack_120 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    in_RDX = 0;
    uVar7 = uVar9;
    if (lVar2 != 0) {
      in_RDX = lVar2 << 2;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
LAB_14085a8e7:
  if (uVar5 == 0) {
    uVar6 = 0;
  }
  else {
    memset(_Dst,(int)CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff),uVar5 + 0x11);
    uVar6 = (uVar5 + 1 & 0xfffffffffffffff8) - (uVar5 + 1 >> 3);
    if (uVar5 < 8) {
      uVar6 = uVar5;
    }
  }
  in_RCX[2] = uVar6;
  in_RCX[3] = 0;
  *in_RCX = _Dst;
  in_RCX[1] = uVar5;
  return;
joined_r0x00014085a8e5:
  for (; lVar8 != 0; lVar8 = lVar8 + -1) {
    if ((short)uVar9 == 0) {
      do {
        auVar1 = *pauVar11;
        uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
        pauVar10 = pauVar10 + -0x78;
        pauVar11 = pauVar11 + 1;
      } while (uVar3 == 0xffff);
      uVar9 = ~(uint)uVar3;
      uVar7 = uVar9 & -(uint)uVar3 - 2;
    }
    else {
      uVar7 = uVar9 - 1 & uVar9;
    }
    uVar4 = 0;
    for (; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    if (*(longlong *)((longlong)pauVar10 + (ulonglong)uVar4 * -0x78 + -0x70) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar2 = *(longlong *)((longlong)pauVar10 + (ulonglong)uVar4 * -0x78 + -0x58);
    in_RDX = 0;
    if (lVar2 != 0) {
      in_RDX = lVar2 << 2;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar9 = uVar7;
  }
  goto LAB_14085a8e7;
}

