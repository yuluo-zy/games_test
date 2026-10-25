   1	
   2	/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
   3	
   4	ulonglong *
   5	FUN_141fe0b90(ulonglong *param_1,float *param_2,ulonglong param_3,float param_4,float param_5)
   6	
   7	{
   8	  float fVar1;
   9	  float fVar2;
  10	  ulonglong uVar3;
  11	  code *pcVar4;
  12	  float fVar5;
  13	  ulonglong uVar6;
  14	  undefined8 *puVar7;
  15	  ulonglong uVar8;
  16	  ulonglong uVar9;
  17	  ulonglong uVar10;
  18	  ulonglong *puVar11;
  19	  ulonglong uVar12;
  20	  float *pfVar13;
  21	  ulonglong uVar14;
  22	  undefined8 uVar15;
  23	  ulonglong uVar16;
  24	  undefined8 *unaff_R13;
  25	  longlong unaff_R14;
  26	  bool bVar17;
  27	  float fVar18;
  28	  float fVar19;
  29	  float fVar20;
  30	  float fVar21;
  31	  float fVar22;
  32	  float fVar23;
  33	  float fVar24;
  34	  float fVar25;
  35	  float fVar26;
  36	  float fVar27;
  37	  float fVar28;
  38	  float fVar29;
  39	  float fVar30;
  40	  float fVar31;
  41	  float fVar32;
  42	  float fVar33;
  43	  float fVar34;
  44	  float fVar35;
  45	  float fVar36;
  46	  float fVar37;
  47	  ulonglong uStack_178;
  48	  ulonglong uStack_170;
  49	  ulonglong uStack_168;
  50	  undefined8 *puStack_160;
  51	  undefined8 *puStack_158;
  52	  float *pfStack_150;
  53	  ulonglong uStack_148;
  54	  float *pfStack_140;
  55	  float fStack_138;
  56	  float fStack_134;
  57	  ulonglong uStack_130;
  58	  longlong lStack_128;
  59	  ulonglong uStack_120;
  60	  ulonglong uStack_118;
  61	  undefined8 *puStack_110;
  62	  ulonglong uStack_108;
  63	  ulonglong uStack_100;
  64	  undefined8 *puStack_f8;
  65	  undefined8 uStack_f0;
  66	  
  67	  uStack_f0 = 0xfffffffffffffffe;
  68	  if (param_3 < 3) {
  69	    if (param_3 == 0) {
  70	      uVar8 = 4;
  71	      puVar7 = (undefined8 *)0x8;
  72	      unaff_R14 = 0;
  73	      uVar16 = 0;
  74	LAB_141fe1316:
  75	      func_0x000142923c00(uVar8,param_2,unaff_R14);
  76	      *param_1 = uVar16;
  77	      param_1[1] = (ulonglong)puVar7;
  78	      param_1[2] = uVar16;
  79	      param_1[3] = param_3;
  80	      param_1[4] = uVar8;
  81	      param_1[5] = param_3;
  82	                    /* WARNING: Read-only address (ram,0x0001429258f0) is written */
  83	      return param_1;
  84	    }
  85	    uVar9 = param_3 * 8;
  86	    puVar7 = (undefined8 *)func_0x000140613c10(uVar9,8);
  87	    if (puVar7 == (undefined8 *)0x0) {
  88	      FUN_1428d8fe3(8,uVar9,&UNK_142fcb408);
  89	    }
  90	    else {
  91	      if (true) {
  92	        *puVar7 = 0;
  93	      }
  94	      if (true) {
  95	        puVar7[1] = 1;
  96	      }
  97	      unaff_R14 = param_3 * 0xc;
  98	      uVar8 = func_0x000140613c10(unaff_R14,4);
  99	      uVar16 = param_3;
 100	      unaff_R13 = puVar7;
 101	      if (uVar8 != 0) goto LAB_141fe1316;
 102	    }
 103	    uStack_100 = uVar9;
 104	    puStack_f8 = unaff_R13;
 105	    FUN_1428d8fe3(4,unaff_R14,&UNK_142fcb470);
 106	  }
 107	  else {
 108	    uStack_130 = 0;
 109	    lStack_128 = 8;
 110	    uStack_120 = 0;
 111	    uStack_118 = 0;
 112	    puStack_110 = (undefined8 *)0x8;
 113	    uStack_108 = 0;
 114	    FUN_140b81df0(&uStack_118,&UNK_142fcb4d0);
 115	    uVar16 = uStack_120;
 116	    *puStack_110 = 0;
 117	    uStack_108 = 1;
 118	    if (uStack_120 == uStack_130) {
 119	      FUN_140562db0(&uStack_130,&UNK_142fcb4e8);
 120	    }
 121	    *(undefined8 *)(lStack_128 + uVar16 * 0x18) = 0;
 122	    *(ulonglong *)(lStack_128 + 8 + uVar16 * 0x18) = param_3 - 1;
 123	    *(undefined1 *)(lStack_128 + 0x10 + uVar16 * 0x18) = 0;
 124	    uStack_120 = uVar16 + 1;
 125	    pfVar13 = param_2 + param_3 * 3;
 126	    if (pfVar13 == (float *)0xc) {
 127	      FUN_1428d9310(&UNK_142fcb500);
 128	    }
 129	    else {
 130	      fVar25 = SQRT((param_2[2] - pfVar13[-1]) * (param_2[2] - pfVar13[-1]) +
 131	                    (param_2[1] - pfVar13[-2]) * (param_2[1] - pfVar13[-2]) +
 132	                    (*param_2 - pfVar13[-3]) * (*param_2 - pfVar13[-3]));
 133	      uStack_100 = CONCAT44(uStack_100._4_4_,
 134	                            (int)CONCAT71((int7)((ulonglong)pfVar13 >> 8),fVar25 < 1e+08));
 135	      pfStack_140 = param_2 + 5;
 136	      puStack_f8 = (undefined8 *)CONCAT44(puStack_f8._4_4_,param_5 * param_5);
 137	      fVar32 = param_5 * param_5;
 138	      while (uVar16 = uStack_120, uStack_120 != 0) {
 139	        while( true ) {
 140	          uVar8 = uStack_108;
 141	          uStack_120 = uVar16 - 1;
 142	          if (*(char *)(lStack_128 + 0x10 + uStack_120 * 0x18) == '\x02') goto LAB_141fe0df0;
 143	          puVar11 = (ulonglong *)(lStack_128 + uStack_120 * 0x18);
 144	          uVar9 = *(ulonglong *)(lStack_128 + 8 + uStack_120 * 0x18);
 145	          uVar3 = *puVar11;
 146	          if (2 < (uVar9 - uVar3) + 1) break;
 147	          if (uStack_108 == uStack_118) {
 148	            FUN_140b81df0(&uStack_118,&UNK_142fcb5a8);
 149	          }
 150	          puStack_110[uVar8] = uVar9;
 151	          uStack_108 = uVar8 + 1;
 152	          uVar16 = uStack_120;
 153	          if (uStack_120 == 0) goto LAB_141fe0df0;
 154	        }
 155	        if (param_3 <= uVar3) {
 156	          FUN_1428d9518(uVar3,param_3,&UNK_142fcb518);
 157	          goto LAB_141fe14f1;
 158	        }
 159	        if (param_3 <= uVar9) {
 160	          FUN_1428d9518(uVar9,param_3,&UNK_142fcb530);
 161	          goto LAB_141fe14f1;
 162	        }
 163	        fStack_138 = fVar25;
 164	        if (uVar9 < uVar3) {
 165	          FUN_1428d9850(uVar3,uVar9,&UNK_142fcb548);
 166	          goto LAB_141fe14f1;
 167	        }
 168	        fVar5 = puStack_f8._0_4_;
 169	        fStack_134 = param_4;
 170	        if (param_2 + uVar3 * 3 + 3 == param_2 + uVar9 * 3 || uVar9 == uVar3) {
 171	          fVar32 = 0.0;
 172	          uVar10 = 0;
 173	        }
 174	        else {
 175	          fVar1 = param_2[uVar3 * 3 + 2];
 176	          fVar30 = (float)*(undefined8 *)(param_2 + uVar9 * 3);
 177	          fVar31 = (float)((ulonglong)*(undefined8 *)(param_2 + uVar9 * 3) >> 0x20);
 178	          fVar2 = param_2[uVar9 * 3 + 2];
 179	          uVar12 = (ulonglong)
 180	                   ((longlong)(param_2 + uVar9 * 3) - (longlong)(param_2 + uVar3 * 3 + 3)) / 0xc;
 181	          fVar18 = (float)*(undefined8 *)(param_2 + uVar3 * 3);
 182	          fVar20 = fVar30 - fVar18;
 183	          fVar19 = (float)((ulonglong)*(undefined8 *)(param_2 + uVar3 * 3) >> 0x20);
 184	          fVar21 = fVar31 - fVar19;
 185	          fVar22 = fVar2 - fVar1;
 186	          fVar23 = fVar22 * fVar22 + fVar21 * fVar21 + fVar20 * fVar20;
 187	          if (fVar23 <= 1e-20) {
 188	            fVar23 = 1e-20;
 189	          }
 190	          if ((fVar32 != 0.0) || (NAN(fVar32))) {
 191	            pfVar13 = pfStack_140 + uVar3 * 3;
 192	            fVar32 = 0.0;
 193	            uVar10 = 0;
 194	            uVar14 = 1;
 195	            do {
 196	              fVar28 = (float)*(undefined8 *)(pfVar13 + -2);
 197	              fVar29 = (float)((ulonglong)*(undefined8 *)(pfVar13 + -2) >> 0x20);
 198	              fVar24 = *pfVar13;
 199	              fVar35 = fVar28 - fVar18;
 200	              fVar36 = fVar29 - fVar19;
 201	              fVar37 = fVar24 - fVar1;
 202	              fVar34 = (fVar22 * fVar37 + fVar21 * fVar36 + fVar20 * fVar35) / fVar23;
 203	              fVar26 = 1.0 - fVar34;
 204	              fVar33 = fVar24 - (fVar2 * fVar34 + fVar1 * fVar26);
 205	              fVar27 = fVar28 - (fVar34 * fVar30 + fVar26 * fVar18);
 206	              fVar34 = fVar29 - (fVar34 * fVar31 + fVar26 * fVar19);
 207	              fVar34 = SQRT(fVar33 * fVar33 + fVar34 * fVar34 + fVar27 * fVar27);
 208	              if (0x7f7fffff < (uint)ABS(fVar34)) goto LAB_141fe1446;
 209	              if (((fVar32 < fVar34) &&
 210	                  (puStack_f8._0_4_ <= fVar37 * fVar37 + fVar36 * fVar36 + fVar35 * fVar35)) &&
 211	                 (puStack_f8._0_4_ <=
 212	                  (fVar24 - fVar2) * (fVar24 - fVar2) +
 213	                  (fVar29 - fVar31) * (fVar29 - fVar31) + (fVar28 - fVar30) * (fVar28 - fVar30))) {
 214	                uVar10 = uVar14;
 215	                fVar32 = fVar34;
 216	              }
 217	              pfVar13 = pfVar13 + 3;
 218	              bVar17 = uVar14 != uVar12;
 219	              uVar14 = uVar14 + 1;
 220	            } while (bVar17);
 221	          }
 222	          else {
 223	            pfVar13 = pfStack_140 + uVar3 * 3;
 224	            fVar32 = 0.0;
 225	            uVar10 = 0;
 226	            uVar14 = 1;
 227	            do {
 228	              fVar34 = (float)*(undefined8 *)(pfVar13 + -2);
 229	              fVar24 = (float)((ulonglong)*(undefined8 *)(pfVar13 + -2) >> 0x20);
 230	              fVar27 = ((*pfVar13 - fVar1) * fVar22 +
 231	                       (fVar24 - fVar19) * fVar21 + (fVar34 - fVar18) * fVar20) / fVar23;
 232	              fVar26 = 1.0 - fVar27;
 233	              fVar24 = fVar24 - (fVar31 * fVar27 + fVar19 * fVar26);
 234	              fVar33 = *pfVar13 - (fVar27 * fVar2 + fVar26 * fVar1);
 235	              fVar34 = fVar34 - (fVar27 * fVar30 + fVar26 * fVar18);
 236	              fVar24 = SQRT(fVar34 * fVar34 + fVar24 * fVar24 + fVar33 * fVar33);
 237	              if (0x7f7fffff < (uint)ABS(fVar24)) goto LAB_141fe1446;
 238	              uVar6 = uVar14;
 239	              if (fVar24 <= fVar32) {
 240	                fVar24 = fVar32;
 241	                uVar6 = uVar10;
 242	              }
 243	              uVar10 = uVar6;
 244	              fVar32 = fVar24;
 245	              pfVar13 = pfVar13 + 3;
 246	              bVar17 = uVar14 != uVar12;
 247	              uVar14 = uVar14 + 1;
 248	            } while (bVar17);
 249	          }
 250	        }
 251	        if ((param_4 < fVar32) || ((fVar25 < fVar32 & (byte)uStack_100) != 0)) {
 252	          uVar9 = uVar10 + uVar3;
 253	          *puVar11 = uVar9;
 254	          *(undefined1 *)(puVar11 + 2) = 0;
 255	          uStack_120 = uVar16;
 256	          if (uVar10 < 3) {
 257	            if (uStack_108 == uStack_118) {
 258	              FUN_140b81df0(&uStack_118,&UNK_142fcb578);
 259	            }
 260	            puStack_110[uVar8] = uVar9;
 261	            uStack_100 = uStack_100 & 0xffffffff00000000;
 262	            goto LAB_141fe1193;
 263	          }
 264	          if (uVar16 == uStack_130) {
 265	            FUN_140562db0(&uStack_130,&UNK_142fcb590);
 266	          }
 267	          *(ulonglong *)(lStack_128 + uVar16 * 0x18) = uVar3;
 268	          *(ulonglong *)(lStack_128 + 8 + uVar16 * 0x18) = uVar9;
 269	          *(undefined1 *)(lStack_128 + 0x10 + uVar16 * 0x18) = 0;
 270	          uStack_100 = uStack_100 & 0xffffffff00000000;
 271	          puVar11 = &uStack_120;
 272	        }
 273	        else {
 274	          if (uStack_108 == uStack_118) {
 275	            FUN_140b81df0(&uStack_118,&UNK_142fcb560);
 276	          }
 277	          puStack_110[uVar8] = uVar9;
 278	LAB_141fe1193:
 279	          puVar11 = &uStack_108;
 280	          uVar16 = uVar8;
 281	        }
 282	        *puVar11 = uVar16 + 1;
 283	        fVar32 = fVar5;
 284	      }
 285	LAB_141fe0df0:
 286	      uVar8 = uStack_108;
 287	      puVar7 = puStack_110;
 288	      uVar16 = uStack_108 * 8;
 289	      uVar15 = 0;
 290	      if ((uStack_108 >> 0x3d == 0) && (uVar16 < 0x7ffffffffffffff9)) {
 291	        if (uVar16 == 0) {
 292	          uVar9 = 8;
 293	          puStack_f8 = (undefined8 *)0x0;
 294	LAB_141fe134d:
 295	          uStack_100 = uVar9;
 296	          func_0x000142923c00(uVar9,puVar7,uVar16);
 297	          puStack_158 = puStack_110 + uStack_108;
 298	          puStack_160 = puStack_110;
 299	          pfStack_150 = param_2;
 300	          uStack_148 = param_3;
 301	          FUN_141206920(&uStack_178,&puStack_160,&UNK_142fcb408);
 302	          *param_1 = (ulonglong)puStack_f8;
 303	          param_1[1] = uStack_100;
 304	          param_1[2] = uVar8;
 305	          param_1[5] = uStack_168;
 306	          param_1[3] = uStack_178;
 307	          param_1[4] = uStack_170;
 308	          if (uStack_118 != 0) {
 309	            func_0x000140613c20(puStack_110,uStack_118 << 3,8);
 310	          }
 311	          if (uStack_130 == 0) {
 312	            return param_1;
 313	          }
 314	          func_0x000140613c20(lStack_128,uStack_130 * 0x18,8);
 315	          return param_1;
 316	        }
 317	        uVar15 = 8;
 318	        uVar9 = func_0x000140613c10(uVar16,8);
 319	        if (uVar9 != 0) {
 320	          puStack_f8 = (undefined8 *)uVar8;
 321	          goto LAB_141fe134d;
 322	        }
 323	      }
 324	      FUN_1428d8fe3(uVar15,uVar16,&UNK_142c19f70);
 325	    }
 326	  }
 327	LAB_141fe14f1:
 328	                    /* WARNING: Does not return */
 329	  pcVar4 = (code *)invalidInstructionException();
 330	  (*pcVar4)();
 331	LAB_141fe1446:
 332	  FUN_1428d9430(&UNK_142fcb5c0,0x26,&UNK_142fcb5e8);
 333	  goto LAB_141fe14f1;
 334	}
 335	
