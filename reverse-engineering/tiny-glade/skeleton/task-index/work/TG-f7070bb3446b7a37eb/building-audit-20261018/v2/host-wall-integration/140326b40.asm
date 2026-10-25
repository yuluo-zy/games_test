0x140326b40 55 push rbp
0x140326b41 56 push rsi
0x140326b42 4881ecc8000000 sub rsp, 0xc8
0x140326b49 488dac2480000000 lea rbp, [rsp + 0x80]
0x140326b51 48c74540feffffff mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x140326b59 48895530 mov qword ptr [rbp + 0x30], rdx
0x140326b5d 4889ce mov rsi, rcx
0x140326b60 4883c138 add rcx, 0x38
0x140326b64 c6453f01 mov byte ptr [rbp + 0x3f], 1
0x140326b68 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140326b72 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140326b7c e86f6bdbff call 0x1400dd6f0
0x140326b81 4885c0 test rax, rax
0x140326b84 0f849c000000 je 0x140326c26
0x140326b8a 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140326b91 0f868f000000 jbe 0x140326c26
0x140326b97 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140326b9e 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140326ba2 4885c9 test rcx, rcx
0x140326ba5 747f je 0x140326c26
0x140326ba7 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140326bae 48f7d1 not rcx
0x140326bb1 488d0c49 lea rcx, [rcx + rcx*2]
0x140326bb5 48c1e105 shl rcx, 5
0x140326bb9 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140326bbf 7465 je 0x140326c26
0x140326bc1 4801c8 add rax, rcx
0x140326bc4 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140326bc8 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140326bce 89505c mov dword ptr [rax + 0x5c], edx
0x140326bd1 488b4530 mov rax, qword ptr [rbp + 0x30]
0x140326bd5 0f104060 movups xmm0, xmmword ptr [rax + 0x60]
0x140326bd9 0f294500 movaps xmmword ptr [rbp], xmm0
0x140326bdd 0f104050 movups xmm0, xmmword ptr [rax + 0x50]
0x140326be1 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x140326be5 0f104040 movups xmm0, xmmword ptr [rax + 0x40]
0x140326be9 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x140326bed 0f1000 movups xmm0, xmmword ptr [rax]
0x140326bf0 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x140326bf4 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
0x140326bf8 0f105830 movups xmm3, xmmword ptr [rax + 0x30]
0x140326bfc 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x140326c00 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x140326c04 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140326c08 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140326c0c c6453f00 mov byte ptr [rbp + 0x3f], 0
0x140326c10 488d55a0 lea rdx, [rbp - 0x60]
0x140326c14 e807a8e4ff call 0x140171420
0x140326c19 4889f0 mov rax, rsi
0x140326c1c 4881c4c8000000 add rsp, 0xc8
0x140326c23 5e pop rsi
0x140326c24 5d pop rbp
0x140326c25 c3 ret 
0x140326c26 488d050b766902 lea rax, [rip + 0x269760b]
0x140326c2d 48894510 mov qword ptr [rbp + 0x10], rax
0x140326c31 48c7451827000000 mov qword ptr [rbp + 0x18], 0x27
0x140326c39 488d4510 lea rax, [rbp + 0x10]
0x140326c3d 48894520 mov qword ptr [rbp + 0x20], rax
0x140326c41 488d054832deff lea rax, [rip - 0x21cdb8]
0x140326c48 48894528 mov qword ptr [rbp + 0x28], rax
0x140326c4c 488d0515776902 lea rax, [rip + 0x2697715]
0x140326c53 488945a0 mov qword ptr [rbp - 0x60], rax
0x140326c57 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x140326c5f 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x140326c67 488d4520 lea rax, [rbp + 0x20]
0x140326c6b 488945b0 mov qword ptr [rbp - 0x50], rax
0x140326c6f 48c745b801000000 mov qword ptr [rbp - 0x48], 1
0x140326c77 c6453f01 mov byte ptr [rbp + 0x3f], 1
0x140326c7b 488d156e756902 lea rdx, [rip + 0x269756e]
0x140326c82 488d4da0 lea rcx, [rbp - 0x60]
0x140326c86 e805275b02 call 0x1428d9390
0x140326c8b 0f0b ud2 
0x140326c8d 0f1f00 nop dword ptr [rax]
0x140326c90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140326c95 55 push rbp
0x140326c96 56 push rsi
0x140326c97 4883ec28 sub rsp, 0x28
0x140326c9b 488daa80000000 lea rbp, [rdx + 0x80]
0x140326ca2 807d3f00 cmp byte ptr [rbp + 0x3f], 0
0x140326ca6 7507 jne 0x140326caf
0x140326ca8 4883c428 add rsp, 0x28
0x140326cac 5e pop rsi
0x140326cad 5d pop rbp
0x140326cae c3 ret 
0x140326caf 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x140326cb3 e8c82bd0ff call 0x140029880
0x140326cb8 ebee jmp 0x140326ca8
