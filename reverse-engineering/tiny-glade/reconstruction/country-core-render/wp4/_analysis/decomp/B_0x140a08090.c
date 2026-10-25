// ADDRESS: 0x140a08090
// SIZE: 318
// SIGNATURE: void __cdecl retain_vertices(void)

void __cdecl country_core::render::mesh::Mesh::retain_vertices(void)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  ushort uVar3;
  uint uVar4;
  longlong in_RCX;
  ulonglong uVar5;
  int iVar6;
  longlong in_RDX;
  uint uVar7;
  ulonglong in_R8;
  longlong lVar8;
  int iVar9;
  undefined1 (*pauVar10) [16];
  longlong lVar11;
  bool bVar12;
  
  alloc::vec::Vec::
  retain<u32,alloc::alloc::Global,country_core::render::mesh::impl_0::retain_vertices::closure_env_0>
            ();
  lVar11 = *(longlong *)(in_RCX + 0x50);
  if (lVar11 != 0) {
    pauVar10 = *(undefined1 (**) [16])(in_RCX + 0x38) + 1;
    auVar1 = **(undefined1 (**) [16])(in_RCX + 0x38);
    uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
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
    do {
      if ((short)uVar4 == 0) {
        do {
          auVar1 = *pauVar10;
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
          pauVar10 = pauVar10 + 1;
        } while (uVar3 == 0xffff);
        uVar4 = ~(uint)uVar3;
        uVar7 = uVar4 & -(uint)uVar3 - 2;
      }
      else {
        uVar7 = uVar4 - 1 & uVar4;
      }
      for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
      }
      VertexAttributeValues::
      retain_by_index<country_core::render::mesh::impl_0::retain_vertices::closure_env_1>();
      lVar11 = lVar11 + -1;
      uVar4 = uVar7;
    } while (lVar11 != 0);
  }
  if (in_R8 != 0) {
    lVar11 = 0;
    iVar6 = 0;
    do {
      bVar12 = *(int *)(in_RDX + lVar11) == 1;
      iVar9 = -1;
      if (bVar12) {
        iVar9 = iVar6;
      }
      iVar6 = iVar6 + (uint)bVar12;
      *(int *)(in_RDX + lVar11) = iVar9;
      lVar11 = lVar11 + 4;
    } while (in_R8 * 4 - lVar11 != 0);
  }
  lVar11 = *(longlong *)(in_RCX + 0x10);
  if (lVar11 != 0) {
    lVar2 = *(longlong *)(in_RCX + 8);
    lVar8 = 0;
    do {
      uVar5 = (ulonglong)*(uint *)(lVar2 + lVar8);
      if (in_R8 <= uVar5) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      iVar6 = *(int *)(in_RDX + uVar5 * 4);
      if (iVar6 == -1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      *(int *)(lVar2 + lVar8) = iVar6;
      lVar8 = lVar8 + 4;
    } while (lVar11 << 2 != lVar8);
  }
  return;
}

