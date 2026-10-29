
void __cdecl country_core::resources::roofs::roof_shape::roof_uv_at_ws_pos(void)

{
  longlong in_RCX;
  undefined1 local_20;
  undefined1 local_1f;
  undefined8 local_1c;
  
  roof_uv_and_point_at_ws_pos();
  if (local_20 == '\0') {
    *(undefined8 *)(in_RCX + 4) = local_1c;
  }
  else {
    *(undefined1 *)(in_RCX + 1) = local_1f;
  }
  *(bool *)in_RCX = local_20 != '\0';
  return;
}



void __cdecl country_core::resources::roofs::roof_shape::roof_uv_and_point_at_ws_pos(void)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong *extraout_RAX_01;
  undefined2 *in_RCX;
  longlong in_RDX;
  undefined1 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  uint uVar12;
  ushort uVar13;
  ulonglong uVar14;
  byte bVar15;
  char cVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar18 [16];
  undefined1 in_XMM1_Ba;
  undefined1 in_XMM1_Bb;
  undefined1 in_XMM1_Bc;
  undefined1 in_XMM1_Bd;
  undefined1 in_XMM1_Be;
  char cVar25;
  undefined1 in_XMM1_Bf;
  char cVar26;
  undefined1 in_XMM1_Bg;
  char cVar27;
  undefined1 in_XMM1_Bh;
  char cVar28;
  undefined1 in_XMM1_Bi;
  char cVar29;
  undefined1 in_XMM1_Bj;
  char cVar30;
  undefined1 in_XMM1_Bk;
  char cVar31;
  undefined1 in_XMM1_Bl;
  char cVar32;
  undefined1 in_XMM1_Bm;
  char cVar33;
  undefined1 in_XMM1_Bn;
  char cVar34;
  undefined1 in_XMM1_Bo;
  char cVar35;
  undefined1 in_XMM1_Bp;
  char cVar36;
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 in_stack_fffffffffffffe98;
  undefined4 uVar41;
  undefined8 local_150;
  undefined4 local_148;
  undefined8 local_144;
  undefined4 local_13c;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined4 local_120;
  undefined8 local_110;
  undefined4 local_108;
  undefined8 local_104;
  undefined4 local_fc;
  longlong local_f8;
  undefined7 uStack_e0;
  undefined4 uStack_d9;
  undefined1 local_b8;
  undefined7 uStack_b7;
  undefined8 local_a7;
  undefined1 uStack_a0;
  undefined8 uStack_9f;
  undefined8 local_97;
  uint local_84;
  undefined8 local_80;
  undefined8 local_78;
  undefined4 local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  int local_50 [6];
  
  uVar41 = (undefined4)((ulonglong)in_stack_fffffffffffffe98 >> 0x20);
  local_b8 = (undefined1)in_RDX;
  uStack_b7 = (undefined7)((ulonglong)in_RDX >> 8);
  if (*(longlong *)(in_stack_00000030 + 0x48) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar2 = *(longlong *)(in_stack_00000030 + 0x30);
    bVar15 = (byte)(extraout_RAX >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar11 = 0;
    iVar5 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    cVar17 = (char)iVar5;
    cVar20 = (char)((uint)iVar5 >> 8);
    cVar22 = (char)((uint)iVar5 >> 0x10);
    cVar24 = (char)((uint)iVar5 >> 0x18);
    iVar5 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    cVar25 = (char)iVar5;
    cVar26 = (char)((uint)iVar5 >> 8);
    cVar27 = (char)((uint)iVar5 >> 0x10);
    cVar28 = (char)((uint)iVar5 >> 0x18);
    iVar5 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    cVar29 = (char)iVar5;
    cVar30 = (char)((uint)iVar5 >> 8);
    cVar31 = (char)((uint)iVar5 >> 0x10);
    cVar32 = (char)((uint)iVar5 >> 0x18);
    iVar5 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    cVar33 = (char)iVar5;
    cVar34 = (char)((uint)iVar5 >> 8);
    cVar35 = (char)((uint)iVar5 >> 0x10);
    cVar36 = (char)((uint)iVar5 >> 0x18);
    uVar10 = extraout_RAX;
    while( true ) {
      uVar10 = *(ulonglong *)(in_stack_00000030 + 0x38) & uVar10;
      pcVar1 = (char *)(lVar2 + uVar10);
      cVar16 = auVar18[0];
      auVar39[0] = -(*pcVar1 == cVar16);
      cVar19 = auVar18[1];
      auVar39[1] = -(pcVar1[1] == cVar19);
      cVar21 = auVar18[2];
      auVar39[2] = -(pcVar1[2] == cVar21);
      cVar23 = auVar18[3];
      auVar39[3] = -(pcVar1[3] == cVar23);
      auVar39[4] = -(pcVar1[4] == cVar16);
      auVar39[5] = -(pcVar1[5] == cVar19);
      auVar39[6] = -(pcVar1[6] == cVar21);
      auVar39[7] = -(pcVar1[7] == cVar23);
      auVar39[8] = -(pcVar1[8] == cVar16);
      auVar39[9] = -(pcVar1[9] == cVar19);
      auVar39[10] = -(pcVar1[10] == cVar21);
      auVar39[0xb] = -(pcVar1[0xb] == cVar23);
      auVar39[0xc] = -(pcVar1[0xc] == cVar16);
      auVar39[0xd] = -(pcVar1[0xd] == cVar19);
      auVar39[0xe] = -(pcVar1[0xe] == cVar21);
      auVar39[0xf] = -(pcVar1[0xf] == cVar23);
      uVar13 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar13;
      while (uVar13 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar12; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar14 = uVar3 + uVar10 & *(ulonglong *)(in_stack_00000030 + 0x38);
        if (*(longlong *)(lVar2 + -0x10 + uVar14 * -0x10) == in_RDX) {
          uVar10 = *(ulonglong *)(lVar2 + -8 + uVar14 * -0x10);
          uVar14 = 0;
          if (0x100000000 < uVar10) {
            uVar14 = uVar10;
          }
          uVar9 = (undefined1)uVar14;
          if (uVar10 >> 0x20 == 0) goto LAB_1408e42ae;
          local_f8 = CONCAT44((int)(uVar10 >> 0x20),
                              CONCAT13((char)(uVar10 >> 0x18),CONCAT21((short)(uVar10 >> 8),uVar9)))
          ;
          uVar9 = 1;
          if (*(longlong *)(in_stack_00000028 + 0x3e10) == 0) goto LAB_1408e42ae;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar2 = *(longlong *)(in_stack_00000028 + 0x3df8);
          bVar15 = (byte)(extraout_RAX_00 >> 0x39);
          auVar18 = ZEXT216(CONCAT11(bVar15,bVar15));
          auVar18 = pshuflw(auVar18,auVar18,0);
          lVar11 = 0;
          iVar5 = -(uint)(CONCAT13(cVar24,CONCAT12(cVar22,CONCAT11(cVar20,cVar17))) ==
                         CONCAT13(cVar24,CONCAT12(cVar22,CONCAT11(cVar20,cVar17))));
          iVar6 = -(uint)(CONCAT13(cVar28,CONCAT12(cVar27,CONCAT11(cVar26,cVar25))) ==
                         CONCAT13(cVar28,CONCAT12(cVar27,CONCAT11(cVar26,cVar25))));
          iVar7 = -(uint)(CONCAT13(cVar32,CONCAT12(cVar31,CONCAT11(cVar30,cVar29))) ==
                         CONCAT13(cVar32,CONCAT12(cVar31,CONCAT11(cVar30,cVar29))));
          iVar8 = -(uint)(CONCAT13(cVar36,CONCAT12(cVar35,CONCAT11(cVar34,cVar33))) ==
                         CONCAT13(cVar36,CONCAT12(cVar35,CONCAT11(cVar34,cVar33))));
          uVar10 = extraout_RAX_00;
          goto LAB_1408e423a;
        }
        uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
      }
      auVar37[0] = -(*pcVar1 == cVar17);
      auVar37[1] = -(pcVar1[1] == cVar20);
      auVar37[2] = -(pcVar1[2] == cVar22);
      auVar37[3] = -(pcVar1[3] == cVar24);
      auVar37[4] = -(pcVar1[4] == cVar25);
      auVar37[5] = -(pcVar1[5] == cVar26);
      auVar37[6] = -(pcVar1[6] == cVar27);
      auVar37[7] = -(pcVar1[7] == cVar28);
      auVar37[8] = -(pcVar1[8] == cVar29);
      auVar37[9] = -(pcVar1[9] == cVar30);
      auVar37[10] = -(pcVar1[10] == cVar31);
      auVar37[0xb] = -(pcVar1[0xb] == cVar32);
      auVar37[0xc] = -(pcVar1[0xc] == cVar33);
      auVar37[0xd] = -(pcVar1[0xd] == cVar34);
      auVar37[0xe] = -(pcVar1[0xe] == cVar35);
      auVar37[0xf] = -(pcVar1[0xf] == cVar36);
      if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar37 >> 0x17,0) & 1) != 0) || (SUB161(auVar37 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar37 >> 0x27,0) & 1) != 0) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar37 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar37 >> 0x3f,0) & 1) != 0) || (SUB161(auVar37 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar37 >> 0x4f,0) & 1) != 0) || (SUB161(auVar37 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar37 >> 0x5f,0) & 1) != 0) || (SUB161(auVar37 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar37 >> 0x6f,0) & 1) != 0) || (SUB161(auVar37 >> 0x77,0) & 1) != 0) ||
          auVar37[0xf] < '\0') break;
      uVar10 = uVar10 + lVar11 + 0x10;
      lVar11 = lVar11 + 0x10;
    }
  }
  uVar9 = 0;
