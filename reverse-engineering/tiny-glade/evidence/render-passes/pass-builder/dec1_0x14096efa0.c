// address: 0x14096efa0
// pdb name: dispatch_3d
// signature: void __cdecl dispatch_3d(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::dispatch_3d(void)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *in_RCX;
  uint *in_RDX;
  
  lVar7 = in_RCX[1];
  uVar1 = *(uint *)(lVar7 + 0x38);
  if (uVar1 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_div_by_zero();
  }
  uVar2 = *(uint *)(lVar7 + 0x3c);
  if (uVar2 != 0) {
    uVar3 = *(uint *)(lVar7 + 0x40);
    if (uVar3 != 0) {
      uVar4 = *in_RDX;
      uVar5 = in_RDX[1];
      uVar6 = in_RDX[2];
      lVar7 = *(longlong *)*in_RCX;
      lVar8 = ((longlong *)*in_RCX)[1];
      if (*(char *)(lVar8 + 0xaba) == '\x01') {
        rhapsody::raw::device::GraphicsDevice::issue_full_pipeline_barrier();
      }
      LOCK();
      *(longlong *)(lVar8 + 0xaa0) = *(longlong *)(lVar8 + 0xaa0) + 1;
      UNLOCK();
      (**(code **)(lVar8 + 0x360))
                (*(undefined8 *)(lVar7 + 0x48),(uVar4 / uVar1 + 1) - (uint)(uVar4 % uVar1 == 0),
                 (uVar5 / uVar2 + 1) - (uint)(uVar5 % uVar2 == 0),
                 (uVar6 / uVar3 + 1) - (uint)(uVar6 % uVar3 == 0));
      return;
    }
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_div_by_zero();
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_const::panic_const_div_by_zero();
}


