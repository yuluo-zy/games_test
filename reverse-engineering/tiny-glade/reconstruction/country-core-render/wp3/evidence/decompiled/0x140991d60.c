// address 0x140991d60
// name update_instance_mesh
// signature void __cdecl update_instance_mesh(void)

void __cdecl country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  ulonglong extraout_RAX;
  longlong lVar6;
  ulonglong uVar7;
  longlong in_RCX;
  int *in_RDX;
  ulonglong uVar8;
  undefined2 in_R8W;
  ulonglong uVar9;
  uint uVar10;
  ushort uVar11;
  ulonglong uVar12;
  bool bVar13;
  byte bVar14;
  char cVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  byte local_69;
  byte *local_68;
  code *local_60;
  undefined **local_58;
  undefined8 local_50;
  byte **local_48;
  undefined8 local_40;
  undefined8 local_38;
  
  local_69 = *(byte *)(in_RDX + 2);
  lVar2 = *(longlong *)(in_RCX + 8 + (ulonglong)((uint)local_69 * 0xc0));
  if (SBORROW8(0,lVar2)) {
    local_68 = &local_69;
    local_60 = gpu_world::impl$17::fmt;
    local_58 = &anon_3703a396df78e7356336212e87fdc18b_44_llvm_1710316161241481988;
    local_50 = 2;
    local_38 = 0;
    local_48 = &local_68;
    local_40 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (lVar2 == -0x7fffffffffffffff) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar2 = in_RCX + (ulonglong)((uint)local_69 * 0xc0);
  uVar9 = (ulonglong)(uint)in_RDX[1];
  if ((uVar9 < *(ulonglong *)(lVar2 + 0x18)) &&
     (*(int *)(*(longlong *)(lVar2 + 0x10) + uVar9 * 4) == *in_RDX)) {
LAB_140991e79:
    uVar12 = *(ulonglong *)(lVar2 + 0x50);
    do {
      uVar8 = uVar9;
      if (uVar12 <= uVar9) {
        uVar8 = uVar12;
      }
      LOCK();
      uVar7 = *(ulonglong *)(lVar2 + 0x50);
      bVar13 = uVar12 == uVar7;
      if (bVar13) {
        *(ulonglong *)(lVar2 + 0x50) = uVar8;
        uVar7 = uVar12;
      }
      UNLOCK();
      uVar12 = uVar7;
    } while (!bVar13);
    uVar12 = *(ulonglong *)(lVar2 + 0x58);
    do {
      uVar8 = uVar9;
      if (uVar9 < uVar12) {
        uVar8 = uVar12;
      }
      LOCK();
      uVar7 = *(ulonglong *)(lVar2 + 0x58);
      bVar13 = uVar12 == uVar7;
      if (bVar13) {
        *(ulonglong *)(lVar2 + 0x58) = uVar8;
        uVar7 = uVar12;
      }
      UNLOCK();
      uVar12 = uVar7;
    } while (!bVar13);
    if (*(ulonglong *)(lVar2 + 0x68) <= uVar9) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    *(undefined2 *)(*(longlong *)(lVar2 + 0x60) + 0xe + uVar9 * 0x10) = in_R8W;
    return;
  }
  if (*(longlong *)(lVar2 + 0x38) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar3 = *(longlong *)(lVar2 + 0x20);
    bVar14 = (byte)(extraout_RAX >> 0x39);
    auVar16 = ZEXT216(CONCAT11(bVar14,bVar14));
    auVar16 = pshuflw(auVar16,auVar16,0);
    lVar6 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = *(ulonglong *)(lVar2 + 0x28) & uVar9;
      pcVar1 = (char *)(lVar3 + uVar9);
      cVar15 = auVar16[0];
      auVar21[0] = -(*pcVar1 == cVar15);
      cVar17 = auVar16[1];
      auVar21[1] = -(pcVar1[1] == cVar17);
      cVar18 = auVar16[2];
      auVar21[2] = -(pcVar1[2] == cVar18);
      cVar19 = auVar16[3];
      auVar21[3] = -(pcVar1[3] == cVar19);
      auVar21[4] = -(pcVar1[4] == cVar15);
      auVar21[5] = -(pcVar1[5] == cVar17);
      auVar21[6] = -(pcVar1[6] == cVar18);
      auVar21[7] = -(pcVar1[7] == cVar19);
      auVar21[8] = -(pcVar1[8] == cVar15);
      auVar21[9] = -(pcVar1[9] == cVar17);
      auVar21[10] = -(pcVar1[10] == cVar18);
      auVar21[0xb] = -(pcVar1[0xb] == cVar19);
      auVar21[0xc] = -(pcVar1[0xc] == cVar15);
      auVar21[0xd] = -(pcVar1[0xd] == cVar17);
      auVar21[0xe] = -(pcVar1[0xe] == cVar18);
      auVar21[0xf] = -(pcVar1[0xf] == cVar19);
      uVar11 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
      uVar10 = (uint)uVar11;
      if (uVar11 != 0) {
        do {
          uVar4 = 0;
          for (uVar5 = uVar10; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          uVar12 = uVar4 + uVar9 & *(ulonglong *)(lVar2 + 0x28);
          if (*in_RDX == *(int *)(lVar3 + uVar12 * -8 + -8)) {
            uVar10 = *(uint *)(lVar3 + -4 + uVar12 * -8);
            uVar9 = (ulonglong)uVar10;
            in_RDX[1] = uVar10;
            goto LAB_140991e79;
          }
          uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
        } while (uVar11 != 0);
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
                      (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar20 >> 0x3f,0) & 1) != 0) || (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar20 >> 0x5f,0) & 1) != 0) || (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0) ||
          auVar20[0xf] < '\0') break;
      uVar9 = uVar9 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

