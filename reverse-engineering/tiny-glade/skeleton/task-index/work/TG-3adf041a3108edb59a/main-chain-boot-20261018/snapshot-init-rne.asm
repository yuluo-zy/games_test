1417a9c50: push rbp
1417a9c51: push r15
1417a9c53: push r14
1417a9c55: push r13
1417a9c57: push r12
1417a9c59: push rsi
1417a9c5a: push rdi
1417a9c5b: push rbx
1417a9c5c: sub rsp, 0x2a8
1417a9c63: lea rbp, [rsp + 0x80]
1417a9c6b: mov qword ptr [rbp + 0x220], 0xfffffffffffffffe
1417a9c76: mov r15, r8
1417a9c79: mov r12, rdx
1417a9c7c: mov rsi, rcx
1417a9c7f: mov rax, qword ptr [rip + 0x1e82722]
1417a9c86: mov rax, qword ptr [rax]
1417a9c89: cmp rax, 3
1417a9c8d: mov qword ptr [rbp + 0x208], rcx
1417a9c94: jbe 0x1417a9daa
1417a9c9a: mov byte ptr [rbp + 0x21f], 1
1417a9ca1: mov byte ptr [rbp + 0x21e], 1
1417a9ca8: mov byte ptr [rbp + 0x21d], 1
1417a9caf: mov byte ptr [rbp + 0x21c], 1
1417a9cb6: mov byte ptr [rbp + 0x21b], 1
1417a9cbd: mov byte ptr [rbp + 0x21a], 1
1417a9cc4: mov byte ptr [rbp + 0x219], 1
1417a9ccb: mov byte ptr [rbp + 0x218], 1
1417a9cd2: mov byte ptr [rbp + 0x217], 1
1417a9cd9: lea rcx, [rip + 0x1532bb8]
1417a9ce0: call 0x14067dd80
1417a9ce5: movdqu xmm0, xmmword ptr [rax]
1417a9ce9: mov eax, dword ptr [rax + 0x10]
1417a9cec: mov qword ptr [rbp - 0x30], 4
1417a9cf4: lea rcx, [rip + 0x15329c5]
1417a9cfb: mov qword ptr [rbp - 0x28], rcx
1417a9cff: mov qword ptr [rbp - 0x20], 0x25
1417a9d07: lea rdx, [rip + 0x1532b7a]
1417a9d0e: mov qword ptr [rbp - 0x10], rdx
1417a9d12: mov qword ptr [rbp - 8], 1
1417a9d1a: mov qword ptr [rbp], 8
1417a9d22: mov qword ptr [rbp - 0x60], 0
1417a9d2a: xorps xmm1, xmm1
1417a9d2d: movups xmmword ptr [rbp + 8], xmm1
1417a9d31: mov qword ptr [rbp - 0x58], rcx
1417a9d35: mov qword ptr [rbp - 0x50], 0x25
1417a9d3d: mov qword ptr [rbp - 0x48], 0
1417a9d45: movdqu xmmword ptr [rbp - 0x40], xmm0
1417a9d4a: mov dword ptr [rbp - 0x18], 1
1417a9d51: mov dword ptr [rbp - 0x14], eax
1417a9d54: mov byte ptr [rbp + 0x21f], 1
1417a9d5b: mov byte ptr [rbp + 0x21e], 1
1417a9d62: mov byte ptr [rbp + 0x21d], 1
1417a9d69: mov byte ptr [rbp + 0x21c], 1
1417a9d70: mov byte ptr [rbp + 0x21b], 1
1417a9d77: mov byte ptr [rbp + 0x21a], 1
1417a9d7e: mov byte ptr [rbp + 0x219], 1
1417a9d85: mov byte ptr [rbp + 0x218], 1
1417a9d8c: mov byte ptr [rbp + 0x217], 1
1417a9d93: lea rcx, [rbp + 0x180]
1417a9d9a: lea rdx, [rbp - 0x60]
1417a9d9e: call 0x140d4a330
1417a9da3: mov rsi, qword ptr [rbp + 0x208]
1417a9daa: lea rcx, [rbp - 0x60]
1417a9dae: mov r8d, 0xc0
1417a9db4: mov rdx, r15
1417a9db7: call 0x142923c00
1417a9dbc: mov rax, qword ptr [r15 + 0xc0]
1417a9dc3: mov qword ptr [rbp + 0x80], rax
1417a9dca: mov rax, qword ptr [r15 + 0xd0]
1417a9dd1: mov qword ptr [rbp + 0x90], rax
1417a9dd8: mov eax, dword ptr [r15 + 0xdc]
1417a9ddf: mov dword ptr [rbp + 0x120], eax
1417a9de5: mov rax, qword ptr [r15 + 0xe0]
1417a9dec: mov qword ptr [rbp + 0x88], rax
1417a9df3: mov rax, qword ptr [r15 + 0xf0]
1417a9dfa: mov qword ptr [rbp + 0x98], rax
1417a9e01: mov eax, dword ptr [r15 + 0xfc]
1417a9e08: mov dword ptr [rbp + 0x124], eax
1417a9e0e: mov rax, qword ptr [r15 + 0x100]
1417a9e15: mov qword ptr [rbp + 0x1e0], rax
1417a9e1c: mov rax, qword ptr [r15 + 0x110]
1417a9e23: mov qword ptr [rbp + 0xa0], rax
1417a9e2a: mov eax, dword ptr [r15 + 0x11c]
1417a9e31: mov dword ptr [rbp + 0x128], eax
1417a9e37: mov rax, qword ptr [r15 + 0x120]
1417a9e3e: mov qword ptr [rbp + 0xc0], rax
1417a9e45: mov rax, qword ptr [r15 + 0x130]
1417a9e4c: mov qword ptr [rbp + 0xc8], rax
1417a9e53: mov rax, qword ptr [rsi + 0x38]
1417a9e57: mov qword ptr [rbp + 0x190], rax
1417a9e5e: mov eax, dword ptr [r15 + 0x13c]
1417a9e65: mov dword ptr [rbp + 0x134], eax
1417a9e6b: mov rax, qword ptr [r15 + 0x240]
1417a9e72: mov qword ptr [rbp + 0xd0], rax
1417a9e79: mov rax, qword ptr [r15 + 0x250]
1417a9e80: mov qword ptr [rbp + 0xd8], rax
1417a9e87: mov eax, dword ptr [r15 + 0x25c]
1417a9e8e: mov dword ptr [rbp + 0x138], eax
1417a9e94: mov rbx, qword ptr [r15 + 0x140]
1417a9e9b: mov rax, qword ptr [r15 + 0x150]
1417a9ea2: mov qword ptr [rbp + 0x150], rax
1417a9ea9: mov eax, dword ptr [r15 + 0x15c]
1417a9eb0: mov dword ptr [rbp + 0x1d0], eax
1417a9eb6: mov rax, qword ptr [r15 + 0x160]
1417a9ebd: mov qword ptr [rbp + 0x160], rax
1417a9ec4: mov rax, qword ptr [r15 + 0x170]
1417a9ecb: mov qword ptr [rbp + 0xb8], rax
1417a9ed2: mov eax, dword ptr [r15 + 0x17c]
1417a9ed9: mov dword ptr [rbp + 0x130], eax
1417a9edf: mov r14, qword ptr [r15 + 0x180]
1417a9ee6: mov rax, qword ptr [r15 + 0x190]
1417a9eed: mov qword ptr [rbp + 0x1f8], rax
1417a9ef4: mov eax, dword ptr [r15 + 0x19c]
1417a9efb: mov dword ptr [rbp + 0x200], eax
1417a9f01: mov rdi, qword ptr [r15 + 0x1a0]
1417a9f08: mov rax, qword ptr [r15 + 0x1b0]
1417a9f0f: mov qword ptr [rbp + 0x60], rax
1417a9f13: mov eax, dword ptr [r15 + 0x1bc]
1417a9f1a: mov dword ptr [rbp + 0x114], eax
1417a9f20: mov rax, qword ptr [r15 + 0x1c0]
1417a9f27: mov qword ptr [rbp + 0x178], rax
1417a9f2e: mov rax, qword ptr [r15 + 0x1d0]
1417a9f35: mov qword ptr [rbp + 0x68], rax
1417a9f39: mov eax, dword ptr [r15 + 0x1dc]
1417a9f40: mov dword ptr [rbp + 0x118], eax
1417a9f46: mov rax, qword ptr [r15 + 0x1e0]
1417a9f4d: mov qword ptr [rbp + 0x1f0], rax
1417a9f54: mov rax, qword ptr [r15 + 0x1f0]
1417a9f5b: mov qword ptr [rbp + 0x170], rax
1417a9f62: mov eax, dword ptr [r15 + 0x1fc]
1417a9f69: mov dword ptr [rbp + 0x140], eax
1417a9f6f: mov rax, qword ptr [r15 + 0x200]
1417a9f76: mov qword ptr [rbp + 0x148], rax
1417a9f7d: mov rax, qword ptr [r15 + 0x210]
1417a9f84: mov qword ptr [rbp + 0x70], rax
1417a9f88: mov eax, dword ptr [r15 + 0x21c]
1417a9f8f: mov dword ptr [rbp + 0x11c], eax
1417a9f95: mov rax, qword ptr [r15 + 0x220]
1417a9f9c: mov qword ptr [rbp + 0xe0], rax
1417a9fa3: mov rax, qword ptr [r15 + 0x230]
1417a9faa: mov qword ptr [rbp + 0xe8], rax
1417a9fb1: mov eax, dword ptr [r15 + 0x23c]
1417a9fb8: mov dword ptr [rbp + 0x13c], eax
1417a9fbe: mov rax, qword ptr [r15 + 0x260]
1417a9fc5: mov qword ptr [rbp + 0xa8], rax
1417a9fcc: mov rax, qword ptr [r15 + 0x270]
1417a9fd3: mov qword ptr [rbp + 0xb0], rax
1417a9fda: mov eax, dword ptr [r15 + 0x27c]
1417a9fe1: mov dword ptr [rbp + 0x12c], eax
1417a9fe7: movdqu xmm0, xmmword ptr [rsi + 0x28]
1417a9fec: movdqa xmmword ptr [rbp + 0x180], xmm0
1417a9ff4: mov byte ptr [rbp + 0x21f], 1
1417a9ffb: mov byte ptr [rbp + 0x21e], 1
1417aa002: mov byte ptr [rbp + 0x21d], 1
1417aa009: mov byte ptr [rbp + 0x21c], 1
1417aa010: mov byte ptr [rbp + 0x21b], 1
1417aa017: mov byte ptr [rbp + 0x21a], 1
1417aa01e: mov byte ptr [rbp + 0x219], 1
1417aa025: mov byte ptr [rbp + 0x218], 1
1417aa02c: mov byte ptr [rbp + 0x217], 0
1417aa033: lea rdx, [rbp + 0x180]
1417aa03a: mov rcx, r12
1417aa03d: call 0x14085ad80
1417aa042: mov rax, qword ptr [rbp + 0x208]
1417aa049: mov rcx, qword ptr [rax + 0x40]
1417aa04d: mov qword ptr [rbp + 0x168], rcx
1417aa054: mov rcx, qword ptr [rax + 0x48]
1417aa058: mov rax, qword ptr [rax + 0x50]
1417aa05c: test rax, rax
1417aa05f: je 0x1417aa17b
1417aa065: lea rdx, [rbx + 0x20]
1417aa069: mov qword ptr [rbp + 0x78], rdx
1417aa06d: imul rax, rax, 0x58
1417aa071: mov qword ptr [rbp + 0x158], rax
1417aa078: xor esi, esi
1417aa07a: mov qword ptr [rbp + 0x1d8], rcx
1417aa081: jmp 0x1417aa102
1417aa083: nop word ptr cs:[rax + rax]
1417aa090: mov rax, qword ptr [rbx + 0x28]
1417aa094: mov rcx, r15
1417aa097: shl rcx, 6
1417aa09b: mov qword ptr [rax + rcx], r12
1417aa09f: mov qword ptr [rax + rcx + 8], r13
1417aa0a4: mov edx, dword ptr [rbp + 0x1e8]
1417aa0aa: mov dword ptr [rax + rcx + 0x10], edx
1417aa0ae: mov rdx, qword ptr [rbp + 0xf0]
1417aa0b5: mov qword ptr [rax + rcx + 0x14], rdx
1417aa0ba: mov edx, dword ptr [rbp + 0xf8]
1417aa0c0: mov dword ptr [rax + rcx + 0x1c], edx
1417aa0c4: movaps xmm0, xmmword ptr [rbp + 0x180]
1417aa0cb: movups xmmword ptr [rax + rcx + 0x20], xmm0
1417aa0d0: movdqu xmm0, xmmword ptr [rbp + 0x18c]
1417aa0d8: movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1417aa0de: mov byte ptr [rax + rcx + 0x3c], 1
1417aa0e3: inc r15
1417aa0e6: mov qword ptr [rbx + 0x30], r15
1417aa0ea: inc qword ptr [rbx + 0x40]
1417aa0ee: add rsi, 0x58
1417aa0f2: cmp qword ptr [rbp + 0x158], rsi
1417aa0f9: mov rcx, qword ptr [rbp + 0x1d8]
1417aa100: je 0x1417aa17b
1417aa102: cmp dword ptr [rcx + rsi + 8], 2
1417aa107: je 0x1417aa17b
1417aa109: mov r13, qword ptr [rcx + rsi]
1417aa10d: mov eax, dword ptr [rcx + rsi + 0x50]
1417aa111: mov dword ptr [rbp + 0x1e8], eax
1417aa117: movups xmm0, xmmword ptr [rcx + rsi + 0x24]
1417aa11c: movups xmm1, xmmword ptr [rcx + rsi + 0x30]
1417aa121: movups xmmword ptr [rbp + 0x18c], xmm1
1417aa128: movaps xmmword ptr [rbp + 0x180], xmm0
1417aa12f: mov rax, qword ptr [rcx + rsi + 0x40]
1417aa134: mov qword ptr [rbp + 0xf0], rax
1417aa13b: mov eax, dword ptr [rcx + rsi + 0x48]
1417aa13f: mov dword ptr [rbp + 0xf8], eax
1417aa145: mov rax, qword ptr [rbp + 0x150]
1417aa14c: mov ecx, dword ptr [rbp + 0x1d0]
1417aa152: mov dword ptr [rax], ecx
1417aa154: mov r15, qword ptr [rbx + 0x30]
1417aa158: mov r12, qword ptr [rbx + 0x40]
1417aa15c: cmp r15, qword ptr [rbx + 0x20]
1417aa160: jne 0x1417aa090
1417aa166: mov rcx, qword ptr [rbp + 0x78]
1417aa16a: lea rdx, [rip + 0x148ffd7]
1417aa171: call 0x1405627b0
1417aa176: jmp 0x1417aa090
1417aa17b: mov rax, qword ptr [rbp + 0x168]
1417aa182: test rax, rax
1417aa185: je 0x1417aa196
1417aa187: imul rdx, rax, 0x58
1417aa18b: mov r8d, 8
1417aa191: call 0x140613c20
1417aa196: mov rax, qword ptr [rbp + 0x90]
1417aa19d: mov ecx, dword ptr [rbp + 0x120]
1417aa1a3: mov dword ptr [rax], ecx
1417aa1a5: mov rax, qword ptr [rbp + 0x208]
1417aa1ac: lea rdx, [rax + 0x58]
1417aa1b0: mov rax, qword ptr [rbp + 0x98]
1417aa1b7: mov ecx, dword ptr [rbp + 0x124]
1417aa1bd: mov dword ptr [rax], ecx
1417aa1bf: mov byte ptr [rbp + 0x21f], 0
1417aa1c6: mov byte ptr [rbp + 0x21e], 1
1417aa1cd: mov byte ptr [rbp + 0x21d], 1
1417aa1d4: mov byte ptr [rbp + 0x21c], 1
1417aa1db: mov byte ptr [rbp + 0x21b], 1
1417aa1e2: mov byte ptr [rbp + 0x21a], 1
1417aa1e9: mov byte ptr [rbp + 0x219], 1
1417aa1f0: mov byte ptr [rbp + 0x218], 1
1417aa1f7: mov byte ptr [rbp + 0x217], 0
1417aa1fe: mov rcx, qword ptr [rbp + 0x80]
1417aa205: mov qword ptr [rbp + 0x158], rdx
1417aa20c: mov r8, qword ptr [rbp + 0x88]
1417aa213: call 0x140a20d70
1417aa218: mov rax, qword ptr [rbp + 0x208]
1417aa21f: mov rcx, qword ptr [rax + 0x70]
1417aa223: mov qword ptr [rbp + 0x1d8], rcx
1417aa22a: mov rcx, qword ptr [rax + 0x78]
1417aa22e: mov qword ptr [rbp + 0x1d0], rcx
1417aa235: mov rcx, qword ptr [rax + 0x80]
1417aa23c: test rcx, rcx
1417aa23f: je 0x1417aa2db
1417aa245: mov rax, rcx
1417aa248: shl rax, 4
1417aa24c: mov r15, qword ptr [rbp + 0x1d0]
1417aa253: add rax, r15
1417aa256: mov qword ptr [rbp + 0x1e8], rax
1417aa25d: lea rax, [r14 + 0x20]
1417aa261: mov qword ptr [rbp + 0x150], rax
1417aa268: jmp 0x1417aa2a2
1417aa26a: nop word ptr [rax + rax]
1417aa270: mov rax, qword ptr [r14 + 0x28]
1417aa274: lea rcx, [r12 + r12*2]
1417aa278: mov qword ptr [rax + rcx*8], rsi
1417aa27c: mov qword ptr [rax + rcx*8 + 8], r13
1417aa281: mov dword ptr [rax + rcx*8 + 0x10], ebx
1417aa285: mov byte ptr [rax + rcx*8 + 0x14], 0
1417aa28a: inc r12
1417aa28d: mov qword ptr [r14 + 0x30], r12
1417aa291: inc qword ptr [r14 + 0x40]
1417aa295: add r15, 0x10
1417aa299: cmp r15, qword ptr [rbp + 0x1e8]
1417aa2a0: je 0x1417aa2db
1417aa2a2: mov r13, qword ptr [r15]
1417aa2a5: mov ebx, dword ptr [r15 + 8]
1417aa2a9: mov rax, qword ptr [rbp + 0x1f8]
1417aa2b0: mov ecx, dword ptr [rbp + 0x200]
1417aa2b6: mov dword ptr [rax], ecx
1417aa2b8: mov r12, qword ptr [r14 + 0x30]
1417aa2bc: mov rsi, qword ptr [r14 + 0x40]
1417aa2c0: cmp r12, qword ptr [r14 + 0x20]
1417aa2c4: jne 0x1417aa270
1417aa2c6: mov rcx, qword ptr [rbp + 0x150]
1417aa2cd: lea rdx, [rip + 0x148fe74]
1417aa2d4: call 0x140562db0
1417aa2d9: jmp 0x1417aa270
1417aa2db: mov rdx, qword ptr [rbp + 0x1d8]
1417aa2e2: test rdx, rdx
1417aa2e5: je 0x1417aa2fd
1417aa2e7: shl rdx, 4
1417aa2eb: mov r8d, 8
1417aa2f1: mov rcx, qword ptr [rbp + 0x1d0]
1417aa2f8: call 0x140613c20
1417aa2fd: mov rax, qword ptr [rbp + 0x208]
1417aa304: mov rcx, qword ptr [rax + 0x88]
1417aa30b: mov qword ptr [rbp + 0x1e8], rcx
1417aa312: mov rcx, qword ptr [rax + 0x90]
1417aa319: mov qword ptr [rbp + 0x200], rcx
1417aa320: mov rsi, qword ptr [rax + 0x98]
1417aa327: test rsi, rsi
1417aa32a: je 0x1417aa3b5
1417aa330: shl rsi, 4
1417aa334: mov r15, qword ptr [rbp + 0x200]
1417aa33b: add rsi, r15
1417aa33e: lea rax, [rdi + 0x20]
1417aa342: mov qword ptr [rbp + 0x1f8], rax
1417aa349: jmp 0x1417aa37f
1417aa34b: nop dword ptr [rax + rax]
1417aa350: mov rax, qword ptr [rdi + 0x28]
1417aa354: lea rcx, [r12 + r12*2]
1417aa358: mov qword ptr [rax + rcx*8], rbx
1417aa35c: mov qword ptr [rax + rcx*8 + 8], r13
1417aa361: mov dword ptr [rax + rcx*8 + 0x10], r14d
1417aa366: mov byte ptr [rax + rcx*8 + 0x14], 0
1417aa36b: inc r12
1417aa36e: mov qword ptr [rdi + 0x30], r12
1417aa372: inc qword ptr [rdi + 0x40]
1417aa376: add r15, 0x10
1417aa37a: cmp r15, rsi
1417aa37d: je 0x1417aa3b5
1417aa37f: mov r13, qword ptr [r15]
1417aa382: mov r14d, dword ptr [r15 + 8]
1417aa386: mov rax, qword ptr [rbp + 0x60]
1417aa38a: mov ecx, dword ptr [rbp + 0x114]
1417aa390: mov dword ptr [rax], ecx
1417aa392: mov r12, qword ptr [rdi + 0x30]
1417aa396: mov rbx, qword ptr [rdi + 0x40]
1417aa39a: cmp r12, qword ptr [rdi + 0x20]
1417aa39e: jne 0x1417aa350
1417aa3a0: mov rcx, qword ptr [rbp + 0x1f8]
1417aa3a7: lea rdx, [rip + 0x148fd9a]
1417aa3ae: call 0x140562db0
1417aa3b3: jmp 0x1417aa350
1417aa3b5: mov rdx, qword ptr [rbp + 0x1e8]
1417aa3bc: test rdx, rdx
1417aa3bf: je 0x1417aa3d7
1417aa3c1: shl rdx, 4
1417aa3c5: mov r8d, 8
1417aa3cb: mov rcx, qword ptr [rbp + 0x200]
1417aa3d2: call 0x140613c20
1417aa3d7: mov rax, qword ptr [rbp + 0x208]
1417aa3de: mov rcx, qword ptr [rax + 0xa0]
1417aa3e5: mov qword ptr [rbp + 0x200], rcx
1417aa3ec: mov rcx, qword ptr [rax + 0xa8]
1417aa3f3: mov qword ptr [rbp + 0x1f8], rcx
1417aa3fa: mov rsi, qword ptr [rax + 0xb0]
1417aa401: test rsi, rsi
1417aa404: mov rdx, qword ptr [rbp + 0x178]
1417aa40b: je 0x1417aa498
1417aa411: shl rsi, 4
1417aa415: mov rdi, qword ptr [rbp + 0x1f8]
1417aa41c: add rsi, rdi
1417aa41f: lea rbx, [rdx + 0x20]
1417aa423: jmp 0x1417aa466
1417aa425: nop word ptr cs:[rax + rax]
1417aa430: mov rdx, qword ptr [rbp + 0x178]
1417aa437: mov rax, qword ptr [rdx + 0x28]
1417aa43b: lea rcx, [r15 + r15*2]
1417aa43f: mov qword ptr [rax + rcx*8], r14
1417aa443: mov qword ptr [rax + rcx*8 + 8], r12
1417aa448: mov dword ptr [rax + rcx*8 + 0x10], r13d
1417aa44d: mov byte ptr [rax + rcx*8 + 0x14], 0
1417aa452: inc r15
1417aa455: mov qword ptr [rdx + 0x30], r15
1417aa459: inc qword ptr [rdx + 0x40]
1417aa45d: add rdi, 0x10
1417aa461: cmp rdi, rsi
1417aa464: je 0x1417aa498
1417aa466: mov r12, qword ptr [rdi]
1417aa469: mov r13d, dword ptr [rdi + 8]
1417aa46d: mov rax, qword ptr [rbp + 0x68]
1417aa471: mov ecx, dword ptr [rbp + 0x118]
1417aa477: mov dword ptr [rax], ecx
1417aa479: mov r15, qword ptr [rdx + 0x30]
1417aa47d: mov r14, qword ptr [rdx + 0x40]
1417aa481: cmp r15, qword ptr [rdx + 0x20]
1417aa485: jne 0x1417aa430
1417aa487: mov rcx, rbx
1417aa48a: lea rdx, [rip + 0x148fcb7]
1417aa491: call 0x140562db0
1417aa496: jmp 0x1417aa430
1417aa498: mov rdx, qword ptr [rbp + 0x200]
1417aa49f: test rdx, rdx
1417aa4a2: je 0x1417aa4ba
1417aa4a4: shl rdx, 4
1417aa4a8: mov r8d, 8
1417aa4ae: mov rcx, qword ptr [rbp + 0x1f8]
1417aa4b5: call 0x140613c20
1417aa4ba: mov rax, qword ptr [rbp + 0x208]
1417aa4c1: lea rcx, [rax + 0x100]
1417aa4c8: mov qword ptr [rbp + 0x1f8], rcx
1417aa4cf: mov rdi, qword ptr [rax + 0x108]
1417aa4d6: mov r15, qword ptr [rax + 0x120]
1417aa4dd: movdqa xmm0, xmmword ptr [rdi]
1417aa4e1: pmovmskb eax, xmm0
1417aa4e5: not eax
1417aa4e7: lea r13, [rdi + 0x10]
1417aa4eb: mov rcx, qword ptr [rbp + 0x1f0]
1417aa4f2: add rcx, 0x20
1417aa4f6: mov qword ptr [rbp + 0x178], rcx
1417aa4fd: jmp 0x1417aa527
1417aa4ff: nop 
1417aa500: mov rdx, qword ptr [rbp + 0x1f0]
1417aa507: mov rax, qword ptr [rdx + 0x28]
1417aa50b: mov rcx, r14
1417aa50e: shl rcx, 4
1417aa512: mov qword ptr [rax + rcx], r12
1417aa516: mov dword ptr [rax + rcx + 8], ebx
1417aa51a: inc r14
1417aa51d: mov qword ptr [rdx + 0x30], r14
1417aa521: inc qword ptr [rdx + 0x40]
1417aa525: mov eax, esi
1417aa527: mov r10, qword ptr [rbp + 0x1e0]
1417aa52e: sub r15, 1
1417aa532: jb 0x1417aa60b
1417aa538: test ax, ax
1417aa53b: je 0x1417aa550
1417aa53d: lea esi, [rax - 1]
1417aa540: and esi, eax
1417aa542: jmp 0x1417aa579
1417aa544: nop word ptr cs:[rax + rax]
1417aa550: movdqa xmm0, xmmword ptr [r13]
1417aa556: pmovmskb eax, xmm0
1417aa55a: add rdi, -0x300
1417aa561: add r13, 0x10
1417aa565: cmp eax, 0xffff
1417aa56a: je 0x1417aa550
1417aa56c: mov ecx, 0xfffffffe
1417aa571: sub ecx, eax
1417aa573: not eax
1417aa575: mov esi, eax
1417aa577: and esi, ecx
1417aa579: tzcnt eax, eax
1417aa57d: neg rax
1417aa580: lea rax, [rax + rax*2]
1417aa584: shl rax, 4
1417aa588: mov ebx, dword ptr [rdi + rax - 0x30]
1417aa58c: mov rax, qword ptr [rbp + 0x170]
1417aa593: mov ecx, dword ptr [rbp + 0x140]
1417aa599: mov dword ptr [rax], ecx
1417aa59b: mov rax, qword ptr [rbp + 0x1f0]
1417aa5a2: mov r14, qword ptr [rax + 0x30]
1417aa5a6: mov r12, qword ptr [rax + 0x40]
1417aa5aa: cmp r14, qword ptr [rax + 0x20]
1417aa5ae: jne 0x1417aa500
1417aa5b4: mov byte ptr [rbp + 0x21f], 0
1417aa5bb: mov byte ptr [rbp + 0x21e], 0
1417aa5c2: mov byte ptr [rbp + 0x21d], 0
1417aa5c9: mov byte ptr [rbp + 0x21c], 0
1417aa5d0: mov byte ptr [rbp + 0x21b], 1
1417aa5d7: mov byte ptr [rbp + 0x21a], 1
1417aa5de: mov byte ptr [rbp + 0x219], 1
1417aa5e5: mov byte ptr [rbp + 0x218], 1
1417aa5ec: mov byte ptr [rbp + 0x217], 0
1417aa5f3: mov rcx, qword ptr [rbp + 0x178]
1417aa5fa: lea rdx, [rip + 0x148fb47]
1417aa601: call 0x140562630
1417aa606: jmp 0x1417aa500
1417aa60b: mov rcx, qword ptr [rbp + 0x1f8]
1417aa612: mov rax, qword ptr [rcx + 0x40]
1417aa616: mov qword ptr [rbp + 0x1c0], rax
1417aa61d: movups xmm0, xmmword ptr [rcx]
1417aa620: movups xmm1, xmmword ptr [rcx + 0x10]
1417aa624: movups xmm2, xmmword ptr [rcx + 0x20]
1417aa628: movups xmm3, xmmword ptr [rcx + 0x30]
1417aa62c: movaps xmmword ptr [rbp + 0x1b0], xmm3
1417aa633: movaps xmmword ptr [rbp + 0x1a0], xmm2
1417aa63a: movaps xmmword ptr [rbp + 0x190], xmm1
1417aa641: movaps xmmword ptr [rbp + 0x180], xmm0
1417aa648: mov rax, qword ptr [rbp + 0xa0]
1417aa64f: mov ecx, dword ptr [rbp + 0x128]
1417aa655: mov dword ptr [rax], ecx
1417aa657: lea rcx, [r10 + 8]
1417aa65b: lea rdx, [r10 + 0x28]
1417aa65f: mov r8d, 0x30
1417aa665: mov r9d, 0x10
1417aa66b: call 0x14004f570
1417aa670: mov rax, qword ptr [rbp + 0x1c0]
1417aa677: mov rcx, qword ptr [rbp + 0x1e0]
1417aa67e: mov qword ptr [rcx + 0x40], rax
1417aa682: movaps xmm0, xmmword ptr [rbp + 0x180]
1417aa689: movaps xmm1, xmmword ptr [rbp + 0x190]
1417aa690: movaps xmm2, xmmword ptr [rbp + 0x1a0]
1417aa697: movaps xmm3, xmmword ptr [rbp + 0x1b0]
1417aa69e: movups xmmword ptr [rcx + 0x30], xmm3
1417aa6a2: movups xmmword ptr [rcx + 0x20], xmm2
1417aa6a6: movups xmmword ptr [rcx + 0x10], xmm1
1417aa6aa: movups xmmword ptr [rcx], xmm0
1417aa6ad: mov rsi, qword ptr [rbp + 0x208]
1417aa6b4: movups xmm0, xmmword ptr [rsi + 0x178]
1417aa6bb: movups xmm1, xmmword ptr [rsi + 0x188]
1417aa6c2: movups xmm2, xmmword ptr [rsi + 0x198]
1417aa6c9: movaps xmmword ptr [rbp + 0x1a0], xmm2
1417aa6d0: movaps xmmword ptr [rbp + 0x190], xmm1
1417aa6d7: movaps xmmword ptr [rbp + 0x180], xmm0
1417aa6de: mov rax, qword ptr [rbp + 0xb0]
1417aa6e5: mov ecx, dword ptr [rbp + 0x12c]
1417aa6eb: mov dword ptr [rax], ecx
1417aa6ed: mov rdi, qword ptr [rbp + 0xa8]
1417aa6f4: mov rdx, qword ptr [rdi + 8]
1417aa6f8: test rdx, rdx
1417aa6fb: je 0x1417aa723
1417aa6fd: lea rax, [rdx*8 + 0x17]
1417aa705: and rax, 0xfffffffffffffff0
1417aa709: add rdx, rax
1417aa70c: add rdx, 0x11
1417aa710: je 0x1417aa723
1417aa712: mov rcx, qword ptr [rdi]
1417aa715: sub rcx, rax
1417aa718: mov r8d, 0x10
1417aa71e: call 0x140613c20
1417aa723: movdqa xmm0, xmmword ptr [rbp + 0x180]
1417aa72b: movaps xmm1, xmmword ptr [rbp + 0x190]
1417aa732: movaps xmm2, xmmword ptr [rbp + 0x1a0]
1417aa739: movups xmmword ptr [rdi + 0x20], xmm2
1417aa73d: movups xmmword ptr [rdi + 0x10], xmm1
1417aa741: movdqu xmmword ptr [rdi], xmm0
1417aa745: mov rdx, qword ptr [rsi + 0xc0]
1417aa74c: mov r8, qword ptr [rsi + 0xc8]
1417aa753: mov byte ptr [rbp + 0x21f], 0
1417aa75a: mov byte ptr [rbp + 0x21e], 0
1417aa761: mov byte ptr [rbp + 0x21d], 0
1417aa768: mov byte ptr [rbp + 0x21c], 0
1417aa76f: mov byte ptr [rbp + 0x21b], 0
1417aa776: mov byte ptr [rbp + 0x21a], 0
1417aa77d: mov byte ptr [rbp + 0x219], 1
1417aa784: mov byte ptr [rbp + 0x218], 1
1417aa78b: mov byte ptr [rbp + 0x217], 0
1417aa792: lea rcx, [rbp - 0x60]
1417aa796: call 0x14083ebe0
1417aa79b: mov rax, qword ptr [rbp + 0xb8]
1417aa7a2: mov ecx, dword ptr [rbp + 0x130]
1417aa7a8: mov dword ptr [rax], ecx
1417aa7aa: mov rax, qword ptr [rbp + 0x160]
1417aa7b1: mov rsi, qword ptr [rax + 0x30]
1417aa7b5: mov rdi, qword ptr [rax + 0x40]
1417aa7b9: cmp rsi, qword ptr [rax + 0x20]
1417aa7bd: jne 0x1417aa815
1417aa7bf: mov rax, qword ptr [rbp + 0x160]
1417aa7c6: lea rcx, [rax + 0x20]
1417aa7ca: mov byte ptr [rbp + 0x21f], 0
1417aa7d1: mov byte ptr [rbp + 0x21e], 0
1417aa7d8: mov byte ptr [rbp + 0x21d], 0
1417aa7df: mov byte ptr [rbp + 0x21c], 0
1417aa7e6: mov byte ptr [rbp + 0x21b], 0
1417aa7ed: mov byte ptr [rbp + 0x21a], 0
1417aa7f4: mov byte ptr [rbp + 0x219], 1
1417aa7fb: mov byte ptr [rbp + 0x218], 1
1417aa802: mov byte ptr [rbp + 0x217], 0
1417aa809: lea rdx, [rip + 0x14de868]
1417aa810: call 0x140562870
1417aa815: mov rcx, qword ptr [rbp + 0x160]
1417aa81c: mov rax, qword ptr [rcx + 0x28]
1417aa820: mov qword ptr [rax + rsi*8], rdi
1417aa824: inc rsi
1417aa827: mov qword ptr [rcx + 0x30], rsi
1417aa82b: inc qword ptr [rcx + 0x40]
1417aa82f: mov rax, qword ptr [rbp + 0xc8]
1417aa836: mov ecx, dword ptr [rbp + 0x134]
1417aa83c: mov dword ptr [rax], ecx
1417aa83e: mov rbx, qword ptr [rbp + 0x208]
1417aa845: movq xmm0, qword ptr [rbx + 0x1a8]
1417aa84d: mov rax, qword ptr [rbp + 0xc0]
1417aa854: movq qword ptr [rax + 0x50], xmm0
1417aa859: mov r8, qword ptr [rax + 0x10]
1417aa85d: mov rdx, qword ptr [rbx + 0xe0]
1417aa864: cmp r8, rdx
1417aa867: jne 0x1417aac80
1417aa86d: mov rcx, qword ptr [rax + 8]
1417aa871: mov rdx, qword ptr [rbx + 0xd8]
1417aa878: shl r8, 2
1417aa87c: call 0x142923c00
1417aa881: mov r15, qword ptr [rbx + 0xe8]
1417aa888: mov r14, qword ptr [rbx + 0xf0]
1417aa88f: mov rax, qword ptr [rbx + 0xf8]
1417aa896: mov rdi, rax
1417aa899: shl rdi, 6
1417aa89d: lea rsi, [r14 + rdi]
1417aa8a1: mov qword ptr [rbp + 0xf0], r14
1417aa8a8: mov qword ptr [rbp + 0xf8], r14
1417aa8af: mov qword ptr [rbp + 0x100], r15
1417aa8b6: mov qword ptr [rbp + 0x108], rsi
1417aa8bd: mov r12, r14
1417aa8c0: test rax, rax
1417aa8c3: je 0x1417aaa0d
1417aa8c9: mov qword ptr [rbp + 0x140], r15
1417aa8d0: mov rax, qword ptr [rbp + 0x148]
1417aa8d7: lea r15, [rax + 0x20]
1417aa8db: lea r12, [r14 + 0x40]
1417aa8df: jmp 0x1417aa966
1417aa8e4: nop word ptr cs:[rax + rax]
1417aa8f0: mov r8, qword ptr [rbp + 0x148]
1417aa8f7: mov rax, qword ptr [r8 + 0x28]
1417aa8fb: lea rcx, [rbx + rbx*8]
1417aa8ff: mov rdx, qword ptr [rbp + 0x1f0]
1417aa906: mov qword ptr [rax + rcx*8], rdx
1417aa90a: mov rdx, qword ptr [rbp + 0x170]
1417aa911: mov qword ptr [rax + rcx*8 + 8], rdx
1417aa916: movdqa xmm0, xmmword ptr [rbp + 0x180]
1417aa91e: movaps xmm1, xmmword ptr [rbp + 0x190]
1417aa925: movdqu xmmword ptr [rax + rcx*8 + 0x10], xmm0
1417aa92b: movups xmmword ptr [rax + rcx*8 + 0x20], xmm1
1417aa930: mov rdx, qword ptr [rbp + 0x1a0]
1417aa937: mov qword ptr [rax + rcx*8 + 0x30], rdx
1417aa93c: mov qword ptr [rax + rcx*8 + 0x38], r13
1417aa941: mov qword ptr [rax + rcx*8 + 0x40], r12
1417aa946: inc rbx
1417aa949: mov qword ptr [r8 + 0x30], rbx
1417aa94d: inc qword ptr [r8 + 0x40]
1417aa951: mov r12, qword ptr [rbp + 0x1e0]
1417aa958: add r12, 0x40
1417aa95c: add rdi, -0x40
1417aa960: je 0x1417aa9f5
1417aa966: mov rcx, qword ptr [r12 - 0x38]
1417aa96b: mov rax, rcx
1417aa96e: neg rax
1417aa971: jo 0x1417aa9f8
1417aa977: mov qword ptr [rbp + 0x1f0], rcx
1417aa97e: mov r13, qword ptr [r12 - 0x40]
1417aa983: mov rax, qword ptr [r12 - 0x30]
1417aa988: mov qword ptr [rbp + 0x170], rax
1417aa98f: mov rax, qword ptr [r12 - 8]
1417aa994: mov qword ptr [rbp + 0x1a0], rax
1417aa99b: movups xmm0, xmmword ptr [r12 - 0x18]
1417aa9a1: movaps xmmword ptr [rbp + 0x190], xmm0
1417aa9a8: mov qword ptr [rbp + 0x1e0], r12
1417aa9af: movups xmm0, xmmword ptr [r12 - 0x28]
1417aa9b5: movaps xmmword ptr [rbp + 0x180], xmm0
1417aa9bc: mov rax, qword ptr [rbp + 0x70]
1417aa9c0: mov ecx, dword ptr [rbp + 0x11c]
1417aa9c6: mov dword ptr [rax], ecx
1417aa9c8: mov rax, qword ptr [rbp + 0x148]
1417aa9cf: mov rbx, qword ptr [rax + 0x30]
1417aa9d3: mov r12, qword ptr [rax + 0x40]
1417aa9d7: cmp rbx, qword ptr [rax + 0x20]
1417aa9db: jne 0x1417aa8f0
1417aa9e1: mov rcx, r15
1417aa9e4: lea rdx, [rip + 0x148f75d]
1417aa9eb: call 0x140b5f9f0
1417aa9f0: jmp 0x1417aa8f0
1417aa9f5: mov r12, rsi
1417aa9f8: mov rbx, qword ptr [rbp + 0x208]
1417aa9ff: mov qword ptr [rbp + 0xf8], r12
1417aaa06: mov r15, qword ptr [rbp + 0x140]
1417aaa0d: sub rsi, r12
1417aaa10: je 0x1417aaa44
1417aaa12: shr rsi, 6
1417aaa16: add r12, 0x10
1417aaa1a: jmp 0x1417aaa29
1417aaa1c: nop dword ptr [rax]
1417aaa20: add r12, 0x40
1417aaa24: dec rsi
1417aaa27: je 0x1417aaa44
1417aaa29: mov rdx, qword ptr [r12 - 8]
1417aaa2e: test rdx, rdx
1417aaa31: je 0x1417aaa20
1417aaa33: mov rcx, qword ptr [r12]
1417aaa37: mov r8d, 1
1417aaa3d: call 0x140613c20
1417aaa42: jmp 0x1417aaa20
1417aaa44: test r15, r15
1417aaa47: je 0x1417aaa5e
1417aaa49: shl r15, 6
1417aaa4d: mov r8d, 8
1417aaa53: mov rcx, r14
1417aaa56: mov rdx, r15
1417aaa59: call 0x140613c20
1417aaa5e: movups xmm0, xmmword ptr [rbx + 0x148]
1417aaa65: movups xmm1, xmmword ptr [rbx + 0x158]
1417aaa6c: movups xmm2, xmmword ptr [rbx + 0x168]
1417aaa73: movaps xmmword ptr [rbp + 0x1a0], xmm2
1417aaa7a: movaps xmmword ptr [rbp + 0x190], xmm1
1417aaa81: movaps xmmword ptr [rbp + 0x180], xmm0
1417aaa88: mov rax, qword ptr [rbp + 0xe8]
1417aaa8f: mov ecx, dword ptr [rbp + 0x13c]
1417aaa95: mov dword ptr [rax], ecx
1417aaa97: mov rsi, qword ptr [rbp + 0xe0]
1417aaa9e: mov rdx, qword ptr [rsi + 8]
1417aaaa2: test rdx, rdx
1417aaaa5: je 0x1417aaacd
1417aaaa7: lea rax, [rdx*8 + 0x17]
1417aaaaf: and rax, 0xfffffffffffffff0
1417aaab3: add rdx, rax
1417aaab6: add rdx, 0x11
1417aaaba: je 0x1417aaacd
1417aaabc: mov rcx, qword ptr [rsi]
1417aaabf: sub rcx, rax
1417aaac2: mov r8d, 0x10
1417aaac8: call 0x140613c20
1417aaacd: movaps xmm0, xmmword ptr [rbp + 0x180]
1417aaad4: movaps xmm1, xmmword ptr [rbp + 0x190]
1417aaadb: movaps xmm2, xmmword ptr [rbp + 0x1a0]
1417aaae2: movups xmmword ptr [rsi + 0x20], xmm2
1417aaae6: movups xmmword ptr [rsi + 0x10], xmm1
1417aaaea: movups xmmword ptr [rsi], xmm0
1417aaaed: mov rax, qword ptr [rbp + 0xd8]
1417aaaf4: mov ecx, dword ptr [rbp + 0x138]
1417aaafa: mov dword ptr [rax], ecx
1417aaafc: mov rax, qword ptr [rbx + 0x20]
1417aab00: mov qword ptr [rbp + 0x1a0], rax
1417aab07: movdqu xmm0, xmmword ptr [rbx]
1417aab0b: movups xmm1, xmmword ptr [rbx + 0x10]
1417aab0f: movaps xmmword ptr [rbp + 0x190], xmm1
1417aab16: movdqa xmmword ptr [rbp + 0x180], xmm0
1417aab1e: mov byte ptr [rbp + 0x21f], 0
1417aab25: mov byte ptr [rbp + 0x21e], 0
1417aab2c: mov byte ptr [rbp + 0x21d], 0
1417aab33: mov byte ptr [rbp + 0x21c], 0
1417aab3a: mov byte ptr [rbp + 0x21b], 0
1417aab41: mov byte ptr [rbp + 0x21a], 0
1417aab48: mov byte ptr [rbp + 0x219], 0
1417aab4f: mov byte ptr [rbp + 0x218], 0
1417aab56: mov byte ptr [rbp + 0x217], 0
1417aab5d: lea rdx, [rbp + 0x180]
1417aab64: mov rcx, qword ptr [rbp + 0xd0]
1417aab6b: call 0x1408ff590
1417aab70: mov r14, qword ptr [rbp + 0x208]
1417aab77: mov rsi, qword ptr [r14 + 0x60]
1417aab7b: mov rdi, qword ptr [r14 + 0x68]
1417aab7f: test rdi, rdi
1417aab82: je 0x1417aabb6
1417aab84: lea rbx, [rsi + 0x18]
1417aab88: jmp 0x1417aab99
1417aab8a: nop word ptr [rax + rax]
1417aab90: add rbx, 0x28
1417aab94: dec rdi
1417aab97: je 0x1417aabb6
1417aab99: mov rax, qword ptr [rbx - 8]
1417aab9d: test rax, rax
1417aaba0: je 0x1417aab90
1417aaba2: mov rcx, qword ptr [rbx]
1417aaba5: imul rdx, rax, 0x58
1417aaba9: mov r8d, 8
1417aabaf: call 0x140613c20
1417aabb4: jmp 0x1417aab90
1417aabb6: mov rax, qword ptr [rbp + 0x158]
1417aabbd: mov rax, qword ptr [rax]
1417aabc0: test rax, rax
1417aabc3: je 0x1417aabdb
1417aabc5: shl rax, 3
1417aabc9: lea rdx, [rax + rax*4]
1417aabcd: mov r8d, 8
1417aabd3: mov rcx, rsi
1417aabd6: call 0x140613c20
1417aabdb: mov rsi, qword ptr [r14 + 0xc0]
1417aabe2: mov rdi, qword ptr [r14 + 0xc8]
1417aabe9: test rdi, rdi
1417aabec: je 0x1417aac27
1417aabee: lea rbx, [rsi + 0x18]
1417aabf2: jmp 0x1417aac09
1417aabf4: nop word ptr cs:[rax + rax]
1417aac00: add rbx, 0x28
1417aac04: dec rdi
1417aac07: je 0x1417aac27
1417aac09: mov rdx, qword ptr [rbx - 8]
1417aac0d: mov rax, rdx
1417aac10: neg rax
1417aac13: jo 0x1417aac00
1417aac15: jae 0x1417aac00
1417aac17: mov rcx, qword ptr [rbx]
1417aac1a: mov r8d, 1
1417aac20: call 0x140613c20
1417aac25: jmp 0x1417aac00
1417aac27: mov rax, qword ptr [r14 + 0xb8]
1417aac2e: test rax, rax
1417aac31: je 0x1417aac49
1417aac33: shl rax, 3
1417aac37: lea rdx, [rax + rax*4]
1417aac3b: mov r8d, 8
1417aac41: mov rcx, rsi
1417aac44: call 0x140613c20
1417aac49: mov rdx, qword ptr [r14 + 0xd0]
1417aac50: test rdx, rdx
1417aac53: je 0x1417aac6b
1417aac55: mov rcx, qword ptr [r14 + 0xd8]
1417aac5c: shl rdx, 2
1417aac60: mov r8d, 4
1417aac66: call 0x140613c20
1417aac6b: nop 
1417aac6c: add rsp, 0x2a8
1417aac73: pop rbx
1417aac74: pop rdi
1417aac75: pop rsi
1417aac76: pop r12
1417aac78: pop r13
1417aac7a: pop r14
1417aac7c: pop r15
1417aac7e: pop rbp