14207c0a0: 55 push rbp
14207c0a1: 4157 push r15
14207c0a3: 4156 push r14
14207c0a5: 4155 push r13
14207c0a7: 4154 push r12
14207c0a9: 56 push rsi
14207c0aa: 57 push rdi
14207c0ab: 53 push rbx
14207c0ac: 4881ecb8010000 sub rsp, 0x1b8
14207c0b3: 488dac2480000000 lea rbp, [rsp + 0x80]
14207c0bb: 440f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm11
14207c0c3: 440f299510010000 movaps xmmword ptr [rbp + 0x110], xmm10
14207c0cb: 440f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm9
14207c0d3: 440f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm8
14207c0db: 0f29bde0000000 movaps xmmword ptr [rbp + 0xe0], xmm7
14207c0e2: 660f7fb5d0000000 movdqa xmmword ptr [rbp + 0xd0], xmm6
14207c0ea: 48c785c8000000feffffff mov qword ptr [rbp + 0xc8], 0xfffffffffffffffe
14207c0f5: 4c89c6 mov rsi, r8
14207c0f8: 4889d7 mov rdi, rdx
14207c0fb: 4889cb mov rbx, rcx
14207c0fe: e8fda8b9fe call 0x140c16a00
14207c103: 84c0 test al, al
14207c105: 7460 je 0x14207c167
14207c107: 4c8d3dee6ff500 lea r15, [rip + 0xf56fee]
14207c10e: 41be36000000 mov r14d, 0x36
14207c114: ba36000000 mov edx, 0x36
14207c119: 4c89f9 mov rcx, r15
14207c11c: e84f2dffff call 0x14206ee70
14207c121: 4883f801 cmp rax, 1
14207c125: 0f85b2000000 jne 0x14207c1dd
14207c12b: 4989d1 mov r9, rdx
14207c12e: 4885d2 test rdx, rdx
14207c131: 743d je 0x14207c170
14207c133: 4983f936 cmp r9, 0x36
14207c137: 7335 jae 0x14207c16e
14207c139: 488d05bc6ff500 lea rax, [rip + 0xf56fbc]
14207c140: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14207c145: 7f29 jg 0x14207c170
14207c147: 488d058a72f500 lea rax, [rip + 0xf5728a]
14207c14e: 4889442420 mov qword ptr [rsp + 0x20], rax
14207c153: 488d0da26ff500 lea rcx, [rip + 0xf56fa2]
14207c15a: ba36000000 mov edx, 0x36
14207c15f: 4531c0 xor r8d, r8d
14207c162: e8d9d78500 call 0x1428d9940
14207c167: 31c0 xor eax, eax
14207c169: e9c2010000 jmp 0x14207c330
14207c16e: 75d7 jne 0x14207c147
14207c170: 4c8d3d856ff500 lea r15, [rip + 0xf56f85]
14207c177: 4c89f9 mov rcx, r15
14207c17a: 4c89ca mov rdx, r9
14207c17d: e8ee2cffff call 0x14206ee70
14207c182: 4883f801 cmp rax, 1
14207c186: 7555 jne 0x14207c1dd
14207c188: 4989d0 mov r8, rdx
14207c18b: 4983c002 add r8, 2
14207c18f: 7439 je 0x14207c1ca
14207c191: 4983f836 cmp r8, 0x36
14207c195: 7331 jae 0x14207c1c8
14207c197: 488d055e6ff500 lea rax, [rip + 0xf56f5e]
14207c19e: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207c1a3: 7f25 jg 0x14207c1ca
14207c1a5: 488d054472f500 lea rax, [rip + 0xf57244]
14207c1ac: 4889442420 mov qword ptr [rsp + 0x20], rax
14207c1b1: 488d0d446ff500 lea rcx, [rip + 0xf56f44]
14207c1b8: ba36000000 mov edx, 0x36
14207c1bd: 41b936000000 mov r9d, 0x36
14207c1c3: e878d78500 call 0x1428d9940
14207c1c8: 75db jne 0x14207c1a5
14207c1ca: 41be34000000 mov r14d, 0x34
14207c1d0: 4929d6 sub r14, rdx
14207c1d3: 4c8d3d226ff500 lea r15, [rip + 0xf56f22]
14207c1da: 4d01c7 add r15, r8
14207c1dd: 4c8d057a72f500 lea r8, [rip + 0xf5727a]
14207c1e4: 4c8d254572f500 lea r12, [rip + 0xf57245]
14207c1eb: 0f1f440000 nop dword ptr [rax + rax]
14207c1f0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14207c1f5: 85c0 test eax, eax
14207c1f7: 7817 js 0x14207c210
14207c1f9: 49ffc8 dec r8
14207c1fc: 83f85c cmp eax, 0x5c
14207c1ff: 7563 jne 0x14207c264
14207c201: eb73 jmp 0x14207c276
14207c203: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207c210: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14207c215: 80f9c0 cmp cl, 0xc0
14207c218: 7d23 jge 0x14207c23d
14207c21a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207c21f: 80fac0 cmp dl, 0xc0
14207c222: 7d22 jge 0x14207c246
14207c224: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14207c229: 4983c0fc add r8, -4
14207c22d: 4183e107 and r9d, 7
14207c231: 41c1e106 shl r9d, 6
14207c235: 83e23f and edx, 0x3f
14207c238: 4409ca or edx, r9d
14207c23b: eb10 jmp 0x14207c24d
14207c23d: 4983c0fe add r8, -2
14207c241: 83e11f and ecx, 0x1f
14207c244: eb0f jmp 0x14207c255
14207c246: 4983c0fd add r8, -3
14207c24a: 83e20f and edx, 0xf
14207c24d: c1e206 shl edx, 6
14207c250: 83e13f and ecx, 0x3f
14207c253: 09d1 or ecx, edx
14207c255: c1e106 shl ecx, 6
14207c258: 243f and al, 0x3f
14207c25a: 0fb6c0 movzx eax, al
14207c25d: 09c8 or eax, ecx
14207c25f: 83f85c cmp eax, 0x5c
14207c262: 7412 je 0x14207c276
14207c264: 83f82f cmp eax, 0x2f
14207c267: 740d je 0x14207c276
14207c269: 4d39e0 cmp r8, r12
14207c26c: 7582 jne 0x14207c1f0
14207c26e: 41bd2e000000 mov r13d, 0x2e
14207c274: eb4d jmp 0x14207c2c3
14207c276: 4c8d25b371f500 lea r12, [rip + 0xf571b3]
14207c27d: 4d29e0 sub r8, r12
14207c280: 49ffc0 inc r8
14207c283: 7432 je 0x14207c2b7
14207c285: 4983f82e cmp r8, 0x2e
14207c289: 732a jae 0x14207c2b5
14207c28b: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
14207c290: 7f25 jg 0x14207c2b7
14207c292: 488d051f71f500 lea rax, [rip + 0xf5711f]
14207c299: 4889442420 mov qword ptr [rsp + 0x20], rax
14207c29e: 488d0d8b71f500 lea rcx, [rip + 0xf5718b]
14207c2a5: ba2e000000 mov edx, 0x2e
14207c2aa: 41b92e000000 mov r9d, 0x2e
14207c2b0: e88bd68500 call 0x1428d9940
14207c2b5: 75db jne 0x14207c292
14207c2b7: 41bd2e000000 mov r13d, 0x2e
14207c2bd: 4d29c5 sub r13, r8
14207c2c0: 4d01c4 add r12, r8
14207c2c3: e868beb9fe call 0x140c18130
14207c2c8: 4889c1 mov rcx, rax
14207c2cb: 488b00 mov rax, qword ptr [rax]
14207c2ce: 4883f801 cmp rax, 1
14207c2d2: 0f85c0070000 jne 0x14207ca98
14207c2d8: 4883c108 add rcx, 8
14207c2dc: 48833900 cmp qword ptr [rcx], 0
14207c2e0: 0f85f0070000 jne 0x14207cad6
14207c2e6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207c2ed: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207c2f4: 4883c108 add rcx, 8
14207c2f8: 4c896c2420 mov qword ptr [rsp + 0x20], r13
14207c2fd: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14207c306: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207c30f: 4c89fa mov rdx, r15
14207c312: 4d89f0 mov r8, r14
14207c315: 4d89e1 mov r9, r12
14207c318: e823aab9fe call 0x140c16d40
14207c31d: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
14207c324: 48ff01 inc qword ptr [rcx]
14207c327: 48894540 mov qword ptr [rbp + 0x40], rax
14207c32b: b801000000 mov eax, 1
14207c330: 48894538 mov qword ptr [rbp + 0x38], rax
14207c334: 488b0e mov rcx, qword ptr [rsi]
14207c337: b214 mov dl, 0x14
14207c339: e8125991fe call 0x140991c50
14207c33e: 48895530 mov qword ptr [rbp + 0x30], rdx
14207c342: 48894568 mov qword ptr [rbp + 0x68], rax
14207c346: 4c8b13 mov r10, qword ptr [rbx]
14207c349: 4c8b5b08 mov r11, qword ptr [rbx + 8]
14207c34d: 4d8bb2f8000000 mov r14, qword ptr [r10 + 0xf8]
14207c354: 498b8200010000 mov rax, qword ptr [r10 + 0x100]
14207c35b: 498d1c86 lea rbx, [r14 + rax*4]
14207c35f: 450fb6ba08010000 movzx r15d, byte ptr [r10 + 0x108]
14207c367: 488b17 mov rdx, qword ptr [rdi]
14207c36a: b808000000 mov eax, 8
14207c36f: 48894570 mov qword ptr [rbp + 0x70], rax
14207c373: 4531e4 xor r12d, r12d
14207c376: 660f76f6 pcmpeqd xmm6, xmm6
14207c37a: f30f103d82958a00 movss xmm7, dword ptr [rip + 0x8a9582]
14207c382: 440f2805f6258b00 movaps xmm8, xmmword ptr [rip + 0x8b25f6]
14207c38a: 4531ed xor r13d, r13d
14207c38d: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
14207c398: 4531c0 xor r8d, r8d
14207c39b: 31f6 xor esi, esi
14207c39d: 4531c9 xor r9d, r9d
14207c3a0: 4488bdc7000000 mov byte ptr [rbp + 0xc7], r15b
14207c3a7: 4584ff test r15b, r15b
14207c3aa: 0f8580010000 jne 0x14207c530
14207c3b0: 4939f1 cmp r9, rsi
14207c3b3: 0f8559010000 jne 0x14207c512
14207c3b9: 0f1f8000000000 nop dword ptr [rax]
14207c3c0: 4939de cmp r14, rbx
14207c3c3: 0f8414060000 je 0x14207c9dd
14207c3c9: 418b0e mov ecx, dword ptr [r14]
14207c3cc: 4983c604 add r14, 4
14207c3d0: 498b8300010000 mov rax, qword ptr [r11 + 0x100]
14207c3d7: 488d0c89 lea rcx, [rcx + rcx*4]
14207c3db: 48c1e105 shl rcx, 5
14207c3df: 488b740858 mov rsi, qword ptr [rax + rcx + 0x58]
14207c3e4: 4885f6 test rsi, rsi
14207c3e7: 74d7 je 0x14207c3c0
14207c3e9: 4889d7 mov rdi, rdx
14207c3ec: 4801c8 add rax, rcx
14207c3ef: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14207c3f5: 498b93a8010000 mov rdx, qword ptr [r11 + 0x1a8]
14207c3fc: 4c8d0cc9 lea r9, [rcx + rcx*8]
14207c400: 4a8d0cca lea rcx, [rdx + r9*8]
14207c404: 4d8b8210010000 mov r8, qword ptr [r10 + 0x110]
14207c40b: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
14207c410: 4c39c2 cmp rdx, r8
14207c413: 762f jbe 0x14207c444
14207c415: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207c419: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207c41d: 4d85c0 test r8, r8
14207c420: 7422 je 0x14207c444
14207c422: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207c426: 49f7d0 not r8
14207c429: 4f8d0440 lea r8, [r8 + r8*2]
14207c42d: 49c1e004 shl r8, 4
14207c431: 4f8b640110 mov r12, qword ptr [r9 + r8 + 0x10]
14207c436: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
14207c43d: 4c39c2 cmp rdx, r8
14207c440: 7711 ja 0x14207c453
14207c442: eb3e jmp 0x14207c482
14207c444: 4531e4 xor r12d, r12d
14207c447: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
14207c44e: 4c39c2 cmp rdx, r8
14207c451: 762f jbe 0x14207c482
14207c453: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207c457: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207c45b: 4d85c0 test r8, r8
14207c45e: 7422 je 0x14207c482
14207c460: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207c464: 49f7d0 not r8
14207c467: 4f8d0440 lea r8, [r8 + r8*2]
14207c46b: 49c1e004 shl r8, 4
14207c46f: 4f8b6c0110 mov r13, qword ptr [r9 + r8 + 0x10]
14207c474: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
14207c47b: 4c39c2 cmp rdx, r8
14207c47e: 7647 jbe 0x14207c4c7
14207c480: eb0f jmp 0x14207c491
14207c482: 4531ed xor r13d, r13d
14207c485: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
14207c48c: 4c39c2 cmp rdx, r8
14207c48f: 7636 jbe 0x14207c4c7
14207c491: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207c495: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207c499: 4d85c0 test r8, r8
14207c49c: 7429 je 0x14207c4c7
14207c49e: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207c4a2: 49f7d0 not r8
14207c4a5: 4f8d0440 lea r8, [r8 + r8*2]
14207c4a9: 49c1e004 shl r8, 4
14207c4ad: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207c4b2: 4c8985b8000000 mov qword ptr [rbp + 0xb8], r8
14207c4b9: 4d8b8228010000 mov r8, qword ptr [r10 + 0x128]
14207c4c0: 4c39c2 cmp rdx, r8
14207c4c3: 763c jbe 0x14207c501
14207c4c5: eb17 jmp 0x14207c4de
14207c4c7: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
14207c4d2: 4d8b8228010000 mov r8, qword ptr [r10 + 0x128]
14207c4d9: 4c39c2 cmp rdx, r8
14207c4dc: 7623 jbe 0x14207c501
14207c4de: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14207c4e2: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
14207c4e6: 4885d2 test rdx, rdx
14207c4e9: 7416 je 0x14207c501
14207c4eb: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14207c4ef: 48f7d2 not rdx
14207c4f2: 488d1452 lea rdx, [rdx + rdx*2]
14207c4f6: 48c1e204 shl rdx, 4
14207c4fa: 4c8b441110 mov r8, qword ptr [rcx + rdx + 0x10]
14207c4ff: eb03 jmp 0x14207c504
14207c501: 4531c0 xor r8d, r8d
14207c504: 488b4050 mov rax, qword ptr [rax + 0x50]
14207c508: 48894570 mov qword ptr [rbp + 0x70], rax
14207c50c: 4531c9 xor r9d, r9d
14207c50f: 4889fa mov rdx, rdi
14207c512: 4c89c8 mov rax, r9
14207c515: 48c1e004 shl rax, 4
14207c519: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
14207c51d: 8b440108 mov eax, dword ptr [rcx + rax + 8]
14207c521: e94e010000 jmp 0x14207c674
14207c526: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207c530: 4939f1 cmp r9, rsi
14207c533: 0f8538010000 jne 0x14207c671
14207c539: 0f1f8000000000 nop dword ptr [rax]
14207c540: 4939de cmp r14, rbx
14207c543: 0f8494040000 je 0x14207c9dd
14207c549: 418b0e mov ecx, dword ptr [r14]
14207c54c: 4983c604 add r14, 4
14207c550: 498b83a8010000 mov rax, qword ptr [r11 + 0x1a8]
14207c557: 488d0cc9 lea rcx, [rcx + rcx*8]
14207c55b: 488b74c810 mov rsi, qword ptr [rax + rcx*8 + 0x10]
14207c560: 4885f6 test rsi, rsi
14207c563: 74db je 0x14207c540
14207c565: 4989d1 mov r9, rdx
14207c568: 488d04c8 lea rax, [rax + rcx*8]
14207c56c: 498b9210010000 mov rdx, qword ptr [r10 + 0x110]
14207c573: 488b4840 mov rcx, qword ptr [rax + 0x40]
14207c577: 4839d1 cmp rcx, rdx
14207c57a: 762f jbe 0x14207c5ab
14207c57c: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207c580: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207c584: 4885d2 test rdx, rdx
14207c587: 7422 je 0x14207c5ab
14207c589: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207c58d: 48f7d2 not rdx
14207c590: 488d1452 lea rdx, [rdx + rdx*2]
14207c594: 48c1e204 shl rdx, 4
14207c598: 4d8b641010 mov r12, qword ptr [r8 + rdx + 0x10]
14207c59d: 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
14207c5a4: 4839d1 cmp rcx, rdx
14207c5a7: 7711 ja 0x14207c5ba
14207c5a9: eb3e jmp 0x14207c5e9
14207c5ab: 4531e4 xor r12d, r12d
14207c5ae: 498b9218010000 mov rdx, qword ptr [r10 + 0x118]
14207c5b5: 4839d1 cmp rcx, rdx
14207c5b8: 762f jbe 0x14207c5e9
14207c5ba: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207c5be: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207c5c2: 4885d2 test rdx, rdx
14207c5c5: 7422 je 0x14207c5e9
14207c5c7: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207c5cb: 48f7d2 not rdx
14207c5ce: 488d1452 lea rdx, [rdx + rdx*2]
14207c5d2: 48c1e204 shl rdx, 4
14207c5d6: 4d8b6c1010 mov r13, qword ptr [r8 + rdx + 0x10]
14207c5db: 498b9220010000 mov rdx, qword ptr [r10 + 0x120]
14207c5e2: 4839d1 cmp rcx, rdx
14207c5e5: 7647 jbe 0x14207c62e
14207c5e7: eb0f jmp 0x14207c5f8
14207c5e9: 4531ed xor r13d, r13d
14207c5ec: 498b9220010000 mov rdx, qword ptr [r10 + 0x120]
14207c5f3: 4839d1 cmp rcx, rdx
14207c5f6: 7636 jbe 0x14207c62e
14207c5f8: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207c5fc: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207c600: 4885d2 test rdx, rdx
14207c603: 7429 je 0x14207c62e
14207c605: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207c609: 48f7d2 not rdx
14207c60c: 488d1452 lea rdx, [rdx + rdx*2]
14207c610: 48c1e204 shl rdx, 4
14207c614: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207c619: 488995b8000000 mov qword ptr [rbp + 0xb8], rdx
14207c620: 498b9228010000 mov rdx, qword ptr [r10 + 0x128]
14207c627: 4839d1 cmp rcx, rdx
14207c62a: 763c jbe 0x14207c668
14207c62c: eb17 jmp 0x14207c645
14207c62e: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
14207c639: 498b9228010000 mov rdx, qword ptr [r10 + 0x128]
14207c640: 4839d1 cmp rcx, rdx
14207c643: 7623 jbe 0x14207c668
14207c645: 488b4838 mov rcx, qword ptr [rax + 0x38]
14207c649: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14207c64d: 4885c9 test rcx, rcx
14207c650: 7416 je 0x14207c668
14207c652: 488b4018 mov rax, qword ptr [rax + 0x18]
14207c656: 48f7d1 not rcx
14207c659: 488d0c49 lea rcx, [rcx + rcx*2]
14207c65d: 48c1e104 shl rcx, 4
14207c661: 4c8b440810 mov r8, qword ptr [rax + rcx + 0x10]
14207c666: eb03 jmp 0x14207c66b
14207c668: 4531c0 xor r8d, r8d
14207c66b: 4c89ca mov rdx, r9
14207c66e: 4531c9 xor r9d, r9d
14207c671: 4489c8 mov eax, r9d
14207c674: 488d0c8500000000 lea rcx, [rax*4]
14207c67c: 4c8d3c49 lea r15, [rcx + rcx*2]
14207c680: 4c896558 mov qword ptr [rbp + 0x58], r12
14207c684: 4b8d0c3c lea rcx, [r12 + r15]
14207c688: 488d3c40 lea rdi, [rax + rax*2]
14207c68c: 48c1e704 shl rdi, 4
14207c690: 4c896d60 mov qword ptr [rbp + 0x60], r13
14207c694: 4c01ef add rdi, r13
14207c697: 4c03bdb8000000 add r15, qword ptr [rbp + 0xb8]
14207c69e: 4c897d50 mov qword ptr [rbp + 0x50], r15
14207c6a2: 498d04c0 lea rax, [r8 + rax*8]
14207c6a6: 488b00 mov rax, qword ptr [rax]
14207c6a9: 488945c0 mov qword ptr [rbp - 0x40], rax
14207c6ad: 48837a4800 cmp qword ptr [rdx + 0x48], 0
14207c6b2: 0f8417030000 je 0x14207c9cf
14207c6b8: 49ffc1 inc r9
14207c6bb: 4c8b7a30 mov r15, qword ptr [rdx + 0x30]
14207c6bf: 4c8b6a38 mov r13, qword ptr [rdx + 0x38]
14207c6c3: 4c896d48 mov qword ptr [rbp + 0x48], r13
14207c6c7: 4921c5 and r13, rax
14207c6ca: 48c1e839 shr rax, 0x39
14207c6ce: 660f6ec0 movd xmm0, eax
14207c6d2: 660f60c0 punpcklbw xmm0, xmm0
14207c6d6: f20f70c000 pshuflw xmm0, xmm0, 0
14207c6db: 66440f70c800 pshufd xmm9, xmm0, 0
14207c6e1: 498d47e8 lea rax, [r15 - 0x18]
14207c6e5: 48894528 mov qword ptr [rbp + 0x28], rax
14207c6e9: 48c785a800000000000000 mov qword ptr [rbp + 0xa8], 0
14207c6f4: f3470f6f142f movdqu xmm10, xmmword ptr [r15 + r13]
14207c6fa: 66410f6fc2 movdqa xmm0, xmm10
14207c6ff: 66410f74c1 pcmpeqb xmm0, xmm9
14207c704: 66440fd7e0 pmovmskb r12d, xmm0
14207c709: 4585e4 test r12d, r12d
14207c70c: 0f849e000000 je 0x14207c7b0
14207c712: 4c899580000000 mov qword ptr [rbp + 0x80], r10
14207c719: 4c899d88000000 mov qword ptr [rbp + 0x88], r11
14207c720: 48899d90000000 mov qword ptr [rbp + 0x90], rbx
14207c727: 488995b0000000 mov qword ptr [rbp + 0xb0], rdx
14207c72e: 4c898598000000 mov qword ptr [rbp + 0x98], r8
14207c735: 4c898da0000000 mov qword ptr [rbp + 0xa0], r9
14207c73c: 48894d78 mov qword ptr [rbp + 0x78], rcx
14207c740: f3410fbcc4 tzcnt eax, r12d
14207c745: 4c01e8 add rax, r13
14207c748: 48234548 and rax, qword ptr [rbp + 0x48]
14207c74c: 48f7d8 neg rax
14207c74f: 488d1c40 lea rbx, [rax + rax*2]
14207c753: 488b4528 mov rax, qword ptr [rbp + 0x28]
14207c757: 488d14d8 lea rdx, [rax + rbx*8]
14207c75b: 488d4dc0 lea rcx, [rbp - 0x40]
14207c75f: e80c06c2fe call 0x140c9cd70
14207c764: 84c0 test al, al
14207c766: 0f8584000000 jne 0x14207c7f0
14207c76c: 418d4424ff lea eax, [r12 - 1]
14207c771: 664421e0 and ax, r12w
14207c775: 4189c4 mov r12d, eax
14207c778: 4c8b9580000000 mov r10, qword ptr [rbp + 0x80]
14207c77f: 4c8b9d88000000 mov r11, qword ptr [rbp + 0x88]
14207c786: 488b9d90000000 mov rbx, qword ptr [rbp + 0x90]
14207c78d: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
14207c794: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
14207c79b: 4c8b8da0000000 mov r9, qword ptr [rbp + 0xa0]
14207c7a2: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
14207c7a6: 7598 jne 0x14207c740
14207c7a8: 0f1f840000000000 nop dword ptr [rax + rax]
14207c7b0: 66440f74d6 pcmpeqb xmm10, xmm6
14207c7b5: 66410fd7c2 pmovmskb eax, xmm10
14207c7ba: 85c0 test eax, eax
14207c7bc: 0f850d020000 jne 0x14207c9cf
14207c7c2: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
14207c7c9: 4901c5 add r13, rax
14207c7cc: 4983c510 add r13, 0x10
14207c7d0: 4883c010 add rax, 0x10
14207c7d4: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
14207c7db: 4c236d48 and r13, qword ptr [rbp + 0x48]
14207c7df: e910ffffff jmp 0x14207c6f4
14207c7e4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207c7f0: 498b54dff0 mov rdx, qword ptr [r15 + rbx*8 - 0x10]
14207c7f5: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
14207c7fc: e80f0b8cfe call 0x14093d310
14207c801: 8b4f18 mov ecx, dword ptr [rdi + 0x18]
14207c804: 894d20 mov dword ptr [rbp + 0x20], ecx
14207c807: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
14207c80b: 48894d18 mov qword ptr [rbp + 0x18], rcx
14207c80f: 0f2827 movaps xmm4, xmmword ptr [rdi]
14207c812: f30f105f08 movss xmm3, dword ptr [rdi + 8]
14207c817: 440f28cb movaps xmm9, xmm3
14207c81b: f3440f58cb addss xmm9, xmm3
14207c820: f3410f59d9 mulss xmm3, xmm9
14207c825: 0f28cc movaps xmm1, xmm4
14207c828: 0f58cc addps xmm1, xmm4
14207c82b: f30f16c1 movshdup xmm0, xmm1
14207c82f: f30f59c4 mulss xmm0, xmm4
14207c833: 0f28d4 movaps xmm2, xmm4
14207c836: f3410f12e9 movsldup xmm5, xmm9
14207c83b: 0f59ec mulps xmm5, xmm4
14207c83e: 0fc6e4ff shufps xmm4, xmm4, 0xff
14207c842: 0f59d1 mulps xmm2, xmm1
14207c845: 0f59cc mulps xmm1, xmm4
14207c848: 440f28d4 movaps xmm10, xmm4
14207c84c: f3450f59d1 mulss xmm10, xmm9
14207c851: f3440f16da movshdup xmm11, xmm2
14207c856: 450f28cb movaps xmm9, xmm11
14207c85a: f3440f58cb addss xmm9, xmm3
14207c85f: 0f28e7 movaps xmm4, xmm7
14207c862: f3410f5ce1 subss xmm4, xmm9
14207c867: 440f28c8 movaps xmm9, xmm0
14207c86b: f3450f58ca addss xmm9, xmm10
14207c870: 410f14e1 unpcklps xmm4, xmm9
14207c874: f3410f5cc2 subss xmm0, xmm10
14207c879: f30f58da addss xmm3, xmm2
14207c87d: 440f28cf movaps xmm9, xmm7
14207c881: f3440f5ccb subss xmm9, xmm3
14207c886: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
14207c88a: 410f14c1 unpcklps xmm0, xmm9
14207c88e: 440f28cd movaps xmm9, xmm5
14207c892: 440f5cc9 subps xmm9, xmm1
14207c896: 0f58cd addps xmm1, xmm5
14207c899: f30f105f20 movss xmm3, dword ptr [rdi + 0x20]
14207c89e: f3410f58d3 addss xmm2, xmm11
14207c8a3: 440f28d7 movaps xmm10, xmm7
14207c8a7: f3440f5cd2 subss xmm10, xmm2
14207c8ac: f30f106f24 movss xmm5, dword ptr [rdi + 0x24]
14207c8b1: 410f16e1 movlhps xmm4, xmm9
14207c8b5: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
14207c8b9: 440fc6c901 shufps xmm9, xmm1, 1
14207c8be: 440fc6d130 shufps xmm10, xmm1, 0x30
14207c8c3: 0f10571c movups xmm2, xmmword ptr [rdi + 0x1c]
14207c8c7: 450fc6ca82 shufps xmm9, xmm10, 0x82
14207c8cc: 0f104f20 movups xmm1, xmmword ptr [rdi + 0x20]
14207c8d0: 0fc6d200 shufps xmm2, xmm2, 0
14207c8d4: 0f59d4 mulps xmm2, xmm4
14207c8d7: 0f28e1 movaps xmm4, xmm1
14207c8da: 0fc6e100 shufps xmm4, xmm1, 0
14207c8de: 0f59e0 mulps xmm4, xmm0
14207c8e1: 0fc6c955 shufps xmm1, xmm1, 0x55
14207c8e5: 410f59c9 mulps xmm1, xmm9
14207c8e9: 410f54d8 andps xmm3, xmm8
14207c8ed: 410f54e8 andps xmm5, xmm8
14207c8f1: 0f28c3 movaps xmm0, xmm3
14207c8f4: f30fc2c303 cmpunordss xmm0, xmm3
14207c8f9: 440f28c8 movaps xmm9, xmm0
14207c8fd: 440f54cd andps xmm9, xmm5
14207c901: f30f5feb maxss xmm5, xmm3
14207c905: f30f105f1c movss xmm3, dword ptr [rdi + 0x1c]
14207c90a: 410f54d8 andps xmm3, xmm8
14207c90e: 0f55c5 andnps xmm0, xmm5
14207c911: 410f56c1 orps xmm0, xmm9
14207c915: 0f28e8 movaps xmm5, xmm0
14207c918: f30f5feb maxss xmm5, xmm3
14207c91c: f30fc2db03 cmpunordss xmm3, xmm3
14207c921: 440f28cb movaps xmm9, xmm3
14207c925: 440f54c8 andps xmm9, xmm0
14207c929: 0f55dd andnps xmm3, xmm5
14207c92c: 0f284710 movaps xmm0, xmmword ptr [rdi + 0x10]
14207c930: 410f56d9 orps xmm3, xmm9
14207c934: 488b5550 mov rdx, qword ptr [rbp + 0x50]
14207c938: 8b4a08 mov ecx, dword ptr [rdx + 8]
14207c93b: 0f28ea movaps xmm5, xmm2
14207c93e: f20f10ec movsd xmm5, xmm4
14207c942: 0fc6d524 shufps xmm2, xmm5, 0x24
14207c946: f20f102a movsd xmm5, qword ptr [rdx]
14207c94a: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
14207c94e: 0fc6e149 shufps xmm4, xmm1, 0x49
14207c952: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
14207c956: 0fc6c802 shufps xmm1, xmm0, 2
14207c95a: 0fc6c898 shufps xmm1, xmm0, 0x98
14207c95e: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
14207c962: f20f116df0 movsd qword ptr [rbp - 0x10], xmm5
14207c967: 894df8 mov dword ptr [rbp - 8], ecx
14207c96a: 8945fc mov dword ptr [rbp - 4], eax
14207c96d: 488d45c0 lea rax, [rbp - 0x40]
14207c971: 4889442420 mov qword ptr [rsp + 0x20], rax
14207c976: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
14207c97a: 488b5578 mov rdx, qword ptr [rbp + 0x78]
14207c97e: 4c8d4518 lea r8, [rbp + 0x18]
14207c982: e8b9394fff call 0x141570340
14207c987: 4c8b9580000000 mov r10, qword ptr [rbp + 0x80]
14207c98e: 4c8b9d88000000 mov r11, qword ptr [rbp + 0x88]
14207c995: 488b9d90000000 mov rbx, qword ptr [rbp + 0x90]
14207c99c: 440fb6bdc7000000 movzx r15d, byte ptr [rbp + 0xc7]
14207c9a4: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
14207c9ab: 4c8b6558 mov r12, qword ptr [rbp + 0x58]
14207c9af: 4c8b6d60 mov r13, qword ptr [rbp + 0x60]
14207c9b3: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
14207c9ba: 4c8b8da0000000 mov r9, qword ptr [rbp + 0xa0]
14207c9c1: 4584ff test r15b, r15b
14207c9c4: 0f84e6f9ffff je 0x14207c3b0
14207c9ca: e961fbffff jmp 0x14207c530
14207c9cf: 488d0dfa6cf500 lea rcx, [rip + 0xf56cfa]
14207c9d6: e835c98500 call 0x1428d9310
14207c9db: 0f0b ud2 
14207c9dd: 488b7568 mov rsi, qword ptr [rbp + 0x68]
14207c9e1: 4889f1 mov rcx, rsi
14207c9e4: e82754c0fe call 0x140c81e10
14207c9e9: 4889f1 mov rcx, rsi
14207c9ec: 4883c901 or rcx, 1
14207c9f0: e81b54c0fe call 0x140c81e10
14207c9f5: 31c9 xor ecx, ecx
14207c9f7: b808000000 mov eax, 8
14207c9fc: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
14207ca01: 0f85db000000 jne 0x14207cae2
14207ca07: 48837d3800 cmp qword ptr [rbp + 0x38], 0
14207ca0c: 7448 je 0x14207ca56
14207ca0e: e81db7b9fe call 0x140c18130
14207ca13: 4889c1 mov rcx, rax
14207ca16: 488b00 mov rax, qword ptr [rax]
14207ca19: 4883f801 cmp rax, 1
14207ca1d: 0f858f000000 jne 0x14207cab2
14207ca23: 4883c108 add rcx, 8
14207ca27: 48833900 cmp qword ptr [rcx], 0
14207ca2b: 0f85a5000000 jne 0x14207cad6
14207ca31: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207ca38: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207ca3f: 4883c108 add rcx, 8
14207ca43: 488b5540 mov rdx, qword ptr [rbp + 0x40]
14207ca47: e8a4a6b9fe call 0x140c170f0
14207ca4c: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207ca53: 48ff00 inc qword ptr [rax]
14207ca56: 0f28b5d0000000 movaps xmm6, xmmword ptr [rbp + 0xd0]
14207ca5d: 0f28bde0000000 movaps xmm7, xmmword ptr [rbp + 0xe0]
14207ca64: 440f2885f0000000 movaps xmm8, xmmword ptr [rbp + 0xf0]
14207ca6c: 440f288d00010000 movaps xmm9, xmmword ptr [rbp + 0x100]
14207ca74: 440f289510010000 movaps xmm10, xmmword ptr [rbp + 0x110]
14207ca7c: 440f289d20010000 movaps xmm11, xmmword ptr [rbp + 0x120]
14207ca84: 4881c4b8010000 add rsp, 0x1b8
14207ca8b: 5b pop rbx
14207ca8c: 5f pop rdi
14207ca8d: 5e pop rsi
14207ca8e: 415c pop r12
14207ca90: 415d pop r13
14207ca92: 415e pop r14
14207ca94: 415f pop r15
14207ca96: 5d pop rbp
14207ca97: c3 ret 
14207ca98: 4885c0 test rax, rax
14207ca9b: 752d jne 0x14207caca
14207ca9d: 31d2 xor edx, edx
14207ca9f: e82cbd8700 call 0x1428f87d0
14207caa4: 4889c1 mov rcx, rax
14207caa7: 4885c0 test rax, rax
14207caaa: 0f852cf8ffff jne 0x14207c2dc
14207cab0: eb18 jmp 0x14207caca
14207cab2: 4885c0 test rax, rax
14207cab5: 7513 jne 0x14207caca
14207cab7: 31d2 xor edx, edx
14207cab9: e812bd8700 call 0x1428f87d0
14207cabe: 4889c1 mov rcx, rax
14207cac1: 4885c0 test rax, rax
14207cac4: 0f855dffffff jne 0x14207ca27
14207caca: 488d0dcfdfbb00 lea rcx, [rip + 0xbbdfcf]
14207cad1: e8fab58500 call 0x1428d80d0
14207cad6: 488d0d93e0bb00 lea rcx, [rip + 0xbbe093]
14207cadd: e86ec78500 call 0x1428d9250
14207cae2: 4889f1 mov rcx, rsi
14207cae5: 31d2 xor edx, edx
14207cae7: e874f48400 call 0x1428cbf60
14207caec: e916ffffff jmp 0x14207ca07
14207caf1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207cb00: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207cb05: 55 push rbp
14207cb06: 4157 push r15
14207cb08: 4156 push r14
14207cb0a: 4155 push r13
14207cb0c: 4154 push r12
14207cb0e: 56 push rsi
14207cb0f: 57 push rdi
14207cb10: 53 push rbx
14207cb11: 4881ec98000000 sub rsp, 0x98
14207cb18: 488daa80000000 lea rbp, [rdx + 0x80]
14207cb1f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207cb25: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207cb2b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207cb31: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14207cb37: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207cb3c: 660f7fb42480000000 movdqa xmmword ptr [rsp + 0x80], xmm6
14207cb45: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207cb4c: 48ff00 inc qword ptr [rax]
14207cb4f: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207cb57: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207cb5c: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207cb62: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207cb68: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207cb6e: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207cb74: 4881c498000000 add rsp, 0x98
14207cb7b: 5b pop rbx
14207cb7c: 5f pop rdi
14207cb7d: 5e pop rsi
14207cb7e: 415c pop r12
14207cb80: 415d pop r13
14207cb82: 415e pop r14
14207cb84: 415f pop r15
14207cb86: 5d pop rbp
14207cb87: c3 ret 
14207cb88: 0f1f840000000000 nop dword ptr [rax + rax]
14207cb90: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207cb95: 55 push rbp
14207cb96: 4157 push r15
14207cb98: 4156 push r14
14207cb9a: 4155 push r13
14207cb9c: 4154 push r12
14207cb9e: 56 push rsi
14207cb9f: 57 push rdi
14207cba0: 53 push rbx
14207cba1: 4881ec98000000 sub rsp, 0x98
14207cba8: 488daa80000000 lea rbp, [rdx + 0x80]
14207cbaf: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207cbb5: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207cbbb: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207cbc1: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14207cbc7: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207cbcc: 660f7fb42480000000 movdqa xmmword ptr [rsp + 0x80], xmm6
14207cbd5: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207cbdc: 48ff00 inc qword ptr [rax]
14207cbdf: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207cbe7: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207cbec: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207cbf2: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207cbf8: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207cbfe: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207cc04: 4881c498000000 add rsp, 0x98
14207cc0b: 5b pop rbx
14207cc0c: 5f pop rdi
14207cc0d: 5e pop rsi
14207cc0e: 415c pop r12
14207cc10: 415d pop r13
14207cc12: 415e pop r14
14207cc14: 415f pop r15
14207cc16: 5d pop rbp
14207cc17: c3 ret 
14207cc18: 0f1f840000000000 nop dword ptr [rax + rax]
14207cc20: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207cc25: 55 push rbp
14207cc26: 4157 push r15
14207cc28: 4156 push r14
14207cc2a: 4155 push r13
14207cc2c: 4154 push r12
14207cc2e: 56 push rsi
14207cc2f: 57 push rdi
14207cc30: 53 push rbx
14207cc31: 4881ec98000000 sub rsp, 0x98
14207cc38: 488daa80000000 lea rbp, [rdx + 0x80]
14207cc3f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207cc45: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207cc4b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207cc51: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14207cc57: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207cc5c: 660f7fb42480000000 movdqa xmmword ptr [rsp + 0x80], xmm6
14207cc65: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
14207cc69: e86246fcfd call 0x1400412d0
14207cc6e: 660f6fb42480000000 movdqa xmm6, xmmword ptr [rsp + 0x80]
14207cc77: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207cc7c: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207cc82: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207cc88: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207cc8e: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207cc94: 4881c498000000 add rsp, 0x98
14207cc9b: 5b pop rbx
14207cc9c: 5f pop rdi
14207cc9d: 5e pop rsi
14207cc9e: 415c pop r12
14207cca0: 415d pop r13
14207cca2: 415e pop r14
14207cca4: 415f pop r15
14207cca6: 5d pop rbp
14207cca7: c3 ret 
14207cca8: 0f1f840000000000 nop dword ptr [rax + rax]
14207ccb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207ccb5: 55 push rbp
14207ccb6: 4157 push r15
14207ccb8: 4156 push r14
14207ccba: 4155 push r13
14207ccbc: 4154 push r12
14207ccbe: 56 push rsi
14207ccbf: 57 push rdi
14207ccc0: 53 push rbx
14207ccc1: 4881ec98000000 sub rsp, 0x98
14207ccc8: 488daa80000000 lea rbp, [rdx + 0x80]
14207cccf: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207ccd5: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207ccdb: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207cce1: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14207cce7: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207ccec: 660f7fb42480000000 movdqa xmmword ptr [rsp + 0x80], xmm6
14207ccf5: 48837d3800 cmp qword ptr [rbp + 0x38], 0
14207ccfa: 7420 je 0x14207cd1c
14207ccfc: 488d4540 lea rax, [rbp + 0x40]
14207cd00: 48894510 mov qword ptr [rbp + 0x10], rax
14207cd04: 488d4510 lea rax, [rbp + 0x10]
14207cd08: 48894508 mov qword ptr [rbp + 8], rax
14207cd0c: 488d0d4566f500 lea rcx, [rip + 0xf56645]
14207cd13: 488d5508 lea rdx, [rbp + 8]
14207cd17: e8b4252bff call 0x14132f2d0
14207cd1c: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207cd24: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207cd29: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207cd2f: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207cd35: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207cd3b: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207cd41: 4881c498000000 add rsp, 0x98
14207cd48: 5b pop rbx
14207cd49: 5f pop rdi
14207cd4a: 5e pop rsi
14207cd4b: 415c pop r12
14207cd4d: 415d pop r13
14207cd4f: 415e pop r14
14207cd51: 415f pop r15
14207cd53: 5d pop rbp
14207cd54: c3 ret 
