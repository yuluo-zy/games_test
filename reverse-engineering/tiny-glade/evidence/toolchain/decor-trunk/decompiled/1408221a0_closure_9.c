// ADDR 0x1408221a0
// SIG void __cdecl closure$9(void)
// ORIG system::closure$9


void __cdecl system_decorator::audio::op_stairs_audio::op_stairs_audio::closure_9(void)

{
  char *pcVar1;
  longlong *plVar2;
  uint uVar3;
  uint uVar4;
  int extraout_EAX;
  int extraout_EAX_00;
  int extraout_EAX_01;
  int extraout_EAX_02;
  ulonglong extraout_RAX;
  longlong lVar5;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  ulonglong uVar6;
  uint uVar7;
  longlong lVar8;
  longlong lVar9;
  ushort uVar10;
  ulonglong uVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  
  plVar2 = (longlong *)*in_RCX;
  country_core::resources::walls::decorator::DecoratorOpEventKind::id();
  if (plVar2[3] == 0) {
    lVar9 = 0;
  }
  else {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar9 = *plVar2;
    bVar12 = (byte)(extraout_RAX >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar5 = 0;
    lVar8 = 0;
    uVar6 = extraout_RAX;
    while( true ) {
      uVar6 = plVar2[1] & uVar6;
      pcVar1 = (char *)(lVar9 + uVar6);
      cVar13 = auVar14[0];
      auVar22[0] = -(*pcVar1 == cVar13);
      cVar15 = auVar14[1];
      auVar22[1] = -(pcVar1[1] == cVar15);
      cVar16 = auVar14[2];
      auVar22[2] = -(pcVar1[2] == cVar16);
      cVar17 = auVar14[3];
      auVar22[3] = -(pcVar1[3] == cVar17);
      auVar22[4] = -(pcVar1[4] == cVar13);
      auVar22[5] = -(pcVar1[5] == cVar15);
      auVar22[6] = -(pcVar1[6] == cVar16);
      auVar22[7] = -(pcVar1[7] == cVar17);
      auVar22[8] = -(pcVar1[8] == cVar13);
      auVar22[9] = -(pcVar1[9] == cVar15);
      auVar22[10] = -(pcVar1[10] == cVar16);
      auVar22[0xb] = -(pcVar1[0xb] == cVar17);
      auVar22[0xc] = -(pcVar1[0xc] == cVar13);
      auVar22[0xd] = -(pcVar1[0xd] == cVar15);
      auVar22[0xe] = -(pcVar1[0xe] == cVar16);
      auVar22[0xf] = -(pcVar1[0xf] == cVar17);
      uVar10 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
      uVar7 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar11 = uVar3 + uVar6 & plVar2[1];
        if (extraout_EAX == *(int *)(lVar9 + uVar11 * -8 + -8)) {
          lVar5 = lVar9 + uVar11 * -8;
          goto LAB_140822275;
        }
        uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
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
      uVar6 = uVar6 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
LAB_140822275:
    lVar9 = lVar5 + -8;
    if (lVar5 == 0) {
      lVar9 = 0;
    }
  }
  plVar2 = (longlong *)in_RCX[2];
  country_core::resources::walls::decorator::DecoratorOpEventKind::id();
  if (plVar2[3] == 0) {
    return;
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  lVar5 = *plVar2;
  bVar12 = (byte)(extraout_RAX_00 >> 0x39);
  auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
  auVar14 = pshuflw(auVar14,auVar14,0);
  lVar8 = 0;
  uVar6 = extraout_RAX_00;
  do {
    uVar6 = plVar2[1] & uVar6;
    pcVar1 = (char *)(lVar5 + uVar6);
    cVar13 = auVar14[0];
    auVar23[0] = -(*pcVar1 == cVar13);
    cVar15 = auVar14[1];
    auVar23[1] = -(pcVar1[1] == cVar15);
    cVar16 = auVar14[2];
    auVar23[2] = -(pcVar1[2] == cVar16);
    cVar17 = auVar14[3];
    auVar23[3] = -(pcVar1[3] == cVar17);
    auVar23[4] = -(pcVar1[4] == cVar13);
    auVar23[5] = -(pcVar1[5] == cVar15);
    auVar23[6] = -(pcVar1[6] == cVar16);
    auVar23[7] = -(pcVar1[7] == cVar17);
    auVar23[8] = -(pcVar1[8] == cVar13);
    auVar23[9] = -(pcVar1[9] == cVar15);
    auVar23[10] = -(pcVar1[10] == cVar16);
    auVar23[0xb] = -(pcVar1[0xb] == cVar17);
    auVar23[0xc] = -(pcVar1[0xc] == cVar13);
    auVar23[0xd] = -(pcVar1[0xd] == cVar15);
    auVar23[0xe] = -(pcVar1[0xe] == cVar16);
    auVar23[0xf] = -(pcVar1[0xf] == cVar17);
    uVar10 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
    uVar7 = (uint)uVar10;
    while (uVar10 != 0) {
      uVar3 = 0;
      for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar3 = uVar3 + 1;
      }
      uVar11 = uVar3 + uVar6 & plVar2[1];
      if (extraout_EAX_00 == *(int *)(lVar5 + uVar11 * -8 + -8)) {
        if (lVar9 == 0) {
          return;
        }
        if (*(char *)(lVar9 + 4) == '\0') {
          return;
        }
        if (*(char *)(lVar5 + -4 + uVar11 * -8) != '\x01') {
          return;
        }
        plVar2 = (longlong *)in_RCX[3];
        country_core::resources::walls::decorator::DecoratorOpEventKind::id();
        if (plVar2[3] == 0) goto LAB_14082243d;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        bVar12 = (byte)(extraout_RAX_01 >> 0x39);
        auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
        auVar14 = pshuflw(auVar14,auVar14,0);
        lVar9 = 0;
        uVar6 = extraout_RAX_01;
        goto LAB_1408223c0;
      }
      uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
      uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
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
      return;
    }
    uVar6 = uVar6 + lVar8 + 0x10;
    lVar8 = lVar8 + 0x10;
  } while( true );
LAB_1408223c0:
  uVar6 = plVar2[1] & uVar6;
  pcVar1 = (char *)(*plVar2 + uVar6);
  cVar13 = auVar14[0];
  auVar24[0] = -(*pcVar1 == cVar13);
  cVar15 = auVar14[1];
  auVar24[1] = -(pcVar1[1] == cVar15);
  cVar16 = auVar14[2];
  auVar24[2] = -(pcVar1[2] == cVar16);
  cVar17 = auVar14[3];
  auVar24[3] = -(pcVar1[3] == cVar17);
  auVar24[4] = -(pcVar1[4] == cVar13);
  auVar24[5] = -(pcVar1[5] == cVar15);
  auVar24[6] = -(pcVar1[6] == cVar16);
  auVar24[7] = -(pcVar1[7] == cVar17);
  auVar24[8] = -(pcVar1[8] == cVar13);
  auVar24[9] = -(pcVar1[9] == cVar15);
  auVar24[10] = -(pcVar1[10] == cVar16);
  auVar24[0xb] = -(pcVar1[0xb] == cVar17);
  auVar24[0xc] = -(pcVar1[0xc] == cVar13);
  auVar24[0xd] = -(pcVar1[0xd] == cVar15);
  auVar24[0xe] = -(pcVar1[0xe] == cVar16);
  auVar24[0xf] = -(pcVar1[0xf] == cVar17);
  uVar10 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
  uVar7 = (uint)uVar10;
  while (uVar10 != 0) {
    uVar3 = 0;
    for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
      uVar3 = uVar3 + 1;
    }
    if (extraout_EAX_01 == *(int *)(*plVar2 + -0x10 + (uVar3 + uVar6 & plVar2[1]) * -0x10))
    goto LAB_14082243d;
    uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
    uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
  }
  auVar20[0] = -(*pcVar1 == -1);
  auVar20[1] = -(pcVar1[1] == -1);
  auVar20[2] = -(pcVar1[2] == -1);
  auVar20[3] = -(pcVar1[3] == -1);
  auVar20[4] = -(pcVar1[4] == -1);
  auVar20[5] = -(pcVar1[5] == -1);
  auVar20[6] = -(pcVar1[6] == -1);
  auVar20[7] = -(pcVar1[7] == -1);
  auVar20[8] = -(pcVar1[8] == -1);
  auVar20[9] = -(pcVar1[9] == -1);
  auVar20[10] = -(pcVar1[10] == -1);
  auVar20[0xb] = -(pcVar1[0xb] == -1);
  auVar20[0xc] = -(pcVar1[0xc] == -1);
  auVar20[0xd] = -(pcVar1[0xd] == -1);
  auVar20[0xe] = -(pcVar1[0xe] == -1);
  auVar20[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') {
LAB_14082243d:
    plVar2 = (longlong *)in_RCX[4];
    country_core::resources::walls::decorator::DecoratorOpEventKind::id();
    if (plVar2[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      bVar12 = (byte)(extraout_RAX_02 >> 0x39);
      auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
      auVar14 = pshuflw(auVar14,auVar14,0);
      lVar9 = 0;
      uVar6 = extraout_RAX_02;
      while( true ) {
        uVar6 = plVar2[1] & uVar6;
        pcVar1 = (char *)(*plVar2 + uVar6);
        cVar13 = auVar14[0];
        auVar25[0] = -(*pcVar1 == cVar13);
        cVar15 = auVar14[1];
        auVar25[1] = -(pcVar1[1] == cVar15);
        cVar16 = auVar14[2];
        auVar25[2] = -(pcVar1[2] == cVar16);
        cVar17 = auVar14[3];
        auVar25[3] = -(pcVar1[3] == cVar17);
        auVar25[4] = -(pcVar1[4] == cVar13);
        auVar25[5] = -(pcVar1[5] == cVar15);
        auVar25[6] = -(pcVar1[6] == cVar16);
        auVar25[7] = -(pcVar1[7] == cVar17);
        auVar25[8] = -(pcVar1[8] == cVar13);
        auVar25[9] = -(pcVar1[9] == cVar15);
        auVar25[10] = -(pcVar1[10] == cVar16);
        auVar25[0xb] = -(pcVar1[0xb] == cVar17);
        auVar25[0xc] = -(pcVar1[0xc] == cVar13);
        auVar25[0xd] = -(pcVar1[0xd] == cVar15);
        auVar25[0xe] = -(pcVar1[0xe] == cVar16);
        auVar25[0xf] = -(pcVar1[0xf] == cVar17);
        uVar10 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf
        ;
        uVar7 = (uint)uVar10;
        while (uVar10 != 0) {
          uVar3 = 0;
          for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
            uVar3 = uVar3 + 1;
          }
          if (extraout_EAX_02 == *(int *)(*plVar2 + -0x10 + (uVar3 + uVar6 & plVar2[1]) * -0x10)) {
            return;
          }
          uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
          uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
        }
        auVar21[0] = -(*pcVar1 == -1);
        auVar21[1] = -(pcVar1[1] == -1);
        auVar21[2] = -(pcVar1[2] == -1);
        auVar21[3] = -(pcVar1[3] == -1);
        auVar21[4] = -(pcVar1[4] == -1);
        auVar21[5] = -(pcVar1[5] == -1);
        auVar21[6] = -(pcVar1[6] == -1);
        auVar21[7] = -(pcVar1[7] == -1);
        auVar21[8] = -(pcVar1[8] == -1);
        auVar21[9] = -(pcVar1[9] == -1);
        auVar21[10] = -(pcVar1[10] == -1);
        auVar21[0xb] = -(pcVar1[0xb] == -1);
        auVar21[0xc] = -(pcVar1[0xc] == -1);
        auVar21[0xd] = -(pcVar1[0xd] == -1);
        auVar21[0xe] = -(pcVar1[0xe] == -1);
        auVar21[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x1f,0) & 1) != 0) || (SUB161(auVar21 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
        uVar6 = uVar6 + lVar9 + 0x10;
        lVar9 = lVar9 + 0x10;
      }
    }
    return;
  }
  uVar6 = uVar6 + lVar9 + 0x10;
  lVar9 = lVar9 + 0x10;
  goto LAB_1408223c0;
}


