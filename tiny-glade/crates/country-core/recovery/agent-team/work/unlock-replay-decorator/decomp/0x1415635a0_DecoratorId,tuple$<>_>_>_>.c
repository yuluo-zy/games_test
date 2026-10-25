
void __cdecl
core::iter::adapters::copied::impl$1::
try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple_<>,core::iter::traits::iterator::Iterator::find::check::closure_env_0<country_core::resources::walls::decorator::DecoratorId,ref_mut_<country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>_>,enum2_<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>_>
          (void)

{
  char *pcVar1;
  int iVar2;
  longlong *plVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  longlong lVar11;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  uint uVar12;
  ushort uVar13;
  ulonglong uVar14;
  undefined1 (*pauVar15) [16];
  longlong lVar16;
  longlong lVar17;
  char cVar18;
  byte bVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  
  lVar16 = in_RCX[4];
  if (lVar16 == 0) {
    return;
  }
  lVar17 = *in_RCX;
  pauVar15 = (undefined1 (*) [16])in_RCX[1];
  plVar3 = (longlong *)*in_RDX;
  iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                 CONCAT13(unaff_XMM6_Bp,
                          CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  uVar10 = (uint)*(ushort *)(in_RCX + 3);
LAB_141563691:
  if ((short)uVar10 == 0) {
    do {
      auVar20 = *pauVar15;
      uVar13 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar20[0xf] >> 7) << 0xf;
      lVar17 = lVar17 + -0x580;
      pauVar15 = pauVar15 + 1;
    } while (uVar13 == 0xffff);
    in_RCX[1] = (longlong)pauVar15;
    *in_RCX = lVar17;
    uVar10 = ~(uint)uVar13;
    uVar12 = uVar10 & -(uint)uVar13 - 2;
    *(short *)(in_RCX + 3) = (short)uVar12;
    in_RCX[4] = lVar16 + -1;
  }
  else {
    uVar12 = uVar10 - 1 & uVar10;
    *(short *)(in_RCX + 3) = (short)uVar12;
    in_RCX[4] = lVar16 + -1;
    if (lVar17 == 0) {
      return;
    }
  }
  lVar16 = lVar16 + -1;
  uVar4 = 0;
  for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
    uVar4 = uVar4 + 1;
  }
  iVar2 = *(int *)(lVar17 + -0x58 + (ulonglong)uVar4 * -0x58);
  if (plVar3[3] == 0) {
    return;
  }
  hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  bVar19 = (byte)(extraout_RAX >> 0x39);
  auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar11 = 0;
  uVar14 = extraout_RAX;
  do {
    uVar14 = plVar3[1] & uVar14;
    pcVar1 = (char *)(*plVar3 + uVar14);
    cVar18 = auVar20[0];
    auVar25[0] = -(*pcVar1 == cVar18);
    cVar21 = auVar20[1];
    auVar25[1] = -(pcVar1[1] == cVar21);
    cVar22 = auVar20[2];
    auVar25[2] = -(pcVar1[2] == cVar22);
    cVar23 = auVar20[3];
    auVar25[3] = -(pcVar1[3] == cVar23);
    auVar25[4] = -(pcVar1[4] == cVar18);
    auVar25[5] = -(pcVar1[5] == cVar21);
    auVar25[6] = -(pcVar1[6] == cVar22);
    auVar25[7] = -(pcVar1[7] == cVar23);
    auVar25[8] = -(pcVar1[8] == cVar18);
    auVar25[9] = -(pcVar1[9] == cVar21);
    auVar25[10] = -(pcVar1[10] == cVar22);
    auVar25[0xb] = -(pcVar1[0xb] == cVar23);
    auVar25[0xc] = -(pcVar1[0xc] == cVar18);
    auVar25[0xd] = -(pcVar1[0xd] == cVar21);
    auVar25[0xe] = -(pcVar1[0xe] == cVar22);
    auVar25[0xf] = -(pcVar1[0xf] == cVar23);
    uVar13 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
    uVar10 = (uint)uVar13;
    while (uVar13 != 0) {
      uVar4 = 0;
      for (uVar5 = uVar10; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      if (iVar2 == *(int *)(*plVar3 + -0x58 + (uVar4 + uVar14 & plVar3[1]) * -0x58)) {
        uVar10 = uVar12;
        if (lVar16 == 0) {
          return;
        }
        goto LAB_141563691;
      }
      uVar13 = (ushort)(uVar10 - 1) & (ushort)uVar10;
      uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar13);
    }
    auVar24[0] = -(*pcVar1 == (char)iVar6);
    auVar24[1] = -(pcVar1[1] == (char)((uint)iVar6 >> 8));
    auVar24[2] = -(pcVar1[2] == (char)((uint)iVar6 >> 0x10));
    auVar24[3] = -(pcVar1[3] == (char)((uint)iVar6 >> 0x18));
    auVar24[4] = -(pcVar1[4] == (char)iVar7);
    auVar24[5] = -(pcVar1[5] == (char)((uint)iVar7 >> 8));
    auVar24[6] = -(pcVar1[6] == (char)((uint)iVar7 >> 0x10));
    auVar24[7] = -(pcVar1[7] == (char)((uint)iVar7 >> 0x18));
    auVar24[8] = -(pcVar1[8] == (char)iVar8);
    auVar24[9] = -(pcVar1[9] == (char)((uint)iVar8 >> 8));
    auVar24[10] = -(pcVar1[10] == (char)((uint)iVar8 >> 0x10));
    auVar24[0xb] = -(pcVar1[0xb] == (char)((uint)iVar8 >> 0x18));
    auVar24[0xc] = -(pcVar1[0xc] == (char)iVar9);
    auVar24[0xd] = -(pcVar1[0xd] == (char)((uint)iVar9 >> 8));
    auVar24[0xe] = -(pcVar1[0xe] == (char)((uint)iVar9 >> 0x10));
    auVar24[0xf] = -(pcVar1[0xf] == (char)((uint)iVar9 >> 0x18));
    if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') {
      return;
    }
    uVar14 = uVar14 + lVar11 + 0x10;
    lVar11 = lVar11 + 0x10;
  } while( true );
}

