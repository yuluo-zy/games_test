
void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_0>
          (void)

{
  float *pfVar1;
  float *pfVar2;
  float *pfVar3;
  float *pfVar4;
  longlong *plVar5;
  ulonglong *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong *plVar11;
  longlong *in_RDX;
  undefined8 *in_R8;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float local_160;
  undefined8 local_15c;
  float local_154;
  float fStack_150;
  float fStack_14c;
  float fStack_148;
  float local_144;
  float fStack_140;
  float fStack_13c;
  float fStack_138;
  float local_134;
  float local_130;
  float local_12c;
  float local_128;
  float local_124;
  float fStack_120;
  float fStack_11c;
  float fStack_118;
  float local_114;
  float fStack_110;
  float fStack_10c;
  float fStack_108;
  float local_104;
  longlong *local_100;
  longlong local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  pfVar1 = (float *)*in_R8;
  pfVar2 = (float *)in_R8[2];
  pfVar3 = (float *)in_R8[3];
  pfVar4 = (float *)in_R8[5];
  plVar5 = (longlong *)*in_RDX;
  ::utils::swizzlers::impl$0::x0y();
  fVar16 = *pfVar2;
  fVar13 = local_160 * fVar16 * 0.5;
  fVar14 = fVar16 * (float)local_15c * 0.5;
  fVar15 = fVar16 * local_15c._4_4_ * 0.5;
  ::utils::swizzlers::impl$0::x0y();
  fVar16 = *pfVar4;
  local_124 = local_160 * fVar16 * 0.5;
  fVar12 = fVar16 * (float)local_15c * 0.5;
  fStack_11c = fVar16 * local_15c._4_4_ * 0.5;
  local_160 = *pfVar1 - fVar13;
  fVar17 = fVar14 + pfVar1[1] + -0.375;
  fVar14 = (pfVar1[1] + -0.375) - fVar14;
  local_15c._4_4_ = pfVar1[2] - fVar15;
  local_130 = *pfVar3 - local_124;
  fVar16 = fVar12 + pfVar3[1] + -0.375;
  fVar12 = (pfVar3[1] + -0.375) - fVar12;
  local_128 = pfVar3[2] - fStack_11c;
  local_15c._0_4_ = fVar14 + -0.5;
  local_154 = *pfVar1 + fVar13;
  fStack_14c = pfVar1[2] + fVar15;
  fStack_150 = fVar17 + -0.5;
  fStack_148 = local_160 + 0.0;
  local_144 = fVar14 + 0.5;
  fStack_140 = local_15c._4_4_ + 0.0;
  fStack_13c = local_154 + 0.0;
  fStack_138 = fVar17 + 0.5;
  local_134 = fStack_14c + 0.0;
  local_12c = fVar12 + -0.5;
  local_124 = *pfVar3 + local_124;
  fStack_11c = pfVar3[2] + fStack_11c;
  fStack_120 = fVar16 + -0.5;
  fStack_118 = local_130 + 0.0;
  local_114 = fVar12 + 0.5;
  fStack_110 = local_128 + 0.0;
  fStack_10c = local_124 + 0.0;
  fStack_108 = fVar16 + 0.5;
  local_104 = fStack_11c + 0.0;
  local_100 = (longlong *)*plVar5;
  puVar6 = (ulonglong *)plVar5[1];
  lVar7 = local_100[2];
  if ((ulonglong)(*local_100 - lVar7) < 0x18) {
    ::alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar7 = local_100[2];
    uVar8 = *puVar6;
  }
  else {
    uVar8 = *puVar6;
  }
  local_f8 = lVar7;
  if (uVar8 < 8) {
    lVar9 = local_100[1];
    uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
    *(undefined4 *)(lVar9 + 8 + lVar7 * 0xc) =
         *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
    *(undefined8 *)(lVar9 + lVar7 * 0xc) = uVar10;
    local_f8 = lVar7 + 1;
    uVar8 = puVar6[1];
    if (uVar8 < 8) {
      uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
      *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
           *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
      *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
      local_f8 = lVar7 + 2;
      uVar8 = puVar6[2];
      if (uVar8 < 8) {
        uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
        *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
             *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
        *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
        local_f8 = lVar7 + 3;
        uVar8 = puVar6[3];
        if (uVar8 < 8) {
          uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
          *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
               *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
          *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
          local_f8 = lVar7 + 4;
          uVar8 = puVar6[4];
          if (uVar8 < 8) {
            uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
            *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                 *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
            *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
            local_f8 = lVar7 + 5;
            uVar8 = puVar6[5];
            if (uVar8 < 8) {
              uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
              *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                   *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
              *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
              local_f8 = lVar7 + 6;
              uVar8 = puVar6[6];
              if (uVar8 < 8) {
                uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                     *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                local_f8 = lVar7 + 7;
                uVar8 = puVar6[7];
                if (uVar8 < 8) {
                  uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                  *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                       *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                  *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                  local_f8 = lVar7 + 8;
                  uVar8 = puVar6[8];
                  if (uVar8 < 8) {
                    uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                    *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                         *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                    *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                    local_f8 = lVar7 + 9;
                    uVar8 = puVar6[9];
                    if (uVar8 < 8) {
                      uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                      *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                           *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                      *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                      local_f8 = lVar7 + 10;
                      uVar8 = puVar6[10];
                      if (uVar8 < 8) {
                        uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                        *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                             *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                        *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                        local_f8 = lVar7 + 0xb;
                        uVar8 = puVar6[0xb];
                        if (uVar8 < 8) {
                          uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                          *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                               *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                          *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                          local_f8 = lVar7 + 0xc;
                          uVar8 = puVar6[0xc];
                          if (uVar8 < 8) {
                            uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                            *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                 *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                            *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                            local_f8 = lVar7 + 0xd;
                            uVar8 = puVar6[0xd];
                            if (uVar8 < 8) {
                              uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                              *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                   *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                              *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                              local_f8 = lVar7 + 0xe;
                              uVar8 = puVar6[0xe];
                              if (uVar8 < 8) {
                                uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                     *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                                *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                local_f8 = lVar7 + 0xf;
                                uVar8 = puVar6[0xf];
                                if (uVar8 < 8) {
                                  uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                  *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                       *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                                  *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                  local_f8 = lVar7 + 0x10;
                                  uVar8 = puVar6[0x10];
                                  if (uVar8 < 8) {
                                    uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                    *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                         *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                                    *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                    local_f8 = lVar7 + 0x11;
                                    uVar8 = puVar6[0x11];
                                    if (uVar8 < 8) {
                                      uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                      *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                           *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4);
                                      *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                      local_f8 = lVar7 + 0x12;
                                      uVar8 = puVar6[0x12];
                                      if (uVar8 < 8) {
                                        uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                        *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                             *(undefined4 *)((longlong)&local_15c + uVar8 * 0xc + 4)
                                        ;
                                        *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                        local_f8 = lVar7 + 0x13;
                                        uVar8 = puVar6[0x13];
                                        if (uVar8 < 8) {
                                          uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                          *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                               *(undefined4 *)
                                                ((longlong)&local_15c + uVar8 * 0xc + 4);
                                          *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                          local_f8 = lVar7 + 0x14;
                                          uVar8 = puVar6[0x14];
                                          if (uVar8 < 8) {
                                            uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                            *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                                 *(undefined4 *)
                                                  ((longlong)&local_15c + uVar8 * 0xc + 4);
                                            *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                            local_f8 = lVar7 + 0x15;
                                            uVar8 = puVar6[0x15];
                                            if (uVar8 < 8) {
                                              uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                              *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                                   *(undefined4 *)
                                                    ((longlong)&local_15c + uVar8 * 0xc + 4);
                                              *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                              local_f8 = lVar7 + 0x16;
                                              uVar8 = puVar6[0x16];
                                              if (uVar8 < 8) {
                                                uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                                *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                                     *(undefined4 *)
                                                      ((longlong)&local_15c + uVar8 * 0xc + 4);
                                                *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                                local_f8 = lVar7 + 0x17;
                                                uVar8 = puVar6[0x17];
                                                if (uVar8 < 8) {
                                                  uVar10 = *(undefined8 *)(&local_160 + uVar8 * 3);
                                                  *(undefined4 *)(lVar9 + 8 + local_f8 * 0xc) =
                                                       *(undefined4 *)
                                                        ((longlong)&local_15c + uVar8 * 0xc + 4);
                                                  *(undefined8 *)(lVar9 + local_f8 * 0xc) = uVar10;
                                                  local_100[2] = lVar7 + 0x18;
                                                  plVar11 = (longlong *)plVar5[2];
                                                  puVar6 = (ulonglong *)plVar5[3];
                                                  lVar7 = plVar11[2];
                                                  if ((ulonglong)(*plVar11 - lVar7) < 6) {
                                                    ::alloc::raw_vec::impl$4::reserve::
                                                    do_reserve_and_handle<alloc::alloc::Global>();
                                                    lVar7 = plVar11[2];
                                                    uVar8 = *puVar6;
                                                  }
                                                  else {
                                                    uVar8 = *puVar6;
                                                  }
                                                  local_f8 = lVar7;
                                                  if (uVar8 < 8) {
                                                    lVar9 = plVar11[1];
                                                    uVar10 = *(undefined8 *)
                                                              ((longlong)&local_15c + uVar8 * 0xc);
                                                    pfVar1 = (float *)(lVar9 + lVar7 * 0x10);
                                                    *pfVar1 = (&local_160)[uVar8 * 3];
                                                    pfVar1[1] = (float)uVar10;
                                                    pfVar1[2] = (float)((ulonglong)uVar10 >> 0x20);
                                                    pfVar1[3] = 1.0;
                                                    local_f8 = lVar7 + 1;
                                                    uVar8 = puVar6[1];
                                                    if (uVar8 < 8) {
                                                      uVar10 = *(undefined8 *)
                                                                ((longlong)&local_15c + uVar8 * 0xc)
                                                      ;
                                                      pfVar1 = (float *)(lVar9 + local_f8 * 0x10);
                                                      *pfVar1 = (&local_160)[uVar8 * 3];
                                                      pfVar1[1] = (float)uVar10;
                                                      pfVar1[2] = (float)((ulonglong)uVar10 >> 0x20)
                                                      ;
                                                      pfVar1[3] = 1.0;
                                                      local_f8 = lVar7 + 2;
                                                      uVar8 = puVar6[2];
                                                      if (uVar8 < 8) {
                                                        uVar10 = *(undefined8 *)
                                                                  ((longlong)&local_15c +
                                                                  uVar8 * 0xc);
                                                        pfVar1 = (float *)(lVar9 + local_f8 * 0x10);
                                                        *pfVar1 = (&local_160)[uVar8 * 3];
                                                        pfVar1[1] = (float)uVar10;
                                                        pfVar1[2] = (float)((ulonglong)uVar10 >>
                                                                           0x20);
                                                        pfVar1[3] = 1.0;
                                                        local_f8 = lVar7 + 3;
                                                        uVar8 = puVar6[3];
                                                        if (uVar8 < 8) {
                                                          uVar10 = *(undefined8 *)
                                                                    ((longlong)&local_15c +
                                                                    uVar8 * 0xc);
                                                          pfVar1 = (float *)(lVar9 + local_f8 * 0x10
                                                                            );
                                                          *pfVar1 = (&local_160)[uVar8 * 3];
                                                          pfVar1[1] = (float)uVar10;
                                                          pfVar1[2] = (float)((ulonglong)uVar10 >>
                                                                             0x20);
                                                          pfVar1[3] = 1.0;
                                                          local_f8 = lVar7 + 4;
                                                          uVar8 = puVar6[4];
                                                          if (uVar8 < 8) {
                                                            uVar10 = *(undefined8 *)
                                                                      ((longlong)&local_15c +
                                                                      uVar8 * 0xc);
                                                            pfVar1 = (float *)(lVar9 + local_f8 *
                                                                                       0x10);
                                                            *pfVar1 = (&local_160)[uVar8 * 3];
                                                            pfVar1[1] = (float)uVar10;
                                                            pfVar1[2] = (float)((ulonglong)uVar10 >>
                                                                               0x20);
                                                            pfVar1[3] = 1.0;
                                                            local_f8 = lVar7 + 5;
                                                            uVar8 = puVar6[5];
                                                            if (uVar8 < 8) {
                                                              uVar10 = *(undefined8 *)
                                                                        ((longlong)&local_15c +
                                                                        uVar8 * 0xc);
                                                              pfVar1 = (float *)(lVar9 + local_f8 *
                                                                                         0x10);
                                                              *pfVar1 = (&local_160)[uVar8 * 3];
                                                              pfVar1[1] = (float)uVar10;
                                                              pfVar1[2] = (float)((ulonglong)uVar10
                                                                                 >> 0x20);
                                                              pfVar1[3] = 1.0;
                                                              plVar11[2] = lVar7 + 6;
                                                              country_core::systems::collision::
                                                              world::RaycastMeshData::new_convex();
                                                              return;
                                                            }
                                                          }
                                                        }
                                                      }
                                                    }
                                                  }
                                                  local_100 = plVar11;
                    // WARNING: Subroutine does not return
                                                  panicking::panic_bounds_check();
                                                }
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
                    // WARNING: Subroutine does not return
  panicking::panic_bounds_check();
}

