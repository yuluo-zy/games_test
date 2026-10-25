0x14158faf0 55 push rbp
0x14158faf1 4157 push r15
0x14158faf3 4156 push r14
0x14158faf5 4155 push r13
0x14158faf7 4154 push r12
0x14158faf9 56 push rsi
0x14158fafa 57 push rdi
0x14158fafb 53 push rbx
0x14158fafc 4881ec98000000 sub rsp, 0x98
0x14158fb03 488dac2480000000 lea rbp, [rsp + 0x80]
0x14158fb0b 48c74510feffffff mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
0x14158fb13 4889ce mov rsi, rcx
0x14158fb16 488b7a08 mov rdi, qword ptr [rdx + 8]
0x14158fb1a 488b5a10 mov rbx, qword ptr [rdx + 0x10]
0x14158fb1e 488d4dc8 lea rcx, [rbp - 0x38]
0x14158fb22 ba01000000 mov edx, 1
0x14158fb27 4989d8 mov r8, rbx
0x14158fb2a e841675fff call 0x140b86270
0x14158fb2f 31c0 xor eax, eax
0x14158fb31 483b45c8 cmp rax, qword ptr [rbp - 0x38]
0x14158fb35 7110 jno 0x14158fb47
0x14158fb37 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x14158fb3b 48894608 mov qword ptr [rsi + 8], rax
0x14158fb3f c60606 mov byte ptr [rsi], 6
0x14158fb42 e9f2000000 jmp 0x14158fc39
0x14158fb47 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x14158fb4b 48894500 mov qword ptr [rbp], rax
0x14158fb4f 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
0x14158fb53 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x14158fb57 4885db test rbx, rbx
0x14158fb5a 7478 je 0x14158fbd4
0x14158fb5c 4c6be358 imul r12, rbx, 0x58
0x14158fb60 4901fc add r12, rdi
0x14158fb63 488d5da8 lea rbx, [rbp - 0x58]
0x14158fb67 4c8d352a876e01 lea r14, [rip + 0x16e872a]
0x14158fb6e 4c8d7df0 lea r15, [rbp - 0x10]
0x14158fb72 eb38 jmp 0x14158fbac
0x14158fb74 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14158fb80 4883c758 add rdi, 0x58
0x14158fb84 488b45f8 mov rax, qword ptr [rbp - 8]
0x14158fb88 4c89e9 mov rcx, r13
0x14158fb8b 48c1e105 shl rcx, 5
0x14158fb8f 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x14158fb93 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x14158fb97 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
0x14158fb9c 0f110408 movups xmmword ptr [rax + rcx], xmm0
0x14158fba0 49ffc5 inc r13
0x14158fba3 4c896d00 mov qword ptr [rbp], r13
0x14158fba7 4c39e7 cmp rdi, r12
0x14158fbaa 7428 je 0x14158fbd4
0x14158fbac 4889d9 mov rcx, rbx
0x14158fbaf 4889fa mov rdx, rdi
0x14158fbb2 e849ab1500 call 0x1416ea700
0x14158fbb7 807da806 cmp byte ptr [rbp - 0x58], 6
0x14158fbbb 7444 je 0x14158fc01
0x14158fbbd 4c8b6d00 mov r13, qword ptr [rbp]
0x14158fbc1 4c3b6df0 cmp r13, qword ptr [rbp - 0x10]
0x14158fbc5 75b9 jne 0x14158fb80
0x14158fbc7 4c89f9 mov rcx, r15
0x14158fbca 4c89f2 mov rdx, r14
0x14158fbcd e88e245fff call 0x140b82060
0x14158fbd2 ebac jmp 0x14158fb80
0x14158fbd4 488b4500 mov rax, qword ptr [rbp]
0x14158fbd8 488945df mov qword ptr [rbp - 0x21], rax
0x14158fbdc 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x14158fbe0 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x14158fbe4 c60604 mov byte ptr [rsi], 4
0x14158fbe7 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
0x14158fbeb 488b45d7 mov rax, qword ptr [rbp - 0x29]
0x14158fbef 488b4ddf mov rcx, qword ptr [rbp - 0x21]
0x14158fbf3 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x14158fbf7 48894610 mov qword ptr [rsi + 0x10], rax
0x14158fbfb 48894e18 mov qword ptr [rsi + 0x18], rcx
0x14158fbff eb38 jmp 0x14158fc39
0x14158fc01 488b45b0 mov rax, qword ptr [rbp - 0x50]
0x14158fc05 48894608 mov qword ptr [rsi + 8], rax
0x14158fc09 c60606 mov byte ptr [rsi], 6
0x14158fc0c 488b4df8 mov rcx, qword ptr [rbp - 8]
0x14158fc10 488b5500 mov rdx, qword ptr [rbp]
0x14158fc14 48894de8 mov qword ptr [rbp - 0x18], rcx
0x14158fc18 e853912fff call 0x140888d70
0x14158fc1d 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x14158fc21 4885d2 test rdx, rdx
0x14158fc24 7413 je 0x14158fc39
0x14158fc26 48c1e205 shl rdx, 5
0x14158fc2a 41b808000000 mov r8d, 8
0x14158fc30 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14158fc34 e8e73f08ff call 0x140613c20
0x14158fc39 4889f0 mov rax, rsi
0x14158fc3c 4881c498000000 add rsp, 0x98
0x14158fc43 5b pop rbx
0x14158fc44 5f pop rdi
0x14158fc45 5e pop rsi
0x14158fc46 415c pop r12
0x14158fc48 415d pop r13
0x14158fc4a 415e pop r14
0x14158fc4c 415f pop r15
0x14158fc4e 5d pop rbp
0x14158fc4f c3 ret 
0x14158fc50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158fc55 55 push rbp
0x14158fc56 4157 push r15
0x14158fc58 4156 push r14
0x14158fc5a 4155 push r13
0x14158fc5c 4154 push r12
0x14158fc5e 56 push rsi
0x14158fc5f 57 push rdi
0x14158fc60 53 push rbx
0x14158fc61 4883ec28 sub rsp, 0x28
0x14158fc65 488daa80000000 lea rbp, [rdx + 0x80]
0x14158fc6c 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x14158fc70 4885d2 test rdx, rdx
0x14158fc73 7413 je 0x14158fc88
0x14158fc75 48c1e205 shl rdx, 5
0x14158fc79 41b808000000 mov r8d, 8
0x14158fc7f 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14158fc83 e8983f08ff call 0x140613c20
0x14158fc88 90 nop 
0x14158fc89 4883c428 add rsp, 0x28
0x14158fc8d 5b pop rbx
0x14158fc8e 5f pop rdi
0x14158fc8f 5e pop rsi
0x14158fc90 415c pop r12
0x14158fc92 415d pop r13
0x14158fc94 415e pop r14
0x14158fc96 415f pop r15
0x14158fc98 5d pop rbp
0x14158fc99 c3 ret 
0x14158fc9a 660f1f440000 nop word ptr [rax + rax]
0x14158fca0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158fca5 55 push rbp
0x14158fca6 4157 push r15
0x14158fca8 4156 push r14
0x14158fcaa 4155 push r13
0x14158fcac 4154 push r12
0x14158fcae 56 push rsi
0x14158fcaf 57 push rdi
0x14158fcb0 53 push rbx
0x14158fcb1 4883ec28 sub rsp, 0x28
0x14158fcb5 488daa80000000 lea rbp, [rdx + 0x80]
0x14158fcbc 488d4df0 lea rcx, [rbp - 0x10]
0x14158fcc0 e8cb962fff call 0x140889390
0x14158fcc5 90 nop 
0x14158fcc6 4883c428 add rsp, 0x28
0x14158fcca 5b pop rbx
0x14158fccb 5f pop rdi
0x14158fccc 5e pop rsi
0x14158fccd 415c pop r12
0x14158fccf 415d pop r13
0x14158fcd1 415e pop r14
0x14158fcd3 415f pop r15
0x14158fcd5 5d pop rbp
0x14158fcd6 c3 ret 
0x14158fcd7 660f1f840000000000 nop word ptr [rax + rax]
0x14158fce0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158fce5 55 push rbp
0x14158fce6 4157 push r15
0x14158fce8 4156 push r14
0x14158fcea 4155 push r13
0x14158fcec 4154 push r12
0x14158fcee 56 push rsi
0x14158fcef 57 push rdi
0x14158fcf0 53 push rbx
0x14158fcf1 4883ec28 sub rsp, 0x28
0x14158fcf5 488daa80000000 lea rbp, [rdx + 0x80]
0x14158fcfc 488d4da8 lea rcx, [rbp - 0x58]
0x14158fd00 e88bdd2cff call 0x14085da90
0x14158fd05 90 nop 
0x14158fd06 4883c428 add rsp, 0x28
0x14158fd0a 5b pop rbx
0x14158fd0b 5f pop rdi
0x14158fd0c 5e pop rsi
0x14158fd0d 415c pop r12
0x14158fd0f 415d pop r13
0x14158fd11 415e pop r14
0x14158fd13 415f pop r15
0x14158fd15 5d pop rbp
0x14158fd16 c3 ret 
