
void __cdecl
system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::
assemble_dormer_roof_tiles::
closure_3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
          (void)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong uVar7;
  float *in_R8;
  float *in_R9;
  ulonglong uVar8;
  ulonglong uVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar15;
  undefined8 uVar14;
  char in_stack_00000028;
  undefined8 local_108;
  float local_100;
  float fStack_f0;
  
  fVar10 = (*in_R8 + *in_R9) * 0.5;
  fVar11 = (in_R8[2] + in_R9[2]) * 0.5;
  ::utils::swizzlers::impl$1::xvy();
  ::utils::swizzlers::impl$0::x0y();
  fVar12 = 1.0 / SQRT(fStack_f0 * fStack_f0 + fVar10 * fVar10 + fVar11 * fVar11);
  fVar11 = fVar12 * fVar11 * 0.0;
  fVar13 = (fVar11 - fVar12 * fStack_f0) * 0.08;
  fVar15 = (fVar12 * fStack_f0 * 0.0 - fVar10 * fVar12 * 0.0) * 0.08;
  fVar11 = (fVar10 * fVar12 - fVar11) * 0.08;
  fVar10 = (float)((ulonglong)local_108 >> 0x20);
  if (in_stack_00000028 == '\0') {
    uVar14 = CONCAT44(fVar15 + fVar10,fVar13 + (float)local_108);
    fVar11 = fVar11 + local_100;
  }
  else {
    fVar11 = local_100 - fVar11;
    uVar14 = CONCAT44(fVar10 - fVar15,(float)local_108 - fVar13);
  }
  ::utils::halver::impl$7::to_f16();
  ::utils::halver::impl$6::to_f16();
  ::utils::halver::impl$5::to_f16();
  puVar2 = *(ulonglong **)(in_RDX + 0x18);
  uVar8 = *puVar2;
  uVar9 = uVar8 + 0xa0761d6478bd642f;
  *puVar2 = uVar9;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar9 ^ 0xe7037ed1a0b428db;
  auVar5._8_8_ = 0;
  auVar5._0_8_ = uVar9;
  uVar7 = SUB168(auVar3 * auVar5,8) ^ SUB168(auVar3 * auVar5,0);
  if ((uint)((int)uVar7 * 200) < 0x60) {
    uVar8 = uVar8 + 0x40ec3ac8f17ac85e;
    do {
      uVar9 = uVar9 + 0xa0761d6478bd642f;
      auVar4._8_8_ = 0;
      auVar4._0_8_ = uVar8 ^ 0xe7037ed1a0b428db;
      auVar6._8_8_ = 0;
      auVar6._0_8_ = uVar8;
      uVar7 = SUB168(auVar4 * auVar6,8) ^ SUB168(auVar4 * auVar6,0);
      uVar8 = uVar8 + 0xa0761d6478bd642f;
    } while ((uint)((int)uVar7 * 200) < 0x60);
    *puVar2 = uVar9;
  }
  uVar1 = **(undefined4 **)(in_RDX + 0x20);
  *(float *)(in_RCX + 1) = fVar11;
  *in_RCX = uVar14;
  *(undefined4 *)((longlong)in_RCX + 0xc) = 0;
  in_RCX[2] = extraout_RAX_00;
  *(short *)((longlong)in_RCX + 0x1c) = (short)((ulonglong)extraout_RAX_01 >> 0x20);
  *(int *)(in_RCX + 3) = (int)extraout_RAX_01;
  *(undefined2 *)((longlong)in_RCX + 0x1e) = 0;
  in_RCX[4] = extraout_RAX;
  *(undefined4 *)(in_RCX + 5) = uVar1;
  *(undefined8 *)((longlong)in_RCX + 0x2c) = 0x3f80000000000000;
  *(int *)((longlong)in_RCX + 0x34) = (int)((uVar7 & 0xffffffff) * 200 >> 0x20) + -100;
  in_RCX[7] = 0;
  return;
}

