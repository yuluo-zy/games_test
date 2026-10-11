
void __cdecl country_core::systems::main_camera::camera_snapshot(void)

{
  longlong *in_RDX;
  ulonglong uVar1;
  longlong *in_R9;
  longlong *in_stack_00000028;
  
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  uVar1 = (ulonglong)*(uint *)(*in_R9 + 0x18);
                    /* WARNING: Could not recover jumptable at 0x000140911ea8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&DAT_142aabc88 + *(int *)(&DAT_142aabc88 + uVar1 * 4)))
            (*(undefined4 *)(*in_R9 + 0x1c),
             *(float *)(*in_RDX + 0x28) * *(float *)(*in_stack_00000028 + 0xec),uVar1,
             &DAT_142aabc88 + *(int *)(&DAT_142aabc88 + uVar1 * 4));
  return;
}

