// ADDR 0x140827130
// SIG void __cdecl validate_blueprints(void)
// ORIG system::validate_blueprints


void __cdecl system_decorator::blueprint::validate_blueprints(void)

{
  undefined1 auVar1 [16];
  ulonglong uVar2;
  longlong lVar3;
  undefined8 uVar4;
  char extraout_AL;
  longlong in_R8;
  longlong lVar5;
  longlong *in_R9;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000070;
  char local_c9;
  undefined1 local_c8 [16];
  undefined1 (*local_b8) [16];
  undefined1 (*local_b0) [16];
  undefined1 *local_a8;
  ushort local_a0;
  undefined8 local_98;
  undefined1 *local_80;
  char *local_50;
  
  **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
  country_core::resources::walls::decorator_storage::DecoratorStorage::clear_modification_tracking()
  ;
  lVar5 = *in_R9;
  if (*(longlong *)(lVar5 + 0x10) != 0) {
    country_core::resources::walls::decorator_subtype::impl$8::from();
    local_c9 = '\0';
    local_b8 = *(undefined1 (**) [16])(lVar5 + 0x18);
    local_a8 = *local_b8 + *(longlong *)(lVar5 + 0x20) + 1;
    auVar1 = *local_b8;
    local_a0 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
    local_b0 = local_b8 + 1;
    local_98 = *(undefined8 *)(lVar5 + 0x30);
    local_80 = local_c8;
    local_50 = &local_c9;
    core::iter::adapters::copied::impl$1::
    try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple_<>,core::iter::traits::iterator::Iterator::all::check::closure_env_0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env_0>,enum2_<core::ops::control_flow::ControlFlow<tuple_<>,tuple_<>_>_>_>
              ();
    if (extraout_AL != '\0') {
      lVar5 = *in_stack_00000040;
      if (*(char *)(lVar5 + 0x281) != '\0') {
        if (local_c9 == '\0') {
          uVar2 = *(ulonglong *)*in_stack_00000048;
          lVar5 = 0;
          if ((((ulonglong *)*in_stack_00000048)[1] - 1) + (ulonglong)(1 < uVar2) <
              (ulonglong)(uVar2 - 2 < 5)) {
            lVar5 = uVar2 - 1;
          }
                    // WARNING: Could not recover jumptable at 0x0001408272b5. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)((longlong)&DAT_142a91c00 + (longlong)(int)(&DAT_142a91c00)[lVar5]))
                    (&DAT_142a91c00,
                     (code *)((longlong)&DAT_142a91c00 + (longlong)(int)(&DAT_142a91c00)[lVar5]));
          return;
        }
        *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
        *(undefined1 *)(lVar5 + 0x282) = 2;
        *(undefined4 *)in_stack_00000070[2] = *(undefined4 *)((longlong)in_stack_00000070 + 0x1c);
        lVar5 = *in_stack_00000070;
        lVar3 = *(longlong *)(lVar5 + 0x30);
        uVar4 = *(undefined8 *)(lVar5 + 0x40);
        if (lVar3 == *(longlong *)(lVar5 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(lVar5 + 0x28) + lVar3 * 8) = uVar4;
        *(longlong *)(lVar5 + 0x30) = lVar3 + 1;
        *(longlong *)(lVar5 + 0x40) = *(longlong *)(lVar5 + 0x40) + 1;
      }
    }
  }
  return;
}


