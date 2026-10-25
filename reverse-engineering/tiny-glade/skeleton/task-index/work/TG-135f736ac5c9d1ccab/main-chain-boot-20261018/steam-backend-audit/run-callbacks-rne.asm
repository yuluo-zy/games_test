1400e6f90: push rbp
1400e6f91: push r15
1400e6f93: push r14
1400e6f95: push r13
1400e6f97: push r12
1400e6f99: push rsi
1400e6f9a: push rdi
1400e6f9b: push rbx
1400e6f9c: sub rsp, 0xd8
1400e6fa3: lea rbp, [rsp + 0x80]
1400e6fab: movdqa xmmword ptr [rbp + 0x40], xmm6
1400e6fb0: mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
1400e6fb8: mov rdi, rcx
1400e6fbb: call 0x140b85f20
1400e6fc0: mov esi, eax
1400e6fc2: mov ecx, eax
1400e6fc4: call 0x142923530
1400e6fc9: pxor xmm0, xmm0
1400e6fcd: movdqa xmmword ptr [rbp - 0x10], xmm0
1400e6fd2: mov qword ptr [rbp], 0
1400e6fda: mov r13, qword ptr [rdi]
1400e6fdd: lea rdi, [r13 + 0x80]
1400e6fe4: lea rax, [r13 + 0xa8]
1400e6feb: mov qword ptr [rbp - 0x40], rax
1400e6fef: lea rax, [r13 + 0xd8]
1400e6ff6: mov qword ptr [rbp - 0x38], rax
1400e6ffa: lea rax, [r13 + 0xb8]
1400e7001: mov qword ptr [rbp - 0x30], rax
1400e7005: lea r12, [rbp - 0x10]
1400e7009: mov r14b, 1
1400e700c: mov r15, qword ptr [rip + 0x354540d]
1400e7013: pcmpeqd xmm6, xmm6
1400e7017: mov qword ptr [rbp + 8], rdi
1400e701b: jmp 0x1400e7030
1400e701d: nop dword ptr [rax]
1400e7020: xor eax, eax
1400e7022: mov rdi, qword ptr [rbp + 8]
1400e7026: xchg byte ptr [rdi], al
1400e7028: cmp al, 2
1400e702a: je 0x1400e72a5
1400e7030: mov ecx, esi
1400e7032: mov rdx, r12
1400e7035: call 0x142923520
1400e703a: test al, al
1400e703c: je 0x1400e72b2
1400e7042: xor eax, eax
1400e7044: lock cmpxchg byte ptr [rdi], r14b
1400e7049: jne 0x1400e7275
1400e704f: mov rax, qword ptr [r15]
1400e7052: shl rax, 1
1400e7055: test rax, rax
1400e7058: jne 0x1400e728c
1400e705e: xor eax, eax
1400e7060: movzx ecx, byte ptr [r13 + 0x81]
1400e7068: test cl, cl
1400e706a: jne 0x1400e72ca
1400e7070: mov edi, dword ptr [rbp - 0xc]
1400e7073: cmp edi, 0x2bf
1400e7079: mov dword ptr [rbp + 0x30], eax
1400e707c: jne 0x1400e70c0
1400e707e: mov rbx, qword ptr [rbp - 8]
1400e7082: mov eax, dword ptr [rbx + 0xc]
1400e7085: test rax, rax
1400e7088: mov qword ptr [rbp + 0x18], rax
1400e708c: je 0x1400e7188
1400e7092: lea rdi, [rax*4]
1400e709a: movzx eax, byte ptr [rip + 0x354db68]
1400e70a1: mov edx, 4
1400e70a6: mov rcx, rdi
1400e70a9: call 0x140613c40
1400e70ae: test rax, rax
1400e70b1: je 0x1400e72fb
1400e70b7: mov r9d, dword ptr [rbx + 0xc]
1400e70bb: jmp 0x1400e7190
1400e70c0: cmp qword ptr [r13 + 0xa0], 0
1400e70c8: je 0x1400e723b
1400e70ce: mov rcx, qword ptr [rbp - 0x40]
1400e70d2: lea rdx, [rbp - 0xc]
1400e70d6: call 0x1405b0ed0
1400e70db: mov r8, qword ptr [r13 + 0x88]
1400e70e2: mov rcx, qword ptr [r13 + 0x90]
1400e70e9: mov rdx, rcx
1400e70ec: and rdx, rax
1400e70ef: shr rax, 0x39
1400e70f3: movd xmm0, eax
1400e70f7: punpcklbw xmm0, xmm0
1400e70fb: pshuflw xmm0, xmm0, 0
1400e7100: pshufd xmm0, xmm0, 0
1400e7105: lea rax, [r8 - 0x18]
1400e7109: xor r9d, r9d
1400e710c: movdqu xmm1, xmmword ptr [r8 + rdx]
1400e7112: movdqa xmm2, xmm1
1400e7116: pcmpeqb xmm2, xmm0
1400e711a: pmovmskb r10d, xmm2
1400e711f: test r10d, r10d
1400e7122: je 0x1400e7150
1400e7124: tzcnt r11d, r10d
1400e7129: add r11, rdx
1400e712c: and r11, rcx
1400e712f: neg r11
1400e7132: lea r11, [r11 + r11*2]
1400e7136: cmp edi, dword ptr [rax + r11*8]
1400e713a: je 0x1400e7172
1400e713c: lea r11d, [r10 - 1]
1400e7140: and r11w, r10w
1400e7144: mov r10d, r11d
1400e7147: jne 0x1400e7124
1400e7149: nop dword ptr [rax]
1400e7150: pcmpeqb xmm1, xmm6
1400e7154: pmovmskb r10d, xmm1
1400e7159: test r10d, r10d
1400e715c: jne 0x1400e723b
1400e7162: add rdx, r9
1400e7165: add rdx, 0x10
1400e7169: add r9, 0x10
1400e716d: and rdx, rcx
1400e7170: jmp 0x1400e710c
1400e7172: mov rdx, qword ptr [rbp - 8]
1400e7176: mov rcx, qword ptr [r8 + r11*8 - 0x10]
1400e717b: mov rax, qword ptr [r8 + r11*8 - 8]
1400e7180: call qword ptr [rax + 0x20]
1400e7183: jmp 0x1400e723b
1400e7188: xor r9d, r9d
1400e718b: mov eax, 4
1400e7190: mov byte ptr [rbp + 0x37], 0
1400e7194: mov rdx, qword ptr [rbx]
1400e7197: mov r8d, dword ptr [rbx + 8]
1400e719b: lea rcx, [rbp + 0x37]
1400e719f: mov qword ptr [rsp + 0x28], rcx
1400e71a4: mov dword ptr [rsp + 0x20], r8d
1400e71a9: mov ecx, esi
1400e71ab: mov qword ptr [rbp + 0x20], rax
1400e71af: mov r8, rax
1400e71b2: call 0x142923510
1400e71b7: test al, al
1400e71b9: je 0x1400e7220
1400e71bb: mov rax, qword ptr [rbx]
1400e71be: mov qword ptr [rbp - 0x48], rax
1400e71c2: mov rcx, qword ptr [rbp - 0x38]
1400e71c6: lea rdi, [rbp - 0x48]
1400e71ca: mov rdx, rdi
1400e71cd: call 0x1405b0d70
1400e71d2: lea rcx, [rbp - 0x28]
1400e71d6: mov rdx, qword ptr [rbp - 0x30]
1400e71da: mov r8, rax
1400e71dd: mov r9, rdi
1400e71e0: call 0x140053320
1400e71e5: mov rcx, qword ptr [rbp - 0x20]
1400e71e9: test rcx, rcx
1400e71ec: je 0x1400e7220
1400e71ee: mov rax, qword ptr [rbp - 0x18]
1400e71f2: movzx r8d, byte ptr [rbp + 0x37]
1400e71f7: mov qword ptr [rbp + 0x10], rcx
1400e71fb: mov rdx, qword ptr [rbp + 0x20]
1400e71ff: mov qword ptr [rbp + 0x28], rax
1400e7203: call qword ptr [rax + 0x18]
1400e7206: mov rax, qword ptr [rbp + 0x28]
1400e720a: mov rdx, qword ptr [rax + 8]
1400e720e: test rdx, rdx
1400e7211: mov rcx, qword ptr [rbp + 0x10]
1400e7215: je 0x1400e7220
1400e7217: mov r8, qword ptr [rax + 0x10]
1400e721b: call 0x140613c20
1400e7220: mov rdx, qword ptr [rbp + 0x18]
1400e7224: test edx, edx
1400e7226: je 0x1400e723b
1400e7228: shl rdx, 2
1400e722c: mov r8d, 4
1400e7232: mov rcx, qword ptr [rbp + 0x20]
1400e7236: call 0x140613c20
1400e723b: mov ecx, esi
1400e723d: call 0x142923500
1400e7242: cmp byte ptr [rbp + 0x30], 0
1400e7246: jne 0x1400e7020
1400e724c: mov rax, qword ptr [r15]
1400e724f: shl rax, 1
1400e7252: test rax, rax
1400e7255: je 0x1400e7020
1400e725b: call 0x1428d86b0
1400e7260: test al, al
1400e7262: jne 0x1400e7020
1400e7268: mov byte ptr [r13 + 0x81], 1
1400e7270: jmp 0x1400e7020
1400e7275: mov rcx, rdi
1400e7278: call 0x1428d8750
1400e727d: mov rax, qword ptr [r15]
1400e7280: shl rax, 1
1400e7283: test rax, rax
1400e7286: je 0x1400e705e
1400e728c: call 0x1428d86b0
1400e7291: xor al, 1
1400e7293: movzx ecx, byte ptr [r13 + 0x81]
1400e729b: test cl, cl
1400e729d: je 0x1400e7070
1400e72a3: jmp 0x1400e72ca
1400e72a5: mov rcx, rdi
1400e72a8: call 0x1428d8810
1400e72ad: jmp 0x1400e7030
1400e72b2: movaps xmm6, xmmword ptr [rbp + 0x40]
1400e72b6: add rsp, 0xd8
1400e72bd: pop rbx
1400e72be: pop rdi
1400e72bf: pop rsi
1400e72c0: pop r12
1400e72c2: pop r13
1400e72c4: pop r14
1400e72c6: pop r15
1400e72c8: pop rbp
1400e72c9: ret 
1400e72ca: mov qword ptr [rbp - 0x28], rdi
1400e72ce: mov byte ptr [rbp - 0x20], al
1400e72d1: lea rax, [rip + 0x2855258]
1400e72d8: mov qword ptr [rsp + 0x20], rax
1400e72dd: lea rcx, [rip + 0x285529c]
1400e72e4: lea r9, [rip + 0x2855275]
1400e72eb: lea r8, [rbp - 0x28]
1400e72ef: mov edx, 0x2b
1400e72f4: call 0x1428d9760
1400e72f9: jmp 0x1400e730f
1400e72fb: lea r8, [rip + 0x2855246]
1400e7302: mov ecx, 4
1400e7307: mov rdx, rdi
1400e730a: call 0x1428d8fe3
1400e730f: ud2 
1400e7311: nop word ptr cs:[rax + rax]
1400e7320: mov qword ptr [rsp + 0x10], rdx
1400e7325: push rbp
1400e7326: push r15
1400e7328: push r14
1400e732a: push r13
1400e732c: push r12
1400e732e: push rsi
1400e732f: push rdi
1400e7330: push rbx
1400e7331: sub rsp, 0x48
1400e7335: lea rbp, [rdx + 0x80]
1400e733c: movdqa xmmword ptr [rsp + 0x30], xmm6
1400e7342: mov rcx, qword ptr [rbp + 8]
1400e7346: mov edx, dword ptr [rbp + 0x30]
1400e7349: call 0x140043000
1400e734e: movaps xmm6, xmmword ptr [rsp + 0x30]
1400e7353: add rsp, 0x48
1400e7357: pop rbx
1400e7358: pop rdi
1400e7359: pop rsi
1400e735a: pop r12
1400e735c: pop r13
1400e735e: pop r14
1400e7360: pop r15
1400e7362: pop rbp
1400e7363: ret 
1400e7364: nop word ptr cs:[rax + rax]
1400e7370: mov qword ptr [rsp + 0x10], rdx
1400e7375: push rbp
1400e7376: push r15
1400e7378: push r14
1400e737a: push r13
1400e737c: push r12
1400e737e: push rsi
1400e737f: push rdi
1400e7380: push rbx
1400e7381: sub rsp, 0x48
1400e7385: lea rbp, [rdx + 0x80]
1400e738c: movdqa xmmword ptr [rsp + 0x30], xmm6
1400e7392: cmp dword ptr [rbp + 0x18], 0
1400e7396: je 0x1400e73af
1400e7398: mov rdx, qword ptr [rbp + 0x18]
1400e739c: shl rdx, 2
1400e73a0: mov r8d, 4
1400e73a6: mov rcx, qword ptr [rbp + 0x20]
1400e73aa: call 0x140613c20
1400e73af: movdqa xmm6, xmmword ptr [rsp + 0x30]
1400e73b5: add rsp, 0x48
1400e73b9: pop rbx
1400e73ba: pop rdi
1400e73bb: pop rsi
1400e73bc: pop r12
1400e73be: pop r13
1400e73c0: pop r14
1400e73c2: pop r15
1400e73c4: pop rbp
1400e73c5: ret 
1400e73c6: nop word ptr cs:[rax + rax]
1400e73d0: mov qword ptr [rsp + 0x10], rdx
1400e73d5: push rbp
1400e73d6: push r15
1400e73d8: push r14
1400e73da: push r13
1400e73dc: push r12
1400e73de: push rsi
1400e73df: push rdi
1400e73e0: push rbx
1400e73e1: sub rsp, 0x48
1400e73e5: lea rbp, [rdx + 0x80]
1400e73ec: movdqa xmmword ptr [rsp + 0x30], xmm6
1400e73f2: mov rax, qword ptr [rbp + 0x28]
1400e73f6: mov rdx, qword ptr [rax + 8]
1400e73fa: test rdx, rdx
1400e73fd: je 0x1400e7410
1400e73ff: mov rax, qword ptr [rbp + 0x28]
1400e7403: mov r8, qword ptr [rax + 0x10]
1400e7407: mov rcx, qword ptr [rbp + 0x10]
1400e740b: call 0x140613c20
1400e7410: movdqa xmm6, xmmword ptr [rsp + 0x30]
1400e7416: add rsp, 0x48
1400e741a: pop rbx
1400e741b: pop rdi
1400e741c: pop rsi
1400e741d: pop r12
1400e741f: pop r13
1400e7421: pop r14
1400e7423: pop r15
1400e7425: pop rbp
1400e7426: ret 
1400e7427: nop word ptr [rax + rax]
1400e7430: mov qword ptr [rsp + 0x10], rdx
1400e7435: push rbp
1400e7436: push r15
1400e7438: push r14
1400e743a: push r13
1400e743c: push r12
1400e743e: push rsi
1400e743f: push rdi
1400e7440: push rbx
1400e7441: sub rsp, 0x48
1400e7445: lea rbp, [rdx + 0x80]
1400e744c: movdqa xmmword ptr [rsp + 0x30], xmm6
1400e7452: lea rcx, [rbp - 0x28]
1400e7456: call 0x1405d2870
1400e745b: movaps xmm6, xmmword ptr [rsp + 0x30]
1400e7460: add rsp, 0x48
1400e7464: pop rbx
1400e7465: pop rdi
1400e7466: pop rsi
1400e7467: pop r12
1400e7469: pop r13
1400e746b: pop r14
1400e746d: pop r15
1400e746f: pop rbp
1400e7470: ret 
