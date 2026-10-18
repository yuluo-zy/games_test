// create_cart_chain @ 0x1403be1a0
// sig: void __cdecl create_cart_chain(void)


void __cdecl tiny_glade::rollercoaster::create_cart_chain(void)

{
  int *piVar1;
  float *pfVar2;
  char *pcVar3;
  float fVar4;
  undefined8 uVar5;
  char cVar6;
  undefined8 uVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  undefined8 uVar11;
  undefined8 *puVar12;
  undefined8 *extraout_RAX;
  ulonglong uVar13;
  longlong lVar14;
  longlong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong *extraout_RAX_02;
  undefined1 *extraout_RAX_03;
  ulonglong *extraout_RAX_04;
  longlong lVar15;
  undefined8 *in_RCX;
  longlong lVar16;
  undefined8 *puVar17;
  undefined8 *puVar18;
  longlong lVar19;
  undefined1 *puVar20;
  undefined4 uVar21;
  ulonglong uVar22;
  undefined8 *puVar23;
  uint uVar24;
  ushort uVar25;
  longlong *plVar26;
  undefined8 *puVar27;
  undefined8 *puVar28;
  byte bVar29;
  char cVar30;
  char cVar34;
  char cVar35;
  float fVar31;
  float fVar32;
  char cVar36;
  undefined1 auVar33 [16];
  float fVar37;
  float fVar38;
  undefined1 auVar39 [16];
  float fVar40;
  float fVar42;
  undefined1 auVar41 [16];
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  float local_3c8;
  float fStack_3c4;
  float fStack_3c0;
  float fStack_3bc;
  undefined8 local_3b8;
  float local_3b0;
  undefined8 local_3ac;
  undefined8 uStack_3a4;
  undefined4 local_39c;
  undefined8 local_398;
  undefined8 local_360;
  undefined8 local_328;
  undefined8 local_2f0;
  undefined8 local_2b8;
  undefined8 local_2a0;
  undefined8 local_288;
  undefined8 local_270;
  undefined4 local_268;
  undefined8 local_260;
  undefined4 local_254;
  undefined4 local_250;
  undefined8 local_24c;
  undefined8 uStack_244;
  undefined8 local_23c;
  undefined8 uStack_234;
  undefined8 local_22c;
  longlong local_210;
  undefined8 local_208;
  undefined2 local_200 [4];
  undefined8 local_1f8;
  float local_1f0;
  undefined8 local_1ec;
  float local_1e4;
  longlong local_1d8;
  longlong local_1c8;
  longlong local_1c0;
  undefined8 *local_1b8;
  undefined8 *local_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined4 local_198;
  undefined8 local_188;
  longlong *plStack_180;
  longlong *local_178;
  undefined8 *puStack_170;
  undefined8 local_168;
  int local_154;
  int local_13c;
  undefined8 *local_138;
  undefined8 *local_130;
  undefined8 *local_128;
  undefined8 *local_120;
  undefined8 *local_118;
  undefined1 *puStack_110;
  undefined8 *local_108;
  undefined8 *puStack_100;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_118 = (undefined8 *)0x0;
  local_108 = (undefined8 *)0x0;
  puStack_100 = (undefined8 *)0x0;
  puStack_110 = &DAT_00000010;
  local_1b8 = in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  puVar17 = (undefined8 *)in_stack_00000038[2];
  extraout_RAX[2] = puVar17;
  uVar5 = in_stack_00000038[1];
  *extraout_RAX = *in_stack_00000038;
  extraout_RAX[1] = uVar5;
  *(undefined1 *)(extraout_RAX + 3) = 1;
  local_138 = (undefined8 *)0x1;
  local_128 = (undefined8 *)0x1;
  local_1c8 = *in_stack_00000030;
  local_1c0 = in_stack_00000030[1];
  local_1d8 = in_stack_00000028 + 0x18;
  local_130 = extraout_RAX;
LAB_1403be305:
  puVar12 = puStack_100;
  puVar18 = local_108;
  puVar20 = puStack_110;
  puVar23 = local_118;
  if (local_128 != (undefined8 *)0x0) {
    local_128 = (undefined8 *)((longlong)local_128 + -1);
    bVar29 = *(byte *)(local_130 + (longlong)local_128 * 4 + 3);
    if (bVar29 != 2) {
      uVar13 = local_130[(longlong)local_128 * 4];
      uVar22 = uVar13 & 0xffffffff;
      if ((uVar22 < *(ulonglong *)(local_1c0 + 0x10)) &&
         (puVar23 = *(undefined8 **)(local_1c0 + 8), puVar17 = puVar23,
         *(int *)((longlong)puVar23 + uVar22 * 0x14) == (int)(uVar13 >> 0x20))) {
        uVar24 = *(uint *)((longlong)puVar23 + uVar22 * 0x14 + 4);
        puVar17 = (undefined8 *)(ulonglong)uVar24;
        if (puVar17 == (undefined8 *)0xffffffff) goto LAB_1403bee8d;
        uVar21 = 0;
        if ((puVar17 < *(undefined8 **)(local_1c8 + 0x38)) &&
           ((*(ulonglong *)(*(longlong *)(local_1c8 + 0x28) + (ulonglong)(uVar24 >> 6) * 8) >>
             ((ulonglong)puVar17 & 0x3f) & 1) != 0)) {
          local_120 = (undefined8 *)
                      CONCAT44(local_120._4_4_,
                               *(undefined4 *)(local_130 + (longlong)local_128 * 4 + 2));
          uVar13 = (ulonglong)*(uint *)((longlong)puVar23 + uVar22 * 0x14 + 0xc);
          lVar14 = *(longlong *)
                    (*(longlong *)(*(longlong *)(local_1c0 + 0x1a8) + 0x18 + uVar13 * 0x48) + 0x10 +
                    ~*(ulonglong *)
                      (*(longlong *)(*(longlong *)(local_1c0 + 0x1a8) + 0x38 + uVar13 * 0x48) +
                      *(longlong *)(local_1c8 + 0x110) * 8) * 0x30);
          lVar16 = (ulonglong)*(uint *)((longlong)puVar23 + uVar22 * 0x14 + 0x10) * 0x30;
          pfVar2 = (float *)(lVar14 + lVar16);
          fVar31 = *pfVar2;
          fVar37 = pfVar2[1];
          fVar49 = pfVar2[2];
          fVar50 = pfVar2[3];
          uVar5 = *(undefined8 *)(lVar14 + 0x10 + lVar16);
          fVar4 = *(float *)(lVar14 + 0x18 + lVar16);
          local_198 = *(undefined4 *)(lVar14 + 0x2c + lVar16);
          puVar17 = (undefined8 *)(lVar14 + 0x1c + lVar16);
          local_1a8 = *puVar17;
          uStack_1a0 = puVar17[1];
          local_f1 = 1;
          country_core::resources::clutter_meshes::clutter_mesh_ty_data();
          uVar7 = *(undefined8 *)((longlong)local_188 + 0x38);
          local_f1 = 1;
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          local_f1 = 1;
          bevy_ecs::system::commands::Commands::spawn_empty();
          uVar11 = local_168;
          plVar26 = plStack_180;
          if (plStack_180 == (longlong *)0x0) {
            plVar26 = local_178;
          }
          lVar14 = plVar26[2];
          if ((ulonglong)(*plVar26 - lVar14) < 0xb8) {
            local_f1 = 1;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar16 = plVar26[1];
          *(code **)(lVar16 + lVar14) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::VelocityOnlyInstanceData>,country_core::components::drawable::PrevTransform>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          pfVar2 = (float *)(lVar16 + 8 + lVar14);
          *pfVar2 = fVar31;
          pfVar2[1] = fVar37;
          pfVar2[2] = fVar49;
          pfVar2[3] = fVar50;
          *(undefined8 *)(lVar16 + 0x18 + lVar14) = uVar5;
          *(float *)(lVar16 + 0x20 + lVar14) = fVar4;
          puVar17 = (undefined8 *)(lVar16 + 0x24 + lVar14);
          *puVar17 = local_1a8;
          puVar17[1] = uStack_1a0;
          *(undefined4 *)(lVar16 + 0x34 + lVar14) = local_198;
          *(undefined4 *)(lVar16 + 0x38 + lVar14) = 0;
          *(undefined8 *)(lVar16 + 0x40 + lVar14) = uVar7;
          *(undefined1 *)(lVar16 + 0x48 + lVar14) = 0x19;
          pfVar2 = (float *)(lVar16 + 0x58 + lVar14);
          *pfVar2 = fVar31;
          pfVar2[1] = fVar37;
          pfVar2[2] = fVar49;
          pfVar2[3] = fVar50;
          *(undefined8 *)(lVar16 + 0x68 + lVar14) = uVar5;
          *(float *)(lVar16 + 0x70 + lVar14) = fVar4;
          puVar17 = (undefined8 *)(lVar16 + 0x74 + lVar14);
          *puVar17 = local_1a8;
          puVar17[1] = uStack_1a0;
          *(undefined4 *)(lVar16 + 0x84 + lVar14) = local_198;
          *(undefined1 *)(lVar16 + 0x88 + lVar14) = 0;
          *(undefined8 *)(lVar16 + 0x98 + lVar14) = uVar11;
          *(code **)(lVar16 + 0xa8 + lVar14) = bevy_ecs::error::handler::panic;
          plVar26[2] = lVar14 + 0xb8;
          local_39c = local_198;
          local_3ac = local_1a8;
          uStack_3a4 = uStack_1a0;
          local_250 = local_120._0_4_;
          local_260 = uVar11;
          local_254 = 0;
          local_23c = 0;
          uStack_234 = 0;
          local_24c = 0;
          uStack_244 = 0;
          local_22c = 0;
          local_398 = 2;
          local_360 = 2;
          local_328 = 2;
          local_2f0 = 2;
          local_2b8 = 0x8000000000000000;
          local_2a0 = 0x8000000000000000;
          local_288 = 0x8000000000000000;
          local_270 = 0;
          local_268 = 0;
          local_3c8 = fVar31;
          fStack_3c4 = fVar37;
          fStack_3c0 = fVar49;
          fStack_3bc = fVar50;
          local_3b8 = uVar5;
          local_3b0 = fVar4;
          if ((bVar29 & 1) == 0) {
            if (puStack_100 == local_118) {
              alloc::collections::vec_deque::VecDeque::
              grow<tiny_glade::rollercoaster::CoasterCart,alloc::alloc::Global>();
            }
            puVar23 = puStack_100;
            puVar18 = (undefined8 *)0x0;
            if (local_118 <= (undefined8 *)((longlong)local_108 + (longlong)puStack_100)) {
              puVar18 = local_118;
            }
            puVar17 = (undefined8 *)0x1b0;
            memmove(puStack_110 +
                    (((longlong)local_108 + (longlong)puStack_100) - (longlong)puVar18) * 0x1b0,
                    &local_3c8,0x1b0);
            puStack_100 = (undefined8 *)((longlong)puVar23 + 1);
          }
          else {
            if (puStack_100 == local_118) {
              alloc::collections::vec_deque::VecDeque::
              grow<tiny_glade::rollercoaster::CoasterCart,alloc::alloc::Global>();
            }
            puVar17 = (undefined8 *)((longlong)local_108 + -1);
            local_108 = (undefined8 *)((longlong)local_118 + (longlong)puVar17);
            if (!CARRY8((ulonglong)local_118,(ulonglong)puVar17)) {
              local_108 = puVar17;
            }
            puStack_100 = (undefined8 *)((longlong)puStack_100 + 1);
            puVar17 = (undefined8 *)0x1b0;
            memmove(puStack_110 + (longlong)local_108 * 0x1b0,&local_3c8,0x1b0);
          }
          local_f1 = 1;
          ::utils::geometry::rectangle::Rectangle3d::extent();
          fVar46 = fVar37 * 0.0 * 1.0 + fVar49 * 0.70710677 * -1.0 +
                   fVar50 * 0.0 + fVar31 * 0.70710677 * 1.0;
          fVar47 = fVar37 * 0.70710677 * 1.0 + fVar49 * 0.0 * 1.0 +
                   fVar50 * 0.70710677 + fVar31 * 0.0 * -1.0;
          fVar48 = fVar37 * 0.0 * -1.0 + fVar49 * 0.70710677 * 1.0 +
                   fVar50 * 0.0 + fVar31 * 0.70710677 * 1.0;
          fVar49 = fVar37 * 0.70710677 * -1.0 + fVar49 * 0.0 * -1.0 +
                   fVar50 * 0.70710677 + fVar31 * 0.0 * -1.0;
          local_208 = 2;
          local_200[0] = 0x100;
          fVar31 = (float)local_188 * 0.5;
          fVar50 = fVar49 * fVar49;
          fVar37 = fVar48 * fVar48 + fVar47 * fVar47 + fVar46 * fVar46;
          fVar49 = fVar49 + fVar49;
          lVar14 = 0;
LAB_1403be7d3:
          if (lVar14 != 2) {
            lVar16 = lVar14 + 1;
            cVar6 = *(char *)((longlong)local_200 + lVar14);
            local_f1 = 1;
            local_210 = lVar16;
            ::utils::geometry::rectangle::Rectangle3d::extent();
            fVar32 = -1.0;
            if (cVar6 == '\0') {
              fVar32 = 1.0;
            }
            fVar43 = fVar32 * fVar31 * -0.0 * 1.1;
            fVar42 = local_188._4_4_ * 0.0 * 0.5;
            fVar38 = fVar42 + fVar43;
            fVar43 = local_188._4_4_ * 0.5 + fVar43;
            fVar42 = fVar42 - fVar32 * fVar31 * 1.1;
            fVar40 = fVar48 * fVar42 + fVar47 * fVar43 + fVar46 * fVar38;
            fVar40 = fVar40 + fVar40;
            local_1f0 = (fVar46 * fVar43 - fVar38 * fVar47) * fVar49 +
                        fVar40 * fVar48 + fVar42 * (fVar50 - fVar37) + fVar4;
            fVar32 = -fVar32;
            fVar44 = (float)((uint)fVar32 & 0x80000000);
            fVar45 = fVar48 * fVar32 + fVar47 * fVar44 + fVar46 * fVar44;
            fVar45 = fVar45 + fVar45;
            local_1ec = CONCAT44((fVar48 * fVar44 - fVar32 * fVar46) * fVar49 +
                                 fVar45 * fVar47 + (fVar50 - fVar37) * fVar44,
                                 (fVar47 * fVar32 - fVar44 * fVar48) * fVar49 +
                                 fVar45 * fVar46 + (fVar50 - fVar37) * fVar44);
            local_1e4 = (fVar46 * fVar44 - fVar44 * fVar47) * fVar49 +
                        fVar45 * fVar48 + (fVar50 - fVar37) * fVar32;
            local_1f8 = CONCAT44((fVar48 * fVar38 - fVar42 * fVar46) * fVar49 +
                                 fVar40 * fVar47 + fVar43 * (fVar50 - fVar37) +
                                 (float)((ulonglong)uVar5 >> 0x20),
                                 (fVar47 * fVar42 - fVar43 * fVar48) * fVar49 +
                                 fVar40 * fVar46 + fVar38 * (fVar50 - fVar37) + (float)uVar5);
            local_f1 = 1;
            puVar17 = &local_1f8;
            country_core::systems::collision::world::Raycasting::raycast();
            lVar14 = lVar16;
            if (((local_154 != 2) && ((char)local_188 == '\x04')) &&
               (plStack_180 == (longlong *)0x2)) {
              iVar10 = (int)local_188._4_4_;
              local_13c = (int)local_188._4_4_;
              local_f1 = 1;
              country_core::resources::walls::decorator_storage::DecoratorStorage::
              lookup_decorator_info();
              if (extraout_RAX_00 != 0) {
                local_f1 = 1;
                country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair();
                bVar29 = *(byte *)(extraout_RAX_00 + 0x46);
                puVar17 = (undefined8 *)(ulonglong)bVar29;
                if (((7 < (byte)(bVar29 - 0x38) || (byte)(bVar29 - 0x38) == 6) && (bVar29 < 0x37))
                   && ((0x40010001800000U >> ((ulonglong)puVar17 & 0x3f) & 1) != 0)) {
                  uVar7 = *extraout_RAX_01;
                  local_f1 = 1;
                  local_120 = puVar17;
                  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                  if (extraout_RAX_02[3] != 0) {
                    core::hash::BuildHasher::
                    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                              ();
                    uVar13 = extraout_RAX_02[1];
                    bVar29 = (byte)((ulonglong)extraout_RAX_03 >> 0x39);
                    auVar33 = ZEXT216(CONCAT11(bVar29,bVar29));
                    auVar33 = pshuflw(auVar33,auVar33,0);
                    lVar16 = 0;
                    puVar20 = extraout_RAX_03;
                    do {
                      puVar17 = (undefined8 *)(uVar13 & (ulonglong)puVar20);
                      pcVar3 = (char *)(*extraout_RAX_02 + (longlong)puVar17);
                      cVar30 = auVar33[0];
                      auVar41[0] = -(*pcVar3 == cVar30);
                      cVar34 = auVar33[1];
                      auVar41[1] = -(pcVar3[1] == cVar34);
                      cVar35 = auVar33[2];
                      auVar41[2] = -(pcVar3[2] == cVar35);
                      cVar36 = auVar33[3];
                      auVar41[3] = -(pcVar3[3] == cVar36);
                      auVar41[4] = -(pcVar3[4] == cVar30);
                      auVar41[5] = -(pcVar3[5] == cVar34);
                      auVar41[6] = -(pcVar3[6] == cVar35);
                      auVar41[7] = -(pcVar3[7] == cVar36);
                      auVar41[8] = -(pcVar3[8] == cVar30);
                      auVar41[9] = -(pcVar3[9] == cVar34);
                      auVar41[10] = -(pcVar3[10] == cVar35);
                      auVar41[0xb] = -(pcVar3[0xb] == cVar36);
                      auVar41[0xc] = -(pcVar3[0xc] == cVar30);
                      auVar41[0xd] = -(pcVar3[0xd] == cVar34);
                      auVar41[0xe] = -(pcVar3[0xe] == cVar35);
                      auVar41[0xf] = -(pcVar3[0xf] == cVar36);
                      uVar25 = (ushort)(SUB161(auVar41 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe |
                               (ushort)(auVar41[0xf] >> 7) << 0xf;
                      uVar24 = (uint)uVar25;
                      while (uVar25 != 0) {
                        uVar8 = 0;
                        for (uVar9 = uVar24; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                          uVar8 = uVar8 + 1;
                        }
                        if (iVar10 == *(int *)(*extraout_RAX_02 + -0x28 +
                                              ((ulonglong)((ulonglong)uVar8 + (longlong)puVar17) &
                                              uVar13) * -0x28)) {
                          local_f1 = 1;
                          system_decorator::decorator_visual_state::PersistentVisualEntities::
                          entities();
                          if (((uVar13 == 0 || extraout_RAX_04 == (ulonglong *)0x0) ||
                              (uVar13 = *extraout_RAX_04, uVar13 >> 0x20 == 0)) ||
                             ((char)local_120 == '8')) goto LAB_1403be7d3;
                          if (puStack_100 == (undefined8 *)0x0) {
                            lVar16 = 0;
                            puVar17 = (undefined8 *)0x0;
LAB_1403beb5e:
                            lVar15 = 0;
                          }
                          else {
                            puVar17 = (undefined8 *)0x0;
                            if (local_118 <= local_108) {
                              puVar17 = local_118;
                            }
                            lVar16 = (longlong)local_108 - (longlong)puVar17;
                            lVar15 = (longlong)puStack_100 - ((longlong)local_118 - lVar16);
                            puVar17 = local_118;
                            if (puStack_100 < (undefined8 *)((longlong)local_118 - lVar16) ||
                                lVar15 == 0) {
                              puVar17 = (undefined8 *)((longlong)puStack_100 + lVar16);
                              goto LAB_1403beb5e;
                            }
                          }
                          lVar19 = (longlong)puVar17 * 0x1b0 + lVar16 * -0x1b0;
                          puVar23 = (undefined8 *)(puStack_110 + lVar16 * 0x1b0);
                          goto LAB_1403beb80;
                        }
                        uVar25 = (ushort)(uVar24 - 1) & (ushort)uVar24;
                        uVar24 = CONCAT22((short)(uVar24 - 1 >> 0x10),uVar25);
                      }
                      auVar39[0] = -(*pcVar3 == -1);
                      auVar39[1] = -(pcVar3[1] == -1);
                      auVar39[2] = -(pcVar3[2] == -1);
                      auVar39[3] = -(pcVar3[3] == -1);
                      auVar39[4] = -(pcVar3[4] == -1);
                      auVar39[5] = -(pcVar3[5] == -1);
                      auVar39[6] = -(pcVar3[6] == -1);
                      auVar39[7] = -(pcVar3[7] == -1);
                      auVar39[8] = -(pcVar3[8] == -1);
                      auVar39[9] = -(pcVar3[9] == -1);
                      auVar39[10] = -(pcVar3[10] == -1);
                      auVar39[0xb] = -(pcVar3[0xb] == -1);
                      auVar39[0xc] = -(pcVar3[0xc] == -1);
                      auVar39[0xd] = -(pcVar3[0xd] == -1);
                      auVar39[0xe] = -(pcVar3[0xe] == -1);
                      auVar39[0xf] = -(pcVar3[0xf] == -1);
                      if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 ||
                                       (SUB161(auVar39 >> 0xf,0) & 1) != 0) ||
                                      (SUB161(auVar39 >> 0x17,0) & 1) != 0) ||
                                     (SUB161(auVar39 >> 0x1f,0) & 1) != 0) ||
                                    (SUB161(auVar39 >> 0x27,0) & 1) != 0) ||
                                   (SUB161(auVar39 >> 0x2f,0) & 1) != 0) ||
                                  (SUB161(auVar39 >> 0x37,0) & 1) != 0) ||
                                 (SUB161(auVar39 >> 0x3f,0) & 1) != 0) ||
                                (SUB161(auVar39 >> 0x47,0) & 1) != 0) ||
                               (SUB161(auVar39 >> 0x4f,0) & 1) != 0) ||
                              (SUB161(auVar39 >> 0x57,0) & 1) != 0) ||
                             (SUB161(auVar39 >> 0x5f,0) & 1) != 0) ||
                            (SUB161(auVar39 >> 0x67,0) & 1) != 0) ||
                           (SUB161(auVar39 >> 0x6f,0) & 1) != 0) ||
                          (SUB161(auVar39 >> 0x77,0) & 1) != 0) || auVar39[0xf] < '\0') break;
                      puVar20 = (undefined1 *)((longlong)puVar17 + lVar16 + 0x10);
                      lVar16 = lVar16 + 0x10;
                    } while( true );
                  }
                }
              }
            }
            goto LAB_1403be7d3;
          }
          goto LAB_1403be305;
        }
      }
      else {
LAB_1403bee8d:
        uVar24 = (uint)puVar17;
        uVar21 = 1;
      }
      local_188 = (undefined8 *)CONCAT44(uVar24,uVar21);
      local_f1 = 1;
      plStack_180 = (longlong *)uVar13;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  local_178 = local_108;
  puStack_170 = puStack_100;
  local_188 = local_118;
  plStack_180 = (longlong *)puStack_110;
  puVar28 = (undefined8 *)((longlong)local_118 - (longlong)puStack_100);
  puVar17 = local_108;
  if (local_108 <= puVar28) goto joined_r0x0001403bee43;
  puVar27 = (undefined8 *)((longlong)local_118 - (longlong)local_108);
  puVar17 = (undefined8 *)((longlong)puStack_100 - (longlong)puVar27);
  if (puVar28 < puVar27) {
    if (puVar17 <= puVar28) {
      memmove(puStack_110 + (longlong)puVar17 * 0x1b0,puStack_110 + (longlong)local_108 * 0x1b0,
              (longlong)puVar27 * 0x1b0);
      memcpy(puVar20 + (longlong)puVar12 * 0x1b0,puVar20,(longlong)puVar17 * 0x1b0);
      goto joined_r0x0001403bee43;
    }
    if (puVar17 < puVar27) {
      if (local_118 != puStack_100) {
        memmove(puStack_110 + (longlong)puVar28 * 0x1b0,puStack_110,(longlong)puVar17 * 0x1b0);
      }
      core::slice::impl$0::rotate_left<tiny_glade::rollercoaster::CoasterCart>();
      puVar17 = puVar28;
      goto joined_r0x0001403bee43;
    }
    if (local_118 != puStack_100) {
      memmove(puStack_110 + (longlong)puVar17 * 0x1b0,puStack_110 + (longlong)local_108 * 0x1b0,
              (longlong)puVar27 * 0x1b0);
    }
    core::slice::impl$0::rotate_right<tiny_glade::rollercoaster::CoasterCart>();
  }
  else {
    memmove(puStack_110 + (longlong)puVar27 * 0x1b0,puStack_110,(longlong)puVar17 * 0x1b0);
    memcpy(puVar20,puVar20 + (longlong)puVar18 * 0x1b0,(longlong)puVar27 * 0x1b0);
  }
  puVar17 = (undefined8 *)0x0;
