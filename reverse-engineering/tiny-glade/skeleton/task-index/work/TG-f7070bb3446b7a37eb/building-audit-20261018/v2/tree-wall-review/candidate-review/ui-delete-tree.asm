141bd2510: 55                           push rbp
141bd2511: 4157                         push r15
141bd2513: 4156                         push r14
141bd2515: 4155                         push r13
141bd2517: 4154                         push r12
141bd2519: 56                           push rsi
141bd251a: 57                           push rdi
141bd251b: 53                           push rbx
141bd251c: 4881ec38010000               sub rsp, 0x138
141bd2523: 488dac2480000000             lea rbp, [rsp + 0x80]
141bd252b: 440f2995a0000000             movaps xmmword ptr [rbp + 0xa0], xmm10
141bd2533: 440f298d90000000             movaps xmmword ptr [rbp + 0x90], xmm9
141bd253b: 440f298580000000             movaps xmmword ptr [rbp + 0x80], xmm8
141bd2543: 0f297d70                     movaps xmmword ptr [rbp + 0x70], xmm7
141bd2547: 0f297560                     movaps xmmword ptr [rbp + 0x60], xmm6
141bd254b: 48c74558feffffff             mov qword ptr [rbp + 0x58], 0xfffffffffffffffe
141bd2553: 488b053e9ba501               mov rax, qword ptr [rip + 0x1a59b3e]
141bd255a: 803800                       cmp byte ptr [rax], 0
141bd255d: 0f85a0050000                 jne 0x141bd2b03
141bd2563: 4c89c6                       mov rsi, r8
141bd2566: 488b8548010000               mov rax, qword ptr [rbp + 0x148]
141bd256d: 488b00                       mov rax, qword ptr [rax]
141bd2570: f30f1030                     movss xmm6, dword ptr [rax]
141bd2574: 488b01                       mov rax, qword ptr [rcx]
141bd2577: 31ff                         xor edi, edi
141bd2579: 833809                       cmp dword ptr [rax], 9
141bd257c: 7562                         jne 0x141bd25e0
141bd257e: 80780406                     cmp byte ptr [rax + 4], 6
141bd2582: 755c                         jne 0x141bd25e0
141bd2584: 4c89cb                       mov rbx, r9
141bd2587: 488b0a                       mov rcx, qword ptr [rdx]
141bd258a: c644242003                   mov byte ptr [rsp + 0x20], 3
141bd258f: 488d15533a3201               lea rdx, [rip + 0x1323a53]
141bd2596: 41b80a000000                 mov r8d, 0xa
141bd259c: 41b10a                       mov r9b, 0xa
141bd259f: e89cbdd5fe                   call 0x14092e340
141bd25a4: 488945e8                     mov qword ptr [rbp - 0x18], rax
141bd25a8: 488955f0                     mov qword ptr [rbp - 0x10], rdx
141bd25ac: 488d4de8                     lea rcx, [rbp - 0x18]
141bd25b0: e85bb9d5fe                   call 0x14092df10
141bd25b5: 3c02                         cmp al, 2
141bd25b7: 400f95c7                     setne dil
141bd25bb: 741e                         je 0x141bd25db
141bd25bd: 488b8530010000               mov rax, qword ptr [rbp + 0x130]
141bd25c4: 488b10                       mov rdx, qword ptr [rax]
141bd25c7: 488d4d10                     lea rcx, [rbp + 0x10]
141bd25cb: e88000e4fe                   call 0x140a12650
141bd25d0: f30f107d10                   movss xmm7, dword ptr [rbp + 0x10]
141bd25d5: f3440f104518                 movss xmm8, dword ptr [rbp + 0x18]
141bd25db: 4989d9                       mov r9, rbx
141bd25de: eb00                         jmp 0x141bd25e0
141bd25e0: 4c8bbd38010000               mov r15, qword ptr [rbp + 0x138]
141bd25e7: f30f593589aadf00             mulss xmm6, dword ptr [rip + 0xdfaa89]
141bd25ef: 488b8528010000               mov rax, qword ptr [rbp + 0x128]
141bd25f6: 498b09                       mov rcx, qword ptr [r9]
141bd25f9: 4d8b4908                     mov r9, qword ptr [r9 + 8]
141bd25fd: 4c8b31                       mov r14, qword ptr [rcx]
141bd2600: 31db                         xor ebx, ebx
141bd2602: 4d89f3                       mov r11, r14
141bd2605: 4d2b5918                     sub r11, qword ptr [r9 + 0x18]
141bd2609: 4c0f42db                     cmovb r11, rbx
141bd260d: 4d2b7138                     sub r14, qword ptr [r9 + 0x38]
141bd2611: 4c8ba520010000               mov r12, qword ptr [rbp + 0x120]
141bd2618: 4c0f42f3                     cmovb r14, rbx
141bd261c: 4d89da                       mov r10, r11
141bd261f: 49c1e205                     shl r10, 5
141bd2623: 4d035108                     add r10, qword ptr [r9 + 8]
141bd2627: 498b5110                     mov rdx, qword ptr [r9 + 0x10]
141bd262b: 4d8b4130                     mov r8, qword ptr [r9 + 0x30]
141bd262f: 4c29da                       sub rdx, r11
141bd2632: 480f42d3                     cmovb rdx, rbx
141bd2636: 41bd08000000                 mov r13d, 8
141bd263c: 4d0f42d5                     cmovb r10, r13
141bd2640: 4d89f3                       mov r11, r14
141bd2643: 49c1e305                     shl r11, 5
141bd2647: 4d035928                     add r11, qword ptr [r9 + 0x28]
141bd264b: 4d29f0                       sub r8, r14
141bd264e: 4c0f42c3                     cmovb r8, rbx
141bd2652: 4d0f42dd                     cmovb r11, r13
141bd2656: 4d8b4940                     mov r9, qword ptr [r9 + 0x40]
141bd265a: 4d29c1                       sub r9, r8
141bd265d: 4929d1                       sub r9, rdx
141bd2660: 4c8909                       mov qword ptr [rcx], r9
141bd2663: 48c1e205                     shl rdx, 5
141bd2667: 4c01d2                       add rdx, r10
141bd266a: 49c1e005                     shl r8, 5
141bd266e: 4d01d8                       add r8, r11
141bd2671: eb26                         jmp 0x141bd2699
141bd2673: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
141bd2680: f3410f107e08                 movss xmm7, dword ptr [r14 + 8]
141bd2686: f3450f104610                 movss xmm8, dword ptr [r14 + 0x10]
141bd268c: f3410f107614                 movss xmm6, dword ptr [r14 + 0x14]
141bd2692: f30f58f6                     addss xmm6, xmm6
141bd2696: 40b701                       mov dil, 1
141bd2699: 4c89db                       mov rbx, r11
141bd269c: 4d89d5                       mov r13, r10
141bd269f: eb29                         jmp 0x141bd26ca
141bd26a1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141bd26b0: 4d8d5520                     lea r10, [r13 + 0x20]
141bd26b4: 4989db                       mov r11, rbx
141bd26b7: 4d89ee                       mov r14, r13
141bd26ba: 49ffc1                       inc r9
141bd26bd: 4c8909                       mov qword ptr [rcx], r9
141bd26c0: 41807e1800                   cmp byte ptr [r14 + 0x18], 0
141bd26c5: 4d89d5                       mov r13, r10
141bd26c8: 74b6                         je 0x141bd2680
141bd26ca: 4d85ed                       test r13, r13
141bd26cd: 7405                         je 0x141bd26d4
141bd26cf: 4939d5                       cmp r13, rdx
141bd26d2: 75dc                         jne 0x141bd26b0
141bd26d4: 4885db                       test rbx, rbx
141bd26d7: 7414                         je 0x141bd26ed
141bd26d9: 4c39c3                       cmp rbx, r8
141bd26dc: 740f                         je 0x141bd26ed
141bd26de: 4c8d5b20                     lea r11, [rbx + 0x20]
141bd26e2: 4531d2                       xor r10d, r10d
141bd26e5: 4989de                       mov r14, rbx
141bd26e8: 4c89db                       mov rbx, r11
141bd26eb: ebcd                         jmp 0x141bd26ba
141bd26ed: 40f6c701                     test dil, 1
141bd26f1: 0f840c040000                 je 0x141bd2b03
141bd26f7: 488b10                       mov rdx, qword ptr [rax]
141bd26fa: 488d4de8                     lea rcx, [rbp - 0x18]
141bd26fe: e8fd89e4fe                   call 0x140a1b100
141bd2703: 4c8b5d08                     mov r11, qword ptr [rbp + 8]
141bd2707: 4d85db                       test r11, r11
141bd270a: 0f84f3030000                 je 0x141bd2b03
141bd2710: 4c8b2e                       mov r13, qword ptr [rsi]
141bd2713: 488b5e08                     mov rbx, qword ptr [rsi + 8]
141bd2717: 418b471c                     mov eax, dword ptr [r15 + 0x1c]
141bd271b: 894538                       mov dword ptr [rbp + 0x38], eax
141bd271e: 498b07                       mov rax, qword ptr [r15]
141bd2721: 488945d8                     mov qword ptr [rbp - 0x28], rax
141bd2725: 498b4710                     mov rax, qword ptr [r15 + 0x10]
141bd2729: 488945c0                     mov qword ptr [rbp - 0x40], rax
141bd272d: 488b7de8                     mov rdi, qword ptr [rbp - 0x18]
141bd2731: 488b75f0                     mov rsi, qword ptr [rbp - 0x10]
141bd2735: 418b44241c                   mov eax, dword ptr [r12 + 0x1c]
141bd273a: 89453c                       mov dword ptr [rbp + 0x3c], eax
141bd273d: 498b0424                     mov rax, qword ptr [r12]
141bd2741: 498b4c2410                   mov rcx, qword ptr [r12 + 0x10]
141bd2746: 48894dc8                     mov qword ptr [rbp - 0x38], rcx
141bd274a: 48894530                     mov qword ptr [rbp + 0x30], rax
141bd274e: 4883c020                     add rax, 0x20
141bd2752: 488945e0                     mov qword ptr [rbp - 0x20], rax
141bd2756: 0fb74500                     movzx eax, word ptr [rbp]
141bd275a: 48895da8                     mov qword ptr [rbp - 0x58], rbx
141bd275e: 4c896dd0                     mov qword ptr [rbp - 0x30], r13
141bd2762: e9c2000000                   jmp 0x141bd2829
141bd2767: b801000000                   mov eax, 1
141bd276c: 4889c3                       mov rbx, rax
141bd276f: 4889c1                       mov rcx, rax
141bd2772: 4c89e2                       mov rdx, r12
141bd2775: 4d89c4                       mov r12, r8
141bd2778: e88314d500                   call 0x142923c00
141bd277d: 49c1e604                     shl r14, 4
141bd2781: 488b45b0                     mov rax, qword ptr [rbp - 0x50]
141bd2785: 460fb6743008                 movzx r14d, byte ptr [rax + r14 + 8]
141bd278b: 0fb6057724a601               movzx eax, byte ptr [rip + 0x1a62477]
141bd2792: b958000000                   mov ecx, 0x58
141bd2797: ba08000000                   mov edx, 8
141bd279c: e86f14a4fe                   call 0x140613c10
141bd27a1: 4885c0                       test rax, rax
141bd27a4: 4c8b6dd0                     mov r13, qword ptr [rbp - 0x30]
141bd27a8: 48895d48                     mov qword ptr [rbp + 0x48], rbx
141bd27ac: 0f84df030000                 je 0x141bd2b91
141bd27b2: 48c70001000000               mov qword ptr [rax], 1
141bd27b9: 48c7400801000000             mov qword ptr [rax + 8], 1
141bd27c1: c7401002000000               mov dword ptr [rax + 0x10], 2
141bd27c8: 488b4d28                     mov rcx, qword ptr [rbp + 0x28]
141bd27cc: 48894818                     mov qword ptr [rax + 0x18], rcx
141bd27d0: 4c896020                     mov qword ptr [rax + 0x20], r12
141bd27d4: 48895828                     mov qword ptr [rax + 0x28], rbx
141bd27d8: 4c896030                     mov qword ptr [rax + 0x30], r12
141bd27dc: 440f114838                   movups xmmword ptr [rax + 0x38], xmm9
141bd27e1: f3440f115048                 movss dword ptr [rax + 0x48], xmm10
141bd27e7: 8b4d40                       mov ecx, dword ptr [rbp + 0x40]
141bd27ea: 89484c                       mov dword ptr [rax + 0x4c], ecx
141bd27ed: 44887050                     mov byte ptr [rax + 0x50], r14b
141bd27f1: 48894518                     mov qword ptr [rbp + 0x18], rax
141bd27f5: 48c745100c000000             mov qword ptr [rbp + 0x10], 0xc
141bd27fd: 488b4dd8                     mov rcx, qword ptr [rbp - 0x28]
141bd2801: 488d5510                     lea rdx, [rbp + 0x10]
141bd2805: 4c8d0534383201               lea r8, [rip + 0x1323834]
141bd280c: e8af26ebfe                   call 0x140a84ec0
141bd2811: 4c8b5db8                     mov r11, qword ptr [rbp - 0x48]
141bd2815: 4c8b5550                     mov r10, qword ptr [rbp + 0x50]
141bd2819: 4489f8                       mov eax, r15d
141bd281c: 4d85db                       test r11, r11
141bd281f: 488b5da8                     mov rbx, qword ptr [rbp - 0x58]
141bd2823: 0f84da020000                 je 0x141bd2b03
141bd2829: 6685c0                       test ax, ax
141bd282c: 7422                         je 0x141bd2850
141bd282e: 448d78ff                     lea r15d, [rax - 1]
141bd2832: 4121c7                       and r15d, eax
141bd2835: 6644897d00                   mov word ptr [rbp], r15w
141bd283a: 49ffcb                       dec r11
141bd283d: 4c895d08                     mov qword ptr [rbp + 8], r11
141bd2841: 4885ff                       test rdi, rdi
141bd2844: 7547                         jne 0x141bd288d
141bd2846: e9b8020000                   jmp 0x141bd2b03
141bd284b: 0f1f440000                   nop dword ptr [rax + rax]
141bd2850: 660f6f06                     movdqa xmm0, xmmword ptr [rsi]
141bd2854: 660fd7c0                     pmovmskb eax, xmm0
141bd2858: 4881c700ffffff               add rdi, -0x100
141bd285f: 4883c610                     add rsi, 0x10
141bd2863: 3dffff0000                   cmp eax, 0xffff
141bd2868: 74e6                         je 0x141bd2850
141bd286a: 488975f0                     mov qword ptr [rbp - 0x10], rsi
141bd286e: 48897de8                     mov qword ptr [rbp - 0x18], rdi
141bd2872: b9feffffff                   mov ecx, 0xfffffffe
141bd2877: 29c1                         sub ecx, eax
141bd2879: f7d0                         not eax
141bd287b: 4189c7                       mov r15d, eax
141bd287e: 4121cf                       and r15d, ecx
141bd2881: 6644897d00                   mov word ptr [rbp], r15w
141bd2886: 49ffcb                       dec r11
141bd2889: 4c895d08                     mov qword ptr [rbp + 8], r11
141bd288d: 4d89d4                       mov r12, r10
141bd2890: f30fbcc8                     tzcnt ecx, eax
141bd2894: c1e104                       shl ecx, 4
141bd2897: 4889f8                       mov rax, rdi
141bd289a: 4829c8                       sub rax, rcx
141bd289d: 488b50f8                     mov rdx, qword ptr [rax - 8]
141bd28a1: 4189d0                       mov r8d, edx
141bd28a4: 4c3b4310                     cmp r8, qword ptr [rbx + 0x10]
141bd28a8: 0f838f020000                 jae 0x141bd2b3d
141bd28ae: 488b4b08                     mov rcx, qword ptr [rbx + 8]
141bd28b2: 4989d1                       mov r9, rdx
141bd28b5: 49c1e920                     shr r9, 0x20
141bd28b9: 4f8d1480                     lea r10, [r8 + r8*4]
141bd28bd: 46390c91                     cmp dword ptr [rcx + r10*4], r9d
141bd28c1: 0f857d020000                 jne 0x141bd2b44
141bd28c7: 4a8d0c91                     lea rcx, [rcx + r10*4]
141bd28cb: 448b5104                     mov r10d, dword ptr [rcx + 4]
141bd28cf: 41beffffffff                 mov r14d, 0xffffffff
141bd28d5: 4d39f2                       cmp r10, r14
141bd28d8: 0f8466020000                 je 0x141bd2b44
141bd28de: 4d395538                     cmp qword ptr [r13 + 0x38], r10
141bd28e2: 0f864f020000                 jbe 0x141bd2b37
141bd28e8: 4d8b4528                     mov r8, qword ptr [r13 + 0x28]
141bd28ec: 4589d1                       mov r9d, r10d
141bd28ef: 41c1e906                     shr r9d, 6
141bd28f3: 4f8b04c8                     mov r8, qword ptr [r8 + r9*8]
141bd28f7: 4d0fa3d0                     bt r8, r10
141bd28fb: 0f8336020000                 jae 0x141bd2b37
141bd2901: 8b510c                       mov edx, dword ptr [rcx + 0xc]
141bd2904: 4c8b83a8010000               mov r8, qword ptr [rbx + 0x1a8]
141bd290b: 4c8d14d2                     lea r10, [rdx + rdx*8]
141bd290f: 4b8d14d0                     lea rdx, [r8 + r10*8]
141bd2913: 4d8b8d10010000               mov r9, qword ptr [r13 + 0x110]
141bd291a: 4f8b44d040                   mov r8, qword ptr [r8 + r10*8 + 0x40]
141bd291f: 4d39c8                       cmp r8, r9
141bd2922: 763c                         jbe 0x141bd2960
141bd2924: 4c8b5238                     mov r10, qword ptr [rdx + 0x38]
141bd2928: 4f8b0cca                     mov r9, qword ptr [r10 + r9*8]
141bd292c: 4d85c9                       test r9, r9
141bd292f: 742f                         je 0x141bd2960
141bd2931: 4c8b5218                     mov r10, qword ptr [rdx + 0x18]
141bd2935: 49f7d1                       not r9
141bd2938: 4f8d0c49                     lea r9, [r9 + r9*2]
141bd293c: 49c1e104                     shl r9, 4
141bd2940: 4f8b640a10                   mov r12, qword ptr [r10 + r9 + 0x10]
141bd2945: 4d8b8d18010000               mov r9, qword ptr [r13 + 0x118]
141bd294c: 4d39c8                       cmp r8, r9
141bd294f: 771e                         ja 0x141bd296f
141bd2951: eb4d                         jmp 0x141bd29a0
141bd2953: 666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
141bd2960: 4531e4                       xor r12d, r12d
141bd2963: 4d8b8d18010000               mov r9, qword ptr [r13 + 0x118]
141bd296a: 4d39c8                       cmp r8, r9
141bd296d: 7631                         jbe 0x141bd29a0
141bd296f: 4c8b5238                     mov r10, qword ptr [rdx + 0x38]
141bd2973: 4f8b0cca                     mov r9, qword ptr [r10 + r9*8]
141bd2977: 4d85c9                       test r9, r9
141bd297a: 7424                         je 0x141bd29a0
141bd297c: 4c8b5218                     mov r10, qword ptr [rdx + 0x18]
141bd2980: 49f7d1                       not r9
141bd2983: 4f8d0c49                     lea r9, [r9 + r9*2]
141bd2987: 49c1e104                     shl r9, 4
141bd298b: 4f8b540a10                   mov r10, qword ptr [r10 + r9 + 0x10]
141bd2990: 4d8b8d20010000               mov r9, qword ptr [r13 + 0x120]
141bd2997: 4d39c8                       cmp r8, r9
141bd299a: 7644                         jbe 0x141bd29e0
141bd299c: eb11                         jmp 0x141bd29af
141bd299e: 6690                         nop 
141bd29a0: 4531d2                       xor r10d, r10d
141bd29a3: 4d8b8d20010000               mov r9, qword ptr [r13 + 0x120]
141bd29aa: 4d39c8                       cmp r8, r9
141bd29ad: 7631                         jbe 0x141bd29e0
141bd29af: 4c8b4238                     mov r8, qword ptr [rdx + 0x38]
141bd29b3: 4f8b04c8                     mov r8, qword ptr [r8 + r9*8]
141bd29b7: 4d85c0                       test r8, r8
141bd29ba: 7424                         je 0x141bd29e0
141bd29bc: 488b5218                     mov rdx, qword ptr [rdx + 0x18]
141bd29c0: 49f7d0                       not r8
141bd29c3: 4f8d0440                     lea r8, [r8 + r8*2]
141bd29c7: 49c1e004                     shl r8, 4
141bd29cb: 4a8b540210                   mov rdx, qword ptr [rdx + r8 + 0x10]
141bd29d0: eb10                         jmp 0x141bd29e2
141bd29d2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141bd29e0: 31d2                         xor edx, edx
141bd29e2: 448b7110                     mov r14d, dword ptr [rcx + 0x10]
141bd29e6: 4b8d1c76                     lea rbx, [r14 + r14*2]
141bd29ea: 48c1e304                     shl rbx, 4
141bd29ee: 496bce58                     imul rcx, r14, 0x58
141bd29f2: 4901ca                       add r10, rcx
141bd29f5: 0f28c7                       movaps xmm0, xmm7
141bd29f8: f3410f5c041c                 subss xmm0, dword ptr [r12 + rbx]
141bd29fe: 410f28c8                     movaps xmm1, xmm8
141bd2a02: f3410f5c4c1c08               subss xmm1, dword ptr [r12 + rbx + 8]
141bd2a09: f30f59c0                     mulss xmm0, xmm0
141bd2a0d: f30f59c9                     mulss xmm1, xmm1
141bd2a11: f30f58c8                     addss xmm1, xmm0
141bd2a15: 0f57c0                       xorps xmm0, xmm0
141bd2a18: f30f51c1                     sqrtss xmm0, xmm1
141bd2a1c: 0f2ef0                       ucomiss xmm6, xmm0
141bd2a1f: 0f86f4fdffff                 jbe 0x141bd2819
141bd2a25: 488955b0                     mov qword ptr [rbp - 0x50], rdx
141bd2a29: 4c895db8                     mov qword ptr [rbp - 0x48], r11
141bd2a2d: 4c8b40f0                     mov r8, qword ptr [rax - 0x10]
141bd2a31: 8b453c                       mov eax, dword ptr [rbp + 0x3c]
141bd2a34: 488b4dc8                     mov rcx, qword ptr [rbp - 0x38]
141bd2a38: 8901                         mov dword ptr [rcx], eax
141bd2a3a: 488b4530                     mov rax, qword ptr [rbp + 0x30]
141bd2a3e: 4c8b6830                     mov r13, qword ptr [rax + 0x30]
141bd2a42: 4c8b4840                     mov r9, qword ptr [rax + 0x40]
141bd2a46: 4c3b6820                     cmp r13, qword ptr [rax + 0x20]
141bd2a4a: 4c895550                     mov qword ptr [rbp + 0x50], r10
141bd2a4e: 4c894528                     mov qword ptr [rbp + 0x28], r8
141bd2a52: 7520                         jne 0x141bd2a74
141bd2a54: 488b4de0                     mov rcx, qword ptr [rbp - 0x20]
141bd2a58: 488d15b1224601               lea rdx, [rip + 0x14622b1]
141bd2a5f: 4c894d40                     mov qword ptr [rbp + 0x40], r9
141bd2a63: e8c8fb98fe                   call 0x140562630
141bd2a68: 4c8b4d40                     mov r9, qword ptr [rbp + 0x40]
141bd2a6c: 4c8b4528                     mov r8, qword ptr [rbp + 0x28]
141bd2a70: 4c8b5550                     mov r10, qword ptr [rbp + 0x50]
141bd2a74: 4901dc                       add r12, rbx
141bd2a77: 488b5530                     mov rdx, qword ptr [rbp + 0x30]
141bd2a7b: 488b4228                     mov rax, qword ptr [rdx + 0x28]
141bd2a7f: 4c89e9                       mov rcx, r13
141bd2a82: 48c1e104                     shl rcx, 4
141bd2a86: 4c890c08                     mov qword ptr [rax + rcx], r9
141bd2a8a: 4c89440808                   mov qword ptr [rax + rcx + 8], r8
141bd2a8f: 49ffc5                       inc r13
141bd2a92: 4c896a30                     mov qword ptr [rdx + 0x30], r13
141bd2a96: 48ff4240                     inc qword ptr [rdx + 0x40]
141bd2a9a: 8b4538                       mov eax, dword ptr [rbp + 0x38]
141bd2a9d: 488b4dc0                     mov rcx, qword ptr [rbp - 0x40]
141bd2aa1: 8901                         mov dword ptr [rcx], eax
141bd2aa3: 450f100c24                   movups xmm9, xmmword ptr [r12]
141bd2aa8: f3450f10542410               movss xmm10, dword ptr [r12 + 0x10]
141bd2aaf: 418b5c2414                   mov ebx, dword ptr [r12 + 0x14]
141bd2ab4: 498d4a48                     lea rcx, [r10 + 0x48]
141bd2ab8: e8733698ff                   call 0x141556130
141bd2abd: 4885d2                       test rdx, rdx
141bd2ac0: 0f88e0000000                 js 0x141bd2ba6
141bd2ac6: 4989c4                       mov r12, rax
141bd2ac9: 4989d0                       mov r8, rdx
141bd2acc: 895d40                       mov dword ptr [rbp + 0x40], ebx
141bd2acf: 0f8492fcffff                 je 0x141bd2767
141bd2ad5: 0fb6052d21a601               movzx eax, byte ptr [rip + 0x1a6212d]
141bd2adc: 41bd01000000                 mov r13d, 1
141bd2ae2: ba01000000                   mov edx, 1
141bd2ae7: 4c89c3                       mov rbx, r8
141bd2aea: 4c89c1                       mov rcx, r8
141bd2aed: e81e11a4fe                   call 0x140613c10
141bd2af2: 4885c0                       test rax, rax
141bd2af5: 0f84c1000000                 je 0x141bd2bbc
141bd2afb: 4989d8                       mov r8, rbx
141bd2afe: e969fcffff                   jmp 0x141bd276c
141bd2b03: 0f287560                     movaps xmm6, xmmword ptr [rbp + 0x60]
141bd2b07: 0f287d70                     movaps xmm7, xmmword ptr [rbp + 0x70]
141bd2b0b: 440f288580000000             movaps xmm8, xmmword ptr [rbp + 0x80]
141bd2b13: 440f288d90000000             movaps xmm9, xmmword ptr [rbp + 0x90]
141bd2b1b: 440f2895a0000000             movaps xmm10, xmmword ptr [rbp + 0xa0]
141bd2b23: 4881c438010000               add rsp, 0x138
141bd2b2a: 5b                           pop rbx
141bd2b2b: 5f                           pop rdi
141bd2b2c: 5e                           pop rsi
141bd2b2d: 415c                         pop r12
141bd2b2f: 415d                         pop r13
141bd2b31: 415e                         pop r14
141bd2b33: 415f                         pop r15
141bd2b35: 5d                           pop rbp
141bd2b36: c3                           ret 
141bd2b37: 49c1e220                     shl r10, 0x20
141bd2b3b: eb24                         jmp 0x141bd2b61
141bd2b3d: 48c1ea20                     shr rdx, 0x20
141bd2b41: 4989d1                       mov r9, rdx
141bd2b44: 48b800000000ffffffff         movabs rax, 0xffffffff00000000
141bd2b4e: 4921c4                       and r12, rax
141bd2b51: 49ffc4                       inc r12
141bd2b54: 4d89e2                       mov r10, r12
141bd2b57: 49c1e120                     shl r9, 0x20
141bd2b5b: 4d09c1                       or r9, r8
141bd2b5e: 4c89ca                       mov rdx, r9
141bd2b61: 4c895510                     mov qword ptr [rbp + 0x10], r10
141bd2b65: 48895518                     mov qword ptr [rbp + 0x18], rdx
141bd2b69: 488d05b8343201               lea rax, [rip + 0x13234b8]
141bd2b70: 4889442420                   mov qword ptr [rsp + 0x20], rax
141bd2b75: 488d0d1c333201               lea rcx, [rip + 0x132331c]
141bd2b7c: 4c8d0df5323201               lea r9, [rip + 0x13232f5]
141bd2b83: 4c8d4510                     lea r8, [rbp + 0x10]
141bd2b87: ba2b000000                   mov edx, 0x2b
141bd2b8c: e8cf6bd000                   call 0x1428d9760
141bd2b91: 4c896550                     mov qword ptr [rbp + 0x50], r12
141bd2b95: b908000000                   mov ecx, 8
141bd2b9a: ba58000000                   mov edx, 0x58
141bd2b9f: e85c64d000                   call 0x1428d9000
141bd2ba4: 0f0b                         ud2 
141bd2ba6: 4531ed                       xor r13d, r13d
141bd2ba9: 4c8d0560333201               lea r8, [rip + 0x1323360]
141bd2bb0: 4c89e9                       mov rcx, r13
141bd2bb3: 488b5548                     mov rdx, qword ptr [rbp + 0x48]
141bd2bb7: e82764d000                   call 0x1428d8fe3
141bd2bbc: 48895d48                     mov qword ptr [rbp + 0x48], rbx
141bd2bc0: 4c8d0549333201               lea r8, [rip + 0x1323349]
141bd2bc7: 4c89e9                       mov rcx, r13
141bd2bca: 488b5548                     mov rdx, qword ptr [rbp + 0x48]
141bd2bce: e81064d000                   call 0x1428d8fe3
141bd2bd3: cc                           int3 
141bd2bd4: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
141bd2be0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
141bd2be5: 55                           push rbp
141bd2be6: 4157                         push r15
141bd2be8: 4156                         push r14
141bd2bea: 4155                         push r13
141bd2bec: 4154                         push r12
141bd2bee: 56                           push rsi
141bd2bef: 57                           push rdi
141bd2bf0: 53                           push rbx
141bd2bf1: 4883ec78                     sub rsp, 0x78
141bd2bf5: 488daa80000000               lea rbp, [rdx + 0x80]
141bd2bfc: 440f29542420                 movaps xmmword ptr [rsp + 0x20], xmm10
141bd2c02: 440f294c2430                 movaps xmmword ptr [rsp + 0x30], xmm9
141bd2c08: 440f29442440                 movaps xmmword ptr [rsp + 0x40], xmm8
141bd2c0e: 0f297c2450                   movaps xmmword ptr [rsp + 0x50], xmm7
141bd2c13: 0f29742460                   movaps xmmword ptr [rsp + 0x60], xmm6
141bd2c18: 48837d5000                   cmp qword ptr [rbp + 0x50], 0
141bd2c1d: 7413                         je 0x141bd2c32
141bd2c1f: 41b801000000                 mov r8d, 1
141bd2c25: 488b4d48                     mov rcx, qword ptr [rbp + 0x48]
141bd2c29: 488b5550                     mov rdx, qword ptr [rbp + 0x50]
141bd2c2d: e8ee0fa4fe                   call 0x140613c20
141bd2c32: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
141bd2c37: 0f287c2450                   movaps xmm7, xmmword ptr [rsp + 0x50]
141bd2c3c: 440f28442440                 movaps xmm8, xmmword ptr [rsp + 0x40]
141bd2c42: 440f284c2430                 movaps xmm9, xmmword ptr [rsp + 0x30]
141bd2c48: 440f28542420                 movaps xmm10, xmmword ptr [rsp + 0x20]
141bd2c4e: 4883c478                     add rsp, 0x78
141bd2c52: 5b                           pop rbx
141bd2c53: 5f                           pop rdi
141bd2c54: 5e                           pop rsi
141bd2c55: 415c                         pop r12
141bd2c57: 415d                         pop r13
141bd2c59: 415e                         pop r14
141bd2c5b: 415f                         pop r15
141bd2c5d: 5d                           pop rbp
141bd2c5e: c3                           ret 
