0x140a44a00 55 push rbp
0x140a44a01 56 push rsi
0x140a44a02 4881ec98000000 sub rsp, 0x98
0x140a44a09 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a44a11 48c74510feffffff mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
0x140a44a19 4989d1 mov r9, rdx
0x140a44a1c 4889ce mov rsi, rcx
0x140a44a1f 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a44a29 488945d8 mov qword ptr [rbp - 0x28], rax
0x140a44a2d 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x140a44a35 48c7450000000000 mov qword ptr [rbp], 0
0x140a44a3d c6450f01 mov byte ptr [rbp + 0xf], 1
0x140a44a41 488d15509a0802 lea rdx, [rip + 0x2089a50]
0x140a44a48 488d4dd8 lea rcx, [rbp - 0x28]
0x140a44a4c 41b803000000 mov r8d, 3
0x140a44a52 e8c90fb500 call 0x141595a20
0x140a44a57 4885c0 test rax, rax
0x140a44a5a 742f je 0x140a44a8b
0x140a44a5c 488d4df0 lea rcx, [rbp - 0x10]
0x140a44a60 48894608 mov qword ptr [rsi + 8], rax
0x140a44a64 c60606 mov byte ptr [rsi], 6
0x140a44a67 e8d444e5ff call 0x140898f40
0x140a44a6c 488b55d8 mov rdx, qword ptr [rbp - 0x28]
0x140a44a70 4889d0 mov rax, rdx
0x140a44a73 48f7d8 neg rax
0x140a44a76 703b jo 0x140a44ab3
0x140a44a78 7339 jae 0x140a44ab3
0x140a44a7a 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140a44a7e 41b801000000 mov r8d, 1
0x140a44a84 e897f1bcff call 0x140613c20
0x140a44a89 eb28 jmp 0x140a44ab3
0x140a44a8b 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
0x140a44a8f 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
0x140a44a93 0f1055f8 movups xmm2, xmmword ptr [rbp - 8]
0x140a44a97 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x140a44a9b 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140a44a9f 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140a44aa3 c6450f00 mov byte ptr [rbp + 0xf], 0
0x140a44aa7 488d55a0 lea rdx, [rbp - 0x60]
0x140a44aab 4889f1 mov rcx, rsi
0x140a44aae e80d1a1400 call 0x140b864c0
0x140a44ab3 4889f0 mov rax, rsi
0x140a44ab6 4881c498000000 add rsp, 0x98
0x140a44abd 5e pop rsi
0x140a44abe 5d pop rbp
0x140a44abf c3 ret 
0x140a44ac0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a44ac5 55 push rbp
0x140a44ac6 56 push rsi
0x140a44ac7 4883ec28 sub rsp, 0x28
0x140a44acb 488daa80000000 lea rbp, [rdx + 0x80]
0x140a44ad2 488b55d8 mov rdx, qword ptr [rbp - 0x28]
0x140a44ad6 488d045500000000 lea rax, [rdx*2]
0x140a44ade 4885c0 test rax, rax
0x140a44ae1 740f je 0x140a44af2
0x140a44ae3 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140a44ae7 41b801000000 mov r8d, 1
0x140a44aed e82ef1bcff call 0x140613c20
0x140a44af2 90 nop 
0x140a44af3 4883c428 add rsp, 0x28
0x140a44af7 5e pop rsi
0x140a44af8 5d pop rbp
0x140a44af9 c3 ret 
0x140a44afa 660f1f440000 nop word ptr [rax + rax]
0x140a44b00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a44b05 55 push rbp
0x140a44b06 56 push rsi
0x140a44b07 4883ec28 sub rsp, 0x28
0x140a44b0b 488daa80000000 lea rbp, [rdx + 0x80]
0x140a44b12 807d0f00 cmp byte ptr [rbp + 0xf], 0
0x140a44b16 7507 jne 0x140a44b1f
0x140a44b18 4883c428 add rsp, 0x28
0x140a44b1c 5e pop rsi
0x140a44b1d 5d pop rbp
0x140a44b1e c3 ret 
0x140a44b1f 488d4dd8 lea rcx, [rbp - 0x28]
0x140a44b23 e83890e1ff call 0x14085db60
0x140a44b28 ebee jmp 0x140a44b18
