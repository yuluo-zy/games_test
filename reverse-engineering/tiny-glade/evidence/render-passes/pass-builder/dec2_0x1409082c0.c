// address: 0x1409082c0
// pdb name: discard
// signature: void __cdecl discard(void)

void __cdecl country_core::render::pass_builder::draw_builder::DrawBuilder::discard(void)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  ushort uVar3;
  uint uVar4;
  longlong *in_RCX;
  undefined1 (*pauVar5) [16];
  uint uVar6;
  uint uVar7;
  undefined1 local_d0 [104];
  undefined1 (*local_68) [16];
  longlong local_50;
  undefined8 local_18;
  
  local_18 = 0xfffffffffffffffe;
  lVar2 = *in_RCX;
  *in_RCX = 2;
  if (lVar2 == 2) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  memcpy(local_d0,in_RCX + 1,0xb0);
  if (local_50 != 0) {
    pauVar5 = local_68 + 1;
    auVar1 = *local_68;
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
    do {
      if ((short)uVar6 == 0) {
        do {
          auVar1 = *pauVar5;
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
          local_68 = local_68 + -0x48;
          pauVar5 = pauVar5 + 1;
        } while (uVar3 == 0xffff);
        uVar7 = ~(uint)uVar3;
        uVar6 = ~(uint)uVar3 & -(uint)uVar3 - 2;
      }
      else {
        uVar7 = uVar6;
        uVar6 = uVar6 - 1 & uVar6;
      }
      uVar4 = 0;
      for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      *(undefined1 *)((longlong)local_68 + (ulonglong)uVar4 * -0x48 + -8) = 2;
      local_50 = local_50 + -1;
    } while (local_50 != 0);
  }
  core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPacket>();
  core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>();
  return;
}


