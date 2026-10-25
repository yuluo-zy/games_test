
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::
build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>(void)

{
  undefined8 *puVar1;
  float *pfVar2;
  byte bVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  int iVar15;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong lVar16;
  byte *in_RCX;
  uint in_EDX;
  longlong *plVar17;
  byte bVar18;
  longlong lVar19;
  float fVar27;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  float fVar28;
  float fVar29;
  float fVar37;
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  undefined8 local_118;
  undefined4 local_110;
  longlong *local_100;
  longlong *local_f8;
  undefined8 local_e8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong *local_c8;
  undefined4 uStack_b8;
  undefined4 local_b4;
  
  fVar6 = *(float *)(in_RCX + 0xc0);
  fVar7 = *(float *)(in_RCX + 0xc4);
  fVar8 = *(float *)(in_RCX + 200);
  fVar44 = *(float *)(in_RCX + 0xcc);
  fVar42 = (float)*(undefined8 *)(in_RCX + 0xd0);
  fVar43 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0xd0) >> 0x20);
  fVar41 = *(float *)(in_RCX + 0xd8);
  local_110 = *(undefined4 *)(in_RCX + 0xe4);
  local_118 = *(undefined8 *)(in_RCX + 0xdc);
  fVar38 = *(float *)(in_RCX + 0x140);
  fVar29 = (float)*(undefined8 *)(in_RCX + 0x144);
  fVar37 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0x144) >> 0x20);
  if (0.0 < SQRT(fVar37 * fVar37 + fVar38 * fVar38 + fVar29 * fVar29)) {
    fVar39 = fVar44 * fVar44;
    fVar40 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar28 = fVar8 * fVar37 + fVar7 * fVar29 + fVar6 * fVar38;
    fVar28 = fVar28 + fVar28;
    fVar27 = fVar44 + fVar44;
    fVar42 = fVar42 + fVar27 * (fVar7 * fVar37 - fVar29 * fVar8) +
                      fVar28 * fVar6 + (fVar39 - fVar40) * fVar38;
    fVar43 = fVar43 + fVar27 * (fVar8 * fVar38 - fVar37 * fVar6) +
                      fVar28 * fVar7 + (fVar39 - fVar40) * fVar29;
    fVar41 = fVar41 + fVar27 * (fVar6 * fVar29 - fVar38 * fVar7) +
                      fVar28 * fVar8 + (fVar39 - fVar40) * fVar37;
  }
  if (*(int *)(in_RCX + 0x30) == 1) {
    local_110 = *(undefined4 *)(in_RCX + 0x3c);
    local_118 = *(undefined8 *)(in_RCX + 0x34);
  }
  uVar5 = *(undefined8 *)(in_RCX + 0xe8);
  plVar17 = *(longlong **)(in_RCX + 0xb0);
  switch(*(undefined8 *)(in_RCX + 0xb8)) {
  case 9:
    if ((char)plVar17[1] == 'r' && *plVar17 == 0x6562755f696e616e) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::UberInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 10:
    if ((short)plVar17[1] == 0x6874 && *plVar17 == 0x6f6c635f696e616e) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::ClothInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    if ((short)plVar17[1] == 0x656b && *plVar17 == 0x6f6d735f696e616e) {
      country_core::resources::walls::decorator::DecoratorRank::new();
      country_core::components::drawable::impl$1::from();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SmokeInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 0x11:
    auVar20[0] = -((char)*plVar17 == __xmm_6f6f645f796e6f636c61625f696e616e);
    auVar20[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142c2adb1);
    auVar20[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142c2adb2);
    auVar20[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142c2adb3);
    auVar20[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142c2adb4);
    auVar20[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142c2adb5);
    auVar20[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142c2adb6);
    auVar20[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142c2adb7);
    auVar20[8] = -((char)plVar17[1] == UNK_142c2adb8);
    auVar20[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142c2adb9);
    auVar20[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142c2adba);
    auVar20[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142c2adbb);
    auVar20[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2adbc);
    auVar20[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2adbd);
    auVar20[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2adbe);
    auVar20[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2adbf);
    auVar30[0] = -((char)plVar17[2] == __xmm_00000000000000000000000000000072);
    auVar30[1] = -(UNK_14292c101 == '\0');
    auVar30[2] = -(UNK_14292c102 == '\0');
    auVar30[3] = -(UNK_14292c103 == '\0');
    auVar30[4] = -(UNK_14292c104 == '\0');
    auVar30[5] = -(UNK_14292c105 == '\0');
    auVar30[6] = -(UNK_14292c106 == '\0');
    auVar30[7] = -(UNK_14292c107 == '\0');
    auVar30[8] = -(UNK_14292c108 == '\0');
    auVar30[9] = -(UNK_14292c109 == '\0');
    auVar30[10] = -(UNK_14292c10a == '\0');
    auVar30[0xb] = -(UNK_14292c10b == '\0');
    auVar30[0xc] = -(UNK_14292c10c == '\0');
    auVar30[0xd] = -(UNK_14292c10d == '\0');
    auVar30[0xe] = -(UNK_14292c10e == '\0');
    auVar30[0xf] = -(UNK_14292c10f == '\0');
    auVar30 = auVar30 & auVar20;
    if ((ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar30[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::BalconyDoorInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 0x15:
    auVar35[0] = -(*(char *)((longlong)plVar17 + 5) == __xmm_6576697373696d655f6e7265746e616c);
    auVar35[1] = -(*(char *)((longlong)plVar17 + 6) == UNK_142c2adc1);
    auVar35[2] = -(*(char *)((longlong)plVar17 + 7) == UNK_142c2adc2);
    auVar35[3] = -((char)plVar17[1] == UNK_142c2adc3);
    auVar35[4] = -(*(char *)((longlong)plVar17 + 9) == UNK_142c2adc4);
    auVar35[5] = -(*(char *)((longlong)plVar17 + 10) == UNK_142c2adc5);
    auVar35[6] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142c2adc6);
    auVar35[7] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2adc7);
    auVar35[8] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2adc8);
    auVar35[9] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2adc9);
    auVar35[10] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2adca);
    auVar35[0xb] = -((char)plVar17[2] == UNK_142c2adcb);
    auVar35[0xc] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142c2adcc);
    auVar35[0xd] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142c2adcd);
    auVar35[0xe] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142c2adce);
    auVar35[0xf] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142c2adcf);
    auVar25[0] = -((char)*plVar17 == __xmm_696d655f6e7265746e616c5f696e616e);
    auVar25[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142c2add1);
    auVar25[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142c2add2);
    auVar25[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142c2add3);
    auVar25[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142c2add4);
    auVar25[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142c2add5);
    auVar25[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142c2add6);
    auVar25[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142c2add7);
    auVar25[8] = -((char)plVar17[1] == UNK_142c2add8);
    auVar25[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142c2add9);
    auVar25[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142c2adda);
    auVar25[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142c2addb);
    auVar25[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2addc);
    auVar25[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2addd);
    auVar25[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2adde);
    auVar25[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2addf);
    auVar25 = auVar25 & auVar35;
    if ((ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar25[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::LanternEmissiveInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 0x17:
    auVar36[0] = -(*(char *)((longlong)plVar17 + 7) == __xmm_726f6c6f635f7865747265765f64696c);
    auVar36[1] = -((char)plVar17[1] == UNK_142c2ade1);
    auVar36[2] = -(*(char *)((longlong)plVar17 + 9) == UNK_142c2ade2);
    auVar36[3] = -(*(char *)((longlong)plVar17 + 10) == UNK_142c2ade3);
    auVar36[4] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142c2ade4);
    auVar36[5] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2ade5);
    auVar36[6] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2ade6);
    auVar36[7] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2ade7);
    auVar36[8] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2ade8);
    auVar36[9] = -((char)plVar17[2] == UNK_142c2ade9);
    auVar36[10] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142c2adea);
    auVar36[0xb] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142c2adeb);
    auVar36[0xc] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142c2adec);
    auVar36[0xd] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142c2aded);
    auVar36[0xe] = -(*(char *)((longlong)plVar17 + 0x15) == UNK_142c2adee);
    auVar36[0xf] = -(*(char *)((longlong)plVar17 + 0x16) == UNK_142c2adef);
    auVar26[0] = -((char)*plVar17 == __xmm_65747265765f64696c6f735f696e616e);
    auVar26[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142abe211);
    auVar26[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142abe212);
    auVar26[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142abe213);
    auVar26[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142abe214);
    auVar26[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142abe215);
    auVar26[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142abe216);
    auVar26[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142abe217);
    auVar26[8] = -((char)plVar17[1] == UNK_142abe218);
    auVar26[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142abe219);
    auVar26[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142abe21a);
    auVar26[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142abe21b);
    auVar26[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142abe21c);
    auVar26[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142abe21d);
    auVar26[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142abe21e);
    auVar26[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe21f);
    auVar26 = auVar26 & auVar36;
    if ((ushort)((ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar26[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_00;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 3;
code_r0x0001412aa5ac:
      *(undefined1 *)(lVar16 + 0x58 + lVar19) = 0;
      *(ulonglong *)(lVar16 + 0x68 + lVar19) = CONCAT44(local_b4,uStack_b8);
      *(code **)(lVar16 + 0x78 + lVar19) = bevy_ecs::error::handler::panic;
      lVar19 = lVar19 + 0x88;
code_r0x0001412aa6ad:
      plVar17[2] = lVar19;
      local_e8 = CONCAT44(local_b4,uStack_b8);
      local_100 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      local_f8 = local_c8;
      goto LAB_1412aa6d5;
    }
    break;
  case 0x19:
    auVar31[0] = -(*(char *)((longlong)plVar17 + 9) == __xmm_736b636972625f776f646e69775f6369);
    auVar31[1] = -(*(char *)((longlong)plVar17 + 10) == UNK_142abe1e1);
    auVar31[2] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142abe1e2);
    auVar31[3] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142abe1e3);
    auVar31[4] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142abe1e4);
    auVar31[5] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142abe1e5);
    auVar31[6] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe1e6);
    auVar31[7] = -((char)plVar17[2] == UNK_142abe1e7);
    auVar31[8] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142abe1e8);
    auVar31[9] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142abe1e9);
    auVar31[10] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142abe1ea);
    auVar31[0xb] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142abe1eb);
    auVar31[0xc] = -(*(char *)((longlong)plVar17 + 0x15) == UNK_142abe1ec);
    auVar31[0xd] = -(*(char *)((longlong)plVar17 + 0x16) == UNK_142abe1ed);
    auVar31[0xe] = -(*(char *)((longlong)plVar17 + 0x17) == UNK_142abe1ee);
    auVar31[0xf] = -((char)plVar17[3] == UNK_142abe1ef);
    auVar21[0] = -((char)*plVar17 == __xmm_646e69775f636968746f675f696e616e);
    auVar21[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142abe1f1);
    auVar21[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142abe1f2);
    auVar21[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142abe1f3);
    auVar21[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142abe1f4);
    auVar21[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142abe1f5);
    auVar21[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142abe1f6);
    auVar21[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142abe1f7);
    auVar21[8] = -((char)plVar17[1] == UNK_142abe1f8);
    auVar21[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142abe1f9);
    auVar21[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142abe1fa);
    auVar21[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142abe1fb);
    auVar21[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142abe1fc);
    auVar21[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142abe1fd);
    auVar21[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142abe1fe);
    auVar21[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe1ff);
    auVar21 = auVar21 & auVar31;
    if ((ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar21[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 0x1b:
    auVar32[0] = -(*(char *)((longlong)plVar17 + 0xb) == __xmm_70614d6f6465626c4164696c6f535f72);
    auVar32[1] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2adf1);
    auVar32[2] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2adf2);
    auVar32[3] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2adf3);
    auVar32[4] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2adf4);
    auVar32[5] = -((char)plVar17[2] == UNK_142c2adf5);
    auVar32[6] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142c2adf6);
    auVar32[7] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142c2adf7);
    auVar32[8] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142c2adf8);
    auVar32[9] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142c2adf9);
    auVar32[10] = -(*(char *)((longlong)plVar17 + 0x15) == UNK_142c2adfa);
    auVar32[0xb] = -(*(char *)((longlong)plVar17 + 0x16) == UNK_142c2adfb);
    auVar32[0xc] = -(*(char *)((longlong)plVar17 + 0x17) == UNK_142c2adfc);
    auVar32[0xd] = -((char)plVar17[3] == UNK_142c2adfd);
    auVar32[0xe] = -(*(char *)((longlong)plVar17 + 0x19) == UNK_142c2adfe);
    auVar32[0xf] = -(*(char *)((longlong)plVar17 + 0x1a) == UNK_142c2adff);
    auVar22[0] = -((char)*plVar17 == __xmm_6c6f535f72657474756c635f696e616e);
    auVar22[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142c2ae01);
    auVar22[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142c2ae02);
    auVar22[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142c2ae03);
    auVar22[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142c2ae04);
    auVar22[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142c2ae05);
    auVar22[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142c2ae06);
    auVar22[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142c2ae07);
    auVar22[8] = -((char)plVar17[1] == UNK_142c2ae08);
    auVar22[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142c2ae09);
    auVar22[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142c2ae0a);
    auVar22[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142c2ae0b);
    auVar22[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142c2ae0c);
    auVar22[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142c2ae0d);
    auVar22[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142c2ae0e);
    auVar22[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142c2ae0f);
    auVar22 = auVar22 & auVar32;
    if ((ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar22[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bVar18 = in_RCX[0x150];
      if (bVar18 == 9) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 0xf;
code_r0x0001412aa687:
      *(byte *)(lVar16 + 0x58 + lVar19) = bVar18;
      *(undefined1 *)(lVar16 + 0x68 + lVar19) = 0;
      *(ulonglong *)(lVar16 + 0x78 + lVar19) = CONCAT44(local_b4,uStack_b8);
      *(code **)(lVar16 + 0x88 + lVar19) = bevy_ecs::error::handler::panic;
      lVar19 = lVar19 + 0x98;
      goto code_r0x0001412aa6ad;
    }
    break;
  case 0x1e:
    auVar33[0] = -(*(char *)((longlong)plVar17 + 0xe) == __xmm_776f646e69775f726f6c6f635f786574);
    auVar33[1] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe281);
    auVar33[2] = -((char)plVar17[2] == UNK_142abe282);
    auVar33[3] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142abe283);
    auVar33[4] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142abe284);
    auVar33[5] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142abe285);
    auVar33[6] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142abe286);
    auVar33[7] = -(*(char *)((longlong)plVar17 + 0x15) == UNK_142abe287);
    auVar33[8] = -(*(char *)((longlong)plVar17 + 0x16) == UNK_142abe288);
    auVar33[9] = -(*(char *)((longlong)plVar17 + 0x17) == UNK_142abe289);
    auVar33[10] = -((char)plVar17[3] == UNK_142abe28a);
    auVar33[0xb] = -(*(char *)((longlong)plVar17 + 0x19) == UNK_142abe28b);
    auVar33[0xc] = -(*(char *)((longlong)plVar17 + 0x1a) == UNK_142abe28c);
    auVar33[0xd] = -(*(char *)((longlong)plVar17 + 0x1b) == UNK_142abe28d);
    auVar33[0xe] = -(*(char *)((longlong)plVar17 + 0x1c) == UNK_142abe28e);
    auVar33[0xf] = -(*(char *)((longlong)plVar17 + 0x1d) == UNK_142abe28f);
    auVar23[0] = -((char)*plVar17 == __xmm_65747265765f64696c6f735f696e616e);
    auVar23[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142abe211);
    auVar23[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142abe212);
    auVar23[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142abe213);
    auVar23[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142abe214);
    auVar23[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142abe215);
    auVar23[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142abe216);
    auVar23[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142abe217);
    auVar23[8] = -((char)plVar17[1] == UNK_142abe218);
    auVar23[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142abe219);
    auVar23[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142abe21a);
    auVar23[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142abe21b);
    auVar23[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142abe21c);
    auVar23[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142abe21d);
    auVar23[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142abe21e);
    auVar23[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe21f);
    auVar23 = auVar23 & auVar33;
    if ((ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar23[0xf] >> 7) << 0xf) == 0xffff) {
      if (*(longlong *)in_RCX != 0) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_03;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 4;
      goto code_r0x0001412aa5ac;
    }
    auVar34[0] = -(*(char *)((longlong)plVar17 + 0xe) == __xmm_736b636972625f726f6c6f635f786574);
    auVar34[1] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe201);
    auVar34[2] = -((char)plVar17[2] == UNK_142abe202);
    auVar34[3] = -(*(char *)((longlong)plVar17 + 0x11) == UNK_142abe203);
    auVar34[4] = -(*(char *)((longlong)plVar17 + 0x12) == UNK_142abe204);
    auVar34[5] = -(*(char *)((longlong)plVar17 + 0x13) == UNK_142abe205);
    auVar34[6] = -(*(char *)((longlong)plVar17 + 0x14) == UNK_142abe206);
    auVar34[7] = -(*(char *)((longlong)plVar17 + 0x15) == UNK_142abe207);
    auVar34[8] = -(*(char *)((longlong)plVar17 + 0x16) == UNK_142abe208);
    auVar34[9] = -(*(char *)((longlong)plVar17 + 0x17) == UNK_142abe209);
    auVar34[10] = -((char)plVar17[3] == UNK_142abe20a);
    auVar34[0xb] = -(*(char *)((longlong)plVar17 + 0x19) == UNK_142abe20b);
    auVar34[0xc] = -(*(char *)((longlong)plVar17 + 0x1a) == UNK_142abe20c);
    auVar34[0xd] = -(*(char *)((longlong)plVar17 + 0x1b) == UNK_142abe20d);
    auVar34[0xe] = -(*(char *)((longlong)plVar17 + 0x1c) == UNK_142abe20e);
    auVar34[0xf] = -(*(char *)((longlong)plVar17 + 0x1d) == UNK_142abe20f);
    auVar24[0] = -((char)*plVar17 == __xmm_65747265765f64696c6f735f696e616e);
    auVar24[1] = -(*(char *)((longlong)plVar17 + 1) == UNK_142abe211);
    auVar24[2] = -(*(char *)((longlong)plVar17 + 2) == UNK_142abe212);
    auVar24[3] = -(*(char *)((longlong)plVar17 + 3) == UNK_142abe213);
    auVar24[4] = -(*(char *)((longlong)plVar17 + 4) == UNK_142abe214);
    auVar24[5] = -(*(char *)((longlong)plVar17 + 5) == UNK_142abe215);
    auVar24[6] = -(*(char *)((longlong)plVar17 + 6) == UNK_142abe216);
    auVar24[7] = -(*(char *)((longlong)plVar17 + 7) == UNK_142abe217);
    auVar24[8] = -((char)plVar17[1] == UNK_142abe218);
    auVar24[9] = -(*(char *)((longlong)plVar17 + 9) == UNK_142abe219);
    auVar24[10] = -(*(char *)((longlong)plVar17 + 10) == UNK_142abe21a);
    auVar24[0xb] = -(*(char *)((longlong)plVar17 + 0xb) == UNK_142abe21b);
    auVar24[0xc] = -(*(char *)((longlong)plVar17 + 0xc) == UNK_142abe21c);
    auVar24[0xd] = -(*(char *)((longlong)plVar17 + 0xd) == UNK_142abe21d);
    auVar24[0xe] = -(*(char *)((longlong)plVar17 + 0xe) == UNK_142abe21e);
    auVar24[0xf] = -(*(char *)((longlong)plVar17 + 0xf) == UNK_142abe21f);
    auVar24 = auVar24 & auVar34;
    if ((ushort)((ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar24[0xf] >> 7) << 0xf) == 0xffff) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::
      spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>_>
                ();
      goto LAB_1412aa6d5;
    }
    break;
  case 0x23:
    iVar15 = memcmp(plVar17,&UNK_142c2ae10,0x23);
    if (iVar15 == 0) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bVar18 = in_RCX[0x150];
      if (bVar18 == 9) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_04;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 0x10;
      goto code_r0x0001412aa687;
    }
    break;
  case 0x24:
    iVar15 = memcmp(plVar17,&UNK_142c2ae33,0x24);
    if (iVar15 == 0) {
      ::utils::name_hash::NameHash::compute();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar17 = (longlong *)CONCAT44(uStack_cc,uStack_d0);
      if ((longlong *)CONCAT44(uStack_cc,uStack_d0) == (longlong *)0x0) {
        plVar17 = local_c8;
      }
      lVar19 = plVar17[2];
      if ((ulonglong)(*plVar17 - lVar19) < 0x88) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar16 = plVar17[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      pfVar2 = (float *)(lVar16 + 8 + lVar19);
      *pfVar2 = fVar6;
      pfVar2[1] = fVar7;
      pfVar2[2] = fVar8;
      pfVar2[3] = fVar44;
      *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
      *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
      *(undefined8 *)(lVar16 + 0x24 + lVar19) = local_118;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = local_110;
      *(undefined8 *)(lVar16 + 0x30 + lVar19) = uVar5;
      *(undefined4 *)(lVar16 + 0x38 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = extraout_RAX_01;
      *(undefined1 *)(lVar16 + 0x48 + lVar19) = 5;
      goto code_r0x0001412aa5ac;
    }
  }
  country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name();
  country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_02 & 1) == 0) {
                    /* WARNING: Subroutine does not return */
    core::option::expect_failed();
  }
  bevy_ecs::system::commands::Commands::
  spawn<country_core::components::drawable::DrawableMeshBundle>();
