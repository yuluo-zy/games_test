// detect_konami_code @ 0x14024b370
// sig: void __cdecl detect_konami_code(void)
// size: 330


void __cdecl tiny_glade::gui::settings::detect_konami_code(void)

{
  undefined1 auVar1 [16];
  uint uVar2;
  uint uVar3;
  longlong lVar4;
  longlong *in_RCX;
  undefined1 (*pauVar5) [16];
  ulonglong *in_RDX;
  uint uVar6;
  undefined1 (*pauVar7) [16];
  ulonglong uVar8;
  ushort auStack_98 [4];
  undefined4 local_90;
  undefined4 local_84;
  undefined4 local_78;
  undefined4 local_6c;
  undefined4 local_60;
  undefined4 local_54;
  undefined4 local_48;
  undefined4 local_3c;
  undefined4 local_30;
  undefined4 local_24;
  
  local_90 = 0x53;
  local_84 = 0x50;
  local_78 = 0x50;
  local_6c = 0x51;
  local_60 = 0x52;
  local_54 = 0x51;
  local_48 = 0x52;
  local_3c = 0x15;
  local_30 = 0x14;
  local_24 = 0x3a;
  lVar4 = *(longlong *)(*in_RCX + 0x38);
  if (lVar4 != 0) {
    pauVar5 = *(undefined1 (**) [16])(*in_RCX + 0x20);
    pauVar7 = pauVar5 + 1;
    auVar1 = *pauVar5;
    uVar6 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
    uVar8 = *in_RDX;
    do {
      if ((short)uVar6 == 0) {
        do {
          auVar1 = *pauVar7;
          pauVar5 = pauVar5 + -0xc;
          pauVar7 = pauVar7 + 1;
          uVar6 = (uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
        } while (uVar6 == 0xffff);
        uVar6 = ~uVar6;
      }
      if (uVar8 < 0xb) {
        uVar2 = 0;
        for (uVar3 = uVar6; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        if (*(int *)(auStack_98 + uVar8 * 6 + -2) ==
            *(int *)((longlong)pauVar5 + (ulonglong)uVar2 * -0xc + -0xc)) {
          if (*(int *)(auStack_98 + uVar8 * 6 + -2) != 0) {
            uVar8 = uVar8 + 1;
            *in_RDX = uVar8;
            if (uVar8 == 0xb) {
              SHOW_DEVELOPER_SETTINGS = 1;
              uVar8 = 0xb;
            }
            goto LAB_14024b40a;
          }
          if (auStack_98[uVar8 * 6] == *(ushort *)((longlong)pauVar5 + (ulonglong)uVar2 * -0xc + -8)
             ) {
                    // WARNING: Could not recover jumptable at 0x00014024b4b3. Too many branches
                    // WARNING: Treating indirect jump as call
            (*(code *)(&DAT_1429a1754 +
                      *(int *)(&DAT_1429a1754 + (ulonglong)auStack_98[uVar8 * 6] * 4)))();
            return;
          }
        }
        *in_RDX = 0;
        uVar8 = 0;
      }
LAB_14024b40a:
      uVar6 = uVar6 - 1 & uVar6;
      lVar4 = lVar4 + -1;
    } while (lVar4 != 0);
  }
  return;
}

