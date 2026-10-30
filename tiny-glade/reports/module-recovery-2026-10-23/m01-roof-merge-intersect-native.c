
void __cdecl
system_decorator::merge_proposal_roof_decorators::
intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env_1>
          (void)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  ushort uVar3;
  int extraout_EAX;
  int iVar4;
  uint uVar5;
  longlong in_stack_00000028;
  longlong *in_stack_00000040;
  undefined1 (*local_b8) [16];
  undefined1 (*local_b0) [16];
  ushort local_a0;
  longlong local_98;
  
  ::utils::geometry::rectangle::Rectangle3d::new();
  local_b8 = *(undefined1 (**) [16])(in_stack_00000028 + 0x30);
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
  local_98 = *(longlong *)(in_stack_00000028 + 0x48);
  do {
    if (local_98 == 0) {
      return;
    }
    uVar5 = (uint)local_a0;
    if (local_a0 == 0) {
      do {
        auVar1 = *local_b0;
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
        local_b8 = local_b8 + -0x50;
        local_b0 = local_b0 + 1;
      } while (uVar3 == 0xffff);
      uVar5 = ~(uint)uVar3;
      local_a0 = (ushort)uVar5 & -uVar3 - 2;
    }
    else {
      local_a0 = local_a0 - 1 & local_a0;
      if (local_b8 == (undefined1 (*) [16])0x0) {
        return;
      }
    }
    local_98 = local_98 + -1;
    iVar4 = 0;
    for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
      iVar4 = iVar4 + 1;
    }
    iVar4 = (int)local_b8 + iVar4 * -0x50 + -0x50;
    core::ops::function::impls::impl$3::
    call_mut<tuple_<tuple_<ref_<country_core::resources::walls::decorator::DecoratorId>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>_>,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env_0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env_1>_>
              ();
    if (extraout_EAX != 0) {
      lVar2 = in_stack_00000040[2];
      if (lVar2 == *in_stack_00000040) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      *(int *)(in_stack_00000040[1] + lVar2 * 4) = iVar4;
      in_stack_00000040[2] = lVar2 + 1;
    }
  } while( true );
}

