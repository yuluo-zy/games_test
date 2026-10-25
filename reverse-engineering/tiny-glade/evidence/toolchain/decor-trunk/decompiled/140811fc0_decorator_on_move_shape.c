// ADDR 0x140811fc0
// SIG void __cdecl decorator_on_move_shape(void)
// ORIG system::decorator_on_move_shape


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_on_move_shape::decorator_on_move_shape(void)

{
  char *pcVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  longlong lVar11;
  longlong extraout_RAX;
  ulonglong uVar12;
  ulonglong uVar13;
  undefined8 *in_RCX;
  ulonglong uVar14;
  undefined8 *in_RDX;
  ulonglong uVar15;
  longlong lVar16;
  undefined4 uVar17;
  longlong lVar18;
  longlong lVar19;
  longlong lVar20;
  int *piVar21;
  ulonglong uVar22;
  uint uVar23;
  ulonglong uVar24;
  ushort uVar25;
  undefined4 *puVar26;
  int *piVar27;
  char cVar28;
  byte bVar29;
  char cVar31;
  char cVar32;
  char cVar33;
  undefined1 auVar30 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  undefined1 unaff_XMM9_Bc;
  undefined1 unaff_XMM9_Bd;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  undefined1 unaff_XMM9_Bg;
  undefined1 unaff_XMM9_Bh;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  undefined1 unaff_XMM9_Bk;
  undefined1 unaff_XMM9_Bl;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  undefined1 unaff_XMM9_Bo;
  undefined1 unaff_XMM9_Bp;
  undefined1 local_128 [8];
  longlong local_120;
  undefined8 local_118;
  undefined4 local_10c;
  undefined1 local_108 [16];
  undefined1 *local_f8;
  ulonglong uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  ulonglong uStack_d0;
  undefined8 local_c8;
  undefined4 *local_b8;
  undefined8 local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong local_98;
  ulonglong *local_90;
  
  local_90 = (ulonglong *)*in_RDX;
  lVar16 = in_RDX[1];
  uVar12 = *local_90;
  uVar14 = 0;
  uVar13 = uVar12 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar12 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar13 = uVar14;
  }
  uVar15 = uVar12 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar12 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar15 = uVar14;
  }
  lVar11 = uVar13 * 0x30 + *(longlong *)(lVar16 + 8);
  uVar12 = *(ulonglong *)(lVar16 + 0x10) - uVar13;
  if (*(ulonglong *)(lVar16 + 0x10) < uVar13) {
    lVar11 = 8;
    uVar12 = uVar14;
  }
  lVar19 = uVar15 * 0x30 + *(longlong *)(lVar16 + 0x28);
  uVar13 = *(ulonglong *)(lVar16 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar16 + 0x30) < uVar15) {
    lVar19 = 8;
    uVar13 = uVar14;
  }
  *local_90 = (*(longlong *)(lVar16 + 0x40) - uVar13) - uVar12;
  auVar10 = ___xmm_7465646279746573646f72616e646f6d;
  auVar9 = ___xmm_6c7967656e657261736f6d6570736575;
  local_a0 = uVar12 * 0x30 + lVar11;
  local_a8 = uVar13 * 0x30 + lVar19;
  local_10c = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_b0 = *in_RCX;
  local_b8 = (undefined4 *)in_RCX[2];
  iVar5 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                 CONCAT13(unaff_XMM9_Bd,
                          CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  iVar6 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                 CONCAT13(unaff_XMM9_Bh,
                          CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  iVar7 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                 CONCAT13(unaff_XMM9_Bl,
                          CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                 CONCAT13(unaff_XMM9_Bp,
                          CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  puVar26 = local_b8;
  uVar17 = local_10c;
  do {
    do {
      if ((lVar11 == 0) || (lVar11 == local_a0)) {
        if ((lVar19 == 0) || (lVar19 == local_a8)) {
          return;
        }
        lVar20 = lVar19 + 0x30;
        lVar16 = 0;
        lVar18 = lVar19;
      }
      else {
        lVar16 = lVar11 + 0x30;
        lVar18 = lVar11;
        lVar20 = lVar19;
      }
      *local_90 = *local_90 + 1;
      *puVar26 = uVar17;
      local_120 = lVar20;
      country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
      lVar11 = lVar16;
      lVar19 = lVar20;
    } while ((extraout_RAX == 0) || (lVar19 = local_120, *(longlong *)(lVar18 + 0x10) == 0));
    piVar27 = *(int **)(lVar18 + 8);
    piVar21 = piVar27 + *(longlong *)(lVar18 + 0x10) * 3;
    local_98 = lVar16;
    do {
      iVar2 = *piVar27;
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (*(longlong *)(extraout_RAX + 0x48) != 0) {
        auVar30 = *(undefined1 (*) [16])(extraout_RAX + 0x50);
        auVar35._0_8_ = auVar30._0_8_;
        auVar35._8_4_ = auVar30._0_4_;
        auVar35._12_4_ = auVar30._4_4_;
        local_108 = auVar35 ^ auVar9;
        auVar36._0_8_ = auVar30._8_8_;
        auVar36._8_4_ = auVar30._8_4_;
        auVar36._12_4_ = auVar30._12_4_;
        local_f8 = SUB168(auVar36 ^ auVar10,0);
        uStack_f0 = SUB168(auVar36 ^ auVar10,8);
        local_d8 = 0;
        uStack_d0 = 0;
        local_c8 = 0;
        local_118 = CONCAT44(local_118._4_4_,iVar2);
        local_e8 = auVar35._0_8_;
        uStack_e0 = auVar36._0_8_;
        core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
        uVar24 = local_d8 << 0x38 | uStack_d0;
        uVar12 = uStack_f0 ^ uVar24;
        uVar22 = local_108._0_8_ + (longlong)local_f8;
        uVar14 = ((longlong)local_f8 << 0xd | (ulonglong)local_f8 >> 0x33) ^ uVar22;
        uVar13 = (uVar12 << 0x10 | uVar12 >> 0x30) ^ local_108._8_8_ + uVar12;
        uVar15 = local_108._8_8_ + uVar12 + uVar14;
        uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar13;
        uVar14 = (uVar14 << 0x11 | uVar14 >> 0x2f) ^ uVar15;
        uVar12 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar22;
        uVar22 = (uVar22 ^ uVar24) + uVar14;
        uVar15 = ((uVar15 << 0x20 | uVar15 >> 0x20) ^ 0xff) + uVar12;
        uVar13 = (uVar14 << 0xd | uVar14 >> 0x33) ^ uVar22;
        uVar12 = (uVar12 << 0x10 | uVar12 >> 0x30) ^ uVar15;
        uVar15 = uVar15 + uVar13;
        uVar22 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar12;
        uVar13 = (uVar13 << 0x11 | uVar13 >> 0x2f) ^ uVar15;
        uVar12 = (uVar12 << 0x15 | uVar12 >> 0x2b) ^ uVar22;
        uVar22 = uVar22 + uVar13;
        uVar14 = (uVar15 << 0x20 | uVar15 >> 0x20) + uVar12;
        uVar13 = (uVar13 << 0xd | uVar13 >> 0x33) ^ uVar22;
        uVar12 = (uVar12 << 0x10 | uVar12 >> 0x30) ^ uVar14;
        uVar14 = uVar14 + uVar13;
        uVar15 = (uVar22 << 0x20 | uVar22 >> 0x20) + uVar12;
        uVar13 = (uVar13 << 0x11 | uVar13 >> 0x2f) ^ uVar14;
        uVar12 = (uVar12 << 0x15 | uVar12 >> 0x2b) ^ uVar15;
        uVar14 = (uVar14 << 0x20 | uVar14 >> 0x20) + uVar12;
        uVar13 = (uVar13 << 0xd | uVar13 >> 0x33) ^ uVar15 + uVar13;
        uVar12 = (uVar12 << 0x10 | uVar12 >> 0x30) ^ uVar14;
        uVar14 = uVar14 + uVar13;
        uVar14 = (uVar14 << 0x20 | uVar14 >> 0x20) ^ (uVar13 << 0x11 | uVar13 >> 0x2f) ^
                 (uVar12 << 0x15 | uVar12 >> 0x2b) ^ uVar14;
        lVar16 = *(longlong *)(extraout_RAX + 0x30);
        bVar29 = (byte)(uVar14 >> 0x39);
        auVar30 = ZEXT216(CONCAT11(bVar29,bVar29));
        auVar30 = pshuflw(auVar30,auVar30,0);
        lVar11 = 0;
        while( true ) {
          uVar14 = uVar14 & *(ulonglong *)(extraout_RAX + 0x38);
          pcVar1 = (char *)(lVar16 + uVar14);
          cVar28 = auVar30[0];
          auVar37[0] = -(*pcVar1 == cVar28);
          cVar31 = auVar30[1];
          auVar37[1] = -(pcVar1[1] == cVar31);
          cVar32 = auVar30[2];
          auVar37[2] = -(pcVar1[2] == cVar32);
          cVar33 = auVar30[3];
          auVar37[3] = -(pcVar1[3] == cVar33);
          auVar37[4] = -(pcVar1[4] == cVar28);
          auVar37[5] = -(pcVar1[5] == cVar31);
          auVar37[6] = -(pcVar1[6] == cVar32);
          auVar37[7] = -(pcVar1[7] == cVar33);
          auVar37[8] = -(pcVar1[8] == cVar28);
          auVar37[9] = -(pcVar1[9] == cVar31);
          auVar37[10] = -(pcVar1[10] == cVar32);
          auVar37[0xb] = -(pcVar1[0xb] == cVar33);
          auVar37[0xc] = -(pcVar1[0xc] == cVar28);
          auVar37[0xd] = -(pcVar1[0xd] == cVar31);
          auVar37[0xe] = -(pcVar1[0xe] == cVar32);
          auVar37[0xf] = -(pcVar1[0xf] == cVar33);
          uVar25 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar37[0xf] >> 7) << 0xf;
          uVar23 = (uint)uVar25;
          while (uVar25 != 0) {
            uVar3 = 0;
            for (uVar4 = uVar23; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
              uVar3 = uVar3 + 1;
            }
            lVar19 = (uVar3 + uVar14 & *(ulonglong *)(extraout_RAX + 0x38)) * -0x50;
            if (iVar2 == *(int *)(lVar16 + -0x50 + lVar19)) {
              bVar29 = *(byte *)(lVar16 + -0x26 + lVar19);
              lVar11 = (ulonglong)bVar29 - 1;
              if (2 < (byte)(bVar29 - 2)) {
                lVar11 = 0;
              }
              if (lVar11 != 0) {
                if (1 < lVar11 - 2U) {
                  local_108._8_8_ = 1;
                  local_108._0_8_ = &PTR_s_internal_error__entered_unreacha_142a8ed90;
                  local_f8 = local_128;
                  uStack_f0 = 0;
                  local_e8 = 0;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_fmt();
                }
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              local_118 = CONCAT44((float)((ulonglong)*(undefined8 *)(piVar27 + 1) >> 0x20) +
                                   (float)((ulonglong)*(undefined8 *)(lVar18 + 0x20) >> 0x20),
                                   (float)*(undefined8 *)(piVar27 + 1) +
                                   (float)*(undefined8 *)(lVar18 + 0x20));
              ::utils::swizzlers::impl$1::xvy();
              *(undefined4 *)(lVar16 + lVar19 + -0x2c) = local_108._8_4_;
              *(undefined8 *)(lVar16 + lVar19 + -0x34) = local_108._0_8_;
              goto LAB_1408123e0;
            }
            uVar25 = (ushort)(uVar23 - 1) & (ushort)uVar23;
            uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar25);
          }
          auVar34[0] = -(*pcVar1 == (char)iVar5);
          auVar34[1] = -(pcVar1[1] == (char)((uint)iVar5 >> 8));
          auVar34[2] = -(pcVar1[2] == (char)((uint)iVar5 >> 0x10));
          auVar34[3] = -(pcVar1[3] == (char)((uint)iVar5 >> 0x18));
          auVar34[4] = -(pcVar1[4] == (char)iVar6);
          auVar34[5] = -(pcVar1[5] == (char)((uint)iVar6 >> 8));
          auVar34[6] = -(pcVar1[6] == (char)((uint)iVar6 >> 0x10));
          auVar34[7] = -(pcVar1[7] == (char)((uint)iVar6 >> 0x18));
          auVar34[8] = -(pcVar1[8] == (char)iVar7);
          auVar34[9] = -(pcVar1[9] == (char)((uint)iVar7 >> 8));
          auVar34[10] = -(pcVar1[10] == (char)((uint)iVar7 >> 0x10));
          auVar34[0xb] = -(pcVar1[0xb] == (char)((uint)iVar7 >> 0x18));
          auVar34[0xc] = -(pcVar1[0xc] == (char)iVar8);
          auVar34[0xd] = -(pcVar1[0xd] == (char)((uint)iVar8 >> 8));
          auVar34[0xe] = -(pcVar1[0xe] == (char)((uint)iVar8 >> 0x10));
          auVar34[0xf] = -(pcVar1[0xf] == (char)((uint)iVar8 >> 0x18));
          if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 || (SUB161(auVar34 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar34 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x2f,0) & 1) != 0) || (SUB161(auVar34 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x47,0) & 1) != 0) || (SUB161(auVar34 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar34 >> 0x57,0) & 1) != 0) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
          uVar14 = uVar14 + lVar11 + 0x10;
          lVar11 = lVar11 + 0x10;
        }
      }
LAB_1408123e0:
      piVar27 = piVar27 + 3;
      lVar11 = local_98;
      lVar19 = local_120;
      puVar26 = local_b8;
      uVar17 = local_10c;
    } while (piVar27 != piVar21);
  } while( true );
}


