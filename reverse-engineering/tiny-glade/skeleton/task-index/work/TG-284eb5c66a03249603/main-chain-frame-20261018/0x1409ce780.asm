0x1409ce780 55 push rbp
0x1409ce781 4157 push r15
0x1409ce783 4156 push r14
0x1409ce785 4155 push r13
0x1409ce787 4154 push r12
0x1409ce789 56 push rsi
0x1409ce78a 57 push rdi
0x1409ce78b 53 push rbx
0x1409ce78c 4881ec180e0000 sub rsp, 0xe18
0x1409ce793 488dac2480000000 lea rbp, [rsp + 0x80]
0x1409ce79b 0f29b5800d0000 movaps xmmword ptr [rbp + 0xd80], xmm6
0x1409ce7a2 48c785780d0000feffffff mov qword ptr [rbp + 0xd78], 0xfffffffffffffffe
0x1409ce7ad 4c89cf mov rdi, r9
0x1409ce7b0 4c89c3 mov rbx, r8
0x1409ce7b3 4989d6 mov r14, rdx
0x1409ce7b6 4989cf mov r15, rcx
0x1409ce7b9 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce7c0 e83b822400 call 0x140c16a00
0x1409ce7c5 8885600d0000 mov byte ptr [rbp + 0xd60], al
0x1409ce7cb 80bd600d000000 cmp byte ptr [rbp + 0xd60], 0
0x1409ce7d2 7479 je 0x1409ce84d
0x1409ce7d4 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce7db 4c8d25e92b0f02 lea r12, [rip + 0x20f2be9]
0x1409ce7e2 b839000000 mov eax, 0x39
0x1409ce7e7 488985680d0000 mov qword ptr [rbp + 0xd68], rax
0x1409ce7ee ba39000000 mov edx, 0x39
0x1409ce7f3 4c89e1 mov rcx, r12
0x1409ce7f6 e805f0ffff call 0x1409cd800
0x1409ce7fb 4883f801 cmp rax, 1
0x1409ce7ff 0f85d7000000 jne 0x1409ce8dc
0x1409ce805 4989d1 mov r9, rdx
0x1409ce808 4885d2 test rdx, rdx
0x1409ce80b 7449 je 0x1409ce856
0x1409ce80d 4983f939 cmp r9, 0x39
0x1409ce811 7341 jae 0x1409ce854
0x1409ce813 488d05b12b0f02 lea rax, [rip + 0x20f2bb1]
0x1409ce81a 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x1409ce81f 7f35 jg 0x1409ce856
0x1409ce821 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce828 488d05f12c0f02 lea rax, [rip + 0x20f2cf1]
0x1409ce82f 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409ce834 488d0d902b0f02 lea rcx, [rip + 0x20f2b90]
0x1409ce83b ba39000000 mov edx, 0x39
0x1409ce840 4531c0 xor r8d, r8d
0x1409ce843 e8f8b0f001 call 0x1428d9940
0x1409ce848 e93c0d0000 jmp 0x1409cf589
0x1409ce84d 31c9 xor ecx, ecx
0x1409ce84f e91c020000 jmp 0x1409cea70
0x1409ce854 75cb jne 0x1409ce821
0x1409ce856 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce85d 4c8d25672b0f02 lea r12, [rip + 0x20f2b67]
0x1409ce864 4c89e1 mov rcx, r12
0x1409ce867 4c89ca mov rdx, r9
0x1409ce86a e891efffff call 0x1409cd800
0x1409ce86f 4883f801 cmp rax, 1
0x1409ce873 7567 jne 0x1409ce8dc
0x1409ce875 4989d0 mov r8, rdx
0x1409ce878 4983c002 add r8, 2
0x1409ce87c 7445 je 0x1409ce8c3
0x1409ce87e 4983f839 cmp r8, 0x39
0x1409ce882 733d jae 0x1409ce8c1
0x1409ce884 488d05402b0f02 lea rax, [rip + 0x20f2b40]
0x1409ce88b 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x1409ce890 7f31 jg 0x1409ce8c3
0x1409ce892 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce899 488d05982c0f02 lea rax, [rip + 0x20f2c98]
0x1409ce8a0 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409ce8a5 488d0d1f2b0f02 lea rcx, [rip + 0x20f2b1f]
0x1409ce8ac ba39000000 mov edx, 0x39
0x1409ce8b1 41b939000000 mov r9d, 0x39
0x1409ce8b7 e884b0f001 call 0x1428d9940
0x1409ce8bc e9c80c0000 jmp 0x1409cf589
0x1409ce8c1 75cf jne 0x1409ce892
0x1409ce8c3 b837000000 mov eax, 0x37
0x1409ce8c8 4829d0 sub rax, rdx
0x1409ce8cb 488985680d0000 mov qword ptr [rbp + 0xd68], rax
0x1409ce8d2 4c8d25f22a0f02 lea r12, [rip + 0x20f2af2]
0x1409ce8d9 4d01c4 add r12, r8
0x1409ce8dc 488d05172d0f02 lea rax, [rip + 0x20f2d17]
0x1409ce8e3 4c8d2de12c0f02 lea r13, [rip + 0x20f2ce1]
0x1409ce8ea 660f1f440000 nop word ptr [rax + rax]
0x1409ce8f0 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x1409ce8f4 85c9 test ecx, ecx
0x1409ce8f6 7818 js 0x1409ce910
0x1409ce8f8 48ffc8 dec rax
0x1409ce8fb 83f95c cmp ecx, 0x5c
0x1409ce8fe 7569 jne 0x1409ce969
0x1409ce900 eb7c jmp 0x1409ce97e
0x1409ce902 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce910 0fb650fe movzx edx, byte ptr [rax - 2]
0x1409ce914 80fac0 cmp dl, 0xc0
0x1409ce917 7d25 jge 0x1409ce93e
0x1409ce919 440fb640fd movzx r8d, byte ptr [rax - 3]
0x1409ce91e 4180f8c0 cmp r8b, 0xc0
0x1409ce922 7d23 jge 0x1409ce947
0x1409ce924 440fb648fc movzx r9d, byte ptr [rax - 4]
0x1409ce929 4883c0fc add rax, -4
0x1409ce92d 4183e107 and r9d, 7
0x1409ce931 41c1e106 shl r9d, 6
0x1409ce935 4183e03f and r8d, 0x3f
0x1409ce939 4509c8 or r8d, r9d
0x1409ce93c eb11 jmp 0x1409ce94f
0x1409ce93e 4883c0fe add rax, -2
0x1409ce942 83e21f and edx, 0x1f
0x1409ce945 eb12 jmp 0x1409ce959
0x1409ce947 4883c0fd add rax, -3
0x1409ce94b 4183e00f and r8d, 0xf
0x1409ce94f 41c1e006 shl r8d, 6
0x1409ce953 83e23f and edx, 0x3f
0x1409ce956 4409c2 or edx, r8d
0x1409ce959 c1e206 shl edx, 6
0x1409ce95c 80e13f and cl, 0x3f
0x1409ce95f 0fb6c9 movzx ecx, cl
0x1409ce962 09d1 or ecx, edx
0x1409ce964 83f95c cmp ecx, 0x5c
0x1409ce967 7415 je 0x1409ce97e
0x1409ce969 83f92f cmp ecx, 0x2f
0x1409ce96c 7410 je 0x1409ce97e
0x1409ce96e 4c39e8 cmp rax, r13
0x1409ce971 0f8579ffffff jne 0x1409ce8f0
0x1409ce977 be2f000000 mov esi, 0x2f
0x1409ce97c eb5b jmp 0x1409ce9d9
0x1409ce97e 4c8d2d462c0f02 lea r13, [rip + 0x20f2c46]
0x1409ce985 4c29e8 sub rax, r13
0x1409ce988 4989c0 mov r8, rax
0x1409ce98b 49ffc0 inc r8
0x1409ce98e 743e je 0x1409ce9ce
0x1409ce990 4983f82f cmp r8, 0x2f
0x1409ce994 7336 jae 0x1409ce9cc
0x1409ce996 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x1409ce99b 7f31 jg 0x1409ce9ce
0x1409ce99d c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce9a4 488d05552b0f02 lea rax, [rip + 0x20f2b55]
0x1409ce9ab 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409ce9b0 488d0d142c0f02 lea rcx, [rip + 0x20f2c14]
0x1409ce9b7 ba2f000000 mov edx, 0x2f
0x1409ce9bc 41b92f000000 mov r9d, 0x2f
0x1409ce9c2 e879aff001 call 0x1428d9940
0x1409ce9c7 e9bd0b0000 jmp 0x1409cf589
0x1409ce9cc 75cf jne 0x1409ce99d
0x1409ce9ce be2e000000 mov esi, 0x2e
0x1409ce9d3 4829c6 sub rsi, rax
0x1409ce9d6 4d01c5 add r13, r8
0x1409ce9d9 e852972400 call 0x140c18130
0x1409ce9de 4889c1 mov rcx, rax
0x1409ce9e1 488b00 mov rax, qword ptr [rax]
0x1409ce9e4 4885c0 test rax, rax
0x1409ce9e7 740f je 0x1409ce9f8
0x1409ce9e9 83f801 cmp eax, 1
0x1409ce9ec 0f856f0b0000 jne 0x1409cf561
0x1409ce9f2 4883c108 add rcx, 8
0x1409ce9f6 eb1a jmp 0x1409cea12
0x1409ce9f8 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409ce9ff 31d2 xor edx, edx
0x1409cea01 e8ca9df201 call 0x1428f87d0
0x1409cea06 4889c1 mov rcx, rax
0x1409cea09 4885c0 test rax, rax
0x1409cea0c 0f844f0b0000 je 0x1409cf561
0x1409cea12 48833900 cmp qword ptr [rcx], 0
0x1409cea16 0f85060b0000 jne 0x1409cf522
0x1409cea1c 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1409cea23 48898d400d0000 mov qword ptr [rbp + 0xd40], rcx
0x1409cea2a 4883c108 add rcx, 8
0x1409cea2e 4889742420 mov qword ptr [rsp + 0x20], rsi
0x1409cea33 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1409cea3c 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1409cea45 4c89e2 mov rdx, r12
0x1409cea48 4c8b85680d0000 mov r8, qword ptr [rbp + 0xd68]
0x1409cea4f 4d89e9 mov r9, r13
0x1409cea52 e8e9822400 call 0x140c16d40
0x1409cea57 4889c6 mov rsi, rax
0x1409cea5a 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x1409cea61 48ff00 inc qword ptr [rax]
0x1409cea64 4889b5200c0000 mov qword ptr [rbp + 0xc20], rsi
0x1409cea6b b901000000 mov ecx, 1
0x1409cea70 488b85080e0000 mov rax, qword ptr [rbp + 0xe08]
0x1409cea77 48898d180c0000 mov qword ptr [rbp + 0xc18], rcx
0x1409cea7e 4c8b28 mov r13, qword ptr [rax]
0x1409cea81 4c8b6018 mov r12, qword ptr [rax + 0x18]
0x1409cea85 4c89e1 mov rcx, r12
0x1409cea88 4183fd01 cmp r13d, 1
0x1409cea8c 7404 je 0x1409cea92
0x1409cea8e 488b4808 mov rcx, qword ptr [rax + 8]
0x1409cea92 0f107008 movups xmm6, xmmword ptr [rax + 8]
0x1409cea96 4883c104 add rcx, 4
0x1409cea9a c685750d000001 mov byte ptr [rbp + 0xd75], 1
0x1409ceaa1 e8ea6c2000 call 0x140bd5790
0x1409ceaa6 66490f6ec5 movq xmm0, r13
0x1409ceaab 660fd68598070000 movq qword ptr [rbp + 0x798], xmm0
0x1409ceab3 0f11b5a0070000 movups xmmword ptr [rbp + 0x7a0], xmm6
0x1409ceaba 4c89a5b0070000 mov qword ptr [rbp + 0x7b0], r12
0x1409ceac1 c785280c000002000000 mov dword ptr [rbp + 0xc28], 2
0x1409ceacb c685750d000001 mov byte ptr [rbp + 0xd75], 1
0x1409cead2 4c897c2440 mov qword ptr [rsp + 0x40], r15
0x1409cead7 4c89742438 mov qword ptr [rsp + 0x38], r14
0x1409ceadc 4889442430 mov qword ptr [rsp + 0x30], rax
0x1409ceae1 488d85280c0000 lea rax, [rbp + 0xc28]
0x1409ceae8 4889442428 mov qword ptr [rsp + 0x28], rax
0x1409ceaed 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x1409ceaf6 488d4dd0 lea rcx, [rbp - 0x30]
0x1409ceafa 4c8d8d98070000 lea r9, [rbp + 0x798]
0x1409ceb01 4889da mov rdx, rbx
0x1409ceb04 4989f8 mov r8, rdi
0x1409ceb07 e8648ee9ff call 0x140867970
0x1409ceb0c 488d8d800b0000 lea rcx, [rbp + 0xb80]
0x1409ceb13 41b898000000 mov r8d, 0x98
0x1409ceb19 488b95100e0000 mov rdx, qword ptr [rbp + 0xe10]
0x1409ceb20 e8db50f501 call 0x142923c00
0x1409ceb25 c685760d000001 mov byte ptr [rbp + 0xd76], 1
0x1409ceb2c e81f7c2100 call 0x140be6750
0x1409ceb31 89c7 mov edi, eax
0x1409ceb33 c685760d000001 mov byte ptr [rbp + 0xd76], 1
0x1409ceb3a 31c9 xor ecx, ecx
0x1409ceb3c e8dfaf70ff call 0x1400d9b20
0x1409ceb41 4885c0 test rax, rax
0x1409ceb44 0f84020a0000 je 0x1409cf54c
0x1409ceb4a 488b8d000e0000 mov rcx, qword ptr [rbp + 0xe00]
0x1409ceb51 89bd40080000 mov dword ptr [rbp + 0x840], edi
0x1409ceb57 66c785480800000000 mov word ptr [rbp + 0x848], 0
0x1409ceb60 c785440800000000803f mov dword ptr [rbp + 0x844], 0x3f800000
0x1409ceb6a 48c7859807000000000000 mov qword ptr [rbp + 0x798], 0
0x1409ceb75 48898d38080000 mov qword ptr [rbp + 0x838], rcx
0x1409ceb7c 48c785a807000000000000 mov qword ptr [rbp + 0x7a8], 0
0x1409ceb87 48c785b007000004000000 mov qword ptr [rbp + 0x7b0], 4
0x1409ceb92 48c785b807000000000000 mov qword ptr [rbp + 0x7b8], 0
0x1409ceb9d f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x1409ceba1 48ff00 inc qword ptr [rax]
0x1409ceba4 0f100d857b0c02 movups xmm1, xmmword ptr [rip + 0x20c7b85]
0x1409cebab 0f118d18080000 movups xmmword ptr [rbp + 0x818], xmm1
0x1409cebb2 0f100d677b0c02 movups xmm1, xmmword ptr [rip + 0x20c7b67]
0x1409cebb9 0f118d08080000 movups xmmword ptr [rbp + 0x808], xmm1
0x1409cebc0 f30f7f8528080000 movdqu xmmword ptr [rbp + 0x828], xmm0
0x1409cebc8 48b80300000001000000 movabs rax, 0x100000003
0x1409cebd2 488985d8070000 mov qword ptr [rbp + 0x7d8], rax
0x1409cebd9 48c785c007000000000000 mov qword ptr [rbp + 0x7c0], 0
0x1409cebe4 48c785c807000001000000 mov qword ptr [rbp + 0x7c8], 1
0x1409cebef 48c785d007000000000000 mov qword ptr [rbp + 0x7d0], 0
0x1409cebfa c685760d000001 mov byte ptr [rbp + 0xd76], 1
0x1409cec01 488d8db8030000 lea rcx, [rbp + 0x3b8]
0x1409cec08 488dbd98070000 lea rdi, [rbp + 0x798]
0x1409cec0f 4c8d45d0 lea r8, [rbp - 0x30]
0x1409cec13 4889fa mov rdx, rdi
0x1409cec16 4531c9 xor r9d, r9d
0x1409cec19 e8c26ff3ff call 0x140905be0
0x1409cec1e 488d9db0040000 lea rbx, [rbp + 0x4b0]
0x1409cec25 488d95b8030000 lea rdx, [rbp + 0x3b8]
0x1409cec2c 41b8f8000000 mov r8d, 0xf8
0x1409cec32 4889d9 mov rcx, rbx
0x1409cec35 e8c64ff501 call 0x142923c00
0x1409cec3a 488d85b00b0000 lea rax, [rbp + 0xbb0]
0x1409cec41 488985280c0000 mov qword ptr [rbp + 0xc28], rax
0x1409cec48 c7859807000002000000 mov dword ptr [rbp + 0x798], 2
0x1409cec52 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1409cec57 488dbd280c0000 lea rdi, [rbp + 0xc28]
0x1409cec5e 49b8ddb340cc9756ba11 movabs r8, 0x11ba5697cc40b3dd
0x1409cec68 4889f9 mov rcx, rdi
0x1409cec6b 4889da mov rdx, rbx
0x1409cec6e 41b11d mov r9b, 0x1d
0x1409cec71 e88afbf1ff call 0x1408ee800
0x1409cec76 488d9d98070000 lea rbx, [rbp + 0x798]
0x1409cec7d 488d95b0040000 lea rdx, [rbp + 0x4b0]
0x1409cec84 41b8f8000000 mov r8d, 0xf8
0x1409cec8a 4889d9 mov rcx, rbx
0x1409cec8d e86e4ff501 call 0x142923c00
0x1409cec92 488d85d40b0000 lea rax, [rbp + 0xbd4]
0x1409cec99 488985a0060000 mov qword ptr [rbp + 0x6a0], rax
0x1409ceca0 c785280c000002000000 mov dword ptr [rbp + 0xc28], 2
0x1409cecaa 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1409cecaf 488d8da0060000 lea rcx, [rbp + 0x6a0]
0x1409cecb6 49b841e279e02c54c5ab movabs r8, 0xabc5542ce079e241
0x1409cecc0 4889da mov rdx, rbx
0x1409cecc3 41b11d mov r9b, 0x1d
0x1409cecc6 e835fbf1ff call 0x1408ee800
0x1409ceccb 41b8f8000000 mov r8d, 0xf8
0x1409cecd1 4889f9 mov rcx, rdi
0x1409cecd4 4889da mov rdx, rbx
0x1409cecd7 e8244ff501 call 0x142923c00
0x1409cecdc 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
0x1409cece6 488985a0060000 mov qword ptr [rbp + 0x6a0], rax
0x1409ceced c7859807000002000000 mov dword ptr [rbp + 0x798], 2
0x1409cecf7 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1409cecfc 488d8da0060000 lea rcx, [rbp + 0x6a0]
0x1409ced03 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
0x1409ced0d 4889fa mov rdx, rdi
0x1409ced10 41b11d mov r9b, 0x1d
0x1409ced13 e8f898f3ff call 0x140908610
0x1409ced18 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409ced1f 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409ced26 41b8f8000000 mov r8d, 0xf8
0x1409ced2c e8cf4ef501 call 0x142923c00
0x1409ced31 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409ced38 742a je 0x1409ced64
0x1409ced3a 440fb68d140c0000 movzx r9d, byte ptr [rbp + 0xc14]
0x1409ced42 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409ced49 bae8000000 mov edx, 0xe8
0x1409ced4e 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409ced55 49b82358d9f7ec04950d movabs r8, 0xd9504ecf7d95823
0x1409ced5f e80cc8f9ff call 0x14096b570
0x1409ced64 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409ced6b 488d9598070000 lea rdx, [rbp + 0x798]
0x1409ced72 41b8f8000000 mov r8d, 0xf8
0x1409ced78 e8834ef501 call 0x142923c00
0x1409ced7d 80bd060d000008 cmp byte ptr [rbp + 0xd06], 8
0x1409ced84 7429 je 0x1409cedaf
0x1409ced86 448b8df80b0000 mov r9d, dword ptr [rbp + 0xbf8]
0x1409ced8d 488d8de00c0000 lea rcx, [rbp + 0xce0]
0x1409ced94 bae8000000 mov edx, 0xe8
0x1409ced99 480395100d0000 add rdx, qword ptr [rbp + 0xd10]
0x1409ceda0 49b8d5226e1f3d305546 movabs r8, 0x4655303d1f6e22d5
0x1409cedaa e8c1c7f9ff call 0x14096b570
0x1409cedaf 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cedb6 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409cedbd 41b8f8000000 mov r8d, 0xf8
0x1409cedc3 e8384ef501 call 0x142923c00
0x1409cedc8 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409cedcf 742a je 0x1409cedfb
0x1409cedd1 f30f109dfc0b0000 movss xmm3, dword ptr [rbp + 0xbfc]
0x1409cedd9 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cede0 bae8000000 mov edx, 0xe8
0x1409cede5 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cedec 49b829f11cc9d8ab8a98 movabs r8, 0x988aabd8c91cf129
0x1409cedf6 e835ccf9ff call 0x14096ba30
0x1409cedfb 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cee02 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cee09 41b8f8000000 mov r8d, 0xf8
0x1409cee0f e8ec4df501 call 0x142923c00
0x1409cee14 80bd060d000008 cmp byte ptr [rbp + 0xd06], 8
0x1409cee1b 7429 je 0x1409cee46
0x1409cee1d 448b8d000c0000 mov r9d, dword ptr [rbp + 0xc00]
0x1409cee24 488d8de00c0000 lea rcx, [rbp + 0xce0]
0x1409cee2b bae8000000 mov edx, 0xe8
0x1409cee30 480395100d0000 add rdx, qword ptr [rbp + 0xd10]
0x1409cee37 49b8bb928d240aa75a22 movabs r8, 0x225aa70a248d92bb
0x1409cee41 e82ac7f9ff call 0x14096b570
0x1409cee46 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cee4d 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409cee54 41b8f8000000 mov r8d, 0xf8
0x1409cee5a e8a14df501 call 0x142923c00
0x1409cee5f 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409cee66 742a je 0x1409cee92
0x1409cee68 f30f109d040c0000 movss xmm3, dword ptr [rbp + 0xc04]
0x1409cee70 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cee77 bae8000000 mov edx, 0xe8
0x1409cee7c 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cee83 49b84a19b433b3b79014 movabs r8, 0x1490b7b333b4194a
0x1409cee8d e89ecbf9ff call 0x14096ba30
0x1409cee92 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cee99 488d9598070000 lea rdx, [rbp + 0x798]
0x1409ceea0 41b8f8000000 mov r8d, 0xf8
0x1409ceea6 e8554df501 call 0x142923c00
0x1409ceeab 80bd060d000008 cmp byte ptr [rbp + 0xd06], 8
0x1409ceeb2 742a je 0x1409ceede
0x1409ceeb4 f30f109d0c0c0000 movss xmm3, dword ptr [rbp + 0xc0c]
0x1409ceebc 488d8de00c0000 lea rcx, [rbp + 0xce0]
0x1409ceec3 bae8000000 mov edx, 0xe8
0x1409ceec8 480395100d0000 add rdx, qword ptr [rbp + 0xd10]
0x1409ceecf 49b8e5e6640228344893 movabs r8, 0x934834280264e6e5
0x1409ceed9 e852cbf9ff call 0x14096ba30
0x1409ceede 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409ceee5 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409ceeec 41b8f8000000 mov r8d, 0xf8
0x1409ceef2 e8094df501 call 0x142923c00
0x1409ceef7 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409ceefe 742a je 0x1409cef2a
0x1409cef00 f30f109d080c0000 movss xmm3, dword ptr [rbp + 0xc08]
0x1409cef08 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cef0f bae8000000 mov edx, 0xe8
0x1409cef14 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cef1b 49b810bfb2562ee9c978 movabs r8, 0x78c9e92e56b2bf10
0x1409cef25 e806cbf9ff call 0x14096ba30
0x1409cef2a 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cef31 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cef38 41b8f8000000 mov r8d, 0xf8
0x1409cef3e e8bd4cf501 call 0x142923c00
0x1409cef43 80bd060d000008 cmp byte ptr [rbp + 0xd06], 8
0x1409cef4a 742e je 0x1409cef7a
0x1409cef4c 488b85980b0000 mov rax, qword ptr [rbp + 0xb98]
0x1409cef53 f30f105830 movss xmm3, dword ptr [rax + 0x30]
0x1409cef58 488d8de00c0000 lea rcx, [rbp + 0xce0]
0x1409cef5f bae8000000 mov edx, 0xe8
0x1409cef64 480395100d0000 add rdx, qword ptr [rbp + 0xd10]
0x1409cef6b 49b8a8d39ddb3773738a movabs r8, 0x8a737337db9dd3a8
0x1409cef75 e8b6caf9ff call 0x14096ba30
0x1409cef7a 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cef81 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409cef88 41b8f8000000 mov r8d, 0xf8
0x1409cef8e e86d4cf501 call 0x142923c00
0x1409cef93 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409cef9a 742a je 0x1409cefc6
0x1409cef9c f30f109d100c0000 movss xmm3, dword ptr [rbp + 0xc10]
0x1409cefa4 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cefab bae8000000 mov edx, 0xe8
0x1409cefb0 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cefb7 49b8491d52670bd502a9 movabs r8, 0xa902d50b67521d49
0x1409cefc1 e86acaf9ff call 0x14096ba30
0x1409cefc6 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cefcd 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cefd4 41b8f8000000 mov r8d, 0xf8
0x1409cefda e8214cf501 call 0x142923c00
0x1409cefdf 0f57db xorps xmm3, xmm3
0x1409cefe2 80bd150c000001 cmp byte ptr [rbp + 0xc15], 1
0x1409cefe9 750c jne 0x1409ceff7
0x1409cefeb 488b85980b0000 mov rax, qword ptr [rbp + 0xb98]
0x1409ceff2 f30f105838 movss xmm3, dword ptr [rax + 0x38]
0x1409ceff7 80bd060d000008 cmp byte ptr [rbp + 0xd06], 8
0x1409ceffe 7422 je 0x1409cf022
0x1409cf000 488d8de00c0000 lea rcx, [rbp + 0xce0]
0x1409cf007 bae8000000 mov edx, 0xe8
0x1409cf00c 480395100d0000 add rdx, qword ptr [rbp + 0xd10]
0x1409cf013 49b8db2d25b8abd1e61d movabs r8, 0x1de6d1abb8252ddb
0x1409cf01d e80ecaf9ff call 0x14096ba30
0x1409cf022 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf029 488d95280c0000 lea rdx, [rbp + 0xc28]
0x1409cf030 41b8f8000000 mov r8d, 0xf8
0x1409cf036 e8c54bf501 call 0x142923c00
0x1409cf03b 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409cf042 742e je 0x1409cf072
0x1409cf044 488b85980b0000 mov rax, qword ptr [rbp + 0xb98]
0x1409cf04b f30f105834 movss xmm3, dword ptr [rax + 0x34]
0x1409cf050 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cf057 bae8000000 mov edx, 0xe8
0x1409cf05c 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cf063 49b88e7cae0269a32783 movabs r8, 0x8327a36902ae7c8e
0x1409cf06d e8bec9f9ff call 0x14096ba30
0x1409cf072 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf079 41b8f8000000 mov r8d, 0xf8
0x1409cf07f 4889f9 mov rcx, rdi
0x1409cf082 e8794bf501 call 0x142923c00
0x1409cf087 4c8b8da00b0000 mov r9, qword ptr [rbp + 0xba0]
0x1409cf08e 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf095 49b87149ea8e31b91b53 movabs r8, 0x531bb9318eea4971
0x1409cf09f 4889fa mov rdx, rdi
0x1409cf0a2 e879acc500 call 0x141629d20
0x1409cf0a7 31c0 xor eax, eax
0x1409cf0a9 483b85800b0000 cmp rax, qword ptr [rbp + 0xb80]
0x1409cf0b0 4c8d3d51280f02 lea r15, [rip + 0x20f2851]
0x1409cf0b7 488b85880b0000 mov rax, qword ptr [rbp + 0xb88]
0x1409cf0be 490f40c7 cmovo rax, r15
0x1409cf0c2 b907000000 mov ecx, 7
0x1409cf0c7 480f418d900b0000 cmovno rcx, qword ptr [rbp + 0xb90]
0x1409cf0cf 488985480d0000 mov qword ptr [rbp + 0xd48], rax
0x1409cf0d6 48898d500d0000 mov qword ptr [rbp + 0xd50], rcx
0x1409cf0dd 488d85480d0000 lea rax, [rbp + 0xd48]
0x1409cf0e4 488985a8050000 mov qword ptr [rbp + 0x5a8], rax
0x1409cf0eb 4c8d259ead73ff lea r12, [rip - 0x8c5262]
0x1409cf0f2 4c89a5b0050000 mov qword ptr [rbp + 0x5b0], r12
0x1409cf0f9 4c8d2da87b1102 lea r13, [rip + 0x2117ba8]
0x1409cf100 4c89ad280c0000 mov qword ptr [rbp + 0xc28], r13
0x1409cf107 48c785300c000001000000 mov qword ptr [rbp + 0xc30], 1
0x1409cf112 488d85a8050000 lea rax, [rbp + 0x5a8]
0x1409cf119 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x1409cf120 48c785400c000001000000 mov qword ptr [rbp + 0xc40], 1
0x1409cf12b 48c785480c000000000000 mov qword ptr [rbp + 0xc48], 0
0x1409cf136 488dbda0060000 lea rdi, [rbp + 0x6a0]
0x1409cf13d 488d9d280c0000 lea rbx, [rbp + 0xc28]
0x1409cf144 4889f9 mov rcx, rdi
0x1409cf147 4889da mov rdx, rbx
0x1409cf14a e841ae3a00 call 0x140d79f90
0x1409cf14f 488b8da8060000 mov rcx, qword ptr [rbp + 0x6a8]
0x1409cf156 488b95b0060000 mov rdx, qword ptr [rbp + 0x6b0]
0x1409cf15d c685740d000001 mov byte ptr [rbp + 0xd74], 1
0x1409cf164 48898d680d0000 mov qword ptr [rbp + 0xd68], rcx
0x1409cf16b e810dc2c00 call 0x140c9cd80
0x1409cf170 488985a8050000 mov qword ptr [rbp + 0x5a8], rax
0x1409cf177 c785280c000002000000 mov dword ptr [rbp + 0xc28], 2
0x1409cf181 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1409cf186 488d8da8050000 lea rcx, [rbp + 0x5a8]
0x1409cf18d 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf194 49b80348be274cb12204 movabs r8, 0x422b14c27be4803
0x1409cf19e 41b11d mov r9b, 0x1d
0x1409cf1a1 e86a94f3ff call 0x140908610
0x1409cf1a6 4c8db598070000 lea r14, [rbp + 0x798]
0x1409cf1ad 41b8f8000000 mov r8d, 0xf8
0x1409cf1b3 4889d9 mov rcx, rbx
0x1409cf1b6 4c89f2 mov rdx, r14
0x1409cf1b9 e8424af501 call 0x142923c00
0x1409cf1be 48b8514282c6c4dfc2bb movabs rax, 0xbbc2dfc4c6824251
0x1409cf1c8 488985a8050000 mov qword ptr [rbp + 0x5a8], rax
0x1409cf1cf c7859807000002000000 mov dword ptr [rbp + 0x798], 2
0x1409cf1d9 4c89742420 mov qword ptr [rsp + 0x20], r14
0x1409cf1de 488d8da8050000 lea rcx, [rbp + 0x5a8]
0x1409cf1e5 49b810be6e0cf5da6504 movabs r8, 0x465daf50c6ebe10
0x1409cf1ef 4889da mov rdx, rbx
0x1409cf1f2 41b11d mov r9b, 0x1d
0x1409cf1f5 e81694f3ff call 0x140908610
0x1409cf1fa 41b8f8000000 mov r8d, 0xf8
0x1409cf200 4c89f1 mov rcx, r14
0x1409cf203 4889da mov rdx, rbx
0x1409cf206 e8f549f501 call 0x142923c00
0x1409cf20b 488d8da8050000 lea rcx, [rbp + 0x5a8]
0x1409cf212 49b86a176778e61bbff7 movabs r8, 0xf7bf1be67867176a
0x1409cf21c 4c89f2 mov rdx, r14
0x1409cf21f e86c06c600 call 0x14162f890
0x1409cf224 488b95a0060000 mov rdx, qword ptr [rbp + 0x6a0]
0x1409cf22b 4885d2 test rdx, rdx
0x1409cf22e 7412 je 0x1409cf242
0x1409cf230 41b801000000 mov r8d, 1
0x1409cf236 488b8d680d0000 mov rcx, qword ptr [rbp + 0xd68]
0x1409cf23d e8de49c4ff call 0x140613c20
0x1409cf242 4c8bb5a80b0000 mov r14, qword ptr [rbp + 0xba8]
0x1409cf249 4d85f6 test r14, r14
0x1409cf24c 746c je 0x1409cf2ba
0x1409cf24e 488dbd98070000 lea rdi, [rbp + 0x798]
0x1409cf255 488d95a8050000 lea rdx, [rbp + 0x5a8]
0x1409cf25c 41b8f8000000 mov r8d, 0xf8
0x1409cf262 4889f9 mov rcx, rdi
0x1409cf265 e89649f501 call 0x142923c00
0x1409cf26a 4c89b5480d0000 mov qword ptr [rbp + 0xd48], r14
0x1409cf271 c785280c000002000000 mov dword ptr [rbp + 0xc28], 2
0x1409cf27b 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1409cf280 488d8d480d0000 lea rcx, [rbp + 0xd48]
0x1409cf287 49b898e95cdc5633b815 movabs r8, 0x15b83356dc5ce998
0x1409cf291 4889fa mov rdx, rdi
0x1409cf294 41b11d mov r9b, 0x1d
0x1409cf297 e824e4f2ff call 0x1408fd6c0
0x1409cf29c 488d8da0060000 lea rcx, [rbp + 0x6a0]
0x1409cf2a3 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf2aa 41b8f8000000 mov r8d, 0xf8
0x1409cf2b0 e84b49f501 call 0x142923c00
0x1409cf2b5 e921010000 jmp 0x1409cf3db
0x1409cf2ba c685160c000000 mov byte ptr [rbp + 0xc16], 0
0x1409cf2c1 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf2c8 488d95a8050000 lea rdx, [rbp + 0x5a8]
0x1409cf2cf 41b8f8000000 mov r8d, 0xf8
0x1409cf2d5 e82649f501 call 0x142923c00
0x1409cf2da 4c89bd300d0000 mov qword ptr [rbp + 0xd30], r15
0x1409cf2e1 48c785380d000007000000 mov qword ptr [rbp + 0xd38], 7
0x1409cf2ec 488d85300d0000 lea rax, [rbp + 0xd30]
0x1409cf2f3 488985a0060000 mov qword ptr [rbp + 0x6a0], rax
0x1409cf2fa 4c89a5a8060000 mov qword ptr [rbp + 0x6a8], r12
0x1409cf301 4c89ad280c0000 mov qword ptr [rbp + 0xc28], r13
0x1409cf308 48c785300c000001000000 mov qword ptr [rbp + 0xc30], 1
0x1409cf313 4889bd380c0000 mov qword ptr [rbp + 0xc38], rdi
0x1409cf31a 48c785400c000001000000 mov qword ptr [rbp + 0xc40], 1
0x1409cf325 48c785480c000000000000 mov qword ptr [rbp + 0xc48], 0
0x1409cf330 c685710d000001 mov byte ptr [rbp + 0xd71], 1
0x1409cf337 488d8d480d0000 lea rcx, [rbp + 0xd48]
0x1409cf33e 488dbd280c0000 lea rdi, [rbp + 0xc28]
0x1409cf345 4889fa mov rdx, rdi
0x1409cf348 e843ac3a00 call 0x140d79f90
0x1409cf34d 488b8d500d0000 mov rcx, qword ptr [rbp + 0xd50]
0x1409cf354 488b95580d0000 mov rdx, qword ptr [rbp + 0xd58]
0x1409cf35b c685730d000001 mov byte ptr [rbp + 0xd73], 1
0x1409cf362 48898d680d0000 mov qword ptr [rbp + 0xd68], rcx
0x1409cf369 e812da2c00 call 0x140c9cd80
0x1409cf36e 488985300d0000 mov qword ptr [rbp + 0xd30], rax
0x1409cf375 c785280c000002000000 mov dword ptr [rbp + 0xc28], 2
0x1409cf37f 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1409cf384 488d8d300d0000 lea rcx, [rbp + 0xd30]
0x1409cf38b 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf392 49b898e95cdc5633b815 movabs r8, 0x15b83356dc5ce998
0x1409cf39c 41b11d mov r9b, 0x1d
0x1409cf39f e86c92f3ff call 0x140908610
0x1409cf3a4 488d8da0060000 lea rcx, [rbp + 0x6a0]
0x1409cf3ab 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf3b2 41b8f8000000 mov r8d, 0xf8
0x1409cf3b8 e84348f501 call 0x142923c00
0x1409cf3bd 488b95480d0000 mov rdx, qword ptr [rbp + 0xd48]
0x1409cf3c4 4885d2 test rdx, rdx
0x1409cf3c7 7412 je 0x1409cf3db
0x1409cf3c9 41b801000000 mov r8d, 1
0x1409cf3cf 488b8d680d0000 mov rcx, qword ptr [rbp + 0xd68]
0x1409cf3d6 e84548c4ff call 0x140613c20
0x1409cf3db 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf3e2 488d95a0060000 lea rdx, [rbp + 0x6a0]
0x1409cf3e9 41b8f8000000 mov r8d, 0xf8
0x1409cf3ef e80c48f501 call 0x142923c00
0x1409cf3f4 80bd7608000008 cmp byte ptr [rbp + 0x876], 8
0x1409cf3fb 742a je 0x1409cf427
0x1409cf3fd 440fb68d160c0000 movzx r9d, byte ptr [rbp + 0xc16]
0x1409cf405 488d8d50080000 lea rcx, [rbp + 0x850]
0x1409cf40c bae8000000 mov edx, 0xe8
0x1409cf411 48039580080000 add rdx, qword ptr [rbp + 0x880]
0x1409cf418 49b816a528a6d98fc31e movabs r8, 0x1ec38fd9a628a516
0x1409cf422 e849c1f9ff call 0x14096b570
0x1409cf427 488dbd280c0000 lea rdi, [rbp + 0xc28]
0x1409cf42e 488d9598070000 lea rdx, [rbp + 0x798]
0x1409cf435 41b8f8000000 mov r8d, 0xf8
0x1409cf43b 4889f9 mov rcx, rdi
0x1409cf43e e8bd47f501 call 0x142923c00
0x1409cf443 ba03000000 mov edx, 3
0x1409cf448 4889f9 mov rcx, rdi
0x1409cf44b e84076f3ff call 0x140906a90
0x1409cf450 488b95800b0000 mov rdx, qword ptr [rbp + 0xb80]
0x1409cf457 4889d0 mov rax, rdx
0x1409cf45a 48f7d8 neg rax
0x1409cf45d 7014 jo 0x1409cf473
0x1409cf45f 7312 jae 0x1409cf473
0x1409cf461 488b8d880b0000 mov rcx, qword ptr [rbp + 0xb88]
0x1409cf468 41b801000000 mov r8d, 1
0x1409cf46e e8ad47c4ff call 0x140613c20
0x1409cf473 488dbd98070000 lea rdi, [rbp + 0x798]
0x1409cf47a 488d55d0 lea rdx, [rbp - 0x30]
0x1409cf47e 41b8e8030000 mov r8d, 0x3e8
0x1409cf484 4889f9 mov rcx, rdi
0x1409cf487 e87447f501 call 0x142923c00
0x1409cf48c c685720d000000 mov byte ptr [rbp + 0xd72], 0
0x1409cf493 4889f9 mov rcx, rdi
0x1409cf496 e8d58ee9ff call 0x140868370
0x1409cf49b 80bd600d000000 cmp byte ptr [rbp + 0xd60], 0
0x1409cf4a2 7463 je 0x1409cf507
0x1409cf4a4 e8878c2400 call 0x140c18130
0x1409cf4a9 4889c1 mov rcx, rax
0x1409cf4ac 488b00 mov rax, qword ptr [rax]
0x1409cf4af 4885c0 test rax, rax
0x1409cf4b2 740f je 0x1409cf4c3
0x1409cf4b4 83f801 cmp eax, 1
0x1409cf4b7 0f85b9000000 jne 0x1409cf576
0x1409cf4bd 4883c108 add rcx, 8
0x1409cf4c1 eb1a jmp 0x1409cf4dd
0x1409cf4c3 c685770d000000 mov byte ptr [rbp + 0xd77], 0
0x1409cf4ca 31d2 xor edx, edx
0x1409cf4cc e8ff92f201 call 0x1428f87d0
0x1409cf4d1 4889c1 mov rcx, rax
0x1409cf4d4 4885c0 test rax, rax
0x1409cf4d7 0f8499000000 je 0x1409cf576
0x1409cf4dd 48833900 cmp qword ptr [rcx], 0
0x1409cf4e1 7554 jne 0x1409cf537
0x1409cf4e3 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1409cf4ea 48898d600d0000 mov qword ptr [rbp + 0xd60], rcx
0x1409cf4f1 4883c108 add rcx, 8
0x1409cf4f5 4889f2 mov rdx, rsi
0x1409cf4f8 e8f37b2400 call 0x140c170f0
0x1409cf4fd 488b85600d0000 mov rax, qword ptr [rbp + 0xd60]
0x1409cf504 48ff00 inc qword ptr [rax]
0x1409cf507 0f28b5800d0000 movaps xmm6, xmmword ptr [rbp + 0xd80]
0x1409cf50e 4881c4180e0000 add rsp, 0xe18
0x1409cf515 5b pop rbx
0x1409cf516 5f pop rdi
0x1409cf517 5e pop rsi
0x1409cf518 415c pop r12
0x1409cf51a 415d pop r13
0x1409cf51c 415e pop r14
0x1409cf51e 415f pop r15
0x1409cf520 5d pop rbp
0x1409cf521 c3 ret 
0x1409cf522 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409cf529 488d0d40b62602 lea rcx, [rip + 0x226b640]
0x1409cf530 e81b9df001 call 0x1428d9250
0x1409cf535 eb52 jmp 0x1409cf589
0x1409cf537 c685770d000000 mov byte ptr [rbp + 0xd77], 0
0x1409cf53e 488d0d2bb62602 lea rcx, [rip + 0x226b62b]
0x1409cf545 e8069df001 call 0x1428d9250
0x1409cf54a eb3d jmp 0x1409cf589
0x1409cf54c c685760d000001 mov byte ptr [rbp + 0xd76], 1
0x1409cf553 488d0d46b52602 lea rcx, [rip + 0x226b546]
0x1409cf55a e8718bf001 call 0x1428d80d0
0x1409cf55f eb28 jmp 0x1409cf589
0x1409cf561 c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409cf568 488d0d31b52602 lea rcx, [rip + 0x226b531]
0x1409cf56f e85c8bf001 call 0x1428d80d0
0x1409cf574 eb13 jmp 0x1409cf589
0x1409cf576 c685770d000000 mov byte ptr [rbp + 0xd77], 0
0x1409cf57d 488d0d1cb52602 lea rcx, [rip + 0x226b51c]
0x1409cf584 e8478bf001 call 0x1428d80d0
0x1409cf589 0f0b ud2 
0x1409cf58b 0f1f440000 nop dword ptr [rax + rax]
0x1409cf590 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf595 55 push rbp
0x1409cf596 4157 push r15
0x1409cf598 4156 push r14
0x1409cf59a 4155 push r13
0x1409cf59c 4154 push r12
0x1409cf59e 56 push rsi
0x1409cf59f 57 push rdi
0x1409cf5a0 53 push rbx
0x1409cf5a1 4883ec58 sub rsp, 0x58
0x1409cf5a5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf5ac 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf5b1 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x1409cf5b8 48ff00 inc qword ptr [rax]
0x1409cf5bb c685770d000001 mov byte ptr [rbp + 0xd77], 1
0x1409cf5c2 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf5c7 4883c458 add rsp, 0x58
0x1409cf5cb 5b pop rbx
0x1409cf5cc 5f pop rdi
0x1409cf5cd 5e pop rsi
0x1409cf5ce 415c pop r12
0x1409cf5d0 415d pop r13
0x1409cf5d2 415e pop r14
0x1409cf5d4 415f pop r15
0x1409cf5d6 5d pop rbp
0x1409cf5d7 c3 ret 
0x1409cf5d8 0f1f840000000000 nop dword ptr [rax + rax]
0x1409cf5e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf5e5 55 push rbp
0x1409cf5e6 4157 push r15
0x1409cf5e8 4156 push r14
0x1409cf5ea 4155 push r13
0x1409cf5ec 4154 push r12
0x1409cf5ee 56 push rsi
0x1409cf5ef 57 push rdi
0x1409cf5f0 53 push rbx
0x1409cf5f1 4883ec58 sub rsp, 0x58
0x1409cf5f5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf5fc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf601 0fb69d750d0000 movzx ebx, byte ptr [rbp + 0xd75]
0x1409cf608 80bd600d000000 cmp byte ptr [rbp + 0xd60], 0
0x1409cf60f 742f je 0x1409cf640
0x1409cf611 488d85200c0000 lea rax, [rbp + 0xc20]
0x1409cf618 488985280d0000 mov qword ptr [rbp + 0xd28], rax
0x1409cf61f 488d85280d0000 lea rax, [rbp + 0xd28]
0x1409cf626 488985200d0000 mov qword ptr [rbp + 0xd20], rax
0x1409cf62d 488d0d6c1e0f02 lea rcx, [rip + 0x20f1e6c]
0x1409cf634 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1409cf63b e890fc9500 call 0x14132f2d0
0x1409cf640 80e301 and bl, 1
0x1409cf643 889d770d0000 mov byte ptr [rbp + 0xd77], bl
0x1409cf649 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf64e 4883c458 add rsp, 0x58
0x1409cf652 5b pop rbx
0x1409cf653 5f pop rdi
0x1409cf654 5e pop rsi
0x1409cf655 415c pop r12
0x1409cf657 415d pop r13
0x1409cf659 415e pop r14
0x1409cf65b 415f pop r15
0x1409cf65d 5d pop rbp
0x1409cf65e c3 ret 
0x1409cf65f 90 nop 
0x1409cf660 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf665 55 push rbp
0x1409cf666 4157 push r15
0x1409cf668 4156 push r14
0x1409cf66a 4155 push r13
0x1409cf66c 4154 push r12
0x1409cf66e 56 push rsi
0x1409cf66f 57 push rdi
0x1409cf670 53 push rbx
0x1409cf671 4883ec58 sub rsp, 0x58
0x1409cf675 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf67c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf681 488d8db0040000 lea rcx, [rbp + 0x4b0]
0x1409cf688 e8333fecff call 0x1408935c0
0x1409cf68d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf692 4883c458 add rsp, 0x58
0x1409cf696 5b pop rbx
0x1409cf697 5f pop rdi
0x1409cf698 5e pop rsi
0x1409cf699 415c pop r12
0x1409cf69b 415d pop r13
0x1409cf69d 415e pop r14
0x1409cf69f 415f pop r15
0x1409cf6a1 5d pop rbp
0x1409cf6a2 c3 ret 
0x1409cf6a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf6b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf6b5 55 push rbp
0x1409cf6b6 4157 push r15
0x1409cf6b8 4156 push r14
0x1409cf6ba 4155 push r13
0x1409cf6bc 4154 push r12
0x1409cf6be 56 push rsi
0x1409cf6bf 57 push rdi
0x1409cf6c0 53 push rbx
0x1409cf6c1 4883ec58 sub rsp, 0x58
0x1409cf6c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf6cc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf6d1 488b95800b0000 mov rdx, qword ptr [rbp + 0xb80]
0x1409cf6d8 488d045500000000 lea rax, [rdx*2]
0x1409cf6e0 4885c0 test rax, rax
0x1409cf6e3 7412 je 0x1409cf6f7
0x1409cf6e5 488b8d880b0000 mov rcx, qword ptr [rbp + 0xb88]
0x1409cf6ec 41b801000000 mov r8d, 1
0x1409cf6f2 e82945c4ff call 0x140613c20
0x1409cf6f7 c685760d000000 mov byte ptr [rbp + 0xd76], 0
0x1409cf6fe 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf703 4883c458 add rsp, 0x58
0x1409cf707 5b pop rbx
0x1409cf708 5f pop rdi
0x1409cf709 5e pop rsi
0x1409cf70a 415c pop r12
0x1409cf70c 415d pop r13
0x1409cf70e 415e pop r14
0x1409cf710 415f pop r15
0x1409cf712 5d pop rbp
0x1409cf713 c3 ret 
0x1409cf714 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf720 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf725 55 push rbp
0x1409cf726 4157 push r15
0x1409cf728 4156 push r14
0x1409cf72a 4155 push r13
0x1409cf72c 4154 push r12
0x1409cf72e 56 push rsi
0x1409cf72f 57 push rdi
0x1409cf730 53 push rbx
0x1409cf731 4883ec58 sub rsp, 0x58
0x1409cf735 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf73c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf741 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf748 e8733eecff call 0x1408935c0
0x1409cf74d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf752 4883c458 add rsp, 0x58
0x1409cf756 5b pop rbx
0x1409cf757 5f pop rdi
0x1409cf758 5e pop rsi
0x1409cf759 415c pop r12
0x1409cf75b 415d pop r13
0x1409cf75d 415e pop r14
0x1409cf75f 415f pop r15
0x1409cf761 5d pop rbp
0x1409cf762 c3 ret 
0x1409cf763 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf770 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf775 55 push rbp
0x1409cf776 4157 push r15
0x1409cf778 4156 push r14
0x1409cf77a 4155 push r13
0x1409cf77c 4154 push r12
0x1409cf77e 56 push rsi
0x1409cf77f 57 push rdi
0x1409cf780 53 push rbx
0x1409cf781 4883ec58 sub rsp, 0x58
0x1409cf785 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf78c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf791 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cf798 e8233eecff call 0x1408935c0
0x1409cf79d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf7a2 4883c458 add rsp, 0x58
0x1409cf7a6 5b pop rbx
0x1409cf7a7 5f pop rdi
0x1409cf7a8 5e pop rsi
0x1409cf7a9 415c pop r12
0x1409cf7ab 415d pop r13
0x1409cf7ad 415e pop r14
0x1409cf7af 415f pop r15
0x1409cf7b1 5d pop rbp
0x1409cf7b2 c3 ret 
0x1409cf7b3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf7c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf7c5 55 push rbp
0x1409cf7c6 4157 push r15
0x1409cf7c8 4156 push r14
0x1409cf7ca 4155 push r13
0x1409cf7cc 4154 push r12
0x1409cf7ce 56 push rsi
0x1409cf7cf 57 push rdi
0x1409cf7d0 53 push rbx
0x1409cf7d1 4883ec58 sub rsp, 0x58
0x1409cf7d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf7dc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf7e1 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf7e8 e8d33decff call 0x1408935c0
0x1409cf7ed 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf7f2 4883c458 add rsp, 0x58
0x1409cf7f6 5b pop rbx
0x1409cf7f7 5f pop rdi
0x1409cf7f8 5e pop rsi
0x1409cf7f9 415c pop r12
0x1409cf7fb 415d pop r13
0x1409cf7fd 415e pop r14
0x1409cf7ff 415f pop r15
0x1409cf801 5d pop rbp
0x1409cf802 c3 ret 
0x1409cf803 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf810 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf815 55 push rbp
0x1409cf816 4157 push r15
0x1409cf818 4156 push r14
0x1409cf81a 4155 push r13
0x1409cf81c 4154 push r12
0x1409cf81e 56 push rsi
0x1409cf81f 57 push rdi
0x1409cf820 53 push rbx
0x1409cf821 4883ec58 sub rsp, 0x58
0x1409cf825 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf82c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf831 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cf838 e8833decff call 0x1408935c0
0x1409cf83d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf842 4883c458 add rsp, 0x58
0x1409cf846 5b pop rbx
0x1409cf847 5f pop rdi
0x1409cf848 5e pop rsi
0x1409cf849 415c pop r12
0x1409cf84b 415d pop r13
0x1409cf84d 415e pop r14
0x1409cf84f 415f pop r15
0x1409cf851 5d pop rbp
0x1409cf852 c3 ret 
0x1409cf853 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf860 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf865 55 push rbp
0x1409cf866 4157 push r15
0x1409cf868 4156 push r14
0x1409cf86a 4155 push r13
0x1409cf86c 4154 push r12
0x1409cf86e 56 push rsi
0x1409cf86f 57 push rdi
0x1409cf870 53 push rbx
0x1409cf871 4883ec58 sub rsp, 0x58
0x1409cf875 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf87c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf881 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf888 e8333decff call 0x1408935c0
0x1409cf88d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf892 4883c458 add rsp, 0x58
0x1409cf896 5b pop rbx
0x1409cf897 5f pop rdi
0x1409cf898 5e pop rsi
0x1409cf899 415c pop r12
0x1409cf89b 415d pop r13
0x1409cf89d 415e pop r14
0x1409cf89f 415f pop r15
0x1409cf8a1 5d pop rbp
0x1409cf8a2 c3 ret 
0x1409cf8a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf8b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf8b5 55 push rbp
0x1409cf8b6 4157 push r15
0x1409cf8b8 4156 push r14
0x1409cf8ba 4155 push r13
0x1409cf8bc 4154 push r12
0x1409cf8be 56 push rsi
0x1409cf8bf 57 push rdi
0x1409cf8c0 53 push rbx
0x1409cf8c1 4883ec58 sub rsp, 0x58
0x1409cf8c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf8cc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf8d1 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cf8d8 e8e33cecff call 0x1408935c0
0x1409cf8dd 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf8e2 4883c458 add rsp, 0x58
0x1409cf8e6 5b pop rbx
0x1409cf8e7 5f pop rdi
0x1409cf8e8 5e pop rsi
0x1409cf8e9 415c pop r12
0x1409cf8eb 415d pop r13
0x1409cf8ed 415e pop r14
0x1409cf8ef 415f pop r15
0x1409cf8f1 5d pop rbp
0x1409cf8f2 c3 ret 
0x1409cf8f3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf900 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf905 55 push rbp
0x1409cf906 4157 push r15
0x1409cf908 4156 push r14
0x1409cf90a 4155 push r13
0x1409cf90c 4154 push r12
0x1409cf90e 56 push rsi
0x1409cf90f 57 push rdi
0x1409cf910 53 push rbx
0x1409cf911 4883ec58 sub rsp, 0x58
0x1409cf915 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf91c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf921 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf928 e8933cecff call 0x1408935c0
0x1409cf92d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf932 4883c458 add rsp, 0x58
0x1409cf936 5b pop rbx
0x1409cf937 5f pop rdi
0x1409cf938 5e pop rsi
0x1409cf939 415c pop r12
0x1409cf93b 415d pop r13
0x1409cf93d 415e pop r14
0x1409cf93f 415f pop r15
0x1409cf941 5d pop rbp
0x1409cf942 c3 ret 
0x1409cf943 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf950 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf955 55 push rbp
0x1409cf956 4157 push r15
0x1409cf958 4156 push r14
0x1409cf95a 4155 push r13
0x1409cf95c 4154 push r12
0x1409cf95e 56 push rsi
0x1409cf95f 57 push rdi
0x1409cf960 53 push rbx
0x1409cf961 4883ec58 sub rsp, 0x58
0x1409cf965 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf96c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf971 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cf978 e8433cecff call 0x1408935c0
0x1409cf97d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf982 4883c458 add rsp, 0x58
0x1409cf986 5b pop rbx
0x1409cf987 5f pop rdi
0x1409cf988 5e pop rsi
0x1409cf989 415c pop r12
0x1409cf98b 415d pop r13
0x1409cf98d 415e pop r14
0x1409cf98f 415f pop r15
0x1409cf991 5d pop rbp
0x1409cf992 c3 ret 
0x1409cf993 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf9a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf9a5 55 push rbp
0x1409cf9a6 4157 push r15
0x1409cf9a8 4156 push r14
0x1409cf9aa 4155 push r13
0x1409cf9ac 4154 push r12
0x1409cf9ae 56 push rsi
0x1409cf9af 57 push rdi
0x1409cf9b0 53 push rbx
0x1409cf9b1 4883ec58 sub rsp, 0x58
0x1409cf9b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cf9bc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cf9c1 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cf9c8 e8f33becff call 0x1408935c0
0x1409cf9cd 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cf9d2 4883c458 add rsp, 0x58
0x1409cf9d6 5b pop rbx
0x1409cf9d7 5f pop rdi
0x1409cf9d8 5e pop rsi
0x1409cf9d9 415c pop r12
0x1409cf9db 415d pop r13
0x1409cf9dd 415e pop r14
0x1409cf9df 415f pop r15
0x1409cf9e1 5d pop rbp
0x1409cf9e2 c3 ret 
0x1409cf9e3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cf9f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cf9f5 55 push rbp
0x1409cf9f6 4157 push r15
0x1409cf9f8 4156 push r14
0x1409cf9fa 4155 push r13
0x1409cf9fc 4154 push r12
0x1409cf9fe 56 push rsi
0x1409cf9ff 57 push rdi
0x1409cfa00 53 push rbx
0x1409cfa01 4883ec58 sub rsp, 0x58
0x1409cfa05 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfa0c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfa11 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cfa18 e8a33becff call 0x1408935c0
0x1409cfa1d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfa22 4883c458 add rsp, 0x58
0x1409cfa26 5b pop rbx
0x1409cfa27 5f pop rdi
0x1409cfa28 5e pop rsi
0x1409cfa29 415c pop r12
0x1409cfa2b 415d pop r13
0x1409cfa2d 415e pop r14
0x1409cfa2f 415f pop r15
0x1409cfa31 5d pop rbp
0x1409cfa32 c3 ret 
0x1409cfa33 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfa40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfa45 55 push rbp
0x1409cfa46 4157 push r15
0x1409cfa48 4156 push r14
0x1409cfa4a 4155 push r13
0x1409cfa4c 4154 push r12
0x1409cfa4e 56 push rsi
0x1409cfa4f 57 push rdi
0x1409cfa50 53 push rbx
0x1409cfa51 4883ec58 sub rsp, 0x58
0x1409cfa55 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfa5c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfa61 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfa68 e8533becff call 0x1408935c0
0x1409cfa6d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfa72 4883c458 add rsp, 0x58
0x1409cfa76 5b pop rbx
0x1409cfa77 5f pop rdi
0x1409cfa78 5e pop rsi
0x1409cfa79 415c pop r12
0x1409cfa7b 415d pop r13
0x1409cfa7d 415e pop r14
0x1409cfa7f 415f pop r15
0x1409cfa81 5d pop rbp
0x1409cfa82 c3 ret 
0x1409cfa83 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfa90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfa95 55 push rbp
0x1409cfa96 4157 push r15
0x1409cfa98 4156 push r14
0x1409cfa9a 4155 push r13
0x1409cfa9c 4154 push r12
0x1409cfa9e 56 push rsi
0x1409cfa9f 57 push rdi
0x1409cfaa0 53 push rbx
0x1409cfaa1 4883ec58 sub rsp, 0x58
0x1409cfaa5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfaac 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfab1 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cfab8 e8033becff call 0x1408935c0
0x1409cfabd 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfac2 4883c458 add rsp, 0x58
0x1409cfac6 5b pop rbx
0x1409cfac7 5f pop rdi
0x1409cfac8 5e pop rsi
0x1409cfac9 415c pop r12
0x1409cfacb 415d pop r13
0x1409cfacd 415e pop r14
0x1409cfacf 415f pop r15
0x1409cfad1 5d pop rbp
0x1409cfad2 c3 ret 
0x1409cfad3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfae0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfae5 55 push rbp
0x1409cfae6 4157 push r15
0x1409cfae8 4156 push r14
0x1409cfaea 4155 push r13
0x1409cfaec 4154 push r12
0x1409cfaee 56 push rsi
0x1409cfaef 57 push rdi
0x1409cfaf0 53 push rbx
0x1409cfaf1 4883ec58 sub rsp, 0x58
0x1409cfaf5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfafc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfb01 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfb08 e8b33aecff call 0x1408935c0
0x1409cfb0d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfb12 4883c458 add rsp, 0x58
0x1409cfb16 5b pop rbx
0x1409cfb17 5f pop rdi
0x1409cfb18 5e pop rsi
0x1409cfb19 415c pop r12
0x1409cfb1b 415d pop r13
0x1409cfb1d 415e pop r14
0x1409cfb1f 415f pop r15
0x1409cfb21 5d pop rbp
0x1409cfb22 c3 ret 
0x1409cfb23 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfb30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfb35 55 push rbp
0x1409cfb36 4157 push r15
0x1409cfb38 4156 push r14
0x1409cfb3a 4155 push r13
0x1409cfb3c 4154 push r12
0x1409cfb3e 56 push rsi
0x1409cfb3f 57 push rdi
0x1409cfb40 53 push rbx
0x1409cfb41 4883ec58 sub rsp, 0x58
0x1409cfb45 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfb4c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfb51 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfb58 e8633aecff call 0x1408935c0
0x1409cfb5d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfb62 4883c458 add rsp, 0x58
0x1409cfb66 5b pop rbx
0x1409cfb67 5f pop rdi
0x1409cfb68 5e pop rsi
0x1409cfb69 415c pop r12
0x1409cfb6b 415d pop r13
0x1409cfb6d 415e pop r14
0x1409cfb6f 415f pop r15
0x1409cfb71 5d pop rbp
0x1409cfb72 c3 ret 
0x1409cfb73 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfb80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfb85 55 push rbp
0x1409cfb86 4157 push r15
0x1409cfb88 4156 push r14
0x1409cfb8a 4155 push r13
0x1409cfb8c 4154 push r12
0x1409cfb8e 56 push rsi
0x1409cfb8f 57 push rdi
0x1409cfb90 53 push rbx
0x1409cfb91 4883ec58 sub rsp, 0x58
0x1409cfb95 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfb9c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfba1 c685740d000000 mov byte ptr [rbp + 0xd74], 0
0x1409cfba8 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfbaf e80c3aecff call 0x1408935c0
0x1409cfbb4 c685740d000000 mov byte ptr [rbp + 0xd74], 0
0x1409cfbbb 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfbc0 4883c458 add rsp, 0x58
0x1409cfbc4 5b pop rbx
0x1409cfbc5 5f pop rdi
0x1409cfbc6 5e pop rsi
0x1409cfbc7 415c pop r12
0x1409cfbc9 415d pop r13
0x1409cfbcb 415e pop r14
0x1409cfbcd 415f pop r15
0x1409cfbcf 5d pop rbp
0x1409cfbd0 c3 ret 
0x1409cfbd1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfbe0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfbe5 55 push rbp
0x1409cfbe6 4157 push r15
0x1409cfbe8 4156 push r14
0x1409cfbea 4155 push r13
0x1409cfbec 4154 push r12
0x1409cfbee 56 push rsi
0x1409cfbef 57 push rdi
0x1409cfbf0 53 push rbx
0x1409cfbf1 4883ec58 sub rsp, 0x58
0x1409cfbf5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfbfc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfc01 488d8d280c0000 lea rcx, [rbp + 0xc28]
0x1409cfc08 e8b339ecff call 0x1408935c0
0x1409cfc0d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfc12 4883c458 add rsp, 0x58
0x1409cfc16 5b pop rbx
0x1409cfc17 5f pop rdi
0x1409cfc18 5e pop rsi
0x1409cfc19 415c pop r12
0x1409cfc1b 415d pop r13
0x1409cfc1d 415e pop r14
0x1409cfc1f 415f pop r15
0x1409cfc21 5d pop rbp
0x1409cfc22 c3 ret 
0x1409cfc23 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfc30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfc35 55 push rbp
0x1409cfc36 4157 push r15
0x1409cfc38 4156 push r14
0x1409cfc3a 4155 push r13
0x1409cfc3c 4154 push r12
0x1409cfc3e 56 push rsi
0x1409cfc3f 57 push rdi
0x1409cfc40 53 push rbx
0x1409cfc41 4883ec58 sub rsp, 0x58
0x1409cfc45 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfc4c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfc51 488b95a0060000 mov rdx, qword ptr [rbp + 0x6a0]
0x1409cfc58 4885d2 test rdx, rdx
0x1409cfc5b 7412 je 0x1409cfc6f
0x1409cfc5d 41b801000000 mov r8d, 1
0x1409cfc63 488b8d680d0000 mov rcx, qword ptr [rbp + 0xd68]
0x1409cfc6a e8b13fc4ff call 0x140613c20
0x1409cfc6f 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfc74 4883c458 add rsp, 0x58
0x1409cfc78 5b pop rbx
0x1409cfc79 5f pop rdi
0x1409cfc7a 5e pop rsi
0x1409cfc7b 415c pop r12
0x1409cfc7d 415d pop r13
0x1409cfc7f 415e pop r14
0x1409cfc81 415f pop r15
0x1409cfc83 5d pop rbp
0x1409cfc84 c3 ret 
0x1409cfc85 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfc90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfc95 55 push rbp
0x1409cfc96 4157 push r15
0x1409cfc98 4156 push r14
0x1409cfc9a 4155 push r13
0x1409cfc9c 4154 push r12
0x1409cfc9e 56 push rsi
0x1409cfc9f 57 push rdi
0x1409cfca0 53 push rbx
0x1409cfca1 4883ec58 sub rsp, 0x58
0x1409cfca5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfcac 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfcb1 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfcb8 e80339ecff call 0x1408935c0
0x1409cfcbd 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfcc2 4883c458 add rsp, 0x58
0x1409cfcc6 5b pop rbx
0x1409cfcc7 5f pop rdi
0x1409cfcc8 5e pop rsi
0x1409cfcc9 415c pop r12
0x1409cfccb 415d pop r13
0x1409cfccd 415e pop r14
0x1409cfccf 415f pop r15
0x1409cfcd1 5d pop rbp
0x1409cfcd2 c3 ret 
0x1409cfcd3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfce0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfce5 55 push rbp
0x1409cfce6 4157 push r15
0x1409cfce8 4156 push r14
0x1409cfcea 4155 push r13
0x1409cfcec 4154 push r12
0x1409cfcee 56 push rsi
0x1409cfcef 57 push rdi
0x1409cfcf0 53 push rbx
0x1409cfcf1 4883ec58 sub rsp, 0x58
0x1409cfcf5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfcfc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfd01 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfd08 e8b338ecff call 0x1408935c0
0x1409cfd0d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfd12 4883c458 add rsp, 0x58
0x1409cfd16 5b pop rbx
0x1409cfd17 5f pop rdi
0x1409cfd18 5e pop rsi
0x1409cfd19 415c pop r12
0x1409cfd1b 415d pop r13
0x1409cfd1d 415e pop r14
0x1409cfd1f 415f pop r15
0x1409cfd21 5d pop rbp
0x1409cfd22 c3 ret 
0x1409cfd23 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfd30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfd35 55 push rbp
0x1409cfd36 4157 push r15
0x1409cfd38 4156 push r14
0x1409cfd3a 4155 push r13
0x1409cfd3c 4154 push r12
0x1409cfd3e 56 push rsi
0x1409cfd3f 57 push rdi
0x1409cfd40 53 push rbx
0x1409cfd41 4883ec58 sub rsp, 0x58
0x1409cfd45 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfd4c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfd51 0fb69d730d0000 movzx ebx, byte ptr [rbp + 0xd73]
0x1409cfd58 488b95480d0000 mov rdx, qword ptr [rbp + 0xd48]
0x1409cfd5f 4885d2 test rdx, rdx
0x1409cfd62 7412 je 0x1409cfd76
0x1409cfd64 41b801000000 mov r8d, 1
0x1409cfd6a 488b8d680d0000 mov rcx, qword ptr [rbp + 0xd68]
0x1409cfd71 e8aa3ec4ff call 0x140613c20
0x1409cfd76 80e301 and bl, 1
0x1409cfd79 889d710d0000 mov byte ptr [rbp + 0xd71], bl
0x1409cfd7f 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfd84 4883c458 add rsp, 0x58
0x1409cfd88 5b pop rbx
0x1409cfd89 5f pop rdi
0x1409cfd8a 5e pop rsi
0x1409cfd8b 415c pop r12
0x1409cfd8d 415d pop r13
0x1409cfd8f 415e pop r14
0x1409cfd91 415f pop r15
0x1409cfd93 5d pop rbp
0x1409cfd94 c3 ret 
0x1409cfd95 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfda0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfda5 55 push rbp
0x1409cfda6 4157 push r15
0x1409cfda8 4156 push r14
0x1409cfdaa 4155 push r13
0x1409cfdac 4154 push r12
0x1409cfdae 56 push rsi
0x1409cfdaf 57 push rdi
0x1409cfdb0 53 push rbx
0x1409cfdb1 4883ec58 sub rsp, 0x58
0x1409cfdb5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfdbc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfdc1 80bd710d000000 cmp byte ptr [rbp + 0xd71], 0
0x1409cfdc8 7422 je 0x1409cfdec
0x1409cfdca 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfdd1 e8ea37ecff call 0x1408935c0
0x1409cfdd6 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfddb 4883c458 add rsp, 0x58
0x1409cfddf 5b pop rbx
0x1409cfde0 5f pop rdi
0x1409cfde1 5e pop rsi
0x1409cfde2 415c pop r12
0x1409cfde4 415d pop r13
0x1409cfde6 415e pop r14
0x1409cfde8 415f pop r15
0x1409cfdea 5d pop rbp
0x1409cfdeb c3 ret 
0x1409cfdec 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfdf1 4883c458 add rsp, 0x58
0x1409cfdf5 5b pop rbx
0x1409cfdf6 5f pop rdi
0x1409cfdf7 5e pop rsi
0x1409cfdf8 415c pop r12
0x1409cfdfa 415d pop r13
0x1409cfdfc 415e pop r14
0x1409cfdfe 415f pop r15
0x1409cfe00 5d pop rbp
0x1409cfe01 c3 ret 
0x1409cfe02 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfe10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfe15 55 push rbp
0x1409cfe16 4157 push r15
0x1409cfe18 4156 push r14
0x1409cfe1a 4155 push r13
0x1409cfe1c 4154 push r12
0x1409cfe1e 56 push rsi
0x1409cfe1f 57 push rdi
0x1409cfe20 53 push rbx
0x1409cfe21 4883ec58 sub rsp, 0x58
0x1409cfe25 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfe2c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfe31 80bd740d000000 cmp byte ptr [rbp + 0xd74], 0
0x1409cfe38 7422 je 0x1409cfe5c
0x1409cfe3a 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409cfe41 e87a37ecff call 0x1408935c0
0x1409cfe46 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfe4b 4883c458 add rsp, 0x58
0x1409cfe4f 5b pop rbx
0x1409cfe50 5f pop rdi
0x1409cfe51 5e pop rsi
0x1409cfe52 415c pop r12
0x1409cfe54 415d pop r13
0x1409cfe56 415e pop r14
0x1409cfe58 415f pop r15
0x1409cfe5a 5d pop rbp
0x1409cfe5b c3 ret 
0x1409cfe5c 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfe61 4883c458 add rsp, 0x58
0x1409cfe65 5b pop rbx
0x1409cfe66 5f pop rdi
0x1409cfe67 5e pop rsi
0x1409cfe68 415c pop r12
0x1409cfe6a 415d pop r13
0x1409cfe6c 415e pop r14
0x1409cfe6e 415f pop r15
0x1409cfe70 5d pop rbp
0x1409cfe71 c3 ret 
0x1409cfe72 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409cfe80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfe85 55 push rbp
0x1409cfe86 4157 push r15
0x1409cfe88 4156 push r14
0x1409cfe8a 4155 push r13
0x1409cfe8c 4154 push r12
0x1409cfe8e 56 push rsi
0x1409cfe8f 57 push rdi
0x1409cfe90 53 push rbx
0x1409cfe91 4883ec58 sub rsp, 0x58
0x1409cfe95 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfe9c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfea1 488b85600d0000 mov rax, qword ptr [rbp + 0xd60]
0x1409cfea8 48ff00 inc qword ptr [rax]
0x1409cfeab c685770d000000 mov byte ptr [rbp + 0xd77], 0
0x1409cfeb2 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cfeb7 4883c458 add rsp, 0x58
0x1409cfebb 5b pop rbx
0x1409cfebc 5f pop rdi
0x1409cfebd 5e pop rsi
0x1409cfebe 415c pop r12
0x1409cfec0 415d pop r13
0x1409cfec2 415e pop r14
0x1409cfec4 415f pop r15
0x1409cfec6 5d pop rbp
0x1409cfec7 c3 ret 
0x1409cfec8 0f1f840000000000 nop dword ptr [rax + rax]
0x1409cfed0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cfed5 55 push rbp
0x1409cfed6 4157 push r15
0x1409cfed8 4156 push r14
0x1409cfeda 4155 push r13
0x1409cfedc 4154 push r12
0x1409cfede 56 push rsi
0x1409cfedf 57 push rdi
0x1409cfee0 53 push rbx
0x1409cfee1 4883ec58 sub rsp, 0x58
0x1409cfee5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cfeec 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfef1 80bd760d000000 cmp byte ptr [rbp + 0xd76], 0
0x1409cfef8 7443 je 0x1409cff3d
0x1409cfefa 488b95800b0000 mov rdx, qword ptr [rbp + 0xb80]
0x1409cff01 488d045500000000 lea rax, [rdx*2]
0x1409cff09 4885c0 test rax, rax
0x1409cff0c 7412 je 0x1409cff20
0x1409cff0e 488b8d880b0000 mov rcx, qword ptr [rbp + 0xb88]
0x1409cff15 41b801000000 mov r8d, 1
0x1409cff1b e8003dc4ff call 0x140613c20
0x1409cff20 c685720d000001 mov byte ptr [rbp + 0xd72], 1
0x1409cff27 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cff2c 4883c458 add rsp, 0x58
0x1409cff30 5b pop rbx
0x1409cff31 5f pop rdi
0x1409cff32 5e pop rsi
0x1409cff33 415c pop r12
0x1409cff35 415d pop r13
0x1409cff37 415e pop r14
0x1409cff39 415f pop r15
0x1409cff3b 5d pop rbp
0x1409cff3c c3 ret 
0x1409cff3d c685720d000001 mov byte ptr [rbp + 0xd72], 1
0x1409cff44 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cff49 4883c458 add rsp, 0x58
0x1409cff4d 5b pop rbx
0x1409cff4e 5f pop rdi
0x1409cff4f 5e pop rsi
0x1409cff50 415c pop r12
0x1409cff52 415d pop r13
0x1409cff54 415e pop r14
0x1409cff56 415f pop r15
0x1409cff58 5d pop rbp
0x1409cff59 c3 ret 
0x1409cff5a 660f1f440000 nop word ptr [rax + rax]
0x1409cff60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cff65 55 push rbp
0x1409cff66 4157 push r15
0x1409cff68 4156 push r14
0x1409cff6a 4155 push r13
0x1409cff6c 4154 push r12
0x1409cff6e 56 push rsi
0x1409cff6f 57 push rdi
0x1409cff70 53 push rbx
0x1409cff71 4883ec58 sub rsp, 0x58
0x1409cff75 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cff7c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cff81 80bd720d000000 cmp byte ptr [rbp + 0xd72], 0
0x1409cff88 7426 je 0x1409cffb0
0x1409cff8a 488d4dd0 lea rcx, [rbp - 0x30]
0x1409cff8e e82d24ecff call 0x1408923c0
0x1409cff93 c685750d000000 mov byte ptr [rbp + 0xd75], 0
0x1409cff9a 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cff9f 4883c458 add rsp, 0x58
0x1409cffa3 5b pop rbx
0x1409cffa4 5f pop rdi
0x1409cffa5 5e pop rsi
0x1409cffa6 415c pop r12
0x1409cffa8 415d pop r13
0x1409cffaa 415e pop r14
0x1409cffac 415f pop r15
0x1409cffae 5d pop rbp
0x1409cffaf c3 ret 
0x1409cffb0 c685750d000000 mov byte ptr [rbp + 0xd75], 0
0x1409cffb7 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409cffbc 4883c458 add rsp, 0x58
0x1409cffc0 5b pop rbx
0x1409cffc1 5f pop rdi
0x1409cffc2 5e pop rsi
0x1409cffc3 415c pop r12
0x1409cffc5 415d pop r13
0x1409cffc7 415e pop r14
0x1409cffc9 415f pop r15
0x1409cffcb 5d pop rbp
0x1409cffcc c3 ret 
0x1409cffcd 0f1f00 nop dword ptr [rax]
0x1409cffd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409cffd5 55 push rbp
0x1409cffd6 4157 push r15
0x1409cffd8 4156 push r14
0x1409cffda 4155 push r13
0x1409cffdc 4154 push r12
0x1409cffde 56 push rsi
0x1409cffdf 57 push rdi
0x1409cffe0 53 push rbx
0x1409cffe1 4883ec58 sub rsp, 0x58
0x1409cffe5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409cffec 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409cfff1 488b8d100e0000 mov rcx, qword ptr [rbp + 0xe10]
0x1409cfff8 488b11 mov rdx, qword ptr [rcx]
0x1409cfffb 488d045500000000 lea rax, [rdx*2]
0x1409d0003 4885c0 test rax, rax
0x1409d0006 0f95c0 setne al
0x1409d0009 8485770d0000 test byte ptr [rbp + 0xd77], al
0x1409d000f 740f je 0x1409d0020
0x1409d0011 488b4908 mov rcx, qword ptr [rcx + 8]
0x1409d0015 41b801000000 mov r8d, 1
0x1409d001b e8003cc4ff call 0x140613c20
0x1409d0020 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409d0025 4883c458 add rsp, 0x58
0x1409d0029 5b pop rbx
0x1409d002a 5f pop rdi
0x1409d002b 5e pop rsi
0x1409d002c 415c pop r12
0x1409d002e 415d pop r13
0x1409d0030 415e pop r14
0x1409d0032 415f pop r15
0x1409d0034 5d pop rbp
0x1409d0035 c3 ret 
0x1409d0036 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409d0040 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409d0045 55 push rbp
0x1409d0046 4157 push r15
0x1409d0048 4156 push r14
0x1409d004a 4155 push r13
0x1409d004c 4154 push r12
0x1409d004e 56 push rsi
0x1409d004f 57 push rdi
0x1409d0050 53 push rbx
0x1409d0051 4883ec58 sub rsp, 0x58
0x1409d0055 488daa80000000 lea rbp, [rdx + 0x80]
0x1409d005c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409d0061 c685730d000000 mov byte ptr [rbp + 0xd73], 0
0x1409d0068 488d8d98070000 lea rcx, [rbp + 0x798]
0x1409d006f e84c35ecff call 0x1408935c0
0x1409d0074 c685730d000000 mov byte ptr [rbp + 0xd73], 0
0x1409d007b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409d0080 4883c458 add rsp, 0x58
0x1409d0084 5b pop rbx
0x1409d0085 5f pop rdi
0x1409d0086 5e pop rsi
0x1409d0087 415c pop r12
0x1409d0089 415d pop r13
0x1409d008b 415e pop r14
0x1409d008d 415f pop r15
0x1409d008f 5d pop rbp
0x1409d0090 c3 ret 
