
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
bevy_ecs::component::ComponentsRegistrator::
register_component<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData>_>
          (void)

{
  char *pcVar1;
  longlong lVar2;
  undefined1 (*pauVar3) [16];
  short sVar4;
  uint uVar5;
  bool bVar6;
  char cVar7;
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
  undefined8 extraout_RAX;
  longlong *in_RCX;
  uint uVar22;
  undefined **_Src;
  undefined1 uVar23;
  byte bVar24;
  uint uVar25;
  ulonglong uVar26;
  longlong lVar27;
  longlong lVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  ushort uVar31;
  undefined **ppuVar32;
  ushort uVar33;
  longlong lVar34;
  bool bVar35;
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar44 [16];
  undefined *local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined *local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  longlong local_160;
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
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined *local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined2 local_90;
  undefined1 local_8e;
  longlong local_88;
  undefined8 *local_80;
  undefined8 local_78;
  undefined8 *local_70;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  byte bVar43;
  
  bVar24 = UNK_14295062f;
  cVar21 = UNK_14295062e;
  cVar20 = UNK_14295062d;
  cVar19 = UNK_14295062c;
  cVar18 = UNK_14295062b;
  cVar17 = UNK_14295062a;
  cVar16 = UNK_142950629;
  cVar15 = UNK_142950628;
  cVar14 = UNK_142950627;
  cVar13 = UNK_142950626;
  cVar12 = UNK_142950625;
  cVar11 = UNK_142950624;
  cVar10 = UNK_142950623;
  cVar9 = UNK_142950622;
  cVar8 = UNK_142950621;
  cVar7 = __xmm_0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f;
  local_60 = 0xfffffffffffffffe;
  local_88 = 0;
  local_80 = (undefined8 *)&DAT_00000008;
  local_78 = 0;
  lVar2 = *in_RCX;
  if (*(longlong *)(lVar2 + 0x30) != 0) {
    lVar34 = *(longlong *)(lVar2 + 0x18);
    uVar30 = *(ulonglong *)(lVar2 + 0x20);
    uVar26 = uVar30 & 0x1e41acc132caf343;
    lVar28 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar34 + uVar26);
      auVar44[0] = -(*pcVar1 == __xmm_0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f);
      auVar44[1] = -(pcVar1[1] == UNK_142950621);
      auVar44[2] = -(pcVar1[2] == UNK_142950622);
      auVar44[3] = -(pcVar1[3] == UNK_142950623);
      auVar44[4] = -(pcVar1[4] == UNK_142950624);
      auVar44[5] = -(pcVar1[5] == UNK_142950625);
      auVar44[6] = -(pcVar1[6] == UNK_142950626);
      auVar44[7] = -(pcVar1[7] == UNK_142950627);
      auVar44[8] = -(pcVar1[8] == UNK_142950628);
      auVar44[9] = -(pcVar1[9] == UNK_142950629);
      auVar44[10] = -(pcVar1[10] == UNK_14295062a);
      auVar44[0xb] = -(pcVar1[0xb] == UNK_14295062b);
      auVar44[0xc] = -(pcVar1[0xc] == UNK_14295062c);
      auVar44[0xd] = -(pcVar1[0xd] == UNK_14295062d);
      auVar44[0xe] = -(pcVar1[0xe] == UNK_14295062e);
      auVar44[0xf] = -(pcVar1[0xf] == UNK_14295062f);
      uVar33 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(auVar44[0xf] >> 7) << 0xf;
      uVar25 = (uint)uVar33;
      while (uVar33 != 0) {
        uVar22 = 0;
        for (uVar5 = uVar25; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar22 = uVar22 + 1;
        }
        uVar29 = uVar22 + uVar26 & uVar30;
        if ((*(longlong *)(lVar34 + -0x18 + uVar29 * -0x18) == -0x2664691c74cae463) &&
           (*(longlong *)(lVar34 + -0x10 + uVar29 * -0x18) == 0x1e41acc132caf343)) {
          return;
        }
        uVar33 = (ushort)(uVar25 - 1) & (ushort)uVar25;
        uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar33);
      }
      auVar40[0] = -(*pcVar1 == -1);
      auVar40[1] = -(pcVar1[1] == -1);
      auVar40[2] = -(pcVar1[2] == -1);
      auVar40[3] = -(pcVar1[3] == -1);
      auVar40[4] = -(pcVar1[4] == -1);
      auVar40[5] = -(pcVar1[5] == -1);
      auVar40[6] = -(pcVar1[6] == -1);
      auVar40[7] = -(pcVar1[7] == -1);
      auVar40[8] = -(pcVar1[8] == -1);
      auVar40[9] = -(pcVar1[9] == -1);
      auVar40[10] = -(pcVar1[10] == -1);
      auVar40[0xb] = -(pcVar1[0xb] == -1);
      auVar40[0xc] = -(pcVar1[0xc] == -1);
      auVar40[0xd] = -(pcVar1[0xd] == -1);
      auVar40[0xe] = -(pcVar1[0xe] == -1);
      auVar40[0xf] = -(pcVar1[0xf] == 0xffU);
      if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar40 >> 0x17,0) & 1) != 0) || (SUB161(auVar40 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar40 >> 0x27,0) & 1) != 0) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar40 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar40 >> 0x3f,0) & 1) != 0) || (SUB161(auVar40 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar40 >> 0x4f,0) & 1) != 0) || (SUB161(auVar40 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar40 >> 0x5f,0) & 1) != 0) || (SUB161(auVar40 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar40 >> 0x6f,0) & 1) != 0) || (SUB161(auVar40 >> 0x77,0) & 1) != 0) ||
          auVar40[0xf] < '\0') break;
      lVar27 = uVar26 + lVar28;
      lVar28 = lVar28 + 0x10;
      uVar26 = lVar27 + 0x10U & uVar30;
    }
  }
  lVar34 = *(longlong *)(lVar2 + 0x80);
  uVar30 = *(ulonglong *)(lVar2 + 0x88);
  uVar26 = uVar30 & 0x1e41acc132caf343;
  lVar28 = 0;
  do {
    pcVar1 = (char *)(lVar34 + uVar26);
    auVar41[0] = -(*pcVar1 == __xmm_0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f);
    auVar41[1] = -(pcVar1[1] == UNK_142950621);
    auVar41[2] = -(pcVar1[2] == UNK_142950622);
    auVar41[3] = -(pcVar1[3] == UNK_142950623);
    auVar41[4] = -(pcVar1[4] == UNK_142950624);
    auVar41[5] = -(pcVar1[5] == UNK_142950625);
    auVar41[6] = -(pcVar1[6] == UNK_142950626);
    auVar41[7] = -(pcVar1[7] == UNK_142950627);
    auVar41[8] = -(pcVar1[8] == UNK_142950628);
    auVar41[9] = -(pcVar1[9] == UNK_142950629);
    auVar41[10] = -(pcVar1[10] == UNK_14295062a);
    auVar41[0xb] = -(pcVar1[0xb] == UNK_14295062b);
    auVar41[0xc] = -(pcVar1[0xc] == UNK_14295062c);
    auVar41[0xd] = -(pcVar1[0xd] == UNK_14295062d);
    auVar41[0xe] = -(pcVar1[0xe] == UNK_14295062e);
    auVar41[0xf] = -(pcVar1[0xf] == UNK_14295062f);
    uVar33 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf;
    uVar25 = (uint)uVar33;
    if (uVar33 != 0) {
LAB_142818a3d:
      uVar22 = 0;
      for (uVar5 = uVar25; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar22 = uVar22 + 1;
      }
      uVar29 = uVar22 + uVar26 & uVar30;
      lVar27 = uVar29 * -0x88;
      if ((*(longlong *)(lVar34 + -0x88 + lVar27) != -0x2664691c74cae463) ||
         (*(longlong *)(lVar34 + -0x80 + lVar27) != 0x1e41acc132caf343)) break;
      lVar28 = ((longlong)(uVar29 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
      uVar30 = lVar28 - 0x10U & uVar30;
      pcVar1 = (char *)(lVar34 + uVar30);
      auVar36[0] = -(*pcVar1 == -1);
      auVar36[1] = -(pcVar1[1] == -1);
      auVar36[2] = -(pcVar1[2] == -1);
      auVar36[3] = -(pcVar1[3] == -1);
      auVar36[4] = -(pcVar1[4] == -1);
      auVar36[5] = -(pcVar1[5] == -1);
      auVar36[6] = -(pcVar1[6] == -1);
      auVar36[7] = -(pcVar1[7] == -1);
      auVar36[8] = -(pcVar1[8] == -1);
      auVar36[9] = -(pcVar1[9] == -1);
      auVar36[10] = -(pcVar1[10] == -1);
      auVar36[0xb] = -(pcVar1[0xb] == -1);
      auVar36[0xc] = -(pcVar1[0xc] == -1);
      auVar36[0xd] = -(pcVar1[0xd] == -1);
      auVar36[0xe] = -(pcVar1[0xe] == -1);
      auVar36[0xf] = -(pcVar1[0xf] == -1);
      uVar33 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe | (ushort)(auVar36[0xf] >> 7) << 0xf;
      pcVar1 = (char *)(lVar34 + lVar28);
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
      if (uVar33 == 0) {
        uVar31 = 0x10;
      }
      else {
        uVar31 = 0xf;
        if (uVar33 != 0) {
          for (; uVar33 >> uVar31 == 0; uVar31 = uVar31 - 1) {
          }
        }
        uVar31 = uVar31 ^ 0xf;
      }
      lVar27 = lVar27 + lVar34;
      sVar4 = 0;
      for (uVar25 = (ushort)((ushort)(SUB161(auVar37 >> 7,0) & 1) |
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
                            (ushort)(auVar37[0xf] >> 7) << 0xf) | 0x10000; (uVar25 & 1) == 0;
          uVar25 = uVar25 >> 1 | 0x80000000) {
        sVar4 = sVar4 + 1;
      }
      uVar23 = 0x80;
      if ((ushort)(uVar31 + sVar4) < 0x10) {
        *(longlong *)(lVar2 + 0x90) = *(longlong *)(lVar2 + 0x90) + 1;
        uVar23 = 0xff;
      }
      *(undefined1 *)(lVar34 + lVar28) = uVar23;
      *(undefined1 *)(lVar34 + 0x10 + uVar30) = uVar23;
      *(longlong *)(lVar2 + 0x98) = *(longlong *)(lVar2 + 0x98) + -1;
      local_160 = *(longlong *)(lVar27 + -0x78);
      if (local_160 != 2) {
        local_f8 = *(undefined8 *)(lVar27 + -0x10);
        uStack_f0 = *(undefined8 *)(lVar27 + -8);
        local_108 = *(undefined8 *)(lVar27 + -0x20);
        uStack_100 = *(undefined8 *)(lVar27 + -0x18);
        local_118 = *(undefined8 *)(lVar27 + -0x30);
        uStack_110 = *(undefined8 *)(lVar27 + -0x28);
        local_158 = *(undefined8 *)(lVar27 + -0x70);
        uStack_150 = *(undefined8 *)(lVar27 + -0x68);
        local_148 = *(undefined8 *)(lVar27 + -0x60);
        uStack_140 = *(undefined8 *)(lVar27 + -0x58);
        local_138 = *(undefined8 *)(lVar27 + -0x50);
        uStack_130 = *(undefined8 *)(lVar27 + -0x48);
        local_128 = *(undefined8 *)(lVar27 + -0x40);
        uStack_120 = *(undefined8 *)(lVar27 + -0x38);
        QueuedRegistration::register();
        return;
      }
      goto LAB_142818b0d;
    }
LAB_142818a09:
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
    auVar38[0xf] = -(pcVar1[0xf] == 0xffU);
    if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') {
LAB_142818b0d:
      uVar30 = *(ulonglong *)in_RCX[1];
      *(ulonglong *)in_RCX[1] = uVar30 + 1;
      lVar2 = *in_RCX;
      local_b0 = 0x8000000000000000;
      local_a8 = &anon_059ff0e347cc0501fbff78d46bf48f57_52_llvm_12238496627001195496;
      local_a0 = 0x7c;
      local_90 = 0x100;
      local_e8 = 1;
      local_e0 = 0xd99b96e38b351b9d;
      local_d8 = 0x1e41acc132caf343;
      local_c0 = 1;
      local_b8 = 0;
      local_98 = 0;
      local_8e = 1;
      local_d0 = 0;
      ComponentInfo::new();
      if (*(ulonglong *)(lVar2 + 0x10) < uVar30 + 1) {
        local_62 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar2 = *(longlong *)(lVar2 + 8);
      lVar34 = uVar30 * 0xd0;
      local_70 = (undefined8 *)(lVar2 + lVar34);
      memcpy(&local_308,&local_238,0xd0);
      if (*(int *)(lVar2 + lVar34) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      _Src = &local_308;
      memcpy(local_70,_Src,0xd0);
      lVar2 = *in_RCX;
      if (*(longlong *)(lVar2 + 0x28) == 0) {
        _Src = (undefined **)0x1;
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<core::any::TypeId,bevy_ecs::bundle::BundleId>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env_0<core::any::TypeId,bevy_ecs::bundle::BundleId,bevy_platform::hash::NoOpHash>_>
                  ();
      }
      local_70 = (undefined8 *)lVar34;
      pauVar3 = *(undefined1 (**) [16])(lVar2 + 0x18);
      uVar26 = *(ulonglong *)(lVar2 + 0x20);
      lVar34 = 0;
      uVar29 = 0x1e41acc132caf343;
      bVar35 = false;
      while( true ) {
        uVar29 = uVar29 & uVar26;
        auVar44 = *(undefined1 (*) [16])(*pauVar3 + uVar29);
        auVar42[0] = -(auVar44[0] == cVar7);
        auVar42[1] = -(auVar44[1] == cVar8);
        auVar42[2] = -(auVar44[2] == cVar9);
        auVar42[3] = -(auVar44[3] == cVar10);
        auVar42[4] = -(auVar44[4] == cVar11);
        auVar42[5] = -(auVar44[5] == cVar12);
        auVar42[6] = -(auVar44[6] == cVar13);
        auVar42[7] = -(auVar44[7] == cVar14);
        auVar42[8] = -(auVar44[8] == cVar15);
        auVar42[9] = -(auVar44[9] == cVar16);
        auVar42[10] = -(auVar44[10] == cVar17);
        auVar42[0xb] = -(auVar44[0xb] == cVar18);
        auVar42[0xc] = -(auVar44[0xc] == cVar19);
        auVar42[0xd] = -(auVar44[0xd] == cVar20);
        bVar43 = auVar44[0xf];
        auVar42[0xe] = -(auVar44[0xe] == cVar21);
        auVar42[0xf] = -(bVar43 == bVar24);
        uVar33 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe | (ushort)(auVar42[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar33;
        while (uVar33 != 0) {
          uVar22 = 0;
          for (uVar5 = uVar25; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar22 = uVar22 + 1;
          }
          ppuVar32 = (undefined **)(uVar22 + uVar29 & uVar26);
          if ((*(longlong *)((longlong)pauVar3 + (longlong)ppuVar32 * -0x18 + -0x18) ==
               -0x2664691c74cae463) &&
             (*(longlong *)((longlong)pauVar3 + (longlong)ppuVar32 * -0x18 + -0x10) ==
              0x1e41acc132caf343)) goto LAB_142818ce4;
          uVar33 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar33);
        }
        bVar6 = !bVar35;
        bVar35 = true;
        if (bVar6) {
          uVar33 = (ushort)(SUB161(auVar44 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(bVar43 >> 7) << 0xf;
          bVar35 = uVar33 != 0;
          uVar25 = 0;
          for (uVar22 = (uint)uVar33; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
            uVar25 = uVar25 + 1;
          }
          _Src = (undefined **)(uVar25 + uVar29 & uVar26);
        }
        auVar39[0] = -(auVar44[0] == -1);
        auVar39[1] = -(auVar44[1] == -1);
        auVar39[2] = -(auVar44[2] == -1);
        auVar39[3] = -(auVar44[3] == -1);
        auVar39[4] = -(auVar44[4] == -1);
        auVar39[5] = -(auVar44[5] == -1);
        auVar39[6] = -(auVar44[6] == -1);
        auVar39[7] = -(auVar44[7] == -1);
        auVar39[8] = -(auVar44[8] == -1);
        auVar39[9] = -(auVar44[9] == -1);
        auVar39[10] = -(auVar44[10] == -1);
        auVar39[0xb] = -(auVar44[0xb] == -1);
        auVar39[0xc] = -(auVar44[0xc] == -1);
        auVar39[0xd] = -(auVar44[0xd] == -1);
        auVar39[0xe] = -(auVar44[0xe] == -1);
        auVar39[0xf] = -(bVar43 == 0xff);
        if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 || (SUB161(auVar39 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar39 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar39 >> 0x1f,0) & 1) != 0) || (SUB161(auVar39 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar39 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar39 >> 0x37,0) & 1) != 0) || (SUB161(auVar39 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar39 >> 0x47,0) & 1) != 0) || (SUB161(auVar39 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar39 >> 0x57,0) & 1) != 0) || (SUB161(auVar39 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar39 >> 0x67,0) & 1) != 0) || (SUB161(auVar39 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar39 >> 0x77,0) & 1) != 0) || auVar39[0xf] < '\0') break;
        uVar29 = uVar29 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
      bVar24 = (*pauVar3)[(longlong)_Src];
      if (-1 < (char)bVar24) {
        auVar44 = *pauVar3;
        uVar25 = 0;
        for (uVar22 = (uint)(ushort)((ushort)(SUB161(auVar44 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar44[0xf] >> 7) << 0xf); (uVar22 & 1) == 0;
            uVar22 = uVar22 >> 1 | 0x80000000) {
          uVar25 = uVar25 + 1;
        }
        _Src = (undefined **)(ulonglong)uVar25;
        bVar24 = (*pauVar3)[(longlong)_Src];
      }
      *(longlong *)(lVar2 + 0x28) = *(longlong *)(lVar2 + 0x28) - (ulonglong)(bVar24 & 1);
      (*pauVar3)[(longlong)_Src] = 0xf;
      pauVar3[1][(ulonglong)(_Src + -2) & uVar26] = 0xf;
      *(longlong *)(lVar2 + 0x30) = *(longlong *)(lVar2 + 0x30) + 1;
      *(undefined8 *)((longlong)pauVar3 + (longlong)_Src * -0x18 + -0x18) = 0xd99b96e38b351b9d;
      *(undefined8 *)((longlong)pauVar3 + (longlong)_Src * -0x18 + -0x10) = 0x1e41acc132caf343;
      ppuVar32 = _Src;
LAB_142818ce4:
      *(ulonglong *)((longlong)pauVar3 + (longlong)ppuVar32 * -0x18 + -8) = uVar30;
      local_228 = _DAT_143162ee0;
      uStack_220 = _UNK_143162ee8;
      local_238 = PTR_DAT_143162ed0;
      uStack_230 = _UNK_143162ed8;
      local_61 = 1;
      enforce_no_required_components_recursion();
      local_61 = 1;
      register_component<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData>_>
                ();
      local_61 = 1;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      *local_80 = extraout_RAX;
      local_78 = 0;
      local_2f8 = local_228;
      uStack_2f0 = uStack_220;
      local_308 = local_238;
      uStack_300 = uStack_230;
      local_70 = (undefined8 *)(*(longlong *)(*in_RCX + 8) + 0x60 + (longlong)local_70);
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<bevy_ecs::component::ComponentId,bevy_ecs::component::RequiredComponent>,allocator_api2::stable::alloc::global::Global>
                ();
      local_70[2] = local_2f8;
      local_70[3] = uStack_2f0;
      *local_70 = local_308;
      local_70[1] = uStack_300;
      if (local_88 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    lVar27 = uVar26 + lVar28;
    lVar28 = lVar28 + 0x10;
    uVar26 = lVar27 + 0x10U & uVar30;
  } while( true );
  uVar33 = (ushort)(uVar25 - 1) & (ushort)uVar25;
  uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar33);
  if (uVar33 == 0) goto LAB_142818a09;
  goto LAB_142818a3d;
}

