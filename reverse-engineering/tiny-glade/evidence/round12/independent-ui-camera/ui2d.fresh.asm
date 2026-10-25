14092a100 55 push rbp
14092a101 4157 push r15
14092a103 4156 push r14
14092a105 4155 push r13
14092a107 4154 push r12
14092a109 56 push rsi
14092a10a 57 push rdi
14092a10b 53 push rbx
14092a10c 4883ec48 sub rsp, 0x48
14092a110 488d6c2440 lea rbp, [rsp + 0x40]
14092a115 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
14092a11d 4889d6 mov rsi, rdx
14092a120 4889cf mov rdi, rcx
14092a123 e8d8c82e00 call 0x140c16a00
14092a128 89c3 mov ebx, eax
14092a12a 84c0 test al, al
14092a12c 0f841e020000 je 0x14092a350
14092a132 4c8d3d8f441802 lea r15, [rip + 0x218448f]
14092a139 41be3d000000 mov r14d, 0x3d
14092a13f ba3d000000 mov edx, 0x3d
14092a144 4c89f9 mov rcx, r15
14092a147 e8d497ffff call 0x140923920
14092a14c 4883f801 cmp rax, 1
14092a150 0f85ab000000 jne 0x14092a201
14092a156 4989d1 mov r9, rdx
14092a159 4885d2 test rdx, rdx
14092a15c 7436 je 0x14092a194
14092a15e 4983f93d cmp r9, 0x3d
14092a162 732e jae 0x14092a192
14092a164 488d055d441802 lea rax, [rip + 0x218445d]
14092a16b 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14092a170 7f22 jg 0x14092a194
14092a172 488d05bf451802 lea rax, [rip + 0x21845bf]
14092a179 4889442420 mov qword ptr [rsp + 0x20], rax
14092a17e 488d0d43441802 lea rcx, [rip + 0x2184443]
14092a185 ba3d000000 mov edx, 0x3d
14092a18a 4531c0 xor r8d, r8d
14092a18d e8aef7fa01 call 0x1428d9940
14092a192 75de jne 0x14092a172
14092a194 4c8d3d2d441802 lea r15, [rip + 0x218442d]
14092a19b 4c89f9 mov rcx, r15
14092a19e 4c89ca mov rdx, r9
14092a1a1 e87a97ffff call 0x140923920
14092a1a6 4883f801 cmp rax, 1
14092a1aa 7555 jne 0x14092a201
14092a1ac 4989d0 mov r8, rdx
14092a1af 4983c002 add r8, 2
14092a1b3 7439 je 0x14092a1ee
14092a1b5 4983f83d cmp r8, 0x3d
14092a1b9 7331 jae 0x14092a1ec
14092a1bb 488d0506441802 lea rax, [rip + 0x2184406]
14092a1c2 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14092a1c7 7f25 jg 0x14092a1ee
14092a1c9 488d0580451802 lea rax, [rip + 0x2184580]
14092a1d0 4889442420 mov qword ptr [rsp + 0x20], rax
14092a1d5 488d0dec431802 lea rcx, [rip + 0x21843ec]
14092a1dc ba3d000000 mov edx, 0x3d
14092a1e1 41b93d000000 mov r9d, 0x3d
14092a1e7 e854f7fa01 call 0x1428d9940
14092a1ec 75db jne 0x14092a1c9
14092a1ee 41be3b000000 mov r14d, 0x3b
14092a1f4 4929d6 sub r14, rdx
14092a1f7 4c8d3dca431802 lea r15, [rip + 0x21843ca]
14092a1fe 4d01c7 add r15, r8
14092a201 488d05cd531802 lea rax, [rip + 0x21853cd]
14092a208 4c8d2591531802 lea r12, [rip + 0x2185391]
14092a20f 90 nop 
14092a210 0fbe48ff movsx ecx, byte ptr [rax - 1]
14092a214 85c9 test ecx, ecx
14092a216 7818 js 0x14092a230
14092a218 48ffc8 dec rax
14092a21b 83f95c cmp ecx, 0x5c
14092a21e 7569 jne 0x14092a289
14092a220 eb7d jmp 0x14092a29f
14092a222 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14092a230 0fb650fe movzx edx, byte ptr [rax - 2]
14092a234 80fac0 cmp dl, 0xc0
14092a237 7d25 jge 0x14092a25e
14092a239 440fb640fd movzx r8d, byte ptr [rax - 3]
14092a23e 4180f8c0 cmp r8b, 0xc0
14092a242 7d23 jge 0x14092a267
14092a244 440fb648fc movzx r9d, byte ptr [rax - 4]
14092a249 4883c0fc add rax, -4
14092a24d 4183e107 and r9d, 7
14092a251 41c1e106 shl r9d, 6
14092a255 4183e03f and r8d, 0x3f
14092a259 4509c8 or r8d, r9d
14092a25c eb11 jmp 0x14092a26f
14092a25e 4883c0fe add rax, -2
14092a262 83e21f and edx, 0x1f
14092a265 eb12 jmp 0x14092a279
14092a267 4883c0fd add rax, -3
14092a26b 4183e00f and r8d, 0xf
14092a26f 41c1e006 shl r8d, 6
14092a273 83e23f and edx, 0x3f
14092a276 4409c2 or edx, r8d
14092a279 c1e206 shl edx, 6
14092a27c 80e13f and cl, 0x3f
14092a27f 0fb6c9 movzx ecx, cl
14092a282 09d1 or ecx, edx
14092a284 83f95c cmp ecx, 0x5c
14092a287 7416 je 0x14092a29f
14092a289 83f92f cmp ecx, 0x2f
14092a28c 7411 je 0x14092a29f
14092a28e 4c39e0 cmp rax, r12
14092a291 0f8579ffffff jne 0x14092a210
14092a297 41bd35000000 mov r13d, 0x35
14092a29d eb50 jmp 0x14092a2ef
14092a29f 4c8d25fa521802 lea r12, [rip + 0x21852fa]
14092a2a6 4c29e0 sub rax, r12
14092a2a9 4989c0 mov r8, rax
14092a2ac 49ffc0 inc r8
14092a2af 7432 je 0x14092a2e3
14092a2b1 4983f835 cmp r8, 0x35
14092a2b5 732a jae 0x14092a2e1
14092a2b7 43803c20bf cmp byte ptr [r8 + r12], 0xbf
14092a2bc 7f25 jg 0x14092a2e3
14092a2be 488d0553441802 lea rax, [rip + 0x2184453]
14092a2c5 4889442420 mov qword ptr [rsp + 0x20], rax
14092a2ca 488d0dcf521802 lea rcx, [rip + 0x21852cf]
14092a2d1 ba35000000 mov edx, 0x35
14092a2d6 41b935000000 mov r9d, 0x35
14092a2dc e85ff6fa01 call 0x1428d9940
14092a2e1 75db jne 0x14092a2be
14092a2e3 41bd34000000 mov r13d, 0x34
14092a2e9 4929c5 sub r13, rax
14092a2ec 4d01c4 add r12, r8
14092a2ef e83cde2e00 call 0x140c18130
14092a2f4 4889c1 mov rcx, rax
14092a2f7 488b00 mov rax, qword ptr [rax]
14092a2fa 4883f801 cmp rax, 1
14092a2fe 0f852c020000 jne 0x14092a530
14092a304 4883c108 add rcx, 8
14092a308 48833900 cmp qword ptr [rcx], 0
14092a30c 0f8558020000 jne 0x14092a56a
14092a312 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14092a319 48894df8 mov qword ptr [rbp - 8], rcx
14092a31d 4883c108 add rcx, 8
14092a321 4c896c2420 mov qword ptr [rsp + 0x20], r13
14092a326 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14092a32f 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14092a338 4c89fa mov rdx, r15
14092a33b 4d89f0 mov r8, r14
14092a33e 4d89e1 mov r9, r12
14092a341 e8fac92e00 call 0x140c16d40
14092a346 4989c6 mov r14, rax
14092a349 488b45f8 mov rax, qword ptr [rbp - 8]
14092a34d 48ff00 inc qword ptr [rax]
14092a350 488b07 mov rax, qword ptr [rdi]
14092a353 488b4f08 mov rcx, qword ptr [rdi + 8]
14092a357 8b5714 mov edx, dword ptr [rdi + 0x14]
14092a35a 4c8b80f8000000 mov r8, qword ptr [rax + 0xf8]
14092a361 4c8b8800010000 mov r9, qword ptr [rax + 0x100]
14092a368 4f8d0c88 lea r9, [r8 + r9*4]
14092a36c 4c8b16 mov r10, qword ptr [rsi]
14092a36f 80b80801000000 cmp byte ptr [rax + 0x108], 0
14092a376 740a je 0x14092a382
14092a378 31f6 xor esi, esi
14092a37a 4531db xor r11d, r11d
14092a37d 4531ff xor r15d, r15d
14092a380 eb6a jmp 0x14092a3ec
14092a382 bf08000000 mov edi, 8
14092a387 31f6 xor esi, esi
14092a389 4531db xor r11d, r11d
14092a38c 4531e4 xor r12d, r12d
14092a38f e917010000 jmp 0x14092a4ab
14092a394 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14092a3a0 488d34fe lea rsi, [rsi + rdi*8]
14092a3a4 488bb810010000 mov rdi, qword ptr [rax + 0x110]
14092a3ab 4c8b7e18 mov r15, qword ptr [rsi + 0x18]
14092a3af 488b7638 mov rsi, qword ptr [rsi + 0x38]
14092a3b3 488b34fe mov rsi, qword ptr [rsi + rdi*8]
14092a3b7 48f7d6 not rsi
14092a3ba 488d3c76 lea rdi, [rsi + rsi*2]
14092a3be 48c1e704 shl rdi, 4
14092a3c2 498b743f10 mov rsi, qword ptr [r15 + rdi + 0x10]
14092a3c7 498b7c3f28 mov rdi, qword ptr [r15 + rdi + 0x28]
14092a3cc 4983c004 add r8, 4
14092a3d0 4531ff xor r15d, r15d
14092a3d3 4589fc mov r12d, r15d
14092a3d6 49ffc7 inc r15
14092a3d9 f2410f1002 movsd xmm0, qword ptr [r10]
14092a3de 428914a7 mov dword ptr [rdi + r12*4], edx
14092a3e2 49c1e404 shl r12, 4
14092a3e6 f2420f110426 movsd qword ptr [rsi + r12], xmm0
14092a3ec 4d39df cmp r15, r11
14092a3ef 75e2 jne 0x14092a3d3
14092a3f1 4d39c8 cmp r8, r9
14092a3f4 0f84e8000000 je 0x14092a4e2
14092a3fa 488bb1a8010000 mov rsi, qword ptr [rcx + 0x1a8]
14092a401 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14092a410 458b18 mov r11d, dword ptr [r8]
14092a413 4b8d3cdb lea rdi, [r11 + r11*8]
14092a417 4c8b5cfe10 mov r11, qword ptr [rsi + rdi*8 + 0x10]
14092a41c 4d85db test r11, r11
14092a41f 0f857bffffff jne 0x14092a3a0
14092a425 4983c004 add r8, 4
14092a429 4d39c8 cmp r8, r9
14092a42c 75e2 jne 0x14092a410
14092a42e e9af000000 jmp 0x14092a4e2
14092a433 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14092a440 4801f7 add rdi, rsi
14092a443 8bb794000000 mov esi, dword ptr [rdi + 0x94]
14092a449 4c8bb9a8010000 mov r15, qword ptr [rcx + 0x1a8]
14092a450 488d34f6 lea rsi, [rsi + rsi*8]
14092a454 4c8ba010010000 mov r12, qword ptr [rax + 0x110]
14092a45b 4d8b6cf718 mov r13, qword ptr [r15 + rsi*8 + 0x18]
14092a460 498b74f738 mov rsi, qword ptr [r15 + rsi*8 + 0x38]
14092a465 4a8b34e6 mov rsi, qword ptr [rsi + r12*8]
14092a469 48f7d6 not rsi
14092a46c 4c8d3c76 lea r15, [rsi + rsi*2]
14092a470 49c1e704 shl r15, 4
14092a474 4b8b743d10 mov rsi, qword ptr [r13 + r15 + 0x10]
14092a479 4f8b7c3d28 mov r15, qword ptr [r13 + r15 + 0x28]
14092a47e 488b7f50 mov rdi, qword ptr [rdi + 0x50]
14092a482 4983c004 add r8, 4
14092a486 4531e4 xor r12d, r12d
14092a489 4d89e5 mov r13, r12
14092a48c 49c1e504 shl r13, 4
14092a490 468b6c2f08 mov r13d, dword ptr [rdi + r13 + 8]
14092a495 49ffc4 inc r12
14092a498 f2410f1002 movsd xmm0, qword ptr [r10]
14092a49d 438914af mov dword ptr [r15 + r13*4], edx
14092a4a1 49c1e504 shl r13, 4
14092a4a5 f2420f11042e movsd qword ptr [rsi + r13], xmm0
14092a4ab 4d39dc cmp r12, r11
14092a4ae 75d9 jne 0x14092a489
14092a4b0 4d39c8 cmp r8, r9
14092a4b3 742d je 0x14092a4e2
14092a4b5 488bb900010000 mov rdi, qword ptr [rcx + 0x100]
14092a4bc 0f1f4000 nop dword ptr [rax]
14092a4c0 458b18 mov r11d, dword ptr [r8]
14092a4c3 4b8d349b lea rsi, [r11 + r11*4]
14092a4c7 48c1e605 shl rsi, 5
14092a4cb 4c8b5c3758 mov r11, qword ptr [rdi + rsi + 0x58]
14092a4d0 4d85db test r11, r11
14092a4d3 0f8567ffffff jne 0x14092a440
14092a4d9 4983c004 add r8, 4
14092a4dd 4d39c8 cmp r8, r9
14092a4e0 75de jne 0x14092a4c0
14092a4e2 84db test bl, bl
14092a4e4 7439 je 0x14092a51f
14092a4e6 e845dc2e00 call 0x140c18130
14092a4eb 4889c1 mov rcx, rax
14092a4ee 488b00 mov rax, qword ptr [rax]
14092a4f1 4883f801 cmp rax, 1
14092a4f5 7553 jne 0x14092a54a
14092a4f7 4883c108 add rcx, 8
14092a4fb 48833900 cmp qword ptr [rcx], 0
14092a4ff 7569 jne 0x14092a56a
14092a501 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14092a508 48894df8 mov qword ptr [rbp - 8], rcx
14092a50c 4883c108 add rcx, 8
14092a510 4c89f2 mov rdx, r14
14092a513 e8d8cb2e00 call 0x140c170f0
14092a518 488b45f8 mov rax, qword ptr [rbp - 8]
14092a51c 48ff00 inc qword ptr [rax]
14092a51f 4883c448 add rsp, 0x48
14092a523 5b pop rbx
14092a524 5f pop rdi
14092a525 5e pop rsi
14092a526 415c pop r12
14092a528 415d pop r13
14092a52a 415e pop r14
14092a52c 415f pop r15
14092a52e 5d pop rbp
14092a52f c3 ret 
14092a530 4885c0 test rax, rax
14092a533 7529 jne 0x14092a55e
14092a535 31d2 xor edx, edx
14092a537 e894e2fc01 call 0x1428f87d0
14092a53c 4889c1 mov rcx, rax
14092a53f 4885c0 test rax, rax
14092a542 0f85c0fdffff jne 0x14092a308
14092a548 eb14 jmp 0x14092a55e
14092a54a 4885c0 test rax, rax
14092a54d 750f jne 0x14092a55e
14092a54f 31d2 xor edx, edx
14092a551 e87ae2fc01 call 0x1428f87d0
14092a556 4889c1 mov rcx, rax
14092a559 4885c0 test rax, rax
14092a55c 759d jne 0x14092a4fb
14092a55e 488d0d3b053102 lea rcx, [rip + 0x231053b]
14092a565 e866dbfa01 call 0x1428d80d0
14092a56a 488d0dff053102 lea rcx, [rip + 0x23105ff]
14092a571 e8daecfa01 call 0x1428d9250
14092a576 cc int3 
14092a577 660f1f840000000000 nop word ptr [rax + rax]
14092a580 4889542410 mov qword ptr [rsp + 0x10], rdx
14092a585 55 push rbp
14092a586 4157 push r15
14092a588 4156 push r14
14092a58a 4155 push r13
14092a58c 4154 push r12
14092a58e 56 push rsi
14092a58f 57 push rdi
14092a590 53 push rbx
14092a591 4883ec38 sub rsp, 0x38
14092a595 488d6a40 lea rbp, [rdx + 0x40]
14092a599 488b45f8 mov rax, qword ptr [rbp - 8]
14092a59d 48ff00 inc qword ptr [rax]
14092a5a0 4883c438 add rsp, 0x38
14092a5a4 5b pop rbx
14092a5a5 5f pop rdi
14092a5a6 5e pop rsi
14092a5a7 415c pop r12
14092a5a9 415d pop r13
14092a5ab 415e pop r14
14092a5ad 415f pop r15
14092a5af 5d pop rbp
14092a5b0 c3 ret 
14092a5b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14092a5c0 4889542410 mov qword ptr [rsp + 0x10], rdx
14092a5c5 55 push rbp
14092a5c6 4157 push r15
14092a5c8 4156 push r14
14092a5ca 4155 push r13
14092a5cc 4154 push r12
14092a5ce 56 push rsi
14092a5cf 57 push rdi
14092a5d0 53 push rbx
14092a5d1 4883ec38 sub rsp, 0x38
14092a5d5 488d6a40 lea rbp, [rdx + 0x40]
14092a5d9 488b45f8 mov rax, qword ptr [rbp - 8]
14092a5dd 48ff00 inc qword ptr [rax]
14092a5e0 4883c438 add rsp, 0x38
14092a5e4 5b pop rbx
14092a5e5 5f pop rdi
14092a5e6 5e pop rsi
14092a5e7 415c pop r12
14092a5e9 415d pop r13
14092a5eb 415e pop r14
14092a5ed 415f pop r15
14092a5ef 5d pop rbp
14092a5f0 c3 ret 
