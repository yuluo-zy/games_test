// 0x1409f18f0 pop_front

void __cdecl country_core::resources::history::replay_cmd_buffer::ReplayBuffer::pop_front(void)

{
  longlong *plVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  longlong *in_RCX;
  longlong lVar15;
  longlong lVar16;
  ulonglong *in_RDX;
  ulonglong uVar17;
  longlong *in_R8;
  
  in_RDX[4] = in_RDX[4] + 1;
  if (in_RDX[3] != 0) {
    uVar2 = in_RDX[2];
    uVar3 = in_RDX[1];
    uVar17 = 0;
    if (*in_RDX <= uVar2 + 1) {
      uVar17 = *in_RDX;
    }
    in_RDX[2] = (uVar2 - uVar17) + 1;
    in_RDX[3] = in_RDX[3] - 1;
    lVar15 = uVar2 * 0x40;
    lVar4 = *(longlong *)(uVar3 + lVar15);
    lVar5 = *(longlong *)(uVar3 + 0x38 + lVar15);
    plVar1 = (longlong *)(uVar3 + 0x28 + lVar15);
    lVar10 = *plVar1;
    lVar11 = plVar1[1];
    plVar1 = (longlong *)(uVar3 + 0x18 + lVar15);
    lVar12 = *plVar1;
    lVar13 = plVar1[1];
    plVar1 = (longlong *)(uVar3 + 8 + lVar15);
    lVar15 = *plVar1;
    lVar14 = plVar1[1];
    if (!SBORROW8(0,lVar4)) {
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar6 = *in_R8;
      lVar7 = *(longlong *)(lVar6 + 0x30);
      uVar8 = *(undefined8 *)(lVar6 + 0x40);
      if (lVar7 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                  ();
      }
      lVar9 = *(longlong *)(lVar6 + 0x28);
      lVar16 = lVar7 * 0x30;
      *(undefined8 *)(lVar9 + lVar16) = 0x8000000000000000;
      *(undefined8 *)(lVar9 + 8 + lVar16) = 0;
      *(undefined8 *)(lVar9 + 0x28 + lVar16) = uVar8;
      *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
      in_RCX[6] = lVar11;
      in_RCX[7] = lVar5;
      in_RCX[4] = lVar13;
      in_RCX[5] = lVar10;
      in_RCX[2] = lVar14;
      in_RCX[3] = lVar12;
      *in_RCX = lVar4;
      in_RCX[1] = lVar15;
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

