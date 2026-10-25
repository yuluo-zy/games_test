0x140680590 push rsi
0x140680591 push rdi
0x140680592 sub rsp, 0x28
0x140680596 mov rdi, rdx
0x140680599 mov rsi, rcx
0x14068059c call 0x140b92210
0x1406805a1 test al, al
0x1406805a3 jne 0x140680618
0x1406805a5 movss xmm0, dword ptr [rdi + 8]
0x1406805aa movss xmm1, dword ptr [rip + 0x22a52be]
0x1406805b2 movss xmm2, dword ptr [rdi + 0x10]
0x1406805b7 mulss xmm2, xmm1
0x1406805bb mulss xmm1, dword ptr [rdi + 0x14]
0x1406805c0 movss xmm3, dword ptr [rdi + 0xc]
0x1406805c5 movaps xmm4, xmm0
0x1406805c8 subss xmm4, xmm2
0x1406805cc movaps xmm5, xmm3
0x1406805cf subss xmm5, xmm1
0x1406805d3 addss xmm2, xmm0
0x1406805d7 addss xmm1, xmm3
0x1406805db add rsi, 0x18
0x1406805df mov rcx, qword ptr [rdi]
0x1406805e2 movd edx, xmm4
0x1406805e6 movd eax, xmm5
0x1406805ea shl rax, 0x20
0x1406805ee or rdx, rax
0x1406805f1 subss xmm2, xmm4
0x1406805f5 subss xmm1, xmm5
0x1406805f9 movd r8d, xmm2
0x1406805fe movd eax, xmm1
0x140680602 shl rax, 0x20
0x140680606 or r8, rax
0x140680609 movss xmm3, dword ptr [rdi + 0x18]
0x14068060e mov qword ptr [rsp + 0x20], rsi
0x140680613 call 0x140b8f510
0x140680618 nop 
0x140680619 add rsp, 0x28
0x14068061d pop rdi
0x14068061e pop rsi
0x14068061f ret 
