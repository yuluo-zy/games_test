0x1416e7940 55 push rbp
0x1416e7941 56 push rsi
0x1416e7942 57 push rdi
0x1416e7943 4881eca0000000 sub rsp, 0xa0
0x1416e794a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416e7952 0f297510 movaps xmmword ptr [rbp + 0x10], xmm6
0x1416e7956 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1416e795e 4889d7 mov rdi, rdx
0x1416e7961 4889ce mov rsi, rcx
0x1416e7964 f30f10720c movss xmm6, dword ptr [rdx + 0xc]
0x1416e7969 48b80000000000000080 movabs rax, 0x8000000000000000
0x1416e7973 488945d0 mov qword ptr [rbp - 0x30], rax
0x1416e7977 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1416e797f 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1416e7987 c6450701 mov byte ptr [rbp + 7], 1
0x1416e798b 488d15a8785a01 lea rdx, [rip + 0x15a78a8]
0x1416e7992 488d4dd0 lea rcx, [rbp - 0x30]
0x1416e7996 41b806000000 mov r8d, 6
0x1416e799c 4989f9 mov r9, rdi
0x1416e799f e8cceceaff call 0x141596670
0x1416e79a4 4885c0 test rax, rax
0x1416e79a7 7523 jne 0x1416e79cc
0x1416e79a9 4c8d4f08 lea r9, [rdi + 8]
0x1416e79ad c6450701 mov byte ptr [rbp + 7], 1
0x1416e79b1 488d1588785a01 lea rdx, [rip + 0x15a7888]
0x1416e79b8 488d4dd0 lea rcx, [rbp - 0x30]
0x1416e79bc 41b806000000 mov r8d, 6
0x1416e79c2 e859e0eaff call 0x141595a20
0x1416e79c7 4885c0 test rax, rax
0x1416e79ca 742f je 0x1416e79fb
0x1416e79cc 488d4de8 lea rcx, [rbp - 0x18]
0x1416e79d0 48894608 mov qword ptr [rsi + 8], rax
0x1416e79d4 c60606 mov byte ptr [rsi], 6
0x1416e79d7 e864151bff call 0x140898f40
0x1416e79dc 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416e79e0 4889d0 mov rax, rdx
0x1416e79e3 48f7d8 neg rax
0x1416e79e6 7045 jo 0x1416e7a2d
0x1416e79e8 7343 jae 0x1416e7a2d
0x1416e79ea 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416e79ee 41b801000000 mov r8d, 1
0x1416e79f4 e827c2f2fe call 0x140613c20
0x1416e79f9 eb32 jmp 0x1416e7a2d
0x1416e79fb 0f57c0 xorps xmm0, xmm0
0x1416e79fe 0f2ef0 ucomiss xmm6, xmm0
0x1416e7a01 753c jne 0x1416e7a3f
0x1416e7a03 7a3a jp 0x1416e7a3f
0x1416e7a05 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1416e7a09 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1416e7a0d 0f1055f0 movups xmm2, xmmword ptr [rbp - 0x10]
0x1416e7a11 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x1416e7a15 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1416e7a19 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1416e7a1d c6450700 mov byte ptr [rbp + 7], 0
0x1416e7a21 488d55a0 lea rdx, [rbp - 0x60]
0x1416e7a25 4889f1 mov rcx, rsi
0x1416e7a28 e893ea49ff call 0x140b864c0
0x1416e7a2d 4889f0 mov rax, rsi
0x1416e7a30 0f287510 movaps xmm6, xmmword ptr [rbp + 0x10]
0x1416e7a34 4881c4a0000000 add rsp, 0xa0
0x1416e7a3b 5f pop rdi
0x1416e7a3c 5e pop rsi
0x1416e7a3d 5d pop rbp
0x1416e7a3e c3 ret 
0x1416e7a3f 4883c70c add rdi, 0xc
0x1416e7a43 c6450701 mov byte ptr [rbp + 7], 1
0x1416e7a47 488d15f8775a01 lea rdx, [rip + 0x15a77f8]
0x1416e7a4e 488d4dd0 lea rcx, [rbp - 0x30]
0x1416e7a52 41b808000000 mov r8d, 8
0x1416e7a58 4989f9 mov r9, rdi
0x1416e7a5b e8b0aeeaff call 0x141592910
0x1416e7a60 4885c0 test rax, rax
0x1416e7a63 0f8563ffffff jne 0x1416e79cc
0x1416e7a69 eb9a jmp 0x1416e7a05
0x1416e7a6b 0f1f440000 nop dword ptr [rax + rax]
0x1416e7a70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e7a75 55 push rbp
0x1416e7a76 56 push rsi
0x1416e7a77 57 push rdi
0x1416e7a78 4883ec30 sub rsp, 0x30
0x1416e7a7c 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e7a83 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1416e7a88 488b55d0 mov rdx, qword ptr [rbp - 0x30]
0x1416e7a8c 488d045500000000 lea rax, [rdx*2]
0x1416e7a94 4885c0 test rax, rax
0x1416e7a97 740f je 0x1416e7aa8
0x1416e7a99 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x1416e7a9d 41b801000000 mov r8d, 1
0x1416e7aa3 e878c1f2fe call 0x140613c20
0x1416e7aa8 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1416e7aad 4883c430 add rsp, 0x30
0x1416e7ab1 5f pop rdi
0x1416e7ab2 5e pop rsi
0x1416e7ab3 5d pop rbp
0x1416e7ab4 c3 ret 
0x1416e7ab5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416e7ac0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e7ac5 55 push rbp
0x1416e7ac6 56 push rsi
0x1416e7ac7 57 push rdi
0x1416e7ac8 4883ec30 sub rsp, 0x30
0x1416e7acc 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e7ad3 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1416e7ad8 807d0700 cmp byte ptr [rbp + 7], 0
0x1416e7adc 750d jne 0x1416e7aeb
0x1416e7ade 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1416e7ae3 4883c430 add rsp, 0x30
0x1416e7ae7 5f pop rdi
0x1416e7ae8 5e pop rsi
0x1416e7ae9 5d pop rbp
0x1416e7aea c3 ret 
0x1416e7aeb 488d4dd0 lea rcx, [rbp - 0x30]
0x1416e7aef e86c6017ff call 0x14085db60
0x1416e7af4 ebe8 jmp 0x1416e7ade
