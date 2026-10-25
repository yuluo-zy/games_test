0x1408d0120 55 push rbp
0x1408d0121 4156 push r14
0x1408d0123 56 push rsi
0x1408d0124 57 push rdi
0x1408d0125 53 push rbx
0x1408d0126 4883ec70 sub rsp, 0x70
0x1408d012a 488d6c2470 lea rbp, [rsp + 0x70]
0x1408d012f 0f2975f0 movaps xmmword ptr [rbp - 0x10], xmm6
0x1408d0133 48c745e8feffffff mov qword ptr [rbp - 0x18], 0xfffffffffffffffe
0x1408d013b 4d89c6 mov r14, r8
0x1408d013e 0f28f1 movaps xmm6, xmm1
0x1408d0141 4889ce mov rsi, rcx
0x1408d0144 418b4008 mov eax, dword ptr [r8 + 8]
0x1408d0148 8945cc mov dword ptr [rbp - 0x34], eax
0x1408d014b 498b00 mov rax, qword ptr [r8]
0x1408d014e 488945c4 mov qword ptr [rbp - 0x3c], rax
0x1408d0152 f30f114dc0 movss dword ptr [rbp - 0x40], xmm1
0x1408d0157 488d4dc0 lea rcx, [rbp - 0x40]
0x1408d015b e880aeb800 call 0x14145afe0
0x1408d0160 4889c7 mov rdi, rax
0x1408d0163 0f28c6 movaps xmm0, xmm6
0x1408d0166 e8b5b7de00 call 0x1416bb920
0x1408d016b 4889d3 mov rbx, rdx
0x1408d016e 488945e0 mov qword ptr [rbp - 0x20], rax
0x1408d0172 488945b0 mov qword ptr [rbp - 0x50], rax
0x1408d0176 488955b8 mov qword ptr [rbp - 0x48], rdx
0x1408d017a 488d4dc0 lea rcx, [rbp - 0x40]
0x1408d017e 4c89f2 mov rdx, r14
0x1408d0181 e8aa193c00 call 0x140c91b30
0x1408d0186 48c70604000000 mov qword ptr [rsi], 4
0x1408d018d 48c7460800000000 mov qword ptr [rsi + 8], 0
0x1408d0195 48c7461004000000 mov qword ptr [rsi + 0x10], 4
0x1408d019d 48c7461800000000 mov qword ptr [rsi + 0x18], 0
0x1408d01a5 48c7463000000000 mov qword ptr [rsi + 0x30], 0
0x1408d01ad 48c7464000000000 mov qword ptr [rsi + 0x40], 0
0x1408d01b5 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x1408d01b9 48894620 mov qword ptr [rsi + 0x20], rax
0x1408d01bd 48895e28 mov qword ptr [rsi + 0x28], rbx
0x1408d01c1 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1408d01c5 0f284dd0 movaps xmm1, xmmword ptr [rbp - 0x30]
0x1408d01c9 0f294650 movaps xmmword ptr [rsi + 0x50], xmm0
0x1408d01cd 0f294e60 movaps xmmword ptr [rsi + 0x60], xmm1
0x1408d01d1 48897e70 mov qword ptr [rsi + 0x70], rdi
0x1408d01d5 4889f0 mov rax, rsi
0x1408d01d8 0f2875f0 movaps xmm6, xmmword ptr [rbp - 0x10]
0x1408d01dc 4883c470 add rsp, 0x70
0x1408d01e0 5b pop rbx
0x1408d01e1 5f pop rdi
0x1408d01e2 5e pop rsi
0x1408d01e3 415e pop r14
0x1408d01e5 5d pop rbp
0x1408d01e6 c3 ret
0x1408d01e7 660f1f840000000000 nop word ptr [rax + rax]
0x1408d01f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d01f5 55 push rbp
0x1408d01f6 4156 push r14
0x1408d01f8 56 push rsi
0x1408d01f9 57 push rdi
0x1408d01fa 53 push rbx
0x1408d01fb 4883ec30 sub rsp, 0x30
0x1408d01ff 488d6a70 lea rbp, [rdx + 0x70]
0x1408d0203 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1408d0208 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x1408d020c f048ff08 lock dec qword ptr [rax]
0x1408d0210 7509 jne 0x1408d021b
0x1408d0212 488d4db0 lea rcx, [rbp - 0x50]
0x1408d0216 e8b5f7d7ff call 0x14064f9d0
0x1408d021b 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1408d0220 4883c430 add rsp, 0x30
0x1408d0224 5b pop rbx
0x1408d0225 5f pop rdi
0x1408d0226 5e pop rsi
0x1408d0227 415e pop r14
0x1408d0229 5d pop rbp
0x1408d022a c3 ret
