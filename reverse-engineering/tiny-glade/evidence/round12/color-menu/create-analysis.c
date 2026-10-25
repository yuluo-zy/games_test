
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void color_create(void *out,float center_x,float center_y,float radius,float start_angle,
                 float icon_size,float spacing,float appearance_divisor,void *icons,void *target,
                 void *initial,void *opening_eye,void *commands)

{
  char cVar1;
  ulonglong *puVar2;
  byte bVar3;
  undefined4 *puVar4;
  byte bVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined *puVar10;
  code *pcVar11;
  longlong lVar12;
  ulonglong uVar13;
  longlong *plVar14;
  undefined8 *puVar15;
  undefined8 uVar16;
  ulonglong *puVar17;
  undefined4 uVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  ulonglong uStack_308;
  ulonglong uStack_300;
  ulonglong uStack_2f8;
  undefined4 uStack_2f0;
  float fStack_2e8;
  float fStack_2e4;
  undefined8 uStack_2e0;
  float fStack_2d8;
  float fStack_2d4;
  undefined8 uStack_2d0;
  float fStack_2c8;
  float fStack_2c4;
  float fStack_2c0;
  undefined4 uStack_2bc;
  undefined1 uStack_2b8;
  undefined1 uStack_2b7;
  undefined8 uStack_2b0;
  undefined8 auStack_268 [2];
  longlong lStack_258;
  longlong lStack_250;
  longlong lStack_248;
  undefined4 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  ulonglong uStack_228;
  ulonglong uStack_220;
  float fStack_218;
  float fStack_214;
  undefined4 uStack_210;
  float fStack_20c;
  ulonglong uStack_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  float fStack_1dc;
  float fStack_1d8;
  undefined8 uStack_1d4;
  undefined4 uStack_1cc;
  ulonglong *puStack_1c8;
  ulonglong *puStack_1c0;
  void *pvStack_1b8;
  longlong lStack_1b0;
  undefined8 uStack_1a8;
  longlong lStack_1a0;
  longlong lStack_198;
  undefined4 *puStack_190;
  ulonglong *puStack_188;
  ulonglong uStack_180;
  float fStack_178;
  float fStack_174;
  float fStack_170;
  float fStack_16c;
  float fStack_168;
  float fStack_164;
  float fStack_160;
  uint uStack_15c;
  undefined8 uStack_158;
  undefined8 uStack_150;
  ulonglong uStack_148;
  ulonglong uStack_140;
  uint uStack_134;
  undefined1 auStack_130 [8];
  longlong *plStack_128;
  longlong *plStack_120;
  undefined8 uStack_110;
  undefined2 uStack_102;
  uint uStack_100;
  byte bStack_fc;
  char cStack_fb;
  byte bStack_fa;
  byte bStack_f9;
  float fStack_f8;
  undefined1 uStack_f3;
  byte bStack_f2;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  uVar13 = *(ulonglong *)((longlong)icons + 0x10);
  if (uVar13 == 0) {
    uVar13 = 0;
  }
  else {
    lVar6 = uVar13 * 0x28;
    lVar12 = 0;
    do {
      uVar13 = uVar13 - (*(char *)(*(longlong *)((longlong)icons + 8) + 0x18 + lVar12) == '\x05');
      lVar12 = lVar12 + 0x28;
    } while (lVar6 != lVar12);
  }
  uVar7 = uVar13 * 8;
  uVar16 = 0;
  if ((uVar13 >> 0x3d == 0) && (uVar7 < 0x7ffffffffffffff9)) {
    if (uVar7 == 0) {
      lVar6 = 8;
      uVar9 = 0;
    }
    else {
      uVar16 = 8;
      lVar6 = func_0x000140613c10(uVar7,8);
      uVar9 = uVar13;
      if (lVar6 == 0) goto LAB_1425f3755;
    }
    uStack_148 = 0;
    fVar22 = ((icon_size + spacing) * ((float)(longlong)uVar13 + _DAT_14295a0e4)) / radius;
    fVar23 = _DAT_14295a0e8 * fVar22 + _DAT_142aa3260 + start_angle;
    uStack_158 = uVar9;
    uStack_150 = lVar6;
    uVar18 = FUN_1425f2460(fVar23);
    FUN_141c5a430(&uStack_308,icons,&UNK_143136e78);
    uVar13 = uStack_2f8;
    uVar7 = uStack_300;
    if (uStack_2f8 < uStack_308) {
      if (uStack_2f8 == 0) {
        func_0x000140613c20(uStack_300,uStack_308 * 0x28,8);
        uVar7 = 8;
      }
      else {
        lVar6 = uStack_2f8 * 0x28;
        uVar7 = func_0x000140613c30(uStack_300,uStack_308 * 0x28,8,lVar6);
        if (uVar7 == 0) {
          FUN_1428d8fe3(8,lVar6,&UNK_143136e90);
          goto override_jmp_1425f2d31_case_2;
        }
      }
    }
    fStack_1d8 = icon_size;
    lStack_258 = 0;
    lStack_250 = 8;
    lStack_248 = 0;
    uStack_238 = *(undefined8 *)target;
    uStack_230 = *(undefined8 *)((longlong)target + 8);
    uStack_200 = *(undefined4 *)initial;
    uStack_1fc = *(undefined4 *)((longlong)initial + 4);
    uStack_1f8 = *(undefined4 *)((longlong)initial + 8);
    uStack_1f4 = *(undefined4 *)((longlong)initial + 0xc);
    uStack_1f0 = *(undefined8 *)((longlong)initial + 0x10);
    uStack_1e8 = *(undefined8 *)((longlong)initial + 0x18);
    uStack_1e0 = *(undefined4 *)((longlong)initial + 0x20);
    uStack_240 = 0;
    uStack_1d4 = *(undefined8 *)opening_eye;
    uStack_1cc = *(undefined4 *)((longlong)opening_eye + 8);
    auStack_268[0] = 0;
    uStack_208 = 0;
    uStack_220 = uVar13;
    puVar17 = *(ulonglong **)((longlong)icons + 8);
    lStack_1b0 = *(longlong *)((longlong)icons + 0x10);
    *(undefined8 *)((longlong)icons + 0x10) = 0;
    puStack_1c0 = puVar17 + lStack_1b0 * 5;
    pvStack_1b8 = icons;
    uStack_1a8 = 0;
    uStack_228 = uVar7;
    fStack_218 = center_x;
    fStack_214 = center_y;
    uStack_210 = uVar18;
    fStack_20c = fVar22;
    fStack_1dc = radius;
    puStack_190 = out;
    puStack_188 = puStack_1c0;
    if (lStack_1b0 != 0) {
      fVar23 = fVar23 * radius;
      bStack_fc = *(byte *)((longlong)initial + 0x1c);
      fStack_170 = *(float *)((longlong)initial + 8);
      cStack_fb = *(char *)((longlong)initial + 0x10);
      fStack_160 = *(float *)((longlong)initial + 0x14);
      bStack_f9 = *(byte *)((longlong)initial + 0x11);
      uStack_15c = *(uint *)((longlong)initial + 0x18);
      fStack_164 = *(float *)((longlong)initial + 4);
      fStack_16c = *(float *)((longlong)initial + 0xc);
      bStack_fa = *(byte *)((longlong)initial + 0x1d);
      fStack_168 = *(float *)initial;
      fStack_178 = _DAT_1429edb90 * icon_size;
      fStack_174 = _DAT_142925950 * icon_size;
      fVar22 = _DAT_142925904 / appearance_divisor;
      fVar25 = (float)(_DAT_142ab3220 & -(uint)(0.0 < start_angle));
      lVar6 = lStack_1b0 * 0x28;
      lVar12 = 1;
      uStack_100 = 0;
      puVar2 = puVar17;
      do {
        puVar17 = puVar2 + 5;
        uStack_140 = *puVar2;
        puStack_1c8 = puVar17;
        if (SBORROW8(0,uStack_140)) break;
        uStack_180 = puVar2[1];
        uVar13 = puVar2[2];
        cVar1 = (char)puVar2[3];
        bStack_f2 = *(byte *)((longlong)puVar2 + 0x19);
        uStack_102 = *(undefined2 *)((longlong)puVar2 + 0x1a);
        fStack_f8 = *(float *)((longlong)puVar2 + 0x1c);
        uStack_134 = (uint)puVar2[4];
        uVar18 = *(undefined4 *)((longlong)puVar2 + 0x24);
        bVar5 = cVar1 - 3;
        if (0xb < bVar5) {
          bVar5 = 8;
        }
        lStack_1a0 = lVar12;
        lStack_198 = lVar6;
        if (bVar5 == 2) {
          fVar23 = fVar23 - fStack_174;
          fVar19 = (float)func_0x000142923e80(fVar23 / radius);
          fVar20 = (float)func_0x000142923fd0(fVar23 / radius);
          uStack_2b7 = 0;
          fVar24 = fStack_178;
          goto LAB_1425f2e60;
        }
        fVar19 = (float)func_0x000142923e80(fVar23 / radius);
        fVar20 = (float)func_0x000142923fd0(fVar23 / radius);
        puVar10 = &UNK_143136f20;
        fVar24 = icon_size;
                    /* WARNING: Switch is manually overridden */
        switch((ulonglong)bVar5) {
        case 0:
          fVar21 = fStack_168;
          goto joined_r0x0001425f2df3;
        case 1:
          bVar3 = bStack_fa;
          goto joined_r0x0001425f2daa;
        case 2:
          goto override_jmp_1425f2d31_case_2;
        case 6:
          uStack_f1 = 1;
          puVar10 = (undefined *)(ulonglong)bStack_f2;
          fVar21 = (float)func_0x000140b06770(puVar10);
          if (_DAT_1429cd060 <= (float)((uint)(fStack_164 - fVar21) & _DAT_14292e980)) break;
          goto LAB_1425f2e2d;
        case 8:
          if (cStack_fb != cVar1) break;
          if ((cStack_fb != '\x02') ||
             (((fStack_160 == fStack_f8 && (bStack_f9 == bStack_f2)) &&
              (puVar10 = (undefined *)(ulonglong)uStack_134, uStack_15c == uStack_134))))
          goto LAB_1425f2e2d;
          goto LAB_1425f2d35;
        case 9:
          if ((fStack_170 == fStack_f8) && (!NAN(fStack_170) && !NAN(fStack_f8)))
          goto LAB_1425f2e2d;
          break;
        case 10:
          bVar3 = bStack_fc;
joined_r0x0001425f2daa:
          if (bVar3 != bStack_f2) break;
          goto LAB_1425f2e2d;
        case 0xb:
          fVar21 = fStack_16c;
joined_r0x0001425f2df3:
          if (fVar21 != fStack_f8) break;
LAB_1425f2e2d:
          if ((uStack_100 & 1) == 0) {
            uStack_208 = lVar12 - 1U;
          }
          uStack_2b7 = 1;
          uStack_100 = (uint)CONCAT71((int7)((ulonglong)puVar10 >> 8),1);
          goto LAB_1425f2e60;
        }
LAB_1425f2d35:
        uStack_2b7 = 0;
LAB_1425f2e60:
        uStack_308 = uStack_140;
        uStack_300 = uStack_180;
        fStack_2d8 = *(float *)(&UNK_143136c10 + (ulonglong)(cVar1 == '\x05') * 4) * fVar24;
        fStack_2c0 = ((float)(lVar12 - 1U) * _DAT_1429eb6f8 + fVar25) * fVar22;
        uStack_2e0 = 0;
        uStack_2d0 = CONCAT44(fVar20 * radius + center_y,fVar19 * radius + center_x);
        uStack_2bc = 0;
        uStack_2b8 = 0;
        uStack_2b0 = 1;
        uStack_f1 = 0;
        uStack_2f8 = uVar13;
        uStack_2f0 = uVar18;
        fStack_2e8 = center_x;
        fStack_2e4 = center_y;
        fStack_2d4 = fStack_2d8;
        fStack_2c8 = center_x;
        fStack_2c4 = center_y;
        FUN_1422bf090(auStack_130,commands,&uStack_308,&UNK_143136ec0);
        uVar16 = uStack_110;
        plVar14 = plStack_128;
                    /* WARNING: Switch is manually overridden */
        switch(*(undefined4 *)(&UNK_143136f50 + (ulonglong)bVar5 * 4)) {
        case 0xff4bc017:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422bd840;
          goto LAB_1425f3057;
        case 0xff4bc055:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422be810;
          goto LAB_1425f3057;
        case 0xff4bc090:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422bde10;
          break;
        case 0xff4bc0ce:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422be4b0;
LAB_1425f3057:
          *(code **)(lVar8 + lVar6) = pcVar11;
          *(code **)(lVar8 + 8 + lVar6) = FUN_1422bf9d0;
          *(undefined8 *)(lVar8 + 0x10 + lVar6) = uVar16;
          *(float *)(lVar8 + 0x18 + lVar6) = fStack_f8;
          *(undefined1 *)(lVar8 + 0x1c + lVar6) = 0;
          goto LAB_1425f3200;
        case 0xff4bc130:
          plVar14 = plStack_128;
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          *(code **)(lVar8 + lVar6) = FUN_1422bd6d0;
          *(code **)(lVar8 + 8 + lVar6) = FUN_1422bf9d0;
          *(undefined8 *)(lVar8 + 0x10 + lVar6) = uVar16;
          *(undefined1 *)(lVar8 + 0x18 + lVar6) = 0;
          plVar14[2] = lVar6 + 0x20;
          lVar6 = lStack_198;
          if (lStack_248 == lStack_258) goto LAB_1425f33a4;
          goto LAB_1425f2c00;
        case 0xff4bc1a5:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422be3c0;
          break;
        case 0xff4bc1e3:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422bdf80;
          break;
        case 0xff4bc21e:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422bd5e0;
          goto LAB_1425f3297;
        case 0xff4bc25d:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422be070;
          break;
        case 0xff4bc2d1:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422be720;
          goto LAB_1425f3297;
        case 0xff4bc30d:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x20) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x20,1,1);
          }
          lVar8 = plVar14[1];
          pcVar11 = FUN_1422bd930;
LAB_1425f3297:
          *(code **)(lVar8 + lVar6) = pcVar11;
          *(code **)(lVar8 + 8 + lVar6) = FUN_1422bf9d0;
          *(undefined8 *)(lVar8 + 0x10 + lVar6) = uVar16;
          *(undefined1 *)(lVar8 + 0x18 + lVar6) = 0;
          *(byte *)(lVar8 + 0x19 + lVar6) = bStack_f2;
          lVar6 = lVar6 + 0x20;
LAB_1425f334c:
          plVar14[2] = lVar6;
          if (uStack_148 == uStack_158) goto LAB_1425f3360;
          goto LAB_1425f337a;
        case 0xff4bc375:
          if (plStack_128 == (longlong *)0x0) {
            plVar14 = plStack_120;
          }
          lVar6 = plVar14[2];
          if ((ulonglong)(*plVar14 - lVar6) < 0x28) {
            uStack_f1 = 0;
            FUN_1429107f0(plVar14,lVar6,0x28,1,1);
          }
          lVar8 = plVar14[1];
          *(code **)(lVar8 + lVar6) = FUN_1422bdc20;
          *(code **)(lVar8 + 8 + lVar6) = FUN_1422bf9d0;
          *(undefined8 *)(lVar8 + 0x10 + lVar6) = uVar16;
          *(char *)(lVar8 + 0x18 + lVar6) = cVar1;
          *(byte *)(lVar8 + 0x19 + lVar6) = bStack_f2;
          *(undefined2 *)(lVar8 + 0x1a + lVar6) = uStack_102;
          *(float *)(lVar8 + 0x1c + lVar6) = fStack_f8;
          *(uint *)(lVar8 + 0x20 + lVar6) = uStack_134;
          *(undefined1 *)(lVar8 + 0x24 + lVar6) = 0;
          lVar6 = lVar6 + 0x28;
          goto LAB_1425f334c;
        }
        *(code **)(lVar8 + lVar6) = pcVar11;
        *(code **)(lVar8 + 8 + lVar6) = FUN_1422bf9d0;
        *(undefined8 *)(lVar8 + 0x10 + lVar6) = uVar16;
        *(undefined1 *)(lVar8 + 0x18 + lVar6) = 0;
LAB_1425f3200:
        plVar14[2] = lVar6 + 0x20;
        if (uStack_148 == uStack_158) {
LAB_1425f3360:
          uVar13 = uStack_148;
          uStack_f1 = 0;
          FUN_140b81df0(&uStack_158,&UNK_143136ed8);
          uStack_148 = uVar13;
        }
LAB_1425f337a:
        *(undefined8 *)(uStack_150 + uStack_148 * 8) = uVar16;
        uStack_148 = uStack_148 + 1;
        lVar6 = lStack_198;
        if (lStack_248 == lStack_258) {
LAB_1425f33a4:
          lVar6 = lStack_198;
          lVar8 = lStack_248;
          uStack_f1 = 0;
          FUN_140b81df0(&lStack_258,&UNK_143136ef0);
          lStack_248 = lVar8;
        }
LAB_1425f2c00:
        *(undefined8 *)(lStack_250 + lStack_248 * 8) = uVar16;
        lStack_248 = lStack_248 + 1;
        fVar23 = fVar23 + fVar24 + spacing;
        lVar12 = lVar12 + 1;
        lVar6 = lVar6 + -0x28;
        puVar2 = puVar17;
      } while (lVar6 != 0);
    }
    puVar4 = puStack_190;
    if (puStack_188 != puVar17) {
      uVar13 = (ulonglong)((longlong)puStack_188 - (longlong)puVar17) / 0x28;
      puVar17 = puVar17 + 1;
      do {
        if (puVar17[-1] != 0) {
          func_0x000140613c20(*puVar17,puVar17[-1],1);
        }
        puVar17 = puVar17 + 5;
        uVar13 = uVar13 - 1;
      } while (uVar13 != 0);
    }
    func_0x000142923c00(&uStack_308,auStack_268,0xa0);
    uStack_f3 = 0;
    FUN_1422bf2c0(&puStack_1c8,commands,&uStack_308,&UNK_143136ea8);
    *(ulonglong *)(puVar4 + 4) = uStack_148;
    *puVar4 = (undefined4)uStack_158;
    puVar4[1] = uStack_158._4_4_;
    puVar4[2] = (undefined4)uStack_150;
    puVar4[3] = uStack_150._4_4_;
    *(undefined8 *)(puVar4 + 6) = uStack_1a8;
    lVar6 = *(longlong *)((longlong)icons + 8);
    lVar12 = *(longlong *)((longlong)icons + 0x10);
    if (lVar12 != 0) {
      puVar15 = (undefined8 *)(lVar6 + 8);
      do {
        if (puVar15[-1] != 0) {
          func_0x000140613c20(*puVar15,puVar15[-1],1);
        }
        puVar15 = puVar15 + 5;
        lVar12 = lVar12 + -1;
      } while (lVar12 != 0);
    }
    if (*(longlong *)icons != 0) {
      func_0x000140613c20(lVar6,*(longlong *)icons * 0x28,8);
    }
    return;
  }
LAB_1425f3755:
  FUN_1428d8fe3(uVar16,uVar7,&UNK_143136e60);
override_jmp_1425f2d31_case_2:
                    /* WARNING: Does not return */
  pcVar11 = (code *)invalidInstructionException();
  (*pcVar11)();
}

