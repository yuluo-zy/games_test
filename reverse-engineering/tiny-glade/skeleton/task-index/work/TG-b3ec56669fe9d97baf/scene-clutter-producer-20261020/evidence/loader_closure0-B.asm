0x1422c1cc0 55 push rbp
0x1422c1cc1 4157 push r15
0x1422c1cc3 4156 push r14
0x1422c1cc5 4155 push r13
0x1422c1cc7 4154 push r12
0x1422c1cc9 56 push rsi
0x1422c1cca 57 push rdi
0x1422c1ccb 53 push rbx
0x1422c1ccc 4881ec78010000 sub rsp, 0x178
0x1422c1cd3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1422c1cdb 0f29b5e0000000 movaps xmmword ptr [rbp + 0xe0], xmm6
0x1422c1ce2 48c785d8000000feffffff mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
0x1422c1ced 4989ce mov r14, rcx
0x1422c1cf0 440fb6ad88010000 movzx r13d, byte ptr [rbp + 0x188]
0x1422c1cf8 440fb6a578010000 movzx r12d, byte ptr [rbp + 0x178]
0x1422c1d00 440fb6bd70010000 movzx r15d, byte ptr [rbp + 0x170]
0x1422c1d08 488bb568010000 mov rsi, qword ptr [rbp + 0x168]
0x1422c1d0f 488bbd60010000 mov rdi, qword ptr [rbp + 0x160]
0x1422c1d16 0fb68590010000 movzx eax, byte ptr [rbp + 0x190]
0x1422c1d1d 488b0d74a33601 mov rcx, qword ptr [rip + 0x136a374]
0x1422c1d24 8401 test byte ptr [rcx], al
0x1422c1d26 0f84b0000000 je 0x1422c1ddc
0x1422c1d2c 4889f9 mov rcx, rdi
0x1422c1d2f 4889f2 mov rdx, rsi
0x1422c1d32 e849b09dfe call 0x140c9cd80
0x1422c1d37 4889c3 mov rbx, rax
0x1422c1d3a c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1422c1d44 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
0x1422c1d4f c7455800000000 mov dword ptr [rbp + 0x58], 0
0x1422c1d56 48c7455000000000 mov qword ptr [rbp + 0x50], 0
0x1422c1d5e 488d051bead700 lea rax, [rip + 0xd7ea1b]
0x1422c1d65 4889442428 mov qword ptr [rsp + 0x28], rax
0x1422c1d6a c744242000000000 mov dword ptr [rsp + 0x20], 0
0x1422c1d72 488d4dd0 lea rcx, [rbp - 0x30]
0x1422c1d76 488d95c8000000 lea rdx, [rbp + 0xc8]
0x1422c1d7d 4c8d4550 lea r8, [rbp + 0x50]
0x1422c1d81 f30f101d7b3b6600 movss xmm3, dword ptr [rip + 0x663b7b]
0x1422c1d89 e8f2659dfe call 0x140c98380
0x1422c1d8e 0fb605742e3701 movzx eax, byte ptr [rip + 0x1372e74]
0x1422c1d95 b948000000 mov ecx, 0x48
0x1422c1d9a ba08000000 mov edx, 8
0x1422c1d9f e86c1e35fe call 0x140613c10
0x1422c1da4 4885c0 test rax, rax
0x1422c1da7 0f84bf010000 je 0x1422c1f6c
0x1422c1dad 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1422c1db1 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1422c1db5 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1422c1db9 0f1100 movups xmmword ptr [rax], xmm0
0x1422c1dbc 48897820 mov qword ptr [rax + 0x20], rdi
0x1422c1dc0 48897028 mov qword ptr [rax + 0x28], rsi
0x1422c1dc4 48c74030ffffffff mov qword ptr [rax + 0x30], 0xffffffffffffffff
0x1422c1dcc 48895838 mov qword ptr [rax + 0x38], rbx
0x1422c1dd0 c740400000803f mov dword ptr [rax + 0x40], 0x3f800000
0x1422c1dd7 e95f010000 jmp 0x1422c1f3b
0x1422c1ddc 4889d3 mov rbx, rdx
0x1422c1ddf 488d4dd0 lea rcx, [rbp - 0x30]
0x1422c1de3 4c89c2 mov rdx, r8
0x1422c1de6 4d89c8 mov r8, r9
0x1422c1de9 e842613400 call 0x142607f30
0x1422c1dee 31c0 xor eax, eax
0x1422c1df0 483b45d0 cmp rax, qword ptr [rbp - 0x30]
0x1422c1df4 0f8081010000 jo 0x1422c1f7b
0x1422c1dfa 488b4540 mov rax, qword ptr [rbp + 0x40]
0x1422c1dfe 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x1422c1e05 0f104530 movups xmm0, xmmword ptr [rbp + 0x30]
0x1422c1e09 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
0x1422c1e10 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
0x1422c1e14 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
0x1422c1e1b 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x1422c1e1f 0f298590000000 movaps xmmword ptr [rbp + 0x90], xmm0
0x1422c1e26 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x1422c1e2a 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x1422c1e2e 0f1055f0 movups xmm2, xmmword ptr [rbp - 0x10]
0x1422c1e32 0f105d00 movups xmm3, xmmword ptr [rbp]
0x1422c1e36 0f299d80000000 movaps xmmword ptr [rbp + 0x80], xmm3
0x1422c1e3d 0f295570 movaps xmmword ptr [rbp + 0x70], xmm2
0x1422c1e41 0f294d60 movaps xmmword ptr [rbp + 0x60], xmm1
0x1422c1e45 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1422c1e49 c685d700000001 mov byte ptr [rbp + 0xd7], 1
0x1422c1e50 488d4d50 lea rcx, [rbp + 0x50]
0x1422c1e54 e8876374fe call 0x140a081e0
0x1422c1e59 0f28f0 movaps xmm6, xmm0
0x1422c1e5c c685d700000001 mov byte ptr [rbp + 0xd7], 1
0x1422c1e63 488d4db0 lea rcx, [rbp - 0x50]
0x1422c1e67 488d5550 lea rdx, [rbp + 0x50]
0x1422c1e6b e8005c74fe call 0x140a07a70
0x1422c1e70 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
0x1422c1e77 48894540 mov qword ptr [rbp + 0x40], rax
0x1422c1e7b 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x1422c1e82 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x1422c1e86 0f2885a0000000 movaps xmm0, xmmword ptr [rbp + 0xa0]
0x1422c1e8d 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1422c1e91 0f288590000000 movaps xmm0, xmmword ptr [rbp + 0x90]
0x1422c1e98 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1422c1e9c 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1422c1ea0 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
0x1422c1ea4 0f285570 movaps xmm2, xmmword ptr [rbp + 0x70]
0x1422c1ea8 0f289d80000000 movaps xmm3, xmmword ptr [rbp + 0x80]
0x1422c1eaf 0f295d00 movaps xmmword ptr [rbp], xmm3
0x1422c1eb3 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x1422c1eb7 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x1422c1ebb 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1422c1ebf 4d8b0e mov r9, qword ptr [r14]
0x1422c1ec2 488b03 mov rax, qword ptr [rbx]
0x1422c1ec5 c685d700000000 mov byte ptr [rbp + 0xd7], 0
0x1422c1ecc 4889442420 mov qword ptr [rsp + 0x20], rax
0x1422c1ed1 488d4dd0 lea rcx, [rbp - 0x30]
0x1422c1ed5 4889fa mov rdx, rdi
0x1422c1ed8 4989f0 mov r8, rsi
0x1422c1edb e8c08d85fe call 0x140b1aca0
0x1422c1ee0 4889c3 mov rbx, rax
0x1422c1ee3 c685d700000000 mov byte ptr [rbp + 0xd7], 0
0x1422c1eea 4889f9 mov rcx, rdi
0x1422c1eed 4889f2 mov rdx, rsi
0x1422c1ef0 e88bae9dfe call 0x140c9cd80
0x1422c1ef5 4989c6 mov r14, rax
0x1422c1ef8 0fb6050a2d3701 movzx eax, byte ptr [rip + 0x1372d0a]
0x1422c1eff b948000000 mov ecx, 0x48
0x1422c1f04 ba08000000 mov edx, 8
0x1422c1f09 e8021d35fe call 0x140613c10
0x1422c1f0e 4885c0 test rax, rax
0x1422c1f11 0f8496000000 je 0x1422c1fad
0x1422c1f17 0f1045b0 movups xmm0, xmmword ptr [rbp - 0x50]
0x1422c1f1b 0f104dc0 movups xmm1, xmmword ptr [rbp - 0x40]
0x1422c1f1f 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1422c1f23 0f1100 movups xmmword ptr [rax], xmm0
0x1422c1f26 48897820 mov qword ptr [rax + 0x20], rdi
0x1422c1f2a 48897028 mov qword ptr [rax + 0x28], rsi
0x1422c1f2e 48895830 mov qword ptr [rax + 0x30], rbx
0x1422c1f32 4c897038 mov qword ptr [rax + 0x38], r14
0x1422c1f36 f30f117040 movss dword ptr [rax + 0x40], xmm6
0x1422c1f3b 44887844 mov byte ptr [rax + 0x44], r15b
0x1422c1f3f 44886845 mov byte ptr [rax + 0x45], r13b
0x1422c1f43 44886046 mov byte ptr [rax + 0x46], r12b
0x1422c1f47 0fb68d80010000 movzx ecx, byte ptr [rbp + 0x180]
0x1422c1f4e 884847 mov byte ptr [rax + 0x47], cl
0x1422c1f51 0f28b5e0000000 movaps xmm6, xmmword ptr [rbp + 0xe0]
0x1422c1f58 4881c478010000 add rsp, 0x178
0x1422c1f5f 5b pop rbx
0x1422c1f60 5f pop rdi
0x1422c1f61 5e pop rsi
0x1422c1f62 415c pop r12
0x1422c1f64 415d pop r13
0x1422c1f66 415e pop r14
0x1422c1f68 415f pop r15
0x1422c1f6a 5d pop rbp
0x1422c1f6b c3 ret 
0x1422c1f6c b908000000 mov ecx, 8
0x1422c1f71 ba48000000 mov edx, 0x48
0x1422c1f76 e885706100 call 0x1428d9000
0x1422c1f7b 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x1422c1f7f 48894550 mov qword ptr [rbp + 0x50], rax
0x1422c1f83 488d050ee8d700 lea rax, [rip + 0xd7e80e]
0x1422c1f8a 4889442420 mov qword ptr [rsp + 0x20], rax
0x1422c1f8f 488d0dbadbd700 lea rcx, [rip + 0xd7dbba]
0x1422c1f96 4c8d0d93dbd700 lea r9, [rip + 0xd7db93]
0x1422c1f9d 4c8d4550 lea r8, [rbp + 0x50]
0x1422c1fa1 ba2b000000 mov edx, 0x2b
0x1422c1fa6 e8b5776100 call 0x1428d9760
0x1422c1fab eb16 jmp 0x1422c1fc3
0x1422c1fad c685d700000000 mov byte ptr [rbp + 0xd7], 0
0x1422c1fb4 b908000000 mov ecx, 8
0x1422c1fb9 ba48000000 mov edx, 0x48
0x1422c1fbe e83d706100 call 0x1428d9000
0x1422c1fc3 0f0b ud2 
0x1422c1fc5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1422c1fd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1422c1fd5 55 push rbp
0x1422c1fd6 4157 push r15
0x1422c1fd8 4156 push r14
0x1422c1fda 4155 push r13
0x1422c1fdc 4154 push r12
0x1422c1fde 56 push rsi
0x1422c1fdf 57 push rdi
0x1422c1fe0 53 push rbx
0x1422c1fe1 4883ec48 sub rsp, 0x48
0x1422c1fe5 488daa80000000 lea rbp, [rdx + 0x80]
0x1422c1fec 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1422c1ff1 80bdd700000000 cmp byte ptr [rbp + 0xd7], 0
0x1422c1ff8 7516 jne 0x1422c2010
0x1422c1ffa 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1422c1fff 4883c448 add rsp, 0x48
0x1422c2003 5b pop rbx
0x1422c2004 5f pop rdi
0x1422c2005 5e pop rsi
0x1422c2006 415c pop r12
0x1422c2008 415d pop r13
0x1422c200a 415e pop r14
0x1422c200c 415f pop r15
0x1422c200e 5d pop rbp
0x1422c200f c3 ret 
0x1422c2010 488d4d50 lea rcx, [rbp + 0x50]
0x1422c2014 e8072bc1fe call 0x140ed4b20
0x1422c2019 ebdf jmp 0x1422c1ffa
0x1422c201b 0f1f440000 nop dword ptr [rax + rax]
0x1422c2020 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1422c2025 55 push rbp
0x1422c2026 4157 push r15
0x1422c2028 4156 push r14
0x1422c202a 4155 push r13
0x1422c202c 4154 push r12
0x1422c202e 56 push rsi
0x1422c202f 57 push rdi
0x1422c2030 53 push rbx
0x1422c2031 4883ec48 sub rsp, 0x48
0x1422c2035 488daa80000000 lea rbp, [rdx + 0x80]
0x1422c203c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1422c2041 488d4d50 lea rcx, [rbp + 0x50]
0x1422c2045 e8e6aba8fe call 0x140d4cc30
0x1422c204a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1422c204f 4883c448 add rsp, 0x48
0x1422c2053 5b pop rbx
0x1422c2054 5f pop rdi
0x1422c2055 5e pop rsi
0x1422c2056 415c pop r12
0x1422c2058 415d pop r13
0x1422c205a 415e pop r14
0x1422c205c 415f pop r15
0x1422c205e 5d pop rbp
0x1422c205f c3 ret 
