// 0x140ee4bf0 flush_candidates_pending_removal :: void __cdecl flush_candidates_pending_removal(void)

void __cdecl system_clutter::autoclutter::AutoClutterState::flush_candidates_pending_removal(void)

{
  void *_Src;
  char *pcVar1;
  ushort uVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  ulonglong extraout_RAX;
  longlong in_RCX;
  ushort *puVar9;
  longlong lVar10;
  ushort *puVar11;
  ulonglong uVar12;
  longlong lVar13;
  ushort *puVar14;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  byte bVar20;
  char cVar21;
  char cVar23;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
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
  undefined8 local_110;
  ulonglong local_108;
  ulonglong local_100;
  ulonglong local_f8;
  undefined8 *local_f0;
  longlong *local_e8;
  
  uVar19 = *(ulonglong *)(in_RCX + 0x40);
  if (uVar19 != 0) {
    puVar11 = *(ushort **)(in_RCX + 0x38);
    local_f0 = &local_110;
    if (uVar19 != 1) {
      if (uVar19 < 0x15) {
        puVar9 = puVar11 + 1;
        lVar10 = 2;
        do {
          uVar17 = puVar9[-1];
          uVar2 = *puVar9;
          lVar15 = lVar10;
          if (uVar17 < uVar2) {
            do {
              *(ushort *)((longlong)puVar11 + lVar15) = uVar17;
              puVar14 = puVar11;
              if (lVar15 == 2) goto LAB_140ee4e63;
              uVar17 = *(ushort *)((longlong)puVar11 + lVar15 + -4);
              lVar15 = lVar15 + -2;
            } while (uVar17 < uVar2);
            puVar14 = (ushort *)(lVar15 + (longlong)puVar11);
LAB_140ee4e63:
            *puVar14 = uVar2;
          }
          puVar9 = puVar9 + 1;
          lVar10 = lVar10 + 2;
        } while (puVar9 != puVar11 + uVar19);
      }
      else {
        core::slice::sort::stable::
        driftsort_main<u16,alloc::slice::impl_0::sort_by::closure_env_0<u16,system_clutter::autoclutter::impl_1::flush_candidates_pending_removal::closure_env_0>,alloc::vec::Vec<u16,alloc::alloc::Global>_>
                  ();
      }
    }
    puVar9 = puVar11 + uVar19;
    iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    do {
      uVar2 = *puVar11;
      uVar12 = (ulonglong)uVar2;
      uVar19 = *(ulonglong *)(in_RCX + 0x28);
      if (uVar19 <= uVar12) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::swap_remove::assert_failed();
      }
      lVar10 = *(longlong *)(in_RCX + 0x20);
      _Src = (void *)(lVar10 + uVar12 * 0x98);
      memcpy(&local_f0,_Src,0x98);
      uVar19 = uVar19 - 1;
      memmove(_Src,(void *)(uVar19 * 0x98 + lVar10),0x98);
      *(ulonglong *)(in_RCX + 0x28) = uVar19;
      if ((int)local_f0 == 4) {
        LOCK();
        *local_e8 = *local_e8 + -1;
        UNLOCK();
        if (*local_e8 == 0) {
          alloc::sync::Arc::drop_slow<slice2_<glam::f32::vec3::Vec3>,alloc::alloc::Global>();
        }
      }
      if (uVar19 != uVar12) {
        if (uVar19 <= uVar12) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar10 = *(longlong *)(*(longlong *)(in_RCX + 0x20) + 0x88 + uVar12 * 0x98);
        if (*(longlong *)(in_RCX + 0x78) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar15 = *(longlong *)(in_RCX + 0x60);
          bVar20 = (byte)(extraout_RAX >> 0x39);
          auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
          auVar22 = pshuflw(auVar22,auVar22,0);
          lVar13 = 0;
          uVar12 = extraout_RAX;
          while( true ) {
            uVar12 = *(ulonglong *)(in_RCX + 0x68) & uVar12;
            pcVar1 = (char *)(lVar15 + uVar12);
            cVar21 = auVar22[0];
            auVar27[0] = -(*pcVar1 == cVar21);
            cVar23 = auVar22[1];
            auVar27[1] = -(pcVar1[1] == cVar23);
            cVar24 = auVar22[2];
            auVar27[2] = -(pcVar1[2] == cVar24);
            cVar25 = auVar22[3];
            auVar27[3] = -(pcVar1[3] == cVar25);
            auVar27[4] = -(pcVar1[4] == cVar21);
            auVar27[5] = -(pcVar1[5] == cVar23);
            auVar27[6] = -(pcVar1[6] == cVar24);
            auVar27[7] = -(pcVar1[7] == cVar25);
            auVar27[8] = -(pcVar1[8] == cVar21);
            auVar27[9] = -(pcVar1[9] == cVar23);
            auVar27[10] = -(pcVar1[10] == cVar24);
            auVar27[0xb] = -(pcVar1[0xb] == cVar25);
            auVar27[0xc] = -(pcVar1[0xc] == cVar21);
            auVar27[0xd] = -(pcVar1[0xd] == cVar23);
            auVar27[0xe] = -(pcVar1[0xe] == cVar24);
            auVar27[0xf] = -(pcVar1[0xf] == cVar25);
            uVar17 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar27[0xf] >> 7) << 0xf;
            uVar16 = (uint)uVar17;
            while (uVar17 != 0) {
              uVar3 = 0;
              for (uVar4 = uVar16; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                uVar3 = uVar3 + 1;
              }
              uVar18 = uVar3 + uVar12 & *(ulonglong *)(in_RCX + 0x68);
              if (lVar10 == *(longlong *)(lVar15 + -0x48 + uVar18 * -0x48)) {
                local_f8 = (ulonglong)*(ushort *)(lVar15 + -6 + uVar18 * -0x48);
                local_110 = 1;
                local_100 = (ulonglong)(local_f8 < 0xfffe);
                local_108 = uVar19;
                if ((local_f8 >= 0xfffe) || (uVar19 != local_f8)) {
                  local_f0 = (undefined8 *)0x0;
                    // WARNING: Subroutine does not return
                  core::panicking::
                  assert_failed<enum2_<core::option::Option<usize>_>,enum2_<core::option::Option<usize>_>_>
                            ();
                }
                *(ushort *)(lVar15 + uVar18 * -0x48 + -6) = uVar2;
                goto LAB_140ee4c50;
              }
              uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
              uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
            }
            auVar26[0] = -(*pcVar1 == (char)iVar5);
            auVar26[1] = -(pcVar1[1] == (char)((uint)iVar5 >> 8));
            auVar26[2] = -(pcVar1[2] == (char)((uint)iVar5 >> 0x10));
            auVar26[3] = -(pcVar1[3] == (char)((uint)iVar5 >> 0x18));
            auVar26[4] = -(pcVar1[4] == (char)iVar6);
            auVar26[5] = -(pcVar1[5] == (char)((uint)iVar6 >> 8));
            auVar26[6] = -(pcVar1[6] == (char)((uint)iVar6 >> 0x10));
            auVar26[7] = -(pcVar1[7] == (char)((uint)iVar6 >> 0x18));
            auVar26[8] = -(pcVar1[8] == (char)iVar7);
            auVar26[9] = -(pcVar1[9] == (char)((uint)iVar7 >> 8));
            auVar26[10] = -(pcVar1[10] == (char)((uint)iVar7 >> 0x10));
            auVar26[0xb] = -(pcVar1[0xb] == (char)((uint)iVar7 >> 0x18));
            auVar26[0xc] = -(pcVar1[0xc] == (char)iVar8);
            auVar26[0xd] = -(pcVar1[0xd] == (char)((uint)iVar8 >> 8));
            auVar26[0xe] = -(pcVar1[0xe] == (char)((uint)iVar8 >> 0x10));
            auVar26[0xf] = -(pcVar1[0xf] == (char)((uint)iVar8 >> 0x18));
            if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar26 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar26 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') break;
            uVar12 = uVar12 + lVar13 + 0x10;
            lVar13 = lVar13 + 0x10;
          }
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
LAB_140ee4c50:
      puVar11 = puVar11 + 1;
    } while (puVar11 != puVar9);
    *(undefined8 *)(in_RCX + 0x40) = 0;
  }
  return;
}


