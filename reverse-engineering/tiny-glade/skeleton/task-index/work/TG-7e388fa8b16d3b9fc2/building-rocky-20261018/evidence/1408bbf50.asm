0x1408bbf50 55 push rbp
0x1408bbf51 4157 push r15
0x1408bbf53 4156 push r14
0x1408bbf55 4155 push r13
0x1408bbf57 4154 push r12
0x1408bbf59 56 push rsi
0x1408bbf5a 57 push rdi
0x1408bbf5b 53 push rbx
0x1408bbf5c 4881ecc8000000 sub rsp, 0xc8
0x1408bbf63 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408bbf6b 48c74540feffffff mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x1408bbf73 4d89ce mov r14, r9
0x1408bbf76 4d89c5 mov r13, r8
0x1408bbf79 4989d0 mov r8, rdx
0x1408bbf7c 4889ca mov rdx, rcx
0x1408bbf7f 488d4da8 lea rcx, [rbp - 0x58]
0x1408bbf83 e8b8d14900 call 0x140d59140
0x1408bbf88 488b4da8 mov rcx, qword ptr [rbp - 0x58]
0x1408bbf8c 488b55b0 mov rdx, qword ptr [rbp - 0x50]
0x1408bbf90 4889c8 mov rax, rcx
0x1408bbf93 48f7d8 neg rax
0x1408bbf96 0f804d030000 jo 0x1408bc2e9
0x1408bbf9c 48894d20 mov qword ptr [rbp + 0x20], rcx
0x1408bbfa0 4c8b45b8 mov r8, qword ptr [rbp - 0x48]
0x1408bbfa4 488d4da8 lea rcx, [rbp - 0x58]
0x1408bbfa8 48895518 mov qword ptr [rbp + 0x18], rdx
0x1408bbfac e84f9bb400 call 0x141405b00
0x1408bbfb1 488b5da8 mov rbx, qword ptr [rbp - 0x58]
0x1408bbfb5 488b7db0 mov rdi, qword ptr [rbp - 0x50]
0x1408bbfb9 4889d8 mov rax, rbx
0x1408bbfbc 48f7d8 neg rax
0x1408bbfbf 7131 jno 0x1408bbff2
0x1408bbfc1 4889f9 mov rcx, rdi
0x1408bbfc4 e887d8fe01 call 0x1428a9850
0x1408bbfc9 4889c6 mov rsi, rax
0x1408bbfcc 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x1408bbfd0 4885d2 test rdx, rdx
0x1408bbfd3 0f847d010000 je 0x1408bc156
0x1408bbfd9 41b801000000 mov r8d, 1
0x1408bbfdf 488b4d18 mov rcx, qword ptr [rbp + 0x18]
0x1408bbfe3 e8387cd5ff call 0x140613c20
0x1408bbfe8 b801000000 mov eax, 1
0x1408bbfed e9e0020000 jmp 0x1408bc2d2
0x1408bbff2 0f1045b8 movups xmm0, xmmword ptr [rbp - 0x48]
0x1408bbff6 0f104dc8 movups xmm1, xmmword ptr [rbp - 0x38]
0x1408bbffa 0f114df8 movups xmmword ptr [rbp - 8], xmm1
0x1408bbffe 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1408bc002 48895dd8 mov qword ptr [rbp - 0x28], rbx
0x1408bc006 48897de0 mov qword ptr [rbp - 0x20], rdi
0x1408bc00a 488b75e8 mov rsi, qword ptr [rbp - 0x18]
0x1408bc00e 4531e4 xor r12d, r12d
0x1408bc011 b930000000 mov ecx, 0x30
0x1408bc016 4889f0 mov rax, rsi
0x1408bc019 48f7e1 mul rcx
0x1408bc01c 4989c7 mov r15, rax
0x1408bc01f 0f80d6020000 jo 0x1408bc2fb
0x1408bc025 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x1408bc02f 4939c7 cmp r15, rax
0x1408bc032 0f87c3020000 ja 0x1408bc2fb
0x1408bc038 4d85ff test r15, r15
0x1408bc03b 7431 je 0x1408bc06e
0x1408bc03d 0fb605c58bd702 movzx eax, byte ptr [rip + 0x2d78bc5]
0x1408bc044 41bc04000000 mov r12d, 4
0x1408bc04a ba04000000 mov edx, 4
0x1408bc04f 4c89f9 mov rcx, r15
0x1408bc052 e8b97bd5ff call 0x140613c10
0x1408bc057 4885c0 test rax, rax
0x1408bc05a 0f849b020000 je 0x1408bc2fb
0x1408bc060 4989c0 mov r8, rax
0x1408bc063 48897528 mov qword ptr [rbp + 0x28], rsi
0x1408bc067 4885f6 test rsi, rsi
0x1408bc06a 7515 jne 0x1408bc081
0x1408bc06c eb76 jmp 0x1408bc0e4
0x1408bc06e 41b804000000 mov r8d, 4
0x1408bc074 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1408bc07c 4885f6 test rsi, rsi
0x1408bc07f 7463 je 0x1408bc0e4
0x1408bc081 498d402c lea rax, [r8 + 0x2c]
0x1408bc085 4889f9 mov rcx, rdi
0x1408bc088 4889f2 mov rdx, rsi
0x1408bc08b 0f1f440000 nop dword ptr [rax + rax]
0x1408bc090 f30f10410c movss xmm0, dword ptr [rcx + 0xc]
0x1408bc095 448b5910 mov r11d, dword ptr [rcx + 0x10]
0x1408bc099 448b4920 mov r9d, dword ptr [rcx + 0x20]
0x1408bc09d 448b5108 mov r10d, dword ptr [rcx + 8]
0x1408bc0a1 448950dc mov dword ptr [rax - 0x24], r10d
0x1408bc0a5 4c8b11 mov r10, qword ptr [rcx]
0x1408bc0a8 4c8950d4 mov qword ptr [rax - 0x2c], r10
0x1408bc0ac f30f1140e0 movss dword ptr [rax - 0x20], xmm0
0x1408bc0b1 4c8b11 mov r10, qword ptr [rcx]
0x1408bc0b4 4c8950e4 mov qword ptr [rax - 0x1c], r10
0x1408bc0b8 448b5108 mov r10d, dword ptr [rcx + 8]
0x1408bc0bc 448950ec mov dword ptr [rax - 0x14], r10d
0x1408bc0c0 448958f0 mov dword ptr [rax - 0x10], r11d
0x1408bc0c4 4c8b5114 mov r10, qword ptr [rcx + 0x14]
0x1408bc0c8 4c8950f4 mov qword ptr [rax - 0xc], r10
0x1408bc0cc 448b511c mov r10d, dword ptr [rcx + 0x1c]
0x1408bc0d0 448950fc mov dword ptr [rax - 4], r10d
0x1408bc0d4 448908 mov dword ptr [rax], r9d
0x1408bc0d7 4883c030 add rax, 0x30
0x1408bc0db 4883c124 add rcx, 0x24
0x1408bc0df 48ffca dec rdx
0x1408bc0e2 75ac jne 0x1408bc090
0x1408bc0e4 4989f1 mov r9, rsi
0x1408bc0e7 48897d08 mov qword ptr [rbp + 8], rdi
0x1408bc0eb 4c89f1 mov rcx, r14
0x1408bc0ee 4d89ef mov r15, r13
0x1408bc0f1 4c89ea mov rdx, r13
0x1408bc0f4 4c894510 mov qword ptr [rbp + 0x10], r8
0x1408bc0f8 e8f300a000 call 0x1412bc1f0
0x1408bc0fd 4c8b6500 mov r12, qword ptr [rbp]
0x1408bc101 4d89e5 mov r13, r12
0x1408bc104 49c1e505 shl r13, 5
0x1408bc108 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x1408bc112 4939c5 cmp r13, rax
0x1408bc115 0f87f4010000 ja 0x1408bc30f
0x1408bc11b 4c89f7 mov rdi, r14
0x1408bc11e 4c8b75f8 mov r14, qword ptr [rbp - 8]
0x1408bc122 4d85ed test r13, r13
0x1408bc125 7439 je 0x1408bc160
0x1408bc127 0fb605db8ad702 movzx eax, byte ptr [rip + 0x2d78adb]
0x1408bc12e b804000000 mov eax, 4
0x1408bc133 48894538 mov qword ptr [rbp + 0x38], rax
0x1408bc137 ba04000000 mov edx, 4
0x1408bc13c 4c89e9 mov rcx, r13
0x1408bc13f e8cc7ad5ff call 0x140613c10
0x1408bc144 4885c0 test rax, rax
0x1408bc147 0f84ca010000 je 0x1408bc317
0x1408bc14d 4989c0 mov r8, rax
0x1408bc150 4c896530 mov qword ptr [rbp + 0x30], r12
0x1408bc154 eb18 jmp 0x1408bc16e
0x1408bc156 b801000000 mov eax, 1
0x1408bc15b e972010000 jmp 0x1408bc2d2
0x1408bc160 41b804000000 mov r8d, 4
0x1408bc166 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x1408bc16e 4d85e4 test r12, r12
0x1408bc171 4c89fa mov rdx, r15
0x1408bc174 4d89f5 mov r13, r14
0x1408bc177 4889f9 mov rcx, rdi
0x1408bc17a 488b7d08 mov rdi, qword ptr [rbp + 8]
0x1408bc17e 4c894538 mov qword ptr [rbp + 0x38], r8
0x1408bc182 0f84b6000000 je 0x1408bc23e
0x1408bc188 4983fc01 cmp r12, 1
0x1408bc18c 7504 jne 0x1408bc192
0x1408bc18e 31c0 xor eax, eax
0x1408bc190 eb71 jmp 0x1408bc203
0x1408bc192 4d89e6 mov r14, r12
0x1408bc195 4983e6fe and r14, 0xfffffffffffffffe
0x1408bc199 4d8d7d1c lea r15, [r13 + 0x1c]
0x1408bc19d 4983c030 add r8, 0x30
0x1408bc1a1 4531c9 xor r9d, r9d
0x1408bc1a4 0f57c0 xorps xmm0, xmm0
0x1408bc1a7 31c0 xor eax, eax
0x1408bc1a9 0f1f8000000000 nop dword ptr [rax]
0x1408bc1b0 478b540ff0 mov r10d, dword ptr [r15 + r9 - 0x10]
0x1408bc1b5 478b5c0fec mov r11d, dword ptr [r15 + r9 - 0x14]
0x1408bc1ba 47895c48d8 mov dword ptr [r8 + r9*2 - 0x28], r11d
0x1408bc1bf 4f8b5c0fe4 mov r11, qword ptr [r15 + r9 - 0x1c]
0x1408bc1c4 4f895c48d0 mov qword ptr [r8 + r9*2 - 0x30], r11
0x1408bc1c9 47895448dc mov dword ptr [r8 + r9*2 - 0x24], r10d
0x1408bc1ce 430f114448e0 movups xmmword ptr [r8 + r9*2 - 0x20], xmm0
0x1408bc1d4 478b140f mov r10d, dword ptr [r15 + r9]
0x1408bc1d8 478b5c0ffc mov r11d, dword ptr [r15 + r9 - 4]
0x1408bc1dd 47895c48f8 mov dword ptr [r8 + r9*2 - 8], r11d
0x1408bc1e2 4f8b5c0ff4 mov r11, qword ptr [r15 + r9 - 0xc]
0x1408bc1e7 4f895c48f0 mov qword ptr [r8 + r9*2 - 0x10], r11
0x1408bc1ec 47895448fc mov dword ptr [r8 + r9*2 - 4], r10d
0x1408bc1f1 4883c002 add rax, 2
0x1408bc1f5 430f110448 movups xmmword ptr [r8 + r9*2], xmm0
0x1408bc1fa 4983c120 add r9, 0x20
0x1408bc1fe 4939c6 cmp r14, rax
0x1408bc201 75ad jne 0x1408bc1b0
0x1408bc203 41f6c401 test r12b, 1
0x1408bc207 4c8b4538 mov r8, qword ptr [rbp + 0x38]
0x1408bc20b 7431 je 0x1408bc23e
0x1408bc20d 4989c2 mov r10, rax
0x1408bc210 49c1e204 shl r10, 4
0x1408bc214 478b4c150c mov r9d, dword ptr [r13 + r10 + 0xc]
0x1408bc219 48c1e005 shl rax, 5
0x1408bc21d 478b5c1508 mov r11d, dword ptr [r13 + r10 + 8]
0x1408bc222 45895c0008 mov dword ptr [r8 + rax + 8], r11d
0x1408bc227 4f8b541500 mov r10, qword ptr [r13 + r10]
0x1408bc22c 4d891400 mov qword ptr [r8 + rax], r10
0x1408bc230 45894c000c mov dword ptr [r8 + rax + 0xc], r9d
0x1408bc235 0f57c0 xorps xmm0, xmm0
0x1408bc238 410f11440010 movups xmmword ptr [r8 + rax + 0x10], xmm0
0x1408bc23e 4d89e1 mov r9, r12
0x1408bc241 e82af89f00 call 0x1412bba70
0x1408bc246 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1408bc24a 4885d2 test rdx, rdx
0x1408bc24d 7413 je 0x1408bc262
0x1408bc24f 48c1e205 shl rdx, 5
0x1408bc253 41b804000000 mov r8d, 4
0x1408bc259 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1408bc25d e8be79d5ff call 0x140613c20
0x1408bc262 488b4528 mov rax, qword ptr [rbp + 0x28]
0x1408bc266 4885c0 test rax, rax
0x1408bc269 7417 je 0x1408bc282
0x1408bc26b 48c1e004 shl rax, 4
0x1408bc26f 488d1440 lea rdx, [rax + rax*2]
0x1408bc273 41b804000000 mov r8d, 4
0x1408bc279 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1408bc27d e89e79d5ff call 0x140613c20
0x1408bc282 4885db test rbx, rbx
0x1408bc285 7416 je 0x1408bc29d
0x1408bc287 48c1e302 shl rbx, 2
0x1408bc28b 488d14db lea rdx, [rbx + rbx*8]
0x1408bc28f 41b804000000 mov r8d, 4
0x1408bc295 4889f9 mov rcx, rdi
0x1408bc298 e88379d5ff call 0x140613c20
0x1408bc29d 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x1408bc2a1 4885d2 test rdx, rdx
0x1408bc2a4 7412 je 0x1408bc2b8
0x1408bc2a6 48c1e204 shl rdx, 4
0x1408bc2aa 41b804000000 mov r8d, 4
0x1408bc2b0 4c89e9 mov rcx, r13
0x1408bc2b3 e86879d5ff call 0x140613c20
0x1408bc2b8 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x1408bc2bc 4885d2 test rdx, rdx
0x1408bc2bf 740f je 0x1408bc2d0
0x1408bc2c1 41b801000000 mov r8d, 1
0x1408bc2c7 488b4d18 mov rcx, qword ptr [rbp + 0x18]
0x1408bc2cb e85079d5ff call 0x140613c20
0x1408bc2d0 31c0 xor eax, eax
0x1408bc2d2 4889f2 mov rdx, rsi
0x1408bc2d5 4881c4c8000000 add rsp, 0xc8
0x1408bc2dc 5b pop rbx
0x1408bc2dd 5f pop rdi
0x1408bc2de 5e pop rsi
0x1408bc2df 415c pop r12
0x1408bc2e1 415d pop r13
0x1408bc2e3 415e pop r14
0x1408bc2e5 415f pop r15
0x1408bc2e7 5d pop rbp
0x1408bc2e8 c3 ret 
0x1408bc2e9 4889d1 mov rcx, rdx
0x1408bc2ec e89fd2fe01 call 0x1428a9590
0x1408bc2f1 4889c6 mov rsi, rax
0x1408bc2f4 b801000000 mov eax, 1
0x1408bc2f9 ebd7 jmp 0x1408bc2d2
0x1408bc2fb 4c8d053e401e02 lea r8, [rip + 0x21e403e]
0x1408bc302 4c89e1 mov rcx, r12
0x1408bc305 4c89fa mov rdx, r15
0x1408bc308 e8d6cc0102 call 0x1428d8fe3
0x1408bc30d eb1b jmp 0x1408bc32a
0x1408bc30f 48c7453800000000 mov qword ptr [rbp + 0x38], 0
0x1408bc317 4c8d0522401e02 lea r8, [rip + 0x21e4022]
0x1408bc31e 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1408bc322 4c89ea mov rdx, r13
0x1408bc325 e8b9cc0102 call 0x1428d8fe3
0x1408bc32a 0f0b ud2 
0x1408bc32c 0f1f4000 nop dword ptr [rax]
0x1408bc330 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408bc335 55 push rbp
0x1408bc336 4157 push r15
0x1408bc338 4156 push r14
0x1408bc33a 4155 push r13
0x1408bc33c 4154 push r12
0x1408bc33e 56 push rsi
0x1408bc33f 57 push rdi
0x1408bc340 53 push rbx
0x1408bc341 4883ec28 sub rsp, 0x28
0x1408bc345 488daa80000000 lea rbp, [rdx + 0x80]
0x1408bc34c 48837d2000 cmp qword ptr [rbp + 0x20], 0
0x1408bc351 7413 je 0x1408bc366
0x1408bc353 41b801000000 mov r8d, 1
0x1408bc359 488b4d18 mov rcx, qword ptr [rbp + 0x18]
0x1408bc35d 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x1408bc361 e8ba78d5ff call 0x140613c20
0x1408bc366 90 nop 
0x1408bc367 4883c428 add rsp, 0x28
0x1408bc36b 5b pop rbx
0x1408bc36c 5f pop rdi
0x1408bc36d 5e pop rsi
0x1408bc36e 415c pop r12
0x1408bc370 415d pop r13
0x1408bc372 415e pop r14
0x1408bc374 415f pop r15
0x1408bc376 5d pop rbp
0x1408bc377 c3 ret 
0x1408bc378 0f1f840000000000 nop dword ptr [rax + rax]
0x1408bc380 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408bc385 55 push rbp
0x1408bc386 4157 push r15
0x1408bc388 4156 push r14
0x1408bc38a 4155 push r13
0x1408bc38c 4154 push r12
0x1408bc38e 56 push rsi
0x1408bc38f 57 push rdi
0x1408bc390 53 push rbx
0x1408bc391 4883ec28 sub rsp, 0x28
0x1408bc395 488daa80000000 lea rbp, [rdx + 0x80]
0x1408bc39c 488d4dd8 lea rcx, [rbp - 0x28]
0x1408bc3a0 e83b47fdff call 0x140890ae0
0x1408bc3a5 90 nop 
0x1408bc3a6 4883c428 add rsp, 0x28
0x1408bc3aa 5b pop rbx
0x1408bc3ab 5f pop rdi
0x1408bc3ac 5e pop rsi
0x1408bc3ad 415c pop r12
0x1408bc3af 415d pop r13
0x1408bc3b1 415e pop r14
0x1408bc3b3 415f pop r15
0x1408bc3b5 5d pop rbp
0x1408bc3b6 c3 ret 
0x1408bc3b7 660f1f840000000000 nop word ptr [rax + rax]
0x1408bc3c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408bc3c5 55 push rbp
0x1408bc3c6 4157 push r15
0x1408bc3c8 4156 push r14
0x1408bc3ca 4155 push r13
0x1408bc3cc 4154 push r12
0x1408bc3ce 56 push rsi
0x1408bc3cf 57 push rdi
0x1408bc3d0 53 push rbx
0x1408bc3d1 4883ec28 sub rsp, 0x28
0x1408bc3d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1408bc3dc 48837d2800 cmp qword ptr [rbp + 0x28], 0
0x1408bc3e1 741b je 0x1408bc3fe
0x1408bc3e3 488b4528 mov rax, qword ptr [rbp + 0x28]
0x1408bc3e7 48c1e004 shl rax, 4
0x1408bc3eb 488d1440 lea rdx, [rax + rax*2]
0x1408bc3ef 41b804000000 mov r8d, 4
0x1408bc3f5 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1408bc3f9 e82278d5ff call 0x140613c20
0x1408bc3fe 90 nop 
0x1408bc3ff 4883c428 add rsp, 0x28
0x1408bc403 5b pop rbx
0x1408bc404 5f pop rdi
0x1408bc405 5e pop rsi
0x1408bc406 415c pop r12
0x1408bc408 415d pop r13
0x1408bc40a 415e pop r14
0x1408bc40c 415f pop r15
0x1408bc40e 5d pop rbp
0x1408bc40f c3 ret 
0x1408bc410 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408bc415 55 push rbp
0x1408bc416 4157 push r15
0x1408bc418 4156 push r14
0x1408bc41a 4155 push r13
0x1408bc41c 4154 push r12
0x1408bc41e 56 push rsi
0x1408bc41f 57 push rdi
0x1408bc420 53 push rbx
0x1408bc421 4883ec28 sub rsp, 0x28
0x1408bc425 488daa80000000 lea rbp, [rdx + 0x80]
0x1408bc42c 48837d3000 cmp qword ptr [rbp + 0x30], 0
0x1408bc431 7417 je 0x1408bc44a
0x1408bc433 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1408bc437 48c1e205 shl rdx, 5
0x1408bc43b 41b804000000 mov r8d, 4
0x1408bc441 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1408bc445 e8d677d5ff call 0x140613c20
0x1408bc44a 90 nop 
0x1408bc44b 4883c428 add rsp, 0x28
0x1408bc44f 5b pop rbx
0x1408bc450 5f pop rdi
0x1408bc451 5e pop rsi
0x1408bc452 415c pop r12
0x1408bc454 415d pop r13
0x1408bc456 415e pop r14
0x1408bc458 415f pop r15
0x1408bc45a 5d pop rbp
0x1408bc45b c3 ret 
