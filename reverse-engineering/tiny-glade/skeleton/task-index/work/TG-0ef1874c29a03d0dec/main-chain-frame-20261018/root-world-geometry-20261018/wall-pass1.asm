14098aaf0 55 push rbp
14098aaf1 4157 push r15
14098aaf3 4156 push r14
14098aaf5 4155 push r13
14098aaf7 4154 push r12
14098aaf9 56 push rsi
14098aafa 57 push rdi
14098aafb 53 push rbx
14098aafc 4881ecd8060000 sub rsp, 0x6d8
14098ab03 488dac2480000000 lea rbp, [rsp + 0x80]
14098ab0b 48c78550060000feffffff mov qword ptr [rbp + 0x650], 0xfffffffffffffffe
14098ab16 4889d6 mov rsi, rdx
14098ab19 4889cf mov rdi, rcx
14098ab1c 4c8d7238 lea r14, [rdx + 0x38]
14098ab20 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
14098ab2a 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
14098ab34 4c89f1 mov rcx, r14
14098ab37 e8b42b75ff call 0x1400dd6f0
14098ab3c 4885c0 test rax, rax
14098ab3f 0f84b1050000 je 0x14098b0f6
14098ab45 48399618020000 cmp qword ptr [rsi + 0x218], rdx
14098ab4c 0f86a4050000 jbe 0x14098b0f6
14098ab52 488b8610020000 mov rax, qword ptr [rsi + 0x210]
14098ab59 488b04d0 mov rax, qword ptr [rax + rdx*8]
14098ab5d 4885c0 test rax, rax
14098ab60 0f8490050000 je 0x14098b0f6
14098ab66 4c8ba6e0010000 mov r12, qword ptr [rsi + 0x1e0]
14098ab6d 48f7d0 not rax
14098ab70 488d0440 lea rax, [rax + rax*2]
14098ab74 48c1e005 shl rax, 5
14098ab78 49837c042000 cmp qword ptr [r12 + rax + 0x20], 0
14098ab7e 0f8472050000 je 0x14098b0f6
14098ab84 4901c4 add r12, rax
14098ab87 498b5c2410 mov rbx, qword ptr [r12 + 0x10]
14098ab8c 448baeb0050000 mov r13d, dword ptr [rsi + 0x5b0]
14098ab93 4c8dbd70040000 lea r15, [rbp + 0x470]
14098ab9a 4c89f9 mov rcx, r15
14098ab9d 4889da mov rdx, rbx
14098aba0 4989f8 mov r8, rdi
14098aba3 e818599400 call 0x1412d04c0
14098aba8 4889f9 mov rcx, rdi
14098abab 4889f2 mov rdx, rsi
14098abae 4d89f8 mov r8, r15
14098abb1 e8fa2f0500 call 0x1409ddbb0
14098abb6 45896c245c mov dword ptr [r12 + 0x5c], r13d
14098abbb 4889d9 mov rcx, rbx
14098abbe e8edc59300 call 0x1412c71b0
14098abc3 4885c0 test rax, rax
14098abc6 0f8452050000 je 0x14098b11e
14098abcc 4889c3 mov rbx, rax
14098abcf c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098abd6 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098abdd 48ba56f7feea51bd0d85 movabs rdx, 0x850dbd51eafef756
14098abe7 49b8354c85394b2a7885 movabs r8, 0x85782a4b39854c35
14098abf1 4c89f1 mov rcx, r14
14098abf4 e8f72a75ff call 0x1400dd6f0
14098abf9 4885c0 test rax, rax
14098abfc 0f8400050000 je 0x14098b102
14098ac02 48399618020000 cmp qword ptr [rsi + 0x218], rdx
14098ac09 0f86f3040000 jbe 0x14098b102
14098ac0f 488b8610020000 mov rax, qword ptr [rsi + 0x210]
14098ac16 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
14098ac1a 4885c9 test rcx, rcx
14098ac1d 0f84df040000 je 0x14098b102
14098ac23 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
14098ac2a 48f7d1 not rcx
14098ac2d 488d0c49 lea rcx, [rcx + rcx*2]
14098ac31 48c1e105 shl rcx, 5
14098ac35 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
14098ac3b 0f84c1040000 je 0x14098b102
14098ac41 4801c8 add rax, rcx
14098ac44 488b4010 mov rax, qword ptr [rax + 0x10]
14098ac48 80781000 cmp byte ptr [rax + 0x10], 0
14098ac4c 0f8497010000 je 0x14098ade9
14098ac52 488b00 mov rax, qword ptr [rax]
14098ac55 89c1 mov ecx, eax
14098ac57 483b4e10 cmp rcx, qword ptr [rsi + 0x10]
14098ac5b 0f8388010000 jae 0x14098ade9
14098ac61 488b5608 mov rdx, qword ptr [rsi + 8]
14098ac65 48c1e820 shr rax, 0x20
14098ac69 488d0c89 lea rcx, [rcx + rcx*4]
14098ac6d 39048a cmp dword ptr [rdx + rcx*4], eax
14098ac70 0f8573010000 jne 0x14098ade9
14098ac76 488d048a lea rax, [rdx + rcx*4]
14098ac7a 837804ff cmp dword ptr [rax + 4], -1
14098ac7e 0f8465010000 je 0x14098ade9
14098ac84 448b600c mov r12d, dword ptr [rax + 0xc]
14098ac88 448b7810 mov r15d, dword ptr [rax + 0x10]
14098ac8c c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098ac93 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098ac9a 48ba52fa262202bc6e3d movabs rdx, 0x3d6ebc022226fa52
14098aca4 49b89dc3237af05805b2 movabs r8, 0xb20558f07a23c39d
14098acae 4c89f1 mov rcx, r14
14098acb1 e8eac4ffff call 0x1409871a0
14098acb6 4885c0 test rax, rax
14098acb9 0f842a010000 je 0x14098ade9
14098acbf 4c39a6b0010000 cmp qword ptr [rsi + 0x1b0], r12
14098acc6 0f861d010000 jbe 0x14098ade9
14098accc 4b8d0ce4 lea rcx, [r12 + r12*8]
14098acd0 48c1e103 shl rcx, 3
14098acd4 48038ea8010000 add rcx, qword ptr [rsi + 0x1a8]
14098acdb c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098ace2 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098ace9 4589f8 mov r8d, r15d
14098acec e85f5e3700 call 0x140d00b50
14098acf1 4989c7 mov r15, rax
14098acf4 4885c0 test rax, rax
14098acf7 0f84ec000000 je 0x14098ade9
14098acfd 49837f2000 cmp qword ptr [r15 + 0x20], 0
14098ad02 0f8422040000 je 0x14098b12a
14098ad08 4983c720 add r15, 0x20
14098ad0c c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098ad13 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098ad1a 488d8d70040000 lea rcx, [rbp + 0x470]
14098ad21 4889da mov rdx, rbx
14098ad24 4d89f8 mov r8, r15
14098ad27 e8348d9500 call 0x1412e3a60
14098ad2c 4c8bb570040000 mov r14, qword ptr [rbp + 0x470]
14098ad33 4c8bad78040000 mov r13, qword ptr [rbp + 0x478]
14098ad3a 488b8580040000 mov rax, qword ptr [rbp + 0x480]
14098ad41 48898500060000 mov qword ptr [rbp + 0x600], rax
14098ad48 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098ad4f 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ad56 c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098ad5d c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098ad64 4889d9 mov rcx, rbx
14098ad67 4c89fa mov rdx, r15
14098ad6a e8918e9500 call 0x1412e3c00
14098ad6f 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098ad76 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ad7d c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098ad84 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098ad8b 488d8d70040000 lea rcx, [rbp + 0x470]
14098ad92 4889da mov rdx, rbx
14098ad95 e8b6949500 call 0x1412e4250
14098ad9a 488b8570040000 mov rax, qword ptr [rbp + 0x470]
14098ada1 4c89fa mov rdx, r15
14098ada4 4c8bbd78040000 mov r15, qword ptr [rbp + 0x478]
14098adab 4c8ba580040000 mov r12, qword ptr [rbp + 0x480]
14098adb2 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098adb9 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098adc0 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098adc7 48898528060000 mov qword ptr [rbp + 0x628], rax
14098adce c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098add5 c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098addc 4889d9 mov rcx, rbx
14098addf 4889c3 mov rbx, rax
14098ade2 e8998e9500 call 0x1412e3c80
14098ade7 eb46 jmp 0x14098ae2f
14098ade9 c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098adf0 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098adf7 488d8d70040000 lea rcx, [rbp + 0x470]
14098adfe 4889da mov rdx, rbx
14098ae01 e84a949500 call 0x1412e4250
14098ae06 488b9d70040000 mov rbx, qword ptr [rbp + 0x470]
14098ae0d 4c8bbd78040000 mov r15, qword ptr [rbp + 0x478]
14098ae14 4c8ba580040000 mov r12, qword ptr [rbp + 0x480]
14098ae1b 41bd04000000 mov r13d, 4
14098ae21 48c7850006000000000000 mov qword ptr [rbp + 0x600], 0
14098ae2c 4531f6 xor r14d, r14d
14098ae2f 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098ae36 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ae3d 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098ae44 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098ae4b c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098ae52 c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098ae59 48897c2430 mov qword ptr [rsp + 0x30], rdi
14098ae5e 4889742428 mov qword ptr [rsp + 0x28], rsi
14098ae63 48c74424202a000000 mov qword ptr [rsp + 0x20], 0x2a
14098ae6c 488d156de01202 lea rdx, [rip + 0x212e06d]
14098ae73 4c8d0d7ee01202 lea r9, [rip + 0x212e07e]
14098ae7a 488d8d70040000 lea rcx, [rbp + 0x470]
14098ae81 41b818000000 mov r8d, 0x18
14098ae87 e81484c700 call 0x1416032a0
14098ae8c 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098ae93 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098ae9a 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098aea1 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098aea8 c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098aeaf c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098aeb6 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098aebd 488d9570040000 lea rdx, [rbp + 0x470]
14098aec4 49b8499ac42ad83ef2a2 movabs r8, 0xa2f23ed82ac49a49
14098aece e86d20c800 call 0x14160cf40
14098aed3 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098aeda 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098aee1 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098aee8 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098aeef c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098aef6 c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098aefd 488d8d70040000 lea rcx, [rbp + 0x470]
14098af04 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098af0b 49b8edb8c743bc36ab35 movabs r8, 0x35ab36bc43c7b8ed
14098af15 e84632c800 call 0x14160e160
14098af1a 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098af21 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098af28 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098af2f 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098af36 c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098af3d c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098af44 488d8de0020000 lea rcx, [rbp + 0x2e0]
14098af4b 488d9570040000 lea rdx, [rbp + 0x470]
14098af52 49b8c5677f3b90a98337 movabs r8, 0x3783a9903b7f67c5
14098af5c e80fbfc700 call 0x141606e70
14098af61 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098af68 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098af6f 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098af76 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098af7d c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098af84 c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098af8b 488d8d50010000 lea rcx, [rbp + 0x150]
14098af92 488d95e0020000 lea rdx, [rbp + 0x2e0]
14098af99 49b8b2513d42f7c60c7d movabs r8, 0x7d0cc6f7423d51b2
14098afa3 e848cdc700 call 0x141607cf0
14098afa8 4c89ad40060000 mov qword ptr [rbp + 0x640], r13
14098afaf 4c89b538060000 mov qword ptr [rbp + 0x638], r14
14098afb6 4c89bd30060000 mov qword ptr [rbp + 0x630], r15
14098afbd 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098afc4 c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098afcb c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098afd2 488d8d70040000 lea rcx, [rbp + 0x470]
14098afd9 488d9550010000 lea rdx, [rbp + 0x150]
14098afe0 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
14098afea 41b901000000 mov r9d, 1
14098aff0 e81bdcc700 call 0x141608c10
14098aff5 4c89bd20060000 mov qword ptr [rbp + 0x620], r15
14098affc 48899d18060000 mov qword ptr [rbp + 0x618], rbx
14098b003 4c89b510060000 mov qword ptr [rbp + 0x610], r14
14098b00a 4c89ad08060000 mov qword ptr [rbp + 0x608], r13
14098b011 80bda605000008 cmp byte ptr [rbp + 0x5a6], 8
14098b018 7428 je 0x14098b042
14098b01a 488d8d80050000 lea rcx, [rbp + 0x580]
14098b021 bae8000000 mov edx, 0xe8
14098b026 480395a8050000 add rdx, qword ptr [rbp + 0x5a8]
14098b02d 49b88957b716a611950e movabs r8, 0xe9511a616b75789
14098b037 41b911000000 mov r9d, 0x11
14098b03d e82e05feff call 0x14096b570
14098b042 488d75c0 lea rsi, [rbp - 0x40]
14098b046 488d9570040000 lea rdx, [rbp + 0x470]
14098b04d 41b890010000 mov r8d, 0x190
14098b053 4889f1 mov rcx, rsi
14098b056 e8a58bf901 call 0x142923c00
14098b05b 488b9518060000 mov rdx, qword ptr [rbp + 0x618]
14098b062 48899570040000 mov qword ptr [rbp + 0x470], rdx
14098b069 4c8b8520060000 mov r8, qword ptr [rbp + 0x620]
14098b070 4c898578040000 mov qword ptr [rbp + 0x478], r8
14098b077 4c89a580040000 mov qword ptr [rbp + 0x480], r12
14098b07e 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
14098b085 48898d88040000 mov qword ptr [rbp + 0x488], rcx
14098b08c 488b8508060000 mov rax, qword ptr [rbp + 0x608]
14098b093 48898590040000 mov qword ptr [rbp + 0x490], rax
14098b09a 4c8b8d00060000 mov r9, qword ptr [rbp + 0x600]
14098b0a1 4c898d98040000 mov qword ptr [rbp + 0x498], r9
14098b0a8 48898540060000 mov qword ptr [rbp + 0x640], rax
14098b0af 48898d38060000 mov qword ptr [rbp + 0x638], rcx
14098b0b6 4c898530060000 mov qword ptr [rbp + 0x630], r8
14098b0bd 48899528060000 mov qword ptr [rbp + 0x628], rdx
14098b0c4 c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098b0cb c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098b0d2 488d9570040000 lea rdx, [rbp + 0x470]
14098b0d9 4889f1 mov rcx, rsi
14098b0dc e85ffcc700 call 0x14160ad40
14098b0e1 90 nop 
14098b0e2 4881c4d8060000 add rsp, 0x6d8
14098b0e9 5b pop rbx
14098b0ea 5f pop rdi
14098b0eb 5e pop rsi
14098b0ec 415c pop r12
14098b0ee 415d pop r13
14098b0f0 415e pop r14
14098b0f2 415f pop r15
14098b0f4 5d pop rbp
14098b0f5 c3 ret 
14098b0f6 488d0d83dd1202 lea rcx, [rip + 0x212dd83]
14098b0fd e80ee2f401 call 0x1428d9310
14098b102 c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098b109 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098b110 488d0d99dd1202 lea rcx, [rip + 0x212dd99]
14098b117 e8f4e1f401 call 0x1428d9310
14098b11c eb26 jmp 0x14098b144
14098b11e 488d0d73dd1202 lea rcx, [rip + 0x212dd73]
14098b125 e8e6e1f401 call 0x1428d9310
14098b12a c6854f06000001 mov byte ptr [rbp + 0x64f], 1
14098b131 c6854e06000001 mov byte ptr [rbp + 0x64e], 1
14098b138 488d0d89dd1202 lea rcx, [rip + 0x212dd89]
14098b13f e8cce1f401 call 0x1428d9310
14098b144 0f0b ud2 
14098b146 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14098b150 4889542410 mov qword ptr [rsp + 0x10], rdx
14098b155 55 push rbp
14098b156 4157 push r15
14098b158 4156 push r14
14098b15a 4155 push r13
14098b15c 4154 push r12
14098b15e 56 push rsi
14098b15f 57 push rdi
14098b160 53 push rbx
14098b161 4883ec38 sub rsp, 0x38
14098b165 488daa80000000 lea rbp, [rdx + 0x80]
14098b16c 0fb69d4e060000 movzx ebx, byte ptr [rbp + 0x64e]
14098b173 488bb528060000 mov rsi, qword ptr [rbp + 0x628]
14098b17a 488bbd30060000 mov rdi, qword ptr [rbp + 0x630]
14098b181 488b9538060000 mov rdx, qword ptr [rbp + 0x638]
14098b188 4885d2 test rdx, rdx
14098b18b 0f94c0 sete al
14098b18e 0a854f060000 or al, byte ptr [rbp + 0x64f]
14098b194 a801 test al, 1
14098b196 7516 jne 0x14098b1ae
14098b198 488b8d40060000 mov rcx, qword ptr [rbp + 0x640]
14098b19f 48c1e203 shl rdx, 3
14098b1a3 41b804000000 mov r8d, 4
14098b1a9 e8728ac8ff call 0x140613c20
14098b1ae 4885f6 test rsi, rsi
14098b1b1 0f94c0 sete al
14098b1b4 08c3 or bl, al
14098b1b6 f6c301 test bl, 1
14098b1b9 7515 jne 0x14098b1d0
14098b1bb 48c1e603 shl rsi, 3
14098b1bf 41b804000000 mov r8d, 4
14098b1c5 4889f9 mov rcx, rdi
14098b1c8 4889f2 mov rdx, rsi
14098b1cb e8508ac8ff call 0x140613c20
14098b1d0 90 nop 
14098b1d1 4883c438 add rsp, 0x38
14098b1d5 5b pop rbx
14098b1d6 5f pop rdi
14098b1d7 5e pop rsi
14098b1d8 415c pop r12
14098b1da 415d pop r13
14098b1dc 415e pop r14
14098b1de 415f pop r15
14098b1e0 5d pop rbp
14098b1e1 c3 ret 
14098b1e2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14098b1f0 4889542410 mov qword ptr [rsp + 0x10], rdx
14098b1f5 55 push rbp
14098b1f6 4157 push r15
14098b1f8 4156 push r14
14098b1fa 4155 push r13
14098b1fc 4154 push r12
14098b1fe 56 push rsi
14098b1ff 57 push rdi
14098b200 53 push rbx
14098b201 4883ec38 sub rsp, 0x38
14098b205 488daa80000000 lea rbp, [rdx + 0x80]
14098b20c 488bb508060000 mov rsi, qword ptr [rbp + 0x608]
14098b213 4889b540060000 mov qword ptr [rbp + 0x640], rsi
14098b21a 488bbd10060000 mov rdi, qword ptr [rbp + 0x610]
14098b221 4889bd38060000 mov qword ptr [rbp + 0x638], rdi
14098b228 4c8bb520060000 mov r14, qword ptr [rbp + 0x620]
14098b22f 4c89b530060000 mov qword ptr [rbp + 0x630], r14
14098b236 488b9d18060000 mov rbx, qword ptr [rbp + 0x618]
14098b23d 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098b244 c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098b24b c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098b252 488d8d70040000 lea rcx, [rbp + 0x470]
14098b259 e852bdf0ff call 0x140896fb0
14098b25e 4889b540060000 mov qword ptr [rbp + 0x640], rsi
14098b265 4889bd38060000 mov qword ptr [rbp + 0x638], rdi
14098b26c 4c89b530060000 mov qword ptr [rbp + 0x630], r14
14098b273 48899d28060000 mov qword ptr [rbp + 0x628], rbx
14098b27a c6854f06000000 mov byte ptr [rbp + 0x64f], 0
14098b281 c6854e06000000 mov byte ptr [rbp + 0x64e], 0
14098b288 4883c438 add rsp, 0x38
14098b28c 5b pop rbx
14098b28d 5f pop rdi
14098b28e 5e pop rsi
14098b28f 415c pop r12
14098b291 415d pop r13
14098b293 415e pop r14
14098b295 415f pop r15
14098b297 5d pop rbp
14098b298 c3 ret 
