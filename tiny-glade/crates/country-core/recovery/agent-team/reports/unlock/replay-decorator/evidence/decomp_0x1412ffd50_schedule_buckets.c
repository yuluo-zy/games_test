
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<country_core::resources::history::replay_cmd_buffer::ReplayBucket,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::vec::Vec<usize,alloc::alloc::Global>_>,country_core::systems::history::history_process::schedule_replay_commands::closure_env_2>_>
          (void)

{
  undefined1 uVar1;
  longlong lVar2;
  longlong lVar3;
  undefined1 *puVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  undefined8 uVar7;
  code *pcVar8;
  ulonglong extraout_RAX;
  ulonglong *in_RCX;
  longlong *in_RDX;
  undefined1 *puVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  ulonglong local_a8;
  ulonglong uStack_a0;
  
  lVar2 = *in_RDX;
  lVar3 = in_RDX[1];
  uVar11 = lVar3 - lVar2;
  uVar10 = uVar11 / 0x18;
  if ((uVar11 < 0x5fffffffffffffe9) && (uVar10 << 6 < 0x7ffffffffffffff9)) {
    if (uVar10 << 6 == 0) {
      local_a8 = 0;
      uStack_a0 = 8;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_a8 = uVar10;
      uStack_a0 = extraout_RAX;
      if (extraout_RAX == 0) goto LAB_1412fff06;
    }
    if (lVar2 == lVar3) {
      uVar10 = 0;
    }
    else {
      puVar4 = (undefined1 *)in_RDX[3];
      puVar5 = (undefined8 *)in_RDX[4];
      puVar9 = (undefined1 *)(uStack_a0 + 0x38);
      uVar11 = 0;
      do {
        from_iter<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,country_core::systems::history::history_process::schedule_replay_commands::closure_2::closure_env_0>_>
                  ();
        uVar1 = *puVar4;
        plVar6 = (longlong *)*puVar5;
        uVar7 = puVar5[1];
        LOCK();
        lVar2 = *plVar6;
        *plVar6 = *plVar6 + 1;
        UNLOCK();
        if (*plVar6 == 0 || SCARRY8(lVar2,1) != *plVar6 < 0) {
                    // WARNING: Does not return
          pcVar8 = (code *)invalidInstructionException();
          (*pcVar8)();
        }
        *(undefined8 *)(puVar9 + -0x28) = local_b0;
        *(undefined8 *)(puVar9 + -0x38) = local_c0;
        *(undefined8 *)(puVar9 + -0x30) = uStack_b8;
        *(longlong **)(puVar9 + -0x20) = plVar6;
        *(undefined8 *)(puVar9 + -0x18) = uVar7;
        *(undefined8 *)(puVar9 + -0x10) = 0;
        *(undefined8 *)(puVar9 + -8) = 0;
        *puVar9 = uVar1;
        uVar11 = uVar11 + 1;
        puVar9 = puVar9 + 0x40;
      } while (uVar10 != uVar11);
    }
    in_RCX[2] = uVar10;
    *in_RCX = local_a8;
    in_RCX[1] = uStack_a0;
    return;
  }
LAB_1412fff06:
                    // WARNING: Subroutine does not return
  raw_vec::handle_error();
}

