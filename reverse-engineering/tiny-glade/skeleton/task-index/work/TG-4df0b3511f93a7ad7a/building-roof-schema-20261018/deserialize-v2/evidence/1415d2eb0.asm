0x1415d2eb0 55 push rbp
0x1415d2eb1 56 push rsi
0x1415d2eb2 4881ecc8000000 sub rsp, 0xc8
0x1415d2eb9 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d2ec1 48c74540feffffff mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x1415d2ec9 4889ce mov rsi, rcx
0x1415d2ecc 0f104218 movups xmm0, xmmword ptr [rdx + 0x18]
0x1415d2ed0 0f104a28 movups xmm1, xmmword ptr [rdx + 0x28]
0x1415d2ed4 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1415d2ed8 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1415d2edc 488b02 mov rax, qword ptr [rdx]
0x1415d2edf 48894530 mov qword ptr [rbp + 0x30], rax
0x1415d2ee3 488b4208 mov rax, qword ptr [rdx + 8]
0x1415d2ee7 4c8b4210 mov r8, qword ptr [rdx + 0x10]
0x1415d2eeb 488d4d18 lea rcx, [rbp + 0x18]
0x1415d2eef 48894528 mov qword ptr [rbp + 0x28], rax
0x1415d2ef3 4889c2 mov rdx, rax
0x1415d2ef6 e8c5691100 call 0x1416e98c0
0x1415d2efb 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1415d2eff 4885d2 test rdx, rdx
0x1415d2f02 740f je 0x1415d2f13
0x1415d2f04 41b801000000 mov r8d, 1
0x1415d2f0a 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1415d2f0e e80d0d04ff call 0x140613c20
0x1415d2f13 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d2f17 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d2f1b 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1415d2f1f 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d2f23 807d1800 cmp byte ptr [rbp + 0x18], 0
0x1415d2f27 741a je 0x1415d2f43
0x1415d2f29 488b4520 mov rax, qword ptr [rbp + 0x20]
0x1415d2f2d 48894608 mov qword ptr [rsi + 8], rax
0x1415d2f31 c60603 mov byte ptr [rsi], 3
0x1415d2f34 c6453f00 mov byte ptr [rbp + 0x3f], 0
0x1415d2f38 488d4da0 lea rcx, [rbp - 0x60]
0x1415d2f3c e8efb42bff call 0x14088e430
0x1415d2f41 eb2e jmp 0x1415d2f71
0x1415d2f43 0fb64519 movzx eax, byte ptr [rbp + 0x19]
0x1415d2f47 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d2f4b 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d2f4f 0f114de0 movups xmmword ptr [rbp - 0x20], xmm1
0x1415d2f53 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1415d2f57 8806 mov byte ptr [rsi], al
0x1415d2f59 0f1045c9 movups xmm0, xmmword ptr [rbp - 0x37]
0x1415d2f5d 0f104dd9 movups xmm1, xmmword ptr [rbp - 0x27]
0x1415d2f61 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x1415d2f65 0f114e11 movups xmmword ptr [rsi + 0x11], xmm1
0x1415d2f69 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1415d2f6d 48894620 mov qword ptr [rsi + 0x20], rax
0x1415d2f71 4889f0 mov rax, rsi
0x1415d2f74 4881c4c8000000 add rsp, 0xc8
0x1415d2f7b 5e pop rsi
0x1415d2f7c 5d pop rbp
0x1415d2f7d c3 ret 
0x1415d2f7e 6690 nop 
0x1415d2f80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2f85 55 push rbp
0x1415d2f86 56 push rsi
0x1415d2f87 4883ec28 sub rsp, 0x28
0x1415d2f8b 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2f92 48837d3000 cmp qword ptr [rbp + 0x30], 0
0x1415d2f97 7413 je 0x1415d2fac
0x1415d2f99 41b801000000 mov r8d, 1
0x1415d2f9f 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1415d2fa3 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1415d2fa7 e8740c04ff call 0x140613c20
0x1415d2fac c6453f01 mov byte ptr [rbp + 0x3f], 1
0x1415d2fb0 4883c428 add rsp, 0x28
0x1415d2fb4 5e pop rsi
0x1415d2fb5 5d pop rbp
0x1415d2fb6 c3 ret 
0x1415d2fb7 660f1f840000000000 nop word ptr [rax + rax]
0x1415d2fc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2fc5 55 push rbp
0x1415d2fc6 56 push rsi
0x1415d2fc7 4883ec28 sub rsp, 0x28
0x1415d2fcb 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2fd2 807d3f00 cmp byte ptr [rbp + 0x3f], 0
0x1415d2fd6 7409 je 0x1415d2fe1
0x1415d2fd8 488d4df0 lea rcx, [rbp - 0x10]
0x1415d2fdc e84fb42bff call 0x14088e430
0x1415d2fe1 90 nop 
0x1415d2fe2 4883c428 add rsp, 0x28
0x1415d2fe6 5e pop rsi
0x1415d2fe7 5d pop rbp
0x1415d2fe8 c3 ret 
