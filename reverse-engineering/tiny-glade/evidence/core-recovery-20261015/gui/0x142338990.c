   1	
   2	void FUN_142338990(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4,
   3	                  longlong *param_5,undefined8 *param_6,undefined8 *param_7,undefined8 *param_8,
   4	                  longlong *param_9,undefined8 *param_10,longlong *param_11,longlong param_12,
   5	                  undefined8 *param_13)
   6	
   7	{
   8	  byte *pbVar1;
   9	  undefined8 uVar2;
  10	  undefined8 uVar3;
  11	  byte bVar4;
  12	  byte bVar5;
  13	  undefined4 uVar6;
  14	  undefined4 *puVar7;
  15	  ulonglong uVar8;
  16	  code *pcVar9;
  17	  int iVar10;
  18	  int iVar11;
  19	  int iVar12;
  20	  char cVar13;
  21	  undefined1 uVar14;
  22	  byte bVar15;
  23	  uint uVar16;
  24	  longlong lVar17;
  25	  longlong *plVar18;
  26	  longlong lVar19;
  27	  longlong lVar20;
  28	  ulonglong uVar21;
  29	  longlong *plVar22;
  30	  undefined *puVar23;
  31	  uint *puVar24;
  32	  ulonglong uVar25;
  33	  undefined *puVar26;
  34	  undefined1 *puVar27;
  35	  undefined *puVar28;
  36	  undefined8 *puVar29;
  37	  float fVar30;
  38	  float fVar31;
  39	  float fVar32;
  40	  float fVar33;
  41	  undefined8 in_stack_fffffffffffffbb8;
  42	  undefined4 uVar34;
  43	  undefined8 uStack_428;
  44	  undefined8 uStack_420;
  45	  float fStack_418;
  46	  float fStack_414;
  47	  undefined4 uStack_410;
  48	  undefined4 uStack_40c;
  49	  uint *puStack_408;
  50	  ulonglong uStack_370;
  51	  undefined8 *puStack_368;
  52	  undefined8 uStack_310;
  53	  undefined8 uStack_308;
  54	  undefined8 uStack_300;
  55	  undefined *puStack_2f8;
  56	  undefined8 uStack_2f0;
  57	  undefined8 uStack_2e8;
  58	  undefined8 uStack_2e0;
  59	  uint *puStack_2d8;
  60	  uint *puStack_2d0;
  61	  longlong lStack_2c8;
  62	  longlong lStack_2b8;
  63	  longlong *plStack_2a8;
  64	  longlong *plStack_2a0;
  65	  longlong lStack_298;
  66	  byte bStack_290;
  67	  longlong lStack_288;
  68	  longlong lStack_280;
  69	  longlong lStack_278;
  70	  longlong lStack_270;
  71	  longlong lStack_260;
  72	  undefined4 uStack_254;
  73	  byte bStack_248;
  74	  byte bStack_240;
  75	  undefined8 uStack_238;
  76	  undefined8 uStack_230;
  77	  undefined8 uStack_228;
  78	  float *pfStack_208;
  79	  undefined8 uStack_200;
  80	  undefined4 *puStack_1f8;
  81	  undefined4 *puStack_1f0;
  82	  int *piStack_1e8;
  83	  undefined4 *puStack_1e0;
  84	  longlong lStack_1d8;
  85	  longlong lStack_1d0;
  86	  longlong lStack_1c8;
  87	  undefined8 *puStack_1c0;
  88	  undefined8 uStack_1b8;
  89	  undefined4 *puStack_1b0;
  90	  longlong lStack_1a8;
  91	  undefined8 uStack_1a0;
  92	  longlong lStack_198;
  93	  undefined4 *puStack_190;
  94	  longlong lStack_188;
  95	  longlong lStack_180;
  96	  uint *puStack_178;
  97	  longlong lStack_170;
  98	  longlong lStack_168;
  99	  undefined4 uStack_160;
 100	  undefined4 uStack_15c;
 101	  undefined4 uStack_158;
 102	  undefined4 uStack_154;
 103	  char *pcStack_150;
 104	  longlong lStack_148;
 105	  longlong lStack_140;
 106	  int *piStack_138;
 107	  int *piStack_130;
 108	  longlong lStack_128;
 109	  longlong lStack_120;
 110	  longlong *plStack_118;
 111	  longlong *plStack_110;
 112	  longlong lStack_108;
 113	  undefined4 uStack_fc;
 114	  longlong lStack_f8;
 115	  longlong lStack_f0;
 116	  longlong lStack_e8;
 117	  uint uStack_dc;
 118	  byte bStack_d6;
 119	  char cStack_d5;
 120	  uint uStack_d4;
 121	  longlong *plStack_d0;
 122	  undefined1 uStack_c2;
 123	  undefined1 uStack_c1;
 124	  undefined8 uStack_c0;
 125	  
 126	  uVar34 = (undefined4)((ulonglong)in_stack_fffffffffffffbb8 >> 0x20);
 127	  uStack_c0 = 0xfffffffffffffffe;
 128	  cVar13 = func_0x000140c16a00();
 129	  plVar22 = param_4;
 130	  if (cVar13 == '\0') {
 131	code_r0x000142338a67:
 132	    lStack_1a8 = 0;
 133	  }
 134	  else {
 135	    puVar23 = &UNK_14304da07;
 136	    plVar22 = (longlong *)0x33;
 137	    uVar21 = 0x33;
 138	    plStack_d0 = param_4;
 139	    lVar17 = FUN_142338130(&UNK_14304da07);
 140	    if (lVar17 == 1) {
 141	      if (uVar21 != 0) {
 142	        if (uVar21 < 0x33) {
 143	          if ((char)(&UNK_14304da07)[uVar21] < -0x40) {
 144	code_r0x000142338a47:
 145	            uVar34 = 1;
 146	            FUN_1428d9940(&UNK_14304da07,0x33,0,uVar21,&UNK_14304dbc8);
 147	            goto code_r0x000142338a67;
 148	          }
 149	        }
 150	        else if (uVar21 != 0x33) goto code_r0x000142338a47;
 151	      }
 152	      puVar23 = &UNK_14304da07;
 153	      lVar17 = FUN_142338130(&UNK_14304da07);
 154	      if (lVar17 == 1) {
 155	        uVar25 = uVar21 + 2;
 156	        if (uVar25 != 0) {
 157	          uVar14 = uVar25 == 0x33;
 158	          if (0x32 < uVar25) goto code_r0x000142338ac8;
 159	          uVar14 = (&UNK_14304da09)[uVar21] == -0x41;
 160	          if ((char)(&UNK_14304da09)[uVar21] < -0x40) {