0x1415d3f80 55 push rbp
0x1415d3f81 4156 push r14
0x1415d3f83 56 push rsi
0x1415d3f84 57 push rdi
0x1415d3f85 53 push rbx
0x1415d3f86 4881ecb0000000 sub rsp, 0xb0
0x1415d3f8d 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d3f95 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x1415d3f9d 4889ce mov rsi, rcx
0x1415d3fa0 0f104218 movups xmm0, xmmword ptr [rdx + 0x18]
0x1415d3fa4 0f104a28 movups xmm1, xmmword ptr [rdx + 0x28]
0x1415d3fa8 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1415d3fac 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1415d3fb0 4c8b0a mov r9, qword ptr [rdx]
0x1415d3fb3 488b4a08 mov rcx, qword ptr [rdx + 8]
0x1415d3fb7 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x1415d3fbb 4883fa07 cmp rdx, 7
0x1415d3fbf 7528 jne 0x1415d3fe9
0x1415d3fc1 8b01 mov eax, dword ptr [rcx]
0x1415d3fc3 3305bfb76b01 xor eax, dword ptr [rip + 0x16bb7bf]
0x1415d3fc9 448b4103 mov r8d, dword ptr [rcx + 3]
0x1415d3fcd 443305b7b76b01 xor r8d, dword ptr [rip + 0x16bb7b7]
0x1415d3fd4 4109c0 or r8d, eax
0x1415d3fd7 0f85a0000000 jne 0x1415d407d
0x1415d3fdd 31db xor ebx, ebx
0x1415d3fdf 4531f6 xor r14d, r14d
0x1415d3fe2 4d85c9 test r9, r9
0x1415d3fe5 7522 jne 0x1415d4009
0x1415d3fe7 eb2e jmp 0x1415d4017
0x1415d3fe9 4883fa08 cmp rdx, 8
0x1415d3fed 0f858a000000 jne 0x1415d407d
0x1415d3ff3 488b01 mov rax, qword ptr [rcx]
0x1415d3ff6 483b0592b76b01 cmp rax, qword ptr [rip + 0x16bb792]
0x1415d3ffd 757e jne 0x1415d407d
0x1415d3fff b301 mov bl, 1
0x1415d4001 4531f6 xor r14d, r14d
0x1415d4004 4d85c9 test r9, r9
0x1415d4007 740e je 0x1415d4017
0x1415d4009 41b801000000 mov r8d, 1
0x1415d400f 4c89ca mov rdx, r9
0x1415d4012 e809fc03ff call 0x140613c20
0x1415d4017 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d401b 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d401f 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1415d4023 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d4027 4584f6 test r14b, r14b
0x1415d402a 7416 je 0x1415d4042
0x1415d402c 48897e08 mov qword ptr [rsi + 8], rdi
0x1415d4030 c60602 mov byte ptr [rsi], 2
0x1415d4033 c6452700 mov byte ptr [rbp + 0x27], 0
0x1415d4037 488d4da0 lea rcx, [rbp - 0x60]
0x1415d403b e8f0a32bff call 0x14088e430
0x1415d4040 eb2a jmp 0x1415d406c
0x1415d4042 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d4046 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d404a 0f114de0 movups xmmword ptr [rbp - 0x20], xmm1
0x1415d404e 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1415d4052 881e mov byte ptr [rsi], bl
0x1415d4054 0f1045c9 movups xmm0, xmmword ptr [rbp - 0x37]
0x1415d4058 0f104dd9 movups xmm1, xmmword ptr [rbp - 0x27]
0x1415d405c 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x1415d4060 0f114e11 movups xmmword ptr [rsi + 0x11], xmm1
0x1415d4064 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1415d4068 48894620 mov qword ptr [rsi + 0x20], rax
0x1415d406c 4889f0 mov rax, rsi
0x1415d406f 4881c4b0000000 add rsp, 0xb0
0x1415d4076 5b pop rbx
0x1415d4077 5f pop rdi
0x1415d4078 5e pop rsi
0x1415d4079 415e pop r14
0x1415d407b 5d pop rbp
0x1415d407c c3 ret 
0x1415d407d 4c894d18 mov qword ptr [rbp + 0x18], r9
0x1415d4081 4c8d0510b76b01 lea r8, [rip + 0x16bb710]
0x1415d4088 41b902000000 mov r9d, 2
0x1415d408e 48894d10 mov qword ptr [rbp + 0x10], rcx
0x1415d4092 e8895d3201 call 0x1428f9e20
0x1415d4097 4889c7 mov rdi, rax
0x1415d409a 41b601 mov r14b, 1
0x1415d409d 4c8b4d18 mov r9, qword ptr [rbp + 0x18]
0x1415d40a1 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d40a5 4d85c9 test r9, r9
0x1415d40a8 0f855bffffff jne 0x1415d4009
0x1415d40ae e964ffffff jmp 0x1415d4017
0x1415d40b3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d40c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d40c5 55 push rbp
0x1415d40c6 4156 push r14
0x1415d40c8 56 push rsi
0x1415d40c9 57 push rdi
0x1415d40ca 53 push rbx
0x1415d40cb 4883ec20 sub rsp, 0x20
0x1415d40cf 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d40d6 48837d1800 cmp qword ptr [rbp + 0x18], 0
0x1415d40db 7413 je 0x1415d40f0
0x1415d40dd 41b801000000 mov r8d, 1
0x1415d40e3 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d40e7 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x1415d40eb e830fb03ff call 0x140613c20
0x1415d40f0 c6452701 mov byte ptr [rbp + 0x27], 1
0x1415d40f4 4883c420 add rsp, 0x20
0x1415d40f8 5b pop rbx
0x1415d40f9 5f pop rdi
0x1415d40fa 5e pop rsi
0x1415d40fb 415e pop r14
0x1415d40fd 5d pop rbp
0x1415d40fe c3 ret 
0x1415d40ff 90 nop 
0x1415d4100 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d4105 55 push rbp
0x1415d4106 4156 push r14
0x1415d4108 56 push rsi
0x1415d4109 57 push rdi
0x1415d410a 53 push rbx
0x1415d410b 4883ec20 sub rsp, 0x20
0x1415d410f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d4116 807d2700 cmp byte ptr [rbp + 0x27], 0
0x1415d411a 7409 je 0x1415d4125
0x1415d411c 488d4df0 lea rcx, [rbp - 0x10]
0x1415d4120 e80ba32bff call 0x14088e430
0x1415d4125 90 nop 
0x1415d4126 4883c420 add rsp, 0x20
0x1415d412a 5b pop rbx
0x1415d412b 5f pop rdi
0x1415d412c 5e pop rsi
0x1415d412d 415e pop r14
0x1415d412f 5d pop rbp
0x1415d4130 c3 ret 
