
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_env_0>_>
          (void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int extraout_EAX;
  int extraout_EAX_00;
  int extraout_EAX_01;
  int *extraout_RAX;
  longlong lVar7;
  ulonglong extraout_RAX_00;
  longlong *in_RCX;
  longlong *plVar8;
  undefined8 *puVar9;
  int iVar10;
  longlong *in_RDX;
  int iVar11;
  ushort uVar12;
  int *piVar13;
  undefined1 (*pauVar14) [16];
  uint uVar15;
  ulonglong uVar16;
  longlong *plVar17;
  longlong *plVar18;
  undefined8 *puVar19;
  undefined8 *puVar20;
  undefined1 (*pauVar21) [16];
  longlong lVar22;
  uint uVar23;
  char cVar24;
  byte bVar25;
  char cVar27;
  char cVar28;
  char cVar29;
  undefined1 auVar26 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
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
  undefined1 (*local_138) [16];
  undefined1 (*local_130) [16];
  undefined1 *local_128;
  ushort local_120;
  longlong local_118;
  longlong *local_110;
  longlong local_108;
  undefined1 local_e0 [8];
  longlong local_d8;
  undefined1 (*local_d0) [16];
  ushort local_c0;
  longlong local_b8;
  byte local_b0;
  longlong local_a8;
  longlong *local_a0;
  ulonglong local_98;
  longlong local_90;
  longlong *local_88;
  longlong local_80;
  int *piStack_78;
  longlong local_70;
  int local_64;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_88 = in_RCX;
  if (*in_RDX == 0) goto LAB_1412feba9;
  while (iVar10 = (int)in_RDX + 0x28,
        core::iter::adapters::copied::impl$1::
        try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple_<>,core::iter::traits::iterator::Iterator::find::check::closure_env_0<country_core::resources::walls::decorator::DecoratorId,ref_mut_<country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>_>,enum2_<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>_>
                  (), extraout_EAX != 1) {
    *in_RDX = 0;
LAB_1412feba9:
    if ((*(byte *)(in_RDX + 0x11) == 2) || (lVar22 = in_RDX[0x10], lVar22 == 0)) {
LAB_1412febf2:
      if (in_RDX[6] != 0) {
        iVar10 = (int)in_RDX + 0x58;
        core::iter::adapters::copied::impl$1::
        try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple_<>,core::iter::traits::iterator::Iterator::find::check::closure_env_0<country_core::resources::walls::decorator::DecoratorId,ref_mut_<country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>_>,enum2_<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>_>
                  ();
        if ((extraout_EAX_00 == 1) || (in_RDX[6] = 0, extraout_EAX_00 != 0)) break;
      }
      *local_88 = 0;
      local_88[1] = 4;
      local_88[2] = 0;
      return;
    }
    uVar12 = *(ushort *)(in_RDX + 0xf);
    uVar15 = (uint)uVar12;
    lVar7 = in_RDX[0xc];
    if (uVar12 == 0) {
      pauVar21 = (undefined1 (*) [16])in_RDX[0xd];
      do {
        auVar26 = *pauVar21;
        uVar12 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar26[0xf] >> 7) << 0xf;
        lVar7 = lVar7 + -0x700;
        pauVar21 = pauVar21 + 1;
      } while (uVar12 == 0xffff);
      in_RDX[0xd] = (longlong)pauVar21;
      in_RDX[0xc] = lVar7;
      uVar15 = ~(uint)uVar12;
      *(ushort *)(in_RDX + 0xf) = (ushort)uVar15 & -uVar12 - 2;
      in_RDX[0x10] = lVar22 + -1;
    }
    else {
      *(ushort *)(in_RDX + 0xf) = uVar12 - 1 & uVar12;
      in_RDX[0x10] = lVar22 + -1;
      if (lVar7 == 0) goto LAB_1412febf2;
    }
    uVar23 = 0;
    for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar23 = uVar23 + 1;
    }
    puVar19 = (undefined8 *)(lVar7 + (ulonglong)uVar23 * -0x70 + -0x60);
    puVar9 = (undefined8 *)(lVar7 + (ulonglong)uVar23 * -0x70 + -0x30);
    puVar20 = puVar19;
    if ((*(byte *)(in_RDX + 0x11) & 1) != 0) {
      puVar20 = puVar9;
      puVar9 = puVar19;
    }
    pauVar21 = (undefined1 (*) [16])*puVar20;
    lVar22 = puVar20[1];
    auVar26 = *pauVar21;
    lVar7 = puVar20[3];
    *in_RDX = (longlong)pauVar21;
    in_RDX[1] = (longlong)(pauVar21 + 1);
    in_RDX[2] = (longlong)(*pauVar21 + lVar22 + 1);
    *(ushort *)(in_RDX + 3) =
         ~((ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar26[0xf] >> 7) << 0xf
          );
    in_RDX[4] = lVar7;
    in_RDX[5] = (longlong)puVar9;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (int *)0x0) {
                    // WARNING: Subroutine does not return
    raw_vec::handle_error();
  }
  *extraout_RAX = iVar10;
  local_80 = 4;
  local_70 = 1;
  piStack_78 = extraout_RAX;
  memcpy(&local_138,in_RDX,0x90);
  local_90 = 1;
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                 CONCAT13(unaff_XMM6_Bp,
                          CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  pauVar21 = local_138;
  pauVar14 = local_130;
  lVar22 = local_118;
  plVar8 = local_110;
  piVar13 = extraout_RAX;
  do {
    while (local_138 = pauVar21, local_130 = pauVar14, local_118 = lVar22, local_110 = plVar8,
          pauVar21 != (undefined1 (*) [16])0x0) {
      if (lVar22 == 0) {
LAB_1412fee30:
        local_138 = (undefined1 (*) [16])0x0;
        break;
      }
      local_a0 = plVar8 + 4;
      uVar15 = (uint)local_120;
LAB_1412fed72:
      if ((short)uVar15 == 0) {
        do {
          auVar26 = *pauVar14;
          uVar12 = (ushort)(SUB161(auVar26 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar26[0xf] >> 7) << 0xf;
          pauVar21 = pauVar21 + -0x58;
          pauVar14 = pauVar14 + 1;
        } while (uVar12 == 0xffff);
        uVar15 = ~(uint)uVar12;
        uVar23 = uVar15 & -(uint)uVar12 - 2;
        local_138 = pauVar21;
        local_130 = pauVar14;
      }
      else {
        uVar23 = uVar15 - 1 & uVar15;
      }
      local_120 = (ushort)uVar23;
      lVar22 = lVar22 + -1;
      uVar2 = 0;
      for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar2 = uVar2 + 1;
      }
      iVar11 = *(int *)((longlong)pauVar21 + (ulonglong)uVar2 * -0x58 + -0x58);
      local_118 = lVar22;
      local_64 = iVar11;
      if (plVar8[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        local_98 = plVar8[1];
        bVar25 = (byte)(extraout_RAX_00 >> 0x39);
        auVar26 = ZEXT216(CONCAT11(bVar25,bVar25));
        auVar26 = pshuflw(auVar26,auVar26,0);
        local_a8 = *plVar8 + -0x58;
        lVar7 = 0;
        uVar16 = extraout_RAX_00;
        while( true ) {
          uVar16 = local_98 & uVar16;
          pcVar1 = (char *)(*plVar8 + uVar16);
          cVar24 = auVar26[0];
          auVar31[0] = -(*pcVar1 == cVar24);
          cVar27 = auVar26[1];
          auVar31[1] = -(pcVar1[1] == cVar27);
          cVar28 = auVar26[2];
          auVar31[2] = -(pcVar1[2] == cVar28);
          cVar29 = auVar26[3];
          auVar31[3] = -(pcVar1[3] == cVar29);
          auVar31[4] = -(pcVar1[4] == cVar24);
          auVar31[5] = -(pcVar1[5] == cVar27);
          auVar31[6] = -(pcVar1[6] == cVar28);
          auVar31[7] = -(pcVar1[7] == cVar29);
          auVar31[8] = -(pcVar1[8] == cVar24);
          auVar31[9] = -(pcVar1[9] == cVar27);
          auVar31[10] = -(pcVar1[10] == cVar28);
          auVar31[0xb] = -(pcVar1[0xb] == cVar29);
          auVar31[0xc] = -(pcVar1[0xc] == cVar24);
          auVar31[0xd] = -(pcVar1[0xd] == cVar27);
          auVar31[0xe] = -(pcVar1[0xe] == cVar28);
          auVar31[0xf] = -(pcVar1[0xf] == cVar29);
          uVar12 = (ushort)(SUB161(auVar31 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar31[0xf] >> 7) << 0xf;
          uVar15 = (uint)uVar12;
          while (uVar12 != 0) {
            uVar2 = 0;
            for (uVar3 = uVar15; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
              uVar2 = uVar2 + 1;
            }
            if (local_64 == *(int *)(local_a8 + (uVar2 + uVar16 & local_98) * -0x58)) {
              uVar15 = uVar23;
              if (lVar22 != 0) goto LAB_1412fed72;
              goto LAB_1412fee30;
            }
            uVar12 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar12);
          }
          auVar30[0] = -(*pcVar1 == (char)iVar10);
          auVar30[1] = -(pcVar1[1] == (char)((uint)iVar10 >> 8));
          auVar30[2] = -(pcVar1[2] == (char)((uint)iVar10 >> 0x10));
          auVar30[3] = -(pcVar1[3] == (char)((uint)iVar10 >> 0x18));
          auVar30[4] = -(pcVar1[4] == (char)iVar4);
          auVar30[5] = -(pcVar1[5] == (char)((uint)iVar4 >> 8));
          auVar30[6] = -(pcVar1[6] == (char)((uint)iVar4 >> 0x10));
          auVar30[7] = -(pcVar1[7] == (char)((uint)iVar4 >> 0x18));
          auVar30[8] = -(pcVar1[8] == (char)iVar5);
          auVar30[9] = -(pcVar1[9] == (char)((uint)iVar5 >> 8));
          auVar30[10] = -(pcVar1[10] == (char)((uint)iVar5 >> 0x10));
          auVar30[0xb] = -(pcVar1[0xb] == (char)((uint)iVar5 >> 0x18));
          auVar30[0xc] = -(pcVar1[0xc] == (char)iVar6);
          auVar30[0xd] = -(pcVar1[0xd] == (char)((uint)iVar6 >> 8));
          auVar30[0xe] = -(pcVar1[0xe] == (char)((uint)iVar6 >> 0x10));
          auVar30[0xf] = -(pcVar1[0xf] == (char)((uint)iVar6 >> 0x18));
          if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 || (SUB161(auVar30 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar30 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar30 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x2f,0) & 1) != 0) || (SUB161(auVar30 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x47,0) & 1) != 0) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') break;
          uVar16 = uVar16 + lVar7 + 0x10;
          lVar7 = lVar7 + 0x10;
        }
      }
LAB_1412fef70:
      if (local_90 == local_80) {
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        piVar13 = piStack_78;
      }
      piVar13[local_90] = iVar11;
      local_90 = local_90 + 1;
      pauVar21 = local_138;
      pauVar14 = local_130;
      lVar22 = local_118;
      plVar8 = local_110;
      local_70 = local_90;
    }
    lVar22 = local_b8;
    if ((local_b0 == 2) || (local_b8 == 0)) {
LAB_1412fef41:
      local_b8 = lVar22;
      if (local_108 != 0) {
        iVar11 = (int)local_e0;
        core::iter::adapters::copied::impl$1::
        try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple_<>,core::iter::traits::iterator::Iterator::find::check::closure_env_0<country_core::resources::walls::decorator::DecoratorId,ref_mut_<country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>_>,enum2_<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>_>
                  ();
        if (extraout_EAX_01 == 1) goto LAB_1412fef70;
      }
      local_88[2] = local_70;
      *local_88 = local_80;
      local_88[1] = (longlong)piStack_78;
      return;
    }
    uVar15 = (uint)local_c0;
    if (local_c0 == 0) {
      do {
        auVar26 = *local_d0;
        uVar12 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar26[0xf] >> 7) << 0xf;
        local_d8 = local_d8 + -0x700;
        local_d0 = local_d0 + 1;
      } while (uVar12 == 0xffff);
      uVar15 = ~(uint)uVar12;
      local_c0 = (ushort)uVar15 & -uVar12 - 2;
    }
    else {
      local_c0 = local_c0 - 1 & local_c0;
      lVar22 = local_b8 + -1;
      if (local_d8 == 0) goto LAB_1412fef41;
    }
    local_b8 = local_b8 + -1;
    uVar23 = 0;
    for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar23 = uVar23 + 1;
    }
    plVar17 = (longlong *)(local_d8 + (ulonglong)uVar23 * -0x70 + -0x60);
    plVar8 = (longlong *)(local_d8 + (ulonglong)uVar23 * -0x70 + -0x30);
    plVar18 = plVar17;
    if ((local_b0 & 1) != 0) {
      plVar18 = plVar8;
      plVar8 = plVar17;
    }
    pauVar21 = (undefined1 (*) [16])*plVar18;
    local_128 = *pauVar21 + plVar18[1] + 1;
    auVar26 = *pauVar21;
    local_120 = ~((ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar26[0xf] >> 7) << 0xf);
    pauVar14 = pauVar21 + 1;
    lVar22 = plVar18[3];
  } while( true );
}