LAB_1408e42ae:
  *(undefined1 *)((longlong)in_RCX + 1) = uVar9;
  *(undefined1 *)in_RCX = 1;
  return;
LAB_1408e423a:
  uVar10 = *(ulonglong *)(in_stack_00000028 + 0x3e00) & uVar10;
  pcVar1 = (char *)(lVar2 + uVar10);
  cVar17 = auVar18[0];
  auVar40[0] = -(*pcVar1 == cVar17);
  cVar20 = auVar18[1];
  auVar40[1] = -(pcVar1[1] == cVar20);
  cVar22 = auVar18[2];
  auVar40[2] = -(pcVar1[2] == cVar22);
  cVar24 = auVar18[3];
  auVar40[3] = -(pcVar1[3] == cVar24);
  auVar40[4] = -(pcVar1[4] == cVar17);
  auVar40[5] = -(pcVar1[5] == cVar20);
  auVar40[6] = -(pcVar1[6] == cVar22);
  auVar40[7] = -(pcVar1[7] == cVar24);
  auVar40[8] = -(pcVar1[8] == cVar17);
  auVar40[9] = -(pcVar1[9] == cVar20);
  auVar40[10] = -(pcVar1[10] == cVar22);
  auVar40[0xb] = -(pcVar1[0xb] == cVar24);
  auVar40[0xc] = -(pcVar1[0xc] == cVar17);
  auVar40[0xd] = -(pcVar1[0xd] == cVar20);
  auVar40[0xe] = -(pcVar1[0xe] == cVar22);
  auVar40[0xf] = -(pcVar1[0xf] == cVar24);
  uVar13 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe | (ushort)(auVar40[0xf] >> 7) << 0xf;
  uVar12 = (uint)uVar13;
  while (uVar13 != 0) {
    uVar3 = 0;
    for (uVar4 = uVar12; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
      uVar3 = uVar3 + 1;
    }
    uVar14 = uVar3 + uVar10 & *(ulonglong *)(in_stack_00000028 + 0x3e00);
    if (local_f8 == *(longlong *)(lVar2 + -0x20 + uVar14 * -0x20)) {
      if (*(char *)(lVar2 + -8 + uVar14 * -0x20) != '\x11') {
        rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
        if (extraout_RAX_01 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
          core::option::expect_failed();
        }
        uVar10 = *extraout_RAX_01 & 0xffffffff;
        if (((uVar10 < *(ulonglong *)(in_stack_00000028 + 0x10)) &&
            (lVar2 = *(longlong *)(in_stack_00000028 + 8), lVar2 != 0)) &&
           (*(uint *)(lVar2 + 0x20 + uVar10 * 0x28) == ((uint)(*extraout_RAX_01 >> 0x20) | 1))) {
          local_150 = *(undefined8 *)((longlong)extraout_RAX_01 + 0x44);
          local_148 = *(undefined4 *)((longlong)extraout_RAX_01 + 0x4c);
          local_138 = *(undefined8 *)((longlong)extraout_RAX_01 + 0x34);
          uStack_130 = *(undefined8 *)((longlong)extraout_RAX_01 + 0x3c);
          ::utils::IsoTransform::from_translation_rotation();
          ::utils::swizzlers::impl$1::xvy();
          local_104 = 0xbf80000080000000;
          local_fc = 0x80000000;
          local_138 = local_68;
          uStack_130 = uStack_60;
          local_120 = local_70;
          local_128 = local_78;
          lVar2 = *(longlong *)(lVar2 + uVar10 * 0x28 + 0x18);
          lVar11 = *(longlong *)(lVar2 + 0x38);
          local_150 = local_110;
          local_148 = local_108;
          local_144 = 0xbf80000080000000;
          local_13c = 0x80000000;
          (**(code **)(lVar11 + 0x38))
                    (local_50,(*(longlong *)(lVar11 + 0x10) - 1U & 0xfffffffffffffff0) +
                              *(longlong *)(lVar2 + 0x30) + 0x10,&local_138,&local_150,
                     CONCAT44(uVar41,0x7f7fffff),1);
          if ((local_50[0] != 4) &&
             (systems::collision::world::rapier_hit_to_raycast_world_result(), local_84 != 2)) {
            uStack_d9._1_3_ = (undefined3)local_97;
            uStack_e0 = (undefined7)uStack_9f;
            uStack_d9._0_1_ = (undefined1)((ulonglong)uStack_9f >> 0x38);
            if ((local_84 & 1) != 0) {
              *(undefined4 *)(in_RCX + 10) = uStack_d9;
              *(ulonglong *)(in_RCX + 6) = CONCAT71(uStack_e0,uStack_a0);
              *(undefined8 *)(in_RCX + 2) = local_80;
              *(undefined1 *)in_RCX = 0;
              return;
            }
            *(undefined1 *)((longlong)in_RCX + 1) = 4;
            *(undefined1 *)in_RCX = 1;
            return;
          }
          *in_RCX = 0x301;
          return;
        }
                    // WARNING: Subroutine does not return
        std::panicking::begin_panic<ref_<str_>_>();
      }
      goto LAB_1408e42ae;
    }
    uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
    uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
  }
  auVar38[0] = -(*pcVar1 == (char)iVar5);
  auVar38[1] = -(pcVar1[1] == (char)((uint)iVar5 >> 8));
  auVar38[2] = -(pcVar1[2] == (char)((uint)iVar5 >> 0x10));
  auVar38[3] = -(pcVar1[3] == (char)((uint)iVar5 >> 0x18));
  auVar38[4] = -(pcVar1[4] == (char)iVar6);
  auVar38[5] = -(pcVar1[5] == (char)((uint)iVar6 >> 8));
  auVar38[6] = -(pcVar1[6] == (char)((uint)iVar6 >> 0x10));
  auVar38[7] = -(pcVar1[7] == (char)((uint)iVar6 >> 0x18));
  auVar38[8] = -(pcVar1[8] == (char)iVar7);
  auVar38[9] = -(pcVar1[9] == (char)((uint)iVar7 >> 8));
  auVar38[10] = -(pcVar1[10] == (char)((uint)iVar7 >> 0x10));
  auVar38[0xb] = -(pcVar1[0xb] == (char)((uint)iVar7 >> 0x18));
  auVar38[0xc] = -(pcVar1[0xc] == (char)iVar8);
  auVar38[0xd] = -(pcVar1[0xd] == (char)((uint)iVar8 >> 8));
  auVar38[0xe] = -(pcVar1[0xe] == (char)((uint)iVar8 >> 0x10));
  auVar38[0xf] = -(pcVar1[0xf] == (char)((uint)iVar8 >> 0x18));
  if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') goto LAB_1408e42ae;
  uVar10 = uVar10 + lVar11 + 0x10;
  lVar11 = lVar11 + 0x10;
  goto LAB_1408e423a;
}


