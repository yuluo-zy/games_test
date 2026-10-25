0x141595a20 55 push rbp
0x141595a21 4157 push r15
0x141595a23 4156 push r14
0x141595a25 4154 push r12
0x141595a27 56 push rsi
0x141595a28 57 push rdi
0x141595a29 53 push rbx
0x141595a2a 4881ec90000000 sub rsp, 0x90
0x141595a31 488dac2480000000 lea rbp, [rsp + 0x80]
0x141595a39 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x141595a41 4c89c7 mov rdi, r8
0x141595a44 4d85c0 test r8, r8
0x141595a47 0f8866010000 js 0x141595bb3
0x141595a4d 4c89cb mov rbx, r9
0x141595a50 4889ce mov rsi, rcx
0x141595a53 7444 je 0x141595a99
0x141595a55 4989d7 mov r15, rdx
0x141595a58 0fb605aaf10902 movzx eax, byte ptr [rip + 0x209f1aa]
0x141595a5f 41bc01000000 mov r12d, 1
0x141595a65 ba01000000 mov edx, 1
0x141595a6a 4889f9 mov rcx, rdi
0x141595a6d e89ee107ff call 0x140613c10
0x141595a72 4885c0 test rax, rax
0x141595a75 0f843b010000 je 0x141595bb6
0x141595a7b 4989c6 mov r14, rax
0x141595a7e 4889c1 mov rcx, rax
0x141595a81 4c89fa mov rdx, r15
0x141595a84 4989f8 mov r8, rdi
0x141595a87 e874e13801 call 0x142923c00
0x141595a8c 488b16 mov rdx, qword ptr [rsi]
0x141595a8f 4889d0 mov rax, rdx
0x141595a92 48f7d8 neg rax
0x141595a95 7113 jno 0x141595aaa
0x141595a97 eb22 jmp 0x141595abb
0x141595a99 41be01000000 mov r14d, 1
0x141595a9f 488b16 mov rdx, qword ptr [rsi]
0x141595aa2 4889d0 mov rax, rdx
0x141595aa5 48f7d8 neg rax
0x141595aa8 7011 jo 0x141595abb
0x141595aaa 730f jae 0x141595abb
0x141595aac 488b4e08 mov rcx, qword ptr [rsi + 8]
0x141595ab0 41b801000000 mov r8d, 1
0x141595ab6 e865e107ff call 0x140613c20
0x141595abb 4c897608 mov qword ptr [rsi + 8], r14
0x141595abf 48897e10 mov qword ptr [rsi + 0x10], rdi
0x141595ac3 48b80000000000000080 movabs rax, 0x8000000000000000
0x141595acd 488906 mov qword ptr [rsi], rax
0x141595ad0 488b4608 mov rax, qword ptr [rsi + 8]
0x141595ad4 488945d0 mov qword ptr [rbp - 0x30], rax
0x141595ad8 488b4610 mov rax, qword ptr [rsi + 0x10]
0x141595adc 488945d8 mov qword ptr [rbp - 0x28], rax
0x141595ae0 48897dc8 mov qword ptr [rbp - 0x38], rdi
0x141595ae4 f30f1003 movss xmm0, dword ptr [rbx]
0x141595ae8 f30f5ac0 cvtss2sd xmm0, xmm0
0x141595aec 66480f7ec0 movq rax, xmm0
0x141595af1 48b9ffffffffffffff7f movabs rcx, 0x7fffffffffffffff
0x141595afb 4821c1 and rcx, rax
0x141595afe 48b8000000000000f07f movabs rax, 0x7ff0000000000000
0x141595b08 4839c1 cmp rcx, rax
0x141595b0b 0f9cc0 setl al
0x141595b0e 00c0 add al, al
0x141595b10 8845a8 mov byte ptr [rbp - 0x58], al
0x141595b13 48c745b002000000 mov qword ptr [rbp - 0x50], 2
0x141595b1b f20f1145b8 movsd qword ptr [rbp - 0x48], xmm0
0x141595b20 4883c618 add rsi, 0x18
0x141595b24 488d4de0 lea rcx, [rbp - 0x20]
0x141595b28 4c8d45c8 lea r8, [rbp - 0x38]
0x141595b2c 4c8d4da8 lea r9, [rbp - 0x58]
0x141595b30 4889f2 mov rdx, rsi
0x141595b33 e8f85eeaff call 0x14143ba30
0x141595b38 0fb645e0 movzx eax, byte ptr [rbp - 0x20]
0x141595b3c 4883f806 cmp rax, 6
0x141595b40 7754 ja 0x141595b96
0x141595b42 488d0d8f2b6e01 lea rcx, [rip + 0x16e2b8f]
0x141595b49 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x141595b4d 4801c8 add rax, rcx
0x141595b50 ffe0 jmp rax
0x141595b52 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141595b56 4885d2 test rdx, rdx
0x141595b59 7444 je 0x141595b9f
0x141595b5b 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141595b5f 41b801000000 mov r8d, 1
0x141595b65 eb28 jmp 0x141595b8f
0x141595b67 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141595b6b 488b55f8 mov rdx, qword ptr [rbp - 8]
0x141595b6f 48894d00 mov qword ptr [rbp], rcx
0x141595b73 e8f8312fff call 0x140888d70
0x141595b78 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141595b7c 4885d2 test rdx, rdx
0x141595b7f 741e je 0x141595b9f
0x141595b81 48c1e205 shl rdx, 5
0x141595b85 41b808000000 mov r8d, 8
0x141595b8b 488b4d00 mov rcx, qword ptr [rbp]
0x141595b8f e88ce007ff call 0x140613c20
0x141595b94 eb09 jmp 0x141595b9f
0x141595b96 488d4de8 lea rcx, [rbp - 0x18]
0x141595b9a e8a13330ff call 0x140898f40
0x141595b9f 31c0 xor eax, eax
0x141595ba1 4881c490000000 add rsp, 0x90
0x141595ba8 5b pop rbx
0x141595ba9 5f pop rdi
0x141595baa 5e pop rsi
0x141595bab 415c pop r12
0x141595bad 415e pop r14
0x141595baf 415f pop r15
0x141595bb1 5d pop rbp
0x141595bb2 c3 ret 
0x141595bb3 4531e4 xor r12d, r12d
0x141595bb6 4c8d051b266e01 lea r8, [rip + 0x16e261b]
0x141595bbd 4c89e1 mov rcx, r12
0x141595bc0 4889fa mov rdx, rdi
0x141595bc3 e81b343401 call 0x1428d8fe3
0x141595bc8 cc int3 
0x141595bc9 0f1f8000000000 nop dword ptr [rax]
0x141595bd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141595bd5 55 push rbp
0x141595bd6 4157 push r15
0x141595bd8 4156 push r14
0x141595bda 4154 push r12
0x141595bdc 56 push rsi
0x141595bdd 57 push rdi
0x141595bde 53 push rbx
0x141595bdf 4883ec20 sub rsp, 0x20
0x141595be3 488daa80000000 lea rbp, [rdx + 0x80]
0x141595bea 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141595bee 4885d2 test rdx, rdx
0x141595bf1 7413 je 0x141595c06
0x141595bf3 48c1e205 shl rdx, 5
0x141595bf7 41b808000000 mov r8d, 8
0x141595bfd 488b4d00 mov rcx, qword ptr [rbp]
0x141595c01 e81ae007ff call 0x140613c20
0x141595c06 90 nop 
0x141595c07 4883c420 add rsp, 0x20
0x141595c0b 5b pop rbx
0x141595c0c 5f pop rdi
0x141595c0d 5e pop rsi
0x141595c0e 415c pop r12
0x141595c10 415e pop r14
0x141595c12 415f pop r15
0x141595c14 5d pop rbp
0x141595c15 c3 ret 
