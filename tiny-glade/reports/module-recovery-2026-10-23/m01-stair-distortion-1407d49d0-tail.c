2450	            iVar100 = movmskpd((int)uVar30 * 8,auVar95);
2451	            if (iVar100 == 3) goto LAB_1407d7050;
2452	            uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
2453	            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
2454	          } while (uVar26 != 0);
2455	        }
2456	        auVar53[0] = -(*pcVar3 == -1);
2457	        auVar53[1] = -(pcVar3[1] == -1);
2458	        auVar53[2] = -(pcVar3[2] == -1);
2459	        auVar53[3] = -(pcVar3[3] == -1);
2460	        auVar53[4] = -(pcVar3[4] == -1);
2461	        auVar53[5] = -(pcVar3[5] == -1);
2462	        auVar53[6] = -(pcVar3[6] == -1);
2463	        auVar53[7] = -(pcVar3[7] == -1);
2464	        auVar53[8] = -(pcVar3[8] == -1);
2465	        auVar53[9] = -(pcVar3[9] == -1);
2466	        auVar53[10] = -(pcVar3[10] == -1);
2467	        auVar53[0xb] = -(pcVar3[0xb] == -1);
2468	        auVar53[0xc] = -(pcVar3[0xc] == -1);
2469	        auVar53[0xd] = -(pcVar3[0xd] == -1);
2470	        auVar53[0xe] = -(pcVar3[0xe] == -1);
2471	        auVar53[0xf] = -(pcVar3[0xf] == -1);
2472	        if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 || (SUB161(auVar53 >> 0xf,0) & 1) != 0)
2473	                        || (SUB161(auVar53 >> 0x17,0) & 1) != 0) ||
2474	                       (SUB161(auVar53 >> 0x1f,0) & 1) != 0) || (SUB161(auVar53 >> 0x27,0) & 1) != 0
2475	                      ) || (SUB161(auVar53 >> 0x2f,0) & 1) != 0) ||
2476	                    (SUB161(auVar53 >> 0x37,0) & 1) != 0) || (SUB161(auVar53 >> 0x3f,0) & 1) != 0)
2477	                  || (SUB161(auVar53 >> 0x47,0) & 1) != 0) || (SUB161(auVar53 >> 0x4f,0) & 1) != 0)
2478	                || (SUB161(auVar53 >> 0x57,0) & 1) != 0) || (SUB161(auVar53 >> 0x5f,0) & 1) != 0) ||
2479	              (SUB161(auVar53 >> 0x67,0) & 1) != 0) || (SUB161(auVar53 >> 0x6f,0) & 1) != 0) ||
2480	            (SUB161(auVar53 >> 0x77,0) & 1) != 0) || auVar53[0xf] < '\0') break;
2481	        uVar20 = uVar20 + lVar40 + 0x10;
2482	        lVar40 = lVar40 + 0x10;
2483	      }
2484	    }
2485	    country_core::resources::stairs::stairs_assembly_params::impl$2::from();
2486	    uVar15 = (uint)(byte)local_198;
2487	    pauVar21 = local_178;
2488	    local_138 = extraout_RAX_44;
2489	    if (*(longlong *)(local_230 + 0x150) != 0) {
2490	      core::hash::BuildHasher::
2491	      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
2492	      lVar40 = *(longlong *)(local_230 + 0x138);
2493	      bVar42 = (byte)(extraout_RAX_45 >> 0x39);
2494	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
2495	      auVar46 = pshuflw(auVar46,auVar46,0);
2496	      lVar27 = 0;
2497	      uVar20 = extraout_RAX_45;
2498	      while( true ) {
2499	        uVar20 = *(ulonglong *)(local_230 + 0x140) & uVar20;
2500	        pcVar3 = (char *)(lVar40 + uVar20);
2501	        cVar44 = auVar46[0];
2502	        auVar77[0] = -(*pcVar3 == cVar44);
2503	        cVar47 = auVar46[1];
2504	        auVar77[1] = -(pcVar3[1] == cVar47);
2505	        cVar48 = auVar46[2];
2506	        auVar77[2] = -(pcVar3[2] == cVar48);
2507	        cVar49 = auVar46[3];
2508	        auVar77[3] = -(pcVar3[3] == cVar49);
2509	        auVar77[4] = -(pcVar3[4] == cVar44);
2510	        auVar77[5] = -(pcVar3[5] == cVar47);
2511	        auVar77[6] = -(pcVar3[6] == cVar48);
2512	        auVar77[7] = -(pcVar3[7] == cVar49);
2513	        auVar77[8] = -(pcVar3[8] == cVar44);
2514	        auVar77[9] = -(pcVar3[9] == cVar47);
2515	        auVar77[10] = -(pcVar3[10] == cVar48);
2516	        auVar77[0xb] = -(pcVar3[0xb] == cVar49);
2517	        auVar77[0xc] = -(pcVar3[0xc] == cVar44);
2518	        auVar77[0xd] = -(pcVar3[0xd] == cVar47);
2519	        auVar77[0xe] = -(pcVar3[0xe] == cVar48);
2520	        auVar77[0xf] = -(pcVar3[0xf] == cVar49);
2521	        uVar26 = (ushort)(SUB161(auVar77 >> 7,0) & 1) | (ushort)(SUB161(auVar77 >> 0xf,0) & 1) << 1
2522	                 | (ushort)(SUB161(auVar77 >> 0x17,0) & 1) << 2 |
2523	                 (ushort)(SUB161(auVar77 >> 0x1f,0) & 1) << 3 |
2524	                 (ushort)(SUB161(auVar77 >> 0x27,0) & 1) << 4 |
2525	                 (ushort)(SUB161(auVar77 >> 0x2f,0) & 1) << 5 |
2526	                 (ushort)(SUB161(auVar77 >> 0x37,0) & 1) << 6 |
2527	                 (ushort)(SUB161(auVar77 >> 0x3f,0) & 1) << 7 |
2528	                 (ushort)(SUB161(auVar77 >> 0x47,0) & 1) << 8 |
2529	                 (ushort)(SUB161(auVar77 >> 0x4f,0) & 1) << 9 |
2530	                 (ushort)(SUB161(auVar77 >> 0x57,0) & 1) << 10 |
2531	                 (ushort)(SUB161(auVar77 >> 0x5f,0) & 1) << 0xb |
2532	                 (ushort)(SUB161(auVar77 >> 0x67,0) & 1) << 0xc |
2533	                 (ushort)(SUB161(auVar77 >> 0x6f,0) & 1) << 0xd |
2534	                 (ushort)(SUB161(auVar77 >> 0x77,0) & 1) << 0xe | (ushort)(auVar77[0xf] >> 7) << 0xf
2535	        ;
2536	        uVar23 = (uint)uVar26;
2537	        while (pauVar21 = local_178, uVar26 != 0) {
2538	          uVar8 = 0;
2539	          for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
2540	            uVar8 = uVar8 + 1;
2541	          }
2542	          lVar38 = (uVar8 + uVar20 & *(ulonglong *)(local_230 + 0x140)) * -0x78;
2543	          if (extraout_RAX_44 == *(undefined1 (**) [16])(lVar40 + -0x78 + lVar38)) {
2544	            *(undefined8 *)(lVar40 + -0x18 + lVar38) = 0;
2545	            goto LAB_1407d6e54;
2546	          }
2547	          uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
2548	          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
2549	        }
2550	        auVar57[0] = -(*pcVar3 == -1);
2551	        auVar57[1] = -(pcVar3[1] == -1);
2552	        auVar57[2] = -(pcVar3[2] == -1);
2553	        auVar57[3] = -(pcVar3[3] == -1);
2554	        auVar57[4] = -(pcVar3[4] == -1);
2555	        auVar57[5] = -(pcVar3[5] == -1);
2556	        auVar57[6] = -(pcVar3[6] == -1);
2557	        auVar57[7] = -(pcVar3[7] == -1);
2558	        auVar57[8] = -(pcVar3[8] == -1);
2559	        auVar57[9] = -(pcVar3[9] == -1);
2560	        auVar57[10] = -(pcVar3[10] == -1);
2561	        auVar57[0xb] = -(pcVar3[0xb] == -1);
2562	        auVar57[0xc] = -(pcVar3[0xc] == -1);
2563	        auVar57[0xd] = -(pcVar3[0xd] == -1);
2564	        auVar57[0xe] = -(pcVar3[0xe] == -1);
2565	        auVar57[0xf] = -(pcVar3[0xf] == -1);
2566	        if ((((((((((((((((SUB161(auVar57 >> 7,0) & 1) != 0 || (SUB161(auVar57 >> 0xf,0) & 1) != 0)
2567	                        || (SUB161(auVar57 >> 0x17,0) & 1) != 0) ||
2568	                       (SUB161(auVar57 >> 0x1f,0) & 1) != 0) || (SUB161(auVar57 >> 0x27,0) & 1) != 0
2569	                      ) || (SUB161(auVar57 >> 0x2f,0) & 1) != 0) ||
2570	                    (SUB161(auVar57 >> 0x37,0) & 1) != 0) || (SUB161(auVar57 >> 0x3f,0) & 1) != 0)
2571	                  || (SUB161(auVar57 >> 0x47,0) & 1) != 0) || (SUB161(auVar57 >> 0x4f,0) & 1) != 0)
2572	                || (SUB161(auVar57 >> 0x57,0) & 1) != 0) || (SUB161(auVar57 >> 0x5f,0) & 1) != 0) ||
2573	              (SUB161(auVar57 >> 0x67,0) & 1) != 0) || (SUB161(auVar57 >> 0x6f,0) & 1) != 0) ||
2574	            (SUB161(auVar57 >> 0x77,0) & 1) != 0) || auVar57[0xf] < '\0') break;
2575	        uVar20 = uVar20 + lVar27 + 0x10;
2576	        lVar27 = lVar27 + 0x10;
2577	      }
2578	    }
2579	  } while( true );
2580	LAB_1407d59a3:
2581	  pfVar28 = local_f8;
2582	  if (puVar29 != puVar31) {
2583	    puVar16 = puVar29 + 2;
2584	    if (*(char *)(puVar29 + 1) == '\x11') goto LAB_1407d56d7;
2585	    local_168 = (undefined1 (*) [16])*puVar29;
2586	    local_160 = CONCAT13(*(undefined1 *)((longlong)puVar29 + 0xb),
2587	                         CONCAT21(*(undefined2 *)((longlong)puVar29 + 9),*(char *)(puVar29 + 1)));
2588	    puVar29 = puVar16;
2589	    if (*(longlong *)(lVar40 + 0x168) != 0) {
2590	      core::hash::BuildHasher::
2591	      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
2592	                ();
2593	      lVar27 = *(longlong *)(lVar40 + 0x150);
2594	      bVar42 = (byte)(extraout_RAX_15 >> 0x39);
2595	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
2596	      auVar46 = pshuflw(auVar46,auVar46,0);
2597	      lVar38 = 0;
2598	      uVar20 = extraout_RAX_15;
2599	      while( true ) {
2600	        uVar20 = *(ulonglong *)(lVar40 + 0x158) & uVar20;
2601	        pcVar3 = (char *)(lVar27 + uVar20);
2602	        cVar44 = auVar46[0];
2603	        auVar80[0] = -(*pcVar3 == cVar44);
2604	        cVar47 = auVar46[1];
2605	        auVar80[1] = -(pcVar3[1] == cVar47);
2606	        cVar48 = auVar46[2];
2607	        auVar80[2] = -(pcVar3[2] == cVar48);
2608	        cVar49 = auVar46[3];
2609	        auVar80[3] = -(pcVar3[3] == cVar49);
2610	        auVar80[4] = -(pcVar3[4] == cVar44);
2611	        auVar80[5] = -(pcVar3[5] == cVar47);
2612	        auVar80[6] = -(pcVar3[6] == cVar48);
2613	        auVar80[7] = -(pcVar3[7] == cVar49);
2614	        auVar80[8] = -(pcVar3[8] == cVar44);
2615	        auVar80[9] = -(pcVar3[9] == cVar47);
2616	        auVar80[10] = -(pcVar3[10] == cVar48);
2617	        auVar80[0xb] = -(pcVar3[0xb] == cVar49);
2618	        auVar80[0xc] = -(pcVar3[0xc] == cVar44);
2619	        auVar80[0xd] = -(pcVar3[0xd] == cVar47);
2620	        auVar80[0xe] = -(pcVar3[0xe] == cVar48);
2621	        auVar80[0xf] = -(pcVar3[0xf] == cVar49);
2622	        uVar26 = (ushort)(SUB161(auVar80 >> 7,0) & 1) | (ushort)(SUB161(auVar80 >> 0xf,0) & 1) << 1
2623	                 | (ushort)(SUB161(auVar80 >> 0x17,0) & 1) << 2 |
2624	                 (ushort)(SUB161(auVar80 >> 0x1f,0) & 1) << 3 |
2625	                 (ushort)(SUB161(auVar80 >> 0x27,0) & 1) << 4 |
2626	                 (ushort)(SUB161(auVar80 >> 0x2f,0) & 1) << 5 |
2627	                 (ushort)(SUB161(auVar80 >> 0x37,0) & 1) << 6 |
2628	                 (ushort)(SUB161(auVar80 >> 0x3f,0) & 1) << 7 |
2629	                 (ushort)(SUB161(auVar80 >> 0x47,0) & 1) << 8 |
2630	                 (ushort)(SUB161(auVar80 >> 0x4f,0) & 1) << 9 |
2631	                 (ushort)(SUB161(auVar80 >> 0x57,0) & 1) << 10 |
2632	                 (ushort)(SUB161(auVar80 >> 0x5f,0) & 1) << 0xb |
2633	                 (ushort)(SUB161(auVar80 >> 0x67,0) & 1) << 0xc |
2634	                 (ushort)(SUB161(auVar80 >> 0x6f,0) & 1) << 0xd |
2635	                 (ushort)(SUB161(auVar80 >> 0x77,0) & 1) << 0xe | (ushort)(auVar80[0xf] >> 7) << 0xf
2636	        ;
2637	        uVar23 = (uint)uVar26;
2638	        while (uVar26 != 0) {
2639	          uVar8 = 0;
2640	          for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
2641	            uVar8 = uVar8 + 1;
2642	          }
2643	          uVar30 = uVar8 + uVar20 & *(ulonglong *)(lVar40 + 0x158);
2644	          if (((char)local_160 == *(char *)(lVar27 + -4 + uVar30 * -0xc)) &&
2645	             (puVar16 = (undefined8 *)(lVar27 + -0xc + uVar30 * -0xc), uVar5 = *puVar16,
2646	             auVar81._0_4_ = -(uint)((int)uVar5 == (int)local_168),
2647	             iVar100 = -(uint)((int)((ulonglong)uVar5 >> 0x20) ==
2648	                              (int)((ulonglong)local_168 >> 0x20)), auVar81._4_4_ = auVar81._0_4_,
2649	             auVar81._8_4_ = iVar100, auVar81._12_4_ = iVar100,

2650	             iVar100 = movmskpd((int)puVar16,auVar81), iVar100 == 3)) goto LAB_1407d59a3;
2651	          uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
2652	          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
2653	        }
2654	        auVar59[0] = -(*pcVar3 == -1);
2655	        auVar59[1] = -(pcVar3[1] == -1);
2656	        auVar59[2] = -(pcVar3[2] == -1);
2657	        auVar59[3] = -(pcVar3[3] == -1);
2658	        auVar59[4] = -(pcVar3[4] == -1);
2659	        auVar59[5] = -(pcVar3[5] == -1);
2660	        auVar59[6] = -(pcVar3[6] == -1);
2661	        auVar59[7] = -(pcVar3[7] == -1);
2662	        auVar59[8] = -(pcVar3[8] == -1);
2663	        auVar59[9] = -(pcVar3[9] == -1);
2664	        auVar59[10] = -(pcVar3[10] == -1);
2665	        auVar59[0xb] = -(pcVar3[0xb] == -1);
2666	        auVar59[0xc] = -(pcVar3[0xc] == -1);
2667	        auVar59[0xd] = -(pcVar3[0xd] == -1);
2668	        auVar59[0xe] = -(pcVar3[0xe] == -1);
2669	        auVar59[0xf] = -(pcVar3[0xf] == -1);
2670	        if ((((((((((((((((SUB161(auVar59 >> 7,0) & 1) != 0 || (SUB161(auVar59 >> 0xf,0) & 1) != 0)
2671	                        || (SUB161(auVar59 >> 0x17,0) & 1) != 0) ||
2672	                       (SUB161(auVar59 >> 0x1f,0) & 1) != 0) || (SUB161(auVar59 >> 0x27,0) & 1) != 0
2673	                      ) || (SUB161(auVar59 >> 0x2f,0) & 1) != 0) ||
2674	                    (SUB161(auVar59 >> 0x37,0) & 1) != 0) || (SUB161(auVar59 >> 0x3f,0) & 1) != 0)
2675	                  || (SUB161(auVar59 >> 0x47,0) & 1) != 0) || (SUB161(auVar59 >> 0x4f,0) & 1) != 0)
2676	                || (SUB161(auVar59 >> 0x57,0) & 1) != 0) || (SUB161(auVar59 >> 0x5f,0) & 1) != 0) ||
2677	              (SUB161(auVar59 >> 0x67,0) & 1) != 0) || (SUB161(auVar59 >> 0x6f,0) & 1) != 0) ||
2678	            (SUB161(auVar59 >> 0x77,0) & 1) != 0) || auVar59[0xf] < '\0') break;
2679	        uVar20 = uVar20 + lVar38 + 0x10;
2680	        lVar38 = lVar38 + 0x10;
2681	      }
2682	    }
2683	    if (plStack_2c0 != (longlong *)0x0) {
2684	      core::hash::BuildHasher::
2685	      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
2686	                ();
2687	      bVar42 = (byte)(extraout_RAX_16 >> 0x39);
2688	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
2689	      auVar46 = pshuflw(auVar46,auVar46,0);
2690	      lVar27 = 0;
2691	      uVar20 = extraout_RAX_16;
2692	      while( true ) {
2693	        uVar20 = (ulonglong)uStack_2d0 & uVar20;
2694	        pcVar3 = *local_2d8 + uVar20;
2695	        cVar44 = auVar46[0];
2696	        auVar82[0] = -(*pcVar3 == cVar44);
2697	        cVar47 = auVar46[1];
2698	        auVar82[1] = -(pcVar3[1] == cVar47);
2699	        cVar48 = auVar46[2];
2700	        auVar82[2] = -(pcVar3[2] == cVar48);
2701	        cVar49 = auVar46[3];
2702	        auVar82[3] = -(pcVar3[3] == cVar49);
2703	        auVar82[4] = -(pcVar3[4] == cVar44);
2704	        auVar82[5] = -(pcVar3[5] == cVar47);
2705	        auVar82[6] = -(pcVar3[6] == cVar48);
2706	        auVar82[7] = -(pcVar3[7] == cVar49);
2707	        auVar82[8] = -(pcVar3[8] == cVar44);
2708	        auVar82[9] = -(pcVar3[9] == cVar47);
2709	        auVar82[10] = -(pcVar3[10] == cVar48);
2710	        auVar82[0xb] = -(pcVar3[0xb] == cVar49);
2711	        auVar82[0xc] = -(pcVar3[0xc] == cVar44);
2712	        auVar82[0xd] = -(pcVar3[0xd] == cVar47);
2713	        auVar82[0xe] = -(pcVar3[0xe] == cVar48);
2714	        auVar82[0xf] = -(pcVar3[0xf] == cVar49);
2715	        uVar26 = (ushort)(SUB161(auVar82 >> 7,0) & 1) | (ushort)(SUB161(auVar82 >> 0xf,0) & 1) << 1
2716	                 | (ushort)(SUB161(auVar82 >> 0x17,0) & 1) << 2 |
2717	                 (ushort)(SUB161(auVar82 >> 0x1f,0) & 1) << 3 |
2718	                 (ushort)(SUB161(auVar82 >> 0x27,0) & 1) << 4 |
2719	                 (ushort)(SUB161(auVar82 >> 0x2f,0) & 1) << 5 |
2720	                 (ushort)(SUB161(auVar82 >> 0x37,0) & 1) << 6 |
2721	                 (ushort)(SUB161(auVar82 >> 0x3f,0) & 1) << 7 |
2722	                 (ushort)(SUB161(auVar82 >> 0x47,0) & 1) << 8 |
2723	                 (ushort)(SUB161(auVar82 >> 0x4f,0) & 1) << 9 |
2724	                 (ushort)(SUB161(auVar82 >> 0x57,0) & 1) << 10 |
2725	                 (ushort)(SUB161(auVar82 >> 0x5f,0) & 1) << 0xb |
2726	                 (ushort)(SUB161(auVar82 >> 0x67,0) & 1) << 0xc |
2727	                 (ushort)(SUB161(auVar82 >> 0x6f,0) & 1) << 0xd |
2728	                 (ushort)(SUB161(auVar82 >> 0x77,0) & 1) << 0xe | (ushort)(auVar82[0xf] >> 7) << 0xf
2729	        ;
2730	        uVar23 = (uint)uVar26;
2731	        while (uVar26 != 0) {
2732	          uVar8 = 0;
2733	          for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
2734	            uVar8 = uVar8 + 1;
2735	          }
2736	          uVar30 = uVar8 + uVar20 & (ulonglong)uStack_2d0;
2737	          if (((char)local_160 == *(char *)((longlong)local_2d8 + uVar30 * -0xc + -4)) &&
2738	             (puVar16 = (undefined8 *)((longlong)local_2d8 + uVar30 * -0xc + -0xc), uVar5 = *puVar16
2739	             , auVar83._0_4_ = -(uint)((int)uVar5 == (int)local_168),
2740	             iVar100 = -(uint)((int)((ulonglong)uVar5 >> 0x20) ==
2741	                              (int)((ulonglong)local_168 >> 0x20)), auVar83._4_4_ = auVar83._0_4_,
2742	             auVar83._8_4_ = iVar100, auVar83._12_4_ = iVar100,
2743	             iVar100 = movmskpd((int)puVar16,auVar83), iVar100 == 3)) goto LAB_1407d59a3;
2744	          uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
2745	          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
2746	        }
2747	        auVar60[0] = -(*pcVar3 == -1);
2748	        auVar60[1] = -(pcVar3[1] == -1);
2749	        auVar60[2] = -(pcVar3[2] == -1);
2750	        auVar60[3] = -(pcVar3[3] == -1);
2751	        auVar60[4] = -(pcVar3[4] == -1);
2752	        auVar60[5] = -(pcVar3[5] == -1);
2753	        auVar60[6] = -(pcVar3[6] == -1);
2754	        auVar60[7] = -(pcVar3[7] == -1);
2755	        auVar60[8] = -(pcVar3[8] == -1);
2756	        auVar60[9] = -(pcVar3[9] == -1);
2757	        auVar60[10] = -(pcVar3[10] == -1);
2758	        auVar60[0xb] = -(pcVar3[0xb] == -1);
2759	        auVar60[0xc] = -(pcVar3[0xc] == -1);
2760	        auVar60[0xd] = -(pcVar3[0xd] == -1);
2761	        auVar60[0xe] = -(pcVar3[0xe] == -1);
2762	        auVar60[0xf] = -(pcVar3[0xf] == -1);
2763	        if ((((((((((((((((SUB161(auVar60 >> 7,0) & 1) != 0 || (SUB161(auVar60 >> 0xf,0) & 1) != 0)
2764	                        || (SUB161(auVar60 >> 0x17,0) & 1) != 0) ||
2765	                       (SUB161(auVar60 >> 0x1f,0) & 1) != 0) || (SUB161(auVar60 >> 0x27,0) & 1) != 0
2766	                      ) || (SUB161(auVar60 >> 0x2f,0) & 1) != 0) ||
2767	                    (SUB161(auVar60 >> 0x37,0) & 1) != 0) || (SUB161(auVar60 >> 0x3f,0) & 1) != 0)
2768	                  || (SUB161(auVar60 >> 0x47,0) & 1) != 0) || (SUB161(auVar60 >> 0x4f,0) & 1) != 0)
2769	                || (SUB161(auVar60 >> 0x57,0) & 1) != 0) || (SUB161(auVar60 >> 0x5f,0) & 1) != 0) ||
2770	              (SUB161(auVar60 >> 0x67,0) & 1) != 0) || (SUB161(auVar60 >> 0x6f,0) & 1) != 0) ||
2771	            (SUB161(auVar60 >> 0x77,0) & 1) != 0) || auVar60[0xf] < '\0') break;
2772	        uVar20 = uVar20 + lVar27 + 0x10;
2773	        lVar27 = lVar27 + 0x10;
2774	      }
2775	    }
2776	    if (*(longlong *)(lVar40 + 0x138) != 0) {
2777	      core::hash::BuildHasher::
2778	      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
2779	                ();
2780	      lVar27 = *(longlong *)(lVar40 + 0x120);
2781	      puVar2 = (ulonglong *)(lVar40 + 0x128);
2782	      bVar42 = (byte)(extraout_RAX_17 >> 0x39);
2783	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
2784	      auVar46 = pshuflw(auVar46,auVar46,0);
2785	      lVar38 = 0;
2786	      uVar20 = extraout_RAX_17;
2787	      while( true ) {
2788	        uVar20 = *puVar2 & uVar20;
2789	        pcVar3 = (char *)(lVar27 + uVar20);
2790	        cVar44 = auVar46[0];
2791	        auVar84[0] = -(*pcVar3 == cVar44);
2792	        cVar47 = auVar46[1];
2793	        auVar84[1] = -(pcVar3[1] == cVar47);
2794	        cVar48 = auVar46[2];
2795	        auVar84[2] = -(pcVar3[2] == cVar48);
2796	        cVar49 = auVar46[3];
2797	        auVar84[3] = -(pcVar3[3] == cVar49);
2798	        auVar84[4] = -(pcVar3[4] == cVar44);
2799	        auVar84[5] = -(pcVar3[5] == cVar47);
2800	        auVar84[6] = -(pcVar3[6] == cVar48);
2801	        auVar84[7] = -(pcVar3[7] == cVar49);
2802	        auVar84[8] = -(pcVar3[8] == cVar44);
2803	        auVar84[9] = -(pcVar3[9] == cVar47);
2804	        auVar84[10] = -(pcVar3[10] == cVar48);
2805	        auVar84[0xb] = -(pcVar3[0xb] == cVar49);
2806	        auVar84[0xc] = -(pcVar3[0xc] == cVar44);
2807	        auVar84[0xd] = -(pcVar3[0xd] == cVar47);
2808	        auVar84[0xe] = -(pcVar3[0xe] == cVar48);
2809	        auVar84[0xf] = -(pcVar3[0xf] == cVar49);
2810	        uVar26 = (ushort)(SUB161(auVar84 >> 7,0) & 1) | (ushort)(SUB161(auVar84 >> 0xf,0) & 1) << 1
2811	                 | (ushort)(SUB161(auVar84 >> 0x17,0) & 1) << 2 |
2812	                 (ushort)(SUB161(auVar84 >> 0x1f,0) & 1) << 3 |
2813	                 (ushort)(SUB161(auVar84 >> 0x27,0) & 1) << 4 |
2814	                 (ushort)(SUB161(auVar84 >> 0x2f,0) & 1) << 5 |
2815	                 (ushort)(SUB161(auVar84 >> 0x37,0) & 1) << 6 |
2816	                 (ushort)(SUB161(auVar84 >> 0x3f,0) & 1) << 7 |
2817	                 (ushort)(SUB161(auVar84 >> 0x47,0) & 1) << 8 |
2818	                 (ushort)(SUB161(auVar84 >> 0x4f,0) & 1) << 9 |
2819	                 (ushort)(SUB161(auVar84 >> 0x57,0) & 1) << 10 |
2820	                 (ushort)(SUB161(auVar84 >> 0x5f,0) & 1) << 0xb |
2821	                 (ushort)(SUB161(auVar84 >> 0x67,0) & 1) << 0xc |
2822	                 (ushort)(SUB161(auVar84 >> 0x6f,0) & 1) << 0xd |
2823	                 (ushort)(SUB161(auVar84 >> 0x77,0) & 1) << 0xe | (ushort)(auVar84[0xf] >> 7) << 0xf
2824	        ;
2825	        uVar23 = (uint)uVar26;
2826	        while (lVar40 = local_e8, uVar26 != 0) {
2827	          uVar8 = 0;
2828	          for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
2829	            uVar8 = uVar8 + 1;
2830	          }
2831	          uVar30 = uVar8 + uVar20 & *puVar2;
2832	          puVar16 = (undefined8 *)(lVar27 + -0x10 + uVar30 * -0x10);
2833	          local_170 = lVar38;
2834	          if (((char)local_160 == *(char *)(puVar16 + 1)) &&
2835	             (auVar85._0_4_ = -(uint)((int)*puVar16 == (int)local_168),
2836	             iVar100 = -(uint)((int)((ulonglong)*puVar16 >> 0x20) ==
2837	                              (int)((ulonglong)local_168 >> 0x20)), auVar85._4_4_ = auVar85._0_4_,
2838	             auVar85._8_4_ = iVar100, auVar85._12_4_ = iVar100,
2839	             iVar100 = movmskpd((int)puVar16,auVar85), iVar100 == 3)) {
2840	            fVar50 = *(float *)(lVar27 + -4 + uVar30 * -0x10);
2841	            if ((fVar50 == fVar103) && (!NAN(fVar50) && !NAN(fVar103))) {
2842	              uStack_130 = (undefined1 (*) [16])CONCAT44(uStack_130._4_4_,local_160);
2843	              local_138 = local_168;
2844	              hashbrown::map::HashMap::
2845	              insert<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
2846	                        ();
2847	            }
2848	            goto LAB_1407d59a3;
2849	          }

2850	          uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
2851	          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
2852	        }
2853	        auVar61[0] = -(*pcVar3 == -1);
2854	        auVar61[1] = -(pcVar3[1] == -1);
2855	        auVar61[2] = -(pcVar3[2] == -1);
2856	        auVar61[3] = -(pcVar3[3] == -1);
2857	        auVar61[4] = -(pcVar3[4] == -1);
2858	        auVar61[5] = -(pcVar3[5] == -1);
2859	        auVar61[6] = -(pcVar3[6] == -1);
2860	        auVar61[7] = -(pcVar3[7] == -1);
2861	        auVar61[8] = -(pcVar3[8] == -1);
2862	        auVar61[9] = -(pcVar3[9] == -1);
2863	        auVar61[10] = -(pcVar3[10] == -1);
2864	        auVar61[0xb] = -(pcVar3[0xb] == -1);
2865	        auVar61[0xc] = -(pcVar3[0xc] == -1);
2866	        auVar61[0xd] = -(pcVar3[0xd] == -1);
2867	        auVar61[0xe] = -(pcVar3[0xe] == -1);
2868	        auVar61[0xf] = -(pcVar3[0xf] == -1);
2869	        if ((((((((((((((((SUB161(auVar61 >> 7,0) & 1) != 0 || (SUB161(auVar61 >> 0xf,0) & 1) != 0)
2870	                        || (SUB161(auVar61 >> 0x17,0) & 1) != 0) ||
2871	                       (SUB161(auVar61 >> 0x1f,0) & 1) != 0) || (SUB161(auVar61 >> 0x27,0) & 1) != 0
2872	                      ) || (SUB161(auVar61 >> 0x2f,0) & 1) != 0) ||
2873	                    (SUB161(auVar61 >> 0x37,0) & 1) != 0) || (SUB161(auVar61 >> 0x3f,0) & 1) != 0)
2874	                  || (SUB161(auVar61 >> 0x47,0) & 1) != 0) || (SUB161(auVar61 >> 0x4f,0) & 1) != 0)
2875	                || (SUB161(auVar61 >> 0x57,0) & 1) != 0) || (SUB161(auVar61 >> 0x5f,0) & 1) != 0) ||
2876	              (SUB161(auVar61 >> 0x67,0) & 1) != 0) || (SUB161(auVar61 >> 0x6f,0) & 1) != 0) ||
2877	            (SUB161(auVar61 >> 0x77,0) & 1) != 0) || auVar61[0xf] < '\0') break;
2878	        uVar20 = uVar20 + lVar38 + 0x10;
2879	        lVar38 = lVar38 + 0x10;
2880	      }
2881	    }
2882	    goto LAB_1407d59a3;
2883	  }
2884	  goto LAB_1407d56d7;
2885	LAB_1407d7050:
2886	  uVar20 = *(ulonglong *)local_108[1];
2887	  uVar30 = *(ulonglong *)local_108[9];
2888	  uVar32 = uVar20 - uVar30;
2889	  if (uVar20 < uVar30 || uVar32 == 0) goto LAB_1407d7111;
2890	  if (*(longlong *)local_108[8] - uVar30 < uVar32) {
2891	    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
2892	    lVar40 = *(longlong *)(local_108[8] + 8);
2893	    puVar19 = (undefined4 *)(lVar40 + *(ulonglong *)local_108[9] * 4);
2894	    uVar17 = *(ulonglong *)local_108[9];
2895	  }
2896	  else {
2897	    lVar40 = *(longlong *)(local_108[8] + 8);
2898	    puVar19 = (undefined4 *)(lVar40 + uVar30 * 4);
2899	    uVar17 = uVar30;
2900	  }
2901	  if (1 < uVar32) {
2902	    uVar34 = ~uVar30 + uVar20;
2903	    lVar27 = 1;
2904	    if (uVar34 < 8) {
2905	LAB_1407d70e9:
2906	      lVar40 = (lVar27 + uVar30) - uVar20;
2907	      do {
2908	        *puVar19 = 0x3f800000;
2909	        puVar19 = puVar19 + 1;
2910	        lVar40 = lVar40 + 1;
2911	      } while (lVar40 != 0);
2912	    }
2913	    else {
2914	      uVar36 = uVar34 & 0xfffffffffffffff8;
2915	      puVar19 = puVar19 + uVar36;
2916	      lVar27 = uVar36 + 1;
2917	      lVar40 = lVar40 + uVar17 * 4;
2918	      uVar39 = 0;
2919	      do {
2920	        puVar29 = (undefined8 *)(lVar40 + uVar39 * 4);
2921	        *puVar29 = 0x3f8000003f800000;
2922	        puVar29[1] = 0x3f8000003f800000;
2923	        puVar29 = (undefined8 *)(lVar40 + 0x10 + uVar39 * 4);
2924	        *puVar29 = 0x3f8000003f800000;
2925	        puVar29[1] = 0x3f8000003f800000;
2926	        uVar39 = uVar39 + 8;
2927	      } while (uVar36 != uVar39);
2928	      if (uVar34 != uVar36) goto LAB_1407d70e9;
2929	    }
2930	    uVar17 = (uVar17 + uVar32) - 1;
2931	  }
2932	  *puVar19 = 0x3f800000;
2933	  uVar20 = uVar17 + 1;
2934	LAB_1407d7111:
2935	  *(ulonglong *)local_108[9] = uVar20;
2936	  pfVar18 = *(float **)(local_108[8] + 8);
2937	  local_f8 = pfVar18 + uVar20;
2938	  iVar100 = 0;
2939	  do {
2940	    if (pfVar18 == local_f8) {
2941	      country_core::resources::stairs::stairs_assembly_params::impl$2::from();
2942	      lVar40 = local_230;
2943	      if (*(longlong *)(local_230 + 0x150) != 0) {
2944	        local_138 = extraout_RAX_42;
2945	        core::hash::BuildHasher::
2946	        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
2947	        lVar27 = *(longlong *)(lVar40 + 0x138);
2948	        uVar20 = *(ulonglong *)(lVar40 + 0x140);
2949	        bVar42 = (byte)(extraout_RAX_43 >> 0x39);
2950	        auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
2951	        auVar46 = pshuflw(auVar46,auVar46,0);
2952	        lVar40 = 0;
2953	        uVar30 = extraout_RAX_43;
2954	        while( true ) {
2955	          uVar30 = uVar20 & uVar30;
2956	          pcVar3 = (char *)(lVar27 + uVar30);
2957	          cVar44 = auVar46[0];
2958	          auVar76[0] = -(*pcVar3 == cVar44);
2959	          cVar47 = auVar46[1];
2960	          auVar76[1] = -(pcVar3[1] == cVar47);
2961	          cVar48 = auVar46[2];
2962	          auVar76[2] = -(pcVar3[2] == cVar48);
2963	          cVar49 = auVar46[3];
2964	          auVar76[3] = -(pcVar3[3] == cVar49);
2965	          auVar76[4] = -(pcVar3[4] == cVar44);
2966	          auVar76[5] = -(pcVar3[5] == cVar47);
2967	          auVar76[6] = -(pcVar3[6] == cVar48);
2968	          auVar76[7] = -(pcVar3[7] == cVar49);
2969	          auVar76[8] = -(pcVar3[8] == cVar44);
2970	          auVar76[9] = -(pcVar3[9] == cVar47);
2971	          auVar76[10] = -(pcVar3[10] == cVar48);
2972	          auVar76[0xb] = -(pcVar3[0xb] == cVar49);
2973	          auVar76[0xc] = -(pcVar3[0xc] == cVar44);
2974	          auVar76[0xd] = -(pcVar3[0xd] == cVar47);
2975	          auVar76[0xe] = -(pcVar3[0xe] == cVar48);
2976	          auVar76[0xf] = -(pcVar3[0xf] == cVar49);
2977	          uVar26 = (ushort)(SUB161(auVar76 >> 7,0) & 1) |
2978	                   (ushort)(SUB161(auVar76 >> 0xf,0) & 1) << 1 |
2979	                   (ushort)(SUB161(auVar76 >> 0x17,0) & 1) << 2 |
2980	                   (ushort)(SUB161(auVar76 >> 0x1f,0) & 1) << 3 |
2981	                   (ushort)(SUB161(auVar76 >> 0x27,0) & 1) << 4 |
2982	                   (ushort)(SUB161(auVar76 >> 0x2f,0) & 1) << 5 |
2983	                   (ushort)(SUB161(auVar76 >> 0x37,0) & 1) << 6 |
2984	                   (ushort)(SUB161(auVar76 >> 0x3f,0) & 1) << 7 |
2985	                   (ushort)(SUB161(auVar76 >> 0x47,0) & 1) << 8 |
2986	                   (ushort)(SUB161(auVar76 >> 0x4f,0) & 1) << 9 |
2987	                   (ushort)(SUB161(auVar76 >> 0x57,0) & 1) << 10 |
2988	                   (ushort)(SUB161(auVar76 >> 0x5f,0) & 1) << 0xb |
2989	                   (ushort)(SUB161(auVar76 >> 0x67,0) & 1) << 0xc |
2990	                   (ushort)(SUB161(auVar76 >> 0x6f,0) & 1) << 0xd |
2991	                   (ushort)(SUB161(auVar76 >> 0x77,0) & 1) << 0xe |
2992	                   (ushort)(auVar76[0xf] >> 7) << 0xf;
2993	          uVar15 = (uint)uVar26;
2994	          while (uVar26 != 0) {
2995	            uVar23 = 0;
2996	            for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
2997	              uVar23 = uVar23 + 1;
2998	            }
2999	            lVar38 = (uVar23 + uVar30 & uVar20) * -0x78;
3000	            if (extraout_RAX_42 == *(undefined1 (**) [16])(lVar27 + -0x78 + lVar38)) {
3001	              _Src = *(void **)(local_108[8] + 8);
3002	              uVar20 = *(ulonglong *)local_108[9];
3003	              if (uVar20 != *(ulonglong *)(lVar27 + -0x18 + lVar38)) goto LAB_1407d778b;
3004	              uVar30 = 0;
3005	              goto LAB_1407d7770;
3006	            }
3007	            uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
3008	            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
3009	          }
3010	          auVar56[0] = -(*pcVar3 == -1);
3011	          auVar56[1] = -(pcVar3[1] == -1);
3012	          auVar56[2] = -(pcVar3[2] == -1);
3013	          auVar56[3] = -(pcVar3[3] == -1);
3014	          auVar56[4] = -(pcVar3[4] == -1);
3015	          auVar56[5] = -(pcVar3[5] == -1);
3016	          auVar56[6] = -(pcVar3[6] == -1);
3017	          auVar56[7] = -(pcVar3[7] == -1);
3018	          auVar56[8] = -(pcVar3[8] == -1);
3019	          auVar56[9] = -(pcVar3[9] == -1);
3020	          auVar56[10] = -(pcVar3[10] == -1);
3021	          auVar56[0xb] = -(pcVar3[0xb] == -1);
3022	          auVar56[0xc] = -(pcVar3[0xc] == -1);
3023	          auVar56[0xd] = -(pcVar3[0xd] == -1);
3024	          auVar56[0xe] = -(pcVar3[0xe] == -1);
3025	          auVar56[0xf] = -(pcVar3[0xf] == -1);
3026	          if ((((((((((((((((SUB161(auVar56 >> 7,0) & 1) != 0 || (SUB161(auVar56 >> 0xf,0) & 1) != 0
3027	                           ) || (SUB161(auVar56 >> 0x17,0) & 1) != 0) ||
3028	                         (SUB161(auVar56 >> 0x1f,0) & 1) != 0) ||
3029	                        (SUB161(auVar56 >> 0x27,0) & 1) != 0) ||
3030	                       (SUB161(auVar56 >> 0x2f,0) & 1) != 0) || (SUB161(auVar56 >> 0x37,0) & 1) != 0
3031	                      ) || (SUB161(auVar56 >> 0x3f,0) & 1) != 0) ||
3032	                    (SUB161(auVar56 >> 0x47,0) & 1) != 0) || (SUB161(auVar56 >> 0x4f,0) & 1) != 0)
3033	                  || (SUB161(auVar56 >> 0x57,0) & 1) != 0) || (SUB161(auVar56 >> 0x5f,0) & 1) != 0)
3034	                || (SUB161(auVar56 >> 0x67,0) & 1) != 0) || (SUB161(auVar56 >> 0x6f,0) & 1) != 0) ||
3035	              (SUB161(auVar56 >> 0x77,0) & 1) != 0) || auVar56[0xf] < '\0') break;
3036	          uVar30 = uVar30 + lVar40 + 0x10;
3037	          lVar40 = lVar40 + 0x10;
3038	        }
3039	      }
3040	                    /* WARNING: Subroutine does not return */
3041	      core::option::unwrap_failed();
3042	    }
3043	    local_138 = (undefined1 (*) [16])*local_f0;
3044	    if (*(longlong *)(local_e8 + 0xd8) != 0) {
3045	      uStack_130._0_4_ = iVar100;
3046	      core::hash::BuildHasher::
3047	      hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
3048	                ();
3049	      lVar40 = *(longlong *)(local_e8 + 0xc0);

3050	      bVar42 = (byte)(extraout_RAX_38 >> 0x39);
3051	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
3052	      auVar46 = pshuflw(auVar46,auVar46,0);
3053	      lVar27 = 0;
3054	      uVar20 = extraout_RAX_38;
3055	      while( true ) {
3056	        uVar20 = *(ulonglong *)(local_e8 + 200) & uVar20;
3057	        auVar78 = *(undefined1 (*) [16])(lVar40 + uVar20);
3058	        cVar44 = auVar46[0];
3059	        auVar96[0] = -(auVar78[0] == cVar44);
3060	        cVar47 = auVar46[1];
3061	        auVar96[1] = -(auVar78[1] == cVar47);
3062	        cVar48 = auVar46[2];
3063	        auVar96[2] = -(auVar78[2] == cVar48);
3064	        cVar49 = auVar46[3];
3065	        auVar96[3] = -(auVar78[3] == cVar49);
3066	        auVar96[4] = -(auVar78[4] == cVar44);
3067	        auVar96[5] = -(auVar78[5] == cVar47);
3068	        auVar96[6] = -(auVar78[6] == cVar48);
3069	        auVar96[7] = -(auVar78[7] == cVar49);
3070	        auVar96[8] = -(auVar78[8] == cVar44);
3071	        auVar96[9] = -(auVar78[9] == cVar47);
3072	        auVar96[10] = -(auVar78[10] == cVar48);
3073	        auVar96[0xb] = -(auVar78[0xb] == cVar49);
3074	        auVar96[0xc] = -(auVar78[0xc] == cVar44);
3075	        auVar96[0xd] = -(auVar78[0xd] == cVar47);
3076	        auVar96[0xe] = -(auVar78[0xe] == cVar48);
3077	        auVar96[0xf] = -(auVar78[0xf] == cVar49);
3078	        uVar26 = (ushort)(SUB161(auVar96 >> 7,0) & 1) | (ushort)(SUB161(auVar96 >> 0xf,0) & 1) << 1
3079	                 | (ushort)(SUB161(auVar96 >> 0x17,0) & 1) << 2 |
3080	                 (ushort)(SUB161(auVar96 >> 0x1f,0) & 1) << 3 |
3081	                 (ushort)(SUB161(auVar96 >> 0x27,0) & 1) << 4 |
3082	                 (ushort)(SUB161(auVar96 >> 0x2f,0) & 1) << 5 |
3083	                 (ushort)(SUB161(auVar96 >> 0x37,0) & 1) << 6 |
3084	                 (ushort)(SUB161(auVar96 >> 0x3f,0) & 1) << 7 |
3085	                 (ushort)(SUB161(auVar96 >> 0x47,0) & 1) << 8 |
3086	                 (ushort)(SUB161(auVar96 >> 0x4f,0) & 1) << 9 |
3087	                 (ushort)(SUB161(auVar96 >> 0x57,0) & 1) << 10 |
3088	                 (ushort)(SUB161(auVar96 >> 0x5f,0) & 1) << 0xb |
3089	                 (ushort)(SUB161(auVar96 >> 0x67,0) & 1) << 0xc |
3090	                 (ushort)(SUB161(auVar96 >> 0x6f,0) & 1) << 0xd |
3091	                 (ushort)(SUB161(auVar96 >> 0x77,0) & 1) << 0xe | (ushort)(auVar96[0xf] >> 7) << 0xf
3092	        ;
3093	        uVar15 = (uint)uVar26;
3094	        while (uVar26 != 0) {
3095	          uVar23 = 0;
3096	          for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
3097	            uVar23 = uVar23 + 1;
3098	          }
3099	          uVar30 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 200);
3100	          uVar5 = *(undefined8 *)(lVar40 + -0x18 + uVar30 * -0x18);
3101	          auVar97._0_4_ = -(uint)((int)uVar5 == (int)local_138);
3102	          iVar101 = -(uint)((int)((ulonglong)uVar5 >> 0x20) == (int)((ulonglong)local_138 >> 0x20));
3103	          auVar97._4_4_ = auVar97._0_4_;
3104	          auVar97._8_4_ = iVar101;
3105	          auVar97._12_4_ = iVar101;
3106	          iVar101 = movmskpd(-(int)uVar30,auVar97);
3107	          if ((iVar101 == 3) && ((int)uStack_130 == *(int *)(lVar40 + -0x18 + uVar30 * -0x18 + 8)))
3108	          {
3109	            if (*(longlong *)(local_e8 + 0x108) == 0) goto LAB_1407d73b0;
3110	            core::hash::BuildHasher::
3111	            hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
3112	                      ();
3113	            lVar27 = *(longlong *)(local_e8 + 0xf0);
3114	            bVar42 = (byte)(extraout_RAX_39 >> 0x39);
3115	            auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
3116	            auVar46 = pshuflw(auVar46,auVar46,0);
3117	            lVar38 = 0;
3118	            uVar20 = extraout_RAX_39;
3119	            goto LAB_1407d72d8;
3120	          }
3121	          uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
3122	          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
3123	        }
3124	        auVar70[0] = -(auVar78[0] == -1);
3125	        auVar70[1] = -(auVar78[1] == -1);
3126	        auVar70[2] = -(auVar78[2] == -1);
3127	        auVar70[3] = -(auVar78[3] == -1);
3128	        auVar70[4] = -(auVar78[4] == -1);
3129	        auVar70[5] = -(auVar78[5] == -1);
3130	        auVar70[6] = -(auVar78[6] == -1);
3131	        auVar70[7] = -(auVar78[7] == -1);
3132	        auVar70[8] = -(auVar78[8] == -1);
3133	        auVar70[9] = -(auVar78[9] == -1);
3134	        auVar70[10] = -(auVar78[10] == -1);
3135	        auVar70[0xb] = -(auVar78[0xb] == -1);
3136	        auVar70[0xc] = -(auVar78[0xc] == -1);
3137	        auVar70[0xd] = -(auVar78[0xd] == -1);
3138	        auVar70[0xe] = -(auVar78[0xe] == -1);
3139	        auVar70[0xf] = -(auVar78[0xf] == -1);
3140	        if ((((((((((((((((SUB161(auVar70 >> 7,0) & 1) != 0 || (SUB161(auVar70 >> 0xf,0) & 1) != 0)
3141	                        || (SUB161(auVar70 >> 0x17,0) & 1) != 0) ||
3142	                       (SUB161(auVar70 >> 0x1f,0) & 1) != 0) || (SUB161(auVar70 >> 0x27,0) & 1) != 0
3143	                      ) || (SUB161(auVar70 >> 0x2f,0) & 1) != 0) ||
3144	                    (SUB161(auVar70 >> 0x37,0) & 1) != 0) || (SUB161(auVar70 >> 0x3f,0) & 1) != 0)
3145	                  || (SUB161(auVar70 >> 0x47,0) & 1) != 0) || (SUB161(auVar70 >> 0x4f,0) & 1) != 0)
3146	                || (SUB161(auVar70 >> 0x57,0) & 1) != 0) || (SUB161(auVar70 >> 0x5f,0) & 1) != 0) ||
3147	              (SUB161(auVar70 >> 0x67,0) & 1) != 0) || (SUB161(auVar70 >> 0x6f,0) & 1) != 0) ||
3148	            (SUB161(auVar70 >> 0x77,0) & 1) != 0) || auVar70[0xf] < '\0') break;
3149	        uVar20 = uVar20 + lVar27 + 0x10;
3150	        lVar27 = lVar27 + 0x10;
3151	      }
3152	    }
3153	LAB_1407d73b0:
3154	    local_138 = (undefined1 (*) [16])*local_f0;
3155	    uStack_130 = (undefined1 (*) [16])CONCAT44(uStack_130._4_4_,iVar100 + -1);
3156	    if (*(longlong *)(local_e8 + 0xd8) != 0) {
3157	      core::hash::BuildHasher::
3158	      hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
3159	                ();
3160	      lVar40 = *(longlong *)(local_e8 + 0xc0);
3161	      bVar42 = (byte)(extraout_RAX_40 >> 0x39);
3162	      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
3163	      auVar46 = pshuflw(auVar46,auVar46,0);
3164	      lVar27 = 0;
3165	      uVar20 = extraout_RAX_40;
3166	      while( true ) {
3167	        uVar20 = *(ulonglong *)(local_e8 + 200) & uVar20;
3168	        auVar78 = *(undefined1 (*) [16])(lVar40 + uVar20);
3169	        cVar44 = auVar46[0];
3170	        auVar98[0] = -(auVar78[0] == cVar44);
3171	        cVar47 = auVar46[1];
3172	        auVar98[1] = -(auVar78[1] == cVar47);
3173	        cVar48 = auVar46[2];
3174	        auVar98[2] = -(auVar78[2] == cVar48);
3175	        cVar49 = auVar46[3];
3176	        auVar98[3] = -(auVar78[3] == cVar49);
3177	        auVar98[4] = -(auVar78[4] == cVar44);
3178	        auVar98[5] = -(auVar78[5] == cVar47);
3179	        auVar98[6] = -(auVar78[6] == cVar48);
3180	        auVar98[7] = -(auVar78[7] == cVar49);
3181	        auVar98[8] = -(auVar78[8] == cVar44);
3182	        auVar98[9] = -(auVar78[9] == cVar47);
3183	        auVar98[10] = -(auVar78[10] == cVar48);
3184	        auVar98[0xb] = -(auVar78[0xb] == cVar49);
3185	        auVar98[0xc] = -(auVar78[0xc] == cVar44);
3186	        auVar98[0xd] = -(auVar78[0xd] == cVar47);
3187	        auVar98[0xe] = -(auVar78[0xe] == cVar48);
3188	        auVar98[0xf] = -(auVar78[0xf] == cVar49);
3189	        uVar26 = (ushort)(SUB161(auVar98 >> 7,0) & 1) | (ushort)(SUB161(auVar98 >> 0xf,0) & 1) << 1
3190	                 | (ushort)(SUB161(auVar98 >> 0x17,0) & 1) << 2 |
3191	                 (ushort)(SUB161(auVar98 >> 0x1f,0) & 1) << 3 |
3192	                 (ushort)(SUB161(auVar98 >> 0x27,0) & 1) << 4 |
3193	                 (ushort)(SUB161(auVar98 >> 0x2f,0) & 1) << 5 |
3194	                 (ushort)(SUB161(auVar98 >> 0x37,0) & 1) << 6 |
3195	                 (ushort)(SUB161(auVar98 >> 0x3f,0) & 1) << 7 |
3196	                 (ushort)(SUB161(auVar98 >> 0x47,0) & 1) << 8 |
3197	                 (ushort)(SUB161(auVar98 >> 0x4f,0) & 1) << 9 |
3198	                 (ushort)(SUB161(auVar98 >> 0x57,0) & 1) << 10 |
3199	                 (ushort)(SUB161(auVar98 >> 0x5f,0) & 1) << 0xb |
3200	                 (ushort)(SUB161(auVar98 >> 0x67,0) & 1) << 0xc |
3201	                 (ushort)(SUB161(auVar98 >> 0x6f,0) & 1) << 0xd |
3202	                 (ushort)(SUB161(auVar98 >> 0x77,0) & 1) << 0xe | (ushort)(auVar98[0xf] >> 7) << 0xf
3203	        ;
3204	        uVar15 = (uint)uVar26;
3205	        while (uVar26 != 0) {
3206	          uVar23 = 0;
3207	          for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
3208	            uVar23 = uVar23 + 1;
3209	          }
3210	          uVar30 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 200);
3211	          uVar5 = *(undefined8 *)(lVar40 + -0x18 + uVar30 * -0x18);
3212	          auVar99._0_4_ = -(uint)((int)uVar5 == (int)local_138);
3213	          iVar101 = -(uint)((int)((ulonglong)uVar5 >> 0x20) == (int)((ulonglong)local_138 >> 0x20));
3214	          auVar99._4_4_ = auVar99._0_4_;
3215	          auVar99._8_4_ = iVar101;
3216	          auVar99._12_4_ = iVar101;
3217	          iVar101 = movmskpd(-(int)uVar30,auVar99);
3218	          if ((iVar101 == 3) && ((int)uStack_130 == *(int *)(lVar40 + -0x18 + uVar30 * -0x18 + 8)))
3219	          {
3220	            if (*(longlong *)(local_e8 + 0x108) == 0) goto LAB_1407d7600;
3221	            core::hash::BuildHasher::
3222	            hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
3223	                      ();
3224	            lVar27 = *(longlong *)(local_e8 + 0xf0);
3225	            bVar42 = (byte)(extraout_RAX_41 >> 0x39);
3226	            auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
3227	            auVar46 = pshuflw(auVar46,auVar46,0);
3228	            lVar38 = 0;
3229	            uVar20 = extraout_RAX_41;
3230	            goto LAB_1407d7528;
3231	          }
3232	          uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
3233	          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
3234	        }
3235	        auVar73[0] = -(auVar78[0] == -1);
3236	        auVar73[1] = -(auVar78[1] == -1);
3237	        auVar73[2] = -(auVar78[2] == -1);
3238	        auVar73[3] = -(auVar78[3] == -1);
3239	        auVar73[4] = -(auVar78[4] == -1);
3240	        auVar73[5] = -(auVar78[5] == -1);
3241	        auVar73[6] = -(auVar78[6] == -1);
3242	        auVar73[7] = -(auVar78[7] == -1);
3243	        auVar73[8] = -(auVar78[8] == -1);
3244	        auVar73[9] = -(auVar78[9] == -1);
3245	        auVar73[10] = -(auVar78[10] == -1);
3246	        auVar73[0xb] = -(auVar78[0xb] == -1);
3247	        auVar73[0xc] = -(auVar78[0xc] == -1);
3248	        auVar73[0xd] = -(auVar78[0xd] == -1);
3249	        auVar73[0xe] = -(auVar78[0xe] == -1);

