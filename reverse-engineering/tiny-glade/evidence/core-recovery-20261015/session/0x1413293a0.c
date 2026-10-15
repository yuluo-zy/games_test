   1	
   2	void FUN_1413293a0(undefined1 *param_1,int *param_2,longlong param_3)
   3	
   4	{
   5	  undefined8 uVar1;
   6	  undefined1 uVar2;
   7	  
   8	  if (param_3 == 5) {
   9	    if (((char)param_2[1] == 'n' && *param_2 == 0x65646c4f) ||
  10	       ((char)param_2[1] == 'n' && *param_2 == 0x65646c6f)) {
  11	      param_1[1] = 1;
  12	      uVar2 = 0;
  13	      goto code_r0x000141329514;
  14	    }
  15	  }
  16	  else if (param_3 == 7) {
  17	    if ((*(int *)((longlong)param_2 + 3) == 0x79726577 && *param_2 == 0x776f6c46) ||
  18	       (*(int *)((longlong)param_2 + 3) == 0x79726577 && *param_2 == 0x776f6c66)) {
  19	      param_1[1] = 2;
  20	      uVar2 = 0;
  21	      goto code_r0x000141329514;
  22	    }
  23	  }
  24	  else if (param_3 == 6) {
  25	    if (((short)param_2[1] == 0x7265 && *param_2 == 0x6d6d7553) ||
  26	       ((short)param_2[1] == 0x7265 && *param_2 == 0x6d6d7573)) {
  27	      param_1[1] = 0;
  28	      uVar2 = 0;
  29	      goto code_r0x000141329514;
  30	    }
  31	    if (((short)param_2[1] == 0x6e6d && *param_2 == 0x75747541) ||
  32	       ((short)param_2[1] == 0x6e6d && *param_2 == 0x75747561)) {
  33	      param_1[1] = 3;
  34	      uVar2 = 0;
  35	      goto code_r0x000141329514;
  36	    }
  37	    if (((short)param_2[1] == 0x7265 && *param_2 == 0x746e6957) ||
  38	       ((short)param_2[1] == 0x7265 && *param_2 == 0x746e6977)) {
  39	      param_1[1] = 4;
  40	      uVar2 = 0;
  41	      goto code_r0x000141329514;
  42	    }
  43	  }
  44	  uVar1 = FUN_1428f9e20(param_2,param_3,&UNK_142c3a7c8,5);
  45	  *(undefined8 *)(param_1 + 8) = uVar1;
  46	  uVar2 = 1;
  47	code_r0x000141329514:
  48	  *param_1 = uVar2;
  49	  return;
  50	}
  51	