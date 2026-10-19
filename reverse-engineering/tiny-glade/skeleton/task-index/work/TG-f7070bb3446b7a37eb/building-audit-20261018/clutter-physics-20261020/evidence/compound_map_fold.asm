0x14157ddd0 4157 push r15
0x14157ddd2 4156 push r14
0x14157ddd4 4155 push r13
0x14157ddd6 4154 push r12
0x14157ddd8 56 push rsi
0x14157ddd9 57 push rdi
0x14157ddda 55 push rbp
0x14157dddb 53 push rbx
0x14157dddc 4883ec78 sub rsp, 0x78
0x14157dde0 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
0x14157dde6 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
0x14157ddeb 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x14157ddf0 488b6908 mov rbp, qword ptr [rcx + 8]
0x14157ddf4 48894c2420 mov qword ptr [rsp + 0x20], rcx
0x14157ddf9 488b4118 mov rax, qword ptr [rcx + 0x18]
0x14157ddfd 4889442430 mov qword ptr [rsp + 0x30], rax
0x14157de02 4839c5 cmp rbp, rax
0x14157de05 0f842c010000 je 0x14157df37
0x14157de0b 4c8b6a08 mov r13, qword ptr [rdx + 8]
0x14157de0f 488b4210 mov rax, qword ptr [rdx + 0x10]
0x14157de13 4a8d0c6d00000000 lea rcx, [r13*2]
0x14157de1b 4c01e9 add rcx, r13
0x14157de1e 48c1e104 shl rcx, 4
0x14157de22 488d3c01 lea rdi, [rcx + rax]
0x14157de26 4883c720 add rdi, 0x20
0x14157de2a 4889542428 mov qword ptr [rsp + 0x28], rdx
0x14157de2f eb3e jmp 0x14157de6f
0x14157de31 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14157de40 440f1147e0 movups xmmword ptr [rdi - 0x20], xmm8
0x14157de45 0f137ff0 movlps qword ptr [rdi - 0x10], xmm7
0x14157de49 0f12ff movhlps xmm7, xmm7
0x14157de4c f30f117ff8 movss dword ptr [rdi - 8], xmm7
0x14157de51 0f1137 movups xmmword ptr [rdi], xmm6
0x14157de54 49ffc5 inc r13
0x14157de57 488b542428 mov rdx, qword ptr [rsp + 0x28]
0x14157de5c 4c896a08 mov qword ptr [rdx + 8], r13
0x14157de60 4883c730 add rdi, 0x30
0x14157de64 483b6c2430 cmp rbp, qword ptr [rsp + 0x30]
0x14157de69 0f84cc000000 je 0x14157df3b
0x14157de6f 488b4d00 mov rcx, qword ptr [rbp]
0x14157de73 488b4508 mov rax, qword ptr [rbp + 8]
0x14157de77 4c8b6510 mov r12, qword ptr [rbp + 0x10]
0x14157de7b 488b5d18 mov rbx, qword ptr [rbp + 0x18]
0x14157de7f 0f287520 movaps xmm6, xmmword ptr [rbp + 0x20]
0x14157de83 4c8b7d30 mov r15, qword ptr [rbp + 0x30]
0x14157de87 488b7538 mov rsi, qword ptr [rbp + 0x38]
0x14157de8b 488b5540 mov rdx, qword ptr [rbp + 0x40]
0x14157de8f 4889542438 mov qword ptr [rsp + 0x38], rdx
0x14157de94 4c8b7548 mov r14, qword ptr [rbp + 0x48]
0x14157de98 0f287d50 movaps xmm7, xmmword ptr [rbp + 0x50]
0x14157de9c 440f284560 movaps xmm8, xmmword ptr [rbp + 0x60]
0x14157dea1 4883ed80 sub rbp, -0x80
0x14157dea5 488b542420 mov rdx, qword ptr [rsp + 0x20]
0x14157deaa 48896a08 mov qword ptr [rdx + 8], rbp
0x14157deae 4885c0 test rax, rax
0x14157deb1 7413 je 0x14157dec6
0x14157deb3 48c1e002 shl rax, 2
0x14157deb7 488d1440 lea rdx, [rax + rax*2]
0x14157debb 41b804000000 mov r8d, 4
0x14157dec1 e85a5d09ff call 0x140613c20
0x14157dec6 4885db test rbx, rbx
0x14157dec9 7416 je 0x14157dee1
0x14157decb 48c1e302 shl rbx, 2
0x14157decf 488d145b lea rdx, [rbx + rbx*2]
0x14157ded3 41b804000000 mov r8d, 4
0x14157ded9 4c89e1 mov rcx, r12
0x14157dedc e83f5d09ff call 0x140613c20
0x14157dee1 4d85ff test r15, r15
0x14157dee4 0f94c0 sete al
0x14157dee7 4885f6 test rsi, rsi
0x14157deea 0f94c1 sete cl
0x14157deed 08c1 or cl, al
0x14157deef 7516 jne 0x14157df07
0x14157def1 48c1e602 shl rsi, 2
0x14157def5 488d1476 lea rdx, [rsi + rsi*2]
0x14157def9 41b804000000 mov r8d, 4
0x14157deff 4c89f9 mov rcx, r15
0x14157df02 e8195d09ff call 0x140613c20
0x14157df07 488b4c2438 mov rcx, qword ptr [rsp + 0x38]
0x14157df0c 4885c9 test rcx, rcx
0x14157df0f 0f94c0 sete al
0x14157df12 4d85f6 test r14, r14
0x14157df15 0f94c2 sete dl
0x14157df18 08c2 or dl, al
0x14157df1a 0f8520ffffff jne 0x14157de40
0x14157df20 49c1e603 shl r14, 3
0x14157df24 41b804000000 mov r8d, 4
0x14157df2a 4c89f2 mov rdx, r14
0x14157df2d e8ee5c09ff call 0x140613c20
0x14157df32 e909ffffff jmp 0x14157de40
0x14157df37 4c8b6a08 mov r13, qword ptr [rdx + 8]
0x14157df3b 488b02 mov rax, qword ptr [rdx]
0x14157df3e 4c8928 mov qword ptr [rax], r13
0x14157df41 488b4c2420 mov rcx, qword ptr [rsp + 0x20]
0x14157df46 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x14157df4b 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
0x14157df50 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
0x14157df56 4883c478 add rsp, 0x78
0x14157df5a 5b pop rbx
0x14157df5b 5d pop rbp
0x14157df5c 5f pop rdi
0x14157df5d 5e pop rsi
0x14157df5e 415c pop r12
0x14157df60 415d pop r13
0x14157df62 415e pop r14
0x14157df64 415f pop r15
0x14157df66 e925bf2fff jmp 0x140879e90
