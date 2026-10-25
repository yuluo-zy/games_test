// signature: void __cdecl triangles(void)

void __cdecl country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::triangles(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong *in_RCX;
  longlong in_RDX;
  undefined4 in_XMM2_Da;
  
  lVar1 = *(longlong *)(in_RDX + 0x10);
  if (lVar1 != 0) {
    lVar2 = *(longlong *)(in_RDX + 8);
    *in_RCX = lVar2;
    in_RCX[1] = lVar2 + lVar1 * 0x24;
    *(undefined4 *)(in_RCX + 2) = in_XMM2_Da;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

