0x140ed7b00 55 push rbp
0x140ed7b01 56 push rsi
0x140ed7b02 4881ecc8000000 sub rsp, 0xc8
0x140ed7b09 488dac2480000000 lea rbp, [rsp + 0x80]
0x140ed7b11 48c74540feffffff mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x140ed7b19 48895530 mov qword ptr [rbp + 0x30], rdx
0x140ed7b1d 4889ce mov rsi, rcx
0x140ed7b20 4883c138 add rcx, 0x38
0x140ed7b24 c6453f01 mov byte ptr [rbp + 0x3f], 1
0x140ed7b28 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140ed7b32 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140ed7b3c e8af5b20ff call 0x1400dd6f0
0x140ed7b41 4885c0 test rax, rax
0x140ed7b44 0f849c000000 je 0x140ed7be6
0x140ed7b4a 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140ed7b51 0f868f000000 jbe 0x140ed7be6
0x140ed7b57 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140ed7b5e 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140ed7b62 4885c9 test rcx, rcx
0x140ed7b65 747f je 0x140ed7be6
0x140ed7b67 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140ed7b6e 48f7d1 not rcx
0x140ed7b71 488d0c49 lea rcx, [rcx + rcx*2]
0x140ed7b75 48c1e105 shl rcx, 5
0x140ed7b79 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140ed7b7f 7465 je 0x140ed7be6
0x140ed7b81 4801c8 add rax, rcx
0x140ed7b84 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140ed7b88 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140ed7b8e 89505c mov dword ptr [rax + 0x5c], edx
0x140ed7b91 488b4530 mov rax, qword ptr [rbp + 0x30]
0x140ed7b95 0f104060 movups xmm0, xmmword ptr [rax + 0x60]
0x140ed7b99 0f294500 movaps xmmword ptr [rbp], xmm0
0x140ed7b9d 0f104050 movups xmm0, xmmword ptr [rax + 0x50]
0x140ed7ba1 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x140ed7ba5 0f104040 movups xmm0, xmmword ptr [rax + 0x40]
0x140ed7ba9 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x140ed7bad 0f1000 movups xmm0, xmmword ptr [rax]
0x140ed7bb0 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x140ed7bb4 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
0x140ed7bb8 0f105830 movups xmm3, xmmword ptr [rax + 0x30]
0x140ed7bbc 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x140ed7bc0 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x140ed7bc4 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140ed7bc8 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140ed7bcc c6453f00 mov byte ptr [rbp + 0x3f], 0
0x140ed7bd0 488d55a0 lea rdx, [rbp - 0x60]
0x140ed7bd4 e8171d0000 call 0x140ed98f0
0x140ed7bd9 4889f0 mov rax, rsi
0x140ed7bdc 4881c4c8000000 add rsp, 0xc8
0x140ed7be3 5e pop rsi
0x140ed7be4 5d pop rbp
0x140ed7be5 c3 ret 
0x140ed7be6 488d0503d6cb01 lea rax, [rip + 0x1cbd603]
0x140ed7bed 48894510 mov qword ptr [rbp + 0x10], rax
0x140ed7bf1 48c7451827000000 mov qword ptr [rbp + 0x18], 0x27
0x140ed7bf9 488d4510 lea rax, [rbp + 0x10]
0x140ed7bfd 48894520 mov qword ptr [rbp + 0x20], rax
0x140ed7c01 488d05882223ff lea rax, [rip - 0xdcdd78]
0x140ed7c08 48894528 mov qword ptr [rbp + 0x28], rax
0x140ed7c0c 488d050dd7cb01 lea rax, [rip + 0x1cbd70d]
0x140ed7c13 488945a0 mov qword ptr [rbp - 0x60], rax
0x140ed7c17 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x140ed7c1f 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x140ed7c27 488d4520 lea rax, [rbp + 0x20]
0x140ed7c2b 488945b0 mov qword ptr [rbp - 0x50], rax
0x140ed7c2f 48c745b801000000 mov qword ptr [rbp - 0x48], 1
0x140ed7c37 c6453f01 mov byte ptr [rbp + 0x3f], 1
0x140ed7c3b 488d1596d5cb01 lea rdx, [rip + 0x1cbd596]
0x140ed7c42 488d4da0 lea rcx, [rbp - 0x60]
0x140ed7c46 e84517a001 call 0x1428d9390
0x140ed7c4b 0f0b ud2 
0x140ed7c4d 0f1f00 nop dword ptr [rax]
0x140ed7c50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ed7c55 55 push rbp
0x140ed7c56 56 push rsi
0x140ed7c57 4883ec28 sub rsp, 0x28
0x140ed7c5b 488daa80000000 lea rbp, [rdx + 0x80]
0x140ed7c62 807d3f00 cmp byte ptr [rbp + 0x3f], 0
0x140ed7c66 7507 jne 0x140ed7c6f
0x140ed7c68 4883c428 add rsp, 0x28
0x140ed7c6c 5e pop rsi
0x140ed7c6d 5d pop rbp
0x140ed7c6e c3 ret 
0x140ed7c6f 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x140ed7c73 e8081c15ff call 0x140029880
0x140ed7c78 ebee jmp 0x140ed7c68
