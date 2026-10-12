
void __cdecl
country_core::render::mesh::Mesh::
set_attribute<alloc::vec::Vec<array_<f32,2>,alloc::alloc::Global>_>(void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  char cVar10;
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
  ushort uVar26;
  int iVar27;
  ulonglong extraout_RAX;
  void *extraout_RAX_00;
  longlong in_RCX;
  void *in_RDX;
  size_t in_R8;
  longlong lVar28;
  longlong *in_R9;
  ulonglong uVar29;
  uint uVar30;
  void *_Dst;
  longlong lVar31;
  byte bVar32;
  undefined1 auVar33 [16];
  char cVar34;
  char cVar35;
  char cVar36;
  char cVar37;
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
  undefined1 auVar38 [16];
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  
  if (*(longlong *)(in_RCX + 0x50) != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar2 = *(longlong *)(in_RCX + 0x38);
    uVar3 = *(ulonglong *)(in_RCX + 0x40);
    bVar32 = (byte)(extraout_RAX >> 0x39);
    auVar33 = ZEXT216(CONCAT11(bVar32,bVar32));
    auVar33 = pshuflw(auVar33,auVar33,0);
    cVar34 = auVar33[0];
    cVar35 = auVar33[1];
    cVar36 = auVar33[2];
    cVar37 = auVar33[3];
    lVar28 = 0;
    iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                   CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar29 = extraout_RAX;
    while( true ) {
      uVar29 = uVar3 & uVar29;
      pcVar1 = (char *)(lVar2 + uVar29);
      cVar10 = *pcVar1;
      cVar11 = pcVar1[1];
      cVar12 = pcVar1[2];
      cVar13 = pcVar1[3];
      cVar14 = pcVar1[4];
      cVar15 = pcVar1[5];
      cVar16 = pcVar1[6];
      cVar17 = pcVar1[7];
      cVar18 = pcVar1[8];
      cVar19 = pcVar1[9];
      cVar20 = pcVar1[10];
      cVar21 = pcVar1[0xb];
      cVar22 = pcVar1[0xc];
      cVar23 = pcVar1[0xd];
      cVar24 = pcVar1[0xe];
      cVar25 = pcVar1[0xf];
      auVar33[0] = -(cVar10 == cVar34);
      auVar33[1] = -(cVar11 == cVar35);
      auVar33[2] = -(cVar12 == cVar36);
      auVar33[3] = -(cVar13 == cVar37);
      auVar33[4] = -(cVar14 == cVar34);
      auVar33[5] = -(cVar15 == cVar35);
      auVar33[6] = -(cVar16 == cVar36);
      auVar33[7] = -(cVar17 == cVar37);
      auVar33[8] = -(cVar18 == cVar34);
      auVar33[9] = -(cVar19 == cVar35);
      auVar33[10] = -(cVar20 == cVar36);
      auVar33[0xb] = -(cVar21 == cVar37);
      auVar33[0xc] = -(cVar22 == cVar34);
      auVar33[0xd] = -(cVar23 == cVar35);
      auVar33[0xe] = -(cVar24 == cVar36);
      auVar33[0xf] = -(cVar25 == cVar37);
      uVar26 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe | (ushort)(auVar33[0xf] >> 7) << 0xf;
      uVar30 = (uint)uVar26;
      while (uVar26 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar30; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar31 = (uVar4 + uVar29 & uVar3) * -0x38;
        if ((*(size_t *)(lVar2 + -0x28 + lVar31) == in_R8) &&
           (iVar27 = memcmp(in_RDX,*(void **)(lVar2 + lVar31 + -0x30),in_R8), iVar27 == 0)) {
          mesh::impl$6::from();
          lVar28 = lVar2 + lVar31;
          lVar2 = lVar2 + lVar31;
          switch(*(undefined8 *)(lVar28 + -0x20)) {
          case 0:
          case 1:
            lVar28 = *(longlong *)(lVar28 + -0x18);
            break;
          case 2:
            lVar28 = *(longlong *)(lVar28 + -0x18);
            break;
          case 3:
            lVar28 = *(longlong *)(lVar28 + -0x18);
            break;
          default:
            lVar28 = *(longlong *)(lVar28 + -0x18);
          }
          if (lVar28 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          *(undefined8 *)(lVar2 + -0x10) = local_f0;
          *(undefined8 *)(lVar2 + -8) = uStack_e8;
          *(undefined8 *)(lVar2 + -0x20) = local_100;
          *(undefined8 *)(lVar2 + -0x18) = uStack_f8;
          return;
        }
        uVar26 = (ushort)(uVar30 - 1) & (ushort)uVar30;
        uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar26);
      }
      auVar38[0] = -(cVar10 == (char)iVar6);
      auVar38[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
      auVar38[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
      auVar38[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
      auVar38[4] = -(cVar14 == (char)iVar7);
      auVar38[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
      auVar38[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
      auVar38[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
      auVar38[8] = -(cVar18 == (char)iVar8);
      auVar38[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
      auVar38[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
      auVar38[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
      auVar38[0xc] = -(cVar22 == (char)iVar9);
      auVar38[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
      auVar38[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
      auVar38[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar38 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar38 >> 0x3f,0) & 1) != 0) || (SUB161(auVar38 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar38 >> 0x4f,0) & 1) != 0) || (SUB161(auVar38 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar38 >> 0x5f,0) & 1) != 0) || (SUB161(auVar38 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar38 >> 0x6f,0) & 1) != 0) || (SUB161(auVar38 >> 0x77,0) & 1) != 0) ||
          auVar38[0xf] < '\0') break;
      uVar29 = uVar29 + lVar28 + 0x10;
      lVar28 = lVar28 + 0x10;
    }
  }
  if ((longlong)in_R8 < 0) {
LAB_141fdc192:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_00;
    if (extraout_RAX_00 == (void *)0x0) goto LAB_141fdc192;
  }
  memcpy(_Dst,in_RDX,in_R8);
  lVar2 = *in_R9;
  lVar28 = in_R9[1];
  mesh::impl$6::from();
  hashbrown::map::HashMap::
  insert<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (lVar2 == 5) {
    return;
  }
  switch(lVar2) {
  case 0:
  case 1:
    break;
  case 2:
    break;
  case 3:
    goto joined_r0x000141fdc159;
  default:
joined_r0x000141fdc159:
    if (lVar28 == 0) {
      return;
    }
    goto code_r0x000141fdc163;
  }
  if (lVar28 != 0) {
code_r0x000141fdc163:
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

