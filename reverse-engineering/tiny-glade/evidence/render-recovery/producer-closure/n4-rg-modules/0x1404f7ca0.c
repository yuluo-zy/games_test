
longlong FUN_1404f7ca0(longlong param_1,longlong param_2)

{
  undefined1 auVar1 [16];
  int iVar2;
  ushort uVar3;
  uint uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  undefined1 (*pauVar10) [16];
  longlong lVar11;
  undefined1 (*pauVar12) [16];
  undefined1 auStack_190 [168];
  undefined1 (*pauStack_e8) [16];
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  longlong lStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_90;
  longlong lStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined8 uStack_64;
  undefined8 uStack_5c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_80 = param_1;
  func_0x000142923c00(auStack_190,param_2,0xa8);
  pauVar12 = *(undefined1 (**) [16])(param_2 + 0xb0);
  uStack_e0 = *(undefined8 *)(param_2 + 0xb8);
  uStack_d8 = *(undefined8 *)(param_2 + 0xc0);
  lVar11 = *(longlong *)(param_2 + 200);
  uStack_c8 = *(undefined8 *)(param_2 + 0xd0);
  uStack_c0 = *(undefined8 *)(param_2 + 0xd8);
  uStack_b8 = *(undefined8 *)(param_2 + 0xe0);
  uStack_b0 = *(undefined8 *)(param_2 + 0xe8);
  uStack_98 = *(undefined8 *)(param_2 + 0x100);
  uStack_90 = *(undefined8 *)(param_2 + 0x108);
  uStack_a8 = *(undefined4 *)(param_2 + 0xf0);
  uStack_a4 = *(undefined4 *)(param_2 + 0xf4);
  uStack_a0 = *(undefined4 *)(param_2 + 0xf8);
  uStack_9c = *(undefined4 *)(param_2 + 0xfc);
  pauStack_e8 = pauVar12;
  lStack_d0 = lVar11;
  if (lVar11 != 0) {
    pauVar10 = pauVar12 + 1;
    auVar1 = *pauVar12;
    uVar8 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
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
    do {
      if ((short)uVar8 == 0) {
        do {
          auVar1 = *pauVar10;
          uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          pauVar12 = pauVar12 + -0x68;
          pauVar10 = pauVar10 + 1;
        } while (uVar3 == 0xffff);
        uVar9 = ~(uint)uVar3;
        uVar8 = uVar9 & -(uint)uVar3 - 2;
      }
      else {
        uVar9 = uVar8;
        uVar8 = uVar8 - 1 & uVar8;
      }
      uVar4 = 0;
      for (; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      lVar7 = (ulonglong)uVar4 * -0x68;
      iVar2 = *(int *)((longlong)pauVar12 + lVar7 + -0x60);
      if (iVar2 == 0) {
        uStack_78 = *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x5c);
        uStack_64 = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x48);
        uStack_5c = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x40);
        uVar5 = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x58);
        uVar6 = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x50);
        uStack_74 = (undefined4)uVar5;
        uStack_70 = (undefined4)((ulonglong)uVar5 >> 0x20);
        uStack_6c = (undefined4)uVar6;
        uStack_68 = (undefined4)((ulonglong)uVar6 >> 0x20);
        FUN_140bd6380(&uStack_78,auStack_190,0x37);
code_r0x0001404f7d90:
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x60) = 2;
        *(undefined1 *)((longlong)pauVar12 + lVar7 + -0x5c) = 0x37;
        auVar1._4_4_ = uStack_74;
        auVar1._0_4_ = uStack_78;
        auVar1._8_4_ = uStack_70;
        auVar1._12_4_ = uStack_6c;
        *(undefined1 (*) [16])((longlong)pauVar12 + lVar7 + -0x5b) = auVar1;
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x4b) = uStack_68;
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x47) = (undefined4)uStack_64;
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x43) = uStack_64._4_4_;
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x3f) = (undefined4)uStack_5c;
        *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x3c) = uStack_5c._3_4_;
      }
      else if (iVar2 == 1) {
        uStack_68 = *(undefined4 *)((longlong)pauVar12 + lVar7 + -0x48);
        uVar5 = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x58);
        uVar6 = *(undefined8 *)((longlong)pauVar12 + lVar7 + -0x50);
        uStack_78 = (undefined4)uVar5;
        uStack_74 = (undefined4)((ulonglong)uVar5 >> 0x20);
        uStack_70 = (undefined4)uVar6;
        uStack_6c = (undefined4)((ulonglong)uVar6 >> 0x20);
        FUN_140bd5120(&uStack_78,auStack_190,0x37);
        goto code_r0x0001404f7d90;
      }
      lVar11 = lVar11 + -1;
    } while (lVar11 != 0);
  }
  lVar11 = lStack_80;
  func_0x000142923c00(lStack_80,auStack_190,0xa8);
  *(undefined1 (**) [16])(lVar11 + 0xa8) = pauStack_e8;
  *(undefined8 *)(lVar11 + 0xb0) = uStack_e0;
  *(undefined8 *)(lVar11 + 0xb8) = uStack_d8;
  *(longlong *)(lVar11 + 0xc0) = lStack_d0;
  *(undefined8 *)(lVar11 + 200) = uStack_c8;
  *(undefined8 *)(lVar11 + 0xd0) = uStack_c0;
  *(undefined8 *)(lVar11 + 0xd8) = uStack_b8;
  *(undefined8 *)(lVar11 + 0xe0) = uStack_b0;
  *(ulonglong *)(lVar11 + 0xe8) = CONCAT44(uStack_a4,uStack_a8);
  *(ulonglong *)(lVar11 + 0xf0) = CONCAT44(uStack_9c,uStack_a0);
  *(undefined8 *)(lVar11 + 0xf8) = uStack_98;
  *(undefined8 *)(lVar11 + 0x100) = uStack_90;
  return lVar11;
}

