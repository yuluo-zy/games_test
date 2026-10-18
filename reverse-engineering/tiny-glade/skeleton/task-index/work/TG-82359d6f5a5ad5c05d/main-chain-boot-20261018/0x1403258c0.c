
// WARNING: Type propagation algorithm not settling

void __cdecl
bevy_state::app::impl$0::init_state<country_core::app_loading_state::AppLoadingState>(void)

{
  undefined **ppuVar1;
  char *pcVar2;
  short sVar3;
  uint uVar4;
  uint uVar5;
  undefined4 uVar6;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  char ******extraout_RAX_01;
  longlong extraout_RAX_02;
  char ******extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  undefined8 *extraout_RAX_10;
  longlong in_RCX;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  undefined1 uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong lVar13;
  ushort uVar14;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  char ******ppppppcVar18;
  ulonglong uVar19;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  char *******local_268;
  code *local_260;
  undefined **local_198;
  char *******local_190;
  char *******pppppppcStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  char *local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined *puStack_148;
  undefined8 local_140;
  char *******pppppppcStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  char *******local_c8;
  code *local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined2 local_70;
  undefined1 local_6e;
  undefined **local_68;
  longlong local_60;
  void *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  warn_if_no_states_plugin_installed();
  ppuVar1 = (undefined **)(in_RCX + 0x38);
  uVar11 = 0x35385c45b6f578e9;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar11)) ||
      (uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar11 * 8), uVar11 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar11 * 0x60) == 0)) {
    lVar8 = in_RCX + 0x5a8;
    local_68 = ppuVar1;
    local_60 = lVar8;
    if (*(longlong *)(in_RCX + 0x88) != 0) {
      lVar7 = *(longlong *)(in_RCX + 0x70);
      uVar11 = *(ulonglong *)(in_RCX + 0x78);
      uVar12 = uVar11 & 0xf1623d81d5dd0c9f;
      lVar15 = 0;
      while( true ) {
        pcVar2 = (char *)(lVar7 + uVar12);
        auVar28[0] = -(*pcVar2 == 'x');
        auVar28[1] = -(pcVar2[1] == 'x');
        auVar28[2] = -(pcVar2[2] == 'x');
        auVar28[3] = -(pcVar2[3] == 'x');
        auVar28[4] = -(pcVar2[4] == 'x');
        auVar28[5] = -(pcVar2[5] == 'x');
        auVar28[6] = -(pcVar2[6] == 'x');
        auVar28[7] = -(pcVar2[7] == 'x');
        auVar28[8] = -(pcVar2[8] == 'x');
        auVar28[9] = -(pcVar2[9] == 'x');
        auVar28[10] = -(pcVar2[10] == 'x');
        auVar28[0xb] = -(pcVar2[0xb] == 'x');
        auVar28[0xc] = -(pcVar2[0xc] == 'x');
        auVar28[0xd] = -(pcVar2[0xd] == 'x');
        auVar28[0xe] = -(pcVar2[0xe] == 'x');
        auVar28[0xf] = -(pcVar2[0xf] == 'x');
        uVar14 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe | (ushort)(auVar28[0xf] >> 7) << 0xf
        ;
        uVar16 = (uint)uVar14;
        while (uVar14 != 0) {
          uVar4 = 0;
          for (uVar5 = uVar16; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          uVar19 = uVar4 + uVar12 & uVar11;
          if ((*(longlong *)(lVar7 + -0x18 + uVar19 * -0x18) == 0x35385c45b6f578e9) &&
             (*(longlong *)(lVar7 + -0x10 + uVar19 * -0x18) == -0xe9dc27e2a22f361)) {
            ppppppcVar18 = *(char *******)(lVar7 + uVar19 * -0x18 + -8);
            if (ppppppcVar18 < *(char *******)(in_RCX + 0x218)) goto LAB_140325dce;
            goto LAB_140325dfc;
          }
          uVar14 = (ushort)(uVar16 - 1) & (ushort)uVar16;
          uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar14);
        }
        auVar24[0] = -(*pcVar2 == -1);
        auVar24[1] = -(pcVar2[1] == -1);
        auVar24[2] = -(pcVar2[2] == -1);
        auVar24[3] = -(pcVar2[3] == -1);
        auVar24[4] = -(pcVar2[4] == -1);
        auVar24[5] = -(pcVar2[5] == -1);
        auVar24[6] = -(pcVar2[6] == -1);
        auVar24[7] = -(pcVar2[7] == -1);
        auVar24[8] = -(pcVar2[8] == -1);
        auVar24[9] = -(pcVar2[9] == -1);
        auVar24[10] = -(pcVar2[10] == -1);
        auVar24[0xb] = -(pcVar2[0xb] == -1);
        auVar24[0xc] = -(pcVar2[0xc] == -1);
        auVar24[0xd] = -(pcVar2[0xd] == -1);
        auVar24[0xe] = -(pcVar2[0xe] == -1);
        auVar24[0xf] = -(pcVar2[0xf] == -1);
        if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x1f,0) & 1) != 0) || (SUB161(auVar24 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') break;
        lVar13 = uVar12 + lVar15;
        lVar15 = lVar15 + 0x10;
        uVar12 = lVar13 + 0x10U & uVar11;
      }
    }
    lVar7 = *(longlong *)(in_RCX + 0xd8);
    uVar11 = *(ulonglong *)(in_RCX + 0xe0);
    uVar12 = uVar11 & 0xf1623d81d5dd0c9f;
    lVar15 = 0;
    do {
      pcVar2 = (char *)(lVar7 + uVar12);
      auVar29[0] = -(*pcVar2 == 'x');
      auVar29[1] = -(pcVar2[1] == 'x');
      auVar29[2] = -(pcVar2[2] == 'x');
      auVar29[3] = -(pcVar2[3] == 'x');
      auVar29[4] = -(pcVar2[4] == 'x');
      auVar29[5] = -(pcVar2[5] == 'x');
      auVar29[6] = -(pcVar2[6] == 'x');
      auVar29[7] = -(pcVar2[7] == 'x');
      auVar29[8] = -(pcVar2[8] == 'x');
      auVar29[9] = -(pcVar2[9] == 'x');
      auVar29[10] = -(pcVar2[10] == 'x');
      auVar29[0xb] = -(pcVar2[0xb] == 'x');
      auVar29[0xc] = -(pcVar2[0xc] == 'x');
      auVar29[0xd] = -(pcVar2[0xd] == 'x');
      auVar29[0xe] = -(pcVar2[0xe] == 'x');
      auVar29[0xf] = -(pcVar2[0xf] == 'x');
      uVar14 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
      uVar16 = (uint)uVar14;
      if (uVar14 != 0) {
LAB_140325b2d:
        uVar4 = 0;
        for (uVar5 = uVar16; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar19 = uVar4 + uVar12 & uVar11;
        lVar13 = uVar19 * -0x88;
        if ((*(longlong *)(lVar7 + -0x88 + lVar13) != 0x35385c45b6f578e9) ||
           (*(longlong *)(lVar7 + -0x80 + lVar13) != -0xe9dc27e2a22f361)) goto LAB_140325b20;
        lVar15 = ((longlong)(uVar19 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
        uVar11 = lVar15 - 0x10U & uVar11;
        pcVar2 = (char *)(lVar7 + uVar11);
        auVar20[0] = -(*pcVar2 == -1);
        auVar20[1] = -(pcVar2[1] == -1);
        auVar20[2] = -(pcVar2[2] == -1);
        auVar20[3] = -(pcVar2[3] == -1);
        auVar20[4] = -(pcVar2[4] == -1);
        auVar20[5] = -(pcVar2[5] == -1);
        auVar20[6] = -(pcVar2[6] == -1);
        auVar20[7] = -(pcVar2[7] == -1);
        auVar20[8] = -(pcVar2[8] == -1);
        auVar20[9] = -(pcVar2[9] == -1);
        auVar20[10] = -(pcVar2[10] == -1);
        auVar20[0xb] = -(pcVar2[0xb] == -1);
        auVar20[0xc] = -(pcVar2[0xc] == -1);
        auVar20[0xd] = -(pcVar2[0xd] == -1);
        auVar20[0xe] = -(pcVar2[0xe] == -1);
        auVar20[0xf] = -(pcVar2[0xf] == -1);
        uVar14 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf
        ;
        pcVar2 = (char *)(lVar7 + lVar15);
        auVar21[0] = -(*pcVar2 == -1);
        auVar21[1] = -(pcVar2[1] == -1);
        auVar21[2] = -(pcVar2[2] == -1);
        auVar21[3] = -(pcVar2[3] == -1);
        auVar21[4] = -(pcVar2[4] == -1);
        auVar21[5] = -(pcVar2[5] == -1);
        auVar21[6] = -(pcVar2[6] == -1);
        auVar21[7] = -(pcVar2[7] == -1);
        auVar21[8] = -(pcVar2[8] == -1);
        auVar21[9] = -(pcVar2[9] == -1);
        auVar21[10] = -(pcVar2[10] == -1);
        auVar21[0xb] = -(pcVar2[0xb] == -1);
        auVar21[0xc] = -(pcVar2[0xc] == -1);
        auVar21[0xd] = -(pcVar2[0xd] == -1);
        auVar21[0xe] = -(pcVar2[0xe] == -1);
        auVar21[0xf] = -(pcVar2[0xf] == -1);
        if (uVar14 == 0) {
          uVar17 = 0x10;
        }
        else {
          uVar17 = 0xf;
          if (uVar14 != 0) {
            for (; uVar14 >> uVar17 == 0; uVar17 = uVar17 - 1) {
            }
          }
          uVar17 = uVar17 ^ 0xf;
        }
        lVar13 = lVar13 + lVar7;
        sVar3 = 0;
        for (uVar16 = (ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                              (ushort)(auVar21[0xf] >> 7) << 0xf) | 0x10000; (uVar16 & 1) == 0;
            uVar16 = uVar16 >> 1 | 0x80000000) {
          sVar3 = sVar3 + 1;
        }
        uVar10 = 0x80;
        if ((ushort)(uVar17 + sVar3) < 0x10) {
          *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
          uVar10 = 0xff;
        }
        *(undefined1 *)(lVar7 + lVar15) = uVar10;
        *(undefined1 *)(lVar7 + 0x10 + uVar11) = uVar10;
        *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
        if (*(undefined ***)(lVar13 + -0x78) == (undefined **)0x2) goto LAB_140325bf8;
        local_130 = *(undefined8 *)(lVar13 + -0x10);
        uStack_128 = *(undefined8 *)(lVar13 + -8);
        local_140 = *(undefined8 *)(lVar13 + -0x20);
        pppppppcStack_138 = *(char ********)(lVar13 + -0x18);
        local_150 = *(undefined8 *)(lVar13 + -0x30);
        puStack_148 = *(undefined **)(lVar13 + -0x28);
        local_190 = *(char ********)(lVar13 + -0x70);
        pppppppcStack_188 = *(char ********)(lVar13 + -0x68);
        local_180 = *(undefined8 *)(lVar13 + -0x60);
        uStack_178 = *(undefined8 *)(lVar13 + -0x58);
        uStack_170 = *(undefined8 *)(lVar13 + -0x50);
        uStack_168 = *(undefined8 *)(lVar13 + -0x48);
        local_160 = *(char **)(lVar13 + -0x40);
        uStack_158 = *(undefined8 *)(lVar13 + -0x38);
        local_198 = *(undefined ***)(lVar13 + -0x78);
        bevy_ecs::component::QueuedRegistration::register();
        ppppppcVar18 = extraout_RAX_01;
        if (extraout_RAX_01 < *(char *******)(in_RCX + 0x218)) goto LAB_140325dce;
        goto LAB_140325dfc;
      }
LAB_140325b00:
      auVar25[0] = -(*pcVar2 == -1);
      auVar25[1] = -(pcVar2[1] == -1);
      auVar25[2] = -(pcVar2[2] == -1);
      auVar25[3] = -(pcVar2[3] == -1);
      auVar25[4] = -(pcVar2[4] == -1);
      auVar25[5] = -(pcVar2[5] == -1);
      auVar25[6] = -(pcVar2[6] == -1);
      auVar25[7] = -(pcVar2[7] == -1);
      auVar25[8] = -(pcVar2[8] == -1);
      auVar25[9] = -(pcVar2[9] == -1);
      auVar25[10] = -(pcVar2[10] == -1);
      auVar25[0xb] = -(pcVar2[0xb] == -1);
      auVar25[0xc] = -(pcVar2[0xc] == -1);
      auVar25[0xd] = -(pcVar2[0xd] == -1);
      auVar25[0xe] = -(pcVar2[0xe] == -1);
      auVar25[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') goto LAB_140325bf8;
      lVar13 = uVar12 + lVar15;
      lVar15 = lVar15 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar11;
    } while( true );
  }
  local_c8 = (char *******)0x1429be158;
  local_c0 = (code *)0x30;
  if (*(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E < 2) {
    return;
  }
  local_268 = (char *******)&local_c8;
  local_260 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  uStack_178 = *extraout_RAX_00;
  uStack_170 = extraout_RAX_00[1];
  uVar6 = *(undefined4 *)(extraout_RAX_00 + 2);
  uStack_168 = 2;
  local_190 = (char *******)0x1429be1e0;
  local_160 = "bevy_state::app";
  puStack_148 = &DAT_1429be1a8;
  pppppppcStack_138 = (char *******)&local_268;
  goto LAB_1403265d7;
LAB_140325b20:
  uVar14 = (ushort)(uVar16 - 1) & (ushort)uVar16;
  uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar14);
  if (uVar14 == 0) goto LAB_140325b00;
  goto LAB_140325b2d;
LAB_140325bf8:
  ppppppcVar18 = *(char *******)(in_RCX + 0x5a8);
  *(ulonglong *)(in_RCX + 0x5a8) = (longlong)ppppppcVar18 + 1U;
  local_90 = 0x8000000000000000;
  local_88 = &anon_893984a47b10772a2f3aa373d073e074_35_llvm_8286655925340897655;
  local_80 = 0x55;
  local_70 = 0x100;
  local_c8 = (char *******)0x1;
  local_c0 = (code *)0x35385c45b6f578e9;
  local_b8 = 0xf1623d81d5dd0c9f;
  local_a0 = 1;
  local_98 = 1;
  local_78 = 0;
  local_6e = 1;
  local_b0 = 0;
  bevy_ecs::component::ComponentInfo::new();
  if (*(ulonglong *)(in_RCX + 0x48) < (longlong)ppppppcVar18 + 1U) {
    local_49 = 1;
    alloc::vec::Vec::
    resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
              ();
  }
  lVar7 = *(longlong *)(in_RCX + 0x40);
  local_58 = (void *)(lVar7 + (longlong)ppppppcVar18 * 0xd0);
  memcpy(&local_198,&local_268,0xd0);
  if (*(int *)(lVar7 + (longlong)ppppppcVar18 * 0xd0) != 2) {
    core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
  }
  memcpy(local_58,&local_198,0xd0);
  hashbrown::map::HashMap::
  insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
            ();
  if (ppppppcVar18 < *(char *******)(in_RCX + 0x218)) {
LAB_140325dce:
    uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + (longlong)ppppppcVar18 * 8);
    if ((uVar11 != 0) && (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar11 * 0x60) != 0)
       ) goto LAB_140325e9a;
  }
LAB_140325dfc:
  local_c8 = (char *******)((ulonglong)local_c8 & 0xffffffffffffff00);
  uVar6 = *(undefined4 *)(in_RCX + 0x5b0);
  bevy_ecs::world::World::flush_components();
  pppppppcStack_188 = (char *******)(in_RCX + 0xf8);
  local_190 = (char *******)&local_268;
  local_268 = (char *******)ppppppcVar18;
  local_198 = ppuVar1;
  bevy_ecs::storage::sparse_set::SparseSet::
  get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl_2::initialize_with::closure_env_0<true,bevy_ecs::world::impl_2::initialize_resource_internal::closure_env_0>_>
            ();
  if (*(longlong *)(extraout_RAX_02 + 0x20) == 0) {
    if (*(longlong *)(extraout_RAX_02 + 0x18) == 0) {
      bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve();
      lVar7 = *(longlong *)(extraout_RAX_02 + 0x20);
    }
    else {
      lVar7 = 0;
    }
    *(longlong *)(extraout_RAX_02 + 0x20) = lVar7 + 1;
    memcpy((void *)(lVar7 * *(size_t *)(extraout_RAX_02 + 8) + *(longlong *)(extraout_RAX_02 + 0x10)
                   ),&local_c8,*(size_t *)(extraout_RAX_02 + 8));
    *(undefined4 *)(extraout_RAX_02 + 0x58) = uVar6;
  }
  else {
    bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked();
  }
  *(undefined4 *)(extraout_RAX_02 + 0x5c) = uVar6;
LAB_140325e9a:
  if (*(longlong *)(in_RCX + 0x88) != 0) {
    lVar7 = *(longlong *)(in_RCX + 0x70);
    uVar11 = *(ulonglong *)(in_RCX + 0x78);
    uVar12 = uVar11 & 0x19d5dd093b264385;
    lVar15 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar7 + uVar12);
      auVar30[0] = -(*pcVar2 == '\f');
      auVar30[1] = -(pcVar2[1] == '\f');
      auVar30[2] = -(pcVar2[2] == '\f');
      auVar30[3] = -(pcVar2[3] == '\f');
      auVar30[4] = -(pcVar2[4] == '\f');
      auVar30[5] = -(pcVar2[5] == '\f');
      auVar30[6] = -(pcVar2[6] == '\f');
      auVar30[7] = -(pcVar2[7] == '\f');
      auVar30[8] = -(pcVar2[8] == '\f');
      auVar30[9] = -(pcVar2[9] == '\f');
      auVar30[10] = -(pcVar2[10] == '\f');
      auVar30[0xb] = -(pcVar2[0xb] == '\f');
      auVar30[0xc] = -(pcVar2[0xc] == '\f');
      auVar30[0xd] = -(pcVar2[0xd] == '\f');
      auVar30[0xe] = -(pcVar2[0xe] == '\f');
      auVar30[0xf] = -(pcVar2[0xf] == '\f');
      uVar14 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
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
      uVar16 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar16; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar19 = uVar4 + uVar12 & uVar11;
        if ((*(longlong *)(lVar7 + -0x18 + uVar19 * -0x18) == -0x4d4e5c7a1782016c) &&
           (*(longlong *)(lVar7 + -0x10 + uVar19 * -0x18) == 0x19d5dd093b264385)) {
          ppppppcVar18 = *(char *******)(lVar7 + uVar19 * -0x18 + -8);
          local_68 = ppuVar1;
          local_60 = lVar8;
          if (ppppppcVar18 < *(char *******)(in_RCX + 0x218)) goto LAB_14032627c;
          goto LAB_1403262aa;
        }
        uVar14 = (ushort)(uVar16 - 1) & (ushort)uVar16;
        uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar14);
      }
      auVar26[0] = -(*pcVar2 == -1);
      auVar26[1] = -(pcVar2[1] == -1);
      auVar26[2] = -(pcVar2[2] == -1);
      auVar26[3] = -(pcVar2[3] == -1);
      auVar26[4] = -(pcVar2[4] == -1);
      auVar26[5] = -(pcVar2[5] == -1);
      auVar26[6] = -(pcVar2[6] == -1);
      auVar26[7] = -(pcVar2[7] == -1);
      auVar26[8] = -(pcVar2[8] == -1);
      auVar26[9] = -(pcVar2[9] == -1);
      auVar26[10] = -(pcVar2[10] == -1);
      auVar26[0xb] = -(pcVar2[0xb] == -1);
      auVar26[0xc] = -(pcVar2[0xc] == -1);
      auVar26[0xd] = -(pcVar2[0xd] == -1);
      auVar26[0xe] = -(pcVar2[0xe] == -1);
      auVar26[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar26 >> 0x6f,0) & 1) != 0) || (SUB161(auVar26 >> 0x77,0) & 1) != 0) ||
          auVar26[0xf] < '\0') break;
      lVar13 = uVar12 + lVar15;
      lVar15 = lVar15 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar11;
    }
  }
  lVar7 = *(longlong *)(in_RCX + 0xd8);
  uVar11 = *(ulonglong *)(in_RCX + 0xe0);
  uVar12 = uVar11 & 0x19d5dd093b264385;
  lVar15 = 0;
  do {
    pcVar2 = (char *)(lVar7 + uVar12);
    auVar31[0] = -(*pcVar2 == '\f');
    auVar31[1] = -(pcVar2[1] == '\f');
    auVar31[2] = -(pcVar2[2] == '\f');
    auVar31[3] = -(pcVar2[3] == '\f');
    auVar31[4] = -(pcVar2[4] == '\f');
    auVar31[5] = -(pcVar2[5] == '\f');
    auVar31[6] = -(pcVar2[6] == '\f');
    auVar31[7] = -(pcVar2[7] == '\f');
    auVar31[8] = -(pcVar2[8] == '\f');
    auVar31[9] = -(pcVar2[9] == '\f');
    auVar31[10] = -(pcVar2[10] == '\f');
    auVar31[0xb] = -(pcVar2[0xb] == '\f');
    auVar31[0xc] = -(pcVar2[0xc] == '\f');
    auVar31[0xd] = -(pcVar2[0xd] == '\f');
    auVar31[0xe] = -(pcVar2[0xe] == '\f');
    auVar31[0xf] = -(pcVar2[0xf] == '\f');
    uVar14 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf;
    uVar16 = (uint)uVar14;
    if (uVar14 != 0) {
LAB_140325fdd:
      uVar4 = 0;
      for (uVar5 = uVar16; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      uVar19 = uVar4 + uVar12 & uVar11;
      lVar13 = uVar19 * -0x88;
      if ((*(longlong *)(lVar7 + -0x88 + lVar13) != -0x4d4e5c7a1782016c) ||
         (*(longlong *)(lVar7 + -0x80 + lVar13) != 0x19d5dd093b264385)) break;
      lVar15 = ((longlong)(uVar19 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
      uVar11 = lVar15 - 0x10U & uVar11;
      pcVar2 = (char *)(lVar7 + uVar11);
      auVar22[0] = -(*pcVar2 == -1);
      auVar22[1] = -(pcVar2[1] == -1);
      auVar22[2] = -(pcVar2[2] == -1);
      auVar22[3] = -(pcVar2[3] == -1);
      auVar22[4] = -(pcVar2[4] == -1);
      auVar22[5] = -(pcVar2[5] == -1);
      auVar22[6] = -(pcVar2[6] == -1);
      auVar22[7] = -(pcVar2[7] == -1);
      auVar22[8] = -(pcVar2[8] == -1);
      auVar22[9] = -(pcVar2[9] == -1);
      auVar22[10] = -(pcVar2[10] == -1);
      auVar22[0xb] = -(pcVar2[0xb] == -1);
      auVar22[0xc] = -(pcVar2[0xc] == -1);
      auVar22[0xd] = -(pcVar2[0xd] == -1);
      auVar22[0xe] = -(pcVar2[0xe] == -1);
      auVar22[0xf] = -(pcVar2[0xf] == -1);
      uVar14 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
      pcVar2 = (char *)(lVar7 + lVar15);
      auVar23[0] = -(*pcVar2 == -1);
      auVar23[1] = -(pcVar2[1] == -1);
      auVar23[2] = -(pcVar2[2] == -1);
      auVar23[3] = -(pcVar2[3] == -1);
      auVar23[4] = -(pcVar2[4] == -1);
      auVar23[5] = -(pcVar2[5] == -1);
      auVar23[6] = -(pcVar2[6] == -1);
      auVar23[7] = -(pcVar2[7] == -1);
      auVar23[8] = -(pcVar2[8] == -1);
      auVar23[9] = -(pcVar2[9] == -1);
      auVar23[10] = -(pcVar2[10] == -1);
      auVar23[0xb] = -(pcVar2[0xb] == -1);
      auVar23[0xc] = -(pcVar2[0xc] == -1);
      auVar23[0xd] = -(pcVar2[0xd] == -1);
      auVar23[0xe] = -(pcVar2[0xe] == -1);
      auVar23[0xf] = -(pcVar2[0xf] == -1);
      if (uVar14 == 0) {
        uVar17 = 0x10;
      }
      else {
        uVar17 = 0xf;
        if (uVar14 != 0) {
          for (; uVar14 >> uVar17 == 0; uVar17 = uVar17 - 1) {
          }
        }
        uVar17 = uVar17 ^ 0xf;
      }
      lVar13 = lVar13 + lVar7;
      sVar3 = 0;
      for (uVar16 = (ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                            (ushort)(auVar23[0xf] >> 7) << 0xf) | 0x10000; (uVar16 & 1) == 0;
          uVar16 = uVar16 >> 1 | 0x80000000) {
        sVar3 = sVar3 + 1;
      }
      uVar10 = 0x80;
      if ((ushort)(uVar17 + sVar3) < 0x10) {
        *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
        uVar10 = 0xff;
      }
      *(undefined1 *)(lVar7 + lVar15) = uVar10;
      *(undefined1 *)(lVar7 + 0x10 + uVar11) = uVar10;
      *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
      if (*(undefined ***)(lVar13 + -0x78) == (undefined **)0x2) goto LAB_1403260ad;
      local_130 = *(undefined8 *)(lVar13 + -0x10);
      uStack_128 = *(undefined8 *)(lVar13 + -8);
      local_140 = *(undefined8 *)(lVar13 + -0x20);
      pppppppcStack_138 = *(char ********)(lVar13 + -0x18);
      local_150 = *(undefined8 *)(lVar13 + -0x30);
      puStack_148 = *(undefined **)(lVar13 + -0x28);
      local_190 = *(char ********)(lVar13 + -0x70);
      pppppppcStack_188 = *(char ********)(lVar13 + -0x68);
      local_180 = *(undefined8 *)(lVar13 + -0x60);
      uStack_178 = *(undefined8 *)(lVar13 + -0x58);
      uStack_170 = *(undefined8 *)(lVar13 + -0x50);
      uStack_168 = *(undefined8 *)(lVar13 + -0x48);
      local_160 = *(char **)(lVar13 + -0x40);
      uStack_158 = *(undefined8 *)(lVar13 + -0x38);
      local_198 = *(undefined ***)(lVar13 + -0x78);
      local_68 = ppuVar1;
      local_60 = lVar8;
      bevy_ecs::component::QueuedRegistration::register();
      ppppppcVar18 = extraout_RAX_03;
      if (extraout_RAX_03 < *(char *******)(in_RCX + 0x218)) goto LAB_14032627c;
      goto LAB_1403262aa;
    }
LAB_140325fa9:
    auVar27[0] = -(*pcVar2 == -1);
    auVar27[1] = -(pcVar2[1] == -1);
    auVar27[2] = -(pcVar2[2] == -1);
    auVar27[3] = -(pcVar2[3] == -1);
    auVar27[4] = -(pcVar2[4] == -1);
    auVar27[5] = -(pcVar2[5] == -1);
    auVar27[6] = -(pcVar2[6] == -1);
    auVar27[7] = -(pcVar2[7] == -1);
    auVar27[8] = -(pcVar2[8] == -1);
    auVar27[9] = -(pcVar2[9] == -1);
    auVar27[10] = -(pcVar2[10] == -1);
    auVar27[0xb] = -(pcVar2[0xb] == -1);
    auVar27[0xc] = -(pcVar2[0xc] == -1);
    auVar27[0xd] = -(pcVar2[0xd] == -1);
    auVar27[0xe] = -(pcVar2[0xe] == -1);
    auVar27[0xf] = -(pcVar2[0xf] == -1);
    if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 || (SUB161(auVar27 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar27 >> 0x17,0) & 1) != 0) || (SUB161(auVar27 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar27 >> 0x27,0) & 1) != 0) || (SUB161(auVar27 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar27 >> 0x37,0) & 1) != 0) || (SUB161(auVar27 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar27 >> 0x47,0) & 1) != 0) || (SUB161(auVar27 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar27 >> 0x57,0) & 1) != 0) || (SUB161(auVar27 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar27 >> 0x67,0) & 1) != 0) || (SUB161(auVar27 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar27 >> 0x77,0) & 1) != 0) || auVar27[0xf] < '\0') goto LAB_1403260ad;
    lVar13 = uVar12 + lVar15;
    lVar15 = lVar15 + 0x10;
    uVar12 = lVar13 + 0x10U & uVar11;
  } while( true );
  uVar14 = (ushort)(uVar16 - 1) & (ushort)uVar16;
  uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar14);
  if (uVar14 == 0) goto LAB_140325fa9;
  goto LAB_140325fdd;