LAB_1412aa6d5:
  if ((in_RCX[0xf0] & 1) == 0) {
    iVar15 = *(int *)(in_RCX + 0x40);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar4 = *(undefined4 *)(in_RCX + 0xf4);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined4 *)(lVar16 + 0x18 + lVar19) = uVar4;
    *(undefined1 *)(lVar16 + 0x1c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
    iVar15 = *(int *)(in_RCX + 0x40);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x58) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::GothicWindowBrickDeformation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x4c);
    uVar9 = *(undefined8 *)(in_RCX + 0x54);
    uVar10 = *(undefined8 *)(in_RCX + 0x5c);
    uVar11 = *(undefined8 *)(in_RCX + 100);
    uVar12 = *(undefined8 *)(in_RCX + 0x6c);
    uVar13 = *(undefined8 *)(in_RCX + 0x70);
    uVar14 = *(undefined8 *)(in_RCX + 0x78);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 0x44);
    puVar1[1] = uVar5;
    puVar1 = (undefined8 *)(lVar16 + 0x28 + lVar19);
    *puVar1 = uVar9;
    puVar1[1] = uVar10;
    puVar1 = (undefined8 *)(lVar16 + 0x38 + lVar19);
    *puVar1 = uVar11;
    puVar1[1] = uVar12;
    puVar1 = (undefined8 *)(lVar16 + 0x44 + lVar19);
    *puVar1 = uVar13;
    puVar1[1] = uVar14;
    *(undefined1 *)(lVar16 + 0x54 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x58;
    bVar18 = in_RCX[0x114];
  }
  else {
    bVar18 = in_RCX[0x114];
  }
  if (bVar18 == 2) {
    iVar15 = *(int *)(in_RCX + 0x80);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::BrickColor>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x108);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 0x100);
    puVar1[1] = uVar5;
    *(undefined8 *)(lVar16 + 0x28 + lVar19) = *(undefined8 *)(in_RCX + 0x110);
    *(undefined1 *)(lVar16 + 0x30 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x38;
    iVar15 = *(int *)(in_RCX + 0x80);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::ClothlineCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = *(undefined8 *)(in_RCX + 0x84);
    *(undefined4 *)(lVar16 + 0x20 + lVar19) = *(undefined4 *)(in_RCX + 0x8c);
    *(undefined1 *)(lVar16 + 0x24 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    bVar18 = in_RCX[0x14e];
  }
  else {
    bVar18 = in_RCX[0x14e];
  }
  if (bVar18 == 0) {
    bVar18 = in_RCX[0x10];
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::SemiTransparent>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined1 *)(lVar16 + 0x18 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
    bVar18 = in_RCX[0x10];
  }
  if ((bVar18 & 1) == 0) {
    bVar18 = in_RCX[0x14f];
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 0x18);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::window_lights::WindowLight>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(uint *)(lVar16 + 0x20 + lVar19) = in_EDX;
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    bVar18 = in_RCX[0x14f];
  }
  if (bVar18 == 0) {
    bVar18 = in_RCX[0x14c];
  }
  else {
    fVar38 = fVar44 * fVar44;
    fVar27 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar29 = (fVar38 - fVar27) * ___xmm_000000003f8000000000000000000000;
    fVar37 = (fVar38 - fVar27) * _UNK_1429cd0c4;
    fVar27 = (fVar38 - fVar27) * _UNK_1429cd0c8;
    fVar38 = _UNK_1429cd0c8 * fVar8 +
             _UNK_1429cd0c4 * fVar7 + ___xmm_000000003f8000000000000000000000 * fVar6;
    fVar38 = fVar38 + fVar38;
    fVar28 = fVar6 * __real_3f800000;
    fVar39 = fVar7 * __real_3f800000;
    fVar40 = fVar44 + fVar44;
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::BalconyDoor>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) = CONCAT44(fVar43,fVar42);
    *(float *)(lVar16 + 0x20 + lVar19) = fVar41;
    *(ulonglong *)(lVar16 + 0x24 + lVar19) =
         CONCAT44(fVar40 * (fVar8 * 0.0 - fVar28) + fVar38 * fVar7 + fVar37,
                  fVar40 * (fVar39 - fVar8 * 0.0) + fVar38 * fVar6 + fVar29);
    *(float *)(lVar16 + 0x2c + lVar19) =
         fVar40 * (fVar6 * 0.0 - fVar7 * 0.0) + fVar38 * fVar8 + fVar27;
    *(undefined1 *)(lVar16 + 0x30 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x38;
    bVar18 = in_RCX[0x14c];
  }
  if (bVar18 == 2) {
    iVar15 = *(int *)(in_RCX + 0x20);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    bVar3 = in_RCX[0x14d];
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::Smoke>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) = (ulonglong)in_EDX;
    *(undefined4 *)(lVar16 + 0x20 + lVar19) = 0;
    *(byte *)(lVar16 + 0x24 + lVar19) = bVar18;
    *(byte *)(lVar16 + 0x25 + lVar19) = bVar3;
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    iVar15 = *(int *)(in_RCX + 0x20);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 0x28);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::AlbedoMap>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(undefined1 *)(lVar16 + 0x20 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    bVar18 = *in_RCX;
  }
  else {
    bVar18 = *in_RCX;
  }
  if ((bVar18 & 1) == 0) {
    iVar15 = *(int *)(in_RCX + 0xf8);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar5 = *(undefined8 *)(in_RCX + 8);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::NormalMap>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined8 *)(lVar16 + 0x18 + lVar19) = uVar5;
    *(undefined1 *)(lVar16 + 0x20 + lVar19) = 0;
    plVar17[2] = lVar19 + 0x28;
    iVar15 = *(int *)(in_RCX + 0xf8);
  }
  if (iVar15 == 1) {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    uVar4 = *(undefined4 *)(in_RCX + 0xfc);
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::RoofFlagCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(undefined4 *)(lVar16 + 0x18 + lVar19) = uVar4;
    *(undefined1 *)(lVar16 + 0x1c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x20;
  }
  if (in_RCX[0x13c] == 2) {
    iVar15 = *(int *)(in_RCX + 0x118);
  }
  else {
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::WallFlagCloth>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    uVar5 = *(undefined8 *)(in_RCX + 0x134);
    puVar1 = (undefined8 *)(lVar16 + 0x18 + lVar19);
    *puVar1 = *(undefined8 *)(in_RCX + 300);
    puVar1[1] = uVar5;
    *(undefined4 *)(lVar16 + 0x28 + lVar19) = *(undefined4 *)(in_RCX + 0x13c);
    *(undefined1 *)(lVar16 + 0x2c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x30;
    iVar15 = *(int *)(in_RCX + 0x118);
  }
  if (iVar15 == 1) {
    fVar38 = *(float *)(in_RCX + 0x11c);
    fVar29 = *(float *)(in_RCX + 0x120);
    fVar37 = *(float *)(in_RCX + 0x124);
    fVar27 = *(float *)(in_RCX + 0x128);
    fVar39 = fVar44 * fVar44;
    fVar40 = fVar8 * fVar8 + fVar7 * fVar7 + fVar6 * fVar6;
    fVar28 = fVar8 * fVar37 + fVar7 * fVar29 + fVar6 * fVar38;
    fVar28 = fVar28 + fVar28;
    fVar44 = fVar44 + fVar44;
    plVar17 = local_100;
    if (local_100 == (longlong *)0x0) {
      plVar17 = local_f8;
    }
    lVar19 = plVar17[2];
    if ((ulonglong)(*plVar17 - lVar19) < 0x40) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = plVar17[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::LanternLightParams>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(ulonglong *)(lVar16 + 0x18 + lVar19) =
         CONCAT44(fVar43 + fVar44 * (fVar8 * fVar38 - fVar37 * fVar6) +
                           fVar28 * fVar7 + (fVar39 - fVar40) * fVar29,
                  fVar42 + fVar44 * (fVar7 * fVar37 - fVar29 * fVar8) +
                           fVar28 * fVar6 + (fVar39 - fVar40) * fVar38);
    *(float *)(lVar16 + 0x20 + lVar19) =
         fVar41 + fVar44 * (fVar6 * fVar29 - fVar38 * fVar7) +
                  fVar28 * fVar8 + (fVar39 - fVar40) * fVar37;
    pfVar2 = (float *)(lVar16 + 0x24 + lVar19);
    *pfVar2 = fVar38;
    pfVar2[1] = fVar29;
    pfVar2[2] = fVar37;
    pfVar2[3] = fVar27;
    *(uint *)(lVar16 + 0x34 + lVar19) = in_EDX;
    *(undefined1 *)(lVar16 + 0x38 + lVar19) = 0;
    *(undefined1 *)(lVar16 + 0x3c + lVar19) = 0;
    plVar17[2] = lVar19 + 0x40;
    iVar15 = *(int *)(in_RCX + 0x90);
  }
  else {
    iVar15 = *(int *)(in_RCX + 0x90);
  }
  if (iVar15 == 1) {
    if (local_100 == (longlong *)0x0) {
      local_100 = local_f8;
    }
    lVar19 = local_100[2];
    if ((ulonglong)(*local_100 - lVar19) < 0x30) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar16 = local_100[1];
    *(code **)(lVar16 + lVar19) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::SittableForBirds>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar16 + 0x10 + lVar19) = local_e8;
    *(uint *)(lVar16 + 0x18 + lVar19) = in_EDX;
    *(undefined8 *)(lVar16 + 0x1c + lVar19) = *(undefined8 *)(in_RCX + 0x94);
    *(undefined4 *)(lVar16 + 0x24 + lVar19) = *(undefined4 *)(in_RCX + 0x9c);
    *(undefined1 *)(lVar16 + 0x28 + lVar19) = 0;
    local_100[2] = lVar19 + 0x30;
  }
  decorator_visual_state::impl$4::add_decorator_visual_entity();
  return;
}

