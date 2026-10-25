// ADDR 0x142001880
// SIG void __cdecl find_best_wall_decorator_merge_position(void)
// ORIG system::find_best_wall_decorator_merge_position


void __cdecl
system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position(void)

{
  longlong lVar1;
  undefined8 uVar2;
  float fVar3;
  undefined8 *puVar4;
  undefined8 *in_RCX;
  undefined8 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  char cVar8;
  longlong in_stack_00000050;
  longlong in_stack_00000058;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined7 uStack_240;
  undefined1 local_239;
  undefined7 uStack_238;
  undefined8 uStack_231;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined7 uStack_1d0;
  undefined1 local_1c9;
  undefined7 uStack_1c8;
  undefined8 uStack_1c1;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined7 uStack_170;
  undefined1 local_169;
  undefined7 uStack_168;
  undefined8 uStack_161;
  char local_159;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 local_f0;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  char cStack_81;
  undefined8 local_80;
  
  if (in_stack_00000058 != 0) {
    lVar1 = in_stack_00000050 + in_stack_00000058 * 4;
    uVar7 = in_stack_00000058 * 4;
    do {
      uVar7 = uVar7 - 4;
      in_stack_00000050 = in_stack_00000050 + 4;
      propose_wall_decorator_merge_position_inner();
      if (local_159 != '\x02') {
        uStack_238 = uStack_168;
        uStack_231 = uStack_161;
        uStack_248 = uStack_178;
        uStack_240 = uStack_170;
        local_239 = local_169;
        local_258 = local_188;
        uStack_250 = uStack_180;
        local_268 = local_198;
        uStack_260 = uStack_190;
        local_278 = local_1a8;
        uStack_270 = uStack_1a0;
        local_288 = local_1b8;
        uStack_280 = uStack_1b0;
        uVar5 = local_158;
        if (in_stack_00000050 != lVar1) {
          uVar6 = 0;
          cVar8 = local_159;
          do {
            propose_wall_decorator_merge_position_inner();
            local_159 = cVar8;
            if (cStack_81 == '\x02') {
              uStack_1c8 = uStack_238;
              uStack_1c1 = uStack_231;
              uStack_1d8 = uStack_248;
              uStack_1d0 = uStack_240;
              local_1c9 = local_239;
              local_1e8 = local_258;
              uStack_1e0 = uStack_250;
              local_1f8 = local_268;
              uStack_1f0 = uStack_260;
              local_208 = local_278;
              uStack_200 = uStack_270;
              local_218 = local_288;
              uStack_210 = uStack_280;
            }
            else {
              local_f0 = local_80;
              uVar2 = local_f0;
              local_100 = local_90;
              local_110 = local_a0;
              uStack_108 = uStack_98;
              local_120 = local_b0;
              uStack_118 = uStack_a8;
              local_130 = local_c0;
              uStack_128 = uStack_b8;
              local_140 = local_d0;
              uStack_138 = uStack_c8;
              local_150 = local_e0;
              uStack_148 = uStack_d8;
              local_1b8 = local_288;
              uStack_1b0 = uStack_280;
              local_1a8 = local_278;
              uStack_1a0 = uStack_270;
              local_198 = local_268;
              uStack_190 = uStack_260;
              local_188 = local_258;
              uStack_180 = uStack_250;
              uStack_178 = uStack_248;
              uStack_170 = uStack_240;
              local_169 = local_239;
              uStack_168 = uStack_238;
              uStack_161 = uStack_231;
              fVar3 = (float)((ulonglong)uVar5 >> 0x20);
              local_f0._4_4_ = (float)((ulonglong)local_80 >> 0x20);
              if (NAN(fVar3) || NAN(local_f0._4_4_)) {
                local_f0 = uVar2;
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              puVar4 = &local_1b8;
              if (local_f0._4_4_ < fVar3) {
                puVar4 = &local_150;
              }
              uStack_1c1 = *(undefined8 *)((longlong)puVar4 + 0x57);
              uStack_1c8 = (undefined7)((ulonglong)*(undefined8 *)((longlong)puVar4 + 0x4f) >> 8);
              uStack_1d8 = puVar4[8];
              uStack_1d0 = (undefined7)puVar4[9];
              local_1c9 = (undefined1)((ulonglong)puVar4[9] >> 0x38);
              local_218 = *puVar4;
              uStack_210 = puVar4[1];
              local_208 = puVar4[2];
              uStack_200 = puVar4[3];
              local_1f8 = puVar4[4];
              uStack_1f0 = puVar4[5];
              local_1e8 = puVar4[6];
              uStack_1e0 = puVar4[7];
              if (local_f0._4_4_ < fVar3) {
                uVar5 = local_80;
                local_159 = cStack_81;
              }
            }
            local_288 = local_218;
            uStack_280 = uStack_210;
            local_278 = local_208;
            uStack_270 = uStack_200;
            local_268 = local_1f8;
            uStack_260 = uStack_1f0;
            local_258 = local_1e8;
            uStack_250 = uStack_1e0;
            uStack_248 = uStack_1d8;
            uStack_240 = uStack_1d0;
            local_239 = local_1c9;
            uStack_238 = uStack_1c8;
            uStack_231 = uStack_1c1;
            uVar6 = uVar6 + 1;
            cVar8 = local_159;
          } while (uVar7 >> 2 != uVar6);
        }
        *(ulonglong *)((longlong)in_RCX + 0x4f) = CONCAT71(uStack_238,local_239);
        *(undefined8 *)((longlong)in_RCX + 0x57) = uStack_231;
        in_RCX[8] = uStack_248;
        in_RCX[9] = CONCAT17(local_239,uStack_240);
        in_RCX[6] = local_258;
        in_RCX[7] = uStack_250;
        in_RCX[4] = local_268;
        in_RCX[5] = uStack_260;
        in_RCX[2] = local_278;
        in_RCX[3] = uStack_270;
        *in_RCX = local_288;
        in_RCX[1] = uStack_280;
        *(char *)((longlong)in_RCX + 0x5f) = local_159;
        in_RCX[0xc] = uVar5;
        return;
      }
    } while (in_stack_00000050 != lVar1);
  }
  *(undefined1 *)((longlong)in_RCX + 0x5f) = 2;
  return;
}


