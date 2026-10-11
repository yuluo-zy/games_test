
void __cdecl
core::iter::adapters::filter_map::impl$2::
next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>
          (void)

{
  longlong lVar1;
  uint uVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  undefined8 *in_RCX;
  uint *puVar9;
  longlong lVar10;
  longlong in_RDX;
  byte local_58;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  
  do {
    lVar3 = *(longlong *)(in_RDX + 0x18);
    lVar10 = *(longlong *)(in_RDX + 0x68);
    if (*(char *)(in_RDX + 0x70) == '\0') {
      if (lVar10 != *(longlong *)(in_RDX + 0x60)) goto LAB_140dcdb3b;
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) goto LAB_140dcdd91;
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar2 = *puVar9;
        lVar10 = *(longlong *)(*(longlong *)(in_RDX + 0x10) + 8);
        puVar9 = puVar9 + 1;
      } while (*(longlong *)(lVar10 + 0x58 + (ulonglong)uVar2 * 0xa0) == 0);
      lVar10 = lVar10 + (ulonglong)uVar2 * 0xa0;
      lVar6 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
      lVar1 = lVar6 + (ulonglong)*(uint *)(lVar10 + 0x94) * 0x48;
      uVar4 = *(ulonglong *)(lVar6 + 0x40 + (ulonglong)*(uint *)(lVar10 + 0x94) * 0x48);
      if ((*(ulonglong *)(lVar3 + 0x110) < uVar4) &&
         (uVar5 = *(ulonglong *)(*(longlong *)(lVar1 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
         uVar5 != 0)) {
        uVar7 = *(ulonglong *)(lVar3 + 0x118);
        *(undefined8 *)(in_RDX + 0x50) =
             *(undefined8 *)(*(longlong *)(lVar1 + 0x18) + 0x10 + ~uVar5 * 0x30);
        if (uVar4 <= uVar7) goto LAB_140dcdd58;
LAB_140dcdd18:
        uVar4 = *(ulonglong *)(*(longlong *)(lVar1 + 0x38) + uVar7 * 8);
        if (uVar4 == 0) goto LAB_140dcdd58;
        uVar8 = *(undefined8 *)(*(longlong *)(lVar1 + 0x18) + 0x10 + ~uVar4 * 0x30);
      }
      else {
        uVar7 = *(ulonglong *)(lVar3 + 0x118);
        *(undefined8 *)(in_RDX + 0x50) = 0;
        if (uVar7 < uVar4) goto LAB_140dcdd18;
LAB_140dcdd58:
        uVar8 = 0;
      }
      *(undefined8 *)(in_RDX + 0x58) = uVar8;
      uVar8 = *(undefined8 *)(lVar10 + 0x58);
      *(undefined8 *)(in_RDX + 0x40) = *(undefined8 *)(lVar10 + 0x50);
      *(undefined8 *)(in_RDX + 0x48) = uVar8;
      *(undefined8 *)(in_RDX + 0x60) = *(undefined8 *)(lVar10 + 0x58);
      *(undefined8 *)(in_RDX + 0x68) = 0;
      lVar10 = 0;
    }
    else if (lVar10 == *(longlong *)(in_RDX + 0x60)) {
      puVar9 = *(uint **)(in_RDX + 0x20);
      do {
        if (puVar9 == *(uint **)(in_RDX + 0x28)) goto LAB_140dcdd91;
        *(uint **)(in_RDX + 0x20) = puVar9 + 1;
        uVar2 = *puVar9;
        lVar10 = *(longlong *)(*(longlong *)(in_RDX + 8) + 8);
        puVar9 = puVar9 + 1;
      } while (*(longlong *)(lVar10 + 0x10 + (ulonglong)uVar2 * 0x48) == 0);
      lVar10 = lVar10 + (ulonglong)uVar2 * 0x48;
      uVar4 = *(ulonglong *)(lVar10 + 0x40);
      if ((*(ulonglong *)(lVar3 + 0x110) < uVar4) &&
         (uVar5 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
         uVar5 != 0)) {
        uVar7 = *(ulonglong *)(lVar3 + 0x118);
        *(undefined8 *)(in_RDX + 0x50) =
             *(undefined8 *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar5 * 0x30);
        if (uVar4 <= uVar7) goto LAB_140dcdd3b;
LAB_140dcdce3:
        uVar4 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + uVar7 * 8);
        if (uVar4 == 0) goto LAB_140dcdd3b;
        uVar8 = *(undefined8 *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar4 * 0x30);
      }
      else {
        uVar7 = *(ulonglong *)(lVar3 + 0x118);
        *(undefined8 *)(in_RDX + 0x50) = 0;
        if (uVar7 < uVar4) goto LAB_140dcdce3;
LAB_140dcdd3b:
        uVar8 = 0;
      }
      *(undefined8 *)(in_RDX + 0x58) = uVar8;
      uVar8 = *(undefined8 *)(lVar10 + 0x10);
      *(undefined8 *)(in_RDX + 0x30) = *(undefined8 *)(lVar10 + 8);
      *(undefined8 *)(in_RDX + 0x38) = uVar8;
      *(undefined8 *)(in_RDX + 0x60) = *(undefined8 *)(lVar10 + 0x10);
      lVar10 = 0;
    }
LAB_140dcdb3b:
    *(longlong *)(in_RDX + 0x68) = lVar10 + 1;
    ops::function::impls::impl$3::
    call_mut<tuple_<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>
              ();
  } while ((local_58 & 1) == 0);
  in_RCX[4] = local_38;
  in_RCX[5] = uStack_30;
  in_RCX[2] = local_48;
  in_RCX[3] = uStack_40;
  uVar8 = 1;
LAB_140dcdd93:
  *in_RCX = uVar8;
  in_RCX[1] = 0;
  return;
LAB_140dcdd91:
  uVar8 = 0;
  goto LAB_140dcdd93;
}