LAB_1403260ad:
  ppppppcVar18 = *(char *******)(in_RCX + 0x5a8);
  *(ulonglong *)(in_RCX + 0x5a8) = (longlong)ppppppcVar18 + 1U;
  local_90 = 0x8000000000000000;
  local_88 = &anon_893984a47b10772a2f3aa373d073e074_330_llvm_8286655925340897655;
  local_80 = 0x59;
  local_70 = 0x100;
  local_c8 = (char *******)0x1;
  local_c0 = (code *)0xb2b1a385e87dfe94;
  local_b8 = 0x19d5dd093b264385;
  local_a0 = 1;
  local_98 = 1;
  local_78 = 0;
  local_6e = 1;
  local_b0 = 0;
  local_68 = ppuVar1;
  local_60 = lVar8;
  bevy_ecs::component::ComponentInfo::new();
  if (*(ulonglong *)(in_RCX + 0x48) < (longlong)ppppppcVar18 + 1U) {
    local_4a = 1;
    alloc::vec::Vec::
    resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
              ();
  }
  lVar8 = *(longlong *)(in_RCX + 0x40);
  local_58 = (void *)(lVar8 + (longlong)ppppppcVar18 * 0xd0);
  memcpy(&local_198,&local_268,0xd0);
  if (*(int *)(lVar8 + (longlong)ppppppcVar18 * 0xd0) != 2) {
    core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
  }
  memcpy(local_58,&local_198,0xd0);
  hashbrown::map::HashMap::
  insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
            ();
  if (ppppppcVar18 < *(char *******)(in_RCX + 0x218)) {
LAB_14032627c:
    uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + (longlong)ppppppcVar18 * 8);
    if ((uVar11 == 0) || (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar11 * 0x60) == 0)
       ) goto LAB_1403262aa;
  }
  else {
LAB_1403262aa:
    local_c8 = (char *******)CONCAT71(local_c8._1_7_,3);
    uVar6 = *(undefined4 *)(in_RCX + 0x5b0);
    bevy_ecs::world::World::flush_components();
    pppppppcStack_188 = (char *******)(in_RCX + 0xf8);
    local_190 = (char *******)&local_268;
    local_268 = (char *******)ppppppcVar18;
    local_198 = ppuVar1;
    bevy_ecs::storage::sparse_set::SparseSet::
    get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl_2::initialize_with::closure_env_0<true,bevy_ecs::world::impl_2::initialize_resource_internal::closure_env_0>_>
              ();
    if (*(longlong *)(extraout_RAX_04 + 0x20) == 0) {
      if (*(longlong *)(extraout_RAX_04 + 0x18) == 0) {
        bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve();
        lVar8 = *(longlong *)(extraout_RAX_04 + 0x20);
      }
      else {
        lVar8 = 0;
      }
      *(longlong *)(extraout_RAX_04 + 0x20) = lVar8 + 1;
      memcpy((void *)(lVar8 * *(size_t *)(extraout_RAX_04 + 8) +
                     *(longlong *)(extraout_RAX_04 + 0x10)),&local_c8,
             *(size_t *)(extraout_RAX_04 + 8));
      *(undefined4 *)(extraout_RAX_04 + 0x58) = uVar6;
    }
    else {
      bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked();
    }
    *(undefined4 *)(extraout_RAX_04 + 0x5c) = uVar6;
  }
  uVar11 = 0x9a5a39e8432e4e13;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_05 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar11)) ||
      (uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar11 * 8), uVar11 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar11 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState>_>
              ();
  }
  uVar11 = 0x8e1bd251962ba784;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_06 != 0) && (uVar11 < *(ulonglong *)(in_RCX + 0x218))) &&
     (uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar11 * 8), uVar11 != 0)) {
    lVar8 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar8) != 0) {
      *(undefined4 *)(*(longlong *)(in_RCX + 0x1e0) + lVar8 + 0x5c) =
           *(undefined4 *)(in_RCX + 0x5b0);
      bevy_ecs::schedule::schedule::Schedules::
      get_mut<bevy_state::state::transitions::StateTransition>();
      if (extraout_RAX_07 != 0) {
        state::freely_mutable_state::FreelyMutableState::
        register_state<country_core::app_loading_state::AppLoadingState>();
        uVar11 = 0x35385c45b6f578e9;
        bevy_ecs::component::Components::get_resource_id();
        if (((extraout_RAX_08 != 0) && (uVar11 < *(ulonglong *)(in_RCX + 0x218))) &&
           (uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar11 * 8), uVar11 != 0)) {
          lVar8 = ~uVar11 * 0x60;
          if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar8) != 0) {
            uVar10 = **(undefined1 **)(*(longlong *)(in_RCX + 0x1e0) + lVar8 + 0x10);
            uVar11 = 0x9a5a39e8432e4e13;
            bevy_ecs::component::Components::get_resource_id();
            if (((extraout_RAX_09 != 0) && (uVar11 < *(ulonglong *)(in_RCX + 0x218))) &&
               (uVar11 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar11 * 8), uVar11 != 0)) {
              lVar8 = ~uVar11 * 0x60;
              if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar8) != 0) {
                lVar8 = *(longlong *)(in_RCX + 0x1e0) + lVar8;
                lVar7 = *(longlong *)(lVar8 + 0x10);
                *(undefined4 *)(lVar8 + 0x5c) = *(undefined4 *)(in_RCX + 0x5b0);
                lVar8 = *(longlong *)(lVar7 + 0x40);
                lVar15 = *(longlong *)(lVar7 + 0x30);
                if (*(longlong *)(lVar7 + 0x20) == lVar15) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  lVar15 = *(longlong *)(lVar7 + 0x30);
                }
                lVar13 = *(longlong *)(lVar7 + 0x28);
                lVar9 = lVar15 * 0x10;
                *(longlong *)(lVar13 + lVar9) = lVar8;
                *(undefined1 *)(lVar13 + 8 + lVar9) = 3;
                *(undefined1 *)(lVar13 + 9 + lVar9) = uVar10;
                *(longlong *)(lVar7 + 0x30) = lVar15 + 1;
                *(longlong *)(lVar7 + 0x40) = lVar8 + 1;
                return;
              }
            }
            if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E == 0) {
              return;
            }
            local_268 = (char *******)0x1429be3e6;
            local_260 = (code *)0x66;
            local_c8 = (char *******)&local_268;
            local_c0 = anyhow::context::impl$3::
                       fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            uStack_178 = *extraout_RAX_10;
            uStack_170 = extraout_RAX_10[1];
            uVar6 = *(undefined4 *)(extraout_RAX_10 + 2);
            uStack_168 = 1;
            local_190 = (char *******)0x1429be518;
            local_160 = 
            "bevy_ecs::worldbevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs"
            ;
            puStack_148 = &DAT_1429be4e0;
            pppppppcStack_138 = (char *******)&local_c8;
LAB_1403265d7:
            local_140 = 2;
            uStack_158 = 0xf;
            local_130 = 1;
            uStack_128 = 0;
            local_198 = (undefined **)0x0;
            pppppppcStack_188 = (char *******)0xf;
            local_180 = 0;
            local_150 = CONCAT44(uVar6,1);
            ::log::__private_api::impl$2::log();
            return;
          }
        }
        local_268 = (char *******)&DAT_1429be527;
        local_260 = (code *)0x55;
        local_c8 = (char *******)&local_268;
        local_c0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_198 = &anon_30503377ee3af8255684fbd6b72664d5_17_llvm_16174880112525456576;
        local_190 = (char *******)0x2;
        uStack_178 = 0;
        pppppppcStack_188 = (char *******)&local_c8;
        local_180 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
    }
  }
                    // WARNING: Subroutine does not return
  core::option::expect_failed();
}

