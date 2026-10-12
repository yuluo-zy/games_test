
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
bevy_ecs::component::ComponentsRegistrator::
register_component<country_core::components::drawable::NaniInstanceDesc>(void)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint3 uVar6;
  ushort uVar7;
  longlong *in_RCX;
  uint uVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  ushort uVar13;
  ulonglong uVar14;
  longlong lVar15;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined *local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined *local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  longlong local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  ulonglong local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined2 local_70;
  undefined1 local_6e;
  longlong local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 *local_50;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  local_68 = 0;
  local_60 = 8;
  local_58 = 0;
  lVar2 = *in_RCX;
  if (*(longlong *)(lVar2 + 0x30) != 0) {
    lVar15 = *(longlong *)(lVar2 + 0x18);
    uVar11 = *(ulonglong *)(lVar2 + 0x20);
    uVar9 = uVar11 & 0x2a887be89e49c16b;
    lVar12 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar15 + uVar9);
      auVar20[0] = -(*pcVar1 == __xmm_15151515151515151515151515151515);
      auVar20[1] = -(pcVar1[1] == UNK_142950761);
      auVar20[2] = -(pcVar1[2] == UNK_142950762);
      auVar20[3] = -(pcVar1[3] == UNK_142950763);
      auVar20[4] = -(pcVar1[4] == UNK_142950764);
      auVar20[5] = -(pcVar1[5] == UNK_142950765);
      auVar20[6] = -(pcVar1[6] == UNK_142950766);
      auVar20[7] = -(pcVar1[7] == UNK_142950767);
      auVar20[8] = -(pcVar1[8] == UNK_142950768);
      auVar20[9] = -(pcVar1[9] == UNK_142950769);
      auVar20[10] = -(pcVar1[10] == UNK_14295076a);
      auVar20[0xb] = -(pcVar1[0xb] == UNK_14295076b);
      auVar20[0xc] = -(pcVar1[0xc] == UNK_14295076c);
      auVar20[0xd] = -(pcVar1[0xd] == UNK_14295076d);
      auVar20[0xe] = -(pcVar1[0xe] == UNK_14295076e);
      auVar20[0xf] = -(pcVar1[0xf] == UNK_14295076f);
      uVar7 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
              (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar3 = 0;
        for (uVar5 = uVar8; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar14 = uVar3 + uVar9 & uVar11;
        if ((*(longlong *)(lVar15 + -0x18 + uVar14 * -0x18) == 0x300777adfd693bf9) &&
           (*(longlong *)(lVar15 + -0x10 + uVar14 * -0x18) == 0x2a887be89e49c16b)) {
          return;
        }
        uVar7 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar7);
      }
      auVar18[0] = -(*pcVar1 == -1);
      auVar18[1] = -(pcVar1[1] == -1);
      auVar18[2] = -(pcVar1[2] == -1);
      auVar18[3] = -(pcVar1[3] == -1);
      auVar18[4] = -(pcVar1[4] == -1);
      auVar18[5] = -(pcVar1[5] == -1);
      auVar18[6] = -(pcVar1[6] == -1);
      auVar18[7] = -(pcVar1[7] == -1);
      auVar18[8] = -(pcVar1[8] == -1);
      auVar18[9] = -(pcVar1[9] == -1);
      auVar18[10] = -(pcVar1[10] == -1);
      auVar18[0xb] = -(pcVar1[0xb] == -1);
      auVar18[0xc] = -(pcVar1[0xc] == -1);
      auVar18[0xd] = -(pcVar1[0xd] == -1);
      auVar18[0xe] = -(pcVar1[0xe] == -1);
      auVar18[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      lVar10 = uVar9 + lVar12;
      lVar12 = lVar12 + 0x10;
      uVar9 = lVar10 + 0x10U & uVar11;
    }
  }
  lVar15 = *(longlong *)(lVar2 + 0x80);
  uVar11 = *(ulonglong *)(lVar2 + 0x88);
  uVar9 = uVar11 & 0x2a887be89e49c16b;
  lVar12 = 0;
  do {
    pcVar1 = (char *)(lVar15 + uVar9);
    auVar21[0] = -(*pcVar1 == __xmm_15151515151515151515151515151515);
    auVar21[1] = -(pcVar1[1] == UNK_142950761);
    auVar21[2] = -(pcVar1[2] == UNK_142950762);
    auVar21[3] = -(pcVar1[3] == UNK_142950763);
    auVar21[4] = -(pcVar1[4] == UNK_142950764);
    auVar21[5] = -(pcVar1[5] == UNK_142950765);
    auVar21[6] = -(pcVar1[6] == UNK_142950766);
    auVar21[7] = -(pcVar1[7] == UNK_142950767);
    auVar21[8] = -(pcVar1[8] == UNK_142950768);
    auVar21[9] = -(pcVar1[9] == UNK_142950769);
    auVar21[10] = -(pcVar1[10] == UNK_14295076a);
    auVar21[0xb] = -(pcVar1[0xb] == UNK_14295076b);
    auVar21[0xc] = -(pcVar1[0xc] == UNK_14295076c);
    auVar21[0xd] = -(pcVar1[0xd] == UNK_14295076d);
    auVar21[0xe] = -(pcVar1[0xe] == UNK_14295076e);
    auVar21[0xf] = -(pcVar1[0xf] == UNK_14295076f);
    uVar7 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
            (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
    uVar8 = (uint)uVar7;
    while (uVar7 != 0) {
      uVar3 = 0;
      for (uVar5 = uVar8; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar3 = uVar3 + 1;
      }
      uVar14 = uVar3 + uVar9 & uVar11;
      lVar10 = uVar14 * -0x88;
      if ((*(longlong *)(lVar15 + -0x88 + lVar10) == 0x300777adfd693bf9) &&
         (*(longlong *)(lVar15 + -0x80 + lVar10) == 0x2a887be89e49c16b)) {
        lVar12 = ((longlong)(uVar14 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
        uVar9 = lVar12 - 0x10U & uVar11;
        pcVar1 = (char *)(lVar15 + uVar9);
        auVar16[0] = -(*pcVar1 == -1);
        auVar16[1] = -(pcVar1[1] == -1);
        auVar16[2] = -(pcVar1[2] == -1);
        auVar16[3] = -(pcVar1[3] == -1);
        auVar16[4] = -(pcVar1[4] == -1);
        auVar16[5] = -(pcVar1[5] == -1);
        auVar16[6] = -(pcVar1[6] == -1);
        auVar16[7] = -(pcVar1[7] == -1);
        auVar16[8] = -(pcVar1[8] == -1);
        auVar16[9] = -(pcVar1[9] == -1);
        auVar16[10] = -(pcVar1[10] == -1);
        auVar16[0xb] = -(pcVar1[0xb] == -1);
        auVar16[0xc] = -(pcVar1[0xc] == -1);
        auVar16[0xd] = -(pcVar1[0xd] == -1);
        auVar16[0xe] = -(pcVar1[0xe] == -1);
        auVar16[0xf] = -(pcVar1[0xf] == -1);
        uVar7 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
        pcVar1 = (char *)(lVar15 + lVar12);
        auVar17[0] = -(*pcVar1 == -1);
        auVar17[1] = -(pcVar1[1] == -1);
        auVar17[2] = -(pcVar1[2] == -1);
        auVar17[3] = -(pcVar1[3] == -1);
        auVar17[4] = -(pcVar1[4] == -1);
        auVar17[5] = -(pcVar1[5] == -1);
        auVar17[6] = -(pcVar1[6] == -1);
        auVar17[7] = -(pcVar1[7] == -1);
        auVar17[8] = -(pcVar1[8] == -1);
        auVar17[9] = -(pcVar1[9] == -1);
        auVar17[10] = -(pcVar1[10] == -1);
        auVar17[0xb] = -(pcVar1[0xb] == -1);
        auVar17[0xc] = -(pcVar1[0xc] == -1);
        auVar17[0xd] = -(pcVar1[0xd] == -1);
        auVar17[0xe] = -(pcVar1[0xe] == -1);
        auVar17[0xf] = -(pcVar1[0xf] == -1);
        if (uVar7 == 0) {
          uVar13 = 0x10;
        }
        else {
          uVar13 = 0xf;
          if (uVar7 != 0) {
            for (; uVar7 >> uVar13 == 0; uVar13 = uVar13 - 1) {
            }
          }
          uVar13 = uVar13 ^ 0xf;
        }
        lVar10 = lVar10 + lVar15;
        iVar4 = 0;
        for (uVar8 = (ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                             (ushort)(auVar17[0xf] >> 7) << 0xf) | 0x10000; (uVar8 & 1) == 0;
            uVar8 = uVar8 >> 1 | 0x80000000) {
          iVar4 = iVar4 + 1;
        }
        uVar6 = (uint3)((uint)iVar4 >> 8);
        uVar11 = CONCAT71((uint7)uVar6,0x80);
        if ((ushort)(uVar13 + (short)iVar4) < 0x10) {
          *(longlong *)(lVar2 + 0x90) = *(longlong *)(lVar2 + 0x90) + 1;
          uVar11 = (ulonglong)CONCAT31(uVar6,0xff);
        }
        *(char *)(lVar15 + lVar12) = (char)uVar11;
        *(char *)(lVar15 + 0x10 + uVar9) = (char)uVar11;
        *(longlong *)(lVar2 + 0x98) = *(longlong *)(lVar2 + 0x98) + -1;
        local_140 = *(longlong *)(lVar10 + -0x78);
        if (local_140 != 2) {
          local_d8 = *(undefined8 *)(lVar10 + -0x10);
          uStack_d0 = *(undefined8 *)(lVar10 + -8);
          local_e8 = *(undefined8 *)(lVar10 + -0x20);
          uStack_e0 = *(undefined8 *)(lVar10 + -0x18);
          local_f8 = *(undefined8 *)(lVar10 + -0x30);
          uStack_f0 = *(undefined8 *)(lVar10 + -0x28);
          local_138 = *(undefined8 *)(lVar10 + -0x70);
          uStack_130 = *(undefined8 *)(lVar10 + -0x68);
          local_128 = *(undefined8 *)(lVar10 + -0x60);
          uStack_120 = *(undefined8 *)(lVar10 + -0x58);
          local_118 = *(undefined8 *)(lVar10 + -0x50);
          uStack_110 = *(undefined8 *)(lVar10 + -0x48);
          local_108 = *(undefined8 *)(lVar10 + -0x40);
          uStack_100 = *(undefined8 *)(lVar10 + -0x38);
          QueuedRegistration::register();
          return;
        }
        goto LAB_142588ec8;
      }
      uVar7 = (ushort)(uVar8 - 1) & (ushort)uVar8;
      uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar7);
    }
    auVar19[0] = -(*pcVar1 == -1);
    auVar19[1] = -(pcVar1[1] == -1);
    auVar19[2] = -(pcVar1[2] == -1);
    auVar19[3] = -(pcVar1[3] == -1);
    auVar19[4] = -(pcVar1[4] == -1);
    auVar19[5] = -(pcVar1[5] == -1);
    auVar19[6] = -(pcVar1[6] == -1);
    auVar19[7] = -(pcVar1[7] == -1);
    auVar19[8] = -(pcVar1[8] == -1);
    auVar19[9] = -(pcVar1[9] == -1);
    auVar19[10] = -(pcVar1[10] == -1);
    auVar19[0xb] = -(pcVar1[0xb] == -1);
    auVar19[0xc] = -(pcVar1[0xc] == -1);
    auVar19[0xd] = -(pcVar1[0xd] == -1);
    auVar19[0xe] = -(pcVar1[0xe] == -1);
    auVar19[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar19 >> 0x37,0) & 1) != 0) || (SUB161(auVar19 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar19 >> 0x47,0) & 1) != 0) || (SUB161(auVar19 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar19 >> 0x57,0) & 1) != 0) || (SUB161(auVar19 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar19 >> 0x67,0) & 1) != 0) || (SUB161(auVar19 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar19 >> 0x77,0) & 1) != 0) || auVar19[0xf] < '\0') {
LAB_142588ec8:
      uVar14 = *(ulonglong *)in_RCX[1];
      uVar9 = uVar14 + 1;
      *(ulonglong *)in_RCX[1] = uVar9;
      _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                ();
      local_90 = 0x8000000000000000;
      local_88 = &anon_059ff0e347cc0501fbff78d46bf48f57_47_llvm_12238496627001195496;
      local_80 = 0x34;
      local_70 = 0x100;
      local_c8 = 1;
      local_c0 = 0x300777adfd693bf9;
      local_b8 = 0x2a887be89e49c16b;
      local_a0 = 8;
      local_98 = 0x18;
      local_78 = 0;
      local_6e = 1;
      local_a8 = uVar11;
      ComponentInfo::new();
      if (*(ulonglong *)(lVar2 + 0x10) < uVar9) {
        local_41 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar2 = *(longlong *)(lVar2 + 8);
      lVar15 = uVar14 * 0xd0;
      local_50 = (undefined8 *)(lVar2 + lVar15);
      memcpy(&local_2e8,&local_218,0xd0);
      if (*(int *)(lVar2 + lVar15) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_50,&local_2e8,0xd0);
      hashbrown::map::HashMap::
      insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
                ();
      local_208 = _DAT_1431019c0;
      uStack_200 = _UNK_1431019c8;
      local_218 = PTR_DAT_1431019b0;
      uStack_210 = _UNK_1431019b8;
      local_42 = 1;
      country_core::components::drawable::impl$9::register_required_components();
      local_2d8 = local_208;
      uStack_2d0 = uStack_200;
      local_2e8 = local_218;
      uStack_2e0 = uStack_210;
      local_50 = (undefined8 *)(*(longlong *)(*in_RCX + 8) + 0x60 + lVar15);
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<bevy_ecs::component::ComponentId,bevy_ecs::component::RequiredComponent>,allocator_api2::stable::alloc::global::Global>
                ();
      local_50[2] = local_2d8;
      local_50[3] = uStack_2d0;
      *local_50 = local_2e8;
      local_50[1] = uStack_2e0;
      if (local_68 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    lVar10 = uVar9 + lVar12;
    lVar12 = lVar12 + 0x10;
    uVar9 = lVar10 + 0x10U & uVar11;
  } while( true );
}

