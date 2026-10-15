   1	
   2	undefined1 * FUN_141329600(undefined1 *param_1,byte *param_2)
   3	
   4	{
   5	  uint *puVar1;
   6	  undefined8 *puVar2;
   7	  undefined4 *puVar3;
   8	  longlong *plVar4;
   9	  byte bVar5;
  10	  byte *pbVar6;
  11	  longlong lVar7;
  12	  char cVar8;
  13	  uint uVar9;
  14	  undefined8 uVar10;
  15	  longlong lVar11;
  16	  byte *pbVar12;
  17	  byte *pbVar13;
  18	  byte *pbVar14;
  19	  byte *pbVar15;
  20	  undefined1 uVar16;
  21	  longlong alStack_150 [2];
  22	  longlong lStack_140;
  23	  undefined8 uStack_138;
  24	  undefined7 uStack_130;
  25	  undefined4 uStack_129;
  26	  undefined4 uStack_125;
  27	  undefined4 uStack_121;
  28	  undefined4 uStack_11d;
  29	  longlong lStack_118;
  30	  longlong lStack_110;
  31	  undefined8 uStack_108;
  32	  undefined8 uStack_100;
  33	  undefined4 uStack_f8;
  34	  undefined4 uStack_f4;
  35	  undefined4 uStack_f0;
  36	  undefined4 uStack_ec;
  37	  undefined4 uStack_e8;
  38	  undefined4 uStack_e4;
  39	  byte *pbStack_d8;
  40	  byte *pbStack_d0;
  41	  byte *pbStack_c8;
  42	  undefined4 uStack_c0;
  43	  undefined4 uStack_bc;
  44	  undefined1 uStack_b8;
  45	  undefined7 uStack_b7;
  46	  undefined4 uStack_b0;
  47	  undefined4 uStack_ac;
  48	  undefined8 uStack_a8;
  49	  byte *pbStack_a0;
  50	  byte *pbStack_98;
  51	  byte *pbStack_88;
  52	  byte *pbStack_80;
  53	  uint uStack_78;
  54	  uint uStack_74;
  55	  uint uStack_70;
  56	  uint uStack_6c;
  57	  undefined4 uStack_68;
  58	  undefined4 uStack_64;
  59	  undefined8 uStack_60;
  60	  byte *pbStack_58;
  61	  byte *pbStack_50;
  62	  undefined1 uStack_41;
  63	  undefined8 uStack_40;
  64	  
  65	  uStack_40 = 0xfffffffffffffffe;
  66	  bVar5 = *param_2;
  67	  pbStack_58 = (byte *)CONCAT44(pbStack_58._4_4_,(uint)bVar5);
  68	  pbStack_80 = param_2;
  69	  if (bVar5 != 3) {
  70	    if (bVar5 != 5) {
  71	      pbStack_d8 = *(byte **)param_2;
  72	      pbStack_d0 = *(byte **)(param_2 + 8);
  73	      pbStack_c8 = *(byte **)(param_2 + 0x10);
  74	      uStack_c0 = *(undefined4 *)(param_2 + 0x18);
  75	      uStack_bc = *(undefined4 *)(param_2 + 0x1c);
  76	      func_0x0001428c9c10(&lStack_118,&pbStack_d8);
  77	      uVar10 = FUN_1428c9a00(&lStack_118,&UNK_142c3a348,&UNK_142c3a318);
  78	      *(undefined8 *)(param_1 + 8) = uVar10;
  79	      *param_1 = 1;
  80	      if ((byte)pbStack_d8 < 3) {
  81	        return param_1;
  82	      }
  83	      if ((byte)pbStack_d8 == 3) {
  84	        if (pbStack_d0 == (byte *)0x0) {
  85	          return param_1;
  86	        }
  87	        uVar10 = 1;
  88	        pbVar12 = pbStack_c8;
  89	        pbVar13 = pbStack_d0;
  90	      }
  91	      else {
  92	        if ((byte)pbStack_d8 != 4) {
  93	          uStack_41 = 0;
  94	          FUN_140898f40(&pbStack_d0);
  95	          return param_1;
  96	        }
  97	        pbStack_50 = pbStack_c8;
  98	        FUN_140888d70(pbStack_c8,CONCAT44(uStack_bc,uStack_c0));
  99	        if (pbStack_d0 == (byte *)0x0) {
 100	          return param_1;
 101	        }
 102	        uVar10 = 8;
 103	        pbVar12 = pbStack_50;
 104	        pbVar13 = (byte *)((longlong)pbStack_d0 << 5);
 105	      }
 106	      goto code_r0x000141329986;
 107	    }
 108	    pbVar12 = *(byte **)(param_2 + 8);
 109	    pbVar13 = *(byte **)(param_2 + 0x10);
 110	    pbVar15 = *(byte **)(param_2 + 0x18);
 111	    uStack_b8 = pbVar12 != (byte *)0x0;
 112	    pbStack_d8 = (byte *)(ulonglong)(byte)uStack_b8;
 113	    pbStack_98 = pbVar15;
 114	    if (!(bool)uStack_b8) {
 115	      pbStack_98 = (byte *)0x0;
 116	    }
 117	    pbStack_d0 = (byte *)0x0;
 118	    uStack_c0 = SUB84(pbVar13,0);
 119	    uStack_bc = (undefined4)((ulonglong)pbVar13 >> 0x20);
 120	    uStack_b7 = 0;
 121	    uStack_b0 = 0;
 122	    uStack_ac = 0;
 123	    pbStack_c8 = pbVar12;
 124	    uStack_a8 = pbVar12;
 125	    pbStack_a0 = pbVar13;
 126	    FUN_14143c450(&uStack_78,&pbStack_d8);
 127	    lVar11 = CONCAT44(uStack_74,uStack_78);
 128	    if (lVar11 != 0) {
 129	      lVar7 = CONCAT44(uStack_64,uStack_68);
 130	      pbVar6 = *(byte **)(lVar11 + 0x168 + lVar7 * 0x18);
 131	      pbVar14 = *(byte **)(lVar11 + 0x178 + lVar7 * 0x18);
 132	      puVar1 = (uint *)(lVar11 + lVar7 * 0x20);
 133	      lStack_118 = *(longlong *)puVar1;
 134	      lStack_110 = *(longlong *)(puVar1 + 2);
 135	      puVar2 = (undefined8 *)(lVar11 + 0x10 + lVar7 * 0x20);
 136	      uStack_108 = *puVar2;
 137	      uStack_100 = puVar2[1];
 138	      if (!SBORROW8(0,(longlong)pbVar6)) {
 139	        uStack_108._4_4_ = (uint)((ulonglong)uStack_108 >> 0x20);
 140	        uStack_68 = (uint)uStack_108;
 141	        uStack_64 = uStack_108._4_4_;
 142	        pbStack_88 = *(byte **)(lVar11 + 0x170 + lVar7 * 0x18);
 143	        uStack_78 = *puVar1;
 144	        uStack_74 = puVar1[1];
 145	        uStack_70 = puVar1[2];
 146	        uStack_6c = puVar1[3];
 147	        uStack_60 = uStack_100;
 148	        pbStack_50 = pbVar6;
 149	        FUN_14143c450(alStack_150,&pbStack_d8);
 150	        if (alStack_150[0] != 0) {
 151	          uStack_108 = *(undefined8 *)(alStack_150[0] + 0x178 + lStack_140 * 0x18);
 152	          plVar4 = (longlong *)(alStack_150[0] + 0x168 + lStack_140 * 0x18);
 153	          lStack_118 = *plVar4;
 154	          lStack_110 = plVar4[1];
 155	          puVar2 = (undefined8 *)(alStack_150[0] + lStack_140 * 0x20);
 156	          uStack_100 = *puVar2;
 157	          uStack_f8 = *(undefined4 *)(puVar2 + 1);
 158	          uStack_f4 = *(undefined4 *)((longlong)puVar2 + 0xc);
 159	          puVar3 = (undefined4 *)(alStack_150[0] + 0x10 + lStack_140 * 0x20);
 160	          uStack_f0 = *puVar3;
 161	          uStack_ec = puVar3[1];
 162	          uStack_e8 = puVar3[2];
 163	          uStack_e4 = puVar3[3];
 164	          if (!SBORROW8(0,lStack_118)) {
 165	            if (lStack_118 != 0) {
 166	              func_0x000140613c20(lStack_110,lStack_118,1);
 167	            }
 168	            if (2 < (byte)uStack_100) {
 169	              if ((byte)uStack_100 == 3) {
 170	                if (CONCAT44(uStack_f4,uStack_f8) != 0) {
 171	                  func_0x000140613c20(CONCAT44(uStack_ec,uStack_f0),CONCAT44(uStack_f4,uStack_f8),1)
 172	                  ;
 173	                }
 174	              }
 175	              else if ((byte)uStack_100 == 4) {
 176	                FUN_140889390();
 177	              }
 178	              else {
 179	                FUN_140898f40(&uStack_f8);
 180	              }
 181	            }
 182	            lStack_118 = CONCAT71(lStack_118._1_7_,0xb);
 183	            uVar10 = FUN_1428f9b40(&lStack_118,&UNK_142c3a308,&UNK_142c3a318);
 184	            *(undefined8 *)(param_1 + 8) = uVar10;
 185	            *param_1 = 1;
 186	            uVar9 = uStack_78 & 0xff;
 187	            if (2 < uVar9) {
 188	              if (uVar9 == 3) {
 189	                if (CONCAT44(uStack_6c,uStack_70) != 0) {
 190	                  func_0x000140613c20(CONCAT44(uStack_64,uStack_68),CONCAT44(uStack_6c,uStack_70),1)
 191	                  ;
 192	                }
 193	              }
 194	              else if (uVar9 == 4) {
 195	                FUN_140889390();
 196	              }
 197	              else {
 198	                FUN_140898f40(&uStack_70);
 199	              }
 200	            }
 201	            if (pbStack_50 != (byte *)0x0) {
 202	              func_0x000140613c20(pbStack_88,pbStack_50,1);
 203	            }
 204	            goto code_r0x000141329715;
 205	          }
 206	        }
 207	        uVar16 = (undefined1)uStack_78;
 208	        uStack_138 = CONCAT17((undefined1)uStack_70,CONCAT43(uStack_74,uStack_78._1_3_));
 209	        uStack_130 = CONCAT43(uStack_6c,uStack_70._1_3_);
 210	        uStack_129 = uStack_68;
 211	        uStack_125 = uStack_64;
 212	        uStack_121 = (undefined4)uStack_60;
 213	        uStack_11d = uStack_60._4_4_;
 214	        uStack_41 = 1;
 215	        FUN_140887df0(&pbStack_d8);
 216	        pbStack_d8 = pbStack_50;
 217	        pbStack_d0 = pbStack_88;
 218	        goto code_r0x000141329742;
 219	      }
 220	    }
 221	    uStack_78 = CONCAT31(uStack_78._1_3_,0xb);
 222	    uVar10 = FUN_1428f9b40(&uStack_78,&UNK_142c3a308,&UNK_142c3a318);
 223	    *(undefined8 *)(param_1 + 8) = uVar10;
 224	    *param_1 = 1;
 225	code_r0x000141329715:
 226	    uStack_41 = 1;
 227	    FUN_140887df0(&pbStack_d8);
 228	    return param_1;
 229	  }
 230	  pbStack_d8 = *(byte **)(param_2 + 8);
 231	  pbStack_d0 = *(byte **)(param_2 + 0x10);
 232	  pbVar14 = *(byte **)(param_2 + 0x18);
 233	  uVar16 = 6;
 234	  pbVar15 = pbVar14;
 235	  pbVar13 = pbStack_d0;
 236	  pbVar12 = pbStack_d8;
 237	code_r0x000141329742:
 238	  uStack_bc = (undefined4)((ulonglong)uStack_138 >> 0x18);
 239	  uStack_b8 = (undefined1)((ulonglong)uStack_138 >> 0x38);
 240	  uStack_b7 = uStack_130;
 241	  uStack_b0 = uStack_129;
 242	  uStack_ac = uStack_125;
 243	  uStack_a8 = (byte *)CONCAT44(uStack_11d,uStack_121);
 244	  uStack_c0 = CONCAT31((int3)uStack_138,uVar16);
 245	  uStack_41 = 1;
 246	  pbStack_c8 = pbVar14;
 247	  FUN_1415d2b10(&lStack_118,&pbStack_d8);
 248	  cVar8 = (char)lStack_118;
 249	  lVar11 = lStack_110;
 250	  if ((char)lStack_118 == '\x03') {
 251	code_r0x0001413297bc:
 252	    *(longlong *)(param_1 + 8) = lVar11;
 253	    *param_1 = 1;
 254	  }
 255	  else {
 256	    uStack_60 = CONCAT44(uStack_f4,uStack_f8);
 257	    uStack_70 = (uint)uStack_108;
 258	    uStack_6c = uStack_108._4_4_;
 259	    uStack_68 = (undefined4)uStack_100;
 260	    uStack_64 = uStack_100._4_4_;
 261	    uStack_78 = (uint)lStack_110;
 262	    uStack_74 = (uint)((ulonglong)lStack_110 >> 0x20);
 263	    uStack_41 = 1;
 264	    lVar11 = FUN_140b88000(&uStack_78);
 265	    if (lVar11 != 0) goto code_r0x0001413297bc;
 266	    param_1[1] = cVar8;
 267	    *param_1 = 0;
 268	  }
 269	  if ((int)pbStack_58 == 3) {
 270	    return param_1;
 271	  }
 272	  if ((int)pbStack_58 == 5) {
 273	    return param_1;
 274	  }
 275	  pbStack_80 = pbVar13;
 276	  pbStack_58 = pbVar12;
 277	  FUN_140888d70(pbVar13,pbVar15);
 278	  if (pbStack_58 == (byte *)0x0) {
 279	    return param_1;
 280	  }
 281	  uVar10 = 8;
 282	  pbVar12 = pbStack_80;
 283	  pbVar13 = (byte *)((longlong)pbStack_58 << 5);
 284	code_r0x000141329986:
 285	  func_0x000140613c20(pbVar12,pbVar13,uVar10);
 286	  return param_1;
 287	}
 288	