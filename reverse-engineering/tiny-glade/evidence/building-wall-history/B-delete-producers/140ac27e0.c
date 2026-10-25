
void __cdecl country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze(void)

{
  char *pcVar1;
  undefined4 uVar2;
  longlong *plVar3;
  undefined4 *puVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  longlong lVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  float fVar15;
  float fVar16;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar17;
  longlong *in_R8;
  ulonglong uVar18;
  longlong *in_R9;
  longlong lVar19;
  uint uVar20;
  ushort uVar21;
  longlong lVar22;
  char cVar23;
  byte bVar24;
  char cVar27;
  char cVar28;
  char cVar29;
  float fVar25;
  float extraout_XMM0_Da;
  undefined1 auVar26 [16];
  float fVar30;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
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
  float local_14c;
  undefined4 local_148;
  undefined4 local_144;
  longlong local_140;
  longlong local_138;
  float *local_130;
  code *local_128;
  undefined **local_120;
  undefined8 local_118;
  float **local_110;
  longlong local_108;
  undefined8 local_100;
  undefined4 local_e8;
  undefined8 local_cc;
  undefined8 uStack_c4;
  undefined4 local_bc;
  undefined1 local_b8;
  undefined1 local_b7;
  undefined4 *local_b0;
  longlong local_a8;
  undefined4 *local_a0;
  longlong *local_98;
  longlong local_90;
  
  local_148 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_b0 = (undefined4 *)in_RCX[2];
  *local_b0 = local_148;
  fVar16 = __real_3e4ccccd;
  fVar15 = __real_3f800000;
  local_140 = *in_RCX;
  if (*(longlong *)(local_140 + 0x10) != 0) {
    lVar17 = *(longlong *)(local_140 + 8);
    local_90 = *(longlong *)(local_140 + 0x10) * 0x98 + lVar17;
    plVar3 = (longlong *)*in_RDX;
    puVar4 = (undefined4 *)in_RDX[2];
    lVar5 = *in_R9;
    uVar2 = *(undefined4 *)((longlong)in_RDX + 0x1c);
    local_98 = plVar3 + 4;
    local_144 = *(undefined4 *)((longlong)in_R8 + 0x1c);
    local_138 = *in_R8;
    local_a0 = (undefined4 *)in_R8[2];
    local_a8 = local_138 + 0x20;
    iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                    CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                    CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                    CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    lVar22 = local_90;
LAB_140ac2b70:
    do {
      lVar8 = *(longlong *)(lVar17 + 0x70);
      local_120 = (undefined **)lVar8;
      if (plVar3[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar24 = (byte)(extraout_RAX >> 0x39);
        auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
        auVar26 = pshuflw(auVar26,auVar26,0);
        lVar19 = 0;
        uVar18 = extraout_RAX;
        while( true ) {
          uVar18 = plVar3[1] & uVar18;
          pcVar1 = (char *)(*plVar3 + uVar18);
          cVar23 = auVar26[0];
          auVar32[0] = -(*pcVar1 == cVar23);
          cVar27 = auVar26[1];
          auVar32[1] = -(pcVar1[1] == cVar27);
          cVar28 = auVar26[2];
          auVar32[2] = -(pcVar1[2] == cVar28);
          cVar29 = auVar26[3];
          auVar32[3] = -(pcVar1[3] == cVar29);
          auVar32[4] = -(pcVar1[4] == cVar23);
          auVar32[5] = -(pcVar1[5] == cVar27);
          auVar32[6] = -(pcVar1[6] == cVar28);
          auVar32[7] = -(pcVar1[7] == cVar29);
          auVar32[8] = -(pcVar1[8] == cVar23);
          auVar32[9] = -(pcVar1[9] == cVar27);
          auVar32[10] = -(pcVar1[10] == cVar28);
          auVar32[0xb] = -(pcVar1[0xb] == cVar29);
          auVar32[0xc] = -(pcVar1[0xc] == cVar23);
          auVar32[0xd] = -(pcVar1[0xd] == cVar27);
          auVar32[0xe] = -(pcVar1[0xe] == cVar28);
          auVar32[0xf] = -(pcVar1[0xf] == cVar29);
          uVar21 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar32[0xf] >> 7) << 0xf;
          uVar20 = (uint)uVar21;
          while (uVar21 != 0) {
            uVar9 = 0;
            for (uVar10 = uVar20; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar9 = uVar9 + 1;
            }
            if (*(longlong *)(*plVar3 + -0x78 + (uVar9 + uVar18 & plVar3[1]) * -0x78) == lVar8) {
              fVar25 = *(float *)(lVar5 + 0x58) / fVar16 + *(float *)(lVar17 + 0x8c);
              fVar30 = 0.0;
              if (0.0 <= fVar25) {
                fVar30 = fVar25;
              }
              fVar25 = fVar15;
              if (fVar30 <= fVar15) {
                fVar25 = fVar30;
              }
              *(float *)(lVar17 + 0x8c) = fVar25;
              ::utils::easing_functions::cubic_in_out();
              if ((*(float *)(lVar17 + 0x8c) == fVar15) &&
                 (!NAN(*(float *)(lVar17 + 0x8c)) && !NAN(fVar15))) {
                *puVar4 = uVar2;
                resources::walls::inner_walls::InnerWalls::remove();
                if (!SBORROW8(0,(longlong)local_120)) {
                  if (local_120 != (undefined **)0x0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                  if (local_108 != 0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                }
                goto LAB_140ac2bc0;
              }
              local_14c = *(float *)(lVar17 + 0x78) * extraout_XMM0_Da +
                          (fVar15 - extraout_XMM0_Da) * 0.0;
              if ((local_14c < 0.0) || (fVar15 < local_14c)) {
                local_130 = &local_14c;
                local_128 = core::fmt::float::impl$5::fmt;
                local_120 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
                local_118 = 1;
                local_100 = 0;
                local_110 = &local_130;
                local_108 = 1;
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_fmt();
              }
              if (NAN(local_14c)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              local_14c = (fVar15 - extraout_XMM0_Da) + *(float *)(lVar17 + 0x7c) * extraout_XMM0_Da
              ;
              if ((local_14c < 0.0) || (fVar15 < local_14c)) {
                local_130 = &local_14c;
                local_128 = core::fmt::float::impl$5::fmt;
                local_120 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
                local_118 = 1;
                local_100 = 0;
                local_110 = &local_130;
                local_108 = 1;
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_fmt();
              }
              if (NAN(local_14c)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              ::utils::curve::Curve::
              crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
              local_b8 = *(char *)(lVar17 + 0x68) == '\x01';
              local_b7 = *(undefined1 *)(lVar17 + 0x69);
              local_e8 = 2;
              local_cc = *(undefined8 *)(lVar17 + 0x54);
              uStack_c4 = *(undefined8 *)(lVar17 + 0x5c);
              local_bc = *(undefined4 *)(lVar17 + 100);
              *puVar4 = uVar2;
              resources::walls::inner_walls::InnerWalls::
              _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state::closure_env_0>
                        ();
              uVar6 = *(undefined8 *)(lVar17 + 0x70);
              local_120 = *(undefined ***)(lVar17 + 0x80);
              local_118 = CONCAT44(local_118._4_4_,*(undefined4 *)(lVar17 + 0x88));
              *local_a0 = local_144;
              lVar22 = *(longlong *)(local_138 + 0x30);
              uVar7 = *(undefined8 *)(local_138 + 0x40);
              if (lVar22 == *(longlong *)(local_138 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                          ();
              }
              lVar8 = *(longlong *)(local_138 + 0x28);
              lVar19 = lVar22 * 0x20;
              *(undefined8 *)(lVar8 + lVar19) = uVar7;
              *(undefined8 *)(lVar8 + 8 + lVar19) = uVar6;
              *(undefined ***)(lVar8 + 0x10 + lVar19) = local_120;
              *(undefined4 *)(lVar8 + 0x18 + lVar19) = (undefined4)local_118;
              *(undefined2 *)(lVar8 + 0x1c + lVar19) = 0;
              *(longlong *)(local_138 + 0x30) = lVar22 + 1;
              *(longlong *)(local_138 + 0x40) = *(longlong *)(local_138 + 0x40) + 1;
              lVar17 = lVar17 + 0x98;
              lVar22 = local_90;
              if (lVar17 != local_90) goto LAB_140ac2b70;
              goto LAB_140ac2bd3;
            }
            uVar21 = (ushort)(uVar20 - 1) & (ushort)uVar20;
            uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar21);
          }
          auVar31[0] = -(*pcVar1 == (char)iVar11);
          auVar31[1] = -(pcVar1[1] == (char)((uint)iVar11 >> 8));
          auVar31[2] = -(pcVar1[2] == (char)((uint)iVar11 >> 0x10));
          auVar31[3] = -(pcVar1[3] == (char)((uint)iVar11 >> 0x18));
          auVar31[4] = -(pcVar1[4] == (char)iVar12);
          auVar31[5] = -(pcVar1[5] == (char)((uint)iVar12 >> 8));
          auVar31[6] = -(pcVar1[6] == (char)((uint)iVar12 >> 0x10));
          auVar31[7] = -(pcVar1[7] == (char)((uint)iVar12 >> 0x18));
          auVar31[8] = -(pcVar1[8] == (char)iVar13);
          auVar31[9] = -(pcVar1[9] == (char)((uint)iVar13 >> 8));
          auVar31[10] = -(pcVar1[10] == (char)((uint)iVar13 >> 0x10));
          auVar31[0xb] = -(pcVar1[0xb] == (char)((uint)iVar13 >> 0x18));
          auVar31[0xc] = -(pcVar1[0xc] == (char)iVar14);
          auVar31[0xd] = -(pcVar1[0xd] == (char)((uint)iVar14 >> 8));
          auVar31[0xe] = -(pcVar1[0xe] == (char)((uint)iVar14 >> 0x10));
          auVar31[0xf] = -(pcVar1[0xf] == (char)((uint)iVar14 >> 0x18));
          if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar31 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar31 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar31 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar31 >> 0x2f,0) & 1) != 0) || (SUB161(auVar31 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') break;
          uVar18 = uVar18 + lVar19 + 0x10;
          lVar19 = lVar19 + 0x10;
        }
      }
LAB_140ac2bc0:
      *(undefined1 *)(lVar17 + 0x90) = 1;
      lVar17 = lVar17 + 0x98;
    } while (lVar17 != lVar22);
  }
LAB_140ac2bd3:
  *local_b0 = local_148;
  alloc::vec::Vec::
  retain<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeCmd,alloc::alloc::Global,country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze::closure_env_0>
            ();
  return;
}