joined_r0x0001403bee43:
  if (puVar17 != (undefined8 *)0x0) {
    memmove(puVar20,puVar20 + (longlong)puVar17 * 0x1b0,(longlong)puVar12 * 0x1b0);
  }
  *local_1b8 = puVar23;
  local_1b8[1] = puVar20;
  local_1b8[2] = puVar12;
  if (local_138 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
  while( true ) {
    puVar17 = puVar23 + 0x36;
    lVar19 = lVar19 + -0x1b0;
    piVar1 = (int *)(puVar23 + 0x2f);
    puVar23 = puVar17;
    if (iVar10 == *piVar1) break;
LAB_1403beb80:
    if (lVar19 == 0) {
      lVar15 = lVar15 * 0x1b0;
      puVar20 = puStack_110;
      goto LAB_1403bebb0;
    }
  }
  goto LAB_1403be7d3;
  while( true ) {
    lVar15 = lVar15 + -0x1b0;
    piVar1 = (int *)(puVar20 + 0x178);
    puVar20 = puVar20 + 0x1b0;
    if (iVar10 == *piVar1) break;
LAB_1403bebb0:
    if (lVar15 == 0) {
      if (local_128 == local_138) {
        local_f1 = 1;
        local_1b0 = local_128;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
        local_128 = local_1b0;
      }
      *(int *)(local_130 + (longlong)local_128 * 4) = (int)uVar13;
      *(int *)((longlong)local_130 + (longlong)local_128 * 0x20 + 4) = (int)(uVar13 >> 0x20);
      local_130[(longlong)local_128 * 4 + 1] = uVar7;
      *(int *)(local_130 + (longlong)local_128 * 4 + 2) = iVar10;
      *(char *)((longlong)local_130 + (longlong)local_128 * 0x20 + 0x14) = (char)local_120;
      *(char *)(local_130 + (longlong)local_128 * 4 + 3) = cVar6;
      puVar17 = (undefined8 *)((longlong)local_128 + 1);
      local_128 = puVar17;
      break;
    }
  }
  goto LAB_1403be7d3;
}

