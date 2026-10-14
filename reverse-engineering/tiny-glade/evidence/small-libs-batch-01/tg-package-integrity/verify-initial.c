   1	
   2	/* WARNING: Type propagation algorithm not settling */
   3	
   4	undefined8 * FUN_14067c3d0(undefined8 *param_1)
   5	
   6	{
   7	  code *pcVar1;
   8	  longlong *******ppppppplVar2;
   9	  longlong *******ppppppplVar3;
  10	  longlong *plVar4;
  11	  longlong ******pppppplVar5;
  12	  char cVar6;
  13	  undefined8 uVar7;
  14	  byte *pbVar8;
  15	  undefined8 *puVar9;
  16	  longlong *******ppppppplVar10;
  17	  longlong lVar11;
  18	  longlong *plVar12;
  19	  ulonglong uVar13;
  20	  undefined1 auStack_1d8 [88];
  21	  longlong *plStack_180;
  22	  longlong *plStack_178;
  23	  longlong *plStack_170;
  24	  longlong *plStack_168;
  25	  longlong lStack_160;
  26	  undefined4 uStack_158;
  27	  undefined4 uStack_154;
  28	  undefined4 uStack_150;
  29	  undefined4 uStack_14c;
  30	  undefined4 uStack_148;
  31	  undefined4 uStack_144;
  32	  int iStack_140;
  33	  undefined4 uStack_13c;
  34	  undefined8 uStack_138;
  35	  undefined8 uStack_130;
  36	  undefined8 uStack_128;
  37	  int iStack_120;
  38	  undefined4 uStack_11c;
  39	  longlong *******ppppppplStack_118;
  40	  longlong *******ppppppplStack_110;
  41	  longlong *******ppppppplStack_108;
  42	  longlong *******ppppppplStack_100;
  43	  longlong *******ppppppplStack_f8;
  44	  undefined8 uStack_f0;
  45	  undefined8 uStack_e8;
  46	  undefined8 uStack_e0;
  47	  longlong *******ppppppplStack_d8;
  48	  undefined8 uStack_d0;
  49	  undefined8 uStack_c8;
  50	  undefined8 uStack_c0;
  51	  undefined8 uStack_b8;
  52	  undefined8 uStack_b0;
  53	  longlong *******ppppppplStack_a8;
  54	  longlong *******ppppppplStack_a0;
  55	  longlong *******ppppppplStack_98;
  56	  longlong *******ppppppplStack_90;
  57	  longlong *******ppppppplStack_88;
  58	  longlong *******ppppppplStack_80;
  59	  longlong *******ppppppplStack_78;
  60	  longlong *******ppppppplStack_70;
  61	  longlong *******ppppppplStack_68;
  62	  longlong *******ppppppplStack_60;
  63	  longlong *plStack_58;
  64	  undefined1 uStack_4a;
  65	  undefined1 uStack_49;
  66	  undefined8 uStack_48;
  67	  
  68	  uStack_48 = 0xfffffffffffffffe;
  69	  FUN_140d58eb0(&uStack_e8,&UNK_142a16af0,0x18);
  70	  ppppppplVar2 = ppppppplStack_d8;
  71	  if (SBORROW8(0,(longlong)uStack_e8)) {
  72	    ppppppplStack_108 = uStack_e0;
  73	    uStack_138 = (longlong ******)0x0;
  74	    uStack_130 = (longlong *)0x1;
  75	    uStack_128 = 0;
  76	    uStack_c8 = (longlong *******)0x0;
  77	    uStack_b8 = (longlong *******)0x0;
  78	    ppppppplStack_a8 = (longlong *******)0x20;
  79	    ppppppplStack_a0 = (longlong *******)CONCAT71(ppppppplStack_a0._1_7_,3);
  80	    ppppppplStack_98 = (longlong *******)&uStack_138;
  81	    ppppppplStack_90 = (longlong *******)&UNK_142a169e0;
  82	    cVar6 = FUN_140d5b010(&ppppppplStack_108,&uStack_c8);
  83	    lVar11 = uStack_128;
  84	    plVar4 = uStack_130;
  85	    pppppplVar5 = uStack_138;
  86	    if (cVar6 == '\0') {
  87	      if (((uint)uStack_e0 & 3) == 1) {
  88	        ppppppplStack_68 = (longlong *******)((longlong)uStack_e0 + -1);
  89	        plStack_58 = *(longlong **)((longlong)uStack_e0 + -1);
  90	        ppppppplStack_60 = *(longlong ********)((longlong)uStack_e0 + 7);
  91	        if (*ppppppplStack_60 != (longlong ******)0x0) {
  92	          (*(code *)*ppppppplStack_60)(plStack_58);
  93	        }
  94	        if (ppppppplStack_60[1] != (longlong ******)0x0) {
  95	          func_0x000140613c20(plStack_58,ppppppplStack_60[1],ppppppplStack_60[2]);
  96	        }
  97	        func_0x000140613c20(ppppppplStack_68,0x18,8);
  98	      }
  99	      param_1[1] = pppppplVar5;
 100	      param_1[2] = plVar4;
 101	      param_1[3] = lVar11;
 102	      *param_1 = 0x8000000000000001;
 103	      return param_1;
 104	    }
 105	    FUN_1428d9760(&UNK_142a16a10,0x37,&uStack_4a,&UNK_142a16ab0,&UNK_142a16a98);
 106	code_r0x00014067cd5d:
 107	                    /* WARNING: Does not return */
 108	    pcVar1 = (code *)invalidInstructionException();
 109	    (*pcVar1)();
 110	  }
 111	  ppppppplStack_68 = uStack_e8;
 112	  ppppppplStack_60 = uStack_e0;
 113	  FUN_140db6090(auStack_1d8,uStack_e0,ppppppplStack_d8);
 114	  uStack_c8 = (longlong *******)0xf52dad71b327e45a;
 115	  func_0x000140f737f0(auStack_1d8,&uStack_c8);
 116	  if ((short)uStack_c8 == 0) {
 117	    ppppppplVar10 = ppppppplStack_60;
 118	    FUN_140f739a0(&uStack_138,ppppppplStack_60,ppppppplVar2);
 119	    ppppppplVar3 = ppppppplStack_60;
 120	    ppppppplVar2 = ppppppplStack_68;
 121	    plVar4 = uStack_130;
 122	    if (!SBORROW8(0,(longlong)uStack_138)) {
 123	      uStack_148 = (undefined4)uStack_128;
 124	      uStack_144 = uStack_128._4_4_;
 125	      iStack_140 = iStack_120;
 126	      uStack_13c = uStack_11c;
 127	      uStack_158 = (undefined4)uStack_138;
 128	      uStack_154 = uStack_138._4_4_;
 129	      uStack_150 = (undefined4)uStack_130;
 130	      uStack_14c = uStack_130._4_4_;
 131	      if (iStack_120 == 1) {
 132	        pbVar8 = (byte *)FUN_140d4d740();
 133	        if ((*pbVar8 & 1) == 0) {
 134	          uStack_49 = 1;
 135	          ppppppplStack_a8 = (longlong *******)FUN_140d64e00();
 136	          pbVar8[0] = 1;
 137	          pbVar8[1] = 0;
 138	          pbVar8[2] = 0;
 139	          pbVar8[3] = 0;
 140	          pbVar8[4] = 0;
 141	          pbVar8[5] = 0;
 142	          pbVar8[6] = 0;
 143	          pbVar8[7] = 0;
 144	          *(longlong ********)(pbVar8 + 8) = ppppppplStack_a8;
 145	          *(longlong ********)(pbVar8 + 0x10) = ppppppplVar10;
 146	        }
 147	        else {
 148	          ppppppplStack_a8 = *(longlong ********)(pbVar8 + 8);
 149	          ppppppplVar10 = *(longlong ********)(pbVar8 + 0x10);
 150	        }
 151	        *(undefined **)(pbVar8 + 8) = (undefined *)((longlong)ppppppplStack_a8 + 1);
 152	        uStack_b8 = (longlong *******)0x0;
 153	        uStack_b0 = (longlong *******)0x0;
 154	        uStack_c8 = (longlong *******)&UNK_142a169b0;
 155	        uStack_c0 = (longlong *******)0x0;
 156	        ppppppplStack_a0 = ppppppplVar10;
 157	        if (uStack_128 != 0) {
 158	          FUN_1428e5220(&uStack_c8,uStack_128,&ppppppplStack_a8,1);
 159	          lVar11 = 0;
 160	          do {
 161	            FUN_140d7bd90(&uStack_e8,uStack_130 + lVar11 * 6);
 162	            FUN_140f7a7a0(&uStack_c8,&uStack_e8);
 163	            lVar11 = lVar11 + 1;
 164	          } while (lVar11 != uStack_128);
 165	        }
 166	        ppppppplStack_118 = ppppppplStack_a8;
 167	        ppppppplStack_110 = ppppppplStack_a0;
 168	        uStack_128 = (longlong)uStack_b8;
 169	        iStack_120 = (int)uStack_b0;
 170	        uStack_11c = uStack_b0._4_4_;
 171	        uStack_138 = (longlong ******)uStack_c8;
 172	        uStack_130 = (longlong *)uStack_c0;
 173	        uStack_e8 = (longlong *******)0x0;
 174	        uStack_e0 = (longlong *******)0x8;
 175	        ppppppplStack_d8 = (longlong *******)0x0;
 176	        uStack_b8 = (longlong *******)CONCAT44(uStack_144,uStack_148);
 177	        uStack_c8 = (longlong *******)CONCAT44(uStack_154,uStack_158);
 178	        uStack_c0 = (longlong *******)CONCAT44(uStack_14c,uStack_150);
 179	        FUN_140f77ad0(&ppppppplStack_108,&uStack_e8,&uStack_c8);
 180	        FUN_140f76810(&plStack_170,&ppppppplStack_108);
 181	        plVar4 = plStack_170;
 182	        ppppppplStack_80 = (longlong *******)0x0;
 183	        ppppppplStack_78 = (longlong *******)0x8;
 184	        ppppppplStack_70 = (longlong *******)0x0;
 185	        ppppppplStack_108 = (longlong *******)0x0;
 186	        ppppppplStack_100 = (longlong *******)0x8;
 187	        ppppppplStack_f8 = (longlong *******)0x0;
 188	        uStack_b0 = (longlong *******)(plStack_168 + lStack_160 * 4);
 189	        uStack_c8 = (longlong *******)plStack_168;
 190	        uStack_c0 = (longlong *******)plStack_168;
 191	        uStack_b8 = (longlong *******)plStack_170;
 192	        if (lStack_160 != 0) {
 193	          plStack_180 = plStack_170;
 194	          plStack_178 = plStack_168;
 195	          plStack_58 = plStack_168 + 4;
 196	                    /* WARNING: Could not recover jumptable at 0x00014067c9b8. Too many branches */
 197	                    /* WARNING: Treating indirect jump as call */
 198	          puVar9 = (undefined8 *)
 199	                   (*(code *)(&UNK_142a16b78 + *(int *)(&UNK_142a16b78 + *plStack_168 * 4)))
 200	                             (&UNK_142a16b78 + *(int *)(&UNK_142a16b78 + *plStack_168 * 4));
 201	          return puVar9;
 202	        }
 203	        if ((longlong)uStack_b0 - (longlong)plStack_168 != 0) {
 204	          uVar13 = (ulonglong)((longlong)uStack_b0 - (longlong)plStack_168) >> 5;
 205	          plVar12 = plStack_168 + 2;
 206	          do {
 207	            if ((plVar12[-2] != 0) && (plVar12[-1] != 0)) {
 208	              func_0x000140613c20(*plVar12,plVar12[-1],1);
 209	            }
 210	            plVar12 = plVar12 + 4;
 211	            uVar13 = uVar13 - 1;
 212	          } while (uVar13 != 0);
 213	        }
 214	        if (plVar4 != (longlong *)0x0) {
 215	          func_0x000140613c20(plStack_168,(longlong)plVar4 << 5,8);
 216	        }
 217	        uStack_e8 = (longlong *******)0x0;
 218	        uStack_e0 = (longlong *******)0x8;
 219	        ppppppplStack_d8 = (longlong *******)0x0;
 220	        uStack_c8 = (longlong *******)0x0;
 221	        uStack_c0 = (longlong *******)0x1;
 222	        uStack_b8 = (longlong *******)0x0;
 223	        FUN_140f79650(&UNK_142a16b08,1,&uStack_c8,&uStack_138,&uStack_e8);
 224	        if (((ppppppplStack_70 == (longlong *******)0x0) &&
 225	            (ppppppplStack_f8 == (longlong *******)0x0)) &&
 226	           (ppppppplStack_d8 == (longlong *******)0x0)) {
 227	          *param_1 = 0x8000000000000004;
 228	          FUN_1405f8f70(&uStack_e8);
 229	          FUN_1405f8f70(&ppppppplStack_108);
 230	          FUN_1405f8f70(&ppppppplStack_80);
 231	          uStack_49 = 0;
 232	          FUN_14004e480(&uStack_138,&ppppppplStack_118,0x18,0x10);
 233	          ppppppplVar2 = ppppppplStack_68;
 234	          ppppppplVar3 = ppppppplStack_60;
 235	        }
 236	        else {
 237	          uStack_b8 = ppppppplStack_70;
 238	          ppppppplVar2 = uStack_b8;
 239	          uStack_c8 = ppppppplStack_80;
 240	          uStack_c0 = ppppppplStack_78;
 241	          ppppppplStack_a0 = ppppppplStack_f8;
 242	          uStack_b0 = ppppppplStack_108;
 243	          ppppppplVar10 = uStack_b0;
 244	          ppppppplStack_a8 = ppppppplStack_100;
 245	          ppppppplStack_98 = uStack_e8;
 246	          ppppppplStack_90 = uStack_e0;
 247	          ppppppplStack_88 = ppppppplStack_d8;
 248	          param_1[8] = ppppppplStack_d8;
 249	          *(undefined4 *)(param_1 + 6) = (undefined4)uStack_e8;
 250	          *(undefined4 *)((longlong)param_1 + 0x34) = uStack_e8._4_4_;
 251	          *(undefined4 *)(param_1 + 7) = (undefined4)uStack_e0;
 252	          *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_e0._4_4_;
 253	          *param_1 = ppppppplStack_80;
 254	          param_1[1] = ppppppplStack_78;
 255	          param_1[4] = ppppppplStack_100;
 256	          param_1[5] = ppppppplStack_f8;
 257	          uStack_b8._0_4_ = SUB84(ppppppplStack_70,0);
 258	          uStack_b8._4_4_ = (undefined4)((ulonglong)ppppppplStack_70 >> 0x20);
 259	          uStack_b0._0_4_ = (int)ppppppplStack_108;
 260	          uStack_b0._4_4_ = (undefined4)((ulonglong)ppppppplStack_108 >> 0x20);
 261	          *(undefined4 *)(param_1 + 2) = (undefined4)uStack_b8;
 262	          *(undefined4 *)((longlong)param_1 + 0x14) = uStack_b8._4_4_;
 263	          *(int *)(param_1 + 3) = (int)uStack_b0;
 264	          *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_b0._4_4_;
 265	          uStack_49 = 0;
 266	          uStack_b8 = ppppppplVar2;
 267	          uStack_b0 = ppppppplVar10;
 268	          FUN_14004e480(&uStack_138,&ppppppplStack_118,0x18,0x10);
 269	          ppppppplVar2 = ppppppplStack_68;
 270	          ppppppplVar3 = ppppppplStack_60;
 271	        }
 272	      }
 273	      else {
 274	        *param_1 = 0x8000000000000003;
 275	        *(int *)(param_1 + 1) = iStack_120;
 276	        if (uStack_128 != 0) {
 277	          plVar12 = uStack_130 + 4;
 278	          lVar11 = uStack_128;
 279	          do {
 280	            if (plVar12[-4] != 0) {
 281	              func_0x000140613c20(plVar12[-3],plVar12[-4],1);
 282	            }
 283	            if (plVar12[-1] != 0) {
 284	              func_0x000140613c20(*plVar12,plVar12[-1],1);
 285	            }
 286	            plVar12 = plVar12 + 6;
 287	            lVar11 = lVar11 + -1;
 288	          } while (lVar11 != 0);
 289	        }
 290	        if (CONCAT44(uStack_154,uStack_158) != 0) {
 291	          func_0x000140613c20(plVar4,CONCAT44(uStack_154,uStack_158) * 0x30,8);
 292	        }
 293	      }
 294	      goto joined_r0x00014067cc49;
 295	    }
 296	    plStack_170 = uStack_130;
 297	    ppppppplStack_80 = (longlong *******)0x0;
 298	    ppppppplStack_78 = (longlong *******)0x1;
 299	    ppppppplStack_70 = (longlong *******)0x0;
 300	    uStack_c8 = (longlong *******)0x0;
 301	    uStack_b8 = (longlong *******)0x0;
 302	    ppppppplStack_a8 = (longlong *******)0x20;
 303	    ppppppplStack_a0 = (longlong *******)CONCAT71(ppppppplStack_a0._1_7_,3);
 304	    ppppppplStack_98 = (longlong *******)&ppppppplStack_80;
 305	    ppppppplStack_90 = (longlong *******)&UNK_142a169e0;
 306	    cVar6 = FUN_140b87050(&plStack_170,&uStack_c8);
 307	    if (cVar6 != '\0') {
 308	      FUN_1428d9760(&UNK_142a16a10,0x37,&uStack_4a,&UNK_142a16ab0,&UNK_142a16a98);
 309	      goto code_r0x00014067cd5d;
 310	    }
 311	    uStack_e8 = ppppppplStack_80;
 312	    uStack_e0 = ppppppplStack_78;
 313	    ppppppplStack_d8 = ppppppplStack_70;
 314	    plStack_58 = plVar4;
 315	    if (*plVar4 == 1) {
 316	      FUN_1405e7a00(plVar4 + 1);
 317	    }
 318	    else if ((*plVar4 == 0) && (plVar4[2] != 0)) {
 319	      func_0x000140613c20(plVar4[1],plVar4[2],1);
 320	    }
 321	    func_0x000140613c20(plStack_58,0x28,8);
 322	    ppppppplStack_f8 = ppppppplStack_d8;
 323	    uStack_f0 = uStack_d0;
 324	    ppppppplStack_108 = uStack_e8;
 325	    ppppppplStack_100 = uStack_e0;
 326	    param_1[3] = ppppppplStack_d8;
 327	    param_1[4] = uStack_d0;
 328	    *(undefined4 *)(param_1 + 1) = (undefined4)uStack_e8;
 329	    *(undefined4 *)((longlong)param_1 + 0xc) = uStack_e8._4_4_;
 330	    *(undefined4 *)(param_1 + 2) = (undefined4)uStack_e0;
 331	    *(undefined4 *)((longlong)param_1 + 0x14) = uStack_e0._4_4_;
 332	    uVar7 = 0x8000000000000002;
 333	  }
 334	  else {
 335	    uVar7 = 0x8000000000000000;
 336	  }
 337	  *param_1 = uVar7;
 338	  ppppppplVar2 = ppppppplStack_68;
 339	  ppppppplVar3 = ppppppplStack_60;
 340	joined_r0x00014067cc49:
 341	  if (ppppppplVar2 != (longlong *******)0x0) {
 342	    func_0x000140613c20(ppppppplVar3,ppppppplVar2,1);
 343	  }
 344	  return param_1;
 345	}
 346	
