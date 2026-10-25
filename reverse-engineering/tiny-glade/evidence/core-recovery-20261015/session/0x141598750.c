   1	
   2	undefined8 FUN_141598750(longlong *param_1,undefined8 param_2,longlong param_3,longlong *param_4)
   3	
   4	{
   5	  longlong lVar1;
   6	  code *pcVar2;
   7	  longlong lVar3;
   8	  undefined8 uVar4;
   9	  char acStack_c8 [9];
  10	  undefined4 uStack_bf;
  11	  undefined2 uStack_bb;
  12	  undefined1 uStack_b9;
  13	  undefined4 uStack_b8;
  14	  undefined4 uStack_b4;
  15	  undefined4 uStack_b0;
  16	  undefined4 uStack_ac;
  17	  longlong lStack_a0;
  18	  longlong lStack_98;
  19	  longlong lStack_90;
  20	  undefined1 auStack_88 [8];
  21	  longlong lStack_80;
  22	  longlong lStack_78;
  23	  undefined8 uStack_70;
  24	  char acStack_68 [8];
  25	  undefined1 uStack_60;
  26	  undefined4 uStack_5f;
  27	  undefined2 uStack_5b;
  28	  undefined1 uStack_59;
  29	  undefined4 uStack_58;
  30	  undefined4 uStack_54;
  31	  undefined4 uStack_50;
  32	  undefined4 uStack_4c;
  33	  longlong lStack_48;
  34	  undefined8 uStack_40;
  35	  
  36	  uStack_40 = 0xfffffffffffffffe;
  37	  if (param_3 < 0) {
  38	    uVar4 = 0;
  39	code_r0x000141598926:
  40	    FUN_1428d8fe3(uVar4,param_3,&UNK_142c781d8);
  41	    pcVar2 = (code *)swi(3);
  42	    uVar4 = (*pcVar2)();
  43	    return uVar4;
  44	  }
  45	  if (param_3 == 0) {
  46	    lVar3 = 1;
  47	    lVar1 = *param_1;
  48	  }
  49	  else {
  50	    uVar4 = 1;
  51	    lVar3 = func_0x000140613c10(param_3,1);
  52	    if (lVar3 == 0) goto code_r0x000141598926;
  53	    func_0x000142923c00(lVar3,param_2,param_3);
  54	    lVar1 = *param_1;
  55	  }
  56	  if ((!SBORROW8(0,lVar1)) && (lVar1 != 0)) {
  57	    func_0x000140613c20(param_1[1],lVar1,1);
  58	  }
  59	  param_1[1] = lVar3;
  60	  param_1[2] = param_3;
  61	  *param_1 = -0x8000000000000000;
  62	  lStack_98 = param_1[1];
  63	  lStack_90 = param_1[2];
  64	  lStack_a0 = param_3;
  65	  if (SBORROW8(0,*param_4)) {
  66	    acStack_68[0] = '\0';
  67	  }
  68	  else {
  69	    lStack_48 = param_3;
  70	    FUN_14085fd80(acStack_68,param_4);
  71	    if (acStack_68[0] == '\x06') {
  72	      uVar4 = CONCAT17(uStack_59,CONCAT25(uStack_5b,CONCAT41(uStack_5f,uStack_60)));
  73	      if (lStack_48 != 0) {
  74	        func_0x000140613c20(lStack_98,lStack_48,1);
  75	        return uVar4;
  76	      }
  77	      return uVar4;
  78	    }
  79	  }
  80	  uStack_b8 = uStack_58;
  81	  uStack_b4 = uStack_54;
  82	  uStack_b0 = uStack_50;
  83	  uStack_ac = uStack_4c;
  84	  acStack_c8[0] = acStack_68[0];
  85	  uStack_bf = uStack_5f;
  86	  uStack_bb = uStack_5b;
  87	  uStack_b9 = uStack_59;
  88	  FUN_14143ba30(auStack_88,param_1 + 3,&lStack_a0,acStack_c8);
  89	  if (false) {
  90	code_r0x000141598906:
  91	    FUN_140898f40(&lStack_80);
  92	  }
  93	  else {
  94	    switch(auStack_88[0]) {
  95	    case 0:
  96	    case 1:
  97	    case 2:
  98	    case 6:
  99	      goto code_r0x000141598911;
 100	    case 3:
 101	      if (lStack_80 == 0) {
 102	        return 0;
 103	      }
 104	      uVar4 = 1;
 105	      break;
 106	    case 4:
 107	      lStack_48 = lStack_78;
 108	      FUN_140888d70(lStack_78,uStack_70);
 109	      if (lStack_80 == 0) {
 110	        return 0;
 111	      }
 112	      lStack_80 = lStack_80 << 5;
 113	      uVar4 = 8;
 114	      lStack_78 = lStack_48;
 115	      break;
 116	    default:
 117	      goto code_r0x000141598906;
 118	    }
 119	    func_0x000140613c20(lStack_78,lStack_80,uVar4);
 120	  }
 121	code_r0x000141598911:
 122	  return 0;
 123	}
 124	