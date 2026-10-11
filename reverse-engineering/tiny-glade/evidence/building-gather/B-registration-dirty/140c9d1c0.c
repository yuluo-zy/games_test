
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::components::transform::impl$4::register_required_components(void)

{
  char *pcVar1;
  ulonglong uVar2;
  longlong lVar3;
  undefined1 (*pauVar4) [16];
  short sVar5;
  uint uVar6;
  bool bVar7;
  char cVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  ulonglong extraout_RAX;
  undefined1 uVar23;
  byte bVar24;
  ushort uVar25;
  uint uVar26;
  uint uVar27;
  ulonglong *in_RDX;
  ulonglong uVar28;
  longlong lVar29;
  ulonglong uVar30;
  longlong lVar31;
  longlong lVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  ushort uVar35;
  bool bVar36;
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar45 [16];
  longlong *in_stack_00000028;
  undefined *local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined *local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  longlong local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  code *local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  char *local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined2 local_a0;
  undefined1 local_9e;
  longlong local_98;
  ulonglong local_90;
  longlong local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 *local_70;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  byte bVar44;
  
  local_60 = 0xfffffffffffffffe;
  uVar2 = *in_RDX;
  local_98 = in_stack_00000028[1];
  lVar3 = in_stack_00000028[2];
  bevy_ecs::component::enforce_no_required_components_recursion();
  bVar24 = UNK_14295049f;
  cVar22 = UNK_14295049e;
  cVar21 = UNK_14295049d;
  cVar20 = UNK_14295049c;
  cVar19 = UNK_14295049b;
  cVar18 = UNK_14295049a;
  cVar17 = UNK_142950499;
  cVar16 = UNK_142950498;
  cVar15 = UNK_142950497;
  cVar14 = UNK_142950496;
  cVar13 = UNK_142950495;
  cVar12 = UNK_142950494;
  cVar11 = UNK_142950493;
  cVar10 = UNK_142950492;
  cVar9 = UNK_142950491;
  cVar8 = __xmm_49494949494949494949494949494949;
  local_88 = 0;
  local_80 = 8;
  local_78 = 0;
  if (*(longlong *)(uVar2 + 0x30) != 0) {
    lVar32 = *(longlong *)(uVar2 + 0x18);
    uVar33 = *(ulonglong *)(uVar2 + 0x20);
    uVar28 = uVar33 & 0x9340ff6b57fed7e4;
    lVar31 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar32 + uVar28);
      auVar45[0] = -(*pcVar1 == __xmm_49494949494949494949494949494949);
      auVar45[1] = -(pcVar1[1] == UNK_142950491);
      auVar45[2] = -(pcVar1[2] == UNK_142950492);
      auVar45[3] = -(pcVar1[3] == UNK_142950493);
      auVar45[4] = -(pcVar1[4] == UNK_142950494);
      auVar45[5] = -(pcVar1[5] == UNK_142950495);
      auVar45[6] = -(pcVar1[6] == UNK_142950496);
      auVar45[7] = -(pcVar1[7] == UNK_142950497);
      auVar45[8] = -(pcVar1[8] == UNK_142950498);
      auVar45[9] = -(pcVar1[9] == UNK_142950499);
      auVar45[10] = -(pcVar1[10] == UNK_14295049a);
      auVar45[0xb] = -(pcVar1[0xb] == UNK_14295049b);
      auVar45[0xc] = -(pcVar1[0xc] == UNK_14295049c);
      auVar45[0xd] = -(pcVar1[0xd] == UNK_14295049d);
      auVar45[0xe] = -(pcVar1[0xe] == UNK_14295049e);
      auVar45[0xf] = -(pcVar1[0xf] == UNK_14295049f);
      uVar25 = (ushort)(SUB161(auVar45 >> 7,0) & 1) | (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe | (ushort)(auVar45[0xf] >> 7) << 0xf;
      uVar26 = (uint)uVar25;
      while (uVar25 != 0) {
        uVar27 = 0;
        for (uVar6 = uVar26; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar27 = uVar27 + 1;
        }
        uVar34 = uVar27 + uVar28 & uVar33;
        if ((*(longlong *)(lVar32 + -0x18 + uVar34 * -0x18) == -0x798dfbb777c9e9fc) &&
           (*(longlong *)(lVar32 + -0x10 + uVar34 * -0x18) == -0x6cbf0094a801281c)) {
          uVar33 = *(ulonglong *)(lVar32 + uVar34 * -0x18 + -8);
          lVar32 = *in_stack_00000028;
          goto joined_r0x000140c9d2e9;
        }
        uVar25 = (ushort)(uVar26 - 1) & (ushort)uVar26;
        uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar25);
      }
      auVar41[0] = -(*pcVar1 == -1);
      auVar41[1] = -(pcVar1[1] == -1);
      auVar41[2] = -(pcVar1[2] == -1);
      auVar41[3] = -(pcVar1[3] == -1);
      auVar41[4] = -(pcVar1[4] == -1);
      auVar41[5] = -(pcVar1[5] == -1);
      auVar41[6] = -(pcVar1[6] == -1);
      auVar41[7] = -(pcVar1[7] == -1);
      auVar41[8] = -(pcVar1[8] == -1);
      auVar41[9] = -(pcVar1[9] == -1);
      auVar41[10] = -(pcVar1[10] == -1);
      auVar41[0xb] = -(pcVar1[0xb] == -1);
      auVar41[0xc] = -(pcVar1[0xc] == -1);
      auVar41[0xd] = -(pcVar1[0xd] == -1);
      auVar41[0xe] = -(pcVar1[0xe] == -1);
      auVar41[0xf] = -(pcVar1[0xf] == 0xffU);
      if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar41 >> 0x17,0) & 1) != 0) || (SUB161(auVar41 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar41 >> 0x27,0) & 1) != 0) || (SUB161(auVar41 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar41 >> 0x4f,0) & 1) != 0) || (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar41 >> 0x5f,0) & 1) != 0) || (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0) ||
          auVar41[0xf] < '\0') break;
      lVar29 = uVar28 + lVar31;
      lVar31 = lVar31 + 0x10;
      uVar28 = lVar29 + 0x10U & uVar33;
    }
  }
  lVar32 = *(longlong *)(uVar2 + 0x80);
  uVar33 = *(ulonglong *)(uVar2 + 0x88);
  uVar28 = uVar33 & 0x9340ff6b57fed7e4;
  lVar31 = 0;
  do {
    pcVar1 = (char *)(lVar32 + uVar28);
    auVar42[0] = -(*pcVar1 == __xmm_49494949494949494949494949494949);
    auVar42[1] = -(pcVar1[1] == UNK_142950491);
    auVar42[2] = -(pcVar1[2] == UNK_142950492);
    auVar42[3] = -(pcVar1[3] == UNK_142950493);
    auVar42[4] = -(pcVar1[4] == UNK_142950494);
    auVar42[5] = -(pcVar1[5] == UNK_142950495);
    auVar42[6] = -(pcVar1[6] == UNK_142950496);
    auVar42[7] = -(pcVar1[7] == UNK_142950497);
    auVar42[8] = -(pcVar1[8] == UNK_142950498);
    auVar42[9] = -(pcVar1[9] == UNK_142950499);
    auVar42[10] = -(pcVar1[10] == UNK_14295049a);
    auVar42[0xb] = -(pcVar1[0xb] == UNK_14295049b);
    auVar42[0xc] = -(pcVar1[0xc] == UNK_14295049c);
    auVar42[0xd] = -(pcVar1[0xd] == UNK_14295049d);
    auVar42[0xe] = -(pcVar1[0xe] == UNK_14295049e);
    auVar42[0xf] = -(pcVar1[0xf] == UNK_14295049f);
    uVar25 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe | (ushort)(auVar42[0xf] >> 7) << 0xf;
    uVar26 = (uint)uVar25;
    if (uVar25 != 0) {
LAB_140c9d36d:
      uVar27 = 0;
      for (uVar6 = uVar26; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar27 = uVar27 + 1;
      }
      uVar34 = uVar27 + uVar28 & uVar33;
      lVar29 = uVar34 * -0x88;
      if ((*(longlong *)(lVar32 + -0x88 + lVar29) != -0x798dfbb777c9e9fc) ||
         (*(longlong *)(lVar32 + -0x80 + lVar29) != -0x6cbf0094a801281c)) break;
      lVar31 = ((longlong)(uVar34 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
      uVar33 = lVar31 - 0x10U & uVar33;
      pcVar1 = (char *)(lVar32 + uVar33);
      auVar37[0] = -(*pcVar1 == -1);
      auVar37[1] = -(pcVar1[1] == -1);
      auVar37[2] = -(pcVar1[2] == -1);
      auVar37[3] = -(pcVar1[3] == -1);
      auVar37[4] = -(pcVar1[4] == -1);
      auVar37[5] = -(pcVar1[5] == -1);
      auVar37[6] = -(pcVar1[6] == -1);
      auVar37[7] = -(pcVar1[7] == -1);
      auVar37[8] = -(pcVar1[8] == -1);
      auVar37[9] = -(pcVar1[9] == -1);
      auVar37[10] = -(pcVar1[10] == -1);
      auVar37[0xb] = -(pcVar1[0xb] == -1);
      auVar37[0xc] = -(pcVar1[0xc] == -1);
      auVar37[0xd] = -(pcVar1[0xd] == -1);
      auVar37[0xe] = -(pcVar1[0xe] == -1);
      auVar37[0xf] = -(pcVar1[0xf] == -1);
      uVar25 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf;
      pcVar1 = (char *)(lVar32 + lVar31);
      auVar38[0] = -(*pcVar1 == -1);
      auVar38[1] = -(pcVar1[1] == -1);
      auVar38[2] = -(pcVar1[2] == -1);
      auVar38[3] = -(pcVar1[3] == -1);
      auVar38[4] = -(pcVar1[4] == -1);
      auVar38[5] = -(pcVar1[5] == -1);
      auVar38[6] = -(pcVar1[6] == -1);
      auVar38[7] = -(pcVar1[7] == -1);
      auVar38[8] = -(pcVar1[8] == -1);
      auVar38[9] = -(pcVar1[9] == -1);
      auVar38[10] = -(pcVar1[10] == -1);
      auVar38[0xb] = -(pcVar1[0xb] == -1);
      auVar38[0xc] = -(pcVar1[0xc] == -1);
      auVar38[0xd] = -(pcVar1[0xd] == -1);
      auVar38[0xe] = -(pcVar1[0xe] == -1);
      auVar38[0xf] = -(pcVar1[0xf] == -1);
      if (uVar25 == 0) {
        uVar35 = 0x10;
      }
      else {
        uVar35 = 0xf;
        if (uVar25 != 0) {
          for (; uVar25 >> uVar35 == 0; uVar35 = uVar35 - 1) {
          }
        }
        uVar35 = uVar35 ^ 0xf;
      }
      lVar29 = lVar29 + lVar32;
      sVar5 = 0;
      for (uVar26 = (ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                            (ushort)(auVar38[0xf] >> 7) << 0xf) | 0x10000; (uVar26 & 1) == 0;
          uVar26 = uVar26 >> 1 | 0x80000000) {
        sVar5 = sVar5 + 1;
      }
      uVar23 = 0x80;
      if ((ushort)(uVar35 + sVar5) < 0x10) {
        *(longlong *)(uVar2 + 0x90) = *(longlong *)(uVar2 + 0x90) + 1;
        uVar23 = 0xff;
      }
      *(undefined1 *)(lVar32 + lVar31) = uVar23;
      *(undefined1 *)(lVar32 + 0x10 + uVar33) = uVar23;
      *(longlong *)(uVar2 + 0x98) = *(longlong *)(uVar2 + 0x98) + -1;
      if (*(longlong *)(lVar29 + -0x78) != 2) {
        local_108 = *(undefined8 *)(lVar29 + -0x10);
        uStack_100 = *(undefined8 *)(lVar29 + -8);
        local_118 = *(undefined8 *)(lVar29 + -0x20);
        uStack_110 = *(undefined8 *)(lVar29 + -0x18);
        local_128 = *(undefined8 *)(lVar29 + -0x30);
        uStack_120 = *(undefined8 *)(lVar29 + -0x28);
        local_168 = *(undefined8 *)(lVar29 + -0x70);
        uStack_160 = *(undefined8 *)(lVar29 + -0x68);
        local_158 = *(undefined8 *)(lVar29 + -0x60);
        uStack_150 = *(undefined8 *)(lVar29 + -0x58);
        local_148 = *(undefined8 *)(lVar29 + -0x50);
        uStack_140 = *(undefined8 *)(lVar29 + -0x48);
        local_138 = *(undefined8 *)(lVar29 + -0x40);
        uStack_130 = *(undefined8 *)(lVar29 + -0x38);
        local_170 = *(longlong *)(lVar29 + -0x78);
        bevy_ecs::component::QueuedRegistration::register();
        lVar32 = *in_stack_00000028;
        uVar33 = extraout_RAX;
        goto joined_r0x000140c9d2e9;
      }
      goto LAB_140c9d43a;
    }
LAB_140c9d337:
    auVar39[0] = -(*pcVar1 == -1);
    auVar39[1] = -(pcVar1[1] == -1);
    auVar39[2] = -(pcVar1[2] == -1);
    auVar39[3] = -(pcVar1[3] == -1);
    auVar39[4] = -(pcVar1[4] == -1);
    auVar39[5] = -(pcVar1[5] == -1);
    auVar39[6] = -(pcVar1[6] == -1);
    auVar39[7] = -(pcVar1[7] == -1);
    auVar39[8] = -(pcVar1[8] == -1);
    auVar39[9] = -(pcVar1[9] == -1);
    auVar39[10] = -(pcVar1[10] == -1);
    auVar39[0xb] = -(pcVar1[0xb] == -1);
    auVar39[0xc] = -(pcVar1[0xc] == -1);
    auVar39[0xd] = -(pcVar1[0xd] == -1);
    auVar39[0xe] = -(pcVar1[0xe] == -1);
    auVar39[0xf] = -(pcVar1[0xf] == 0xffU);
    if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 || (SUB161(auVar39 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar39 >> 0x17,0) & 1) != 0) || (SUB161(auVar39 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar39 >> 0x27,0) & 1) != 0) || (SUB161(auVar39 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar39 >> 0x37,0) & 1) != 0) || (SUB161(auVar39 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar39 >> 0x47,0) & 1) != 0) || (SUB161(auVar39 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar39 >> 0x57,0) & 1) != 0) || (SUB161(auVar39 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar39 >> 0x67,0) & 1) != 0) || (SUB161(auVar39 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar39 >> 0x77,0) & 1) != 0) || auVar39[0xf] < '\0') {
LAB_140c9d43a:
      uVar33 = *(ulonglong *)in_RDX[1];
      *(ulonglong *)in_RDX[1] = uVar33 + 1;
      local_c0 = 0x8000000000000000;
      local_b8 = 
      "utils::components::transform::Transform/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\slice.rs"
      ;
      local_b0 = 0x27;
      local_a0 = 0x100;
      local_f8 = 1;
      local_f0 = 0x8672044888361604;
      local_e8 = 0x9340ff6b57fed7e4;
      local_d0 = 0x10;
      local_c8 = 0x30;
      local_a8 = 0;
      local_9e = 1;
      local_e0 = 2;
      local_d8 = bevy_ecs::component::
                 component_clone_via_clone<utils::components::transform::Transform>;
      bevy_ecs::component::ComponentInfo::new();
      if (*(ulonglong *)(uVar2 + 0x10) < uVar33 + 1) {
        local_61 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar32 = *(longlong *)(uVar2 + 8);
      lVar31 = uVar33 * 0xd0;
      local_70 = (undefined8 *)(lVar32 + lVar31);
      memcpy(&local_318,&local_248,0xd0);
      if (*(int *)(lVar32 + lVar31) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_70,&local_318,0xd0);
      local_90 = *in_RDX;
      if (*(longlong *)(local_90 + 0x28) == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<core::any::TypeId,bevy_ecs::bundle::BundleId>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env_0<core::any::TypeId,bevy_ecs::bundle::BundleId,bevy_platform::hash::NoOpHash>_>
                  ();
      }
      pauVar4 = *(undefined1 (**) [16])(local_90 + 0x18);
      uVar2 = *(ulonglong *)(local_90 + 0x20);
      lVar32 = 0;
      uVar34 = 0x9340ff6b57fed7e4;
      bVar36 = false;
      uVar28 = local_90;
      while( true ) {
        uVar34 = uVar34 & uVar2;
        auVar45 = *(undefined1 (*) [16])(*pauVar4 + uVar34);
        auVar43[0] = -(auVar45[0] == cVar8);
        auVar43[1] = -(auVar45[1] == cVar9);
        auVar43[2] = -(auVar45[2] == cVar10);
        auVar43[3] = -(auVar45[3] == cVar11);
        auVar43[4] = -(auVar45[4] == cVar12);
        auVar43[5] = -(auVar45[5] == cVar13);
        auVar43[6] = -(auVar45[6] == cVar14);
        auVar43[7] = -(auVar45[7] == cVar15);
        auVar43[8] = -(auVar45[8] == cVar16);
        auVar43[9] = -(auVar45[9] == cVar17);
        auVar43[10] = -(auVar45[10] == cVar18);
        auVar43[0xb] = -(auVar45[0xb] == cVar19);
        auVar43[0xc] = -(auVar45[0xc] == cVar20);
        auVar43[0xd] = -(auVar45[0xd] == cVar21);
        bVar44 = auVar45[0xf];
        auVar43[0xe] = -(auVar45[0xe] == cVar22);
        auVar43[0xf] = -(bVar44 == bVar24);
        uVar25 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe | (ushort)(auVar43[0xf] >> 7) << 0xf
        ;
        uVar26 = (uint)uVar25;
        while (uVar25 != 0) {
          uVar27 = 0;
          for (uVar6 = uVar26; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar27 = uVar27 + 1;
          }
          uVar30 = uVar27 + uVar34 & uVar2;
          if ((*(longlong *)((longlong)pauVar4 + uVar30 * -0x18 + -0x18) == -0x798dfbb777c9e9fc) &&
             (*(longlong *)((longlong)pauVar4 + uVar30 * -0x18 + -0x10) == -0x6cbf0094a801281c))
          goto LAB_140c9d62d;
          uVar25 = (ushort)(uVar26 - 1) & (ushort)uVar26;
          uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar25);
        }
        bVar7 = !bVar36;
        bVar36 = true;
        if (bVar7) {
          uVar25 = (ushort)(SUB161(auVar45 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe | (ushort)(bVar44 >> 7) << 0xf;
          bVar36 = uVar25 != 0;
          uVar26 = 0;
          for (uVar27 = (uint)uVar25; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
            uVar26 = uVar26 + 1;
          }
          uVar28 = uVar26 + uVar34 & uVar2;
        }
        auVar40[0] = -(auVar45[0] == -1);
        auVar40[1] = -(auVar45[1] == -1);
        auVar40[2] = -(auVar45[2] == -1);
        auVar40[3] = -(auVar45[3] == -1);
        auVar40[4] = -(auVar45[4] == -1);
        auVar40[5] = -(auVar45[5] == -1);
        auVar40[6] = -(auVar45[6] == -1);
        auVar40[7] = -(auVar45[7] == -1);
        auVar40[8] = -(auVar45[8] == -1);
        auVar40[9] = -(auVar45[9] == -1);
        auVar40[10] = -(auVar45[10] == -1);
        auVar40[0xb] = -(auVar45[0xb] == -1);
        auVar40[0xc] = -(auVar45[0xc] == -1);
        auVar40[0xd] = -(auVar45[0xd] == -1);
        auVar40[0xe] = -(auVar45[0xe] == -1);
        auVar40[0xf] = -(bVar44 == 0xff);
        if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar40 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar40 >> 0x1f,0) & 1) != 0) || (SUB161(auVar40 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar40 >> 0x37,0) & 1) != 0) || (SUB161(auVar40 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar40 >> 0x47,0) & 1) != 0) || (SUB161(auVar40 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar40 >> 0x57,0) & 1) != 0) || (SUB161(auVar40 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar40 >> 0x67,0) & 1) != 0) || (SUB161(auVar40 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar40 >> 0x77,0) & 1) != 0) || auVar40[0xf] < '\0') break;
        uVar34 = uVar34 + lVar32 + 0x10;
        lVar32 = lVar32 + 0x10;
      }
      bVar24 = (*pauVar4)[uVar28];
      if (-1 < (char)bVar24) {
        auVar45 = *pauVar4;
        uVar26 = 0;
        for (uVar27 = (uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar45[0xf] >> 7) << 0xf); (uVar27 & 1) == 0;
            uVar27 = uVar27 >> 1 | 0x80000000) {
          uVar26 = uVar26 + 1;
        }
        uVar28 = (ulonglong)uVar26;
        bVar24 = (*pauVar4)[uVar28];
      }
      *(longlong *)(local_90 + 0x28) = *(longlong *)(local_90 + 0x28) - (ulonglong)(bVar24 & 1);
      (*pauVar4)[uVar28] = 0x49;
      pauVar4[1][uVar28 - 0x10 & uVar2] = 0x49;
      *(longlong *)(local_90 + 0x30) = *(longlong *)(local_90 + 0x30) + 1;
      *(undefined8 *)((longlong)pauVar4 + uVar28 * -0x18 + -0x18) = 0x8672044888361604;
      *(undefined8 *)((longlong)pauVar4 + uVar28 * -0x18 + -0x10) = 0x9340ff6b57fed7e4;
      uVar30 = uVar28;
