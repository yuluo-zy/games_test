0x14205e980 55 push rbp
0x14205e981 4157 push r15
0x14205e983 4156 push r14
0x14205e985 4154 push r12
0x14205e987 56 push rsi
0x14205e988 57 push rdi
0x14205e989 53 push rbx
0x14205e98a 4881ec50020000 sub rsp, 0x250
0x14205e991 488dac2480000000 lea rbp, [rsp + 0x80]
0x14205e999 48c785c8010000feffffff mov qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
0x14205e9a4 4889d6 mov rsi, rdx
0x14205e9a7 4889cb mov rbx, rcx
0x14205e9aa 488d7a38 lea rdi, [rdx + 0x38]
0x14205e9ae 488d82a8050000 lea rax, [rdx + 0x5a8]
0x14205e9b5 4889bda8010000 mov qword ptr [rbp + 0x1a8], rdi
0x14205e9bc 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x14205e9c3 4883ba8800000000 cmp qword ptr [rdx + 0x88], 0
0x14205e9cb 48b89b0614a84e6c1412 movabs rax, 0x12146c4ea814069b
0x14205e9d5 48b9c676ae958b5344fa movabs rcx, 0xfa44538b95ae76c6
0x14205e9df 0f8495000000 je 0x14205ea7a
0x14205e9e5 488b5670 mov rdx, qword ptr [rsi + 0x70]
0x14205e9e9 4c8b4678 mov r8, qword ptr [rsi + 0x78]
0x14205e9ed 4d89c1 mov r9, r8
0x14205e9f0 4921c1 and r9, rax
0x14205e9f3 4531d2 xor r10d, r10d
0x14205e9f6 660f6f05321b8f00 movdqa xmm0, xmmword ptr [rip + 0x8f1b32]
0x14205e9fe 660f76c9 pcmpeqd xmm1, xmm1
0x14205ea02 f3420f6f140a movdqu xmm2, xmmword ptr [rdx + r9]
0x14205ea08 660f6fda movdqa xmm3, xmm2
0x14205ea0c 660f74d8 pcmpeqb xmm3, xmm0
0x14205ea10 66440fd7db pmovmskb r11d, xmm3
0x14205ea15 4585db test r11d, r11d
0x14205ea18 7533 jne 0x14205ea4d
0x14205ea1a 660f74d1 pcmpeqb xmm2, xmm1
0x14205ea1e 66440fd7da pmovmskb r11d, xmm2
0x14205ea23 4585db test r11d, r11d
0x14205ea26 7552 jne 0x14205ea7a
0x14205ea28 4d01d1 add r9, r10
0x14205ea2b 4983c110 add r9, 0x10
0x14205ea2f 4983c210 add r10, 0x10
0x14205ea33 4d21c1 and r9, r8
0x14205ea36 ebca jmp 0x14205ea02
0x14205ea38 0f1f840000000000 nop dword ptr [rax + rax]
0x14205ea40 458d73ff lea r14d, [r11 - 1]
0x14205ea44 664521de and r14w, r11w
0x14205ea48 4589f3 mov r11d, r14d
0x14205ea4b 74cd je 0x14205ea1a
0x14205ea4d f3450fbcf3 tzcnt r14d, r11d
0x14205ea52 4d01ce add r14, r9
0x14205ea55 4d21c6 and r14, r8
0x14205ea58 49f7de neg r14
0x14205ea5b 4f8d3476 lea r14, [r14 + r14*2]
0x14205ea5f 4a394cf2e8 cmp qword ptr [rdx + r14*8 - 0x18], rcx
0x14205ea64 75da jne 0x14205ea40
0x14205ea66 4a3944f2f0 cmp qword ptr [rdx + r14*8 - 0x10], rax
0x14205ea6b 75d3 jne 0x14205ea40
0x14205ea6d 4a8d04f2 lea rax, [rdx + r14*8]
0x14205ea71 4c8b70f8 mov r14, qword ptr [rax - 8]
0x14205ea75 e9ed020000 jmp 0x14205ed67
0x14205ea7a 0fb69698000000 movzx edx, byte ptr [rsi + 0x98]
0x14205ea81 488b96d8000000 mov rdx, qword ptr [rsi + 0xd8]
0x14205ea88 4c8b8ee0000000 mov r9, qword ptr [rsi + 0xe0]
0x14205ea8f 4d89ca mov r10, r9
0x14205ea92 4921c2 and r10, rax
0x14205ea95 4531db xor r11d, r11d
0x14205ea98 660f6f05901a8f00 movdqa xmm0, xmmword ptr [rip + 0x8f1a90]
0x14205eaa0 660f76c9 pcmpeqd xmm1, xmm1
0x14205eaa4 f3420f6f1412 movdqu xmm2, xmmword ptr [rdx + r10]
0x14205eaaa 660f6fda movdqa xmm3, xmm2
0x14205eaae 660f74d8 pcmpeqb xmm3, xmm0
0x14205eab2 66440fd7f3 pmovmskb r14d, xmm3
0x14205eab7 4585f6 test r14d, r14d
0x14205eaba 7531 jne 0x14205eaed
0x14205eabc 660f74d1 pcmpeqb xmm2, xmm1
0x14205eac0 66440fd7c2 pmovmskb r8d, xmm2
0x14205eac5 4585c0 test r8d, r8d
0x14205eac8 0f85ef000000 jne 0x14205ebbd
0x14205eace 4d01da add r10, r11
0x14205ead1 4983c210 add r10, 0x10
0x14205ead5 4983c310 add r11, 0x10
0x14205ead9 4d21ca and r10, r9
0x14205eadc ebc6 jmp 0x14205eaa4
0x14205eade 6690 nop 
0x14205eae0 458d46ff lea r8d, [r14 - 1]
0x14205eae4 664521f0 and r8w, r14w
0x14205eae8 4589c6 mov r14d, r8d
0x14205eaeb 74cf je 0x14205eabc
0x14205eaed f3450fbcfe tzcnt r15d, r14d
0x14205eaf2 4d01d7 add r15, r10
0x14205eaf5 4d21cf and r15, r9
0x14205eaf8 4d89f8 mov r8, r15
0x14205eafb 49f7d8 neg r8
0x14205eafe 4e8d24fd00000000 lea r12, [r15*8]
0x14205eb06 49c1e007 shl r8, 7
0x14205eb0a 4d29e0 sub r8, r12
0x14205eb0d 4a398c0278ffffff cmp qword ptr [rdx + r8 - 0x88], rcx
0x14205eb15 75c9 jne 0x14205eae0
0x14205eb17 4a39440280 cmp qword ptr [rdx + r8 - 0x80], rax
0x14205eb1c 75c2 jne 0x14205eae0
0x14205eb1e 4d89fa mov r10, r15
0x14205eb21 49c1e207 shl r10, 7
0x14205eb25 4f8d1cfa lea r11, [r10 + r15*8]
0x14205eb29 49c1fb03 sar r11, 3
0x14205eb2d 49baf1f0f0f0f0f0f0f0 movabs r10, 0xf0f0f0f0f0f0f0f1
0x14205eb37 4d0fafd3 imul r10, r11
0x14205eb3b 4d8d5af0 lea r11, [r10 - 0x10]
0x14205eb3f 4d21cb and r11, r9
0x14205eb42 f3420f6f041a movdqu xmm0, xmmword ptr [rdx + r11]
0x14205eb48 660f76c9 pcmpeqd xmm1, xmm1
0x14205eb4c 660f74c1 pcmpeqb xmm0, xmm1
0x14205eb50 66440fd7f0 pmovmskb r14d, xmm0
0x14205eb55 f3420f6f0412 movdqu xmm0, xmmword ptr [rdx + r10]
0x14205eb5b 660f74c1 pcmpeqb xmm0, xmm1
0x14205eb5f 66440fd7c8 pmovmskb r9d, xmm0
0x14205eb64 4585f6 test r14d, r14d
0x14205eb67 740b je 0x14205eb74
0x14205eb69 66450fbdf6 bsr r14w, r14w
0x14205eb6e 4183f60f xor r14d, 0xf
0x14205eb72 eb05 jmp 0x14205eb79
0x14205eb74 6641be1000 mov r14w, 0x10
0x14205eb79 4901d0 add r8, rdx
0x14205eb7c 4181c900000100 or r9d, 0x10000
0x14205eb83 f3450fbcc9 tzcnt r9d, r9d
0x14205eb88 4501ce add r14d, r9d
0x14205eb8b 41b180 mov r9b, 0x80
0x14205eb8e 664183fe0f cmp r14w, 0xf
0x14205eb93 770a ja 0x14205eb9f
0x14205eb95 48ff86e8000000 inc qword ptr [rsi + 0xe8]
0x14205eb9c 41b1ff mov r9b, 0xff
0x14205eb9f 46880c12 mov byte ptr [rdx + r10], r9b
0x14205eba3 46884c1a10 mov byte ptr [rdx + r11 + 0x10], r9b
0x14205eba8 48ff8ef0000000 dec qword ptr [rsi + 0xf0]
0x14205ebaf 498b5088 mov rdx, qword ptr [r8 - 0x78]
0x14205ebb3 4883fa02 cmp rdx, 2
0x14205ebb7 0f8537010000 jne 0x14205ecf4
0x14205ebbd 4c8bb6a8050000 mov r14, qword ptr [rsi + 0x5a8]
0x14205ebc4 4d8d7e01 lea r15, [r14 + 1]
0x14205ebc8 4c89bea8050000 mov qword ptr [rsi + 0x5a8], r15
0x14205ebcf 48ba0000000000000080 movabs rdx, 0x8000000000000000
0x14205ebd9 48899580010000 mov qword ptr [rbp + 0x180], rdx
0x14205ebe0 488d15245bbd00 lea rdx, [rip + 0xbd5b24]
0x14205ebe7 48899588010000 mov qword ptr [rbp + 0x188], rdx
0x14205ebee 48c7859001000041000000 mov qword ptr [rbp + 0x190], 0x41
0x14205ebf9 66c785a00100000001 mov word ptr [rbp + 0x1a0], 0x100
0x14205ec02 48c7854801000001000000 mov qword ptr [rbp + 0x148], 1
0x14205ec0d 48898d50010000 mov qword ptr [rbp + 0x150], rcx
0x14205ec14 48898558010000 mov qword ptr [rbp + 0x158], rax
0x14205ec1b 48c7857001000008000000 mov qword ptr [rbp + 0x170], 8
0x14205ec26 48c7857801000008000000 mov qword ptr [rbp + 0x178], 8
0x14205ec31 48c7859801000000000000 mov qword ptr [rbp + 0x198], 0
0x14205ec3c c685a201000001 mov byte ptr [rbp + 0x1a2], 1
0x14205ec43 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
0x14205ec4e 488d4da8 lea rcx, [rbp - 0x58]
0x14205ec52 4c8d8548010000 lea r8, [rbp + 0x148]
0x14205ec59 4c89f2 mov rdx, r14
0x14205ec5c e80f0ccefe call 0x140d3f870
0x14205ec61 4c397e48 cmp qword ptr [rsi + 0x48], r15
0x14205ec65 7319 jae 0x14205ec80
0x14205ec67 c685c701000001 mov byte ptr [rbp + 0x1c7], 1
0x14205ec6e 4c8d054b06f700 lea r8, [rip + 0xf7064b]
0x14205ec75 4889f9 mov rcx, rdi
0x14205ec78 4c89fa mov rdx, r15
0x14205ec7b e8c05629ff call 0x1412f4340
0x14205ec80 4c8b7e40 mov r15, qword ptr [rsi + 0x40]
0x14205ec84 4d69e6d0000000 imul r12, r14, 0xd0
0x14205ec8b 4b8d0427 lea rax, [r15 + r12]
0x14205ec8f 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
0x14205ec96 488d4d78 lea rcx, [rbp + 0x78]
0x14205ec9a 488d55a8 lea rdx, [rbp - 0x58]
0x14205ec9e 41b8d0000000 mov r8d, 0xd0
0x14205eca4 e8574f8c00 call 0x142923c00
0x14205eca9 43833c2702 cmp dword ptr [r15 + r12], 2
0x14205ecae 740c je 0x14205ecbc
0x14205ecb0 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
0x14205ecb7 e8941e59fe call 0x1405f0b50
0x14205ecbc 488d5578 lea rdx, [rbp + 0x78]
0x14205ecc0 41b8d0000000 mov r8d, 0xd0
0x14205ecc6 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
0x14205eccd e82e4f8c00 call 0x142923c00
0x14205ecd2 488d4e70 lea rcx, [rsi + 0x70]
0x14205ecd6 48bac676ae958b5344fa movabs rdx, 0xfa44538b95ae76c6
0x14205ece0 49b89b0614a84e6c1412 movabs r8, 0x12146c4ea814069b
0x14205ecea 4d89f1 mov r9, r14
0x14205eced e82e3f02fe call 0x140082c20
0x14205ecf2 eb73 jmp 0x14205ed67
0x14205ecf4 410f1040f0 movups xmm0, xmmword ptr [r8 - 0x10]
0x14205ecf9 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x14205ed00 410f1040e0 movups xmm0, xmmword ptr [r8 - 0x20]
0x14205ed05 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x14205ed0c 410f1040d0 movups xmm0, xmmword ptr [r8 - 0x30]
0x14205ed11 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14205ed18 f3410f6f4090 movdqu xmm0, xmmword ptr [r8 - 0x70]
0x14205ed1e f3410f6f48a0 movdqu xmm1, xmmword ptr [r8 - 0x60]
0x14205ed24 f3410f6f50b0 movdqu xmm2, xmmword ptr [r8 - 0x50]
0x14205ed2a f3410f6f58c0 movdqu xmm3, xmmword ptr [r8 - 0x40]
0x14205ed30 f30f7f9db0000000 movdqu xmmword ptr [rbp + 0xb0], xmm3
0x14205ed38 f30f7f95a0000000 movdqu xmmword ptr [rbp + 0xa0], xmm2
0x14205ed40 f30f7f8d90000000 movdqu xmmword ptr [rbp + 0x90], xmm1
0x14205ed48 f30f7f8580000000 movdqu xmmword ptr [rbp + 0x80], xmm0
0x14205ed50 48895578 mov qword ptr [rbp + 0x78], rdx
0x14205ed54 488d4d78 lea rcx, [rbp + 0x78]
0x14205ed58 488d95a8010000 lea rdx, [rbp + 0x1a8]
0x14205ed5f e8dc0ccefe call 0x140d3fa40
0x14205ed64 4989c6 mov r14, rax
0x14205ed67 48899d48010000 mov qword ptr [rbp + 0x148], rbx
0x14205ed6e 8b9eb0050000 mov ebx, dword ptr [rsi + 0x5b0]
0x14205ed74 4889f1 mov rcx, rsi
0x14205ed77 e8d494c9fe call 0x140cf8250
0x14205ed7c 488d86f8000000 lea rax, [rsi + 0xf8]
0x14205ed83 4c8975a8 mov qword ptr [rbp - 0x58], r14
0x14205ed87 4881c6d8010000 add rsi, 0x1d8
0x14205ed8e 48897d78 mov qword ptr [rbp + 0x78], rdi
0x14205ed92 488d4da8 lea rcx, [rbp - 0x58]
0x14205ed96 48898d80000000 mov qword ptr [rbp + 0x80], rcx
0x14205ed9d 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14205eda4 4c8d4578 lea r8, [rbp + 0x78]
0x14205eda8 4889f1 mov rcx, rsi
0x14205edab 4c89f2 mov rdx, r14
0x14205edae e84dcb7cfe call 0x14082b900
0x14205edb3 4889c6 mov rsi, rax
0x14205edb6 4883782000 cmp qword ptr [rax + 0x20], 0
0x14205edbb 7413 je 0x14205edd0
0x14205edbd 4c8d8548010000 lea r8, [rbp + 0x148]
0x14205edc4 4889f1 mov rcx, rsi
0x14205edc7 31d2 xor edx, edx
0x14205edc9 e8a208cafe call 0x140cff670
0x14205edce eb2c jmp 0x14205edfc
0x14205edd0 48837e1800 cmp qword ptr [rsi + 0x18], 0
0x14205edd5 743a je 0x14205ee11
0x14205edd7 31c9 xor ecx, ecx
0x14205edd9 488d4101 lea rax, [rcx + 1]
0x14205eddd 48894620 mov qword ptr [rsi + 0x20], rax
0x14205ede1 4c8b4608 mov r8, qword ptr [rsi + 8]
0x14205ede5 490fafc8 imul rcx, r8
0x14205ede9 48034e10 add rcx, qword ptr [rsi + 0x10]
0x14205eded 488d9548010000 lea rdx, [rbp + 0x148]
0x14205edf4 e8074e8c00 call 0x142923c00
0x14205edf9 895e58 mov dword ptr [rsi + 0x58], ebx
0x14205edfc 895e5c mov dword ptr [rsi + 0x5c], ebx
0x14205edff 4881c450020000 add rsp, 0x250
0x14205ee06 5b pop rbx
0x14205ee07 5f pop rdi
0x14205ee08 5e pop rsi
0x14205ee09 415c pop r12
0x14205ee0b 415e pop r14
0x14205ee0d 415f pop r15
0x14205ee0f 5d pop rbp
0x14205ee10 c3 ret 
0x14205ee11 ba01000000 mov edx, 1
0x14205ee16 4889f1 mov rcx, rsi
0x14205ee19 e822fe8600 call 0x1428cec40
0x14205ee1e 488b4e20 mov rcx, qword ptr [rsi + 0x20]
0x14205ee22 ebb5 jmp 0x14205edd9
0x14205ee24 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14205ee30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14205ee35 55 push rbp
0x14205ee36 4157 push r15
0x14205ee38 4156 push r14
0x14205ee3a 4154 push r12
0x14205ee3c 56 push rsi
0x14205ee3d 57 push rdi
0x14205ee3e 53 push rbx
0x14205ee3f 4883ec20 sub rsp, 0x20
0x14205ee43 488daa80000000 lea rbp, [rdx + 0x80]
0x14205ee4a 488d5578 lea rdx, [rbp + 0x78]
0x14205ee4e 41b8d0000000 mov r8d, 0xd0
0x14205ee54 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
0x14205ee5b e8a04d8c00 call 0x142923c00
0x14205ee60 c685c701000000 mov byte ptr [rbp + 0x1c7], 0
0x14205ee67 4883c420 add rsp, 0x20
0x14205ee6b 5b pop rbx
0x14205ee6c 5f pop rdi
0x14205ee6d 5e pop rsi
0x14205ee6e 415c pop r12
0x14205ee70 415e pop r14
0x14205ee72 415f pop r15
0x14205ee74 5d pop rbp
0x14205ee75 c3 ret 
0x14205ee76 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14205ee80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14205ee85 55 push rbp
0x14205ee86 4157 push r15
0x14205ee88 4156 push r14
0x14205ee8a 4154 push r12
0x14205ee8c 56 push rsi
0x14205ee8d 57 push rdi
0x14205ee8e 53 push rbx
0x14205ee8f 4883ec20 sub rsp, 0x20
0x14205ee93 488daa80000000 lea rbp, [rdx + 0x80]
0x14205ee9a 80bdc701000000 cmp byte ptr [rbp + 0x1c7], 0
0x14205eea1 750f jne 0x14205eeb2
0x14205eea3 4883c420 add rsp, 0x20
0x14205eea7 5b pop rbx
0x14205eea8 5f pop rdi
0x14205eea9 5e pop rsi
0x14205eeaa 415c pop r12
0x14205eeac 415e pop r14
0x14205eeae 415f pop r15
0x14205eeb0 5d pop rbp
0x14205eeb1 c3 ret 
0x14205eeb2 488d4da8 lea rcx, [rbp - 0x58]
0x14205eeb6 e8951c59fe call 0x1405f0b50
0x14205eebb ebe6 jmp 0x14205eea3
