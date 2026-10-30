
void __cdecl
system_decorator::decorator_visual::stairs_assemble::playermade_stairs_add_wall_holes(void)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  undefined4 *puVar4;
  ushort uVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar9;
  uint uVar10;
  undefined1 (*pauVar11) [16];
  undefined1 (*pauVar12) [16];
  
  lVar9 = ((undefined8 *)*in_RCX)[3];
  if (lVar9 != 0) {
    uVar3 = *(undefined4 *)(in_RDX + 0x1c);
    puVar4 = *(undefined4 **)(in_RDX + 0x10);
    pauVar12 = *(undefined1 (**) [16])*in_RCX;
    pauVar11 = pauVar12 + 1;
    auVar2 = *pauVar12;
    uVar10 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar10 == 0) {
        do {
          auVar2 = *pauVar11;
          uVar5 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
          pauVar12 = pauVar12 + -0x110;
          pauVar11 = pauVar11 + 1;
        } while (uVar5 == 0xffff);
        uVar10 = ~(uint)uVar5;
        uVar7 = uVar10 & -(uint)uVar5 - 2;
      }
      else {
        uVar7 = uVar10 - 1 & uVar10;
      }
      uVar6 = 0;
      for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      if (*(longlong *)(pauVar12[(ulonglong)uVar6 * -0x11 + -0xd] + 8) != 0) {
        lVar8 = *(longlong *)pauVar12[(ulonglong)uVar6 * -0x11 + -0xd];
        lVar1 = lVar8 + *(longlong *)(pauVar12[(ulonglong)uVar6 * -0x11 + -0xd] + 8) * 0x18;
        do {
          lVar8 = lVar8 + 0x18;
          *puVar4 = uVar3;
          country_core::resources::walls::wall_holes::WallHoles::add();
        } while (lVar8 != lVar1);
      }
      lVar9 = lVar9 + -1;
      uVar10 = uVar7;
    } while (lVar9 != 0);
  }
  return;
}


