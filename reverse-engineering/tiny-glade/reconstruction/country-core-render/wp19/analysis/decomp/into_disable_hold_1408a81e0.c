// into_disable_hold  0x1408a81e0
// signature: void __cdecl into_disable_hold(void)
// callees: [{"name": "panic_fmt", "address": "1428d9390"}]


void __cdecl country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  if (*(int *)(in_RDX + 9) != 2) {
    *(undefined1 *)((longlong)in_RDX + 0x54) = 0;
    uVar1 = in_RDX[9];
    in_RCX[8] = in_RDX[8];
    in_RCX[9] = uVar1;
    uVar1 = *in_RDX;
    uVar2 = in_RDX[1];
    uVar3 = in_RDX[2];
    uVar4 = in_RDX[3];
    uVar5 = in_RDX[4];
    uVar6 = in_RDX[5];
    uVar7 = in_RDX[7];
    in_RCX[6] = in_RDX[6];
    in_RCX[7] = uVar7;
    in_RCX[4] = uVar5;
    in_RCX[5] = uVar6;
    in_RCX[2] = uVar3;
    in_RCX[3] = uVar4;
    *in_RCX = uVar1;
    in_RCX[1] = uVar2;
    in_RCX[10] = in_RDX[10];
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

