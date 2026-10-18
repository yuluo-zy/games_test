
void __cdecl country_core::resources::stairs::stairs_state::StairsState::reduce_diff(void)

{
  undefined1 auVar1 [16];
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined1 (*pauVar2) [16];
  undefined8 in_R8;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  undefined8 in_stack_00000028;
  undefined8 local_d0;
  undefined8 uStack_c8;
  ulonglong local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  ulonglong local_a8;
  int local_a0;
  
  lVar3 = in_RDX[3];
  if (lVar3 != 0) {
    pauVar2 = (undefined1 (*) [16])*in_RDX + 1;
    auVar1 = *(undefined1 (*) [16])*in_RDX;
    uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
    do {
      uVar5 = uVar4;
      if ((short)uVar4 == 0) {
        do {
          auVar1 = *pauVar2;
          pauVar2 = pauVar2 + 1;
          uVar4 = (uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
        } while (uVar4 == 0xffff);
        uVar4 = ~uVar4;
        uVar5 = uVar4;
      }
      for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>_>
                ();
      if (local_a0 == 1) {
        hashbrown::raw::RawTable::
        erase<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
                  ();
      }
      uVar4 = uVar5 - 1 & uVar5;
      lVar3 = lVar3 + -1;
    } while (lVar3 != 0);
  }
  alloc::vec::spec_from_iter::impl$0::
  from_iter<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,std::collections::hash::set::IntoIter<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>_>
            ();
  alloc::vec::spec_from_iter::impl$0::
  from_iter<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,std::collections::hash::set::IntoIter<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>_>
            ();
  if (1 < local_a8) {
    if (local_a8 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,bool_(*)(ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>,ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>)>
                ();
    }
    else {
      core::slice::sort::unstable::
      ipnsort<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,bool_(*)(ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>,ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>)>
                ();
    }
  }
  if (1 < local_c0) {
    if (local_c0 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,bool_(*)(ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>,ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>)>
                ();
    }
    else {
      core::slice::sort::unstable::
      ipnsort<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,bool_(*)(ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>,ref_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>)>
                ();
    }
  }
  in_RCX[6] = local_a8;
  in_RCX[4] = local_b8;
  in_RCX[5] = uStack_b0;
  in_RCX[9] = local_c0;
  in_RCX[7] = local_d0;
  in_RCX[8] = uStack_c8;
  *in_RCX = 1;
  in_RCX[1] = in_R8;
  in_RCX[2] = 1;
  in_RCX[3] = in_stack_00000028;
  return;
}

