// 0x1415fc230 bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass> size=2693

void __cdecl
country_core::render::pass_builder::bind_resources::impl$0::
bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>(void)

{
  char *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  int iVar6;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *puVar7;
  undefined8 *extraout_RAX_01;
  undefined8 *puVar8;
  void *in_RCX;
  void *in_RDX;
  ulonglong uVar9;
  longlong in_R8;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  uint uVar14;
  ushort uVar15;
  ulonglong uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined8 local_380 [3];
  longlong local_368;
  longlong local_360;
  longlong local_358;
  longlong local_270;
  ulonglong local_268;
  char local_24a;
  longlong local_248;
  undefined *local_1f0;
  undefined8 local_1e8;
  longlong local_1d8;
  longlong local_1d0;
  longlong local_1c8;
  longlong local_e0;
  ulonglong local_d8;
  char local_ba;
  longlong local_b8;
  longlong local_60;
  void *local_58;
  int local_50;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_49 = 1;
  uVar9 = 0x8226cc786e679355;
  local_58 = in_RDX;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_R8 + 0x218) <= uVar9)) ||
      (uVar9 = *(ulonglong *)(*(longlong *)(in_R8 + 0x210) + uVar9 * 8), uVar9 == 0)) ||
     (*(longlong *)(*(longlong *)(in_R8 + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)) {
    local_49 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_49 = 1;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_1f0 = &anon_107183261458dea85466985370debbb6_52_llvm_3641509820080252961;
    local_1e8 = 0x2c;
    if ((extraout_RAX_00[3] != 0x2c) ||
       (iVar6 = memcmp(&anon_107183261458dea85466985370debbb6_52_llvm_3641509820080252961,
                       (void *)extraout_RAX_00[2],0x2c), iVar6 != 0)) {
      local_380[0] = 0;
      local_49 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_49 = 1;
    puVar7 = (undefined8 *)(**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    if (puVar7 != (undefined8 *)0x0) {
      local_49 = 1;
      frozen::FrozenMap::
      get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                ();
      if (extraout_RAX_01 != (undefined8 *)0x0) {
        local_1f0 = &anon_107183261458dea85466985370debbb6_93_llvm_3641509820080252961;
        local_1e8 = 0x26;
        if ((extraout_RAX_01[3] != 0x26) ||
           (iVar6 = memcmp(&anon_107183261458dea85466985370debbb6_93_llvm_3641509820080252961,
                           (void *)extraout_RAX_01[2],0x26), iVar6 != 0)) {
          local_380[0] = 0;
          local_49 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        local_49 = 1;
        puVar8 = (undefined8 *)(**(code **)(extraout_RAX_01[1] + 0x18))(*extraout_RAX_01);
        if (puVar8 != (undefined8 *)0x0) {
          memcpy(local_380,local_58,400);
          pass_resource_access::read_access_buffer<country_core::resources::rt::RtTrianglesSsbo>();
          if (*(longlong *)(local_248 + 0xb0) != 0) {
            lVar3 = *(longlong *)(local_248 + 0x98);
            uVar9 = *(ulonglong *)(local_248 + 0xa0);
            uVar10 = uVar9 & 0x168ddb2c3b5f38a9;
            lVar13 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar3 + uVar10);
              auVar23[0] = -(*pcVar1 == '\v');
              auVar23[1] = -(pcVar1[1] == '\v');
              auVar23[2] = -(pcVar1[2] == '\v');
              auVar23[3] = -(pcVar1[3] == '\v');
              auVar23[4] = -(pcVar1[4] == '\v');
              auVar23[5] = -(pcVar1[5] == '\v');
              auVar23[6] = -(pcVar1[6] == '\v');
              auVar23[7] = -(pcVar1[7] == '\v');
              auVar23[8] = -(pcVar1[8] == '\v');
              auVar23[9] = -(pcVar1[9] == '\v');
              auVar23[10] = -(pcVar1[10] == '\v');
              auVar23[0xb] = -(pcVar1[0xb] == '\v');
              auVar23[0xc] = -(pcVar1[0xc] == '\v');
              auVar23[0xd] = -(pcVar1[0xd] == '\v');
              auVar23[0xe] = -(pcVar1[0xe] == '\v');
              auVar23[0xf] = -(pcVar1[0xf] == '\v');
              uVar15 = (ushort)(SUB161(auVar23 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(auVar23[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                if (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x10) == 0x168ddb2c3b5f38a9) {
                  lVar13 = uVar16 * -0x10;
                  local_50 = *(int *)(lVar3 + -8 + lVar13);
                  uVar2 = *(undefined4 *)(lVar3 + -4 + lVar13);
                  if (local_50 != 0) {
                    local_1f0 = (undefined *)0x0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (local_358 == local_368) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_360 + local_358 * 8) = uVar2;
                  *(undefined1 *)(local_360 + 4 + local_358 * 8) = extraout_AL;
                  local_358 = local_358 + 1;
                  goto LAB_1415fc51b;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar17[0] = -(*pcVar1 == -1);
              auVar17[1] = -(pcVar1[1] == -1);
              auVar17[2] = -(pcVar1[2] == -1);
              auVar17[3] = -(pcVar1[3] == -1);
              auVar17[4] = -(pcVar1[4] == -1);
              auVar17[5] = -(pcVar1[5] == -1);
              auVar17[6] = -(pcVar1[6] == -1);
              auVar17[7] = -(pcVar1[7] == -1);
              auVar17[8] = -(pcVar1[8] == -1);
              auVar17[9] = -(pcVar1[9] == -1);
              auVar17[10] = -(pcVar1[10] == -1);
              auVar17[0xb] = -(pcVar1[0xb] == -1);
              auVar17[0xc] = -(pcVar1[0xc] == -1);
              auVar17[0xd] = -(pcVar1[0xd] == -1);
              auVar17[0xe] = -(pcVar1[0xe] == -1);
              auVar17[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar17 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar17 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar17 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar17 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar17 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar17 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar17 >> 0x67,0) & 1) != 0) || (SUB161(auVar17 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar17 >> 0x77,0) & 1) != 0) || auVar17[0xf] < '\0') break;
              lVar11 = uVar10 + lVar13;
              lVar13 = lVar13 + 0x10;
              uVar10 = lVar11 + 0x10U & uVar9;
            }
          }
LAB_1415fc51b:
          memcpy(&local_1f0,local_380,400);
          pass_resource_access::read_access_buffer<country_core::resources::rt::RtTriangleMetaSsbo>
                    ();
          if (*(longlong *)(local_b8 + 0xb0) != 0) {
            lVar3 = *(longlong *)(local_b8 + 0x98);
            uVar9 = *(ulonglong *)(local_b8 + 0xa0);
            uVar10 = uVar9 & 0x47ddebfb264383d0;
            lVar13 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar3 + uVar10);
              auVar24[0] = -(*pcVar1 == '#');
              auVar24[1] = -(pcVar1[1] == '#');
              auVar24[2] = -(pcVar1[2] == '#');
              auVar24[3] = -(pcVar1[3] == '#');
              auVar24[4] = -(pcVar1[4] == '#');
              auVar24[5] = -(pcVar1[5] == '#');
              auVar24[6] = -(pcVar1[6] == '#');
              auVar24[7] = -(pcVar1[7] == '#');
              auVar24[8] = -(pcVar1[8] == '#');
              auVar24[9] = -(pcVar1[9] == '#');
              auVar24[10] = -(pcVar1[10] == '#');
              auVar24[0xb] = -(pcVar1[0xb] == '#');
              auVar24[0xc] = -(pcVar1[0xc] == '#');
              auVar24[0xd] = -(pcVar1[0xd] == '#');
              auVar24[0xe] = -(pcVar1[0xe] == '#');
              auVar24[0xf] = -(pcVar1[0xf] == '#');
              uVar15 = (ushort)(SUB161(auVar24 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(auVar24[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                if (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x10) == 0x47ddebfb264383d0) {
                  lVar13 = uVar16 * -0x10;
                  local_50 = *(int *)(lVar3 + -8 + lVar13);
                  uVar2 = *(undefined4 *)(lVar3 + -4 + lVar13);
                  if (local_50 != 0) {
                    local_380[0] = 0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (local_1c8 == local_1d8) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_1d0 + local_1c8 * 8) = uVar2;
                  *(undefined1 *)(local_1d0 + 4 + local_1c8 * 8) = extraout_AL_00;
                  local_1c8 = local_1c8 + 1;
                  goto LAB_1415fc66a;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar18[0] = -(*pcVar1 == -1);
              auVar18[1] = -(pcVar1[1] == -1);
              auVar18[2] = -(pcVar1[2] == -1);
              auVar18[3] = -(pcVar1[3] == -1);
              auVar18[4] = -(pcVar1[4] == -1);
              auVar18[5] = -(pcVar1[5] == -1);
              auVar18[6] = -(pcVar1[6] == -1);
              auVar18[7] = -(pcVar1[7] == -1);
              auVar18[8] = -(pcVar1[8] == -1);
              auVar18[9] = -(pcVar1[9] == -1);
              auVar18[10] = -(pcVar1[10] == -1);
              auVar18[0xb] = -(pcVar1[0xb] == -1);
              auVar18[0xc] = -(pcVar1[0xc] == -1);
              auVar18[0xd] = -(pcVar1[0xd] == -1);
              auVar18[0xe] = -(pcVar1[0xe] == -1);
              auVar18[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar18 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar18 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar18 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar18 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar18 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar18 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar18 >> 0x67,0) & 1) != 0) || (SUB161(auVar18 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar18 >> 0x77,0) & 1) != 0) || auVar18[0xf] < '\0') break;
              lVar11 = uVar10 + lVar13;
              lVar13 = lVar13 + 0x10;
              uVar10 = lVar11 + 0x10U & uVar9;
            }
          }
LAB_1415fc66a:
          memcpy(local_380,&local_1f0,400);
          pass_resource_access::read_access_buffer<country_core::resources::rt::RtBvhSsbo>();
          lVar3 = local_358;
          if (*(longlong *)(local_248 + 0xb0) != 0) {
            lVar13 = *(longlong *)(local_248 + 0x98);
            uVar9 = *(ulonglong *)(local_248 + 0xa0);
            uVar10 = uVar9 & 0x339e76846042a271;
            lVar11 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar13 + uVar10);
              auVar25[0] = -(*pcVar1 == '\x19');
              auVar25[1] = -(pcVar1[1] == '\x19');
              auVar25[2] = -(pcVar1[2] == '\x19');
              auVar25[3] = -(pcVar1[3] == '\x19');
              auVar25[4] = -(pcVar1[4] == '\x19');
              auVar25[5] = -(pcVar1[5] == '\x19');
              auVar25[6] = -(pcVar1[6] == '\x19');
              auVar25[7] = -(pcVar1[7] == '\x19');
              auVar25[8] = -(pcVar1[8] == '\x19');
              auVar25[9] = -(pcVar1[9] == '\x19');
              auVar25[10] = -(pcVar1[10] == '\x19');
              auVar25[0xb] = -(pcVar1[0xb] == '\x19');
              auVar25[0xc] = -(pcVar1[0xc] == '\x19');
              auVar25[0xd] = -(pcVar1[0xd] == '\x19');
              auVar25[0xe] = -(pcVar1[0xe] == '\x19');
              auVar25[0xf] = -(pcVar1[0xf] == '\x19');
              uVar15 = (ushort)(SUB161(auVar25 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(auVar25[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                if (*(longlong *)(lVar13 + -0x10 + uVar16 * -0x10) == 0x339e76846042a271) {
                  lVar11 = uVar16 * -0x10;
                  local_50 = *(int *)(lVar13 + -8 + lVar11);
                  uVar2 = *(undefined4 *)(lVar13 + -4 + lVar11);
                  if (local_50 != 0) {
                    local_1f0 = (undefined *)0x0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (local_358 == local_368) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_360 + lVar3 * 8) = uVar2;
                  *(undefined1 *)(local_360 + 4 + lVar3 * 8) = extraout_AL_01;
                  local_358 = lVar3 + 1;
                  goto LAB_1415fc79b;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar19[0] = -(*pcVar1 == -1);
              auVar19[1] = -(pcVar1[1] == -1);
              auVar19[2] = -(pcVar1[2] == -1);
              auVar19[3] = -(pcVar1[3] == -1);
              auVar19[4] = -(pcVar1[4] == -1);
              auVar19[5] = -(pcVar1[5] == -1);
              auVar19[6] = -(pcVar1[6] == -1);
              auVar19[7] = -(pcVar1[7] == -1);
              auVar19[8] = -(pcVar1[8] == -1);
              auVar19[9] = -(pcVar1[9] == -1);
              auVar19[10] = -(pcVar1[10] == -1);
              auVar19[0xb] = -(pcVar1[0xb] == -1);
              auVar19[0xc] = -(pcVar1[0xc] == -1);
              auVar19[0xd] = -(pcVar1[0xd] == -1);
              auVar19[0xe] = -(pcVar1[0xe] == -1);
              auVar19[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar19 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar19 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar19 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar19 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar19 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar19 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar19 >> 0x67,0) & 1) != 0) || (SUB161(auVar19 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar19 >> 0x77,0) & 1) != 0) || auVar19[0xf] < '\0') break;
              lVar12 = uVar10 + lVar11;
              lVar11 = lVar11 + 0x10;
              uVar10 = lVar12 + 0x10U & uVar9;
            }
          }
LAB_1415fc79b:
          memcpy(&local_1f0,local_380,400);
          pass_resource_access::
          read_access_buffer<country_core::resources::color_id::WallColorIdSsbo>();
          lVar3 = local_1c8;
          if (*(longlong *)(local_b8 + 0xb0) != 0) {
            lVar13 = *(longlong *)(local_b8 + 0x98);
            uVar9 = *(ulonglong *)(local_b8 + 0xa0);
            uVar10 = uVar9 & 0xe9701a257aa8828c;
            lVar11 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar13 + uVar10);
              auVar26[0] = -(*pcVar1 == 't');
              auVar26[1] = -(pcVar1[1] == 't');
              auVar26[2] = -(pcVar1[2] == 't');
              auVar26[3] = -(pcVar1[3] == 't');
              auVar26[4] = -(pcVar1[4] == 't');
              auVar26[5] = -(pcVar1[5] == 't');
              auVar26[6] = -(pcVar1[6] == 't');
              auVar26[7] = -(pcVar1[7] == 't');
              auVar26[8] = -(pcVar1[8] == 't');
              auVar26[9] = -(pcVar1[9] == 't');
              auVar26[10] = -(pcVar1[10] == 't');
              auVar26[0xb] = -(pcVar1[0xb] == 't');
              auVar26[0xc] = -(pcVar1[0xc] == 't');
              auVar26[0xd] = -(pcVar1[0xd] == 't');
              auVar26[0xe] = -(pcVar1[0xe] == 't');
              auVar26[0xf] = -(pcVar1[0xf] == 't');
              uVar15 = (ushort)(SUB161(auVar26 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(auVar26[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                if (*(longlong *)(lVar13 + -0x10 + uVar16 * -0x10) == -0x168fe5da85577d74) {
                  lVar11 = uVar16 * -0x10;
                  local_50 = *(int *)(lVar13 + -8 + lVar11);
                  uVar2 = *(undefined4 *)(lVar13 + -4 + lVar11);
                  if (local_50 != 0) {
                    local_380[0] = 0;
                    // WARNING: Subroutine does not return
                    core::panicking::
                    assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                              ();
                  }
                  if (local_1c8 == local_1d8) {
                    alloc::raw_vec::RawVec::
                    grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined4 *)(local_1d0 + lVar3 * 8) = uVar2;
                  *(undefined1 *)(local_1d0 + 4 + lVar3 * 8) = extraout_AL_02;
                  local_1c8 = lVar3 + 1;
                  goto LAB_1415fc8ea;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar20[0] = -(*pcVar1 == -1);
              auVar20[1] = -(pcVar1[1] == -1);
              auVar20[2] = -(pcVar1[2] == -1);
              auVar20[3] = -(pcVar1[3] == -1);
              auVar20[4] = -(pcVar1[4] == -1);
              auVar20[5] = -(pcVar1[5] == -1);
              auVar20[6] = -(pcVar1[6] == -1);
              auVar20[7] = -(pcVar1[7] == -1);
              auVar20[8] = -(pcVar1[8] == -1);
              auVar20[9] = -(pcVar1[9] == -1);
              auVar20[10] = -(pcVar1[10] == -1);
              auVar20[0xb] = -(pcVar1[0xb] == -1);
              auVar20[0xc] = -(pcVar1[0xc] == -1);
              auVar20[0xd] = -(pcVar1[0xd] == -1);
              auVar20[0xe] = -(pcVar1[0xe] == -1);
              auVar20[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar20 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar20 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar20 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') break;
              lVar12 = uVar10 + lVar11;
              lVar11 = lVar11 + 0x10;
              uVar10 = lVar12 + 0x10U & uVar9;
            }
          }
LAB_1415fc8ea:
          memcpy(local_380,&local_1f0,400);
          if (((local_24a != '\b') && (*(longlong *)(local_248 + 0xe8) != 0)) &&
             (*(longlong *)(local_248 + 0x130) != 0)) {
            lVar3 = *(longlong *)(local_248 + 0x118);
            uVar9 = *(ulonglong *)(local_248 + 0x120);
            uVar10 = uVar9 & 0x1c9f2eef0ac739ac;
            lVar13 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar3 + uVar10);
              auVar27[0] = -(*pcVar1 == '\x0e');
              auVar27[1] = -(pcVar1[1] == '\x0e');
              auVar27[2] = -(pcVar1[2] == '\x0e');
              auVar27[3] = -(pcVar1[3] == '\x0e');
              auVar27[4] = -(pcVar1[4] == '\x0e');
              auVar27[5] = -(pcVar1[5] == '\x0e');
              auVar27[6] = -(pcVar1[6] == '\x0e');
              auVar27[7] = -(pcVar1[7] == '\x0e');
              auVar27[8] = -(pcVar1[8] == '\x0e');
              auVar27[9] = -(pcVar1[9] == '\x0e');
              auVar27[10] = -(pcVar1[10] == '\x0e');
              auVar27[0xb] = -(pcVar1[0xb] == '\x0e');
              auVar27[0xc] = -(pcVar1[0xc] == '\x0e');
              auVar27[0xd] = -(pcVar1[0xd] == '\x0e');
              auVar27[0xe] = -(pcVar1[0xe] == '\x0e');
              auVar27[0xf] = -(pcVar1[0xf] == '\x0e');
              uVar15 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar27[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                local_60 = lVar13;
                if (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x10) == 0x1c9f2eef0ac739ac) {
                  uVar9 = (ulonglong)*(uint *)(lVar3 + -8 + uVar16 * -0x10);
                  if (local_268 < uVar9) {
                    // WARNING: Subroutine does not return
                    core::slice::index::slice_start_index_len_fail();
                  }
                  *(int *)(local_270 + uVar9) = (int)*puVar7;
                  goto LAB_1415fca17;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar21[0] = -(*pcVar1 == -1);
              auVar21[1] = -(pcVar1[1] == -1);
              auVar21[2] = -(pcVar1[2] == -1);
              auVar21[3] = -(pcVar1[3] == -1);
              auVar21[4] = -(pcVar1[4] == -1);
              auVar21[5] = -(pcVar1[5] == -1);
              auVar21[6] = -(pcVar1[6] == -1);
              auVar21[7] = -(pcVar1[7] == -1);
              auVar21[8] = -(pcVar1[8] == -1);
              auVar21[9] = -(pcVar1[9] == -1);
              auVar21[10] = -(pcVar1[10] == -1);
              auVar21[0xb] = -(pcVar1[0xb] == -1);
              auVar21[0xc] = -(pcVar1[0xc] == -1);
              auVar21[0xd] = -(pcVar1[0xd] == -1);
              auVar21[0xe] = -(pcVar1[0xe] == -1);
              auVar21[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar21 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar21 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar21 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
              lVar11 = uVar10 + lVar13;
              lVar13 = lVar13 + 0x10;
              uVar10 = lVar11 + 0x10U & uVar9;
            }
          }
LAB_1415fca17:
          memcpy(&local_1f0,local_380,400);
          if (((local_ba != '\b') && (*(longlong *)(local_b8 + 0xe8) != 0)) &&
             (*(longlong *)(local_b8 + 0x130) != 0)) {
            lVar3 = *(longlong *)(local_b8 + 0x118);
            uVar9 = *(ulonglong *)(local_b8 + 0x120);
            uVar10 = uVar9 & 0xc2b7f2fdf5e21521;
            lVar13 = 0;
            while( true ) {
              pcVar1 = (char *)(lVar3 + uVar10);
              auVar28[0] = -(*pcVar1 == 'a');
              auVar28[1] = -(pcVar1[1] == 'a');
              auVar28[2] = -(pcVar1[2] == 'a');
              auVar28[3] = -(pcVar1[3] == 'a');
              auVar28[4] = -(pcVar1[4] == 'a');
              auVar28[5] = -(pcVar1[5] == 'a');
              auVar28[6] = -(pcVar1[6] == 'a');
              auVar28[7] = -(pcVar1[7] == 'a');
              auVar28[8] = -(pcVar1[8] == 'a');
              auVar28[9] = -(pcVar1[9] == 'a');
              auVar28[10] = -(pcVar1[10] == 'a');
              auVar28[0xb] = -(pcVar1[0xb] == 'a');
              auVar28[0xc] = -(pcVar1[0xc] == 'a');
              auVar28[0xd] = -(pcVar1[0xd] == 'a');
              auVar28[0xe] = -(pcVar1[0xe] == 'a');
              auVar28[0xf] = -(pcVar1[0xf] == 'a');
              uVar15 = (ushort)(SUB161(auVar28 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar28[0xf] >> 7) << 0xf;
              uVar14 = (uint)uVar15;
              while (uVar15 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                uVar16 = uVar4 + uVar10 & uVar9;
                if (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x10) == -0x3d480d020a1deadf) {
                  uVar9 = (ulonglong)*(uint *)(lVar3 + -8 + uVar16 * -0x10);
                  if (local_d8 < uVar9) {
                    // WARNING: Subroutine does not return
                    core::slice::index::slice_start_index_len_fail();
                  }
                  *(int *)(local_e0 + uVar9) = (int)*puVar8;
                  goto LAB_1415fcb18;
                }
                uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
                uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
              }
              auVar22[0] = -(*pcVar1 == -1);
              auVar22[1] = -(pcVar1[1] == -1);
              auVar22[2] = -(pcVar1[2] == -1);
              auVar22[3] = -(pcVar1[3] == -1);
              auVar22[4] = -(pcVar1[4] == -1);
              auVar22[5] = -(pcVar1[5] == -1);
              auVar22[6] = -(pcVar1[6] == -1);
              auVar22[7] = -(pcVar1[7] == -1);
              auVar22[8] = -(pcVar1[8] == -1);
              auVar22[9] = -(pcVar1[9] == -1);
              auVar22[10] = -(pcVar1[10] == -1);
              auVar22[0xb] = -(pcVar1[0xb] == -1);
              auVar22[0xc] = -(pcVar1[0xc] == -1);
              auVar22[0xd] = -(pcVar1[0xd] == -1);
              auVar22[0xe] = -(pcVar1[0xe] == -1);
              auVar22[0xf] = -(pcVar1[0xf] == -1);
              if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar22 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar22 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar22 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar22 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar22 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar22 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar22 >> 0x67,0) & 1) != 0) || (SUB161(auVar22 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar22 >> 0x77,0) & 1) != 0) || auVar22[0xf] < '\0') break;
              lVar11 = uVar10 + lVar13;
              lVar13 = lVar13 + 0x10;
              uVar10 = lVar11 + 0x10U & uVar9;
            }
          }
LAB_1415fcb18:
          memcpy(in_RCX,&local_1f0,400);
          return;
        }
      }
      local_49 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  local_49 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