3250	        auVar73[0xf] = -(auVar78[0xf] == -1);
3251	        if ((((((((((((((((SUB161(auVar73 >> 7,0) & 1) != 0 || (SUB161(auVar73 >> 0xf,0) & 1) != 0)
3252	                        || (SUB161(auVar73 >> 0x17,0) & 1) != 0) ||
3253	                       (SUB161(auVar73 >> 0x1f,0) & 1) != 0) || (SUB161(auVar73 >> 0x27,0) & 1) != 0
3254	                      ) || (SUB161(auVar73 >> 0x2f,0) & 1) != 0) ||
3255	                    (SUB161(auVar73 >> 0x37,0) & 1) != 0) || (SUB161(auVar73 >> 0x3f,0) & 1) != 0)
3256	                  || (SUB161(auVar73 >> 0x47,0) & 1) != 0) || (SUB161(auVar73 >> 0x4f,0) & 1) != 0)
3257	                || (SUB161(auVar73 >> 0x57,0) & 1) != 0) || (SUB161(auVar73 >> 0x5f,0) & 1) != 0) ||
3258	              (SUB161(auVar73 >> 0x67,0) & 1) != 0) || (SUB161(auVar73 >> 0x6f,0) & 1) != 0) ||
3259	            (SUB161(auVar73 >> 0x77,0) & 1) != 0) || auVar73[0xf] < '\0') break;
3260	        uVar20 = uVar20 + lVar27 + 0x10;
3261	        lVar27 = lVar27 + 0x10;
3262	      }
3263	    }
3264	LAB_1407d7600:
3265	    fVar103 = 0.0;
3266	    if (0.45 <= *pfVar18) {
3267	      fVar50 = (*pfVar18 + -0.45) / 0.55;
3268	      if (0.0 <= fVar50) {
3269	        fVar103 = fVar50;
3270	      }
3271	      fVar50 = 1.0;
3272	      if (fVar103 <= 1.0) {
3273	        fVar50 = fVar103;
3274	      }
3275	      fVar103 = (3.0 - (fVar50 + fVar50)) * fVar50 * fVar50 * 0.3 + 0.7;
3276	    }
3277	    iVar100 = iVar100 + 1;
3278	    *pfVar18 = fVar103;
3279	    pfVar18 = pfVar18 + 1;
3280	  } while( true );
3281	  while( true ) {
3282	    fVar103 = *(float *)((longlong)_Src + uVar30 * 4);
3283	    pfVar18 = (float *)(*(longlong *)(lVar27 + -0x20 + lVar38) + uVar30 * 4);
3284	    uVar30 = uVar30 + 1;
3285	    if ((fVar103 != *pfVar18) || (NAN(fVar103) || NAN(*pfVar18))) break;
3286	LAB_1407d7770:
3287	    if (uVar20 == uVar30) {
3288	      uVar15 = (uint)(byte)local_198;
3289	      pauVar21 = local_178;
3290	      goto LAB_1407d6e54;
3291	    }
3292	  }
3293	LAB_1407d778b:
3294	  lVar27 = lVar27 + lVar38;
3295	  local_108[0x14][8] = 1;
3296	  *(undefined8 *)(lVar27 + -0x18) = 0;
3297	  if (*(ulonglong *)(lVar27 + -0x28) < uVar20) {
3298	    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
3299	    lVar40 = *(longlong *)(lVar27 + -0x18);
3300	  }
3301	  else {
3302	    lVar40 = 0;
3303	  }
3304	  memcpy((void *)(lVar40 * 4 + *(longlong *)(lVar27 + -0x20)),_Src,uVar20 * 4);
3305	  *(ulonglong *)(lVar27 + -0x18) = lVar40 + uVar20;
3306	  uVar15 = (uint)(byte)local_198;
3307	  pauVar21 = local_178;
3308	  goto LAB_1407d6e54;
3309	LAB_1407d72d8:
3310	  uVar20 = *(ulonglong *)(local_e8 + 0xf8) & uVar20;
3311	  pcVar3 = (char *)(lVar27 + uVar20);
3312	  cVar44 = auVar46[0];
3313	  auVar71[0] = -(*pcVar3 == cVar44);
3314	  cVar47 = auVar46[1];
3315	  auVar71[1] = -(pcVar3[1] == cVar47);
3316	  cVar48 = auVar46[2];
3317	  auVar71[2] = -(pcVar3[2] == cVar48);
3318	  cVar49 = auVar46[3];
3319	  auVar71[3] = -(pcVar3[3] == cVar49);
3320	  auVar71[4] = -(pcVar3[4] == cVar44);
3321	  auVar71[5] = -(pcVar3[5] == cVar47);
3322	  auVar71[6] = -(pcVar3[6] == cVar48);
3323	  auVar71[7] = -(pcVar3[7] == cVar49);
3324	  auVar71[8] = -(pcVar3[8] == cVar44);
3325	  auVar71[9] = -(pcVar3[9] == cVar47);
3326	  auVar71[10] = -(pcVar3[10] == cVar48);
3327	  auVar71[0xb] = -(pcVar3[0xb] == cVar49);
3328	  auVar71[0xc] = -(pcVar3[0xc] == cVar44);
3329	  auVar71[0xd] = -(pcVar3[0xd] == cVar47);
3330	  auVar71[0xe] = -(pcVar3[0xe] == cVar48);
3331	  auVar71[0xf] = -(pcVar3[0xf] == cVar49);
3332	  uVar26 = (ushort)(SUB161(auVar71 >> 7,0) & 1) | (ushort)(SUB161(auVar71 >> 0xf,0) & 1) << 1 |
3333	           (ushort)(SUB161(auVar71 >> 0x17,0) & 1) << 2 |
3334	           (ushort)(SUB161(auVar71 >> 0x1f,0) & 1) << 3 |
3335	           (ushort)(SUB161(auVar71 >> 0x27,0) & 1) << 4 |
3336	           (ushort)(SUB161(auVar71 >> 0x2f,0) & 1) << 5 |
3337	           (ushort)(SUB161(auVar71 >> 0x37,0) & 1) << 6 |
3338	           (ushort)(SUB161(auVar71 >> 0x3f,0) & 1) << 7 |
3339	           (ushort)(SUB161(auVar71 >> 0x47,0) & 1) << 8 |
3340	           (ushort)(SUB161(auVar71 >> 0x4f,0) & 1) << 9 |
3341	           (ushort)(SUB161(auVar71 >> 0x57,0) & 1) << 10 |
3342	           (ushort)(SUB161(auVar71 >> 0x5f,0) & 1) << 0xb |
3343	           (ushort)(SUB161(auVar71 >> 0x67,0) & 1) << 0xc |
3344	           (ushort)(SUB161(auVar71 >> 0x6f,0) & 1) << 0xd |
3345	           (ushort)(SUB161(auVar71 >> 0x77,0) & 1) << 0xe | (ushort)(auVar71[0xf] >> 7) << 0xf;
3346	  uVar15 = (uint)uVar26;
3347	  if (uVar26 != 0) {
3348	    do {
3349	      uVar23 = 0;
3350	      for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
3351	        uVar23 = uVar23 + 1;
3352	      }
3353	      uVar32 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 0xf8);
3354	      puVar29 = (undefined8 *)(lVar27 + -0x10 + uVar32 * -0x10);
3355	      if ((*(char *)(lVar40 + uVar30 * -0x18 + -4) == *(char *)(puVar29 + 1)) &&
3356	         (uVar5 = *(undefined8 *)(lVar40 + uVar30 * -0x18 + -0xc),
3357	         iVar101 = -(uint)((int)*puVar29 == (int)uVar5),
3358	         iVar102 = -(uint)((int)((ulonglong)*puVar29 >> 0x20) == (int)((ulonglong)uVar5 >> 0x20)),
3359	         auVar72._4_4_ = iVar101, auVar72._0_4_ = iVar101, auVar72._8_4_ = iVar102,
3360	         auVar72._12_4_ = iVar102, iVar101 = movmskpd((int)puVar29,auVar72), iVar101 == 3)) {
3361	        fVar103 = *(float *)(lVar27 + -4 + uVar32 * -0x10);
3362	        fVar50 = *pfVar18;
3363	        fVar4 = fVar103;
3364	        if (fVar50 <= fVar103) {
3365	          fVar4 = fVar50;
3366	        }
3367	        *pfVar18 = (float)(~-(uint)NAN(fVar50) & (uint)fVar4 | -(uint)NAN(fVar50) & (uint)fVar103);
3368	        goto LAB_1407d73b0;
3369	      }
3370	      uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
3371	      uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
3372	    } while (uVar26 != 0);
3373	  }
3374	  auVar54[0] = -(*pcVar3 == -1);
3375	  auVar54[1] = -(pcVar3[1] == -1);
3376	  auVar54[2] = -(pcVar3[2] == -1);
3377	  auVar54[3] = -(pcVar3[3] == -1);
3378	  auVar54[4] = -(pcVar3[4] == -1);
3379	  auVar54[5] = -(pcVar3[5] == -1);
3380	  auVar54[6] = -(pcVar3[6] == -1);
3381	  auVar54[7] = -(pcVar3[7] == -1);
3382	  auVar54[8] = -(pcVar3[8] == -1);
3383	  auVar54[9] = -(pcVar3[9] == -1);
3384	  auVar54[10] = -(pcVar3[10] == -1);
3385	  auVar54[0xb] = -(pcVar3[0xb] == -1);
3386	  auVar54[0xc] = -(pcVar3[0xc] == -1);
3387	  auVar54[0xd] = -(pcVar3[0xd] == -1);
3388	  auVar54[0xe] = -(pcVar3[0xe] == -1);
3389	  auVar54[0xf] = -(pcVar3[0xf] == -1);
3390	  if ((((((((((((((((SUB161(auVar54 >> 7,0) & 1) != 0 || (SUB161(auVar54 >> 0xf,0) & 1) != 0) ||
3391	                  (SUB161(auVar54 >> 0x17,0) & 1) != 0) || (SUB161(auVar54 >> 0x1f,0) & 1) != 0) ||
3392	                (SUB161(auVar54 >> 0x27,0) & 1) != 0) || (SUB161(auVar54 >> 0x2f,0) & 1) != 0) ||
3393	              (SUB161(auVar54 >> 0x37,0) & 1) != 0) || (SUB161(auVar54 >> 0x3f,0) & 1) != 0) ||
3394	            (SUB161(auVar54 >> 0x47,0) & 1) != 0) || (SUB161(auVar54 >> 0x4f,0) & 1) != 0) ||
3395	          (SUB161(auVar54 >> 0x57,0) & 1) != 0) || (SUB161(auVar54 >> 0x5f,0) & 1) != 0) ||
3396	        (SUB161(auVar54 >> 0x67,0) & 1) != 0) || (SUB161(auVar54 >> 0x6f,0) & 1) != 0) ||
3397	      (SUB161(auVar54 >> 0x77,0) & 1) != 0) || auVar54[0xf] < '\0') goto LAB_1407d73b0;
3398	  uVar20 = uVar20 + lVar38 + 0x10;
3399	  lVar38 = lVar38 + 0x10;
3400	  goto LAB_1407d72d8;
3401	LAB_1407d7528:
3402	  uVar20 = *(ulonglong *)(local_e8 + 0xf8) & uVar20;
3403	  pcVar3 = (char *)(lVar27 + uVar20);
3404	  cVar44 = auVar46[0];
3405	  auVar74[0] = -(*pcVar3 == cVar44);
3406	  cVar47 = auVar46[1];
3407	  auVar74[1] = -(pcVar3[1] == cVar47);
3408	  cVar48 = auVar46[2];
3409	  auVar74[2] = -(pcVar3[2] == cVar48);
3410	  cVar49 = auVar46[3];
3411	  auVar74[3] = -(pcVar3[3] == cVar49);
3412	  auVar74[4] = -(pcVar3[4] == cVar44);
3413	  auVar74[5] = -(pcVar3[5] == cVar47);
3414	  auVar74[6] = -(pcVar3[6] == cVar48);
3415	  auVar74[7] = -(pcVar3[7] == cVar49);
3416	  auVar74[8] = -(pcVar3[8] == cVar44);
3417	  auVar74[9] = -(pcVar3[9] == cVar47);
3418	  auVar74[10] = -(pcVar3[10] == cVar48);
3419	  auVar74[0xb] = -(pcVar3[0xb] == cVar49);
3420	  auVar74[0xc] = -(pcVar3[0xc] == cVar44);
3421	  auVar74[0xd] = -(pcVar3[0xd] == cVar47);
3422	  auVar74[0xe] = -(pcVar3[0xe] == cVar48);
3423	  auVar74[0xf] = -(pcVar3[0xf] == cVar49);
3424	  uVar26 = (ushort)(SUB161(auVar74 >> 7,0) & 1) | (ushort)(SUB161(auVar74 >> 0xf,0) & 1) << 1 |
3425	           (ushort)(SUB161(auVar74 >> 0x17,0) & 1) << 2 |
3426	           (ushort)(SUB161(auVar74 >> 0x1f,0) & 1) << 3 |
3427	           (ushort)(SUB161(auVar74 >> 0x27,0) & 1) << 4 |
3428	           (ushort)(SUB161(auVar74 >> 0x2f,0) & 1) << 5 |
3429	           (ushort)(SUB161(auVar74 >> 0x37,0) & 1) << 6 |
3430	           (ushort)(SUB161(auVar74 >> 0x3f,0) & 1) << 7 |
3431	           (ushort)(SUB161(auVar74 >> 0x47,0) & 1) << 8 |
3432	           (ushort)(SUB161(auVar74 >> 0x4f,0) & 1) << 9 |
3433	           (ushort)(SUB161(auVar74 >> 0x57,0) & 1) << 10 |
3434	           (ushort)(SUB161(auVar74 >> 0x5f,0) & 1) << 0xb |
3435	           (ushort)(SUB161(auVar74 >> 0x67,0) & 1) << 0xc |
3436	           (ushort)(SUB161(auVar74 >> 0x6f,0) & 1) << 0xd |
3437	           (ushort)(SUB161(auVar74 >> 0x77,0) & 1) << 0xe | (ushort)(auVar74[0xf] >> 7) << 0xf;
3438	  uVar15 = (uint)uVar26;
3439	  if (uVar26 != 0) {
3440	    do {
3441	      uVar23 = 0;
3442	      for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
3443	        uVar23 = uVar23 + 1;
3444	      }
3445	      uVar32 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 0xf8);
3446	      puVar29 = (undefined8 *)(lVar27 + -0x10 + uVar32 * -0x10);
3447	      if ((*(char *)(lVar40 + uVar30 * -0x18 + -4) == *(char *)(puVar29 + 1)) &&
3448	         (uVar5 = *(undefined8 *)(lVar40 + uVar30 * -0x18 + -0xc),
3449	         iVar101 = -(uint)((int)*puVar29 == (int)uVar5),

3450	         iVar102 = -(uint)((int)((ulonglong)*puVar29 >> 0x20) == (int)((ulonglong)uVar5 >> 0x20)),
3451	         auVar75._4_4_ = iVar101, auVar75._0_4_ = iVar101, auVar75._8_4_ = iVar102,
3452	         auVar75._12_4_ = iVar102, iVar101 = movmskpd((int)puVar29,auVar75), iVar101 == 3)) {
3453	        fVar103 = *(float *)(lVar27 + -4 + uVar32 * -0x10);
3454	        fVar50 = *pfVar18;
3455	        fVar4 = fVar103;
3456	        if (fVar50 <= fVar103) {
3457	          fVar4 = fVar50;
3458	        }
3459	        *pfVar18 = (float)(~-(uint)NAN(fVar50) & (uint)fVar4 | -(uint)NAN(fVar50) & (uint)fVar103);
3460	        goto LAB_1407d7600;
3461	      }
3462	      uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
3463	      uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
3464	    } while (uVar26 != 0);
3465	  }
3466	  auVar55[0] = -(*pcVar3 == -1);
3467	  auVar55[1] = -(pcVar3[1] == -1);
3468	  auVar55[2] = -(pcVar3[2] == -1);
3469	  auVar55[3] = -(pcVar3[3] == -1);
3470	  auVar55[4] = -(pcVar3[4] == -1);
3471	  auVar55[5] = -(pcVar3[5] == -1);
3472	  auVar55[6] = -(pcVar3[6] == -1);
3473	  auVar55[7] = -(pcVar3[7] == -1);
3474	  auVar55[8] = -(pcVar3[8] == -1);
3475	  auVar55[9] = -(pcVar3[9] == -1);
3476	  auVar55[10] = -(pcVar3[10] == -1);
3477	  auVar55[0xb] = -(pcVar3[0xb] == -1);
3478	  auVar55[0xc] = -(pcVar3[0xc] == -1);
3479	  auVar55[0xd] = -(pcVar3[0xd] == -1);
3480	  auVar55[0xe] = -(pcVar3[0xe] == -1);
3481	  auVar55[0xf] = -(pcVar3[0xf] == -1);
3482	  if ((((((((((((((((SUB161(auVar55 >> 7,0) & 1) != 0 || (SUB161(auVar55 >> 0xf,0) & 1) != 0) ||
3483	                  (SUB161(auVar55 >> 0x17,0) & 1) != 0) || (SUB161(auVar55 >> 0x1f,0) & 1) != 0) ||
3484	                (SUB161(auVar55 >> 0x27,0) & 1) != 0) || (SUB161(auVar55 >> 0x2f,0) & 1) != 0) ||
3485	              (SUB161(auVar55 >> 0x37,0) & 1) != 0) || (SUB161(auVar55 >> 0x3f,0) & 1) != 0) ||
3486	            (SUB161(auVar55 >> 0x47,0) & 1) != 0) || (SUB161(auVar55 >> 0x4f,0) & 1) != 0) ||
3487	          (SUB161(auVar55 >> 0x57,0) & 1) != 0) || (SUB161(auVar55 >> 0x5f,0) & 1) != 0) ||
3488	        (SUB161(auVar55 >> 0x67,0) & 1) != 0) || (SUB161(auVar55 >> 0x6f,0) & 1) != 0) ||
3489	      (SUB161(auVar55 >> 0x77,0) & 1) != 0) || auVar55[0xf] < '\0') goto LAB_1407d7600;
3490	  uVar20 = uVar20 + lVar38 + 0x10;
3491	  lVar38 = lVar38 + 0x10;
3492	  goto LAB_1407d7528;
3493	}

