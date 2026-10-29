
void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<tuple_<ref_<country_core::resources::walls::decorator::DecoratorId>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>_>,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env_0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env_1>_>
          (void)

{
  char cVar1;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 uVar2;
  int *in_RDX;
  byte bVar3;
  longlong in_R8;
  byte bVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined8 uStack_d8;
  float fStack_d0;
  undefined8 *puStack_c8;
  code *pcStack_c0;
  undefined **ppuStack_b8;
  undefined8 uStack_b0;
  undefined8 **ppuStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  float fStack_88;
  float fStack_84;
  float fStack_80;
  float fStack_7c;
  undefined8 uStack_78;
  float fStack_70;
  
  if (*(int *)**(undefined8 **)*in_RCX != *in_RDX) {
    switch(*(undefined1 *)(in_R8 + 0x22)) {
    case 2:
    case 4:
      cVar1 = *(char *)((undefined8 *)*in_RCX)[1];
      bVar4 = cVar1 - 0x38;
      if (7 < bVar4) {
        bVar4 = 6;
      }
      bVar3 = *(char *)(in_R8 + 0x46) - 0x38;
      if (7 < bVar3) {
        bVar3 = 6;
      }
      if ((bVar4 == bVar3) && (cVar1 == *(char *)(in_R8 + 0x46) || bVar4 != 6)) {
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        transform();
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        country_core::resources::walls::decorator::DecoratorRank::new();
        uVar2 = extraout_AL;
        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
        uStack_d8._0_2_ = CONCAT11(uVar2,extraout_AL_00);
        std::collections::hash::map::HashMap::
        get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                  ();
        if (extraout_RAX == 0) {
          pcStack_c0 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
          ppuStack_b8 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
          uStack_b0 = 1;
          uStack_98 = 0;
          ppuStack_a8 = &puStack_c8;
          uStack_a0 = 1;
          puStack_c8 = &uStack_d8;
                    // WARNING: Subroutine does not return
          panicking::panic_fmt();
        }
        fVar6 = *(float *)(extraout_RAX + 0x3c) - *(float *)(extraout_RAX + 0x38);
        fVar5 = fStack_7c * fStack_7c;
        fVar8 = fStack_80 * fStack_80 + fStack_84 * fStack_84 + fStack_88 * fStack_88;
        fVar9 = fStack_80 * 0.09900001 + fStack_84 * 0.0 + fStack_88 * 0.0;
        fVar9 = fVar9 + fVar9;
        fStack_7c = fStack_7c + fStack_7c;
        fVar7 = fVar6 * 0.0 * 0.5;
        fStack_d0 = (fVar7 + fStack_70) -
                    ((fStack_84 * -0.0 + fStack_88 * 0.0) * fStack_7c +
                    fVar9 * fStack_80 + (fVar5 - fVar8) * 0.09900001);
        uStack_d8 = CONCAT44(((float)((ulonglong)uStack_78 >> 0x20) + fVar6 * 0.5) -
                             ((fStack_88 * -0.09900001 + fStack_80 * 0.0) * fStack_7c +
                             fVar9 * fStack_84 + (fVar5 - fVar8) * 0.0),
                             ((float)uStack_78 + fVar7) -
                             ((fStack_80 * -0.0 + fStack_84 * 0.09900001) * fStack_7c +
                             fVar9 * fStack_88 + (fVar5 - fVar8) * 0.0));
        puStack_c8 = (undefined8 *)CONCAT44(fVar6 + 0.0,*(float *)(extraout_RAX + 0x30) + 0.4);
        pcStack_c0 = (code *)CONCAT44(pcStack_c0._4_4_,0x3e99999a);
        ::utils::geometry::rectangle::Rectangle3d::new();
        ::utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d();
      }
      break;
    case 3:
                    // WARNING: Subroutine does not return
      panicking::panic();
    }
  }
  return;
}

