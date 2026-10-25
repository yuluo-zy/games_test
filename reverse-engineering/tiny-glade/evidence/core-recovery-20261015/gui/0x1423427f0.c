   1	
   2	void FUN_1423427f0(longlong *param_1,undefined8 *param_2,longlong *param_3,undefined8 *param_4,
   3	                  longlong *param_5,longlong param_6)
   4	
   5	{
   6	  uint *puVar1;
   7	  undefined1 uVar2;
   8	  char cVar3;
   9	  char cVar4;
  10	  undefined4 uVar5;
  11	  ulonglong *puVar6;
  12	  longlong lVar7;
  13	  longlong lVar8;
  14	  undefined8 uVar9;
  15	  longlong lVar10;
  16	  longlong lVar11;
  17	  longlong lVar12;
  18	  longlong lVar13;
  19	  ulonglong uVar14;
  20	  ulonglong uVar15;
  21	  longlong lVar16;
  22	  longlong lVar17;
  23	  longlong lVar18;
  24	  ulonglong uVar19;
  25	  ulonglong uVar20;
  26	  longlong lVar21;
  27	  longlong lVar22;
  28	  uint *puVar23;
  29	  bool bVar24;
  30	  bool bVar25;
  31	  bool bVar26;
  32	  bool bVar27;
  33	  longlong *plStack_a8;
  34	  undefined4 *puStack_70;
  35	  
  36	  puVar6 = (ulonglong *)*param_2;
  37	  lVar7 = param_2[1];
  38	  uVar14 = *puVar6;
  39	  uVar15 = 0;
  40	  uVar19 = uVar14 - *(ulonglong *)(lVar7 + 0x18);
  41	  if (uVar14 < *(ulonglong *)(lVar7 + 0x18)) {
  42	    uVar19 = uVar15;
  43	  }
  44	  uVar20 = uVar14 - *(ulonglong *)(lVar7 + 0x38);
  45	  if (uVar14 < *(ulonglong *)(lVar7 + 0x38)) {
  46	    uVar20 = uVar15;
  47	  }
  48	  lVar16 = uVar19 * 0x10 + *(longlong *)(lVar7 + 8);
  49	  uVar14 = *(ulonglong *)(lVar7 + 0x10) - uVar19;
  50	  if (*(ulonglong *)(lVar7 + 0x10) < uVar19) {
  51	    lVar16 = 8;
  52	    uVar14 = uVar15;
  53	  }
  54	  lVar12 = uVar20 * 0x10 + *(longlong *)(lVar7 + 0x28);
  55	  uVar19 = *(ulonglong *)(lVar7 + 0x30) - uVar20;
  56	  if (*(ulonglong *)(lVar7 + 0x30) < uVar20) {
  57	    lVar12 = 8;
  58	    uVar19 = uVar15;
  59	  }
  60	  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar19) - uVar14;
  61	  lVar21 = uVar14 * 0x10 + lVar16;
  62	  lVar22 = uVar19 * 0x10 + lVar12;
  63	  lVar7 = *param_1;
  64	  lVar8 = param_1[1];
  65	  uVar5 = *(undefined4 *)((longlong)param_3 + 0x1c);
  66	  lVar18 = *param_3;
  67	  puStack_70 = (undefined4 *)param_3[2];
  68	  do {
  69	    if ((lVar16 == 0) || (lVar16 == lVar21)) {
  70	      if ((lVar12 == 0) || (lVar12 == lVar22)) break;
  71	      lVar16 = 0;
  72	      lVar13 = lVar12 + 0x10;
  73	      lVar17 = lVar12;
  74	    }
  75	    else {
  76	      lVar13 = lVar12;
  77	      lVar17 = lVar16;
  78	      lVar16 = lVar16 + 0x10;
  79	    }
  80	    lVar12 = lVar13;
  81	    *puVar6 = *puVar6 + 1;
  82	    uVar14 = *(ulonglong *)(lVar17 + 8) & 0xffffffff;
  83	    if ((uVar14 < *(ulonglong *)(lVar8 + 0x10)) &&
  84	       (*(int *)(*(longlong *)(lVar8 + 8) + uVar14 * 0x14) ==
  85	        (int)(*(ulonglong *)(lVar17 + 8) >> 0x20))) {
  86	      lVar13 = *(longlong *)(lVar8 + 8) + uVar14 * 0x14;
  87	      uVar14 = (ulonglong)*(uint *)(lVar13 + 4);
  88	      if ((uVar14 != 0xffffffff) &&
  89	         ((uVar14 < *(ulonglong *)(lVar7 + 0x38) &&
  90	          ((*(ulonglong *)
  91	             (*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar13 + 4) >> 6) * 8) >>
  92	            (uVar14 & 0x3f) & 1) != 0)))) {
  93	        uVar2 = *(undefined1 *)
  94	                 (*(longlong *)
  95	                   (*(longlong *)
  96	                     (*(longlong *)(lVar8 + 0x1a8) + 0x18 +
  97	                     (ulonglong)*(uint *)(lVar13 + 0xc) * 0x48) + 0x10 +
  98	                   ~*(ulonglong *)
  99	                     (*(longlong *)
 100	                       (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
 101	                       (ulonglong)*(uint *)(lVar13 + 0xc) * 0x48) + *(longlong *)(lVar7 + 0x110) * 8
 102	                     ) * 0x30) + (ulonglong)*(uint *)(lVar13 + 0x10));
 103	        *puStack_70 = uVar5;
 104	        lVar13 = *(longlong *)(lVar18 + 0x30);
 105	        uVar9 = *(undefined8 *)(lVar18 + 0x40);
 106	        if (lVar13 == *(longlong *)(lVar18 + 0x20)) {
 107	          FUN_140562630(lVar18 + 0x20,&UNK_142bb5270);
 108	        }
 109	        lVar17 = *(longlong *)(lVar18 + 0x28);
 110	        *(undefined8 *)(lVar17 + lVar13 * 0x10) = uVar9;
 111	        *(undefined1 *)(lVar17 + 8 + lVar13 * 0x10) = uVar2;
 112	        *(longlong *)(lVar18 + 0x30) = lVar13 + 1;
 113	        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
 114	      }
 115	    }
 116	  } while( true );
 117	  lVar16 = param_4[1];
 118	  uVar14 = *(ulonglong *)*param_4;
 119	  uVar15 = 0;
 120	  uVar19 = uVar14 - *(ulonglong *)(lVar16 + 0x18);
 121	  if (uVar14 < *(ulonglong *)(lVar16 + 0x18)) {
 122	    uVar19 = uVar15;
 123	  }
 124	  uVar20 = uVar14 - *(ulonglong *)(lVar16 + 0x38);
 125	  if (uVar14 < *(ulonglong *)(lVar16 + 0x38)) {
 126	    uVar20 = uVar15;
 127	  }
 128	  bVar24 = *(ulonglong *)(lVar16 + 0x30) < uVar20;
 129	  uVar20 = *(ulonglong *)(lVar16 + 0x30) - uVar20;
 130	  bVar26 = uVar20 != 0;
 131	  if (bVar24) {
 132	    uVar20 = uVar15;
 133	  }
 134	  bVar25 = *(ulonglong *)(lVar16 + 0x10) < uVar19;
 135	  uVar19 = *(ulonglong *)(lVar16 + 0x10) - uVar19;
 136	  bVar27 = uVar19 != 0;
 137	  if (bVar25) {
 138	    uVar19 = uVar15;
 139	  }
 140	  uVar14 = uVar19 + uVar20;
 141	  if (!bVar25 && bVar27 || !bVar24 && bVar26) {
 142	    uVar14 = uVar15;
 143	  }
 144	  *(ulonglong *)*param_4 = *(longlong *)(lVar16 + 0x40) - uVar14;
 145	  puVar23 = *(uint **)(lVar7 + 0xf8);
 146	  plStack_a8 = *(longlong **)(param_6 + 8);
 147	  if (plStack_a8 == (longlong *)0x0) {
 148	    plStack_a8 = *(longlong **)(param_6 + 0x10);
 149	  }
 150	  uVar5 = *(undefined4 *)((longlong)param_1 + 0x14);
 151	  puVar1 = puVar23 + *(longlong *)(lVar7 + 0x100);
 152	  lVar16 = *param_5;
 153	  if (*(char *)(lVar7 + 0x108) == '\0') {
 154	    lVar22 = 8;
 155	    lVar21 = 0;
 156	    lVar18 = 0;
 157	    lVar12 = 0;
 158	    lVar13 = 0;
 159	    do {
 160	      if (lVar13 == lVar12) {
 161	        if (puVar23 == puVar1) {
 162	          return;
 163	        }
 164	        while( true ) {
 165	          lVar12 = *(longlong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar23 * 0xa0);
 166	          if (lVar12 != 0) break;
 167	          puVar23 = puVar23 + 1;
 168	          if (puVar23 == puVar1) {
 169	            return;
 170	          }
 171	        }
 172	        lVar13 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar23 * 0xa0;
 173	        uVar14 = (ulonglong)*(uint *)(lVar13 + 0x94);
 174	        lVar21 = *(longlong *)(lVar8 + 0x1a8);
 175	        lVar18 = lVar21 + uVar14 * 0x48;
 176	        lVar22 = *(longlong *)(lVar21 + 0x38 + uVar14 * 0x48);
 177	        if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar21 + 0x40 + uVar14 * 0x48)) {
 178	          uVar14 = *(ulonglong *)(lVar22 + *(ulonglong *)(lVar7 + 0x110) * 8);
 179	          lVar17 = *(longlong *)(lVar18 + 0x18);
 180	          if (uVar14 == 0) {
 181	            lVar18 = 0;
 182	          }
 183	          else {
 184	            lVar18 = *(longlong *)(lVar17 + 0x10 + ~uVar14 * 0x30);
 185	          }
 186	        }
 187	        else {
 188	          lVar17 = *(longlong *)(lVar18 + 0x18);
 189	          lVar18 = 0;
 190	        }
 191	        puVar23 = puVar23 + 1;
 192	        lVar22 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
 193	        lVar21 = *(longlong *)(lVar17 + 0x10 + lVar22);
 194	        puStack_70 = *(undefined4 **)(lVar17 + 0x28 + lVar22);
 195	        lVar22 = *(longlong *)(lVar13 + 0x50);
 196	        lVar13 = 0;
 197	      }
 198	      uVar14 = (ulonglong)*(uint *)(lVar22 + 8 + lVar13 * 0x10);
 199	      lVar17 = uVar14 * 3 + lVar21;
 200	      if (((!bVar25 && bVar27 || !bVar24 && bVar26) && (*(char *)(lVar17 + 2) != '\x01')) &&
 201	         (*(char *)(lVar16 + 0x18) == *(char *)(lVar18 + uVar14))) {
 202	        uVar9 = *(undefined8 *)(lVar22 + lVar13 * 0x10);
 203	        lVar11 = plStack_a8[2];
 204	        if ((ulonglong)(*plStack_a8 - lVar11) < 0x28) {
 205	          FUN_1428a58d0(plStack_a8,lVar11,0x28,1,1);
 206	        }
 207	        lVar10 = plStack_a8[1];
 208	        *(code **)(lVar10 + lVar11) = FUN_140fdf200;
 209	        *(code **)(lVar10 + 8 + lVar11) = FUN_141e4f860;
 210	        *(undefined8 *)(lVar10 + 0x10 + lVar11) = uVar9;
 211	        *(undefined8 *)(lVar10 + 0x18 + lVar11) = 0x3e4ccccd3dcccccd;
 212	        *(undefined4 *)(lVar10 + 0x20 + lVar11) = 0;
 213	        *(undefined1 *)(lVar10 + 0x24 + lVar11) = 0;
 214	        plStack_a8[2] = lVar11 + 0x28;
 215	      }
 216	      lVar13 = lVar13 + 1;
 217	      cVar3 = *(char *)(lVar16 + 0x18);
 218	      cVar4 = *(char *)(lVar18 + uVar14);
 219	      puStack_70[uVar14] = uVar5;
 220	      *(bool *)(lVar17 + 2) = cVar3 == cVar4;
 221	    } while( true );
 222	  }
 223	  lVar21 = 8;
 224	  lVar18 = 0;
 225	  lVar12 = 0;
 226	  uVar14 = 0;
 227	  uVar19 = 0;
 228	  do {
 229	    if (uVar19 == uVar14) {
 230	      if (puVar23 == puVar1) {
 231	        return;
 232	      }
 233	      while( true ) {
 234	        uVar14 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar23 * 0x48);
 235	        if (uVar14 != 0) break;
 236	        puVar23 = puVar23 + 1;
 237	        if (puVar23 == puVar1) {
 238	          return;
 239	        }
 240	      }
 241	      lVar21 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar23 * 0x48;
 242	      if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar21 + 0x40)) {
 243	        uVar19 = *(ulonglong *)(*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
 244	        lVar22 = *(longlong *)(lVar21 + 0x18);
 245	        if (uVar19 == 0) goto code_r0x000142342c1b;
 246	        lVar12 = *(longlong *)(lVar22 + 0x10 + ~uVar19 * 0x30);
 247	      }
 248	      else {
 249	        lVar22 = *(longlong *)(lVar21 + 0x18);
 250	code_r0x000142342c1b:
 251	        lVar12 = 0;
 252	      }
 253	      puVar23 = puVar23 + 1;
 254	      lVar13 = ~*(ulonglong *)(*(longlong *)(lVar21 + 0x38) + *(longlong *)(lVar7 + 0x118) * 8) *
 255	               0x30;
 256	      lVar18 = *(longlong *)(lVar22 + 0x10 + lVar13);
 257	      puStack_70 = *(undefined4 **)(lVar22 + 0x28 + lVar13);
 258	      lVar21 = *(longlong *)(lVar21 + 8);
 259	      uVar19 = 0;
 260	    }
 261	    uVar15 = uVar19 & 0xffffffff;
 262	    lVar22 = uVar15 * 3 + lVar18;
 263	    if (((!bVar25 && bVar27 || !bVar24 && bVar26) && (*(char *)(lVar22 + 2) == '\0')) &&
 264	       (*(char *)(lVar16 + 0x18) == *(char *)(lVar12 + uVar15))) {
 265	      uVar9 = *(undefined8 *)(lVar21 + uVar19 * 8);
 266	      lVar13 = plStack_a8[2];
 267	      if ((ulonglong)(*plStack_a8 - lVar13) < 0x28) {
 268	        FUN_1428a58d0(plStack_a8,lVar13,0x28,1,1);
 269	      }
 270	      lVar17 = plStack_a8[1];
 271	      *(code **)(lVar17 + lVar13) = FUN_140fdf200;
 272	      *(code **)(lVar17 + 8 + lVar13) = FUN_141e4f860;
 273	      *(undefined8 *)(lVar17 + 0x10 + lVar13) = uVar9;
 274	      *(undefined8 *)(lVar17 + 0x18 + lVar13) = 0x3e4ccccd3dcccccd;
 275	      *(undefined4 *)(lVar17 + 0x20 + lVar13) = 0;
 276	      *(undefined1 *)(lVar17 + 0x24 + lVar13) = 0;
 277	      plStack_a8[2] = lVar13 + 0x28;
 278	    }
 279	    uVar19 = uVar19 + 1;
 280	    cVar3 = *(char *)(lVar16 + 0x18);
 281	    cVar4 = *(char *)(lVar12 + uVar15);
 282	    puStack_70[uVar15] = uVar5;
 283	    *(bool *)(lVar22 + 2) = cVar3 == cVar4;
 284	  } while( true );
 285	}
 286	