0x1408d0230 55 push rbp
0x1408d0231 4157 push r15
0x1408d0233 4156 push r14
0x1408d0235 4155 push r13
0x1408d0237 4154 push r12
0x1408d0239 56 push rsi
0x1408d023a 57 push rdi
0x1408d023b 53 push rbx
0x1408d023c 4883ec78 sub rsp, 0x78
0x1408d0240 488d6c2470 lea rbp, [rsp + 0x70]
0x1408d0245 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x1408d024d 4c89c7 mov rdi, r8
0x1408d0250 488955f0 mov qword ptr [rbp - 0x10], rdx
0x1408d0254 4889ce mov rsi, rcx
0x1408d0257 c645ff01 mov byte ptr [rbp - 1], 1
0x1408d025b 4c89c1 mov rcx, r8
0x1408d025e e8bdbbb800 call 0x14145be20
0x1408d0263 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x1408d0267 4c8b7108 mov r14, qword ptr [rcx + 8]
0x1408d026b 488b4910 mov rcx, qword ptr [rcx + 0x10]
0x1408d026f 4989cc mov r12, rcx
0x1408d0272 49c1e407 shl r12, 7
0x1408d0276 4f8d3c26 lea r15, [r14 + r12]
0x1408d027a 4885c9 test rcx, rcx
0x1408d027d 742f je 0x1408d02ae
0x1408d027f 4531ed xor r13d, r13d
0x1408d0282 488d5dd0 lea rbx, [rbp - 0x30]
0x1408d0286 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408d0290 4b8b4c2e70 mov rcx, qword ptr [r14 + r13 + 0x70]
0x1408d0295 488945d0 mov qword ptr [rbp - 0x30], rax
0x1408d0299 48894dd8 mov qword ptr [rbp - 0x28], rcx
0x1408d029d 4889d9 mov rcx, rbx
0x1408d02a0 e86bd6ceff call 0x1405bd910
0x1408d02a5 4983ed80 sub r13, -0x80
0x1408d02a9 4d39ec cmp r12, r13
0x1408d02ac 75e2 jne 0x1408d0290
0x1408d02ae 4889c3 mov rbx, rax
0x1408d02b1 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1408d02b5 488b00 mov rax, qword ptr [rax]
0x1408d02b8 4c8975d0 mov qword ptr [rbp - 0x30], r14
0x1408d02bc 4c8975d8 mov qword ptr [rbp - 0x28], r14
0x1408d02c0 488945e0 mov qword ptr [rbp - 0x20], rax
0x1408d02c4 4c897de8 mov qword ptr [rbp - 0x18], r15
0x1408d02c8 c645ff00 mov byte ptr [rbp - 1], 0
0x1408d02cc 4c8d05bd321d02 lea r8, [rip + 0x21d32bd]
0x1408d02d3 488d4db8 lea rcx, [rbp - 0x48]
0x1408d02d7 488d55d0 lea rdx, [rbp - 0x30]
0x1408d02db e8f06ca200 call 0x1412f6fd0
0x1408d02e0 c645ff00 mov byte ptr [rbp - 1], 0
0x1408d02e4 488d4db8 lea rcx, [rbp - 0x48]
0x1408d02e8 e823b5de00 call 0x1416bb810
0x1408d02ed 48c70604000000 mov qword ptr [rsi], 4
0x1408d02f4 48c7460800000000 mov qword ptr [rsi + 8], 0
0x1408d02fc 48c7461004000000 mov qword ptr [rsi + 0x10], 4
0x1408d0304 48c7461800000000 mov qword ptr [rsi + 0x18], 0
0x1408d030c 48c7463000000000 mov qword ptr [rsi + 0x30], 0
0x1408d0314 48c7464000000000 mov qword ptr [rsi + 0x40], 0
0x1408d031c 48894620 mov qword ptr [rsi + 0x20], rax
0x1408d0320 48895628 mov qword ptr [rsi + 0x28], rdx
0x1408d0324 0f2807 movaps xmm0, xmmword ptr [rdi]
0x1408d0327 0f284f10 movaps xmm1, xmmword ptr [rdi + 0x10]
0x1408d032b 0f294650 movaps xmmword ptr [rsi + 0x50], xmm0
0x1408d032f 0f294e60 movaps xmmword ptr [rsi + 0x60], xmm1
0x1408d0333 48895e70 mov qword ptr [rsi + 0x70], rbx
0x1408d0337 4889f0 mov rax, rsi
0x1408d033a 4883c478 add rsp, 0x78
0x1408d033e 5b pop rbx
0x1408d033f 5f pop rdi
0x1408d0340 5e pop rsi
0x1408d0341 415c pop r12
0x1408d0343 415d pop r13
0x1408d0345 415e pop r14
0x1408d0347 415f pop r15
0x1408d0349 5d pop rbp
0x1408d034a c3 ret
0x1408d034b 0f1f440000 nop dword ptr [rax + rax]
0x1408d0350 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d0355 55 push rbp
0x1408d0356 4157 push r15
0x1408d0358 4156 push r14
0x1408d035a 4155 push r13
0x1408d035c 4154 push r12
0x1408d035e 56 push rsi
0x1408d035f 57 push rdi
0x1408d0360 53 push rbx
0x1408d0361 4883ec28 sub rsp, 0x28
0x1408d0365 488d6a70 lea rbp, [rdx + 0x70]
0x1408d0369 807dff00 cmp byte ptr [rbp - 1], 0
0x1408d036d 7511 jne 0x1408d0380
0x1408d036f 4883c428 add rsp, 0x28
0x1408d0373 5b pop rbx
0x1408d0374 5f pop rdi
0x1408d0375 5e pop rsi
0x1408d0376 415c pop r12
0x1408d0378 415d pop r13
0x1408d037a 415e pop r14
0x1408d037c 415f pop r15
0x1408d037e 5d pop rbp
0x1408d037f c3 ret
0x1408d0380 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x1408d0384 e8975dfaff call 0x140876120
0x1408d0389 ebe4 jmp 0x1408d036f