LAB_140c9d62d:
      *(ulonglong *)((longlong)pauVar4 + uVar30 * -0x18 + -8) = uVar33;
      local_238 = _DAT_142b2eca0;
      uStack_230 = _UNK_142b2eca8;
      local_248 = PTR_DAT_142b2ec90;
      uStack_240 = _UNK_142b2ec98;
      local_62 = 1;
      local_70 = (undefined8 *)lVar31;
      register_required_components();
      local_308 = local_238;
      uStack_300 = uStack_230;
      local_318 = local_248;
      uStack_310 = uStack_240;
      local_70 = (undefined8 *)(*(longlong *)(*in_RDX + 8) + 0x60 + (longlong)local_70);
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<bevy_ecs::component::ComponentId,bevy_ecs::component::RequiredComponent>,allocator_api2::stable::alloc::global::Global>
                ();
      local_70[2] = local_308;
      local_70[3] = uStack_300;
      *local_70 = local_318;
      local_70[1] = uStack_310;
      if (local_88 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar32 = *in_stack_00000028;
joined_r0x000140c9d2e9:
      if (lVar3 == lVar32) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        *(ulonglong *)(in_stack_00000028[1] + lVar3 * 8) = uVar33;
        in_stack_00000028[2] = lVar3 + 1;
      }
      else {
        *(ulonglong *)(local_98 + lVar3 * 8) = uVar33;
        in_stack_00000028[2] = lVar3 + 1;
        if (lVar3 + 1 == 0) {
          return;
        }
      }
      in_stack_00000028[2] = lVar3;
      return;
    }
    lVar29 = uVar28 + lVar31;
    lVar31 = lVar31 + 0x10;
    uVar28 = lVar29 + 0x10U & uVar33;
  } while( true );
  uVar25 = (ushort)(uVar26 - 1) & (ushort)uVar26;
  uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar25);
  if (uVar25 == 0) goto LAB_140c9d337;
  goto LAB_140c9d36d;
}

