0x1412bba70 55 push rbp
0x1412bba71 56 push rsi
0x1412bba72 4881ecd8000000 sub rsp, 0xd8
0x1412bba79 488dac2480000000 lea rbp, [rsp + 0x80]
0x1412bba81 48c74550feffffff mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
0x1412bba89 4889ce mov rsi, rcx
0x1412bba8c 49c1e105 shl r9, 5
0x1412bba90 488d4da8 lea rcx, [rbp - 0x58]
0x1412bba94 e8078a3500 call 0x1416144a0
0x1412bba99 0fb60569913702 movzx eax, byte ptr [rip + 0x2379169]
0x1412bbaa0 b928000000 mov ecx, 0x28
0x1412bbaa5 ba08000000 mov edx, 8
0x1412bbaaa e8618135ff call 0x140613c10
0x1412bbaaf 4885c0 test rax, rax
0x1412bbab2 0f84b0000000 je 0x1412bbb68
0x1412bbab8 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
0x1412bbabc 48894820 mov qword ptr [rax + 0x20], rcx
0x1412bbac0 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x1412bbac4 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x1412bbac8 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1412bbacc 0f1100 movups xmmword ptr [rax], xmm0
0x1412bbacf 488d0de7237e01 lea rcx, [rip + 0x17e23e7]
0x1412bbad6 48894d00 mov qword ptr [rbp], rcx
0x1412bbada 48c7450840000000 mov qword ptr [rbp + 8], 0x40
0x1412bbae2 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bbae6 488d05eb099701 lea rax, [rip + 0x19709eb]
0x1412bbaed 48894518 mov qword ptr [rbp + 0x18], rax
0x1412bbaf1 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x1412bbaf9 c6454605 mov byte ptr [rbp + 0x46], 5
0x1412bbafd 48894d20 mov qword ptr [rbp + 0x20], rcx
0x1412bbb01 48c7452840000000 mov qword ptr [rbp + 0x28], 0x40
0x1412bbb09 0fb605f9903702 movzx eax, byte ptr [rip + 0x23790f9]
0x1412bbb10 b940000000 mov ecx, 0x40
0x1412bbb15 ba08000000 mov edx, 8
0x1412bbb1a e8f18035ff call 0x140613c10
0x1412bbb1f 4885c0 test rax, rax
0x1412bbb22 7455 je 0x1412bbb79
0x1412bbb24 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x1412bbb28 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x1412bbb2c 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x1412bbb30 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x1412bbb34 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1412bbb38 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1412bbb3c 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1412bbb40 0f1100 movups xmmword ptr [rax], xmm0
0x1412bbb43 48badbc9c5cd8572392a movabs rdx, 0x2a397285cdc5c9db
0x1412bbb4d 4889f1 mov rcx, rsi
0x1412bbb50 4989c0 mov r8, rax
0x1412bbb53 e898e06aff call 0x140969bf0
0x1412bbb58 4883f801 cmp rax, 1
0x1412bbb5c 742c je 0x1412bbb8a
0x1412bbb5e 4881c4d8000000 add rsp, 0xd8
0x1412bbb65 5e pop rsi
0x1412bbb66 5d pop rbp
0x1412bbb67 c3 ret 
0x1412bbb68 b908000000 mov ecx, 8
0x1412bbb6d ba28000000 mov edx, 0x28
0x1412bbb72 e889d46101 call 0x1428d9000
0x1412bbb77 eb0f jmp 0x1412bbb88
0x1412bbb79 b908000000 mov ecx, 8
0x1412bbb7e ba40000000 mov edx, 0x40
0x1412bbb83 e878d46101 call 0x1428d9000
0x1412bbb88 0f0b ud2 
0x1412bbb8a 4889d0 mov rax, rdx
0x1412bbb8d 4883c210 add rdx, 0x10
0x1412bbb91 4889d6 mov rsi, rdx
0x1412bbb94 4883781840 cmp qword ptr [rax + 0x18], 0x40
0x1412bbb99 757c jne 0x1412bbc17
0x1412bbb9b 488b16 mov rdx, qword ptr [rsi]
0x1412bbb9e 488d0d18237e01 lea rcx, [rip + 0x17e2318]
0x1412bbba5 41b840000000 mov r8d, 0x40
0x1412bbbab e840806601 call 0x142923bf0
0x1412bbbb0 85c0 test eax, eax
0x1412bbbb2 7563 jne 0x1412bbc17
0x1412bbbb4 488d05e5219701 lea rax, [rip + 0x19721e5]
0x1412bbbbb 488945d0 mov qword ptr [rbp - 0x30], rax
0x1412bbbbf 488d05da7c27ff lea rax, [rip - 0xd88326]
0x1412bbbc6 488945d8 mov qword ptr [rbp - 0x28], rax
0x1412bbbca 4889e8 mov rax, rbp
0x1412bbbcd 488945e0 mov qword ptr [rbp - 0x20], rax
0x1412bbbd1 488d05b8e2e4fe lea rax, [rip - 0x11b1d48]
0x1412bbbd8 488945e8 mov qword ptr [rbp - 0x18], rax
0x1412bbbdc 488d05451a9701 lea rax, [rip + 0x1971a45]
0x1412bbbe3 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bbbe7 48c7451803000000 mov qword ptr [rbp + 0x18], 3
0x1412bbbef 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x1412bbbf7 488d45d0 lea rax, [rbp - 0x30]
0x1412bbbfb 48894520 mov qword ptr [rbp + 0x20], rax
0x1412bbbff 48c7452802000000 mov qword ptr [rbp + 0x28], 2
0x1412bbc07 488d154a1a9701 lea rdx, [rip + 0x1971a4a]
0x1412bbc0e 488d4d10 lea rcx, [rbp + 0x10]
0x1412bbc12 e879d76101 call 0x1428d9390
0x1412bbc17 488d0582219701 lea rax, [rip + 0x1972182]
0x1412bbc1e 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bbc22 488d05777c27ff lea rax, [rip - 0xd88389]
0x1412bbc29 48894518 mov qword ptr [rbp + 0x18], rax
0x1412bbc2d 48897520 mov qword ptr [rbp + 0x20], rsi
0x1412bbc31 488d0558e2e4fe lea rax, [rip - 0x11b1da8]
0x1412bbc38 48894528 mov qword ptr [rbp + 0x28], rax
0x1412bbc3c 4889e9 mov rcx, rbp
0x1412bbc3f 48894d30 mov qword ptr [rbp + 0x30], rcx
0x1412bbc43 48894538 mov qword ptr [rbp + 0x38], rax
0x1412bbc47 488d051a199701 lea rax, [rip + 0x197191a]
0x1412bbc4e 488945d0 mov qword ptr [rbp - 0x30], rax
0x1412bbc52 48c745d803000000 mov qword ptr [rbp - 0x28], 3
0x1412bbc5a 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x1412bbc62 488d4510 lea rax, [rbp + 0x10]
0x1412bbc66 488945e0 mov qword ptr [rbp - 0x20], rax
0x1412bbc6a 48c745e803000000 mov qword ptr [rbp - 0x18], 3
0x1412bbc72 488d1557199701 lea rdx, [rip + 0x1971957]
0x1412bbc79 488d4dd0 lea rcx, [rbp - 0x30]
0x1412bbc7d e80ed76101 call 0x1428d9390
0x1412bbc82 cc int3 
0x1412bbc83 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1412bbc90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412bbc95 55 push rbp
0x1412bbc96 56 push rsi
0x1412bbc97 4883ec28 sub rsp, 0x28
0x1412bbc9b 488daa80000000 lea rbp, [rdx + 0x80]
0x1412bbca2 488d4da8 lea rcx, [rbp - 0x58]
0x1412bbca6 e8056892ff call 0x140be24b0
0x1412bbcab 90 nop 
0x1412bbcac 4883c428 add rsp, 0x28
0x1412bbcb0 5e pop rsi
0x1412bbcb1 5d pop rbp
0x1412bbcb2 c3 ret 
0x1412bbcb3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1412bbcc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412bbcc5 55 push rbp
0x1412bbcc6 56 push rsi
0x1412bbcc7 4883ec28 sub rsp, 0x28
0x1412bbccb 488daa80000000 lea rbp, [rdx + 0x80]
0x1412bbcd2 488d4d10 lea rcx, [rbp + 0x10]
0x1412bbcd6 e8d547e0fe call 0x1400c04b0
0x1412bbcdb 90 nop 
0x1412bbcdc 4883c428 add rsp, 0x28
0x1412bbce0 5e pop rsi
0x1412bbce1 5d pop rbp
0x1412bbce2 c3 ret 
