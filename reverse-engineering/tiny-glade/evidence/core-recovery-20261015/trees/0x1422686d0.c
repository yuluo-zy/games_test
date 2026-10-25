   1	
   2	void FUN_1422686d0(longlong *param_1,longlong *param_2)
   3	
   4	{
   5	  uint *puVar1;
   6	  float *pfVar2;
   7	  undefined4 uVar3;
   8	  uint uVar4;
   9	  longlong lVar5;
  10	  longlong lVar6;
  11	  longlong lVar7;
  12	  undefined4 *puVar8;
  13	  ulonglong uVar9;
  14	  undefined8 uVar10;
  15	  undefined8 uVar11;
  16	  longlong lVar12;
  17	  ulonglong uVar13;
  18	  longlong lVar14;
  19	  ulonglong uVar15;
  20	  longlong lVar16;
  21	  longlong lVar17;
  22	  longlong lVar18;
  23	  longlong lVar19;
  24	  uint *puVar20;
  25	  
  26	  lVar5 = *param_1;
  27	  lVar6 = param_1[1];
  28	  puVar20 = *(uint **)(lVar5 + 0xf8);
  29	  puVar1 = puVar20 + *(longlong *)(lVar5 + 0x100);
  30	  uVar3 = *(undefined4 *)((longlong)param_2 + 0x1c);
  31	  lVar7 = *param_2;
  32	  puVar8 = (undefined4 *)param_2[2];
  33	  if (*(char *)(lVar5 + 0x108) == '\0') {
  34	    lVar19 = 8;
  35	    lVar18 = 0;
  36	    lVar17 = 0;
  37	    lVar16 = 0;
  38	    do {
  39	      if (lVar16 == lVar17) {
  40	        do {
  41	          if (puVar20 == puVar1) {
  42	            return;
  43	          }
  44	          uVar4 = *puVar20;
  45	          puVar20 = puVar20 + 1;
  46	          lVar17 = *(longlong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)uVar4 * 0xa0);
  47	        } while (lVar17 == 0);
  48	        lVar19 = *(longlong *)(lVar6 + 0x100) + (ulonglong)uVar4 * 0xa0;
  49	        uVar15 = (ulonglong)*(uint *)(lVar19 + 0x94);
  50	        lVar18 = *(longlong *)(lVar6 + 0x1a8);
  51	        if (*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar18 + 0x40 + uVar15 * 0x48)) {
  52	          uVar9 = *(ulonglong *)
  53	                   (*(longlong *)(lVar18 + 0x38 + uVar15 * 0x48) + *(ulonglong *)(lVar5 + 0x110) * 8
  54	                   );
  55	          if (uVar9 == 0) {
  56	            lVar18 = 0;
  57	          }
  58	          else {
  59	            lVar18 = *(longlong *)
  60	                      (*(longlong *)(lVar18 + uVar15 * 0x48 + 0x18) + 0x10 + ~uVar9 * 0x30);
  61	          }
  62	        }
  63	        else {
  64	          lVar18 = 0;
  65	        }
  66	        lVar19 = *(longlong *)(lVar19 + 0x50);
  67	        lVar16 = 0;
  68	      }
  69	      lVar14 = lVar16 * 0x10;
  70	      lVar16 = lVar16 + 1;
  71	      pfVar2 = (float *)(lVar18 + 4 + (ulonglong)*(uint *)(lVar19 + 8 + lVar14) * 0x10);
  72	      if (*pfVar2 <= 0.01 && *pfVar2 != 0.01) {
  73	        uVar10 = *(undefined8 *)(lVar19 + lVar14);
  74	        *puVar8 = uVar3;
  75	        lVar14 = *(longlong *)(lVar7 + 0x30);
  76	        uVar11 = *(undefined8 *)(lVar7 + 0x40);
  77	        if (lVar14 == *(longlong *)(lVar7 + 0x20)) {
  78	          FUN_140562630(lVar7 + 0x20,&UNK_143034d10);
  79	        }
  80	        lVar12 = *(longlong *)(lVar7 + 0x28);
  81	        *(undefined8 *)(lVar12 + lVar14 * 0x10) = uVar11;
  82	        *(undefined8 *)(lVar12 + 8 + lVar14 * 0x10) = uVar10;
  83	        *(longlong *)(lVar7 + 0x30) = lVar14 + 1;
  84	        *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
  85	      }
  86	    } while( true );
  87	  }
  88	  lVar18 = 8;
  89	  lVar17 = 0;
  90	  uVar15 = 0;
  91	  uVar9 = 0;
  92	  do {
  93	    uVar13 = uVar9;
  94	    if (uVar13 == uVar15) {
  95	      do {
  96	        if (puVar20 == puVar1) {
  97	          return;
  98	        }
  99	        uVar4 = *puVar20;
 100	        puVar20 = puVar20 + 1;
 101	        uVar15 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)uVar4 * 0x48);
 102	      } while (uVar15 == 0);
 103	      lVar18 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)uVar4 * 0x48;
 104	      if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar18 + 0x40)) &&
 105	         (uVar9 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8),
 106	         uVar9 != 0)) {
 107	        lVar17 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar9 * 0x30);
 108	      }
 109	      else {
 110	        lVar17 = 0;
 111	      }
 112	      lVar18 = *(longlong *)(lVar18 + 8);
 113	      uVar13 = 0;
 114	    }
 115	    pfVar2 = (float *)(lVar17 + 4 + (uVar13 & 0xffffffff) * 0x10);
 116	    uVar9 = uVar13 + 1;
 117	    if (*pfVar2 <= 0.01 && *pfVar2 != 0.01) {
 118	      uVar10 = *(undefined8 *)(lVar18 + uVar13 * 8);
 119	      *puVar8 = uVar3;
 120	      lVar19 = *(longlong *)(lVar7 + 0x30);
 121	      uVar11 = *(undefined8 *)(lVar7 + 0x40);
 122	      if (lVar19 == *(longlong *)(lVar7 + 0x20)) {
 123	        FUN_140562630(lVar7 + 0x20,&UNK_143034d10);
 124	      }
 125	      lVar16 = *(longlong *)(lVar7 + 0x28);
 126	      *(undefined8 *)(lVar16 + lVar19 * 0x10) = uVar11;
 127	      *(undefined8 *)(lVar16 + 8 + lVar19 * 0x10) = uVar10;
 128	      *(longlong *)(lVar7 + 0x30) = lVar19 + 1;
 129	      *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
 130	    }
 131	  } while( true );
 132	}
 133	