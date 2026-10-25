// ADDR 0x140811e80
// SIG void __cdecl seal_persistent_entities(void)
// ORIG system::seal_persistent_entities


void __cdecl
system_decorator::decorator_visual_state::DecoratorVisualState::seal_persistent_entities(void)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  ulonglong extraout_RAX;
  longlong in_RCX;
  int in_EDX;
  ulonglong uVar5;
  longlong lVar6;
  uint uVar7;
  ushort uVar8;
  ulonglong uVar9;
  byte bVar10;
  char cVar11;
  char cVar13;
  char cVar14;
  char cVar15;
  undefined1 auVar12 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  
  if (*(longlong *)(in_RCX + 0x30) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar2 = *(longlong *)(in_RCX + 0x18);
    bVar10 = (byte)(extraout_RAX >> 0x39);
    auVar12 = ZEXT216(CONCAT11(bVar10,bVar10));
    auVar12 = pshuflw(auVar12,auVar12,0);
    lVar6 = 0;
    uVar5 = extraout_RAX;
    while( true ) {
      uVar5 = *(ulonglong *)(in_RCX + 0x20) & uVar5;
      pcVar1 = (char *)(lVar2 + uVar5);
      cVar11 = auVar12[0];
      auVar17[0] = -(*pcVar1 == cVar11);
      cVar13 = auVar12[1];
      auVar17[1] = -(pcVar1[1] == cVar13);
      cVar14 = auVar12[2];
      auVar17[2] = -(pcVar1[2] == cVar14);
      cVar15 = auVar12[3];
      auVar17[3] = -(pcVar1[3] == cVar15);
      auVar17[4] = -(pcVar1[4] == cVar11);
      auVar17[5] = -(pcVar1[5] == cVar13);
      auVar17[6] = -(pcVar1[6] == cVar14);
      auVar17[7] = -(pcVar1[7] == cVar15);
      auVar17[8] = -(pcVar1[8] == cVar11);
      auVar17[9] = -(pcVar1[9] == cVar13);
      auVar17[10] = -(pcVar1[10] == cVar14);
      auVar17[0xb] = -(pcVar1[0xb] == cVar15);
      auVar17[0xc] = -(pcVar1[0xc] == cVar11);
      auVar17[0xd] = -(pcVar1[0xd] == cVar13);
      auVar17[0xe] = -(pcVar1[0xe] == cVar14);
      auVar17[0xf] = -(pcVar1[0xf] == cVar15);
      uVar8 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar7 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar9 = uVar3 + uVar5 & *(ulonglong *)(in_RCX + 0x20);
        if (*(int *)(lVar2 + -0x28 + uVar9 * -0x28) == in_EDX) {
          if (*(int *)(lVar2 + -0x20 + uVar9 * -0x28) != 1) {
            lVar2 = lVar2 + uVar9 * -0x28;
            *(undefined8 *)(lVar2 + -0x20) = 1;
            *(undefined8 *)(lVar2 + -0x18) = *(undefined8 *)(lVar2 + -0x18);
            *(undefined8 *)(lVar2 + -0x10) = *(undefined8 *)(lVar2 + -0x10);
            *(undefined8 *)(lVar2 + -8) = *(undefined8 *)(lVar2 + -8);
            return;
          }
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar8 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar8);
      }
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
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') break;
      uVar5 = uVar5 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
  return;
}


