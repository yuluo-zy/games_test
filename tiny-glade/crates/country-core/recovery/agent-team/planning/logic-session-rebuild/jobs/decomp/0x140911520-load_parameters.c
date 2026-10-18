// 0x140911520 load_parameters
// signature: void __cdecl load_parameters(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "from_euler_angles", "address": "140d4b440"}, {"name": "handle_alloc_error", "address": "1428d9000"}]


void __cdecl country_core::systems::camera_rig::MainCamera::load_parameters(void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  char in_R8B;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float local_58;
  float fStack_54;
  float fStack_50;
  float fStack_4c;
  
  *(undefined1 *)(in_RCX + 0x4d0) = 0;
  fVar1 = *(float *)(in_RDX + 2);
  uVar3 = *in_RDX;
  glam::euler::impl$46::from_euler_angles();
  fVar2 = *(float *)((longlong)in_RDX + 0x14);
  fVar8 = fVar2 * 0.0;
  fVar7 = fStack_4c * fStack_4c;
  fVar9 = fStack_50 * fStack_50 + fStack_54 * fStack_54 + local_58 * local_58;
  fVar6 = fStack_50 * fVar2 + fStack_54 * fVar8 + local_58 * fVar8;
  fVar6 = fVar6 + fVar6;
  fVar5 = fStack_4c + fStack_4c;
  *(ulonglong *)(in_RCX + 0x144) =
       CONCAT44(((float)((ulonglong)in_RDX[1] >> 0x20) +
                fVar5 * (fStack_50 * fVar8 - fVar2 * local_58) +
                fVar6 * fStack_54 + (fVar7 - fVar9) * fVar8) -
                (float)((ulonglong)*(undefined8 *)(in_RCX + 0x138) >> 0x20),
                ((float)in_RDX[1] +
                fVar5 * (fStack_54 * fVar2 - fVar8 * fStack_50) +
                fVar6 * local_58 + (fVar7 - fVar9) * fVar8) - (float)*(undefined8 *)(in_RCX + 0x138)
               );
  *(float *)(in_RCX + 0x14c) =
       (fVar5 * (local_58 * fVar8 - fVar8 * fStack_54) + fVar6 * fStack_50 + (fVar7 - fVar9) * fVar2
       + fVar1) - *(float *)(in_RCX + 0x140);
  *(undefined4 *)(in_RCX + 0x118) = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 0;
    uVar4 = *(undefined8 *)(in_RCX + 0x150);
    *(float *)(extraout_RAX + 1) = (float)((ulonglong)uVar3 >> 0x20) - (float)uVar4;
    *(float *)((longlong)extraout_RAX + 0xc) = (float)uVar3 - (float)((ulonglong)uVar4 >> 0x20);
    *(undefined4 *)(extraout_RAX + 2) = 0x80000000;
    *(undefined4 *)((longlong)extraout_RAX + 0x14) = 0x80000000;
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
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

