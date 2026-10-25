0x140ee2560 55 push rbp
0x140ee2561 56 push rsi
0x140ee2562 57 push rdi
0x140ee2563 53 push rbx
0x140ee2564 4881ece8060000 sub rsp, 0x6e8
0x140ee256b 488dac2480000000 lea rbp, [rsp + 0x80]
0x140ee2573 48c78560060000feffffff mov qword ptr [rbp + 0x660], 0xfffffffffffffffe
0x140ee257e 4889ce mov rsi, rcx
0x140ee2581 0f104260 movups xmm0, xmmword ptr [rdx + 0x60]
0x140ee2585 0f2985d0040000 movaps xmmword ptr [rbp + 0x4d0], xmm0
0x140ee258c 0f104250 movups xmm0, xmmword ptr [rdx + 0x50]
0x140ee2590 0f2985c0040000 movaps xmmword ptr [rbp + 0x4c0], xmm0
0x140ee2597 0f104240 movups xmm0, xmmword ptr [rdx + 0x40]
0x140ee259b 0f2985b0040000 movaps xmmword ptr [rbp + 0x4b0], xmm0
0x140ee25a2 0f1002 movups xmm0, xmmword ptr [rdx]
0x140ee25a5 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x140ee25a9 0f105220 movups xmm2, xmmword ptr [rdx + 0x20]
0x140ee25ad 0f105a30 movups xmm3, xmmword ptr [rdx + 0x30]
0x140ee25b1 0f299da0040000 movaps xmmword ptr [rbp + 0x4a0], xmm3
0x140ee25b8 0f299590040000 movaps xmmword ptr [rbp + 0x490], xmm2
0x140ee25bf 0f298d80040000 movaps xmmword ptr [rbp + 0x480], xmm1
0x140ee25c6 0f298570040000 movaps xmmword ptr [rbp + 0x470], xmm0
0x140ee25cd 0f1082d0000000 movups xmm0, xmmword ptr [rdx + 0xd0]
0x140ee25d4 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
0x140ee25db 0f1082c0000000 movups xmm0, xmmword ptr [rdx + 0xc0]
0x140ee25e2 0f298550040000 movaps xmmword ptr [rbp + 0x450], xmm0
0x140ee25e9 0f1082b0000000 movups xmm0, xmmword ptr [rdx + 0xb0]
0x140ee25f0 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
0x140ee25f7 0f104270 movups xmm0, xmmword ptr [rdx + 0x70]
0x140ee25fb 0f108a80000000 movups xmm1, xmmword ptr [rdx + 0x80]
0x140ee2602 0f109290000000 movups xmm2, xmmword ptr [rdx + 0x90]
0x140ee2609 0f109aa0000000 movups xmm3, xmmword ptr [rdx + 0xa0]
0x140ee2610 0f299d30040000 movaps xmmword ptr [rbp + 0x430], xmm3
0x140ee2617 0f299520040000 movaps xmmword ptr [rbp + 0x420], xmm2
0x140ee261e 0f298d10040000 movaps xmmword ptr [rbp + 0x410], xmm1
0x140ee2625 0f298500040000 movaps xmmword ptr [rbp + 0x400], xmm0
0x140ee262c 0fb605d6257502 movzx eax, byte ptr [rip + 0x27525d6]
0x140ee2633 b940050000 mov ecx, 0x540
0x140ee2638 ba08000000 mov edx, 8
0x140ee263d e8ce1573ff call 0x140613c10
0x140ee2642 4885c0 test rax, rax
0x140ee2645 0f8476070000 je 0x140ee2dc1
0x140ee264b 48898550060000 mov qword ptr [rbp + 0x650], rax
0x140ee2652 0f57c0 xorps xmm0, xmm0
0x140ee2655 0f118530050000 movups xmmword ptr [rbp + 0x530], xmm0
0x140ee265c 0f118548050000 movups xmmword ptr [rbp + 0x548], xmm0
0x140ee2663 0f118560050000 movups xmmword ptr [rbp + 0x560], xmm0
0x140ee266a 0f118578050000 movups xmmword ptr [rbp + 0x578], xmm0
0x140ee2671 0f118590050000 movups xmmword ptr [rbp + 0x590], xmm0
0x140ee2678 c785a005000000000000 mov dword ptr [rbp + 0x5a0], 0
0x140ee2682 0f1185c8050000 movups xmmword ptr [rbp + 0x5c8], xmm0
0x140ee2689 0f1185e0050000 movups xmmword ptr [rbp + 0x5e0], xmm0
0x140ee2690 0f1185f8050000 movups xmmword ptr [rbp + 0x5f8], xmm0
0x140ee2697 0f118510060000 movups xmmword ptr [rbp + 0x610], xmm0
0x140ee269e 0f118528060000 movups xmmword ptr [rbp + 0x628], xmm0
0x140ee26a5 c7853806000000000000 mov dword ptr [rbp + 0x638], 0
0x140ee26af 48c785e804000000000000 mov qword ptr [rbp + 0x4e8], 0
0x140ee26ba 48c7851005000000000000 mov qword ptr [rbp + 0x510], 0
0x140ee26c5 48c7851805000008000000 mov qword ptr [rbp + 0x518], 8
0x140ee26d0 48c7852005000000000000 mov qword ptr [rbp + 0x520], 0
0x140ee26db 48c7852805000010000000 mov qword ptr [rbp + 0x528], 0x10
0x140ee26e6 48c7854005000010000000 mov qword ptr [rbp + 0x540], 0x10
0x140ee26f1 48c7855805000010000000 mov qword ptr [rbp + 0x558], 0x10
0x140ee26fc 48c7857005000010000000 mov qword ptr [rbp + 0x570], 0x10
0x140ee2707 48c7858805000010000000 mov qword ptr [rbp + 0x588], 0x10
0x140ee2712 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x140ee271c 48899da8050000 mov qword ptr [rbp + 0x5a8], rbx
0x140ee2723 488d055652cb01 lea rax, [rip + 0x1cb5256]
0x140ee272a 488985b0050000 mov qword ptr [rbp + 0x5b0], rax
0x140ee2731 48c785b805000057000000 mov qword ptr [rbp + 0x5b8], 0x57
0x140ee273c 48c785c005000010000000 mov qword ptr [rbp + 0x5c0], 0x10
0x140ee2747 48c785d805000010000000 mov qword ptr [rbp + 0x5d8], 0x10
0x140ee2752 48c785f005000010000000 mov qword ptr [rbp + 0x5f0], 0x10
0x140ee275d 48c7850806000010000000 mov qword ptr [rbp + 0x608], 0x10
0x140ee2768 48c7852006000010000000 mov qword ptr [rbp + 0x620], 0x10
0x140ee2773 c7854006000000000000 mov dword ptr [rbp + 0x640], 0
0x140ee277d 66c785440600000100 mov word ptr [rbp + 0x644], 1
0x140ee2786 c7854806000000000000 mov dword ptr [rbp + 0x648], 0
0x140ee2790 0fb60572247502 movzx eax, byte ptr [rip + 0x2752472]
0x140ee2797 b968010000 mov ecx, 0x168
0x140ee279c ba08000000 mov edx, 8
0x140ee27a1 e86a1473ff call 0x140613c10
0x140ee27a6 4885c0 test rax, rax
0x140ee27a9 0f842a060000 je 0x140ee2dd9
0x140ee27af 4889c7 mov rdi, rax
0x140ee27b2 488d95e8040000 lea rdx, [rbp + 0x4e8]
0x140ee27b9 41b868010000 mov r8d, 0x168
0x140ee27bf 4889c1 mov rcx, rax
0x140ee27c2 e83914a401 call 0x142923c00
0x140ee27c7 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
0x140ee27ce 4c8d05f350cb01 lea r8, [rip + 0x1cb50f3]
0x140ee27d5 488d8d20030000 lea rcx, [rbp + 0x320]
0x140ee27dc 4889fa mov rdx, rdi
0x140ee27df e82c23e0ff call 0x140ce4b10
0x140ee27e4 c6855e06000001 mov byte ptr [rbp + 0x65e], 1
0x140ee27eb 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x140ee27f2 e899350000 call 0x140ee5d90
0x140ee27f7 c6855d06000001 mov byte ptr [rbp + 0x65d], 1
0x140ee27fe 488d8d40020000 lea rcx, [rbp + 0x240]
0x140ee2805 e8068de000 call 0x141ceb510
0x140ee280a 0f2885d0040000 movaps xmm0, xmmword ptr [rbp + 0x4d0]
0x140ee2811 0f2985f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm0
0x140ee2818 0f2885c0040000 movaps xmm0, xmmword ptr [rbp + 0x4c0]
0x140ee281f 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
0x140ee2826 0f2885b0040000 movaps xmm0, xmmword ptr [rbp + 0x4b0]
0x140ee282d 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
0x140ee2834 0f288570040000 movaps xmm0, xmmword ptr [rbp + 0x470]
0x140ee283b 0f288d80040000 movaps xmm1, xmmword ptr [rbp + 0x480]
0x140ee2842 0f289590040000 movaps xmm2, xmmword ptr [rbp + 0x490]
0x140ee2849 0f289da0040000 movaps xmm3, xmmword ptr [rbp + 0x4a0]
0x140ee2850 0f299dc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm3
0x140ee2857 0f2995b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm2
0x140ee285e 0f298da0030000 movaps xmmword ptr [rbp + 0x3a0], xmm1
0x140ee2865 0f298590030000 movaps xmmword ptr [rbp + 0x390], xmm0
0x140ee286c 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x140ee2873 e8d8b8e000 call 0x141cee150
0x140ee2878 488d8d60010000 lea rcx, [rbp + 0x160]
0x140ee287f e89cc5e000 call 0x141ceee20
0x140ee2884 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140ee288b e8f0d1e000 call 0x141cefa80
0x140ee2890 488d8d80000000 lea rcx, [rbp + 0x80]
0x140ee2897 e844dce000 call 0x141cf04e0
0x140ee289c 488d4d10 lea rcx, [rbp + 0x10]
0x140ee28a0 e86be6e000 call 0x141cf0f10
0x140ee28a5 488d4da0 lea rcx, [rbp - 0x60]
0x140ee28a9 e8e2efe000 call 0x141cf1890
0x140ee28ae 488d8de8040000 lea rcx, [rbp + 0x4e8]
0x140ee28b5 e8b602e100 call 0x141cf2b70
0x140ee28ba 0f288560040000 movaps xmm0, xmmword ptr [rbp + 0x460]
0x140ee28c1 488b8550060000 mov rax, qword ptr [rbp + 0x650]
0x140ee28c8 0f118030050000 movups xmmword ptr [rax + 0x530], xmm0
0x140ee28cf 0f288550040000 movaps xmm0, xmmword ptr [rbp + 0x450]
0x140ee28d6 0f118020050000 movups xmmword ptr [rax + 0x520], xmm0
0x140ee28dd 0f288540040000 movaps xmm0, xmmword ptr [rbp + 0x440]
0x140ee28e4 0f118010050000 movups xmmword ptr [rax + 0x510], xmm0
0x140ee28eb 0f288500040000 movaps xmm0, xmmword ptr [rbp + 0x400]
0x140ee28f2 0f288d10040000 movaps xmm1, xmmword ptr [rbp + 0x410]
0x140ee28f9 0f289520040000 movaps xmm2, xmmword ptr [rbp + 0x420]
0x140ee2900 0f289d30040000 movaps xmm3, xmmword ptr [rbp + 0x430]
0x140ee2907 0f119800050000 movups xmmword ptr [rax + 0x500], xmm3
0x140ee290e 0f1190f0040000 movups xmmword ptr [rax + 0x4f0], xmm2
0x140ee2915 0f1188e0040000 movups xmmword ptr [rax + 0x4e0], xmm1
0x140ee291c 0f1180d0040000 movups xmmword ptr [rax + 0x4d0], xmm0
0x140ee2923 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
0x140ee292a 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
0x140ee2931 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
0x140ee2938 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
0x140ee293f 0f1100 movups xmmword ptr [rax], xmm0
0x140ee2942 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x140ee2946 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x140ee294a 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x140ee294e 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
0x140ee2955 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x140ee2959 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
0x140ee2960 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x140ee2964 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
0x140ee296b 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x140ee296f 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
0x140ee2976 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x140ee297d 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
0x140ee2984 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x140ee298b 0f1085f0020000 movups xmm0, xmmword ptr [rbp + 0x2f0]
0x140ee2992 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x140ee2999 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x140ee29a0 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
0x140ee29a7 0f1095d0020000 movups xmm2, xmmword ptr [rbp + 0x2d0]
0x140ee29ae 0f109de0020000 movups xmm3, xmmword ptr [rbp + 0x2e0]
0x140ee29b5 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x140ee29bc 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x140ee29c3 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x140ee29ca 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x140ee29ce 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
0x140ee29d5 0f108d50020000 movups xmm1, xmmword ptr [rbp + 0x250]
0x140ee29dc 0f109560020000 movups xmm2, xmmword ptr [rbp + 0x260]
0x140ee29e3 0f109d70020000 movups xmm3, xmmword ptr [rbp + 0x270]
0x140ee29ea 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x140ee29f1 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x140ee29f8 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x140ee29ff 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x140ee2a06 0f108580020000 movups xmm0, xmmword ptr [rbp + 0x280]
0x140ee2a0d 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x140ee2a14 0f108590020000 movups xmm0, xmmword ptr [rbp + 0x290]
0x140ee2a1b 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x140ee2a22 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
0x140ee2a29 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x140ee2a30 0f288590030000 movaps xmm0, xmmword ptr [rbp + 0x390]
0x140ee2a37 0f288da0030000 movaps xmm1, xmmword ptr [rbp + 0x3a0]
0x140ee2a3e 0f2895b0030000 movaps xmm2, xmmword ptr [rbp + 0x3b0]
0x140ee2a45 0f289dc0030000 movaps xmm3, xmmword ptr [rbp + 0x3c0]
0x140ee2a4c 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x140ee2a53 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x140ee2a5a 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x140ee2a61 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x140ee2a68 0f2885d0030000 movaps xmm0, xmmword ptr [rbp + 0x3d0]
0x140ee2a6f 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x140ee2a76 0f2885f0030000 movaps xmm0, xmmword ptr [rbp + 0x3f0]
0x140ee2a7d 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x140ee2a84 0f2885e0030000 movaps xmm0, xmmword ptr [rbp + 0x3e0]
0x140ee2a8b 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x140ee2a92 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
0x140ee2a99 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x140ee2aa0 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
0x140ee2aa7 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x140ee2aae 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
0x140ee2ab5 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x140ee2abc 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
0x140ee2ac3 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
0x140ee2aca 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
0x140ee2ad1 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
0x140ee2ad8 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x140ee2adf 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x140ee2ae6 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x140ee2aed 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x140ee2af4 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
0x140ee2afb 0f108d70010000 movups xmm1, xmmword ptr [rbp + 0x170]
0x140ee2b02 0f109580010000 movups xmm2, xmmword ptr [rbp + 0x180]
0x140ee2b09 0f109d90010000 movups xmm3, xmmword ptr [rbp + 0x190]
0x140ee2b10 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x140ee2b17 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x140ee2b1e 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x140ee2b25 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x140ee2b2c 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
0x140ee2b33 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x140ee2b3a 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
0x140ee2b41 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x140ee2b48 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
0x140ee2b4f 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x140ee2b56 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x140ee2b5d 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x140ee2b64 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x140ee2b6b 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x140ee2b72 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
0x140ee2b79 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
0x140ee2b80 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
0x140ee2b87 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
0x140ee2b8e 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x140ee2b95 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
0x140ee2b9c 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x140ee2ba3 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
0x140ee2baa 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x140ee2bb1 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
0x140ee2bb8 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x140ee2bbf 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
0x140ee2bc6 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
0x140ee2bcd 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
0x140ee2bd4 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
0x140ee2bdb 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
0x140ee2be2 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
0x140ee2be9 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
0x140ee2bf0 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140ee2bf7 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
0x140ee2bfe 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x140ee2c05 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
0x140ee2c0c 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
0x140ee2c13 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
0x140ee2c1a 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x140ee2c1e 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x140ee2c22 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x140ee2c26 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x140ee2c2a 0f118080030000 movups xmmword ptr [rax + 0x380], xmm0
0x140ee2c31 0f118890030000 movups xmmword ptr [rax + 0x390], xmm1
0x140ee2c38 0f1190a0030000 movups xmmword ptr [rax + 0x3a0], xmm2
0x140ee2c3f 0f1198b0030000 movups xmmword ptr [rax + 0x3b0], xmm3
0x140ee2c46 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x140ee2c4a 0f1180c0030000 movups xmmword ptr [rax + 0x3c0], xmm0
0x140ee2c51 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140ee2c55 0f1180d0030000 movups xmmword ptr [rax + 0x3d0], xmm0
0x140ee2c5c 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x140ee2c60 0f1180e0030000 movups xmmword ptr [rax + 0x3e0], xmm0
0x140ee2c67 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x140ee2c6b 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140ee2c6f 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x140ee2c73 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x140ee2c77 0f1180f0030000 movups xmmword ptr [rax + 0x3f0], xmm0
0x140ee2c7e 0f118800040000 movups xmmword ptr [rax + 0x400], xmm1
0x140ee2c85 0f119010040000 movups xmmword ptr [rax + 0x410], xmm2
0x140ee2c8c 0f119820040000 movups xmmword ptr [rax + 0x420], xmm3
0x140ee2c93 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x140ee2c97 0f118030040000 movups xmmword ptr [rax + 0x430], xmm0
0x140ee2c9e 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x140ee2ca2 0f118040040000 movups xmmword ptr [rax + 0x440], xmm0
0x140ee2ca9 0f104500 movups xmm0, xmmword ptr [rbp]
0x140ee2cad 0f118050040000 movups xmmword ptr [rax + 0x450], xmm0
0x140ee2cb4 0f1085e8040000 movups xmm0, xmmword ptr [rbp + 0x4e8]
0x140ee2cbb 0f108df8040000 movups xmm1, xmmword ptr [rbp + 0x4f8]
0x140ee2cc2 0f109508050000 movups xmm2, xmmword ptr [rbp + 0x508]
0x140ee2cc9 0f109d18050000 movups xmm3, xmmword ptr [rbp + 0x518]
0x140ee2cd0 0f118060040000 movups xmmword ptr [rax + 0x460], xmm0
0x140ee2cd7 0f118870040000 movups xmmword ptr [rax + 0x470], xmm1
0x140ee2cde 0f119080040000 movups xmmword ptr [rax + 0x480], xmm2
0x140ee2ce5 0f119890040000 movups xmmword ptr [rax + 0x490], xmm3
0x140ee2cec 0f108528050000 movups xmm0, xmmword ptr [rbp + 0x528]
0x140ee2cf3 0f1180a0040000 movups xmmword ptr [rax + 0x4a0], xmm0
0x140ee2cfa 0f108548050000 movups xmm0, xmmword ptr [rbp + 0x548]
0x140ee2d01 0f1180c0040000 movups xmmword ptr [rax + 0x4c0], xmm0
0x140ee2d08 0f108538050000 movups xmm0, xmmword ptr [rbp + 0x538]
0x140ee2d0f 0f1180b0040000 movups xmmword ptr [rax + 0x4b0], xmm0
0x140ee2d16 48c785f00400000c000000 mov qword ptr [rbp + 0x4f0], 0xc
0x140ee2d21 488985f8040000 mov qword ptr [rbp + 0x4f8], rax
0x140ee2d28 48c785000500000c000000 mov qword ptr [rbp + 0x500], 0xc
0x140ee2d33 48c7850805000000000000 mov qword ptr [rbp + 0x508], 0
0x140ee2d3e 48c7851005000008000000 mov qword ptr [rbp + 0x510], 8
0x140ee2d49 0f57c0 xorps xmm0, xmm0
0x140ee2d4c 0f118518050000 movups xmmword ptr [rbp + 0x518], xmm0
0x140ee2d53 48899de8040000 mov qword ptr [rbp + 0x4e8], rbx
0x140ee2d5a 488d8d20050000 lea rcx, [rbp + 0x520]
0x140ee2d61 e8ca64ffff call 0x140ed9230
0x140ee2d66 0f108548050000 movups xmm0, xmmword ptr [rbp + 0x548]
0x140ee2d6d 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x140ee2d71 0f108538050000 movups xmm0, xmmword ptr [rbp + 0x538]
0x140ee2d78 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x140ee2d7c 0f108528050000 movups xmm0, xmmword ptr [rbp + 0x528]
0x140ee2d83 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x140ee2d87 0f1085e8040000 movups xmm0, xmmword ptr [rbp + 0x4e8]
0x140ee2d8e 0f108df8040000 movups xmm1, xmmword ptr [rbp + 0x4f8]
0x140ee2d95 0f109508050000 movups xmm2, xmmword ptr [rbp + 0x508]
0x140ee2d9c 0f109d18050000 movups xmm3, xmmword ptr [rbp + 0x518]
0x140ee2da3 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x140ee2da7 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x140ee2dab 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x140ee2daf 0f1106 movups xmmword ptr [rsi], xmm0
0x140ee2db2 4889f0 mov rax, rsi
0x140ee2db5 4881c4e8060000 add rsp, 0x6e8
0x140ee2dbc 5b pop rbx
0x140ee2dbd 5f pop rdi
0x140ee2dbe 5e pop rsi
0x140ee2dbf 5d pop rbp
0x140ee2dc0 c3 ret 
0x140ee2dc1 c6855c06000001 mov byte ptr [rbp + 0x65c], 1
0x140ee2dc8 b908000000 mov ecx, 8
0x140ee2dcd ba40050000 mov edx, 0x540
0x140ee2dd2 e829629f01 call 0x1428d9000
0x140ee2dd7 eb0f jmp 0x140ee2de8
0x140ee2dd9 b908000000 mov ecx, 8
0x140ee2dde ba68010000 mov edx, 0x168
0x140ee2de3 e818629f01 call 0x1428d9000
0x140ee2de8 0f0b ud2 
0x140ee2dea 660f1f440000 nop word ptr [rax + rax]
0x140ee2df0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2df5 55 push rbp
0x140ee2df6 56 push rsi
0x140ee2df7 57 push rdi
0x140ee2df8 53 push rbx
0x140ee2df9 4883ec28 sub rsp, 0x28
0x140ee2dfd 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2e04 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
0x140ee2e0b 488d8de8040000 lea rcx, [rbp + 0x4e8]
0x140ee2e12 e8f94070ff call 0x1405e6f10
0x140ee2e17 c6855f06000001 mov byte ptr [rbp + 0x65f], 1
0x140ee2e1e 4883c428 add rsp, 0x28
0x140ee2e22 5b pop rbx
0x140ee2e23 5f pop rdi
0x140ee2e24 5e pop rsi
0x140ee2e25 5d pop rbp
0x140ee2e26 c3 ret 
0x140ee2e27 660f1f840000000000 nop word ptr [rax + rax]
0x140ee2e30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2e35 55 push rbp
0x140ee2e36 56 push rsi
0x140ee2e37 57 push rdi
0x140ee2e38 53 push rbx
0x140ee2e39 4883ec28 sub rsp, 0x28
0x140ee2e3d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2e44 0fb69d5f060000 movzx ebx, byte ptr [rbp + 0x65f]
0x140ee2e4b ba40050000 mov edx, 0x540
0x140ee2e50 41b808000000 mov r8d, 8
0x140ee2e56 488b8d50060000 mov rcx, qword ptr [rbp + 0x650]
0x140ee2e5d e8be0d73ff call 0x140613c20
0x140ee2e62 889d5c060000 mov byte ptr [rbp + 0x65c], bl
0x140ee2e68 4883c428 add rsp, 0x28
0x140ee2e6c 5b pop rbx
0x140ee2e6d 5f pop rdi
0x140ee2e6e 5e pop rsi
0x140ee2e6f 5d pop rbp
0x140ee2e70 c3 ret 
0x140ee2e71 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee2e80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2e85 55 push rbp
0x140ee2e86 56 push rsi
0x140ee2e87 57 push rdi
0x140ee2e88 53 push rbx
0x140ee2e89 4883ec28 sub rsp, 0x28
0x140ee2e8d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2e94 488d4da0 lea rcx, [rbp - 0x60]
0x140ee2e98 e8e36914ff call 0x140029880
0x140ee2e9d 90 nop 
0x140ee2e9e 4883c428 add rsp, 0x28
0x140ee2ea2 5b pop rbx
0x140ee2ea3 5f pop rdi
0x140ee2ea4 5e pop rsi
0x140ee2ea5 5d pop rbp
0x140ee2ea6 c3 ret 
0x140ee2ea7 660f1f840000000000 nop word ptr [rax + rax]
0x140ee2eb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2eb5 55 push rbp
0x140ee2eb6 56 push rsi
0x140ee2eb7 57 push rdi
0x140ee2eb8 53 push rbx
0x140ee2eb9 4883ec28 sub rsp, 0x28
0x140ee2ebd 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2ec4 0fb69d5c060000 movzx ebx, byte ptr [rbp + 0x65c]
0x140ee2ecb 488d8d00040000 lea rcx, [rbp + 0x400]
0x140ee2ed2 e8a96914ff call 0x140029880
0x140ee2ed7 80fb01 cmp bl, 1
0x140ee2eda 7409 je 0x140ee2ee5
0x140ee2edc 4883c428 add rsp, 0x28
0x140ee2ee0 5b pop rbx
0x140ee2ee1 5f pop rdi
0x140ee2ee2 5e pop rsi
0x140ee2ee3 5d pop rbp
0x140ee2ee4 c3 ret 
0x140ee2ee5 488d8d70040000 lea rcx, [rbp + 0x470]
0x140ee2eec e88f6914ff call 0x140029880
0x140ee2ef1 ebe9 jmp 0x140ee2edc
0x140ee2ef3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee2f00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2f05 55 push rbp
0x140ee2f06 56 push rsi
0x140ee2f07 57 push rdi
0x140ee2f08 53 push rbx
0x140ee2f09 4883ec28 sub rsp, 0x28
0x140ee2f0d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2f14 488d8de8040000 lea rcx, [rbp + 0x4e8]
0x140ee2f1b e8606914ff call 0x140029880
0x140ee2f20 90 nop 
0x140ee2f21 4883c428 add rsp, 0x28
0x140ee2f25 5b pop rbx
0x140ee2f26 5f pop rdi
0x140ee2f27 5e pop rsi
0x140ee2f28 5d pop rbp
0x140ee2f29 c3 ret 
0x140ee2f2a 660f1f440000 nop word ptr [rax + rax]
0x140ee2f30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2f35 55 push rbp
0x140ee2f36 56 push rsi
0x140ee2f37 57 push rdi
0x140ee2f38 53 push rbx
0x140ee2f39 4883ec28 sub rsp, 0x28
0x140ee2f3d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2f44 0fb69d5e060000 movzx ebx, byte ptr [rbp + 0x65e]
0x140ee2f4b 488d8d20030000 lea rcx, [rbp + 0x320]
0x140ee2f52 e8296914ff call 0x140029880
0x140ee2f57 889d5f060000 mov byte ptr [rbp + 0x65f], bl
0x140ee2f5d 4883c428 add rsp, 0x28
0x140ee2f61 5b pop rbx
0x140ee2f62 5f pop rdi
0x140ee2f63 5e pop rsi
0x140ee2f64 5d pop rbp
0x140ee2f65 c3 ret 
0x140ee2f66 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee2f70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2f75 55 push rbp
0x140ee2f76 56 push rsi
0x140ee2f77 57 push rdi
0x140ee2f78 53 push rbx
0x140ee2f79 4883ec28 sub rsp, 0x28
0x140ee2f7d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2f84 0fb69d5d060000 movzx ebx, byte ptr [rbp + 0x65d]
0x140ee2f8b 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x140ee2f92 e8e96814ff call 0x140029880
0x140ee2f97 889d5e060000 mov byte ptr [rbp + 0x65e], bl
0x140ee2f9d 4883c428 add rsp, 0x28
0x140ee2fa1 5b pop rbx
0x140ee2fa2 5f pop rdi
0x140ee2fa3 5e pop rsi
0x140ee2fa4 5d pop rbp
0x140ee2fa5 c3 ret 
0x140ee2fa6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee2fb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2fb5 55 push rbp
0x140ee2fb6 56 push rsi
0x140ee2fb7 57 push rdi
0x140ee2fb8 53 push rbx
0x140ee2fb9 4883ec28 sub rsp, 0x28
0x140ee2fbd 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee2fc4 488d8d90030000 lea rcx, [rbp + 0x390]
0x140ee2fcb e8b06814ff call 0x140029880
0x140ee2fd0 488d8d40020000 lea rcx, [rbp + 0x240]
0x140ee2fd7 e8a46814ff call 0x140029880
0x140ee2fdc c6855d06000000 mov byte ptr [rbp + 0x65d], 0
0x140ee2fe3 4883c428 add rsp, 0x28
0x140ee2fe7 5b pop rbx
0x140ee2fe8 5f pop rdi
0x140ee2fe9 5e pop rsi
0x140ee2fea 5d pop rbp
0x140ee2feb c3 ret 
0x140ee2fec 0f1f4000 nop dword ptr [rax]
0x140ee2ff0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee2ff5 55 push rbp
0x140ee2ff6 56 push rsi
0x140ee2ff7 57 push rdi
0x140ee2ff8 53 push rbx
0x140ee2ff9 4883ec28 sub rsp, 0x28
0x140ee2ffd 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3004 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x140ee300b e8706814ff call 0x140029880
0x140ee3010 90 nop 
0x140ee3011 4883c428 add rsp, 0x28
0x140ee3015 5b pop rbx
0x140ee3016 5f pop rdi
0x140ee3017 5e pop rsi
0x140ee3018 5d pop rbp
0x140ee3019 c3 ret 
0x140ee301a 660f1f440000 nop word ptr [rax + rax]
0x140ee3020 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee3025 55 push rbp
0x140ee3026 56 push rsi
0x140ee3027 57 push rdi
0x140ee3028 53 push rbx
0x140ee3029 4883ec28 sub rsp, 0x28
0x140ee302d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3034 488d8d60010000 lea rcx, [rbp + 0x160]
0x140ee303b e8406814ff call 0x140029880
0x140ee3040 90 nop 
0x140ee3041 4883c428 add rsp, 0x28
0x140ee3045 5b pop rbx
0x140ee3046 5f pop rdi
0x140ee3047 5e pop rsi
0x140ee3048 5d pop rbp
0x140ee3049 c3 ret 
0x140ee304a 660f1f440000 nop word ptr [rax + rax]
0x140ee3050 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee3055 55 push rbp
0x140ee3056 56 push rsi
0x140ee3057 57 push rdi
0x140ee3058 53 push rbx
0x140ee3059 4883ec28 sub rsp, 0x28
0x140ee305d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3064 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140ee306b e8106814ff call 0x140029880
0x140ee3070 90 nop 
0x140ee3071 4883c428 add rsp, 0x28
0x140ee3075 5b pop rbx
0x140ee3076 5f pop rdi
0x140ee3077 5e pop rsi
0x140ee3078 5d pop rbp
0x140ee3079 c3 ret 
0x140ee307a 660f1f440000 nop word ptr [rax + rax]
0x140ee3080 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee3085 55 push rbp
0x140ee3086 56 push rsi
0x140ee3087 57 push rdi
0x140ee3088 53 push rbx
0x140ee3089 4883ec28 sub rsp, 0x28
0x140ee308d 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3094 488d8d80000000 lea rcx, [rbp + 0x80]
0x140ee309b e8e06714ff call 0x140029880
0x140ee30a0 90 nop 
0x140ee30a1 4883c428 add rsp, 0x28
0x140ee30a5 5b pop rbx
0x140ee30a6 5f pop rdi
0x140ee30a7 5e pop rsi
0x140ee30a8 5d pop rbp
0x140ee30a9 c3 ret 
0x140ee30aa 660f1f440000 nop word ptr [rax + rax]
0x140ee30b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee30b5 55 push rbp
0x140ee30b6 56 push rsi
0x140ee30b7 57 push rdi
0x140ee30b8 53 push rbx
0x140ee30b9 4883ec28 sub rsp, 0x28
0x140ee30bd 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee30c4 488d4d10 lea rcx, [rbp + 0x10]
0x140ee30c8 e8b36714ff call 0x140029880
0x140ee30cd 90 nop 
0x140ee30ce 4883c428 add rsp, 0x28
0x140ee30d2 5b pop rbx
0x140ee30d3 5f pop rdi
0x140ee30d4 5e pop rsi
0x140ee30d5 5d pop rbp
0x140ee30d6 c3 ret 
