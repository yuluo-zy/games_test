// ADDRESS: 0x140938240
// SIZE: 258
// SIGNATURE: void __cdecl new_draw(void)

void __cdecl country_core::render::shader::RasterShaderProgram::new_draw(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 *in_RCX;
  undefined8 in_R8;
  undefined8 in_R9;
  undefined8 *in_stack_00000028;
  longlong local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined8 uStack_38;
  
  compile_shader_variants();
  if (local_b0 != 0) {
    in_RCX[0x19] = local_40;
    in_RCX[0x1a] = uStack_38;
    in_RCX[0x17] = local_50;
    in_RCX[0x18] = uStack_48;
    in_RCX[0x15] = local_60;
    in_RCX[0x16] = uStack_58;
    in_RCX[0x13] = local_70;
    in_RCX[0x14] = uStack_68;
    in_RCX[0x11] = local_80;
    in_RCX[0x12] = uStack_78;
    in_RCX[0xf] = local_90;
    in_RCX[0x10] = uStack_88;
    in_RCX[0xd] = local_a0;
    in_RCX[0xe] = uStack_98;
    in_RCX[0xb] = local_b0;
    in_RCX[0xc] = uStack_a8;
    in_RCX[8] = in_stack_00000028[8];
    uVar1 = *in_stack_00000028;
    uVar2 = in_stack_00000028[1];
    uVar3 = in_stack_00000028[2];
    uVar4 = in_stack_00000028[3];
    uVar5 = in_stack_00000028[4];
    uVar6 = in_stack_00000028[5];
    uVar7 = in_stack_00000028[7];
    in_RCX[6] = in_stack_00000028[6];
    in_RCX[7] = uVar7;
    in_RCX[4] = uVar5;
    in_RCX[5] = uVar6;
    in_RCX[2] = uVar3;
    in_RCX[3] = uVar4;
    *in_RCX = uVar1;
    in_RCX[1] = uVar2;
    in_RCX[9] = in_R8;
    in_RCX[10] = in_R9;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

