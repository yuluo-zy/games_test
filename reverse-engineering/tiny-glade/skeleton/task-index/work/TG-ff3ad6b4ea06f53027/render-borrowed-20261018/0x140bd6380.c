
void __cdecl rhapsody::rg::import_export::impl$0::export(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *in_RCX;
  longlong lVar7;
  longlong in_RDX;
  undefined1 in_R8B;
  
  lVar2 = *(longlong *)(in_RDX + 0x28);
  if (lVar2 == *(longlong *)(in_RDX + 0x18)) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  }
  lVar3 = *(longlong *)(in_RDX + 0x20);
  lVar7 = lVar2 * 0x30;
  *(undefined4 *)(lVar3 + lVar7) = 0;
  uVar4 = in_RCX[1];
  uVar5 = in_RCX[2];
  uVar6 = in_RCX[3];
  puVar1 = (undefined8 *)(lVar3 + 4 + lVar7);
  *puVar1 = *in_RCX;
  puVar1[1] = uVar4;
  puVar1 = (undefined8 *)(lVar3 + 0x14 + lVar7);
  *puVar1 = uVar5;
  puVar1[1] = uVar6;
  *(undefined4 *)(lVar3 + 0x24 + lVar7) = *(undefined4 *)(in_RCX + 4);
  *(undefined1 *)(lVar3 + 0x28 + lVar7) = in_R8B;
  *(longlong *)(in_RDX + 0x28) = lVar2 + 1;
  return;
}

