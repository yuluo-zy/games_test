
void __cdecl
core::iter::adapters::map::impl$2::
next<country_core::systems::render_loop::resources::critters::FireflySsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,system_critters::firefly::fireflies_send_to_gpu::closure_env_0>
          (void)

{
  longlong lVar1;
  uint uVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  undefined4 uVar6;
  longlong lVar7;
  ulonglong uVar8;
  undefined4 *in_RCX;
  longlong in_RDX;
  uint *puVar9;
  undefined8 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  
  lVar11 = *(longlong *)(in_RDX + 0x18);
  uVar8 = *(ulonglong *)(in_RDX + 0x68);
  if (*(char *)(in_RDX + 0x70) == '\0') {
    if (uVar8 == *(ulonglong *)(in_RDX + 0x60)) {
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) goto LAB_141dbd1ba;
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar2 = *puVar9;
        lVar7 = *(longlong *)(*(longlong *)(in_RDX + 0x10) + 8);
        lVar3 = *(longlong *)(lVar7 + 0x58 + (ulonglong)uVar2 * 0xa0);
        puVar9 = puVar9 + 1;
      } while (lVar3 == 0);
      lVar7 = lVar7 + (ulonglong)uVar2 * 0xa0;
      uVar12 = (ulonglong)*(uint *)(lVar7 + 0x94);
      lVar5 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
      lVar1 = lVar5 + uVar12 * 0x48;
      uVar8 = *(ulonglong *)(lVar11 + 0x118);
      uVar12 = *(ulonglong *)(lVar5 + 0x40 + uVar12 * 0x48);
      if ((*(ulonglong *)(lVar11 + 0x110) < uVar12) &&
         (uVar4 = *(ulonglong *)(*(longlong *)(lVar1 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
         uVar4 != 0)) {
        lVar11 = *(longlong *)(*(longlong *)(lVar1 + 0x18) + 0x10 + ~uVar4 * 0x30);
        *(longlong *)(in_RDX + 0x50) = lVar11;
        if (uVar12 <= uVar8) goto LAB_141dbd25e;
LAB_141dbd212:
        uVar8 = *(ulonglong *)(*(longlong *)(lVar1 + 0x38) + uVar8 * 8);
        if (uVar8 == 0) goto LAB_141dbd25e;
        uVar10 = *(undefined8 *)(*(longlong *)(lVar1 + 0x18) + 0x10 + ~uVar8 * 0x30);
      }
      else {
        lVar11 = 0;
        *(undefined8 *)(in_RDX + 0x50) = 0;
        if (uVar8 < uVar12) goto LAB_141dbd212;
LAB_141dbd25e:
        uVar10 = 0;
      }
      *(undefined8 *)(in_RDX + 0x58) = uVar10;
      lVar7 = *(longlong *)(lVar7 + 0x50);
      *(longlong *)(in_RDX + 0x40) = lVar7;
      *(longlong *)(in_RDX + 0x48) = lVar3;
      *(longlong *)(in_RDX + 0x60) = lVar3;
      uVar8 = 0;
    }
    else {
      lVar7 = *(longlong *)(in_RDX + 0x40);
      lVar11 = *(longlong *)(in_RDX + 0x50);
    }
    lVar11 = (ulonglong)*(uint *)(lVar7 + 8 + uVar8 * 0x10) * 0x30 + lVar11;
  }
  else {
    if (uVar8 == *(ulonglong *)(in_RDX + 0x60)) {
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) goto LAB_141dbd1ba;
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar2 = *puVar9;
        lVar7 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
        lVar3 = *(longlong *)(lVar7 + 0x10 + (ulonglong)uVar2 * 0x48);
        puVar9 = puVar9 + 1;
      } while (lVar3 == 0);
      lVar7 = lVar7 + (ulonglong)uVar2 * 0x48;
      uVar8 = *(ulonglong *)(lVar11 + 0x118);
      uVar12 = *(ulonglong *)(lVar7 + 0x40);
      if ((*(ulonglong *)(lVar11 + 0x110) < uVar12) &&
         (uVar4 = *(ulonglong *)(*(longlong *)(lVar7 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
         uVar4 != 0)) {
        lVar11 = *(longlong *)(*(longlong *)(lVar7 + 0x18) + 0x10 + ~uVar4 * 0x30);
        *(longlong *)(in_RDX + 0x50) = lVar11;
        if (uVar12 <= uVar8) goto LAB_141dbd235;
LAB_141dbd1e3:
        uVar8 = *(ulonglong *)(*(longlong *)(lVar7 + 0x38) + uVar8 * 8);
        if (uVar8 == 0) goto LAB_141dbd235;
        uVar10 = *(undefined8 *)(*(longlong *)(lVar7 + 0x18) + 0x10 + ~uVar8 * 0x30);
      }
      else {
        lVar11 = 0;
        *(undefined8 *)(in_RDX + 0x50) = 0;
        if (uVar8 < uVar12) goto LAB_141dbd1e3;
LAB_141dbd235:
        uVar10 = 0;
      }
      *(undefined8 *)(in_RDX + 0x58) = uVar10;
      *(undefined8 *)(in_RDX + 0x30) = *(undefined8 *)(lVar7 + 8);
      *(longlong *)(in_RDX + 0x38) = lVar3;
      *(longlong *)(in_RDX + 0x60) = lVar3;
      uVar8 = 0;
    }
    else {
      lVar11 = *(longlong *)(in_RDX + 0x50);
    }
    lVar11 = (uVar8 & 0xffffffff) * 0x30 + lVar11;
  }
  *(ulonglong *)(in_RDX + 0x68) = uVar8 + 1;
  in_RCX[3] = *(undefined4 *)(lVar11 + 0x18);
  *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(lVar11 + 0x10);
  in_RCX[4] = 0;
  uVar6 = 1;
LAB_141dbd2b0:
  *in_RCX = uVar6;
  return;
LAB_141dbd1ba:
  uVar6 = 0;
  goto LAB_141dbd2b0;
}

