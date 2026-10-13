// address 0x14098f690
// name new_curved_xy
// signature void __cdecl new_curved_xy(void)

void __cdecl
country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy(void)

{
  int iVar1;
  uint uVar2;
  undefined8 *in_RCX;
  uint uVar3;
  undefined8 *in_RDX;
  float fVar4;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar5;
  undefined4 in_stack_00000028;
  float in_stack_00000030;
  float in_stack_00000038;
  
  fVar5 = in_XMM3_Da * 0.5 + in_XMM2_Da;
  fVar4 = ceilf((ABS(in_stack_00000038 - in_stack_00000030) * fVar5 + 1.0) / 0.6);
  uVar3 = 0;
  if (0.0 <= fVar4) {
    uVar3 = (uint)(longlong)fVar4;
  }
  if (4.294967e+09 < fVar4) {
    uVar3 = 0xffffffff;
  }
  iVar1 = 0x1f;
  if (uVar3 - 1 != 0) {
    for (; uVar3 - 1 >> iVar1 == 0; iVar1 = iVar1 + -1) {
    }
  }
  uVar2 = 1;
  if (1 < uVar3) {
    uVar2 = (0xffffffffU >> (~(byte)iVar1 & 0x1f)) + 1;
  }
  if (uVar2 != 0) {
    uVar3 = 0;
    for (uVar2 = uVar2 | 0x20; (uVar2 & 1) == 0; uVar2 = uVar2 >> 1 | 0x80000000) {
      uVar3 = uVar3 + 1;
    }
    uVar2 = 2;
    if (2 < uVar3) {
      uVar2 = uVar3;
    }
    *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(in_RDX + 1);
    *in_RCX = *in_RDX;
    *(undefined4 *)((longlong)in_RCX + 0xc) = 0;
    *(float *)(in_RCX + 2) = in_XMM2_Da - in_XMM3_Da * 0.5;
    *(float *)((longlong)in_RCX + 0x14) = fVar5;
    *(float *)(in_RCX + 3) = in_stack_00000030;
    *(float *)((longlong)in_RCX + 0x1c) = in_stack_00000038;
    in_RCX[4] = 0;
    *(undefined4 *)(in_RCX + 5) = in_stack_00000028;
    *(undefined4 *)((longlong)in_RCX + 0x2c) = 0;
    *(uint *)(in_RCX + 6) = uVar2;
    *(undefined8 *)((longlong)in_RCX + 0x34) = 0;
    *(undefined4 *)((longlong)in_RCX + 0x3c) = 0;
    return;
  }
                    // WARNING: Subroutine does not return
  core::num::int_log10::panic_for_nonpositive_argument();
}

