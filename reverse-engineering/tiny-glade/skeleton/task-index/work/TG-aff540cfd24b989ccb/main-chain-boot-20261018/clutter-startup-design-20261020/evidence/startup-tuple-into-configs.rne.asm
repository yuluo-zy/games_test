141cb2410: 55 push rbp
141cb2411: 56 push rsi
141cb2412: 57 push rdi
141cb2413: 53 push rbx
141cb2414: 4881eca8020000 sub rsp, 0x2a8
141cb241b: 488dac2480000000 lea rbp, [rsp + 0x80]
141cb2423: 48c78520020000feffffff mov qword ptr [rbp + 0x220], 0xfffffffffffffffe
141cb242e: 4889ce mov rsi, rcx
141cb2431: 0fb605d1279801 movzx eax, byte ptr [rip + 0x19827d1]
141cb2438: b950010000 mov ecx, 0x150
141cb243d: ba08000000 mov edx, 8
141cb2442: e8c91796fe call 0x140613c10
141cb2447: 4885c0 test rax, rax
141cb244a: 0f84c7020000 je 0x141cb2717
141cb2450: 48898518020000 mov qword ptr [rbp + 0x218], rax
141cb2457: 0f57c0 xorps xmm0, xmm0
141cb245a: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
141cb2461: 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
141cb2468: 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
141cb246f: 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
141cb2476: 0f118508010000 movups xmmword ptr [rbp + 0x108], xmm0
141cb247d: c7851801000000000000 mov dword ptr [rbp + 0x118], 0
141cb2487: 0f118540010000 movups xmmword ptr [rbp + 0x140], xmm0
141cb248e: 0f118558010000 movups xmmword ptr [rbp + 0x158], xmm0
141cb2495: 0f118570010000 movups xmmword ptr [rbp + 0x170], xmm0
141cb249c: 0f118588010000 movups xmmword ptr [rbp + 0x188], xmm0
141cb24a3: 0f1185a0010000 movups xmmword ptr [rbp + 0x1a0], xmm0
141cb24aa: c785b001000000000000 mov dword ptr [rbp + 0x1b0], 0
141cb24b4: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
141cb24bf: 48c7859000000008000000 mov qword ptr [rbp + 0x90], 8
141cb24ca: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
141cb24d5: 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
141cb24e0: 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
141cb24eb: 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
141cb24f6: 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
141cb2501: 48c7850001000010000000 mov qword ptr [rbp + 0x100], 0x10
141cb250c: 48bb0000000000000080 movabs rbx, 0x8000000000000000
141cb2516: 48899d20010000 mov qword ptr [rbp + 0x120], rbx
141cb251d: 488d05ac942701 lea rax, [rip + 0x12794ac]
141cb2524: 48898528010000 mov qword ptr [rbp + 0x128], rax
141cb252b: 48c7853001000045000000 mov qword ptr [rbp + 0x130], 0x45
141cb2536: 48c7853801000010000000 mov qword ptr [rbp + 0x138], 0x10
141cb2541: 48c7855001000010000000 mov qword ptr [rbp + 0x150], 0x10
141cb254c: 48c7856801000010000000 mov qword ptr [rbp + 0x168], 0x10
141cb2557: 48c7858001000010000000 mov qword ptr [rbp + 0x180], 0x10
141cb2562: 48c7859801000010000000 mov qword ptr [rbp + 0x198], 0x10
141cb256d: c785b801000000000000 mov dword ptr [rbp + 0x1b8], 0
141cb2577: 66c785bc0100000100 mov word ptr [rbp + 0x1bc], 1
141cb2580: 48899dd0010000 mov qword ptr [rbp + 0x1d0], rbx
141cb2587: c7851002000000000000 mov dword ptr [rbp + 0x210], 0
141cb2591: 0fb60571269801 movzx eax, byte ptr [rip + 0x1982671]
141cb2598: b990010000 mov ecx, 0x190
141cb259d: ba08000000 mov edx, 8
141cb25a2: e8691696fe call 0x140613c10
141cb25a7: 4885c0 test rax, rax
141cb25aa: 0f8476010000 je 0x141cb2726
141cb25b0: 4889c7 mov rdi, rax
141cb25b3: 488d9588000000 lea rdx, [rbp + 0x88]
141cb25ba: 41b890010000 mov r8d, 0x190
141cb25c0: 4889c1 mov rcx, rax
141cb25c3: e83816c700 call 0x142923c00
141cb25c8: 4c8d0549932701 lea r8, [rip + 0x1279349]
141cb25cf: 488d4d18 lea rcx, [rbp + 0x18]
141cb25d3: 4889fa mov rdx, rdi
141cb25d6: e8352503ff call 0x140ce4b10
141cb25db: 488d4da8 lea rcx, [rbp - 0x58]
141cb25df: e89cb90300 call 0x141cedf80
141cb25e4: 488d8d88000000 lea rcx, [rbp + 0x88]
141cb25eb: e8700e6100 call 0x1422c3460
141cb25f0: 0f104578 movups xmm0, xmmword ptr [rbp + 0x78]
141cb25f4: 488b8518020000 mov rax, qword ptr [rbp + 0x218]
141cb25fb: 0f114060 movups xmmword ptr [rax + 0x60], xmm0
141cb25ff: 0f104568 movups xmm0, xmmword ptr [rbp + 0x68]
141cb2603: 0f114050 movups xmmword ptr [rax + 0x50], xmm0
141cb2607: 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
141cb260b: 0f114040 movups xmmword ptr [rax + 0x40], xmm0
141cb260f: 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
141cb2613: 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
141cb2617: 0f105538 movups xmm2, xmmword ptr [rbp + 0x38]
141cb261b: 0f105d48 movups xmm3, xmmword ptr [rbp + 0x48]
141cb261f: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
141cb2623: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
141cb2627: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
141cb262b: 0f1100 movups xmmword ptr [rax], xmm0
141cb262e: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
141cb2632: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
141cb2636: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
141cb263a: 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
141cb263e: 0f114070 movups xmmword ptr [rax + 0x70], xmm0
141cb2642: 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
141cb2649: 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
141cb2650: 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
141cb2657: 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
141cb265b: 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
141cb2662: 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
141cb2666: 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
141cb266d: 0f104508 movups xmm0, xmmword ptr [rbp + 8]
141cb2671: 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
141cb2678: 0f108588000000 movups xmm0, xmmword ptr [rbp + 0x88]
141cb267f: 0f108d98000000 movups xmm1, xmmword ptr [rbp + 0x98]
141cb2686: 0f1095a8000000 movups xmm2, xmmword ptr [rbp + 0xa8]
141cb268d: 0f109db8000000 movups xmm3, xmmword ptr [rbp + 0xb8]
141cb2694: 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
141cb269b: 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
141cb26a2: 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
141cb26a9: 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
141cb26b0: 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
141cb26b7: 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
141cb26be: 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
141cb26c5: 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
141cb26cc: 0f1085e8000000 movups xmm0, xmmword ptr [rbp + 0xe8]
141cb26d3: 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
141cb26da: 48c7460803000000 mov qword ptr [rsi + 8], 3
141cb26e2: 48894610 mov qword ptr [rsi + 0x10], rax
141cb26e6: 48c7461803000000 mov qword ptr [rsi + 0x18], 3
141cb26ee: 48c7462000000000 mov qword ptr [rsi + 0x20], 0
141cb26f6: 48c7462808000000 mov qword ptr [rsi + 0x28], 8
141cb26fe: 0f57c0 xorps xmm0, xmm0
141cb2701: 0f114630 movups xmmword ptr [rsi + 0x30], xmm0
141cb2705: 48891e mov qword ptr [rsi], rbx
141cb2708: 4889f0 mov rax, rsi
141cb270b: 4881c4a8020000 add rsp, 0x2a8
141cb2712: 5b pop rbx
141cb2713: 5f pop rdi
141cb2714: 5e pop rsi
141cb2715: 5d pop rbp
141cb2716: c3 ret 
141cb2717: b908000000 mov ecx, 8
141cb271c: ba50010000 mov edx, 0x150
141cb2721: e8da68c200 call 0x1428d9000
141cb2726: b908000000 mov ecx, 8
141cb272b: ba90010000 mov edx, 0x190
141cb2730: e8cb68c200 call 0x1428d9000
141cb2735: 0f0b ud2 
141cb2737: 660f1f840000000000 nop word ptr [rax + rax]
141cb2740: 4889542410 mov qword ptr [rsp + 0x10], rdx
141cb2745: 55 push rbp
141cb2746: 56 push rsi
141cb2747: 57 push rdi
141cb2748: 53 push rbx
141cb2749: 4883ec28 sub rsp, 0x28
141cb274d: 488daa80000000 lea rbp, [rdx + 0x80]
141cb2754: 488d8d88000000 lea rcx, [rbp + 0x88]
141cb275b: e8303d93fe call 0x1405e6490
141cb2760: 90 nop 
141cb2761: 4883c428 add rsp, 0x28
141cb2765: 5b pop rbx
141cb2766: 5f pop rdi
141cb2767: 5e pop rsi
141cb2768: 5d pop rbp
141cb2769: c3 ret 
141cb276a: 660f1f440000 nop word ptr [rax + rax]
141cb2770: 4889542410 mov qword ptr [rsp + 0x10], rdx
141cb2775: 55 push rbp
141cb2776: 56 push rsi
141cb2777: 57 push rdi
141cb2778: 53 push rbx
141cb2779: 4883ec28 sub rsp, 0x28
141cb277d: 488daa80000000 lea rbp, [rdx + 0x80]
141cb2784: ba50010000 mov edx, 0x150
141cb2789: 41b808000000 mov r8d, 8
141cb278f: 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
141cb2796: e8851496fe call 0x140613c20
141cb279b: 90 nop 
141cb279c: 4883c428 add rsp, 0x28
141cb27a0: 5b pop rbx
141cb27a1: 5f pop rdi
141cb27a2: 5e pop rsi
141cb27a3: 5d pop rbp
141cb27a4: c3 ret 
141cb27a5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
141cb27b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141cb27b5: 55 push rbp
141cb27b6: 56 push rsi
141cb27b7: 57 push rdi
141cb27b8: 53 push rbx
141cb27b9: 4883ec28 sub rsp, 0x28
141cb27bd: 488daa80000000 lea rbp, [rdx + 0x80]
141cb27c4: 488d4d18 lea rcx, [rbp + 0x18]
141cb27c8: e8b37037fe call 0x140029880
141cb27cd: 90 nop 
141cb27ce: 4883c428 add rsp, 0x28
141cb27d2: 5b pop rbx
141cb27d3: 5f pop rdi
141cb27d4: 5e pop rsi
141cb27d5: 5d pop rbp
141cb27d6: c3 ret 
141cb27d7: 660f1f840000000000 nop word ptr [rax + rax]
141cb27e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141cb27e5: 55 push rbp
141cb27e6: 56 push rsi
141cb27e7: 57 push rdi
141cb27e8: 53 push rbx
141cb27e9: 4883ec28 sub rsp, 0x28
141cb27ed: 488daa80000000 lea rbp, [rdx + 0x80]
141cb27f4: 488d4da8 lea rcx, [rbp - 0x58]
141cb27f8: e8837037fe call 0x140029880
141cb27fd: 90 nop 
141cb27fe: 4883c428 add rsp, 0x28
141cb2802: 5b pop rbx
141cb2803: 5f pop rdi
141cb2804: 5e pop rsi
141cb2805: 5d pop rbp
141cb2806: c3 ret 
