// ADDR 0x14081bac0
// SIG void __cdecl decorator_on_freehand_wall_edited(void)
// ORIG system::decorator_on_freehand_wall_edited


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited(void)

{
  longlong lVar1;
  int *piVar2;
  undefined4 uVar3;
  int iVar4;
  ulonglong *puVar5;
  longlong lVar6;
  undefined4 *puVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  longlong lVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined8 *in_RDX;
  longlong lVar19;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  int *piVar23;
  longlong in_R8;
  ulonglong uVar24;
  uint uVar25;
  ushort uVar26;
  longlong lVar27;
  char cVar28;
  byte bVar29;
  char cVar31;
  char cVar32;
  char cVar33;
  undefined4 extraout_XMM0_Da;
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
  longlong local_e8;
  longlong lStack_e0;
  ulonglong local_d8;
  ulonglong uStack_d0;
  
  puVar5 = (ulonglong *)*in_RDX;
  lVar6 = in_RDX[1];
  uVar17 = *puVar5;
  uVar20 = 0;
  uVar18 = uVar17 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar18 = uVar20;
  }
  uVar24 = uVar17 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar24 = uVar20;
  }
  lVar16 = uVar18 * 0x28 + *(longlong *)(lVar6 + 8);
  uVar17 = *(ulonglong *)(lVar6 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar18) {
    lVar16 = 8;
    uVar17 = uVar20;
  }
  lVar22 = uVar24 * 0x28 + *(longlong *)(lVar6 + 0x28);
  uVar18 = *(ulonglong *)(lVar6 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar24) {
    lVar22 = 8;
    uVar18 = uVar20;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar18) - uVar17;
  auVar15 = ___xmm_7465646279746573646f72616e646f6d;
  auVar14 = ___xmm_6c7967656e657261736f6d6570736575;
  lVar6 = lVar16 + uVar17 * 0x28;
  lVar1 = lVar22 + uVar18 * 0x28;
  uVar3 = *(undefined4 *)(in_R8 + 0x1c);
  puVar7 = *(undefined4 **)(in_R8 + 0x10);
  iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                            CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                  CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                            CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                  CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  do {
    do {
      if ((lVar16 == 0) || (lVar16 == lVar6)) {
        if ((lVar22 == 0) || (lVar22 == lVar1)) {
          return;
        }
        lVar16 = 0;
        lVar27 = lVar22;
        lVar22 = lVar22 + 0x28;
      }
      else {
        lVar27 = lVar16;
        lVar16 = lVar16 + 0x28;
      }
      *puVar5 = *puVar5 + 1;
      *puVar7 = uVar3;
      country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    } while ((extraout_RAX == 0) || (*(longlong *)(lVar27 + 0x10) == 0));
    piVar23 = *(int **)(lVar27 + 8);
    piVar2 = piVar23 + *(longlong *)(lVar27 + 0x10) * 3;
    do {
      iVar4 = *piVar23;
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (*(longlong *)(extraout_RAX + 0x48) != 0) {
        auVar30 = *(undefined1 (*) [16])(extraout_RAX + 0x50);
        auVar35._0_8_ = auVar30._0_8_;
        auVar35._8_4_ = auVar30._0_4_;
        auVar35._12_4_ = auVar30._4_4_;
        auVar35 = auVar35 ^ auVar14;
        auVar36._0_8_ = auVar30._8_8_;
        auVar36._8_4_ = auVar30._8_4_;
        auVar36._12_4_ = auVar30._12_4_;
        core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
        local_d8 = SUB168(auVar36 ^ auVar15,0);
        uStack_d0 = SUB168(auVar36 ^ auVar15,8);
        local_e8 = auVar35._0_8_;
        uVar24 = local_e8 + local_d8;
        lStack_e0 = auVar35._8_8_;
        uVar18 = (local_d8 << 0xd | local_d8 >> 0x33) ^ uVar24;
        uVar17 = (uStack_d0 << 0x10 | uStack_d0 >> 0x30) ^ lStack_e0 + uStack_d0;
        uVar20 = lStack_e0 + uStack_d0 + uVar18;
        uVar24 = (uVar24 << 0x20 | uVar24 >> 0x20) + uVar17;
        uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar20;
        uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar24;
        uVar24 = uVar24 + uVar18;
        uVar20 = ((uVar20 << 0x20 | uVar20 >> 0x20) ^ 0xff) + uVar17;
        uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar24;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
        uVar20 = uVar20 + uVar18;
        uVar24 = (uVar24 << 0x20 | uVar24 >> 0x20) + uVar17;
        uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar20;
        uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar24;
        uVar24 = uVar24 + uVar18;
        uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar17;
        uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar24;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
        uVar20 = uVar20 + uVar18;
        uVar24 = (uVar24 << 0x20 | uVar24 >> 0x20) + uVar17;
        uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar20;
        uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar24;
        uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar17;
        uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar24 + uVar18;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
        uVar20 = uVar20 + uVar18;
        uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) ^ (uVar18 << 0x11 | uVar18 >> 0x2f) ^
                 (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar20;
        lVar27 = *(longlong *)(extraout_RAX + 0x30);
        bVar29 = (byte)(uVar20 >> 0x39);
        auVar30 = ZEXT216(CONCAT11(bVar29,bVar29));
        auVar30 = pshuflw(auVar30,auVar30,0);
        lVar19 = 0;
        while( true ) {
          uVar20 = uVar20 & *(ulonglong *)(extraout_RAX + 0x38);
          auVar36 = *(undefined1 (*) [16])(lVar27 + uVar20);
          cVar28 = auVar30[0];
          auVar37[0] = -(auVar36[0] == cVar28);
          cVar31 = auVar30[1];
          auVar37[1] = -(auVar36[1] == cVar31);
          cVar32 = auVar30[2];
          auVar37[2] = -(auVar36[2] == cVar32);
          cVar33 = auVar30[3];
          auVar37[3] = -(auVar36[3] == cVar33);
          auVar37[4] = -(auVar36[4] == cVar28);
          auVar37[5] = -(auVar36[5] == cVar31);
          auVar37[6] = -(auVar36[6] == cVar32);
          auVar37[7] = -(auVar36[7] == cVar33);
          auVar37[8] = -(auVar36[8] == cVar28);
          auVar37[9] = -(auVar36[9] == cVar31);
          auVar37[10] = -(auVar36[10] == cVar32);
          auVar37[0xb] = -(auVar36[0xb] == cVar33);
          auVar37[0xc] = -(auVar36[0xc] == cVar28);
          auVar37[0xd] = -(auVar36[0xd] == cVar31);
          auVar37[0xe] = -(auVar36[0xe] == cVar32);
          auVar37[0xf] = -(auVar36[0xf] == cVar33);
          uVar26 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
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
          uVar25 = (uint)uVar26;
          while (uVar26 != 0) {
            uVar8 = 0;
            for (uVar9 = uVar25; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
              uVar8 = uVar8 + 1;
            }
            lVar21 = (uVar8 + uVar20 & *(ulonglong *)(extraout_RAX + 0x38)) * -0x50;
            if (iVar4 == *(int *)(lVar27 + -0x50 + lVar21)) {
              bVar29 = *(byte *)(lVar27 + -0x26 + lVar21);
              lVar19 = (ulonglong)bVar29 - 1;
              if (2 < (byte)(bVar29 - 2)) {
                lVar19 = 0;
              }
              if (lVar19 == 0) {
                country_core::resources::walls::public_walls::PublicWalls::get();
                if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                if (*(ulonglong *)(extraout_RAX_00 + 0x10) <= (ulonglong)(uint)piVar23[1]) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(extraout_RAX_00 + 0x10) <= (ulonglong)(uint)piVar23[1] + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                lVar27 = lVar27 + lVar21;
                ::utils::swizzlers::impl$1::xvy();
                lStack_e0._0_4_ = auVar35._8_4_;
                *(undefined4 *)(lVar27 + -0x2c) = (undefined4)lStack_e0;
                *(longlong *)(lVar27 + -0x34) = local_e8;
                auVar30 = ZEXT416((uint)piVar23[2]);
                ::utils::wall_space::WallSpaceCoord::from_curve_internal_coord();
                *(undefined4 *)(lVar27 + -0x48) = extraout_XMM0_Da;
                *(int *)(lVar27 + -0x44) = auVar30._0_4_;
              }
              else if (lVar19 != 1) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              goto LAB_14081bfb0;
            }
            uVar26 = (ushort)(uVar25 - 1) & (ushort)uVar25;
            uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar26);
          }
          auVar34[0] = -(auVar36[0] == (char)iVar10);
          auVar34[1] = -(auVar36[1] == (char)((uint)iVar10 >> 8));
          auVar34[2] = -(auVar36[2] == (char)((uint)iVar10 >> 0x10));
          auVar34[3] = -(auVar36[3] == (char)((uint)iVar10 >> 0x18));
          auVar34[4] = -(auVar36[4] == (char)iVar11);
          auVar34[5] = -(auVar36[5] == (char)((uint)iVar11 >> 8));
          auVar34[6] = -(auVar36[6] == (char)((uint)iVar11 >> 0x10));
          auVar34[7] = -(auVar36[7] == (char)((uint)iVar11 >> 0x18));
          auVar34[8] = -(auVar36[8] == (char)iVar12);
          auVar34[9] = -(auVar36[9] == (char)((uint)iVar12 >> 8));
          auVar34[10] = -(auVar36[10] == (char)((uint)iVar12 >> 0x10));
          auVar34[0xb] = -(auVar36[0xb] == (char)((uint)iVar12 >> 0x18));
          auVar34[0xc] = -(auVar36[0xc] == (char)iVar13);
          auVar34[0xd] = -(auVar36[0xd] == (char)((uint)iVar13 >> 8));
          auVar34[0xe] = -(auVar36[0xe] == (char)((uint)iVar13 >> 0x10));
          auVar34[0xf] = -(auVar36[0xf] == (char)((uint)iVar13 >> 0x18));
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
          uVar20 = uVar20 + lVar19 + 0x10;
          lVar19 = lVar19 + 0x10;
        }
      }
LAB_14081bfb0:
      piVar23 = piVar23 + 3;
    } while (piVar23 != piVar2);
  } while( true );
}


