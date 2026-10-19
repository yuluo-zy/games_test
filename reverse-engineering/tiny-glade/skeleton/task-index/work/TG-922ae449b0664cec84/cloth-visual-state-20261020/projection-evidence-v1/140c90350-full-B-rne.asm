140c90350: 55 push rbp
140c90351: 4157 push r15
140c90353: 4156 push r14
140c90355: 4154 push r12
140c90357: 56 push rsi
140c90358: 57 push rdi
140c90359: 53 push rbx
140c9035a: 4883ec60 sub rsp, 0x60
140c9035e: 488d6c2460 lea rbp, [rsp + 0x60]
140c90363: 0f2975f0 movaps xmmword ptr [rbp - 0x10], xmm6
140c90367: 48c745e8feffffff mov qword ptr [rbp - 0x18], 0xfffffffffffffffe
140c9036f: 488b7d60 mov rdi, qword ptr [rbp + 0x60]
140c90373: 4883fa01 cmp rdx, 1
140c90377: 0f86a2000000 jbe 0x140c9041f
140c9037d: 4989d6 mov r14, rdx
140c90380: 4c8d3c9500000000 lea r15, [rdx*4]
140c90388: 4531e4 xor r12d, r12d
140c9038b: 4889d0 mov rax, rdx
140c9038e: 48c1e83e shr rax, 0x3e
140c90392: 0f859b000000 jne 0x140c90433
140c90398: 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
140c903a2: 4939c7 cmp r15, rax
140c903a5: 0f8788000000 ja 0x140c90433
140c903ab: 4c89cb mov rbx, r9
140c903ae: 0f28f2 movaps xmm6, xmm2
140c903b1: 4889ce mov rsi, rcx
140c903b4: 0fb6054e489a02 movzx eax, byte ptr [rip + 0x29a484e]
140c903bb: 41bc04000000 mov r12d, 4
140c903c1: ba04000000 mov edx, 4
140c903c6: 4c89f9 mov rcx, r15
140c903c9: e8423898ff call 0x140613c10
140c903ce: 4885c0 test rax, rax
140c903d1: 7460 je 0x140c90433
140c903d3: 4c8975d0 mov qword ptr [rbp - 0x30], r14
140c903d7: 488945d8 mov qword ptr [rbp - 0x28], rax
140c903db: 48c745e000000000 mov qword ptr [rbp - 0x20], 0
140c903e3: 48897c2420 mov qword ptr [rsp + 0x20], rdi
140c903e8: 4c8d4dd0 lea r9, [rbp - 0x30]
140c903ec: 4c89f1 mov rcx, r14
140c903ef: 0f28ce movaps xmm1, xmm6
140c903f2: 4989d8 mov r8, rbx
140c903f5: e856fcffff call 0x140c90050
140c903fa: 488b45e0 mov rax, qword ptr [rbp - 0x20]
140c903fe: 48894610 mov qword ptr [rsi + 0x10], rax
140c90402: 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
140c90406: 0f1106 movups xmmword ptr [rsi], xmm0
140c90409: 4889f0 mov rax, rsi
140c9040c: 0f2875f0 movaps xmm6, xmmword ptr [rbp - 0x10]
140c90410: 4883c460 add rsp, 0x60
140c90414: 5b pop rbx
140c90415: 5f pop rdi
140c90416: 5e pop rsi
140c90417: 415c pop r12
140c90419: 415e pop r14
140c9041b: 415f pop r15
140c9041d: 5d pop rbp
140c9041e: c3 ret 
140c9041f: 488d0d5ad4e901 lea rcx, [rip + 0x1e9d45a]
140c90426: ba1d000000 mov edx, 0x1d
140c9042b: 4989f8 mov r8, rdi
140c9042e: e8fd8fc401 call 0x1428d9430
140c90433: 4c89e1 mov rcx, r12
140c90436: 4c89fa mov rdx, r15
140c90439: 4989f8 mov r8, rdi
140c9043c: e8a28bc401 call 0x1428d8fe3
140c90441: cc int3 
140c90442: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140c90450: 4889542410 mov qword ptr [rsp + 0x10], rdx
140c90455: 55 push rbp
140c90456: 4157 push r15
140c90458: 4156 push r14
140c9045a: 4154 push r12
140c9045c: 56 push rsi
140c9045d: 57 push rdi
140c9045e: 53 push rbx
140c9045f: 4883ec40 sub rsp, 0x40
140c90463: 488d6a60 lea rbp, [rdx + 0x60]
140c90467: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
140c9046c: 488b55d0 mov rdx, qword ptr [rbp - 0x30]
140c90470: 4885d2 test rdx, rdx
140c90473: 7413 je 0x140c90488
140c90475: 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
140c90479: 48c1e202 shl rdx, 2
140c9047d: 41b804000000 mov r8d, 4
140c90483: e8983798ff call 0x140613c20
140c90488: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
140c9048d: 4883c440 add rsp, 0x40
140c90491: 5b pop rbx
140c90492: 5f pop rdi
140c90493: 5e pop rsi
140c90494: 415c pop r12
140c90496: 415e pop r14
140c90498: 415f pop r15
140c9049a: 5d pop rbp
140c9049b: c3 ret 
