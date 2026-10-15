// ADDR 0x142001d40
// SIG void __cdecl propose_wall_decorator_merge_position_inner(void)
// ORIG system::propose_wall_decorator_merge_position_inner


void __cdecl
system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner(void)

{
  char *pcVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  byte bVar4;
  longlong lVar5;
  undefined8 uVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  int extraout_EAX;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  float *in_RCX;
  byte bVar15;
  ulonglong uVar16;
  undefined1 uVar17;
  byte bVar18;
  undefined8 *in_R8;
  longlong lVar19;
  uint uVar20;
  ushort uVar21;
  longlong lVar22;
  byte bVar23;
  char cVar24;
  char cVar28;
  float extraout_XMM0_Da;
  char cVar26;
  char cVar27;
  undefined1 auVar25 [16];
  undefined1 in_XMM1_Ba;
  undefined1 in_XMM1_Bb;
  undefined1 in_XMM1_Bc;
  undefined1 in_XMM1_Bd;
  undefined1 in_XMM1_Be;
  undefined1 in_XMM1_Bf;
  undefined1 in_XMM1_Bg;
  undefined1 in_XMM1_Bh;
  undefined1 in_XMM1_Bi;
  undefined1 in_XMM1_Bj;
  undefined1 in_XMM1_Bk;
  undefined1 in_XMM1_Bl;
  undefined1 in_XMM1_Bm;
  undefined1 in_XMM1_Bn;
  undefined1 in_XMM1_Bo;
  undefined1 in_XMM1_Bp;
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float in_stack_00000040;
  float in_stack_00000048;
  float in_stack_00000050;
  longlong in_stack_00000058;
  float local_120;
  float local_11c;
  undefined4 local_118;
  float local_114;
  float *local_110;
  code *local_108;
  longlong local_100;
  undefined1 local_f5;
  byte local_f4;
  byte local_f3;
  undefined1 local_f2;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  if (*(longlong *)(in_stack_00000058 + 0x48) == 0) {
LAB_142002021:
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  bVar4 = *(byte *)((longlong)in_R8 + 0x46);
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  lVar5 = *(longlong *)(in_stack_00000058 + 0x30);
  bVar23 = (byte)(extraout_RAX >> 0x39);
  auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
  auVar25 = pshuflw(auVar25,auVar25,0);
  lVar19 = 0;
  iVar9 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                 CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
  iVar10 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                  CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
  iVar11 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                  CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
  iVar12 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                  CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
  uVar16 = extraout_RAX;
  do {
    uVar16 = *(ulonglong *)(in_stack_00000058 + 0x38) & uVar16;
    pcVar1 = (char *)(lVar5 + uVar16);
    cVar24 = auVar25[0];
    auVar30[0] = -(*pcVar1 == cVar24);
    cVar26 = auVar25[1];
    auVar30[1] = -(pcVar1[1] == cVar26);
    cVar27 = auVar25[2];
    auVar30[2] = -(pcVar1[2] == cVar27);
    cVar28 = auVar25[3];
    auVar30[3] = -(pcVar1[3] == cVar28);
    auVar30[4] = -(pcVar1[4] == cVar24);
    auVar30[5] = -(pcVar1[5] == cVar26);
    auVar30[6] = -(pcVar1[6] == cVar27);
    auVar30[7] = -(pcVar1[7] == cVar28);
    auVar30[8] = -(pcVar1[8] == cVar24);
    auVar30[9] = -(pcVar1[9] == cVar26);
    auVar30[10] = -(pcVar1[10] == cVar27);
    auVar30[0xb] = -(pcVar1[0xb] == cVar28);
    auVar30[0xc] = -(pcVar1[0xc] == cVar24);
    auVar30[0xd] = -(pcVar1[0xd] == cVar26);
    auVar30[0xe] = -(pcVar1[0xe] == cVar27);
    auVar30[0xf] = -(pcVar1[0xf] == cVar28);
    uVar21 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf;
    uVar20 = (uint)uVar21;
    while (uVar21 != 0) {
      uVar7 = 0;
      for (uVar8 = uVar20; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      lVar22 = (uVar7 + uVar16 & *(ulonglong *)(in_stack_00000058 + 0x38)) * -0x50;
      if (*(float *)(lVar5 + -0x50 + lVar22) == in_stack_00000050) {
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        country_core::resources::walls::decorator::DecoratorRank::new();
        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
        std::collections::hash::map::HashMap::
        get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                  ();
        if (extraout_RAX_00 == 0) {
LAB_142002393:
          local_110 = &local_120;
          local_108 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        lVar5 = lVar5 + lVar22;
        bVar23 = *(byte *)(lVar5 + -0x26);
        if (1 < bVar23) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        bVar15 = bVar4 - 0x38;
        if (7 < bVar15) {
          bVar15 = 6;
        }
        bVar18 = *(byte *)(lVar5 + -2) - 0x38;
        if (7 < bVar18) {
          bVar18 = 6;
        }
        if ((bVar15 != 6 || *(byte *)(lVar5 + -2) == bVar4) && (bVar18 == bVar15)) {
          fVar37 = *(float *)(extraout_RAX_00 + 0x3c) - *(float *)(extraout_RAX_00 + 0x38);
          fVar33 = *(float *)(lVar5 + -0x48);
          fVar31 = *(float *)(lVar5 + -0x44);
          uVar6 = *(undefined8 *)(lVar5 + -0x40);
          local_f5 = *(undefined1 *)(lVar5 + -0x28);
          local_f4 = *(byte *)(lVar5 + -0x27);
          uVar16 = (ulonglong)*(byte *)(lVar5 + -4) + (ulonglong)*(byte *)((longlong)in_R8 + 0x44);
          local_f1 = extraout_AL;
          country_core::resources::walls::decorator::DecoratorType::max_rank();
          if (uVar16 <= extraout_RAX_01) {
            fVar32 = 0.0;
            fVar34 = 1.0;
            if (bVar4 == 0x3a) {
              fVar32 = fVar34;
              fVar34 = 0.0;
            }
            bVar15 = *(byte *)((longlong)in_R8 + 0x45);
            local_f2 = *(undefined1 *)((longlong)in_R8 + 0x47);
            uVar3 = in_R8[5];
            local_f3 = bVar23;
            country_core::resources::walls::decorator::DecoratorRank::new();
            country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
            std::collections::hash::map::HashMap::
            get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                      ();
            if (extraout_RAX_02 != 0) {
              uVar2 = *(undefined4 *)(extraout_RAX_02 + 0x30);
              fVar36 = *(float *)(extraout_RAX_02 + 0x3c) - *(float *)(extraout_RAX_02 + 0x38);
              fVar35 = fVar31 - in_stack_00000048;
              if (ABS(fVar35) < fVar37 * 0.5) {
                ::utils::wall_space::impl$1::distance();
              }
              country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall();
              if ((extraout_RAX_03 == 0) ||
                 (country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
                            (), uVar17 = local_f5, extraout_AL_00 == '\0')) {
                if (fVar34 <= fVar32) {
                  _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                            ();
                  uVar17 = extraout_AL_02;
                }
                else {
                  ::utils::wall_space::impl$1::subtract();
                  fVar33 = 0.0;
                  ::utils::wall_space::impl$1::project();
                  if (extraout_EAX != 1) goto LAB_142001fb8;
                  if (local_f1 == '\t') {
                    fVar33 = (fVar36 - fVar37) * 0.5;
                  }
                  fVar31 = fVar31 + fVar33;
                  _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                            ();
                  fVar35 = fVar31 - in_stack_00000048;
                  fVar33 = extraout_XMM0_Da;
                  uVar17 = extraout_AL_01;
                }
              }
              local_110 = (float *)0x0;
              local_108 = (code *)0x4;
              local_100 = 0;
              local_120 = fVar33;
              local_11c = fVar31;
              local_118 = uVar2;
              local_114 = fVar36;
              collision_detection::
              intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env_1>
                        ();
              if (local_100 == 0) {
                *in_RCX = fVar33;
                in_RCX[1] = fVar31;
                *(undefined8 *)(in_RCX + 2) = uVar6;
                *(undefined1 *)(in_RCX + 4) = uVar17;
                *(byte *)((longlong)in_RCX + 0x11) = local_f4 & 1;
                *(byte *)((longlong)in_RCX + 0x12) = local_f3;
                uVar6 = in_R8[1];
                uVar13 = in_R8[2];
                uVar14 = in_R8[3];
                *(undefined8 *)(in_RCX + 6) = *in_R8;
                *(undefined8 *)(in_RCX + 8) = uVar6;
                *(undefined8 *)(in_RCX + 10) = uVar13;
                *(undefined8 *)(in_RCX + 0xc) = uVar14;
                *(undefined8 *)(in_RCX + 0xe) = in_R8[4];
                *(undefined8 *)(in_RCX + 0x10) = uVar3;
                uVar6 = in_R8[7];
                *(undefined8 *)(in_RCX + 0x12) = in_R8[6];
                *(undefined8 *)(in_RCX + 0x14) = uVar6;
                in_RCX[0x16] = *(float *)(in_R8 + 8);
                *(char *)(in_RCX + 0x17) = (char)uVar16;
                *(ushort *)((longlong)in_RCX + 0x5d) = (ushort)bVar4 * 0x100 + (ushort)bVar15;
                *(undefined1 *)((longlong)in_RCX + 0x5f) = local_f2;
                in_RCX[0x18] = in_stack_00000050;
                in_RCX[0x19] = SQRT(fVar35 * fVar35 +
                                    (fVar33 - in_stack_00000040) * (fVar33 - in_stack_00000040));
              }
              else {
                *(undefined1 *)((longlong)in_RCX + 0x5f) = 2;
              }
              if (local_110 == (float *)0x0) {
                return;
              }
              std::alloc::__default_lib_allocator::__rust_dealloc();
              return;
            }
            goto LAB_142002393;
          }
        }
LAB_142001fb8:
        *(undefined1 *)((longlong)in_RCX + 0x5f) = 2;
        return;
      }
      uVar21 = (ushort)(uVar20 - 1) & (ushort)uVar20;
      uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar21);
    }
    auVar29[0] = -(*pcVar1 == (char)iVar9);
    auVar29[1] = -(pcVar1[1] == (char)((uint)iVar9 >> 8));
    auVar29[2] = -(pcVar1[2] == (char)((uint)iVar9 >> 0x10));
    auVar29[3] = -(pcVar1[3] == (char)((uint)iVar9 >> 0x18));
    auVar29[4] = -(pcVar1[4] == (char)iVar10);
    auVar29[5] = -(pcVar1[5] == (char)((uint)iVar10 >> 8));
    auVar29[6] = -(pcVar1[6] == (char)((uint)iVar10 >> 0x10));
    auVar29[7] = -(pcVar1[7] == (char)((uint)iVar10 >> 0x18));
    auVar29[8] = -(pcVar1[8] == (char)iVar11);
    auVar29[9] = -(pcVar1[9] == (char)((uint)iVar11 >> 8));
    auVar29[10] = -(pcVar1[10] == (char)((uint)iVar11 >> 0x10));
    auVar29[0xb] = -(pcVar1[0xb] == (char)((uint)iVar11 >> 0x18));
    auVar29[0xc] = -(pcVar1[0xc] == (char)iVar12);
    auVar29[0xd] = -(pcVar1[0xd] == (char)((uint)iVar12 >> 8));
    auVar29[0xe] = -(pcVar1[0xe] == (char)((uint)iVar12 >> 0x10));
    auVar29[0xf] = -(pcVar1[0xf] == (char)((uint)iVar12 >> 0x18));
    if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x17,0) & 1) != 0) || (SUB161(auVar29 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x27,0) & 1) != 0) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x37,0) & 1) != 0) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') goto LAB_142002021;
    uVar16 = uVar16 + lVar19 + 0x10;
    lVar19 = lVar19 + 0x10;
  } while( true );
}


