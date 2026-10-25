0x1416e8f20 55 push rbp
0x1416e8f21 56 push rsi
0x1416e8f22 57 push rdi
0x1416e8f23 4881ecd0000000 sub rsp, 0xd0
0x1416e8f2a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416e8f32 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x1416e8f3a 4889ce mov rsi, rcx
0x1416e8f3d 0fb602 movzx eax, byte ptr [rdx]
0x1416e8f40 85c0 test eax, eax
0x1416e8f42 7452 je 0x1416e8f96
0x1416e8f44 83f801 cmp eax, 1
0x1416e8f47 0f8591000000 jne 0x1416e8fde
0x1416e8f4d 0fb605b5bcf401 movzx eax, byte ptr [rip + 0x1f4bcb5]
0x1416e8f54 b911000000 mov ecx, 0x11
0x1416e8f59 ba01000000 mov edx, 1
0x1416e8f5e e8adacf2fe call 0x140613c10
0x1416e8f63 4885c0 test rax, rax
0x1416e8f66 0f84f7010000 je 0x1416e9163
0x1416e8f6c 0f10054d685a01 movups xmm0, xmmword ptr [rip + 0x15a684d]
0x1416e8f73 0f1100 movups xmmword ptr [rax], xmm0
0x1416e8f76 c6401079 mov byte ptr [rax + 0x10], 0x79
0x1416e8f7a c60603 mov byte ptr [rsi], 3
0x1416e8f7d 48c7460811000000 mov qword ptr [rsi + 8], 0x11
0x1416e8f85 48894610 mov qword ptr [rsi + 0x10], rax
0x1416e8f89 48c7461811000000 mov qword ptr [rsi + 0x18], 0x11
0x1416e8f91 e995010000 jmp 0x1416e912b
0x1416e8f96 0fb6056cbcf401 movzx eax, byte ptr [rip + 0x1f4bc6c]
0x1416e8f9d b908000000 mov ecx, 8
0x1416e8fa2 ba01000000 mov edx, 1
0x1416e8fa7 e864acf2fe call 0x140613c10
0x1416e8fac 4885c0 test rax, rax
0x1416e8faf 0f84c4010000 je 0x1416e9179
0x1416e8fb5 48b94d61746368696e67 movabs rcx, 0x676e69686374614d
0x1416e8fbf 488908 mov qword ptr [rax], rcx
0x1416e8fc2 c60603 mov byte ptr [rsi], 3
0x1416e8fc5 48c7460808000000 mov qword ptr [rsi + 8], 8
0x1416e8fcd 48894610 mov qword ptr [rsi + 0x10], rax
0x1416e8fd1 48c7461808000000 mov qword ptr [rsi + 0x18], 8
0x1416e8fd9 e94d010000 jmp 0x1416e912b
0x1416e8fde 4889d7 mov rdi, rdx
0x1416e8fe1 488d0530685a01 lea rax, [rip + 0x15a6830]
0x1416e8fe8 4889442420 mov qword ptr [rsp + 0x20], rax
0x1416e8fed 48c744243003000000 mov qword ptr [rsp + 0x30], 3
0x1416e8ff6 48c744242808000000 mov qword ptr [rsp + 0x28], 8
0x1416e8fff 488d1534685a01 lea rdx, [rip + 0x15a6834]
0x1416e9006 488d4dc0 lea rcx, [rbp - 0x40]
0x1416e900a 41b80d000000 mov r8d, 0xd
0x1416e9010 41b902000000 mov r9d, 2
0x1416e9016 e815d449ff call 0x140b86430
0x1416e901b 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x1416e901f 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x1416e9023 4889ca mov rdx, rcx
0x1416e9026 48f7da neg rdx
0x1416e9029 710c jno 0x1416e9037
0x1416e902b 48894608 mov qword ptr [rsi + 8], rax
0x1416e902f c60606 mov byte ptr [rsi], 6
0x1416e9032 e9f4000000 jmp 0x1416e912b
0x1416e9037 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1416e903b 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1416e903f 0f114d30 movups xmmword ptr [rbp + 0x30], xmm1
0x1416e9043 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1416e9047 4c8d4f04 lea r9, [rdi + 4]
0x1416e904b 48894d10 mov qword ptr [rbp + 0x10], rcx
0x1416e904f 48894518 mov qword ptr [rbp + 0x18], rax
0x1416e9053 c6454701 mov byte ptr [rbp + 0x47], 1
0x1416e9057 488d15c2675a01 lea rdx, [rip + 0x15a67c2]
0x1416e905e 488d4d10 lea rcx, [rbp + 0x10]
0x1416e9062 41b808000000 mov r8d, 8
0x1416e9068 e8c351eaff call 0x14158e230
0x1416e906d 4885c0 test rax, rax
0x1416e9070 7549 jne 0x1416e90bb
0x1416e9072 4c8d4f01 lea r9, [rdi + 1]
0x1416e9076 c6454701 mov byte ptr [rbp + 0x47], 1
0x1416e907a 488d15a7675a01 lea rdx, [rip + 0x15a67a7]
0x1416e9081 488d4d10 lea rcx, [rbp + 0x10]
0x1416e9085 41b808000000 mov r8d, 8
0x1416e908b e8e056eaff call 0x14158e770
0x1416e9090 4885c0 test rax, rax
0x1416e9093 7526 jne 0x1416e90bb
0x1416e9095 4883c708 add rdi, 8
0x1416e9099 c6454701 mov byte ptr [rbp + 0x47], 1
0x1416e909d 488d158c675a01 lea rdx, [rip + 0x15a678c]
0x1416e90a4 488d4d10 lea rcx, [rbp + 0x10]
0x1416e90a8 41b80a000000 mov r8d, 0xa
0x1416e90ae 4989f9 mov r9, rdi
0x1416e90b1 e8ea4feaff call 0x14158e0a0
0x1416e90b6 4885c0 test rax, rax
0x1416e90b9 747e je 0x1416e9139
0x1416e90bb 48894608 mov qword ptr [rsi + 8], rax
0x1416e90bf c60606 mov byte ptr [rsi], 6
0x1416e90c2 488b5510 mov rdx, qword ptr [rbp + 0x10]
0x1416e90c6 4885d2 test rdx, rdx
0x1416e90c9 740f je 0x1416e90da
0x1416e90cb 488b4d18 mov rcx, qword ptr [rbp + 0x18]
0x1416e90cf 41b801000000 mov r8d, 1
0x1416e90d5 e846abf2fe call 0x140613c20
0x1416e90da 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1416e90de 4885c9 test rcx, rcx
0x1416e90e1 742f je 0x1416e9112
0x1416e90e3 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1416e90e7 488b4538 mov rax, qword ptr [rbp + 0x38]
0x1416e90eb 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1416e90f3 48894dd0 mov qword ptr [rbp - 0x30], rcx
0x1416e90f7 488955d8 mov qword ptr [rbp - 0x28], rdx
0x1416e90fb 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1416e9103 48894df0 mov qword ptr [rbp - 0x10], rcx
0x1416e9107 488955f8 mov qword ptr [rbp - 8], rdx
0x1416e910b b901000000 mov ecx, 1
0x1416e9110 eb04 jmp 0x1416e9116
0x1416e9112 31c9 xor ecx, ecx
0x1416e9114 31c0 xor eax, eax
0x1416e9116 48894dc0 mov qword ptr [rbp - 0x40], rcx
0x1416e911a 48894de0 mov qword ptr [rbp - 0x20], rcx
0x1416e911e 48894500 mov qword ptr [rbp], rax
0x1416e9122 488d4dc0 lea rcx, [rbp - 0x40]
0x1416e9126 e8951819ff call 0x14087a9c0
0x1416e912b 4889f0 mov rax, rsi
0x1416e912e 4881c4d0000000 add rsp, 0xd0
0x1416e9135 5f pop rdi
0x1416e9136 5e pop rsi
0x1416e9137 5d pop rbp
0x1416e9138 c3 ret 
0x1416e9139 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x1416e913d 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x1416e9141 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x1416e9145 0f2955e0 movaps xmmword ptr [rbp - 0x20], xmm2
0x1416e9149 0f294dd0 movaps xmmword ptr [rbp - 0x30], xmm1
0x1416e914d 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416e9151 c6454700 mov byte ptr [rbp + 0x47], 0
0x1416e9155 488d55c0 lea rdx, [rbp - 0x40]
0x1416e9159 4889f1 mov rcx, rsi
0x1416e915c e8dfd449ff call 0x140b86640
0x1416e9161 ebc8 jmp 0x1416e912b
0x1416e9163 4c8d059e615a01 lea r8, [rip + 0x15a619e]
0x1416e916a b901000000 mov ecx, 1
0x1416e916f ba11000000 mov edx, 0x11
0x1416e9174 e86afe1e01 call 0x1428d8fe3
0x1416e9179 4c8d0588615a01 lea r8, [rip + 0x15a6188]
0x1416e9180 b901000000 mov ecx, 1
0x1416e9185 ba08000000 mov edx, 8
0x1416e918a e854fe1e01 call 0x1428d8fe3
0x1416e918f cc int3 
0x1416e9190 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9195 55 push rbp
0x1416e9196 56 push rsi
0x1416e9197 57 push rdi
0x1416e9198 4883ec40 sub rsp, 0x40
0x1416e919c 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e91a3 807d4700 cmp byte ptr [rbp + 0x47], 0
0x1416e91a7 7508 jne 0x1416e91b1
0x1416e91a9 4883c440 add rsp, 0x40
0x1416e91ad 5f pop rdi
0x1416e91ae 5e pop rsi
0x1416e91af 5d pop rbp
0x1416e91b0 c3 ret 
0x1416e91b1 488d4d10 lea rcx, [rbp + 0x10]
0x1416e91b5 e876371aff call 0x14088c930
0x1416e91ba ebed jmp 0x1416e91a9
