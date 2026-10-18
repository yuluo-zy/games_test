1404f6e50: 55 push rbp
1404f6e51: 4157 push r15
1404f6e53: 4156 push r14
1404f6e55: 56 push rsi
1404f6e56: 57 push rdi
1404f6e57: 53 push rbx
1404f6e58: 4881ece8010000 sub rsp, 0x1e8
1404f6e5f: 488dac2480000000 lea rbp, [rsp + 0x80]
1404f6e67: 48c78560010000feffffff mov qword ptr [rbp + 0x160], 0xfffffffffffffffe
1404f6e72: 4889d7 mov rdi, rdx
1404f6e75: 4889ce mov rsi, rcx
1404f6e78: 488b9dc0010000 mov rbx, qword ptr [rbp + 0x1c0]
1404f6e7f: 4c89c1 mov rcx, r8
1404f6e82: 4c89ca mov rdx, r9
1404f6e85: e8b6877100 call 0x140c0f640
1404f6e8a: 48898550010000 mov qword ptr [rbp + 0x150], rax
1404f6e91: 4883bfc800000000 cmp qword ptr [rdi + 0xc8], 0
1404f6e99: 0f84a3020000 je 0x1404f7142
1404f6e9f: 488d8fd0000000 lea rcx, [rdi + 0xd0]
1404f6ea6: 4c8dbd50010000 lea r15, [rbp + 0x150]
1404f6ead: 4c89fa mov rdx, r15
1404f6eb0: e88ba70b00 call 0x1405b1640
1404f6eb5: 4c8bb7b0000000 mov r14, qword ptr [rdi + 0xb0]
1404f6ebc: 488b8fb8000000 mov rcx, qword ptr [rdi + 0xb8]
1404f6ec3: 4889ca mov rdx, rcx
1404f6ec6: 4821c2 and rdx, rax
1404f6ec9: 48c1e839 shr rax, 0x39
1404f6ecd: 660f6ec0 movd xmm0, eax
1404f6ed1: 660f60c0 punpcklbw xmm0, xmm0
1404f6ed5: f20f70c000 pshuflw xmm0, xmm0, 0
1404f6eda: 660f70c000 pshufd xmm0, xmm0, 0
1404f6edf: 4d8d4698 lea r8, [r14 - 0x68]
1404f6ee3: 4531c9 xor r9d, r9d
1404f6ee6: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
1404f6eed: 660f76c9 pcmpeqd xmm1, xmm1
1404f6ef1: f3410f6f1416 movdqu xmm2, xmmword ptr [r14 + rdx]
1404f6ef7: 660f6fda movdqa xmm3, xmm2
1404f6efb: 660f74d8 pcmpeqb xmm3, xmm0
1404f6eff: 66440fd7d3 pmovmskb r10d, xmm3
1404f6f04: 4585d2 test r10d, r10d
1404f6f07: 7427 je 0x1404f6f30
1404f6f09: f3450fbcda tzcnt r11d, r10d
1404f6f0e: 4901d3 add r11, rdx
1404f6f11: 4921cb and r11, rcx
1404f6f14: 49f7db neg r11
1404f6f17: 4d6bdb68 imul r11, r11, 0x68
1404f6f1b: 4b3b0418 cmp rax, qword ptr [r8 + r11]
1404f6f1f: 7431 je 0x1404f6f52
1404f6f21: 458d5aff lea r11d, [r10 - 1]
1404f6f25: 664521d3 and r11w, r10w
1404f6f29: 4589da mov r10d, r11d
1404f6f2c: 75db jne 0x1404f6f09
1404f6f2e: 6690 nop 
1404f6f30: 660f74d1 pcmpeqb xmm2, xmm1
1404f6f34: 66440fd7d2 pmovmskb r10d, xmm2
1404f6f39: 4585d2 test r10d, r10d
1404f6f3c: 0f8500020000 jne 0x1404f7142
1404f6f42: 4c01ca add rdx, r9
1404f6f45: 4883c210 add rdx, 0x10
1404f6f49: 4983c110 add r9, 0x10
1404f6f4d: 4821ca and rdx, rcx
1404f6f50: eb9f jmp 0x1404f6ef1
1404f6f52: 43837c1ea002 cmp dword ptr [r14 + r11 - 0x60], 2
1404f6f58: 0f858f030000 jne 0x1404f72ed
1404f6f5e: 4d01de add r14, r11
1404f6f61: 410fb64efe movzx ecx, byte ptr [r14 - 2]
1404f6f66: 80f907 cmp cl, 7
1404f6f69: 0f84ae030000 je 0x1404f731d
1404f6f6f: 418b56f4 mov edx, dword ptr [r14 - 0xc]
1404f6f73: 4d8b46ec mov r8, qword ptr [r14 - 0x14]
1404f6f77: 448b4b14 mov r9d, dword ptr [rbx + 0x14]
1404f6f7b: 4931d1 xor r9, rdx
1404f6f7e: 4c33430c xor r8, qword ptr [rbx + 0xc]
1404f6f82: 4d09c8 or r8, r9
1404f6f85: 0f85ac000000 jne 0x1404f7037
1404f6f8b: 3a4b1e cmp cl, byte ptr [rbx + 0x1e]
1404f6f8e: 0f8502040000 jne 0x1404f7396
1404f6f94: 418b46e0 mov eax, dword ptr [r14 - 0x20]
1404f6f98: 3b03 cmp eax, dword ptr [rbx]
1404f6f9a: 0f85f6030000 jne 0x1404f7396
1404f6fa0: 418b46e4 mov eax, dword ptr [r14 - 0x1c]
1404f6fa4: 3b4304 cmp eax, dword ptr [rbx + 4]
1404f6fa7: 0f85e9030000 jne 0x1404f7396
1404f6fad: 418b46e8 mov eax, dword ptr [r14 - 0x18]
1404f6fb1: 3b4308 cmp eax, dword ptr [rbx + 8]
1404f6fb4: 0f85dc030000 jne 0x1404f7396
1404f6fba: 410fb646fc movzx eax, byte ptr [r14 - 4]
1404f6fbf: 3a431c cmp al, byte ptr [rbx + 0x1c]
1404f6fc2: 0f85ce030000 jne 0x1404f7396
1404f6fc8: 418b46f8 mov eax, dword ptr [r14 - 8]
1404f6fcc: 3b4318 cmp eax, dword ptr [rbx + 0x18]
1404f6fcf: 0f85c1030000 jne 0x1404f7396
1404f6fd5: 410fb646fd movzx eax, byte ptr [r14 - 3]
1404f6fda: 3a431d cmp al, byte ptr [rbx + 0x1d]
1404f6fdd: 0f85b3030000 jne 0x1404f7396
1404f6fe3: 450fb64ea4 movzx r9d, byte ptr [r14 - 0x5c]
1404f6fe8: 498d56c8 lea rdx, [r14 - 0x38]
1404f6fec: 488d4d14 lea rcx, [rbp + 0x14]
1404f6ff0: 4989f8 mov r8, rdi
1404f6ff3: e8a8f06d00 call 0x140bd60a0
1404f6ff8: 8b4514 mov eax, dword ptr [rbp + 0x14]
1404f6ffb: 41c746a000000000 mov dword ptr [r14 - 0x60], 0
1404f7003: 418946a4 mov dword ptr [r14 - 0x5c], eax
1404f7007: 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
1404f700b: 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
1404f700f: 410f114eb8 movups xmmword ptr [r14 - 0x48], xmm1
1404f7014: 410f1146a8 movups xmmword ptr [r14 - 0x58], xmm0
1404f7019: 0f104514 movups xmm0, xmmword ptr [rbp + 0x14]
1404f701d: 0f104d24 movups xmm1, xmmword ptr [rbp + 0x24]
1404f7021: 0f1106 movups xmmword ptr [rsi], xmm0
1404f7024: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1404f7028: 8b4534 mov eax, dword ptr [rbp + 0x34]
1404f702b: 894620 mov dword ptr [rsi + 0x20], eax
1404f702e: c6462401 mov byte ptr [rsi + 0x24], 1
1404f7032: e9a3020000 jmp 0x1404f72da
1404f7037: 488b0d6a531303 mov rcx, qword ptr [rip + 0x313536a]
1404f703e: 488b09 mov rcx, qword ptr [rcx]
1404f7041: 4883f905 cmp rcx, 5
1404f7045: 0f82f7000000 jb 0x1404f7142
1404f704b: 4983c6ec add r14, -0x14
1404f704f: 488d430c lea rax, [rbx + 0xc]
1404f7053: 4c897d38 mov qword ptr [rbp + 0x38], r15
1404f7057: 488d0d12877100 lea rcx, [rip + 0x718712]
1404f705e: 48894d40 mov qword ptr [rbp + 0x40], rcx
1404f7062: 4c897548 mov qword ptr [rbp + 0x48], r14
1404f7066: 488d0d93fd0b00 lea rcx, [rip + 0xbfd93]
1404f706d: 48894d50 mov qword ptr [rbp + 0x50], rcx
1404f7071: 48894558 mov qword ptr [rbp + 0x58], rax
1404f7075: 48894d60 mov qword ptr [rbp + 0x60], rcx
1404f7079: 488d0d105e4f02 lea rcx, [rip + 0x24f5e10]
1404f7080: e8fb6c1800 call 0x14067dd80
1404f7085: 0f1000 movups xmm0, xmmword ptr [rax]
1404f7088: 8b4010 mov eax, dword ptr [rax + 0x10]
1404f708b: 48c785c800000005000000 mov qword ptr [rbp + 0xc8], 5
1404f7096: 488d0d0b5e4f02 lea rcx, [rip + 0x24f5e0b]
1404f709d: 48898dd0000000 mov qword ptr [rbp + 0xd0], rcx
1404f70a4: 48c785d800000016000000 mov qword ptr [rbp + 0xd8], 0x16
1404f70af: 488d15aa5d4f02 lea rdx, [rip + 0x24f5daa]
1404f70b6: 488995e8000000 mov qword ptr [rbp + 0xe8], rdx
1404f70bd: 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
1404f70c8: 488d5538 lea rdx, [rbp + 0x38]
1404f70cc: 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
1404f70d3: 48c7850001000003000000 mov qword ptr [rbp + 0x100], 3
1404f70de: 48c7850801000000000000 mov qword ptr [rbp + 0x108], 0
1404f70e9: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
1404f70f4: 48898da0000000 mov qword ptr [rbp + 0xa0], rcx
1404f70fb: 48c785a800000016000000 mov qword ptr [rbp + 0xa8], 0x16
1404f7106: 48c785b000000000000000 mov qword ptr [rbp + 0xb0], 0
1404f7111: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1404f7118: c785e000000001000000 mov dword ptr [rbp + 0xe0], 1
1404f7122: 8985e4000000 mov dword ptr [rbp + 0xe4], eax
1404f7128: 488d8d18010000 lea rcx, [rbp + 0x118]
1404f712f: 488d9598000000 lea rdx, [rbp + 0x98]
1404f7136: e8f5318500 call 0x140d4a330
1404f713b: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
1404f7142: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1404f7149: 4c8b48f8 mov r9, qword ptr [rax - 8]
1404f714d: 48c745b800000000 mov qword ptr [rbp - 0x48], 0
1404f7155: f30f6f03 movdqu xmm0, xmmword ptr [rbx]
1404f7159: f30f6f4b10 movdqu xmm1, xmmword ptr [rbx + 0x10]
1404f715e: 660f7f4de0 movdqa xmmword ptr [rbp - 0x20], xmm1
1404f7163: 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
1404f7168: 488d4db8 lea rcx, [rbp - 0x48]
1404f716c: 48894c2428 mov qword ptr [rsp + 0x28], rcx
1404f7171: 488d4dd0 lea rcx, [rbp - 0x30]
1404f7175: 48894c2420 mov qword ptr [rsp + 0x20], rcx
1404f717a: 4c8db598000000 lea r14, [rbp + 0x98]
1404f7181: 4c89f1 mov rcx, r14
1404f7184: 4989c0 mov r8, rax
1404f7187: e864340c00 call 0x1405ba5f0
1404f718c: 488d4d38 lea rcx, [rbp + 0x38]
1404f7190: 4c89f2 mov rdx, r14
1404f7193: 4989d8 mov r8, rbx
1404f7196: e825abe1ff call 0x140311cc0
1404f719b: 0fb6456e movzx eax, byte ptr [rbp + 0x6e]
1404f719f: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
1404f71a3: 3c07 cmp al, 7
1404f71a5: 750c jne 0x1404f71b3
1404f71a7: 48890e mov qword ptr [rsi], rcx
1404f71aa: c6462402 mov byte ptr [rsi + 0x24], 2
1404f71ae: e927010000 jmp 0x1404f72da
1404f71b3: 0f104540 movups xmm0, xmmword ptr [rbp + 0x40]
1404f71b7: 0f104d50 movups xmm1, xmmword ptr [rbp + 0x50]
1404f71bb: f30f6f555e movdqu xmm2, xmmword ptr [rbp + 0x5e]
1404f71c0: f30f7f953e010000 movdqu xmmword ptr [rbp + 0x13e], xmm2
1404f71c8: 0f118d30010000 movups xmmword ptr [rbp + 0x130], xmm1
1404f71cf: 0f118520010000 movups xmmword ptr [rbp + 0x120], xmm0
1404f71d6: 0fb6556f movzx edx, byte ptr [rbp + 0x6f]
1404f71da: 48898d18010000 mov qword ptr [rbp + 0x118], rcx
1404f71e1: 88854e010000 mov byte ptr [rbp + 0x14e], al
1404f71e7: 88954f010000 mov byte ptr [rbp + 0x14f], dl
1404f71ed: c6855f01000001 mov byte ptr [rbp + 0x15f], 1
1404f71f4: 488d4df0 lea rcx, [rbp - 0x10]
1404f71f8: 488d9518010000 lea rdx, [rbp + 0x118]
1404f71ff: 4989f8 mov r8, rdi
1404f7202: 4531c9 xor r9d, r9d
1404f7205: e896ee6d00 call 0x140bd60a0
1404f720a: 4881c7b0000000 add rdi, 0xb0
1404f7211: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
1404f7218: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
1404f721f: 0f108518010000 movups xmm0, xmmword ptr [rbp + 0x118]
1404f7226: 0f108d28010000 movups xmm1, xmmword ptr [rbp + 0x128]
1404f722d: f30f6f9538010000 movdqu xmm2, xmmword ptr [rbp + 0x138]
1404f7235: f30f7f95e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm2
1404f723d: 0f118dd0000000 movups xmmword ptr [rbp + 0xd0], xmm1
1404f7244: 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
1404f724b: 8b45f0 mov eax, dword ptr [rbp - 0x10]
1404f724e: 0f1045f4 movups xmm0, xmmword ptr [rbp - 0xc]
1404f7252: 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
1404f7259: 0f104504 movups xmm0, xmmword ptr [rbp + 4]
1404f725d: 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
1404f7264: 89859c000000 mov dword ptr [rbp + 0x9c], eax
1404f726a: c7859800000000000000 mov dword ptr [rbp + 0x98], 0
1404f7274: 4c8b8550010000 mov r8, qword ptr [rbp + 0x150]
1404f727b: c6855f01000000 mov byte ptr [rbp + 0x15f], 0
1404f7282: 488d4d38 lea rcx, [rbp + 0x38]
1404f7286: 4c8d8d98000000 lea r9, [rbp + 0x98]
1404f728d: 4889fa mov rdx, rdi
1404f7290: e8bb1cddff call 0x1402c8f50
1404f7295: 837d3803 cmp dword ptr [rbp + 0x38], 3
1404f7299: 7422 je 0x1404f72bd
1404f729b: 80bd9600000007 cmp byte ptr [rbp + 0x96], 7
1404f72a2: 7409 je 0x1404f72ad
1404f72a4: 488d4d60 lea rcx, [rbp + 0x60]
1404f72a8: e8d348b4ff call 0x14003bb80
1404f72ad: c6855f01000000 mov byte ptr [rbp + 0x15f], 0
1404f72b4: 488d4d60 lea rcx, [rbp + 0x60]
1404f72b8: e8f3b16e00 call 0x140be24b0
1404f72bd: 8b4510 mov eax, dword ptr [rbp + 0x10]
1404f72c0: 894620 mov dword ptr [rsi + 0x20], eax
1404f72c3: f30f6f45f0 movdqu xmm0, xmmword ptr [rbp - 0x10]
1404f72c8: f30f6f4d00 movdqu xmm1, xmmword ptr [rbp]
1404f72cd: f30f7f4e10 movdqu xmmword ptr [rsi + 0x10], xmm1
1404f72d2: f30f7f06 movdqu xmmword ptr [rsi], xmm0
1404f72d6: c6462400 mov byte ptr [rsi + 0x24], 0
1404f72da: 4889f0 mov rax, rsi
1404f72dd: 4881c4e8010000 add rsp, 0x1e8
1404f72e4: 5b pop rbx
1404f72e5: 5f pop rdi
1404f72e6: 5e pop rsi
1404f72e7: 415e pop r14
1404f72e9: 415f pop r15
1404f72eb: 5d pop rbp
1404f72ec: c3 ret 
1404f72ed: 4c89bd18010000 mov qword ptr [rbp + 0x118], r15
1404f72f4: 488d0575847100 lea rax, [rip + 0x718475]
1404f72fb: 48898520010000 mov qword ptr [rbp + 0x120], rax
1404f7302: 488d05075b4f02 lea rax, [rip + 0x24f5b07]
1404f7309: 48898598000000 mov qword ptr [rbp + 0x98], rax
1404f7310: 48c785a000000001000000 mov qword ptr [rbp + 0xa0], 1
1404f731b: eb2e jmp 0x1404f734b
1404f731d: 4c89bd18010000 mov qword ptr [rbp + 0x118], r15
1404f7324: 488d0545847100 lea rax, [rip + 0x718445]
1404f732b: 48898520010000 mov qword ptr [rbp + 0x120], rax
1404f7332: 488d05c75b4f02 lea rax, [rip + 0x24f5bc7]
1404f7339: 48898598000000 mov qword ptr [rbp + 0x98], rax
1404f7340: 48c785a000000002000000 mov qword ptr [rbp + 0xa0], 2
1404f734b: 488d8518010000 lea rax, [rbp + 0x118]
1404f7352: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
1404f7359: 48c785b000000001000000 mov qword ptr [rbp + 0xb0], 1
1404f7364: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
1404f736f: 488d7d38 lea rdi, [rbp + 0x38]
1404f7373: 488d9598000000 lea rdx, [rbp + 0x98]
1404f737a: 4889f9 mov rcx, rdi
1404f737d: e80e2c8800 call 0x140d79f90
1404f7382: 4889f9 mov rcx, rdi
1404f7385: e896b73a02 call 0x1428a2b20
1404f738a: 488906 mov qword ptr [rsi], rax
1404f738d: c6462402 mov byte ptr [rsi + 0x24], 2
1404f7391: e944ffffff jmp 0x1404f72da
1404f7396: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
1404f73a1: 4983c6e0 add r14, -0x20
1404f73a5: 488d05145b4f02 lea rax, [rip + 0x24f5b14]
1404f73ac: 4889442420 mov qword ptr [rsp + 0x20], rax
1404f73b1: 4c8d8d98000000 lea r9, [rbp + 0x98]
1404f73b8: 31c9 xor ecx, ecx
1404f73ba: 4c89f2 mov rdx, r14
1404f73bd: 4989d8 mov r8, rbx
1404f73c0: e8c5253b02 call 0x1428a998a
1404f73c5: cc int3 
1404f73c6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1404f73d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1404f73d5: 55 push rbp
1404f73d6: 4157 push r15
1404f73d8: 4156 push r14
1404f73da: 56 push rsi
1404f73db: 57 push rdi
1404f73dc: 53 push rbx
1404f73dd: 4883ec38 sub rsp, 0x38
1404f73e1: 488daa80000000 lea rbp, [rdx + 0x80]
1404f73e8: c6855f01000000 mov byte ptr [rbp + 0x15f], 0
1404f73ef: 488d4d60 lea rcx, [rbp + 0x60]
1404f73f3: e8b8b06e00 call 0x140be24b0
1404f73f8: c6855f01000000 mov byte ptr [rbp + 0x15f], 0
1404f73ff: 4883c438 add rsp, 0x38
1404f7403: 5b pop rbx
1404f7404: 5f pop rdi
1404f7405: 5e pop rsi
1404f7406: 415e pop r14
1404f7408: 415f pop r15
1404f740a: 5d pop rbp
1404f740b: c3 ret 
1404f740c: 0f1f4000 nop dword ptr [rax]
1404f7410: 4889542410 mov qword ptr [rsp + 0x10], rdx
1404f7415: 55 push rbp
1404f7416: 4157 push r15
1404f7418: 4156 push r14
1404f741a: 56 push rsi
1404f741b: 57 push rdi
1404f741c: 53 push rbx
1404f741d: 4883ec38 sub rsp, 0x38
1404f7421: 488daa80000000 lea rbp, [rdx + 0x80]
1404f7428: 80bd5f01000000 cmp byte ptr [rbp + 0x15f], 0
1404f742f: 750d jne 0x1404f743e
1404f7431: 4883c438 add rsp, 0x38
1404f7435: 5b pop rbx
1404f7436: 5f pop rdi
1404f7437: 5e pop rsi
1404f7438: 415e pop r14
1404f743a: 415f pop r15
1404f743c: 5d pop rbp
1404f743d: c3 ret 
1404f743e: 488d8d18010000 lea rcx, [rbp + 0x118]
1404f7445: e896710f00 call 0x1405ee5e0
1404f744a: ebe5 jmp 0x1404f7431
