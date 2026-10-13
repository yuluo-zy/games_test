// ADDRESS: 0x140a34170
// SIZE: 758
// SIGNATURE: void __cdecl set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global>_>(void)

void __cdecl
country_core::render::mesh::Mesh::
set_attribute<alloc::vec::Vec<array_<f32,4>,alloc::alloc::Global>_>(void)

{
  char *pcVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  ushort uVar27;
  int iVar28;
  ulonglong extraout_RAX;
  void *extraout_RAX_00;
  longlong in_RCX;
  void *in_RDX;
  size_t in_R8;
  longlong lVar29;
  undefined8 *in_R9;
  ulonglong uVar30;
  uint uVar31;
  longlong lVar32;
  void *_Dst;
  longlong lVar33;
  byte bVar34;
  undefined1 auVar35 [16];
  char cVar36;
  char cVar37;
  char cVar38;
  char cVar39;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  undefined1 auVar40 [16];
  longlong local_c8;
  longlong local_c0;
  
  if (*(longlong *)(in_RCX + 0x50) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar32 = *(longlong *)(in_RCX + 0x38);
    uVar2 = *(ulonglong *)(in_RCX + 0x40);
    bVar34 = (byte)(extraout_RAX >> 0x39);
    auVar35 = ZEXT216(CONCAT11(bVar34,bVar34));
    auVar35 = pshuflw(auVar35,auVar35,0);
    cVar36 = auVar35[0];
    cVar37 = auVar35[1];
    cVar38 = auVar35[2];
    cVar39 = auVar35[3];
    lVar29 = 0;
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar30 = extraout_RAX;
    while( true ) {
      uVar30 = uVar2 & uVar30;
      pcVar1 = (char *)(lVar32 + uVar30);
      cVar11 = *pcVar1;
      cVar12 = pcVar1[1];
      cVar13 = pcVar1[2];
      cVar14 = pcVar1[3];
      cVar15 = pcVar1[4];
      cVar16 = pcVar1[5];
      cVar17 = pcVar1[6];
      cVar18 = pcVar1[7];
      cVar19 = pcVar1[8];
      cVar20 = pcVar1[9];
      cVar21 = pcVar1[10];
      cVar22 = pcVar1[0xb];
      cVar23 = pcVar1[0xc];
      cVar24 = pcVar1[0xd];
      cVar25 = pcVar1[0xe];
      cVar26 = pcVar1[0xf];
      auVar35[0] = -(cVar11 == cVar36);
      auVar35[1] = -(cVar12 == cVar37);
      auVar35[2] = -(cVar13 == cVar38);
      auVar35[3] = -(cVar14 == cVar39);
      auVar35[4] = -(cVar15 == cVar36);
      auVar35[5] = -(cVar16 == cVar37);
      auVar35[6] = -(cVar17 == cVar38);
      auVar35[7] = -(cVar18 == cVar39);
      auVar35[8] = -(cVar19 == cVar36);
      auVar35[9] = -(cVar20 == cVar37);
      auVar35[10] = -(cVar21 == cVar38);
      auVar35[0xb] = -(cVar22 == cVar39);
      auVar35[0xc] = -(cVar23 == cVar36);
      auVar35[0xd] = -(cVar24 == cVar37);
      auVar35[0xe] = -(cVar25 == cVar38);
      auVar35[0xf] = -(cVar26 == cVar39);
      uVar27 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf;
      uVar31 = (uint)uVar27;
      while (uVar27 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar31; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar33 = (uVar5 + uVar30 & uVar2) * -0x38;
        if ((*(size_t *)(lVar32 + -0x28 + lVar33) == in_R8) &&
           (iVar28 = memcmp(in_RDX,*(void **)(lVar32 + lVar33 + -0x30),in_R8), iVar28 == 0)) {
          uVar3 = *in_R9;
          uVar4 = in_R9[1];
          lVar29 = in_R9[2];
          if (lVar29 != 0) {
            lVar32 = lVar32 + lVar33;
            switch(*(undefined8 *)(lVar32 + -0x20)) {
            case 0:
            case 1:
              lVar33 = *(longlong *)(lVar32 + -0x18);
              break;
            case 2:
              lVar33 = *(longlong *)(lVar32 + -0x18);
              break;
            case 3:
              lVar33 = *(longlong *)(lVar32 + -0x18);
              break;
            default:
              lVar33 = *(longlong *)(lVar32 + -0x18);
            }
            if (lVar33 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            *(undefined8 *)(lVar32 + -0x20) = 4;
            *(undefined8 *)(lVar32 + -0x18) = uVar3;
            *(undefined8 *)(lVar32 + -0x10) = uVar4;
            *(longlong *)(lVar32 + -8) = lVar29;
            return;
          }
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        uVar27 = (ushort)(uVar31 - 1) & (ushort)uVar31;
        uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar27);
      }
      auVar40[0] = -(cVar11 == (char)iVar7);
      auVar40[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar40[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar40[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar40[4] = -(cVar15 == (char)iVar8);
      auVar40[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar40[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar40[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar40[8] = -(cVar19 == (char)iVar9);
      auVar40[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar40[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar40[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar40[0xc] = -(cVar23 == (char)iVar10);
      auVar40[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar40[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar40[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar40 >> 0x17,0) & 1) != 0) || (SUB161(auVar40 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar40 >> 0x27,0) & 1) != 0) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar40 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar40 >> 0x3f,0) & 1) != 0) || (SUB161(auVar40 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar40 >> 0x4f,0) & 1) != 0) || (SUB161(auVar40 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar40 >> 0x5f,0) & 1) != 0) || (SUB161(auVar40 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar40 >> 0x6f,0) & 1) != 0) || (SUB161(auVar40 >> 0x77,0) & 1) != 0) ||
          auVar40[0xf] < '\0') break;
      uVar30 = uVar30 + lVar29 + 0x10;
      lVar29 = lVar29 + 0x10;
    }
  }
  if ((longlong)in_R8 < 0) {
LAB_140a344dc:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_140a344dc;
  }
  memcpy(_Dst,in_RDX,in_R8);
  if (in_R9[2] == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  hashbrown::map::HashMap::
  insert<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (local_c8 == 5) {
    return;
  }
  switch(local_c8) {
  case 0:
  case 1:
    break;
  case 2:
    break;
  case 3:
    goto joined_r0x000140a3445f;
  default:
joined_r0x000140a3445f:
    if (local_c0 == 0) {
      return;
    }
    goto code_r0x000140a34469;
  }
  if (local_c0 != 0) {
code_r0x000140a34469:
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

