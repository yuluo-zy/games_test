// ADDRESS: 0x140a35240
// SIZE: 687
// SIGNATURE: void __cdecl set_attribute<enum2$<country_core::render::mesh::VertexAttributeValues>_>(void)

void __cdecl
country_core::render::mesh::Mesh::
set_attribute<enum2_<country_core::render::mesh::VertexAttributeValues>_>(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  ulonglong uVar4;
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
  undefined8 uVar27;
  undefined8 uVar28;
  undefined8 uVar29;
  ushort uVar30;
  int iVar31;
  ulonglong extraout_RAX;
  void *extraout_RAX_00;
  longlong in_RCX;
  void *in_RDX;
  size_t in_R8;
  longlong lVar32;
  undefined8 *in_R9;
  ulonglong uVar33;
  uint uVar34;
  void *_Dst;
  longlong lVar35;
  byte bVar36;
  undefined1 auVar37 [16];
  char cVar38;
  char cVar39;
  char cVar40;
  char cVar41;
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
  undefined1 auVar42 [16];
  longlong local_b8;
  longlong local_b0;
  
  if (*(longlong *)(in_RCX + 0x50) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar3 = *(longlong *)(in_RCX + 0x38);
    uVar4 = *(ulonglong *)(in_RCX + 0x40);
    bVar36 = (byte)(extraout_RAX >> 0x39);
    auVar37 = ZEXT216(CONCAT11(bVar36,bVar36));
    auVar37 = pshuflw(auVar37,auVar37,0);
    cVar38 = auVar37[0];
    cVar39 = auVar37[1];
    cVar40 = auVar37[2];
    cVar41 = auVar37[3];
    lVar32 = 0;
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
    uVar33 = extraout_RAX;
    while( true ) {
      uVar33 = uVar4 & uVar33;
      pcVar1 = (char *)(lVar3 + uVar33);
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
      auVar37[0] = -(cVar11 == cVar38);
      auVar37[1] = -(cVar12 == cVar39);
      auVar37[2] = -(cVar13 == cVar40);
      auVar37[3] = -(cVar14 == cVar41);
      auVar37[4] = -(cVar15 == cVar38);
      auVar37[5] = -(cVar16 == cVar39);
      auVar37[6] = -(cVar17 == cVar40);
      auVar37[7] = -(cVar18 == cVar41);
      auVar37[8] = -(cVar19 == cVar38);
      auVar37[9] = -(cVar20 == cVar39);
      auVar37[10] = -(cVar21 == cVar40);
      auVar37[0xb] = -(cVar22 == cVar41);
      auVar37[0xc] = -(cVar23 == cVar38);
      auVar37[0xd] = -(cVar24 == cVar39);
      auVar37[0xe] = -(cVar25 == cVar40);
      auVar37[0xf] = -(cVar26 == cVar41);
      uVar30 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf;
      uVar34 = (uint)uVar30;
      while (uVar30 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar34; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar35 = (uVar5 + uVar33 & uVar4) * -0x38;
        if ((*(size_t *)(lVar3 + -0x28 + lVar35) == in_R8) &&
           (iVar31 = memcmp(in_RDX,*(void **)(lVar3 + lVar35 + -0x30),in_R8), iVar31 == 0)) {
          lVar32 = lVar3 + lVar35;
          lVar2 = lVar3 + lVar35;
          switch(*(undefined8 *)(lVar3 + -0x20 + lVar35)) {
          case 0:
          case 1:
            lVar3 = *(longlong *)(lVar32 + -0x18);
            break;
          case 2:
            lVar3 = *(longlong *)(lVar32 + -0x18);
            break;
          case 3:
            lVar3 = *(longlong *)(lVar32 + -0x18);
            break;
          default:
            lVar3 = *(longlong *)(lVar32 + -0x18);
          }
          if (lVar3 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          uVar27 = *in_R9;
          uVar28 = in_R9[1];
          uVar29 = in_R9[3];
          *(undefined8 *)(lVar2 + -0x10) = in_R9[2];
          *(undefined8 *)(lVar2 + -8) = uVar29;
          *(undefined8 *)(lVar2 + -0x20) = uVar27;
          *(undefined8 *)(lVar2 + -0x18) = uVar28;
          return;
        }
        uVar30 = (ushort)(uVar34 - 1) & (ushort)uVar34;
        uVar34 = CONCAT22((short)(uVar34 - 1 >> 0x10),uVar30);
      }
      auVar42[0] = -(cVar11 == (char)iVar7);
      auVar42[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar42[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar42[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar42[4] = -(cVar15 == (char)iVar8);
      auVar42[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar42[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar42[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar42[8] = -(cVar19 == (char)iVar9);
      auVar42[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar42[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar42[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar42[0xc] = -(cVar23 == (char)iVar10);
      auVar42[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar42[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar42[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar42 >> 0x4f,0) & 1) != 0) || (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar42 >> 0x5f,0) & 1) != 0) || (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0) ||
          auVar42[0xf] < '\0') break;
      uVar33 = uVar33 + lVar32 + 0x10;
      lVar32 = lVar32 + 0x10;
    }
  }
  if ((longlong)in_R8 < 0) {
LAB_140a3555d:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_140a3555d;
  }
  memcpy(_Dst,in_RDX,in_R8);
  hashbrown::map::HashMap::
  insert<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (local_b8 != 5) {
    switch(local_b8) {
    case 0:
    case 1:
      break;
    case 2:
      break;
    case 3:
      break;
    default:
    }
    if (local_b0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
  }
  return;
}

