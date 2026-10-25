   1	
   2	void FUN_14233c300(longlong *param_1,longlong param_2,longlong *param_3)
   3	
   4	{
   5	  longlong lVar1;
   6	  longlong lVar2;
   7	  longlong lVar3;
   8	  longlong lVar4;
   9	  longlong lVar5;
  10	  ulonglong uVar6;
  11	  undefined4 *puVar7;
  12	  longlong lVar8;
  13	  byte bVar9;
  14	  longlong *plVar10;
  15	  ulonglong uVar11;
  16	  ulonglong uVar12;
  17	  longlong lVar13;
  18	  ulonglong uVar14;
  19	  char *pcVar15;
  20	  float *pfVar16;
  21	  uint *puVar17;
  22	  byte bVar18;
  23	  float fVar19;
  24	  float fVar20;
  25	  longlong lStack_1d8;
  26	  longlong lStack_1c8;
  27	  longlong lStack_1c0;
  28	  longlong lStack_1b8;
  29	  longlong lStack_1b0;
  30	  longlong lStack_198;
  31	  uint *puStack_140;
  32	  uint *puStack_138;
  33	  longlong lStack_130;
  34	  longlong lStack_120;
  35	  ulonglong uStack_110;
  36	  ulonglong uStack_108;
  37	  longlong lStack_100;
  38	  byte bStack_f8;
  39	  longlong lStack_f0;
  40	  longlong lStack_e0;
  41	  undefined4 uStack_d4;
  42	  longlong lStack_d0;
  43	  undefined4 uStack_b4;
  44	  longlong lStack_b0;
  45	  longlong lStack_a0;
  46	  undefined4 uStack_94;
  47	  byte bStack_88;
  48	  byte bStack_80;
  49	  
  50	  lVar1 = *param_1;
  51	  lVar2 = param_1[1];
  52	  func_0x000142623450(&puStack_140,lVar2,lVar1,(int)param_1[2],
  53	                      *(undefined4 *)((longlong)param_1 + 0x14));
  54	  plVar10 = *(longlong **)(param_2 + 8);
  55	  if (plVar10 == (longlong *)0x0) {
  56	    plVar10 = *(longlong **)(param_2 + 0x10);
  57	  }
  58	  lStack_198 = lStack_120;
  59	  lStack_1d8 = lStack_100;
  60	  lStack_1b0 = lStack_f0;
  61	  lStack_1b8 = lStack_e0;
  62	  lStack_1c0 = lStack_b0;
  63	  lVar3 = *param_3;
  64	  lStack_1c8 = lStack_a0;
  65	  uVar12 = uStack_110;
  66	  puVar17 = puStack_140;
  67	  bVar9 = bStack_f8;
  68	  bVar18 = bStack_88;
  69	  do {
  70	    if ((bStack_80 & 1) == 0) {
  71	      if (uStack_108 == uVar12) {
  72	        if (puVar17 == puStack_138) {
  73	          return;
  74	        }
  75	        while( true ) {
  76	          uVar12 = *(ulonglong *)(*(longlong *)(lVar2 + 0x100) + 0x58 + (ulonglong)*puVar17 * 0xa0);
  77	          if (uVar12 != 0) break;
  78	          puVar17 = puVar17 + 1;
  79	          if (puVar17 == puStack_138) {
  80	            return;
  81	          }
  82	        }
  83	        lVar8 = *(longlong *)(lVar2 + 0x100) + (ulonglong)*puVar17 * 0xa0;
  84	        uVar6 = (ulonglong)*(uint *)(lVar8 + 0x94);
  85	        lVar13 = *(longlong *)(lVar2 + 0x1a8);
  86	        lVar5 = *(longlong *)(lVar13 + 0x18 + uVar6 * 0x48);
  87	        lVar4 = *(longlong *)(lVar13 + 0x38 + uVar6 * 0x48);
  88	        uVar11 = *(ulonglong *)(lVar1 + 0x128);
  89	        if ((uVar11 < *(ulonglong *)(lVar8 + 0x88)) &&
  90	           (*(longlong *)(*(longlong *)(lVar8 + 0x80) + uVar11 * 8) != 0)) {
  91	          bVar9 = 1;
  92	          if ((uVar11 < *(ulonglong *)(lVar13 + 0x40 + uVar6 * 0x48)) &&
  93	             (uVar11 = *(ulonglong *)(lVar4 + uVar11 * 8), uVar11 != 0)) {
  94	            lStack_1d8 = *(longlong *)(lVar5 + 0x10 + ~uVar11 * 0x30);
  95	          }
  96	          else {
  97	            lStack_1d8 = 0;
  98	          }
  99	        }
 100	        else {
 101	          bVar9 = 0;
 102	        }
 103	        if (*(ulonglong *)(lVar1 + 0x130) < *(ulonglong *)(lVar8 + 0x88)) {
 104	          bVar18 = *(longlong *)(*(longlong *)(lVar8 + 0x80) + *(ulonglong *)(lVar1 + 0x130) * 8) !=
 105	                   0;
 106	        }
 107	        else {
 108	          bVar18 = false;
 109	        }
 110	        lVar13 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar1 + 0x110) * 8) * 0x30;
 111	        lStack_1b0 = *(longlong *)(lVar5 + 0x10 + lVar13);
 112	        lStack_1b8 = *(longlong *)(lVar5 + 0x28 + lVar13);
 113	        lVar13 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar1 + 0x120) * 8) * 0x30;
 114	        lStack_1c0 = *(longlong *)(lVar5 + 0x10 + lVar13);
 115	        lStack_1c8 = *(longlong *)(lVar5 + 0x28 + lVar13);
 116	        puVar17 = puVar17 + 1;
 117	        lStack_198 = *(longlong *)(lVar8 + 0x50);
 118	        uStack_108 = 0;
 119	      }
 120	      uVar11 = *(ulonglong *)(lStack_198 + uStack_108 * 0x10);
 121	      puVar7 = (undefined4 *)(uVar11 & 0xffffffff);
 122	      if ((puVar7 < *(undefined4 **)(lStack_d0 + 0x88)) &&
 123	         (*(int *)(*(longlong *)(lStack_d0 + 0x80) + (longlong)puVar7 * 8) == 1)) {
 124	        uVar6 = (ulonglong)*(uint *)(*(longlong *)(lStack_d0 + 0x80) + 4 + (longlong)puVar7 * 8);
 125	        pfVar16 = (float *)(*(longlong *)(lStack_d0 + 8) * uVar6 + *(longlong *)(lStack_d0 + 0x10));
 126	        puVar7 = (undefined4 *)(uVar6 * 4 + *(longlong *)(lStack_d0 + 0x50));
 127	      }
 128	      else {
 129	        pfVar16 = (float *)0x0;
 130	      }
 131	      uVar14 = (ulonglong)*(uint *)(lStack_198 + 8 + uStack_108 * 0x10);
 132	      uVar6 = uStack_108;
 133	    }
 134	    else {
 135	      if (uStack_108 == uVar12) {
 136	        return;
 137	      }
 138	      uVar11 = *(ulonglong *)(lStack_130 + uStack_108 * 8);
 139	      puVar7 = (undefined4 *)(uVar11 & 0xffffffff);
 140	      if ((puVar7 < *(undefined4 **)(lStack_d0 + 0x88)) &&
 141	         (*(int *)(*(longlong *)(lStack_d0 + 0x80) + (longlong)puVar7 * 8) == 1)) {
 142	        uVar6 = (ulonglong)*(uint *)(*(longlong *)(lStack_d0 + 0x80) + 4 + (longlong)puVar7 * 8);
 143	        pfVar16 = (float *)(*(longlong *)(lStack_d0 + 8) * uVar6 + *(longlong *)(lStack_d0 + 0x10));
 144	        puVar7 = (undefined4 *)(uVar6 * 4 + *(longlong *)(lStack_d0 + 0x50));
 145	      }
 146	      else {
 147	        pfVar16 = (float *)0x0;
 148	      }
 149	      uVar14 = uStack_108 & 0xffffffff;
 150	      uVar6 = uStack_108;
 151	    }
 152	    lVar13 = lStack_1b0 + uVar14 * 0x14;
 153	    pcVar15 = (char *)(uVar14 + lStack_1d8);
 154	    if ((bVar9 & 1) == 0) {
 155	      pcVar15 = (char *)0x0;
 156	    }
 157	    fVar20 = *(float *)(lVar3 + 0x58);
 158	    fVar19 = pfVar16[1];
 159	    *puVar7 = uStack_b4;
 160	    fVar19 = fVar20 / fVar19 + pfVar16[2];
 161	    fVar20 = 0.0;
 162	    if (0.0 <= fVar19) {
 163	      fVar20 = fVar19;
 164	    }
 165	    fVar19 = 1.0;
 166	    if (fVar20 <= 1.0) {
 167	      fVar19 = fVar20;
 168	    }
 169	    pfVar16[2] = fVar19;
 170	    fVar19 = (1.0 - fVar19) * 0.1 + fVar19;
 171	    fVar19 = fVar19 * 4.0 * (1.0 - fVar19);
 172	    fVar20 = *pfVar16;
 173	    *(undefined4 *)(lStack_1b8 + uVar14 * 4) = uStack_d4;
 174	    *(float *)(lVar13 + 0x10) = (fVar19 * fVar19 * fVar20 + 1.0) * *(float *)(lVar13 + 0x10);
 175	    fVar20 = pfVar16[2];
 176	    if ((fVar20 == 1.0) && (!NAN(fVar20))) {
 177	      lVar13 = plVar10[2];
 178	      if ((ulonglong)(*plVar10 - lVar13) < 0x18) {
 179	        FUN_1428a58d0(fVar20,lVar13,0x18,1,1);
 180	      }
 181	      lVar5 = plVar10[1];
 182	      *(code **)(lVar5 + lVar13) = FUN_140fde3f0;
 183	      *(code **)(lVar5 + 8 + lVar13) = FUN_140fdfb10;
 184	      *(ulonglong *)(lVar5 + 0x10 + lVar13) = uVar11;
 185	      plVar10[2] = lVar13 + 0x18;
 186	    }
 187	    uStack_108 = uVar6 + 1;
 188	    if (((pcVar15 != (char *)0x0) && ((*pcVar15 == '\x04' || (*pcVar15 == '	')))) ||
 189	       ((bVar18 & 1) != 0)) {
 190	      fVar20 = pfVar16[2];
 191	      fVar19 = pfVar16[2];
 192	      *(undefined4 *)(lStack_1c8 + uVar14 * 4) = uStack_94;
 193	      *(bool *)(uVar14 * 3 + lStack_1c0 + 2) = fVar19 <= 1.0 && fVar20 != 1.0;
 194	    }
 195	  } while( true );
 196	}
 197	