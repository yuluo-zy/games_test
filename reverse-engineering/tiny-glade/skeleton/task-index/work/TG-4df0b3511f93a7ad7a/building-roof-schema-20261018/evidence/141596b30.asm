0x141596b30 55 push rbp
0x141596b31 4157 push r15
0x141596b33 4156 push r14
0x141596b35 4154 push r12
0x141596b37 56 push rsi
0x141596b38 57 push rdi
0x141596b39 53 push rbx
0x141596b3a 4881ecb0000000 sub rsp, 0xb0
0x141596b41 488dac2480000000 lea rbp, [rsp + 0x80]
0x141596b49 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x141596b51 4d89c4 mov r12, r8
0x141596b54 4d85c0 test r8, r8
0x141596b57 0f8879010000 js 0x141596cd6
0x141596b5d 4c89cf mov rdi, r9
0x141596b60 4889ce mov rsi, rcx
0x141596b63 7444 je 0x141596ba9
0x141596b65 4989d6 mov r14, rdx
0x141596b68 0fb6059ae00902 movzx eax, byte ptr [rip + 0x209e09a]
0x141596b6f 41bf01000000 mov r15d, 1
0x141596b75 ba01000000 mov edx, 1
0x141596b7a 4c89e1 mov rcx, r12
0x141596b7d e88ed007ff call 0x140613c10
0x141596b82 4885c0 test rax, rax
0x141596b85 0f844e010000 je 0x141596cd9
0x141596b8b 4889c3 mov rbx, rax
0x141596b8e 4889c1 mov rcx, rax
0x141596b91 4c89f2 mov rdx, r14
0x141596b94 4d89e0 mov r8, r12
0x141596b97 e864d03801 call 0x142923c00
0x141596b9c 488b16 mov rdx, qword ptr [rsi]
0x141596b9f 4889d0 mov rax, rdx
0x141596ba2 48f7d8 neg rax
0x141596ba5 7112 jno 0x141596bb9
0x141596ba7 eb21 jmp 0x141596bca
0x141596ba9 bb01000000 mov ebx, 1
0x141596bae 488b16 mov rdx, qword ptr [rsi]
0x141596bb1 4889d0 mov rax, rdx
0x141596bb4 48f7d8 neg rax
0x141596bb7 7011 jo 0x141596bca
0x141596bb9 730f jae 0x141596bca
0x141596bbb 488b4e08 mov rcx, qword ptr [rsi + 8]
0x141596bbf 41b801000000 mov r8d, 1
0x141596bc5 e856d007ff call 0x140613c20
0x141596bca 48895e08 mov qword ptr [rsi + 8], rbx
0x141596bce 4c896610 mov qword ptr [rsi + 0x10], r12
0x141596bd2 48b80000000000000080 movabs rax, 0x8000000000000000
0x141596bdc 488906 mov qword ptr [rsi], rax
0x141596bdf 488b4608 mov rax, qword ptr [rsi + 8]
0x141596be3 488945f0 mov qword ptr [rbp - 0x10], rax
0x141596be7 488b4610 mov rax, qword ptr [rsi + 0x10]
0x141596beb 488945f8 mov qword ptr [rbp - 8], rax
0x141596bef 4c896520 mov qword ptr [rbp + 0x20], r12
0x141596bf3 4c8965e8 mov qword ptr [rbp - 0x18], r12
0x141596bf7 488d4dc8 lea rcx, [rbp - 0x38]
0x141596bfb 4889fa mov rdx, rdi
0x141596bfe e81d231500 call 0x1416e8f20
0x141596c03 807dc806 cmp byte ptr [rbp - 0x38], 6
0x141596c07 752b jne 0x141596c34
0x141596c09 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x141596c0d 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x141596c11 4885d2 test rdx, rdx
0x141596c14 0f84aa000000 je 0x141596cc4
0x141596c1a 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141596c1e 41b801000000 mov r8d, 1
0x141596c24 4889c6 mov rsi, rax
0x141596c27 e8f4cf07ff call 0x140613c20
0x141596c2c 4889f0 mov rax, rsi
0x141596c2f e990000000 jmp 0x141596cc4
0x141596c34 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
0x141596c38 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
0x141596c3c 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x141596c40 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x141596c44 4883c618 add rsi, 0x18
0x141596c48 4889e9 mov rcx, rbp
0x141596c4b 4c8d45e8 lea r8, [rbp - 0x18]
0x141596c4f 4c8d4da0 lea r9, [rbp - 0x60]
0x141596c53 4889f2 mov rdx, rsi
0x141596c56 e8d54deaff call 0x14143ba30
0x141596c5b 0fb64500 movzx eax, byte ptr [rbp]
0x141596c5f 4883f806 cmp rax, 6
0x141596c63 7754 ja 0x141596cb9
0x141596c65 488d0d4c1b6e01 lea rcx, [rip + 0x16e1b4c]
0x141596c6c 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x141596c70 4801c8 add rax, rcx
0x141596c73 ffe0 jmp rax
0x141596c75 488b5508 mov rdx, qword ptr [rbp + 8]
0x141596c79 4885d2 test rdx, rdx
0x141596c7c 7444 je 0x141596cc2
0x141596c7e 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x141596c82 41b801000000 mov r8d, 1
0x141596c88 eb28 jmp 0x141596cb2
0x141596c8a 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x141596c8e 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x141596c92 48894d20 mov qword ptr [rbp + 0x20], rcx
0x141596c96 e8d5202fff call 0x140888d70
0x141596c9b 488b5508 mov rdx, qword ptr [rbp + 8]
0x141596c9f 4885d2 test rdx, rdx
0x141596ca2 741e je 0x141596cc2
0x141596ca4 48c1e205 shl rdx, 5
0x141596ca8 41b808000000 mov r8d, 8
0x141596cae 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x141596cb2 e869cf07ff call 0x140613c20
0x141596cb7 eb09 jmp 0x141596cc2
0x141596cb9 488d4d08 lea rcx, [rbp + 8]
0x141596cbd e87e2230ff call 0x140898f40
0x141596cc2 31c0 xor eax, eax
0x141596cc4 4881c4b0000000 add rsp, 0xb0
0x141596ccb 5b pop rbx
0x141596ccc 5f pop rdi
0x141596ccd 5e pop rsi
0x141596cce 415c pop r12
0x141596cd0 415e pop r14
0x141596cd2 415f pop r15
0x141596cd4 5d pop rbp
0x141596cd5 c3 ret 
0x141596cd6 4531ff xor r15d, r15d
0x141596cd9 4c8d05f8146e01 lea r8, [rip + 0x16e14f8]
0x141596ce0 4c89f9 mov rcx, r15
0x141596ce3 4c89e2 mov rdx, r12
0x141596ce6 e8f8223401 call 0x1428d8fe3
0x141596ceb cc int3 
0x141596cec 0f1f4000 nop dword ptr [rax]
0x141596cf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141596cf5 55 push rbp
0x141596cf6 4157 push r15
0x141596cf8 4156 push r14
0x141596cfa 4154 push r12
0x141596cfc 56 push rsi
0x141596cfd 57 push rdi
0x141596cfe 53 push rbx
0x141596cff 4883ec20 sub rsp, 0x20
0x141596d03 488daa80000000 lea rbp, [rdx + 0x80]
0x141596d0a 488b5508 mov rdx, qword ptr [rbp + 8]
0x141596d0e 4885d2 test rdx, rdx
0x141596d11 7413 je 0x141596d26
0x141596d13 48c1e205 shl rdx, 5
0x141596d17 41b808000000 mov r8d, 8
0x141596d1d 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x141596d21 e8face07ff call 0x140613c20
0x141596d26 90 nop 
0x141596d27 4883c420 add rsp, 0x20
0x141596d2b 5b pop rbx
0x141596d2c 5f pop rdi
0x141596d2d 5e pop rsi
0x141596d2e 415c pop r12
0x141596d30 415e pop r14
0x141596d32 415f pop r15
0x141596d34 5d pop rbp
0x141596d35 c3 ret 
0x141596d36 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141596d40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141596d45 55 push rbp
0x141596d46 4157 push r15
0x141596d48 4156 push r14
0x141596d4a 4154 push r12
0x141596d4c 56 push rsi
0x141596d4d 57 push rdi
0x141596d4e 53 push rbx
0x141596d4f 4883ec20 sub rsp, 0x20
0x141596d53 488daa80000000 lea rbp, [rdx + 0x80]
0x141596d5a 48837d2000 cmp qword ptr [rbp + 0x20], 0
0x141596d5f 7413 je 0x141596d74
0x141596d61 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141596d65 41b801000000 mov r8d, 1
0x141596d6b 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x141596d6f e8acce07ff call 0x140613c20
0x141596d74 90 nop 
0x141596d75 4883c420 add rsp, 0x20
0x141596d79 5b pop rbx
0x141596d7a 5f pop rdi
0x141596d7b 5e pop rsi
0x141596d7c 415c pop r12
0x141596d7e 415e pop r14
0x141596d80 415f pop r15
0x141596d82 5d pop rbp
0x141596d83 c3 ret 
