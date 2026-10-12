
void __cdecl country_core::systems::camera_rig::MainCamera::load_parameters(void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  char in_R8B;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined4 uVar11;
  float local_58;
  float fStack_54;
  float fStack_50;
  float fStack_4c;
  
  *(undefined1 *)(in_RCX + 0x4d0) = 0;
  fVar1 = *(float *)(in_RDX + 2);
  uVar3 = *in_RDX;
  glam::euler::impl$46::from_euler_angles();
  fVar2 = *(float *)((longlong)in_RDX + 0x14);
  fVar9 = fVar2 * 0.0;
  fVar8 = fStack_4c * fStack_4c;
  fVar10 = fStack_50 * fStack_50 + fStack_54 * fStack_54 + local_58 * local_58;
  fVar7 = fStack_50 * fVar2 + fStack_54 * fVar9 + local_58 * fVar9;
  fVar7 = fVar7 + fVar7;
  fVar6 = fStack_4c + fStack_4c;
  *(ulonglong *)(in_RCX + 0x144) =
       CONCAT44(((float)((ulonglong)in_RDX[1] >> 0x20) +
                fVar6 * (fStack_50 * fVar9 - fVar2 * local_58) +
                fVar7 * fStack_54 + (fVar8 - fVar10) * fVar9) -
                (float)((ulonglong)*(undefined8 *)(in_RCX + 0x138) >> 0x20),
                ((float)in_RDX[1] +
                fVar6 * (fStack_54 * fVar2 - fVar9 * fStack_50) +
                fVar7 * local_58 + (fVar8 - fVar10) * fVar9) -
                (float)*(undefined8 *)(in_RCX + 0x138));
  *(float *)(in_RCX + 0x14c) =
       (fVar6 * (local_58 * fVar9 - fVar9 * fStack_54) +
        fVar7 * fStack_50 + (fVar8 - fVar10) * fVar2 + fVar1) - *(float *)(in_RCX + 0x140);
  *(undefined4 *)(in_RCX + 0x118) = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 0;
    uVar4 = *(undefined8 *)(in_RCX + 0x150);
    uVar11 = (undefined4)__xmm_00000000000000008000000080000000;
    uVar5 = (ulonglong)__xmm_00000000000000008000000080000000 >> 0x20;
    *(float *)(extraout_RAX + 1) = (float)((ulonglong)uVar3 >> 0x20) - (float)uVar4;
    *(float *)((longlong)extraout_RAX + 0xc) = (float)uVar3 - (float)((ulonglong)uVar4 >> 0x20);
    *(undefined4 *)(extraout_RAX + 2) = uVar11;
    *(int *)((longlong)extraout_RAX + 0x14) = (int)uVar5;
    *(undefined4 *)(extraout_RAX + 3) = 0xbf800000;
    if (*(longlong *)(in_RCX + 0x100) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(undefined8 *)(in_RCX + 0x100) = 1;
    *(undefined8 **)(in_RCX + 0x108) = extraout_RAX;
    *(undefined8 *)(in_RCX + 0x110) = 1;
    if (in_R8B == '\0') {
      *(undefined4 *)(in_RCX + 0x194) = *(undefined4 *)(in_RDX + 3);
    }
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

